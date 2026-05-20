use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AndroidArtifactSource {
    pub package_name: String,
    pub relative_database_paths: Vec<String>,
}

impl AndroidArtifactSource {
    pub fn new(package_name: impl Into<String>, relative_database_paths: Vec<String>) -> Self {
        Self {
            package_name: package_name.into(),
            relative_database_paths,
        }
    }
}
