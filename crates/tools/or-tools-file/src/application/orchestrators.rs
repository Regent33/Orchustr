use crate::domain::contracts::{DataSource, FileStore};
use crate::domain::entities::FileContent;
use crate::domain::errors::FileError;
use async_trait::async_trait;
use or_tools_core::{Tool, ToolCapability, ToolError, ToolInput, ToolMeta, ToolOutput};
use serde_json::Value;
use std::sync::Arc;

pub struct FileOrchestrator {
    store: Arc<dyn FileStore>,
}

impl FileOrchestrator {
    #[must_use]
    pub fn new(store: Arc<dyn FileStore>) -> Self {
        Self { store }
    }

    pub async fn read(&self, path: &str) -> Result<FileContent, FileError> {
        let span = tracing::info_span!(
            "tools.file.read",
            otel.name = "tools.file.read",
            path,
            status = tracing::field::Empty
        );
        let _guard = span.enter();
        let result = self.store.read(path).await;
        span.record("status", if result.is_ok() { "success" } else { "failure" });
        result
    }
}

pub struct FileStoreTool {
    store: Arc<dyn FileStore>,
}

impl FileStoreTool {
    #[must_use]
    pub fn new(store: Arc<dyn FileStore>) -> Self {
        Self { store }
    }
}

#[async_trait]
impl Tool for FileStoreTool {
    fn meta(&self) -> ToolMeta {
        ToolMeta::new("file", "Local filesystem read/write/list/delete")
            .with_capability(ToolCapability::Filesystem)
    }

    async fn invoke(&self, input: ToolInput) -> Result<ToolOutput, ToolError> {
        let op = input
            .payload
            .get("op")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ToolError::invalid_input(&input.tool, "missing `op`"))?;
        let payload = match op {
            "read" => {
                let path = get_str(&input, "path")?;
                let fc = self.store.read(path).await?;
                serde_json::to_value(&fc).map_err(|e| ser_err(&input.tool, e))?
            }
            "write" => {
                let path = get_str(&input, "path")?;
                let content = get_str(&input, "content")?;
                self.store.write(path, content).await?;
                Value::String("ok".into())
            }
            "list" => {
                let path = get_str(&input, "path")?;
                let entries = self.store.list(path).await?;
                serde_json::to_value(&entries).map_err(|e| ser_err(&input.tool, e))?
            }
            "delete" => {
                let path = get_str(&input, "path")?;
                self.store.delete(path).await?;
                Value::String("ok".into())
            }
            other => {
                return Err(ToolError::invalid_input(
                    &input.tool,
                    format!("unknown op `{other}`"),
                ));
            }
        };
        Ok(ToolOutput::new(input.tool, payload))
    }
}

pub struct DataSourceTool {
    source: Arc<dyn DataSource>,
}

impl DataSourceTool {
    #[must_use]
    pub fn new(source: Arc<dyn DataSource>) -> Self {
        Self { source }
    }
}

#[async_trait]
impl Tool for DataSourceTool {
    fn meta(&self) -> ToolMeta {
        ToolMeta::new(
            format!("datasource.{}", self.source.name()),
            "External data source",
        )
        .with_capability(ToolCapability::Network)
    }

    async fn invoke(&self, input: ToolInput) -> Result<ToolOutput, ToolError> {
        let result = self.source.fetch(input.payload).await?;
        Ok(ToolOutput::new(input.tool, result))
    }
}

fn get_str<'a>(input: &'a ToolInput, key: &str) -> Result<&'a str, ToolError> {
    input
        .payload
        .get(key)
        .and_then(|v| v.as_str())
        .ok_or_else(|| ToolError::invalid_input(&input.tool, format!("missing `{key}`")))
}

fn ser_err(tool: &str, e: serde_json::Error) -> ToolError {
    ToolError::Serialization {
        tool: tool.into(),
        reason: e.to_string(),
    }
}
