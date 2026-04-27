use crate::domain::contracts::FileStore;
use crate::domain::entities::{FileContent, FileEntry};
use crate::domain::errors::FileError;
use async_trait::async_trait;
use tokio::fs;
use tokio::io::AsyncWriteExt;

pub struct LocalFileSystem;

#[async_trait]
impl FileStore for LocalFileSystem {
    async fn read(&self, path: &str) -> Result<FileContent, FileError> {
        let content = fs::read_to_string(path).await.map_err(|e| FileError::Io {
            path: path.into(),
            reason: e.to_string(),
        })?;
        let size_bytes = content.len() as u64;
        Ok(FileContent {
            path: path.into(),
            content,
            size_bytes,
        })
    }

    async fn write(&self, path: &str, content: &str) -> Result<(), FileError> {
        let mut f = fs::File::create(path).await.map_err(|e| FileError::Io {
            path: path.into(),
            reason: e.to_string(),
        })?;
        f.write_all(content.as_bytes())
            .await
            .map_err(|e| FileError::Io {
                path: path.into(),
                reason: e.to_string(),
            })?;
        Ok(())
    }

    async fn list(&self, path: &str) -> Result<Vec<FileEntry>, FileError> {
        let mut dir = fs::read_dir(path).await.map_err(|e| FileError::Io {
            path: path.into(),
            reason: e.to_string(),
        })?;
        let mut entries = Vec::new();
        while let Some(entry) = dir.next_entry().await.map_err(|e| FileError::Io {
            path: path.into(),
            reason: e.to_string(),
        })? {
            let meta = entry.metadata().await.map_err(|e| FileError::Io {
                path: path.into(),
                reason: e.to_string(),
            })?;
            entries.push(FileEntry {
                path: entry.path().to_string_lossy().into_owned(),
                size_bytes: meta.len(),
                is_dir: meta.is_dir(),
                modified_at: None,
            });
        }
        Ok(entries)
    }

    async fn delete(&self, path: &str) -> Result<(), FileError> {
        fs::remove_file(path).await.map_err(|e| FileError::Io {
            path: path.into(),
            reason: e.to_string(),
        })
    }
}
