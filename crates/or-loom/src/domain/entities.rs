use crate::domain::errors::LoomError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NodeResult<T> {
    Advance(T),
    Branch { state: T, next: String },
    Pause { checkpoint_id: String, state: T },
}

impl<T> NodeResult<T> {
    pub fn advance(state: T) -> Result<Self, LoomError> {
        Ok(Self::Advance(state))
    }

    pub fn branch(state: T, next: impl Into<String>) -> Result<Self, LoomError> {
        Ok(Self::Branch {
            state,
            next: next.into(),
        })
    }

    pub fn pause(checkpoint_id: impl Into<String>, state: T) -> Result<Self, LoomError> {
        Ok(Self::Pause {
            checkpoint_id: checkpoint_id.into(),
            state,
        })
    }
}
