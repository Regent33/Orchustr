use super::shared::{decode, expect_ok, load_credential, transport};
use crate::domain::contracts::VectorStoreClient;
use crate::domain::entities::{
    CollectionConfig, DeleteRequest, QueryFilter, UpsertBatch, VectorMatch,
};
use crate::domain::errors::VectorError;
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{Value, json};

const PROVIDER: &str = "pinecone";
const API_KEY_ENV: &str = "PINECONE_API_KEY";
const HOST_ENV: &str = "PINECONE_HOST";

#[derive(Clone)]
pub struct PineconeClient {
    client: reqwest::Client,
    host: String,
    api_key: String,
}

impl PineconeClient {
    pub fn from_env() -> Result<Self, VectorError> {
        Ok(Self {
            client: reqwest::Client::new(),
            host: load_credential(HOST_ENV)?,
            api_key: load_credential(API_KEY_ENV)?,
        })
    }

    #[must_use]
    pub fn with_config(
        client: reqwest::Client,
        host: impl Into<String>,
        api_key: impl Into<String>,
    ) -> Self {
        Self {
            client,
            host: host.into(),
            api_key: api_key.into(),
        }
    }
}

#[derive(Deserialize)]
struct QueryResponse {
    #[serde(default)]
    matches: Vec<PineconeMatch>,
}

#[derive(Deserialize)]
struct PineconeMatch {
    id: String,
    score: f32,
    #[serde(default)]
    metadata: Value,
}

#[async_trait]
impl VectorStoreClient for PineconeClient {
    fn name(&self) -> &'static str {
        PROVIDER
    }

    async fn ensure_collection(&self, _cfg: CollectionConfig) -> Result<(), VectorError> {
        // Pinecone indexes are created via the control-plane API, not inline.
        Ok(())
    }

    async fn upsert(&self, batch: UpsertBatch) -> Result<(), VectorError> {
        let vectors: Vec<Value> = batch
            .items
            .into_iter()
            .map(|item| {
                json!({
                    "id": item.id,
                    "values": item.vector,
                    "metadata": item.metadata,
                })
            })
            .collect();
        let body = json!({ "vectors": vectors });
        let resp = self
            .client
            .post(format!("{}/vectors/upsert", self.host))
            .header("Api-Key", &self.api_key)
            .json(&body)
            .send()
            .await
            .map_err(|e| transport(PROVIDER, e))?;
        expect_ok(PROVIDER, resp).await
    }

    async fn delete(&self, req: DeleteRequest) -> Result<(), VectorError> {
        let body = json!({ "ids": req.ids });
        let resp = self
            .client
            .post(format!("{}/vectors/delete", self.host))
            .header("Api-Key", &self.api_key)
            .json(&body)
            .send()
            .await
            .map_err(|e| transport(PROVIDER, e))?;
        expect_ok(PROVIDER, resp).await
    }

    async fn query(&self, filter: QueryFilter) -> Result<Vec<VectorMatch>, VectorError> {
        let body = json!({
            "vector": filter.vector,
            "topK": filter.top_k,
            "includeMetadata": true,
            "filter": filter.filter,
        });
        let resp = self
            .client
            .post(format!("{}/query", self.host))
            .header("Api-Key", &self.api_key)
            .json(&body)
            .send()
            .await
            .map_err(|e| transport(PROVIDER, e))?;
        let parsed: QueryResponse = decode(PROVIDER, resp).await?;
        Ok(parsed
            .matches
            .into_iter()
            .map(|m| VectorMatch {
                id: m.id,
                score: m.score,
                metadata: m.metadata,
            })
            .collect())
    }
}
