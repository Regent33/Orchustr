use crate::domain::entities::CheckpointRecord;
use crate::domain::errors::CheckpointError;
use or_core::{OrchState, PersistenceBackend};

#[derive(Debug, Clone)]
pub struct CheckpointGate<B> {
    backend: B,
    graph_id: String,
}

impl<B> CheckpointGate<B> {
    #[must_use]
    pub fn new(graph_id: impl Into<String>, backend: B) -> Self {
        Self {
            backend,
            graph_id: graph_id.into(),
        }
    }
}

impl<B> CheckpointGate<B>
where
    B: PersistenceBackend,
{
    pub async fn pause<T: OrchState>(
        &self,
        checkpoint_id: &str,
        resume_from: &str,
        state: &T,
    ) -> Result<(), CheckpointError> {
        let record = CheckpointRecord {
            checkpoint_id: checkpoint_id.to_owned(),
            resume_from: resume_from.to_owned(),
            state: state.clone(),
        };
        let value = serde_json::to_value(record)
            .map_err(|error| CheckpointError::Serialization(error.to_string()))?;
        self.backend
            .save_state(&self.storage_key(checkpoint_id), value)
            .await
            .map_err(|error| CheckpointError::Storage(error.to_string()))
    }

    pub async fn resume<T: OrchState>(
        &self,
        checkpoint_id: &str,
    ) -> Result<CheckpointRecord<T>, CheckpointError> {
        let value = self
            .backend
            .load_state(&self.storage_key(checkpoint_id))
            .await
            .map_err(|error| CheckpointError::Storage(error.to_string()))?
            .ok_or_else(|| CheckpointError::MissingCheckpoint(checkpoint_id.to_owned()))?;
        serde_json::from_value(value)
            .map_err(|error| CheckpointError::Serialization(error.to_string()))
    }

    fn storage_key(&self, checkpoint_id: &str) -> String {
        format!("{}:{checkpoint_id}", self.graph_id)
    }
}
