use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IosBackupManifest {
    pub backup_root: String,
    pub manifest_db: String,
    pub info_plist: String,
}

impl IosBackupManifest {
    pub fn from_backup_root(root: impl Into<String>) -> Self {
        let root = root.into();
        Self {
            manifest_db: format!("{root}/Manifest.db"),
            info_plist: format!("{root}/Info.plist"),
            backup_root: root,
        }
    }
}
