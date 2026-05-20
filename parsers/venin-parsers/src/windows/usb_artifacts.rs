use venin_core::{InputType, OutputFormat, ToolKind, ToolMetadata, ToolStatus};

pub fn metadata() -> ToolMetadata {
    ToolMetadata {
        id: "windows.usb_artifacts".to_string(),
        display_name: "Windows USB Artefact Parser".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        kind: ToolKind::Parser,
        status: ToolStatus::Scaffold,
        supported_artifacts: vec![
            "SYSTEM registry hive USBSTOR keys".to_string(),
            "setupapi.dev.log".to_string(),
        ],
        input_types: vec![InputType::RegistryHive, InputType::File],
        output_formats: vec![
            OutputFormat::Json,
            OutputFormat::Csv,
            OutputFormat::TimelineJson,
        ],
        output_schemas: vec![],
        evidence_notes: vec![
            "TODO: correlate registry device IDs, setupapi timestamps, and mounted volume data."
                .to_string(),
        ],
    }
}
