use or_tools_core::{Tool, ToolInput};
use or_tools_loaders::application::orchestrators::LoaderTool;
use or_tools_loaders::{
    DocumentKind, DocumentLoader, LoaderError, LoaderOrchestrator, LoaderRequest, LoaderSource,
};
use serde_json::json;
use std::sync::Arc;

fn raw_req(content: &str, kind: DocumentKind) -> LoaderRequest {
    LoaderRequest {
        source: LoaderSource::Raw {
            content: content.to_string(),
        },
        kind_hint: Some(kind),
        chunk_size: 0,
        metadata: serde_json::Value::Null,
    }
}

#[cfg(feature = "text")]
#[tokio::test]
async fn text_loader_returns_content() {
    use or_tools_loaders::infra::text::TextLoader;
    let loader = TextLoader;
    let docs = loader
        .load(raw_req("hello world", DocumentKind::Text))
        .await
        .unwrap();
    assert_eq!(docs.len(), 1);
    assert_eq!(docs[0].content, "hello world");
}

#[cfg(feature = "markdown")]
#[tokio::test]
async fn markdown_loader_strips_frontmatter() {
    use or_tools_loaders::infra::markdown::MarkdownLoader;
    let loader = MarkdownLoader;
    let md = "---\ntitle: test\n---\n# Hello";
    let docs = loader
        .load(raw_req(md, DocumentKind::Markdown))
        .await
        .unwrap();
    assert_eq!(docs.len(), 1);
    assert!(
        docs[0].content.contains("# Hello"),
        "expected heading, got: {}",
        docs[0].content
    );
}

#[cfg(feature = "json")]
#[tokio::test]
async fn json_loader_validates_json() {
    use or_tools_loaders::infra::json::JsonLoader;
    let loader = JsonLoader;
    let docs = loader
        .load(raw_req(r#"{"key": "val"}"#, DocumentKind::Json))
        .await
        .unwrap();
    assert!(!docs.is_empty());
}

#[cfg(feature = "json")]
#[tokio::test]
async fn json_loader_rejects_invalid_json() {
    use or_tools_loaders::infra::json::JsonLoader;
    let loader = JsonLoader;
    let result = loader.load(raw_req("{invalid}", DocumentKind::Json)).await;
    assert!(matches!(result, Err(LoaderError::Parse(_))));
}

#[cfg(all(feature = "text", feature = "json"))]
#[tokio::test]
async fn orchestrator_routes_by_kind_hint() {
    use or_tools_loaders::infra::{json::JsonLoader, text::TextLoader};
    let mut orch = LoaderOrchestrator::new();
    orch.register(Arc::new(TextLoader));
    orch.register(Arc::new(JsonLoader));

    let docs = orch
        .load(raw_req("hello", DocumentKind::Text))
        .await
        .unwrap();
    assert_eq!(docs[0].kind, DocumentKind::Text);
}

#[cfg(feature = "text")]
#[tokio::test]
async fn text_loader_chunks_large_content() {
    use or_tools_loaders::infra::text::TextLoader;
    let loader = TextLoader;
    let req = LoaderRequest {
        source: LoaderSource::Raw {
            content: "a".repeat(100),
        },
        kind_hint: Some(DocumentKind::Text),
        chunk_size: 30,
        metadata: serde_json::Value::Null,
    };
    let docs = loader.load(req).await.unwrap();
    assert!(docs.len() > 1);
}

#[cfg(all(feature = "text", feature = "json"))]
#[tokio::test]
async fn loader_tool_dispatches_via_tool_trait() {
    use or_tools_loaders::infra::{json::JsonLoader, text::TextLoader};
    let mut orch = LoaderOrchestrator::new();
    orch.register(Arc::new(TextLoader));
    orch.register(Arc::new(JsonLoader));
    let tool = LoaderTool::new(Arc::new(orch));
    let out = tool
        .invoke(ToolInput::new(
            "loader",
            json!({
                "source": { "type": "raw", "content": "hello" },
                "kind_hint": "text",
                "chunk_size": 0,
                "metadata": null
            }),
        ))
        .await
        .unwrap();
    let arr = out.payload.as_array().unwrap();
    assert_eq!(arr.len(), 1);
}
