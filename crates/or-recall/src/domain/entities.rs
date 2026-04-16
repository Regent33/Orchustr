use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum MemoryKind {
    ShortTerm,
    LongTerm,
    Episodic,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RecallEntry {
    pub id: String,
    pub kind: MemoryKind,
    pub content: String,
    pub metadata: serde_json::Value,
}
