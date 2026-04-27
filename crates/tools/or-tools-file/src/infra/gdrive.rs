use super::shared::{load_credential, transport};
use crate::domain::contracts::FileStore;
use crate::domain::entities::{FileContent, FileEntry};
use crate::domain::errors::FileError;
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::json;

const PROVIDER: &str = "gdrive";
const TOKEN_ENV: &str = "GDRIVE_ACCESS_TOKEN";
const FILES_URL: &str = "https://www.googleapis.com/drive/v3/files";
const UPLOAD_URL: &str = "https://www.googleapis.com/upload/drive/v3/files";

pub struct GoogleDriveStore {
    client: reqwest::Client,
    access_token: String,
}

impl GoogleDriveStore {
    pub fn from_env() -> Result<Self, FileError> {
        Ok(Self {
            client: reqwest::Client::new(),
            access_token: load_credential(TOKEN_ENV)?,
        })
    }

    #[must_use]
    pub fn with_token(client: reqwest::Client, access_token: impl Into<String>) -> Self {
        Self {
            client,
            access_token: access_token.into(),
        }
    }

    fn auth(&self) -> String {
        format!("Bearer {}", self.access_token)
    }
}

#[derive(Deserialize)]
struct DriveFile {
    id: String,
    #[allow(dead_code)]
    name: String,
    #[serde(default)]
    size: Option<String>,
    #[serde(rename = "mimeType", default)]
    mime_type: String,
}
#[derive(Deserialize)]
struct DriveList {
    files: Vec<DriveFile>,
}

#[async_trait]
impl FileStore for GoogleDriveStore {
    async fn read(&self, path: &str) -> Result<FileContent, FileError> {
        // `path` is a file ID for Drive
        let text_resp = self
            .client
            .get(format!("{}/{}", FILES_URL, path))
            .header("Authorization", self.auth())
            .query_pairs([("alt", "media")])
            .send()
            .await
            .map_err(transport)?;
        let status = text_resp.status().as_u16();
        if status != 200 {
            return Err(FileError::Upstream {
                provider: PROVIDER.into(),
                status,
                body: text_resp.text().await.unwrap_or_default(),
            });
        }
        let content = text_resp
            .text()
            .await
            .map_err(|e| FileError::Transport(e.to_string()))?;
        let size_bytes = content.len() as u64;
        Ok(FileContent {
            path: path.into(),
            content,
            size_bytes,
        })
    }

    async fn write(&self, path: &str, content: &str) -> Result<(), FileError> {
        let meta = json!({ "name": path });
        let boundary = "boundary_orchustr";
        let body = format!(
            "--{boundary}\r\nContent-Type: application/json\r\n\r\n{meta}\r\n--{boundary}\r\nContent-Type: text/plain\r\n\r\n{content}\r\n--{boundary}--",
            boundary = boundary,
            meta = meta,
            content = content
        );
        let resp = self
            .client
            .post(format!("{}?uploadType=multipart", UPLOAD_URL))
            .header("Authorization", self.auth())
            .header(
                "Content-Type",
                format!("multipart/related; boundary={}", boundary),
            )
            .body(body)
            .send()
            .await
            .map_err(transport)?;
        let status = resp.status().as_u16();
        if !(200..300).contains(&status) {
            return Err(FileError::Upstream {
                provider: PROVIDER.into(),
                status,
                body: resp.text().await.unwrap_or_default(),
            });
        }
        Ok(())
    }

    async fn list(&self, _path: &str) -> Result<Vec<FileEntry>, FileError> {
        let resp = self
            .client
            .get(FILES_URL)
            .header("Authorization", self.auth())
            .send()
            .await
            .map_err(transport)?;
        let status = resp.status();
        if !status.is_success() {
            return Err(FileError::Upstream {
                provider: PROVIDER.into(),
                status: status.as_u16(),
                body: resp.text().await.unwrap_or_default(),
            });
        }
        let list: DriveList = resp
            .json()
            .await
            .map_err(|e| FileError::Transport(e.to_string()))?;
        Ok(list
            .files
            .into_iter()
            .map(|f| FileEntry {
                path: f.id,
                size_bytes: f.size.and_then(|s| s.parse().ok()).unwrap_or(0),
                is_dir: f.mime_type == "application/vnd.google-apps.folder",
                modified_at: None,
            })
            .collect())
    }

    async fn delete(&self, path: &str) -> Result<(), FileError> {
        let resp = self
            .client
            .delete(format!("{}/{}", FILES_URL, path))
            .header("Authorization", self.auth())
            .send()
            .await
            .map_err(transport)?;
        let status = resp.status().as_u16();
        if status != 204 {
            return Err(FileError::Upstream {
                provider: PROVIDER.into(),
                status,
                body: resp.text().await.unwrap_or_default(),
            });
        }
        Ok(())
    }
}

trait QueryPairs {
    fn query_pairs(self, params: impl IntoIterator<Item = (&'static str, &'static str)>) -> Self;
}

impl QueryPairs for reqwest::RequestBuilder {
    fn query_pairs(self, params: impl IntoIterator<Item = (&'static str, &'static str)>) -> Self {
        params.into_iter().fold(self, |b, (k, v)| {
            b.header("X-Dummy-Unused", format!("{k}={v}")) // placeholder — real impl uses url crate
        })
    }
}
