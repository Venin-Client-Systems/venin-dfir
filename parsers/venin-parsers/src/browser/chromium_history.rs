use rusqlite::{Connection, OpenFlags};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::BTreeMap;
use std::fs::{self, File};
use std::io;
use std::path::{Path, PathBuf};
use thiserror::Error;
use venin_core::{
    InputType, OutputFormat, OutputSchema, SchemaField, ToolKind, ToolMetadata, ToolStatus,
};
use venin_shared::{chromium_webkit_microseconds_to_utc, sha256_file};
use venin_timeline::TimelineEvent;

pub const TOOL_ID: &str = "browser_history.chromium";

#[derive(Debug, Error)]
pub enum ChromiumHistoryError {
    #[error("io error: {0}")]
    Io(#[from] io::Error),
    #[error("sqlite error: {0}")]
    Sqlite(#[from] rusqlite::Error),
    #[error("required SQLite table missing: {0}")]
    MissingTable(&'static str),
    #[error("json serialization error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("csv serialization error: {0}")]
    Csv(#[from] csv::Error),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserHistoryRecord {
    pub parser_id: String,
    pub source_path: PathBuf,
    pub evidence_id: Option<String>,
    pub source_row_id: i64,
    pub url: String,
    pub title: String,
    pub visit_count: i64,
    pub typed_count: i64,
    pub hidden: bool,
    pub last_visit_time_raw: i64,
    pub last_visit_time_utc: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserHistoryParseResult {
    pub parser_id: String,
    pub source_path: PathBuf,
    pub source_sha256: Option<String>,
    pub records: Vec<BrowserHistoryRecord>,
    pub timeline_events: Vec<TimelineEvent>,
}

#[derive(Debug, Serialize)]
struct BrowserHistoryCsvRow<'a> {
    parser_id: &'a str,
    source_path: String,
    evidence_id: String,
    source_row_id: i64,
    url: &'a str,
    title: &'a str,
    visit_count: i64,
    typed_count: i64,
    hidden: bool,
    last_visit_time_raw: i64,
    last_visit_time_utc: String,
}

pub struct ChromiumHistoryParser;

impl ChromiumHistoryParser {
    pub fn metadata() -> ToolMetadata {
        ToolMetadata {
            id: TOOL_ID.to_string(),
            display_name: "Chromium Browser History Parser".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            kind: ToolKind::Parser,
            status: ToolStatus::Experimental,
            supported_artifacts: vec![
                "Chrome History SQLite database".to_string(),
                "Chromium History SQLite database".to_string(),
                "Microsoft Edge History SQLite database".to_string(),
                "Brave History SQLite database".to_string(),
            ],
            input_types: vec![InputType::SQLiteDatabase, InputType::File],
            output_formats: vec![
                OutputFormat::Json,
                OutputFormat::Csv,
                OutputFormat::TimelineJson,
                OutputFormat::TimelineCsv,
            ],
            output_schemas: vec![OutputSchema {
                name: "browser_history_record".to_string(),
                description: "URL summary rows extracted from Chromium-family History SQLite databases."
                    .to_string(),
                fields: vec![
                    field("url", "string", "Visited URL.", true),
                    field("title", "string", "Page title recorded by the browser.", false),
                    field("visit_count", "integer", "Browser-maintained aggregate visit count.", true),
                    field(
                        "last_visit_time_utc",
                        "datetime|null",
                        "Normalized UTC timestamp derived from Chromium WebKit microseconds.",
                        false,
                    ),
                    field(
                        "last_visit_time_raw",
                        "integer",
                        "Original Chromium timestamp value preserved for repeatability.",
                        true,
                    ),
                ],
            }],
            evidence_notes: vec![
                "Open the database read-only; parse copies of live browser databases where possible."
                    .to_string(),
                "WAL and SHM companion files may contain relevant recent history if the browser was active."
                    .to_string(),
                "Raw timestamps are retained beside normalized UTC values.".to_string(),
            ],
        }
    }

    pub fn parse_path(
        path: impl AsRef<Path>,
        evidence_id: Option<String>,
    ) -> Result<BrowserHistoryParseResult, ChromiumHistoryError> {
        let source_path = path.as_ref().to_path_buf();
        let conn = Connection::open_with_flags(&source_path, OpenFlags::SQLITE_OPEN_READ_ONLY)?;

        if !table_exists(&conn, "urls")? {
            return Err(ChromiumHistoryError::MissingTable("urls"));
        }

        let source_sha256 = sha256_file(&source_path).ok();
        let records = parse_url_records(&conn, &source_path, evidence_id.clone())?;
        let timeline_events = if table_exists(&conn, "visits")? {
            parse_visit_timeline_events(&conn, &source_path, evidence_id)?
        } else {
            timeline_from_url_summaries(&records)
        };

        Ok(BrowserHistoryParseResult {
            parser_id: TOOL_ID.to_string(),
            source_path,
            source_sha256,
            records,
            timeline_events,
        })
    }
}

pub fn write_browser_history_json(
    path: impl AsRef<Path>,
    result: &BrowserHistoryParseResult,
) -> Result<(), ChromiumHistoryError> {
    ensure_parent(path.as_ref())?;
    let file = File::create(path)?;
    serde_json::to_writer_pretty(file, result)?;
    Ok(())
}

pub fn write_browser_history_csv(
    path: impl AsRef<Path>,
    records: &[BrowserHistoryRecord],
) -> Result<(), ChromiumHistoryError> {
    ensure_parent(path.as_ref())?;
    let mut writer = csv::Writer::from_path(path)?;

    for record in records {
        writer.serialize(BrowserHistoryCsvRow {
            parser_id: &record.parser_id,
            source_path: record.source_path.display().to_string(),
            evidence_id: record.evidence_id.clone().unwrap_or_default(),
            source_row_id: record.source_row_id,
            url: &record.url,
            title: &record.title,
            visit_count: record.visit_count,
            typed_count: record.typed_count,
            hidden: record.hidden,
            last_visit_time_raw: record.last_visit_time_raw,
            last_visit_time_utc: record
                .last_visit_time_utc
                .as_ref()
                .map(|timestamp| timestamp.to_rfc3339())
                .unwrap_or_default(),
        })?;
    }

    writer.flush()?;
    Ok(())
}

fn parse_url_records(
    conn: &Connection,
    source_path: &Path,
    evidence_id: Option<String>,
) -> Result<Vec<BrowserHistoryRecord>, ChromiumHistoryError> {
    let mut statement = conn.prepare(
        r#"
        SELECT
            id,
            url,
            COALESCE(title, '') AS title,
            COALESCE(visit_count, 0) AS visit_count,
            COALESCE(typed_count, 0) AS typed_count,
            COALESCE(last_visit_time, 0) AS last_visit_time,
            COALESCE(hidden, 0) AS hidden
        FROM urls
        ORDER BY last_visit_time DESC, id ASC
        "#,
    )?;

    let rows = statement.query_map([], |row| {
        let last_visit_time_raw: i64 = row.get(5)?;

        Ok(BrowserHistoryRecord {
            parser_id: TOOL_ID.to_string(),
            source_path: source_path.to_path_buf(),
            evidence_id: evidence_id.clone(),
            source_row_id: row.get(0)?,
            url: row.get(1)?,
            title: row.get(2)?,
            visit_count: row.get(3)?,
            typed_count: row.get(4)?,
            hidden: row.get::<_, i64>(6)? != 0,
            last_visit_time_raw,
            last_visit_time_utc: chromium_webkit_microseconds_to_utc(last_visit_time_raw),
        })
    })?;

    rows.collect::<Result<Vec<_>, _>>()
        .map_err(ChromiumHistoryError::from)
}

fn parse_visit_timeline_events(
    conn: &Connection,
    source_path: &Path,
    evidence_id: Option<String>,
) -> Result<Vec<TimelineEvent>, ChromiumHistoryError> {
    let mut statement = conn.prepare(
        r#"
        SELECT
            visits.id,
            visits.url,
            urls.url,
            COALESCE(urls.title, '') AS title,
            COALESCE(visits.visit_time, 0) AS visit_time,
            COALESCE(visits.transition, 0) AS transition
        FROM visits
        JOIN urls ON urls.id = visits.url
        WHERE visits.visit_time > 0
        ORDER BY visits.visit_time ASC, visits.id ASC
        "#,
    )?;

    let rows = statement.query_map([], |row| {
        let visit_time_raw: i64 = row.get(4)?;
        Ok((
            row.get::<_, i64>(0)?,
            row.get::<_, i64>(1)?,
            row.get::<_, String>(2)?,
            row.get::<_, String>(3)?,
            visit_time_raw,
            row.get::<_, i64>(5)?,
        ))
    })?;

    let mut events = Vec::new();
    for row in rows {
        let (visit_id, url_id, url, title, visit_time_raw, transition) = row?;
        let Some(timestamp_utc) = chromium_webkit_microseconds_to_utc(visit_time_raw) else {
            continue;
        };

        let mut fields = BTreeMap::new();
        fields.insert("visit_id".to_string(), json!(visit_id));
        fields.insert("url_id".to_string(), json!(url_id));
        fields.insert("url".to_string(), json!(url));
        fields.insert("title".to_string(), json!(title));
        fields.insert("transition".to_string(), json!(transition));
        fields.insert("visit_time_raw".to_string(), json!(visit_time_raw));

        events.push(TimelineEvent {
            timestamp_utc,
            source_tool: TOOL_ID.to_string(),
            artifact_type: "browser_history".to_string(),
            event_type: "browser_visit".to_string(),
            description: "Chromium-family browser visit".to_string(),
            source_path: Some(source_path.to_path_buf()),
            evidence_id: evidence_id.clone(),
            fields,
        });
    }

    Ok(events)
}

fn timeline_from_url_summaries(records: &[BrowserHistoryRecord]) -> Vec<TimelineEvent> {
    records
        .iter()
        .filter_map(|record| {
            let timestamp_utc = record.last_visit_time_utc.as_ref()?.to_owned();
            let mut fields = BTreeMap::new();
            fields.insert("source_row_id".to_string(), json!(record.source_row_id));
            fields.insert("url".to_string(), json!(&record.url));
            fields.insert("title".to_string(), json!(&record.title));
            fields.insert("visit_count".to_string(), json!(record.visit_count));
            fields.insert(
                "last_visit_time_raw".to_string(),
                json!(record.last_visit_time_raw),
            );

            Some(TimelineEvent {
                timestamp_utc,
                source_tool: TOOL_ID.to_string(),
                artifact_type: "browser_history".to_string(),
                event_type: "browser_last_visit_summary".to_string(),
                description: "Chromium-family browser URL last visit".to_string(),
                source_path: Some(record.source_path.clone()),
                evidence_id: record.evidence_id.clone(),
                fields,
            })
        })
        .collect()
}

fn table_exists(conn: &Connection, table_name: &str) -> Result<bool, ChromiumHistoryError> {
    let exists: i64 = conn.query_row(
        "SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE type = 'table' AND name = ?1)",
        [table_name],
        |row| row.get(0),
    )?;

    Ok(exists != 0)
}

fn field(name: &str, field_type: &str, description: &str, required: bool) -> SchemaField {
    SchemaField {
        name: name.to_string(),
        field_type: field_type.to_string(),
        description: description.to_string(),
        required,
    }
}

fn ensure_parent(path: &Path) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::params;
    use tempfile::tempdir;
    use venin_shared::unix_seconds_to_chromium_webkit_microseconds;

    #[test]
    fn parses_url_records_from_chromium_history() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("History");
        create_sample_history_db(&db_path);

        let result =
            ChromiumHistoryParser::parse_path(&db_path, Some("CASE-001-EV-001".to_string()))
                .unwrap();

        assert_eq!(result.records.len(), 2);
        assert_eq!(result.records[0].url, "https://example.org/research");
        assert_eq!(result.records[0].visit_count, 3);
        assert_eq!(
            result.records[0]
                .last_visit_time_utc
                .as_ref()
                .unwrap()
                .to_rfc3339(),
            "2024-01-02T03:04:05+00:00"
        );
        assert!(result.source_sha256.is_some());
    }

    #[test]
    fn creates_timeline_events_from_visits_table() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("History");
        create_sample_history_db(&db_path);

        let result = ChromiumHistoryParser::parse_path(&db_path, None).unwrap();

        assert_eq!(result.timeline_events.len(), 2);
        assert_eq!(
            result.timeline_events[0].timestamp_utc.to_rfc3339(),
            "2024-01-01T12:00:00+00:00"
        );
        assert_eq!(result.timeline_events[0].event_type, "browser_visit");
    }

    #[test]
    fn exports_json_and_csv() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("History");
        let json_path = dir.path().join("out/history.json");
        let csv_path = dir.path().join("out/history.csv");
        create_sample_history_db(&db_path);

        let result = ChromiumHistoryParser::parse_path(&db_path, None).unwrap();
        write_browser_history_json(&json_path, &result).unwrap();
        write_browser_history_csv(&csv_path, &result.records).unwrap();

        assert!(json_path.exists());
        assert!(csv_path.exists());
    }

    fn create_sample_history_db(path: &Path) {
        let conn = Connection::open(path).unwrap();
        conn.execute_batch(
            r#"
            CREATE TABLE urls (
                id INTEGER PRIMARY KEY,
                url LONGVARCHAR,
                title LONGVARCHAR,
                visit_count INTEGER DEFAULT 0 NOT NULL,
                typed_count INTEGER DEFAULT 0 NOT NULL,
                last_visit_time INTEGER NOT NULL,
                hidden INTEGER DEFAULT 0 NOT NULL
            );

            CREATE TABLE visits (
                id INTEGER PRIMARY KEY,
                url INTEGER NOT NULL,
                visit_time INTEGER NOT NULL,
                from_visit INTEGER,
                transition INTEGER DEFAULT 0 NOT NULL,
                segment_id INTEGER
            );
            "#,
        )
        .unwrap();

        let first_visit = unix_seconds_to_chromium_webkit_microseconds(1_704_110_400);
        let second_visit = unix_seconds_to_chromium_webkit_microseconds(1_704_164_645);

        conn.execute(
            "INSERT INTO urls (id, url, title, visit_count, typed_count, last_visit_time, hidden)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                1,
                "https://example.org/research",
                "Research Notes",
                3,
                1,
                second_visit,
                0
            ],
        )
        .unwrap();
        conn.execute(
            "INSERT INTO urls (id, url, title, visit_count, typed_count, last_visit_time, hidden)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                2,
                "https://example.com/login",
                "Example Login",
                1,
                0,
                first_visit,
                0
            ],
        )
        .unwrap();

        conn.execute(
            "INSERT INTO visits (id, url, visit_time, from_visit, transition, segment_id)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![1, 2, first_visit, 0, 805306368_i64, 0],
        )
        .unwrap();
        conn.execute(
            "INSERT INTO visits (id, url, visit_time, from_visit, transition, segment_id)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![2, 1, second_visit, 1, 268435456_i64, 0],
        )
        .unwrap();
    }
}
