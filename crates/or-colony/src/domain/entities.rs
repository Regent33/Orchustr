use or_core::DynState;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ColonyMember {
    pub name: String,
    pub role: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ColonyMessage {
    pub from: String,
    pub to: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ColonyResult {
    pub summary: String,
    pub state: DynState,
    pub transcript: Vec<ColonyMessage>,
}
