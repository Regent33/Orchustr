use crate::domain::contracts::RecallStore;
use crate::domain::entities::{MemoryKind, RecallEntry};
use crate::domain::errors::RecallError;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::{Pool, Row, Sqlite};
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct SqliteRecallStore {
    pool: Pool<Sqlite>,
}

impl SqliteRecallStore {
    pub async fn connect(database_url: &str) -> Result<Self, RecallError> {
        let options = SqliteConnectOptions::from_str(database_url)
            .map_err(|error| RecallError::Storage(error.to_string()))?
            .pragma("journal_mode", "WAL")
            .pragma("busy_timeout", "5000");
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect_with(options)
            .await
            .map_err(|error| RecallError::Storage(error.to_string()))?;
        let store = Self { pool };
        store.migrate().await?;
        Ok(store)
    }

    async fn migrate(&self) -> Result<(), RecallError> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS recall_entries (
                id TEXT PRIMARY KEY,
                kind TEXT NOT NULL,
                content TEXT NOT NULL,
                metadata TEXT NOT NULL
            )",
        )
        .execute(&self.pool)
        .await
        .map_err(|error| RecallError::Storage(error.to_string()))?;
        Ok(())
    }
}

impl RecallStore for SqliteRecallStore {
    async fn store(&self, entry: RecallEntry) -> Result<(), RecallError> {
        let metadata = serde_json::to_string(&entry.metadata)
            .map_err(|error| RecallError::Serialization(error.to_string()))?;
        sqlx::query(
            "INSERT OR REPLACE INTO recall_entries (id, kind, content, metadata)
             VALUES (?1, ?2, ?3, ?4)",
        )
        .bind(&entry.id)
        .bind(format_kind(&entry.kind))
        .bind(&entry.content)
        .bind(metadata)
        .execute(&self.pool)
        .await
        .map_err(|error| RecallError::Storage(error.to_string()))?;
        Ok(())
    }

    async fn list(&self, kind: MemoryKind) -> Result<Vec<RecallEntry>, RecallError> {
        let rows = sqlx::query(
            "SELECT id, kind, content, metadata FROM recall_entries WHERE kind = ?1 ORDER BY id",
        )
        .bind(format_kind(&kind))
        .fetch_all(&self.pool)
        .await
        .map_err(|error| RecallError::Storage(error.to_string()))?;
        rows.into_iter()
            .map(|row| {
                let metadata =
                    serde_json::from_str::<serde_json::Value>(&row.get::<String, _>("metadata"))
                        .map_err(|error| RecallError::Serialization(error.to_string()))?;
                Ok(RecallEntry {
                    id: row.get("id"),
                    kind: parse_kind(&row.get::<String, _>("kind"))?,
                    content: row.get("content"),
                    metadata,
                })
            })
            .collect()
    }
}

fn format_kind(kind: &MemoryKind) -> &'static str {
    match kind {
        MemoryKind::ShortTerm => "short_term",
        MemoryKind::LongTerm => "long_term",
        MemoryKind::Episodic => "episodic",
    }
}

fn parse_kind(raw: &str) -> Result<MemoryKind, RecallError> {
    match raw {
        "short_term" => Ok(MemoryKind::ShortTerm),
        "long_term" => Ok(MemoryKind::LongTerm),
        "episodic" => Ok(MemoryKind::Episodic),
        other => Err(RecallError::Storage(format!(
            "unknown memory kind: {other}"
        ))),
    }
}
