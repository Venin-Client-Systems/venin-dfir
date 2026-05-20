use venin_core::{InputType, OutputFormat, ToolKind, ToolMetadata, ToolStatus};

pub fn metadata() -> ToolMetadata {
    ToolMetadata {
        id: "windows.recent_files".to_string(),
        display_name: "Windows Recent Files Parser".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        kind: ToolKind::Parser,
        status: ToolStatus::Scaffold,
        supported_artifacts: vec![
            "RecentDocs registry keys".to_string(),
            "Jump Lists".to_string(),
        ],
        input_types: vec![InputType::RegistryHive, InputType::Directory],
        output_formats: vec![
            OutputFormat::Json,
            OutputFormat::Csv,
            OutputFormat::TimelineJson,
        ],
        output_schemas: vec![],
        evidence_notes: vec![
            "TODO: separate RecentDocs, AutomaticDestinations, and CustomDestinations parsing."
                .to_string(),
        ],
    }
}
