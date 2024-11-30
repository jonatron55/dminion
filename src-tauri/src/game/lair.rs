use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lair {
    pub name: String,
    pub notes: String,
    pub portrait: PathBuf,
}
