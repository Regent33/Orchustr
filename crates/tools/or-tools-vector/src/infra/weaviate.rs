use super::shared::{expect_ok, load_credential, transport};
use crate::domain::contracts::VectorStoreClient;
use crate::domain::entities::{
    CollectionConfig, DeleteRequest, QueryFilter, UpsertBatch, VectorMatch,
};
use crate::domain::errors::VectorError;
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{Value, json};

const PROVIDER: &str = "weaviate";
const URL_ENV: &str = "WEAVIATE_URL";
const API_KEY_ENV: &str = "WEAVIATE_API_KEY";

#[derive(Clone)]
pub struct WeaviateClient {
    client: reqwest::Client,
    base_url: String,
    api_key: Option<String>,
}

impl WeaviateClient {
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
            rb = rb.header("Authorization", format!("Bearer {key}"));
        }
        rb
    }
}

#[derive(Deserialize)]
struct WeaviateNearVectorResult {
    data: WeaviateData,
}

#[derive(Deserialize)]
struct WeaviateData {
    #[serde(rename = "Get")]
    get: Value,
}

#[async_trait]
impl VectorStoreClient for WeaviateClient {
    fn name(&self) -> &'static str {
        PROVIDER
    }

    async fn ensure_collection(&self, cfg: CollectionConfig) -> Result<(), VectorError> {
        let body = json!({
            "class": cfg.name,
            "vectorizer": "none",
            "vectorIndexConfig": { "distance": cfg.distance.as_str() },
        });
        let resp = self
            .req(reqwest::Method::POST, "/v1/schema")
            .json(&body)
            .send()
            .await
            .map_err(|e| transport(PROVIDER, e))?;
        // 422 = already exists; treat as success
        if resp.status().as_u16() != 422 {
            expect_ok(PROVIDER, resp).await?;
        }
        Ok(())
    }

    async fn upsert(&self, batch: UpsertBatch) -> Result<(), VectorError> {
        for item in batch.items {
            let body = json!({
                "class": batch.collection,
                "id": item.id,
                "vector": item.vector,
                "properties": item.metadata,
            });
            let resp = self
                .req(reqwest::Method::POST, "/v1/objects")
                .json(&body)
                .send()
                .await
                .map_err(|e| transport(PROVIDER, e))?;
            let status = resp.status().as_u16();
            if status != 200 && status != 201 {
                let body = resp.text().await.unwrap_or_default();
                return Err(VectorError::Upstream {
                    provider: PROVIDER.into(),
                    status,
                    body,
                });
            }
        }
        Ok(())
    }

    async fn delete(&self, req: DeleteRequest) -> Result<(), VectorError> {
        for id in req.ids {
            let resp = self
                .req(
                    reqwest::Method::DELETE,
                    &format!("/v1/objects/{}/{}", req.collection, id),
                )
                .send()
                .await
                .map_err(|e| transport(PROVIDER, e))?;
            expect_ok(PROVIDER, resp).await?;
        }
        Ok(())
    }

    async fn query(&self, filter: QueryFilter) -> Result<Vec<VectorMatch>, VectorError> {
        let gql = format!(
            r#"{{ Get {{ {class} (nearVector: {{vector: {vec_json}}}, limit: {limit}) {{ _additional {{ id distance }} }} }} }}"#,
            class = filter.collection,
            vec_json = serde_json::to_string(&filter.vector).unwrap_or_default(),
            limit = filter.top_k,
        );
        let resp = self
            .req(reqwest::Method::POST, "/v1/graphql")
            .json(&json!({ "query": gql }))
            .send()
            .await
            .map_err(|e| transport(PROVIDER, e))?;
        let parsed: WeaviateNearVectorResult =
            resp.json().await.map_err(|e| VectorError::Serialization {
                provider: PROVIDER.into(),
                reason: e.to_string(),
            })?;
        let items = parsed
            .data
            .get
            .get(&filter.collection)
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();
        Ok(items
            .into_iter()
            .map(|obj| {
                let add = obj.get("_additional").cloned().unwrap_or(Value::Null);
                VectorMatch {
                    id: add
                        .get("id")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string(),
                    score: 1.0 - add.get("distance").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32,
                    metadata: obj,
                }
            })
            .collect())
    }
}
