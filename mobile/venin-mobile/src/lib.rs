use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MobileBackupDescriptor {
    pub platform: String,
    pub root_path: String,
    pub manifest_path: Option<String>,
}

impl MobileBackupDescriptor {
    pub fn ios(root_path: impl Into<String>) -> Self {
        let root_path = root_path.into();
        Self {
            platform: "ios".to_string(),
            manifest_path: Some(format!("{root_path}/Manifest.db")),
            root_path,
        }
    }
}
