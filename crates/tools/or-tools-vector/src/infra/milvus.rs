use super::shared::{decode, expect_ok, load_credential, transport};
use crate::domain::contracts::VectorStoreClient;
use crate::domain::entities::{
    CollectionConfig, DeleteRequest, QueryFilter, UpsertBatch, VectorMatch,
};
use crate::domain::errors::VectorError;
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{Value, json};

const PROVIDER: &str = "milvus";
const URL_ENV: &str = "MILVUS_URL";
const TOKEN_ENV: &str = "MILVUS_TOKEN";

/// Milvus REST API v2 client (Milvus 2.4+).
#[derive(Clone)]
pub struct MilvusClient {
    client: reqwest::Client,
    base_url: String,
    token: Option<String>,
}

impl MilvusClient {
    pub fn from_env() -> Result<Self, VectorError> {
        Ok(Self {
            client: reqwest::Client::new(),
            base_url: load_credential(URL_ENV)?,
            token: std::env::var(TOKEN_ENV).ok(),
        })
    }

    #[must_use]
    pub fn with_config(
        client: reqwest::Client,
        base_url: impl Into<String>,
        token: Option<String>,
    ) -> Self {
        Self {
            client,
            base_url: base_url.into(),
            token,
        }
    }

    fn req(&self, method: reqwest::Method, path: &str) -> reqwest::RequestBuilder {
        let url = format!(
            "{}/v2/vectordb{}",
            self.base_url.trim_end_matches('/'),
            path
        );
        let mut rb = self.client.request(method, url);
        if let Some(tok) = &self.token {
            rb = rb.header("Authorization", format!("Bearer {tok}"));
        }
        rb
    }
}

#[derive(Deserialize)]
struct MilvusQueryResult {
    data: Vec<MilvusMatch>,
}

#[derive(Deserialize)]
struct MilvusMatch {
    id: Value,
    distance: f32,
    #[serde(rename = "$meta", default)]
    meta: Value,
}

#[async_trait]
impl VectorStoreClient for MilvusClient {
    fn name(&self) -> &'static str {
        PROVIDER
    }

    async fn ensure_collection(&self, cfg: CollectionConfig) -> Result<(), VectorError> {
        let body = json!({
            "collectionName": cfg.name,
            "dimension": cfg.dimension,
            "metricType": cfg.distance.as_str().to_uppercase(),
        });
        let resp = self
            .req(reqwest::Method::POST, "/collections/create")
            .json(&body)
            .send()
            .await
            .map_err(|e| transport(PROVIDER, e))?;
        expect_ok(PROVIDER, resp).await
    }

    async fn upsert(&self, batch: UpsertBatch) -> Result<(), VectorError> {
        let data: Vec<Value> = batch
            .items
            .into_iter()
            .map(|item| {
                json!({
                    "id": item.id,
                    "vector": item.vector,
                    "metadata": item.metadata,
                })
            })
            .collect();
        let body = json!({ "collectionName": batch.collection, "data": data });
        let resp = self
            .req(reqwest::Method::POST, "/entities/upsert")
            .json(&body)
            .send()
            .await
            .map_err(|e| transport(PROVIDER, e))?;
        expect_ok(PROVIDER, resp).await
    }

    async fn delete(&self, req: DeleteRequest) -> Result<(), VectorError> {
        let body = json!({ "collectionName": req.collection, "ids": req.ids });
        let resp = self
            .req(reqwest::Method::POST, "/entities/delete")
            .json(&body)
            .send()
            .await
            .map_err(|e| transport(PROVIDER, e))?;
        expect_ok(PROVIDER, resp).await
    }

    async fn query(&self, filter: QueryFilter) -> Result<Vec<VectorMatch>, VectorError> {
        let body = json!({
            "collectionName": filter.collection,
            "data": [filter.vector],
            "limit": filter.top_k,
            "outputFields": ["*"],
        });
        let resp = self
            .req(reqwest::Method::POST, "/entities/search")
            .json(&body)
            .send()
            .await
            .map_err(|e| transport(PROVIDER, e))?;
        let parsed: MilvusQueryResult = decode(PROVIDER, resp).await?;
        Ok(parsed
            .data
            .into_iter()
            .map(|m| VectorMatch {
                id: m.id.as_str().unwrap_or("").to_string(),
                score: m.distance,
                metadata: m.meta,
            })
            .collect())
    }
}
