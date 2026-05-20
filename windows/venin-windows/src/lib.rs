use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowsArtifactSource {
    pub artifact_type: String,
    pub expected_paths: Vec<String>,
}

pub fn default_sources() -> Vec<WindowsArtifactSource> {
    vec![
        WindowsArtifactSource {
            artifact_type: "prefetch".to_string(),
            expected_paths: vec![r"C:\Windows\Prefetch".to_string()],
        },
        WindowsArtifactSource {
            artifact_type: "recent_files".to_string(),
            expected_paths: vec![r"%APPDATA%\Microsoft\Windows\Recent".to_string()],
        },
    ]
}
