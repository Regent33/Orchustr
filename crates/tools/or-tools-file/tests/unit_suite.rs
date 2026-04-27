use async_trait::async_trait;
use or_tools_core::{Tool, ToolError, ToolInput};
use or_tools_file::application::orchestrators::FileStoreTool;
use or_tools_file::{FileContent, FileEntry, FileError, FileStore};
use serde_json::{Value, json};
use std::sync::Arc;

struct MemStore {
    data: std::sync::Arc<tokio::sync::Mutex<std::collections::HashMap<String, String>>>,
}

impl MemStore {
    fn new() -> Self {
        Self {
            data: Default::default(),
        }
    }
}

#[async_trait]
impl FileStore for MemStore {
    async fn read(&self, path: &str) -> Result<FileContent, FileError> {
        let lock = self.data.lock().await;
        let content = lock
            .get(path)
            .cloned()
            .ok_or_else(|| FileError::NotFound(path.into()))?;
        let size = content.len() as u64;
        Ok(FileContent {
            path: path.into(),
            content,
            size_bytes: size,
        })
    }
    async fn write(&self, path: &str, content: &str) -> Result<(), FileError> {
        self.data.lock().await.insert(path.into(), content.into());
        Ok(())
    }
    async fn list(&self, _path: &str) -> Result<Vec<FileEntry>, FileError> {
        Ok(self
            .data
            .lock()
            .await
            .keys()
            .map(|k| FileEntry {
                path: k.clone(),
                size_bytes: 0,
                is_dir: false,
                modified_at: None,
            })
            .collect())
    }
    async fn delete(&self, path: &str) -> Result<(), FileError> {
        self.data
            .lock()
            .await
            .remove(path)
            .map(|_| ())
            .ok_or_else(|| FileError::NotFound(path.into()))
    }
}

#[tokio::test]
async fn read_returns_not_found_for_missing() {
    let store = Arc::new(MemStore::new());
    let err = store.read("missing.txt").await.unwrap_err();
    assert!(matches!(err, FileError::NotFound(_)));
}

#[tokio::test]
async fn write_then_read_roundtrip() {
    let store = Arc::new(MemStore::new());
    store.write("hello.txt", "world").await.unwrap();
    let fc = store.read("hello.txt").await.unwrap();
    assert_eq!(fc.content, "world");
}

#[tokio::test]
async fn file_store_tool_read_op() {
    let store = Arc::new(MemStore::new());
    store.write("a.txt", "abc").await.unwrap();
    let tool = FileStoreTool::new(store);
    let out = tool
        .invoke(ToolInput::new(
            "file",
            json!({ "op": "read", "path": "a.txt" }),
        ))
        .await
        .unwrap();
    assert_eq!(out.payload["content"], "abc");
}

#[tokio::test]
async fn file_store_tool_unknown_op() {
    let tool = FileStoreTool::new(Arc::new(MemStore::new()));
    let err = tool
        .invoke(ToolInput::new("file", json!({ "op": "bork" })))
        .await
        .unwrap_err();
    assert!(matches!(err, ToolError::InvalidInput { .. }));
}

#[cfg(feature = "json-toolkit")]
#[tokio::test]
async fn json_toolkit_resolves_path() {
    use or_tools_file::domain::contracts::DataSource;
    use or_tools_file::infra::json_toolkit::JsonToolkit;
    let tk = JsonToolkit;
    let val = tk
        .fetch(json!({ "data": { "x": { "y": 99 } }, "path": ["x", "y"] }))
        .await
        .unwrap();
    assert_eq!(val, 99);
}

#[cfg(feature = "json-toolkit")]
#[tokio::test]
async fn json_toolkit_returns_null_for_missing() {
    use or_tools_file::domain::contracts::DataSource;
    use or_tools_file::infra::json_toolkit::JsonToolkit;
    let val = JsonToolkit
        .fetch(json!({ "data": {}, "path": ["nope"] }))
        .await
        .unwrap();
    assert_eq!(val, Value::Null);
}
