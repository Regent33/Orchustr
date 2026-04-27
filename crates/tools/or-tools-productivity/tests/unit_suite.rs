use async_trait::async_trait;
use or_tools_core::{Tool, ToolError, ToolInput};
use or_tools_productivity::application::orchestrators::ProductivityTool;
use or_tools_productivity::{
    CalendarClient, CalendarEvent, Email, EmailClient, Issue, KnowledgeBase, Page,
    ProductivityError, ProductivityOrchestrator, ProjectTracker, TeamMessenger,
};
use serde_json::{Value, json};
use std::sync::Arc;

// ── Stubs ─────────────────────────────────────────────────────────────────────

struct StubEmail;
#[async_trait]
impl EmailClient for StubEmail {
    fn name(&self) -> &'static str {
        "stub-email"
    }
    async fn list(&self, _: Value) -> Result<Vec<Email>, ProductivityError> {
        Ok(vec![Email {
            id: "e1".into(),
            from: "a@b.com".into(),
            to: vec![],
            subject: "Hi".into(),
            body: "body".into(),
            timestamp: None,
        }])
    }
    async fn send_email(&self, email: Email) -> Result<String, ProductivityError> {
        Ok(email.id)
    }
}

struct StubCalendar;
#[async_trait]
impl CalendarClient for StubCalendar {
    fn name(&self) -> &'static str {
        "stub-cal"
    }
    async fn list_events(&self, _: Value) -> Result<Vec<CalendarEvent>, ProductivityError> {
        Ok(vec![])
    }
    async fn create_event(&self, event: CalendarEvent) -> Result<String, ProductivityError> {
        Ok(event.id)
    }
}

struct StubTracker;
#[async_trait]
impl ProjectTracker for StubTracker {
    fn name(&self) -> &'static str {
        "stub-tracker"
    }
    async fn list_issues(&self, _: Value) -> Result<Vec<Issue>, ProductivityError> {
        Ok(vec![Issue {
            id: "i1".into(),
            title: "Bug".into(),
            description: None,
            status: "open".into(),
            assignee: None,
            labels: vec![],
        }])
    }
    async fn create_issue(&self, issue: Issue) -> Result<String, ProductivityError> {
        Ok(issue.id)
    }
}

struct StubKnowledge;
#[async_trait]
impl KnowledgeBase for StubKnowledge {
    fn name(&self) -> &'static str {
        "stub-kb"
    }
    async fn search(&self, _: Value) -> Result<Vec<Page>, ProductivityError> {
        Ok(vec![Page {
            id: "p1".into(),
            title: "Doc".into(),
            content: "content".into(),
            url: None,
        }])
    }
    async fn create_page(&self, page: Page) -> Result<String, ProductivityError> {
        Ok(page.id)
    }
}

struct StubMessenger;
#[async_trait]
impl TeamMessenger for StubMessenger {
    fn name(&self) -> &'static str {
        "stub-msg"
    }
    async fn post(&self, _: &str, _: &str) -> Result<String, ProductivityError> {
        Ok("ts123".into())
    }
    async fn search_messages(&self, _: Value) -> Result<Vec<Page>, ProductivityError> {
        Ok(vec![])
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[tokio::test]
async fn tool_lists_emails() {
    let orch = Arc::new(ProductivityOrchestrator::new().with_email(Arc::new(StubEmail)));
    let tool = ProductivityTool::new(orch);
    let out = tool
        .invoke(ToolInput::new(
            "productivity",
            json!({ "op": "list_emails", "query": {} }),
        ))
        .await
        .unwrap();
    assert!(
        out.payload
            .as_array()
            .map(|a| !a.is_empty())
            .unwrap_or(false)
    );
    assert_eq!(out.payload[0]["id"], "e1");
}

#[tokio::test]
async fn tool_lists_events() {
    let orch = Arc::new(ProductivityOrchestrator::new().with_calendar(Arc::new(StubCalendar)));
    let tool = ProductivityTool::new(orch);
    let out = tool
        .invoke(ToolInput::new(
            "productivity",
            json!({ "op": "list_events", "query": {} }),
        ))
        .await
        .unwrap();
    assert!(out.payload.is_array());
}

#[tokio::test]
async fn tool_lists_issues() {
    let orch = Arc::new(ProductivityOrchestrator::new().with_tracker(Arc::new(StubTracker)));
    let tool = ProductivityTool::new(orch);
    let out = tool
        .invoke(ToolInput::new(
            "productivity",
            json!({ "op": "list_issues", "query": {} }),
        ))
        .await
        .unwrap();
    assert_eq!(out.payload[0]["id"], "i1");
}

#[tokio::test]
async fn tool_searches_knowledge() {
    let orch = Arc::new(ProductivityOrchestrator::new().with_knowledge(Arc::new(StubKnowledge)));
    let tool = ProductivityTool::new(orch);
    let out = tool
        .invoke(ToolInput::new(
            "productivity",
            json!({ "op": "search_knowledge", "query": {} }),
        ))
        .await
        .unwrap();
    assert_eq!(out.payload[0]["title"], "Doc");
}

#[tokio::test]
async fn tool_posts_message() {
    let orch = Arc::new(ProductivityOrchestrator::new().with_messenger(Arc::new(StubMessenger)));
    let tool = ProductivityTool::new(orch);
    let out = tool
        .invoke(ToolInput::new(
            "productivity",
            json!({ "op": "post_message", "channel": "#general", "text": "hello" }),
        ))
        .await
        .unwrap();
    assert_eq!(out.payload["id"], "ts123");
}

#[tokio::test]
async fn tool_rejects_unknown_op() {
    let orch = Arc::new(ProductivityOrchestrator::new());
    let tool = ProductivityTool::new(orch);
    let err = tool
        .invoke(ToolInput::new(
            "productivity",
            json!({ "op": "fly_to_mars" }),
        ))
        .await
        .unwrap_err();
    assert!(matches!(err, ToolError::InvalidInput { .. }));
}

#[tokio::test]
async fn tool_errors_when_client_missing() {
    let orch = Arc::new(ProductivityOrchestrator::new());
    let tool = ProductivityTool::new(orch);
    let err = tool
        .invoke(ToolInput::new(
            "productivity",
            json!({ "op": "list_emails" }),
        ))
        .await
        .unwrap_err();
    assert!(matches!(err, ToolError::InvalidInput { .. }));
}
