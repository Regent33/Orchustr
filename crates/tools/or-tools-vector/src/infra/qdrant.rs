use super::shared::{decode, expect_ok, load_credential, transport};
use crate::domain::contracts::VectorStoreClient;
use crate::domain::entities::{
    CollectionConfig, DeleteRequest, Distance, QueryFilter, UpsertBatch, VectorMatch,
};
use crate::domain::errors::VectorError;
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{Value, json};

const PROVIDER: &str = "qdrant";
const URL_ENV: &str = "QDRANT_URL";
const API_KEY_ENV: &str = "QDRANT_API_KEY";

#[derive(Clone)]
pub struct QdrantClient {
    client: reqwest::Client,
    base_url: String,
    api_key: Option<String>,
}

impl QdrantClient {
    pub fn from_env() -> Result<Self, VectorError> {
        Ok(Self {
            client: reqwest::Client::new(),
            base_url: load_credential(URL_ENV)?,
            api_key: std::env::var(API_KEY_ENV).ok(),
        })
    }

    #[must_use]
    pub fn with_config(
        client: reqwest::Client,
        base_url: impl Into<String>,
        api_key: Option<String>,
    ) -> Self {
        Self {
            client,
            base_url: base_url.into(),
            api_key,
        }
    }

    fn req(&self, method: reqwest::Method, path: &str) -> reqwest::RequestBuilder {
        let url = format!("{}{}", self.base_url.trim_end_matches('/'), path);
        let mut rb = self.client.request(method, url);
        if let Some(key) = &self.api_key {
            rb = rb.header("api-key", key);
        }
        rb
    }
}

#[derive(Deserialize)]
struct QdrantQueryResult {
    result: Vec<QdrantMatch>,
}

#[derive(Deserialize)]
struct QdrantMatch {
    id: Value,
    score: f32,
    #[serde(default)]
    payload: Value,
}

#[async_trait]
impl VectorStoreClient for QdrantClient {
    fn name(&self) -> &'static str {
        PROVIDER
    }

    async fn ensure_collection(&self, cfg: CollectionConfig) -> Result<(), VectorError> {
        let qdrant_distance = match cfg.distance {
            Distance::Cosine => "Cosine",
            Distance::Euclidean => "Euclid",
            Distance::DotProduct => "Dot",
        };
        let body = json!({
            "vectors": { "size": cfg.dimension, "distance": qdrant_distance }
        });
        let resp = self
            .req(reqwest::Method::PUT, &format!("/collections/{}", cfg.name))
            .json(&body)
            .send()
            .await
            .map_err(|e| transport(PROVIDER, e))?;
        expect_ok(PROVIDER, resp).await
    }

    async fn upsert(&self, batch: UpsertBatch) -> Result<(), VectorError> {
        let points: Vec<Value> = batch
            .items
            .into_iter()
            .map(|item| {
                json!({
                    "id": item.id,
                    "vector": item.vector,
                    "payload": item.metadata,
                })
            })
            .collect();
        let body = json!({ "points": points });
        let resp = self
            .req(
                reqwest::Method::PUT,
                &format!("/collections/{}/points", batch.collection),
            )
            .json(&body)
            .send()
            .await
            .map_err(|e| transport(PROVIDER, e))?;
        expect_ok(PROVIDER, resp).await
    }

    async fn delete(&self, req: DeleteRequest) -> Result<(), VectorError> {
        let body = json!({ "points": req.ids });
        let resp = self
            .req(
                reqwest::Method::POST,
                &format!("/collections/{}/points/delete", req.collection),
            )
            .json(&body)
            .send()
            .await
            .map_err(|e| transport(PROVIDER, e))?;
        expect_ok(PROVIDER, resp).await
    }

    async fn query(&self, filter: QueryFilter) -> Result<Vec<VectorMatch>, VectorError> {
        let body = json!({
            "vector": filter.vector,
            "limit": filter.top_k,
            "with_payload": true,
            "filter": filter.filter,
        });
        let resp = self
            .req(
                reqwest::Method::POST,
                &format!("/collections/{}/points/search", filter.collection),
            )
            .json(&body)
            .send()
            .await
            .map_err(|e| transport(PROVIDER, e))?;
        let parsed: QdrantQueryResult = decode(PROVIDER, resp).await?;
        Ok(parsed
            .result
            .into_iter()
            .map(|m| VectorMatch {
                id: m.id.as_str().unwrap_or("").to_string(),
                score: m.score,
                metadata: m.payload,
            })
            .collect())
    }
}
