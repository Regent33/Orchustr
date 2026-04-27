//! Bridge entry point for `or-tools-vector` (vector store clients).

use super::helpers::{
    block_on, from_field, get_str, invocation, json_value, required_str, unsupported,
    unsupported_provider,
};
use crate::domain::errors::BridgeError;
use or_tools_vector::infra::{
    chroma::ChromaClient, milvus::MilvusClient, pgvector::PgVectorClient, pinecone::PineconeClient,
    qdrant::QdrantClient, weaviate::WeaviateClient,
};
use or_tools_vector::{
    CollectionConfig, DeleteRequest, QueryFilter, UpsertBatch, VectorStoreClient,
};
use serde_json::{Value, json};

pub(crate) fn invoke(operation: &str, payload: Value) -> Result<Value, BridgeError> {
    let provider_name = required_str(&payload, "provider", "or-tools-vector", operation)?;
    let client = build_vector_client(provider_name, payload.get("config"))?;
    match operation {
        "ensure_collection" => {
            let cfg: CollectionConfig = from_field(&payload, "data", "or-tools-vector", operation)?;
            block_on("or-tools-vector", operation, client.ensure_collection(cfg))?;
            Ok(json!({ "status": "ok" }))
        }
        "upsert" => {
            let batch: UpsertBatch = from_field(&payload, "data", "or-tools-vector", operation)?;
            block_on("or-tools-vector", operation, client.upsert(batch))?;
            Ok(json!({ "status": "ok" }))
        }
        "delete" => {
            let req: DeleteRequest = from_field(&payload, "data", "or-tools-vector", operation)?;
            block_on("or-tools-vector", operation, client.delete(req))?;
            Ok(json!({ "status": "ok" }))
        }
        "query" => {
            let filter: QueryFilter = from_field(&payload, "data", "or-tools-vector", operation)?;
            block_on("or-tools-vector", operation, client.query(filter)).and_then(json_value)
        }
        _ => Err(unsupported("or-tools-vector", operation)),
    }
}

fn build_vector_client(
    provider: &str,
    config: Option<&Value>,
) -> Result<Box<dyn VectorStoreClient>, BridgeError> {
    let cfg = config.and_then(Value::as_object);
    let client = reqwest::Client::new();
    let store: Box<dyn VectorStoreClient> = match provider {
        "pinecone" => Box::new(
            if let (Some(host), Some(api_key)) = (
                cfg.and_then(|v| get_str(v, "host")),
                cfg.and_then(|v| get_str(v, "api_key")),
            ) {
                PineconeClient::with_config(client, host, api_key)
            } else {
                PineconeClient::from_env()
                    .map_err(|error| invocation("or-tools-vector", "invoke", error))?
            },
        ),
        "weaviate" => Box::new(
            if let Some(base_url) = cfg.and_then(|v| get_str(v, "base_url")) {
                WeaviateClient::with_config(
                    client,
                    base_url,
                    cfg.and_then(|v| get_str(v, "api_key")).map(str::to_owned),
                )
            } else {
                WeaviateClient::from_env()
                    .map_err(|error| invocation("or-tools-vector", "invoke", error))?
            },
        ),
        "qdrant" => Box::new(
            if let Some(base_url) = cfg.and_then(|v| get_str(v, "base_url")) {
                QdrantClient::with_config(
                    client,
                    base_url,
                    cfg.and_then(|v| get_str(v, "api_key")).map(str::to_owned),
                )
            } else {
                QdrantClient::from_env()
                    .map_err(|error| invocation("or-tools-vector", "invoke", error))?
            },
        ),
        "chroma" => Box::new(
            if let Some(base_url) = cfg.and_then(|v| get_str(v, "base_url")) {
                ChromaClient::with_config(client, base_url)
            } else {
                ChromaClient::from_env()
            },
        ),
        "milvus" => Box::new(
            if let Some(base_url) = cfg.and_then(|v| get_str(v, "base_url")) {
                MilvusClient::with_config(
                    client,
                    base_url,
                    cfg.and_then(|v| get_str(v, "token")).map(str::to_owned),
                )
            } else {
                MilvusClient::from_env()
                    .map_err(|error| invocation("or-tools-vector", "invoke", error))?
            },
        ),
        "pgvector" => {
            if config.is_some() {
                return Err(BridgeError::InvalidInput(
                    "pgvector currently uses environment-based connection setup only".into(),
                ));
            }
            Box::new(block_on(
                "or-tools-vector",
                "connect",
                PgVectorClient::from_env(),
            )?)
        }
        other => return Err(unsupported_provider("or-tools-vector", other)),
    };
    Ok(store)
}
