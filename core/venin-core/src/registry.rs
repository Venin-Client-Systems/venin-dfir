use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ToolRegistryError {
    #[error("tool id already registered: {0}")]
    DuplicateTool(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ToolKind {
    Parser,
    Acquisition,
    Timeline,
    Mobile,
    Utility,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ToolStatus {
    Stable,
    Experimental,
    Scaffold,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InputType {
    File,
    Directory,
    SQLiteDatabase,
    RegistryHive,
    FilesystemImage,
    MobileBackup,
    MemoryImage,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OutputFormat {
    Json,
    Csv,
    TimelineJson,
    TimelineCsv,
    Markdown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SchemaField {
    pub name: String,
    pub field_type: String,
    pub description: String,
    pub required: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OutputSchema {
    pub name: String,
    pub description: String,
    pub fields: Vec<SchemaField>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolMetadata {
    pub id: String,
    pub display_name: String,
    pub version: String,
    pub kind: ToolKind,
    pub status: ToolStatus,
    pub supported_artifacts: Vec<String>,
    pub input_types: Vec<InputType>,
    pub output_formats: Vec<OutputFormat>,
    pub output_schemas: Vec<OutputSchema>,
    pub evidence_notes: Vec<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ToolRegistry {
    tools: BTreeMap<String, ToolMetadata>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, metadata: ToolMetadata) -> Result<(), ToolRegistryError> {
        if self.tools.contains_key(&metadata.id) {
            return Err(ToolRegistryError::DuplicateTool(metadata.id));
        }

        self.tools.insert(metadata.id.clone(), metadata);
        Ok(())
    }

    pub fn get(&self, id: &str) -> Option<&ToolMetadata> {
        self.tools.get(id)
    }

    pub fn list(&self) -> impl Iterator<Item = &ToolMetadata> {
        self.tools.values()
    }

    pub fn by_kind(&self, kind: ToolKind) -> impl Iterator<Item = &ToolMetadata> {
        self.tools.values().filter(move |tool| tool.kind == kind)
    }

    pub fn len(&self) -> usize {
        self.tools.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tools.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_tool() -> ToolMetadata {
        ToolMetadata {
            id: "browser_history.chromium".to_string(),
            display_name: "Chromium History Parser".to_string(),
            version: "0.1.0".to_string(),
            kind: ToolKind::Parser,
            status: ToolStatus::Experimental,
            supported_artifacts: vec!["Chromium History SQLite database".to_string()],
            input_types: vec![InputType::SQLiteDatabase],
            output_formats: vec![OutputFormat::Json],
            output_schemas: vec![],
            evidence_notes: vec![],
        }
    }

    #[test]
    fn rejects_duplicate_tool_ids() {
        let mut registry = ToolRegistry::new();
        registry.register(sample_tool()).unwrap();
        let err = registry.register(sample_tool()).unwrap_err();
        assert!(matches!(err, ToolRegistryError::DuplicateTool(_)));
    }
}
