use venin_core::{InputType, OutputFormat, ToolKind, ToolMetadata, ToolStatus};

pub fn metadata() -> ToolMetadata {
    ToolMetadata {
        id: "windows.prefetch".to_string(),
        display_name: "Windows Prefetch Parser".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        kind: ToolKind::Parser,
        status: ToolStatus::Scaffold,
        supported_artifacts: vec!["Windows Prefetch .pf files".to_string()],
        input_types: vec![InputType::File, InputType::Directory],
        output_formats: vec![
            OutputFormat::Json,
            OutputFormat::Csv,
            OutputFormat::TimelineJson,
        ],
        output_schemas: vec![],
        evidence_notes: vec![
            "TODO: support SCCA versions, run counts, execution times, and file references."
                .to_string(),
        ],
    }
}
