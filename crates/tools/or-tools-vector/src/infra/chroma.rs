use super::shared::{decode, expect_ok, transport};
use crate::domain::contracts::VectorStoreClient;
use crate::domain::entities::{
    CollectionConfig, DeleteRequest, QueryFilter, UpsertBatch, VectorMatch,
};
use crate::domain::errors::VectorError;
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{Value, json};

const PROVIDER: &str = "chroma";
const DEFAULT_URL: &str = "http://localhost:8000";
const URL_ENV: &str = "CHROMA_URL";

/// ChromaDB HTTP client. Defaults to `http://localhost:8000` (in-process dev).
#[derive(Clone)]
pub struct ChromaClient {
    client: reqwest::Client,
    base_url: String,
}

impl ChromaClient {
    pub fn from_env() -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url: std::env::var(URL_ENV).unwrap_or_else(|_| DEFAULT_URL.to_string()),
        }
    }

    #[must_use]
    pub fn with_config(client: reqwest::Client, base_url: impl Into<String>) -> Self {
        Self {
            client,
            base_url: base_url.into(),
        }
    }

    fn url(&self, path: &str) -> String {
        format!("{}/api/v1{}", self.base_url.trim_end_matches('/'), path)
    }
}

#[derive(Deserialize)]
struct ChromaCollection {
    id: String,
}

#[derive(Deserialize)]
struct ChromaQueryResult {
    ids: Vec<Vec<String>>,
    distances: Vec<Vec<f32>>,
    #[serde(default)]
    metadatas: Vec<Vec<Value>>,
}

#[async_trait]
impl VectorStoreClient for ChromaClient {
    fn name(&self) -> &'static str {
        PROVIDER
    }

    async fn ensure_collection(&self, cfg: CollectionConfig) -> Result<(), VectorError> {
        let body = json!({ "name": cfg.name, "metadata": { "hnsw:space": cfg.distance.as_str() } });
        let resp = self
            .client
            .post(self.url("/collections"))
            .json(&body)
            .send()
            .await
            .map_err(|e| transport(PROVIDER, e))?;
        let status = resp.status().as_u16();
        if status != 200 && status != 201 && status != 409 {
            let body = resp.text().await.unwrap_or_default();
            return Err(VectorError::Upstream {
                provider: PROVIDER.into(),
                status,
                body,
            });
        }
        Ok(())
    }

    async fn upsert(&self, batch: UpsertBatch) -> Result<(), VectorError> {
        let coll_resp = self
            .client
            .get(self.url(&format!("/collections/{}", batch.collection)))
            .send()
            .await
            .map_err(|e| transport(PROVIDER, e))?;
        let coll: ChromaCollection = decode(PROVIDER, coll_resp).await?;
        let (ids, embeddings, metadatas): (Vec<_>, Vec<_>, Vec<_>) = batch
            .items
            .into_iter()
            .map(|i| (i.id, i.vector, i.metadata))
            .multiunzip3();
        let body = json!({ "ids": ids, "embeddings": embeddings, "metadatas": metadatas });
        let resp = self
            .client
            .post(self.url(&format!("/collections/{}/upsert", coll.id)))
            .json(&body)
            .send()
            .await
            .map_err(|e| transport(PROVIDER, e))?;
        expect_ok(PROVIDER, resp).await
    }

    async fn delete(&self, req: DeleteRequest) -> Result<(), VectorError> {
        let coll_resp = self
            .client
            .get(self.url(&format!("/collections/{}", req.collection)))
            .send()
            .await
            .map_err(|e| transport(PROVIDER, e))?;
        let coll: ChromaCollection = decode(PROVIDER, coll_resp).await?;
        let body = json!({ "ids": req.ids });
        let resp = self
            .client
            .post(self.url(&format!("/collections/{}/delete", coll.id)))
            .json(&body)
            .send()
            .await
            .map_err(|e| transport(PROVIDER, e))?;
        expect_ok(PROVIDER, resp).await
    }

    async fn query(&self, filter: QueryFilter) -> Result<Vec<VectorMatch>, VectorError> {
        let coll_resp = self
            .client
            .get(self.url(&format!("/collections/{}", filter.collection)))
            .send()
            .await
            .map_err(|e| transport(PROVIDER, e))?;
        let coll: ChromaCollection = decode(PROVIDER, coll_resp).await?;
        let body = json!({
            "query_embeddings": [filter.vector],
            "n_results": filter.top_k,
            "include": ["metadatas", "distances"],
        });
        let resp = self
            .client
            .post(self.url(&format!("/collections/{}/query", coll.id)))
            .json(&body)
            .send()
            .await
            .map_err(|e| transport(PROVIDER, e))?;
        let parsed: ChromaQueryResult = decode(PROVIDER, resp).await?;
        let ids = parsed.ids.into_iter().next().unwrap_or_default();
        let dists = parsed.distances.into_iter().next().unwrap_or_default();
        let metas = parsed.metadatas.into_iter().next().unwrap_or_default();
        Ok(ids
            .into_iter()
            .enumerate()
            .map(|(i, id)| VectorMatch {
                id,
                score: 1.0 - dists.get(i).copied().unwrap_or(0.0),
                metadata: metas.get(i).cloned().unwrap_or(Value::Null),
            })
            .collect())
    }
}

trait Multiunzip3<A, B, C>: Sized {
    fn multiunzip3(self) -> (Vec<A>, Vec<B>, Vec<C>);
}

impl<I, A, B, C> Multiunzip3<A, B, C> for I
where
    I: Iterator<Item = (A, B, C)>,
{
    fn multiunzip3(self) -> (Vec<A>, Vec<B>, Vec<C>) {
        let mut va = Vec::new();
        let mut vb = Vec::new();
        let mut vc = Vec::new();
        for (a, b, c) in self {
            va.push(a);
            vb.push(b);
            vc.push(c);
        }
        (va, vb, vc)
    }
}
