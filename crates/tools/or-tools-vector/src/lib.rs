//! Vector-store tool implementations for Pinecone, Weaviate, Qdrant, Chroma,
//! Milvus, and PGVector. Each backend is feature-gated so callers only
//! compile what they use.

pub mod application;
pub mod domain;
pub mod infra;

pub use application::orchestrators::RagOrchestrator;
pub use domain::contracts::VectorStoreClient;
pub use domain::entities::{
    CollectionConfig, DeleteRequest, Distance, QueryFilter, UpsertBatch, UpsertItem, VectorMatch,
};
pub use domain::errors::VectorError;
