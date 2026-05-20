use venin_core::{InputType, OutputFormat, ToolKind, ToolMetadata, ToolStatus};

pub fn metadata() -> ToolMetadata {
    ToolMetadata {
        id: "mobile.ios_backup".to_string(),
        display_name: "iOS Backup Parser".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        kind: ToolKind::Mobile,
        status: ToolStatus::Scaffold,
        supported_artifacts: vec![
            "Manifest.db".to_string(),
            "Info.plist".to_string(),
            "iTunes/Finder backup directories".to_string(),
        ],
        input_types: vec![InputType::MobileBackup, InputType::Directory],
        output_formats: vec![OutputFormat::Json, OutputFormat::Csv, OutputFormat::TimelineJson],
        output_schemas: vec![],
        evidence_notes: vec!["TODO: map domain-relative paths through Manifest.db without modifying backup contents.".to_string()],
    }
}
