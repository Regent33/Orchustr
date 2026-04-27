use crate::domain::contracts::MessageSender;
use crate::domain::entities::{Message, SendResult};
use crate::domain::errors::CommsError;
use async_trait::async_trait;
use or_tools_core::{Tool, ToolCapability, ToolError, ToolInput, ToolMeta, ToolOutput};
use std::sync::Arc;

pub struct CommsOrchestrator {
    senders: Vec<Arc<dyn MessageSender>>,
}

impl CommsOrchestrator {
    pub fn new(senders: Vec<Arc<dyn MessageSender>>) -> Self {
        Self { senders }
    }

    pub async fn send(&self, msg: Message) -> Result<SendResult, CommsError> {
        let ch = format!("{:?}", msg.channel).to_lowercase();
        for sender in &self.senders {
            if sender.channel() == ch {
                return sender.send(msg).await;
            }
        }
        Err(CommsError::UnsupportedChannel(ch))
    }
}

pub struct CommsTool {
    orchestrator: Arc<CommsOrchestrator>,
}

impl CommsTool {
    pub fn new(orchestrator: Arc<CommsOrchestrator>) -> Self {
        Self { orchestrator }
    }
}

#[async_trait]
impl Tool for CommsTool {
    fn meta(&self) -> ToolMeta {
        ToolMeta::new(
            "comms",
            "Send messages via SMS, Telegram, Discord, WhatsApp, Facebook, Messenger",
        )
        .with_capability(ToolCapability::Network)
    }

    async fn invoke(&self, input: ToolInput) -> Result<ToolOutput, ToolError> {
        let msg: Message =
            serde_json::from_value(input.payload.clone()).map_err(|e| ToolError::InvalidInput {
                tool: "comms".into(),
                reason: e.to_string(),
            })?;
        let result = self
            .orchestrator
            .send(msg)
            .await
            .map_err(|e| ToolError::Upstream {
                tool: "comms".into(),
                status: 0,
                body: e.to_string(),
            })?;
        Ok(ToolOutput::new(
            "comms",
            serde_json::to_value(&result).unwrap_or_default(),
        ))
    }
}
