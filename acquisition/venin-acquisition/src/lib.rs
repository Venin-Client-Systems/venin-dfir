use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcquisitionPlan {
    pub id: String,
    pub target: String,
    pub method: String,
    pub notes: Vec<String>,
}

impl AcquisitionPlan {
    pub fn scaffold(id: impl Into<String>, target: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            target: target.into(),
            method: "TODO: platform-specific acquisition method".to_string(),
            notes: vec![
                "Preserve original evidence media where possible.".to_string(),
                "Record acquisition commands, tool versions, hashes, and operator notes."
                    .to_string(),
            ],
        }
    }
}
