use venin_core::{InputType, OutputFormat, ToolKind, ToolMetadata, ToolStatus};

pub fn metadata() -> ToolMetadata {
    ToolMetadata {
        id: "windows.lnk".to_string(),
        display_name: "Windows Shell Link Parser".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        kind: ToolKind::Parser,
        status: ToolStatus::Scaffold,
        supported_artifacts: vec!["Windows .lnk shell link files".to_string()],
        input_types: vec![InputType::File, InputType::Directory],
        output_formats: vec![
            OutputFormat::Json,
            OutputFormat::Csv,
            OutputFormat::TimelineJson,
        ],
        output_schemas: vec![],
        evidence_notes: vec![
            "TODO: preserve target metadata, MAC times, volume serials, and tracker data."
                .to_string(),
        ],
    }
}
