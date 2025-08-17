use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Lair {
    pub name: String,
    pub notes: String,
    pub action: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_portrait: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_portrait: Option<String>,
}
