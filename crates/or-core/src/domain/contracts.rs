#![allow(async_fn_in_trait)]

use crate::domain::entities::VectorRecord;
use crate::domain::errors::CoreError;
use serde_json::Value;
use std::collections::HashMap;

pub type DynState = HashMap<String, Value>;

pub trait OrchState:
    Clone + Send + Sync + serde::Serialize + serde::de::DeserializeOwned + 'static
{
    fn merge(current: &Self, patch: Self) -> Self {
        let _ = current;
        patch
    }
}

impl OrchState for DynState {}

#[cfg_attr(test, mockall::automock)]
pub trait PersistenceBackend: Send + Sync + 'static {
    async fn save_state(&self, scope: &str, state: Value) -> Result<(), CoreError>;
    async fn load_state(&self, scope: &str) -> Result<Option<Value>, CoreError>;
}

#[cfg_attr(test, mockall::automock)]
pub trait VectorStore: Send + Sync + 'static {
    async fn upsert(&self, id: &str, vector: Vec<f32>, metadata: Value) -> Result<(), CoreError>;
    async fn query(&self, vector: Vec<f32>, limit: usize) -> Result<Vec<VectorRecord>, CoreError>;
}
