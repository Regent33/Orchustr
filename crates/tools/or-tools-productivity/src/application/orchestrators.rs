use crate::domain::contracts::{
    CalendarClient, EmailClient, KnowledgeBase, ProjectTracker, TeamMessenger,
};
use crate::domain::errors::ProductivityError;
use async_trait::async_trait;
use or_tools_core::{Tool, ToolCapability, ToolError, ToolInput, ToolMeta, ToolOutput};
use serde_json::Value;
use std::sync::Arc;

pub struct ProductivityOrchestrator {
    pub email: Option<Arc<dyn EmailClient>>,
    pub calendar: Option<Arc<dyn CalendarClient>>,
    pub tracker: Option<Arc<dyn ProjectTracker>>,
    pub knowledge: Option<Arc<dyn KnowledgeBase>>,
    pub messenger: Option<Arc<dyn TeamMessenger>>,
}

impl ProductivityOrchestrator {
    pub fn new() -> Self {
        Self {
            email: None,
            calendar: None,
            tracker: None,
            knowledge: None,
            messenger: None,
        }
    }
    pub fn with_email(mut self, c: Arc<dyn EmailClient>) -> Self {
        self.email = Some(c);
        self
    }
    pub fn with_calendar(mut self, c: Arc<dyn CalendarClient>) -> Self {
        self.calendar = Some(c);
        self
    }
    pub fn with_tracker(mut self, c: Arc<dyn ProjectTracker>) -> Self {
        self.tracker = Some(c);
        self
    }
    pub fn with_knowledge(mut self, c: Arc<dyn KnowledgeBase>) -> Self {
        self.knowledge = Some(c);
        self
    }
    pub fn with_messenger(mut self, c: Arc<dyn TeamMessenger>) -> Self {
        self.messenger = Some(c);
        self
    }
}

impl Default for ProductivityOrchestrator {
    fn default() -> Self {
        Self::new()
    }
}

pub struct ProductivityTool {
    orchestrator: Arc<ProductivityOrchestrator>,
}

impl ProductivityTool {
    pub fn new(orchestrator: Arc<ProductivityOrchestrator>) -> Self {
        Self { orchestrator }
    }
}

#[async_trait]
impl Tool for ProductivityTool {
    fn meta(&self) -> ToolMeta {
        ToolMeta::new(
            "productivity",
            "Email, calendar, project tracking, knowledge base, and team messaging",
        )
        .with_capability(ToolCapability::Network)
    }

    async fn invoke(&self, input: ToolInput) -> Result<ToolOutput, ToolError> {
        let op = input
            .payload
            .get("op")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let query = input.payload.get("query").cloned().unwrap_or(Value::Null);
        let missing = |r: &str| ToolError::InvalidInput {
            tool: "productivity".into(),
            reason: r.into(),
        };
        let upstream = |e: ProductivityError| ToolError::Upstream {
            tool: "productivity".into(),
            status: 0,
            body: e.to_string(),
        };

        let result = match op {
            "list_emails" => {
                let c = self
                    .orchestrator
                    .email
                    .as_ref()
                    .ok_or_else(|| missing("no email client configured"))?;
                serde_json::to_value(c.list(query).await.map_err(upstream)?).unwrap_or_default()
            }
            "list_events" => {
                let c = self
                    .orchestrator
                    .calendar
                    .as_ref()
                    .ok_or_else(|| missing("no calendar client configured"))?;
                serde_json::to_value(c.list_events(query).await.map_err(upstream)?)
                    .unwrap_or_default()
            }
            "list_issues" => {
                let c = self
                    .orchestrator
                    .tracker
                    .as_ref()
                    .ok_or_else(|| missing("no project tracker configured"))?;
                serde_json::to_value(c.list_issues(query).await.map_err(upstream)?)
                    .unwrap_or_default()
            }
            "search_knowledge" => {
                let c = self
                    .orchestrator
                    .knowledge
                    .as_ref()
                    .ok_or_else(|| missing("no knowledge base configured"))?;
                serde_json::to_value(c.search(query).await.map_err(upstream)?).unwrap_or_default()
            }
            "post_message" => {
                let c = self
                    .orchestrator
                    .messenger
                    .as_ref()
                    .ok_or_else(|| missing("no team messenger configured"))?;
                let channel = input
                    .payload
                    .get("channel")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let text = input
                    .payload
                    .get("text")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let id = c.post(channel, text).await.map_err(upstream)?;
                serde_json::json!({ "id": id })
            }
            _ => {
                return Err(ToolError::InvalidInput {
                    tool: "productivity".into(),
                    reason: format!("unknown op: {op}"),
                });
            }
        };
        Ok(ToolOutput::new("productivity", result))
    }
}
