use async_trait::async_trait;
use or_tools_core::{Tool, ToolError, ToolInput};
use or_tools_web::{FetchRequest, FetchResponse, HttpMethod, ScrapedPage, WebBrowser, WebError};
use or_tools_web::{Scraper, WebOrchestrator};
use serde_json::json;
use std::sync::Arc;

struct StubBrowser;

#[async_trait]
impl WebBrowser for StubBrowser {
    fn name(&self) -> &'static str {
        "stub"
    }

    async fn fetch(&self, req: FetchRequest) -> Result<FetchResponse, WebError> {
        Ok(FetchResponse {
            status: 200,
            body: format!("fetched:{}", req.url),
            headers: Default::default(),
            final_url: Some(req.url),
        })
    }
}

struct StubScraper;

#[async_trait]
impl Scraper for StubScraper {
    fn name(&self) -> &'static str {
        "stub"
    }

    async fn scrape(&self, url: &str) -> Result<ScrapedPage, WebError> {
        Ok(ScrapedPage {
            url: url.into(),
            title: Some("Hello".into()),
            text: "body".into(),
            links: vec!["https://a".into()],
        })
    }
}

#[tokio::test]
async fn orchestrator_rejects_invalid_url() {
    let orch = WebOrchestrator::new(Arc::new(StubBrowser));
    let err = orch
        .fetch(FetchRequest::get("not-a-url"))
        .await
        .unwrap_err();
    assert!(matches!(err, WebError::InvalidUrl(_)));
}

#[tokio::test]
async fn orchestrator_rejects_file_scheme() {
    let orch = WebOrchestrator::new(Arc::new(StubBrowser));
    let err = orch
        .fetch(FetchRequest::get("file:///etc/passwd"))
        .await
        .unwrap_err();
    assert!(matches!(err, WebError::UnsafeScheme(_)));
}

#[tokio::test]
async fn orchestrator_fetches_http_url() {
    let orch = WebOrchestrator::new(Arc::new(StubBrowser));
    let resp = orch
        .fetch(FetchRequest::get("https://example.com"))
        .await
        .unwrap();
    assert_eq!(resp.status, 200);
    assert!(resp.body.starts_with("fetched:"));
}

#[tokio::test]
async fn browser_tool_invokes_fetch() {
    let tool = or_tools_web::application::orchestrators::BrowserTool::new(StubBrowser);
    let out = tool
        .invoke(ToolInput::new(
            "web.stub",
            json!({ "url": "https://example.com", "method": "GET" }),
        ))
        .await
        .unwrap();
    assert_eq!(out.payload["status"], 200);
}

#[tokio::test]
async fn browser_tool_rejects_unsafe_scheme() {
    let tool = or_tools_web::application::orchestrators::BrowserTool::new(StubBrowser);
    let err = tool
        .invoke(ToolInput::new(
            "web.stub",
            json!({ "url": "javascript:alert(1)", "method": "GET" }),
        ))
        .await
        .unwrap_err();
    assert!(matches!(err, ToolError::InvalidInput { .. }));
}

#[tokio::test]
async fn scraper_tool_returns_page() {
    let tool = or_tools_web::application::orchestrators::ScraperTool::new(StubScraper);
    let out = tool
        .invoke(ToolInput::new(
            "web.scrape.stub",
            json!({ "url": "https://example.com" }),
        ))
        .await
        .unwrap();
    assert_eq!(out.payload["title"], "Hello");
}

#[tokio::test]
async fn http_method_serializes() {
    let method = HttpMethod::Post;
    let json = serde_json::to_string(&method).unwrap();
    assert_eq!(json, "\"POST\"");
}
