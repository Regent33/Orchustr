use crate::domain::contracts::DataSource;
use crate::domain::errors::FileError;
use async_trait::async_trait;
use serde_json::Value;

/// JSON Toolkit — resolves a dot-path inside a JSON value.
/// Input: `{ "data": <json>, "path": ["key","subkey"] }`
/// Output: the value at that path, or null.
pub struct JsonToolkit;

#[async_trait]
impl DataSource for JsonToolkit {
    fn name(&self) -> &'static str {
        "json"
    }

    async fn fetch(&self, query: Value) -> Result<Value, FileError> {
        let data = query
            .get("data")
            .ok_or_else(|| FileError::Json("missing `data`".into()))?;
        let path = query
            .get("path")
            .and_then(|v| v.as_array())
            .ok_or_else(|| FileError::Json("missing `path` array".into()))?;

        let mut current = data;
        for segment in path {
            let key = segment
                .as_str()
                .ok_or_else(|| FileError::Json("path segment must be a string".into()))?;
            current = current
                .get(key)
                .or_else(|| key.parse::<usize>().ok().and_then(|i| current.get(i)))
                .unwrap_or(&Value::Null);
        }
        Ok(current.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn resolves_nested_path() {
        let tk = JsonToolkit;
        let val = tk
            .fetch(json!({ "data": { "a": { "b": 42 } }, "path": ["a", "b"] }))
            .await
            .unwrap();
        assert_eq!(val, 42);
    }

    #[tokio::test]
    async fn returns_null_for_missing_key() {
        let tk = JsonToolkit;
        let val = tk
            .fetch(json!({ "data": {}, "path": ["missing"] }))
            .await
            .unwrap();
        assert_eq!(val, Value::Null);
    }
}
