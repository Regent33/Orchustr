#![allow(async_fn_in_trait)]

use crate::domain::entities::{MemoryKind, RecallEntry};
use crate::domain::errors::RecallError;

#[cfg_attr(test, mockall::automock)]
pub trait RecallStore: Send + Sync + 'static {
    async fn store(&self, entry: RecallEntry) -> Result<(), RecallError>;
    async fn list(&self, kind: MemoryKind) -> Result<Vec<RecallEntry>, RecallError>;
}
