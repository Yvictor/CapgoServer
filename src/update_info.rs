use poem_openapi::Object;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Object)]
pub struct UpdateInfo {
    pub version: String,
    pub url: String,
    // pub release_notes: String,
}

impl UpdateInfo {
    pub fn new(version: &str, url: &str, release_notes: &str) -> Self {
        UpdateInfo {
            version: version.to_string(),
            url: url.to_string(),
            // release_notes: release_notes.to_string(),
        }
    }
}
