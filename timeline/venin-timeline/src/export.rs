use crate::TimelineEvent;
use serde::Serialize;
use std::fs::{self, File};
use std::io;
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TimelineExportError {
    #[error("io error: {0}")]
    Io(#[from] io::Error),
    #[error("json serialization error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("csv serialization error: {0}")]
    Csv(#[from] csv::Error),
}

#[derive(Debug, Serialize)]
struct TimelineCsvRow<'a> {
    timestamp_utc: String,
    source_tool: &'a str,
    artifact_type: &'a str,
    event_type: &'a str,
    description: &'a str,
    source_path: String,
    evidence_id: String,
    fields_json: String,
}

pub fn write_timeline_json(
    path: impl AsRef<Path>,
    events: &[TimelineEvent],
) -> Result<(), TimelineExportError> {
    ensure_parent(path.as_ref())?;
    let file = File::create(path)?;
    serde_json::to_writer_pretty(file, events)?;
    Ok(())
}

pub fn write_timeline_csv(
    path: impl AsRef<Path>,
    events: &[TimelineEvent],
) -> Result<(), TimelineExportError> {
    ensure_parent(path.as_ref())?;
    let mut writer = csv::Writer::from_path(path)?;

    for event in events {
        writer.serialize(TimelineCsvRow {
            timestamp_utc: event.timestamp_utc.to_rfc3339(),
            source_tool: &event.source_tool,
            artifact_type: &event.artifact_type,
            event_type: &event.event_type,
            description: &event.description,
            source_path: event
                .source_path
                .as_ref()
                .map(|path| path.display().to_string())
                .unwrap_or_default(),
            evidence_id: event.evidence_id.clone().unwrap_or_default(),
            fields_json: serde_json::to_string(&event.fields)?,
        })?;
    }

    writer.flush()?;
    Ok(())
}

fn ensure_parent(path: &Path) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    Ok(())
}
