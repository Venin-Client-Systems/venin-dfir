use venin_core::{InputType, OutputFormat, ToolKind, ToolMetadata, ToolStatus};

pub fn metadata() -> ToolMetadata {
    ToolMetadata {
        id: "mobile.android_sqlite".to_string(),
        display_name: "Android SQLite Artefact Parser".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        kind: ToolKind::Mobile,
        status: ToolStatus::Scaffold,
        supported_artifacts: vec![
            "Android application SQLite databases".to_string(),
            "ADB backup extracted application data".to_string(),
        ],
        input_types: vec![InputType::SQLiteDatabase, InputType::Directory],
        output_formats: vec![
            OutputFormat::Json,
            OutputFormat::Csv,
            OutputFormat::TimelineJson,
        ],
        output_schemas: vec![],
        evidence_notes: vec![
            "TODO: add schema-driven parser definitions for common app databases.".to_string(),
        ],
    }
}
