use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CheckpointRecord<T> {
    pub checkpoint_id: String,
    pub resume_from: String,
    pub state: T,
}
