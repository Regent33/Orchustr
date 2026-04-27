use crate::domain::contracts::VectorStoreClient;
use crate::domain::entities::{
    CollectionConfig, DeleteRequest, QueryFilter, UpsertBatch, VectorMatch,
};
use crate::domain::errors::VectorError;
use async_trait::async_trait;
use serde_json::Value;
use sqlx::PgPool;

const PROVIDER: &str = "pgvector";
const DB_URL_ENV: &str = "PGVECTOR_DATABASE_URL";

/// PGVector backend — wraps a `sqlx::PgPool`. The collection name maps to a
/// Postgres table with columns `(id TEXT PRIMARY KEY, embedding VECTOR(n),
/// metadata JSONB)`. Run `CREATE EXTENSION IF NOT EXISTS vector` first.
#[derive(Clone)]
pub struct PgVectorClient {
    pool: PgPool,
}

impl PgVectorClient {
    pub async fn from_env() -> Result<Self, VectorError> {
        let url = std::env::var(DB_URL_ENV)
            .map_err(|_| VectorError::MissingCredential(DB_URL_ENV.into()))?;
        let pool = PgPool::connect(&url)
            .await
            .map_err(|e| VectorError::Transport {
                provider: PROVIDER.into(),
                reason: e.to_string(),
            })?;
        Ok(Self { pool })
    }

    #[must_use]
    pub fn with_pool(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl VectorStoreClient for PgVectorClient {
    fn name(&self) -> &'static str {
        PROVIDER
    }

    async fn ensure_collection(&self, cfg: CollectionConfig) -> Result<(), VectorError> {
        let sql = format!(
            r#"CREATE TABLE IF NOT EXISTS "{}" (
                id TEXT PRIMARY KEY,
                embedding VECTOR({}),
                metadata JSONB DEFAULT '{{}}'::jsonb
            )"#,
            cfg.name, cfg.dimension
        );
        sqlx::query(&sql)
            .execute(&self.pool)
            .await
            .map_err(|e| VectorError::Transport {
                provider: PROVIDER.into(),
                reason: e.to_string(),
            })?;
        Ok(())
    }

    async fn upsert(&self, batch: UpsertBatch) -> Result<(), VectorError> {
        for item in batch.items {
            let vec_str = format!(
                "[{}]",
                item.vector
                    .iter()
                    .map(|f| f.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            );
            let meta = item.metadata.to_string();
            let sql = format!(
                r#"INSERT INTO "{}" (id, embedding, metadata)
                   VALUES ($1, $2::vector, $3::jsonb)
                   ON CONFLICT (id) DO UPDATE
                   SET embedding = EXCLUDED.embedding, metadata = EXCLUDED.metadata"#,
                batch.collection
            );
            sqlx::query(&sql)
                .bind(&item.id)
                .bind(&vec_str)
                .bind(&meta)
                .execute(&self.pool)
                .await
                .map_err(|e| VectorError::Transport {
                    provider: PROVIDER.into(),
                    reason: e.to_string(),
                })?;
        }
        Ok(())
    }

    async fn delete(&self, req: DeleteRequest) -> Result<(), VectorError> {
        let sql = format!(r#"DELETE FROM "{}" WHERE id = ANY($1)"#, req.collection);
        sqlx::query(&sql)
            .bind(&req.ids)
            .execute(&self.pool)
            .await
            .map_err(|e| VectorError::Transport {
                provider: PROVIDER.into(),
                reason: e.to_string(),
            })?;
        Ok(())
    }

    async fn query(&self, filter: QueryFilter) -> Result<Vec<VectorMatch>, VectorError> {
        let vec_str = format!(
            "[{}]",
            filter
                .vector
                .iter()
                .map(|f| f.to_string())
                .collect::<Vec<_>>()
                .join(",")
        );
        let sql = format!(
            r#"SELECT id, metadata, 1 - (embedding <=> $1::vector) AS score
               FROM "{}" ORDER BY embedding <=> $1::vector LIMIT $2"#,
            filter.collection
        );
        let rows = sqlx::query(&sql)
            .bind(&vec_str)
            .bind(filter.top_k as i64)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| VectorError::Transport {
                provider: PROVIDER.into(),
                reason: e.to_string(),
            })?;
        let results = rows
            .into_iter()
            .map(|row| {
                use sqlx::Row;
                let id: String = row.try_get("id").unwrap_or_default();
                let score: f64 = row.try_get("score").unwrap_or(0.0);
                let meta: Value = row
                    .try_get::<serde_json::Value, _>("metadata")
                    .unwrap_or(Value::Null);
                VectorMatch {
                    id,
                    score: score as f32,
                    metadata: meta,
                }
            })
            .collect();
        Ok(results)
    }
}
