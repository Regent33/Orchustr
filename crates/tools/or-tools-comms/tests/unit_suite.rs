use async_trait::async_trait;
use or_tools_comms::application::orchestrators::CommsTool;
use or_tools_comms::{Channel, CommsError, CommsOrchestrator, Message, MessageSender, SendResult};
use or_tools_core::{Tool, ToolError, ToolInput};
use serde_json::json;
use std::sync::Arc;

struct StubSender {
    ch: &'static str,
    channel_val: Channel,
}

#[async_trait]
impl MessageSender for StubSender {
    fn channel(&self) -> &'static str {
        self.ch
    }
    async fn send(&self, _: Message) -> Result<SendResult, CommsError> {
        Ok(SendResult {
            message_id: "stub-id".into(),
            channel: self.channel_val.clone(),
            status: "sent".into(),
        })
    }
}

fn telegram_sender() -> StubSender {
    StubSender {
        ch: "telegram",
        channel_val: Channel::Telegram,
    }
}

fn sms_sender() -> StubSender {
    StubSender {
        ch: "sms",
        channel_val: Channel::Sms,
    }
}

#[tokio::test]
async fn orchestrator_routes_to_matching_sender() {
    let orch = CommsOrchestrator::new(vec![Arc::new(telegram_sender())]);
    let msg = Message {
        channel: Channel::Telegram,
        to: "chat123".into(),
        body: "hello".into(),
        from: None,
    };
    let result = orch.send(msg).await.unwrap();
    assert_eq!(result.message_id, "stub-id");
    assert_eq!(result.channel, Channel::Telegram);
}

#[tokio::test]
async fn orchestrator_returns_error_for_unsupported_channel() {
    let orch = CommsOrchestrator::new(vec![]);
    let msg = Message {
        channel: Channel::Discord,
        to: "chan".into(),
        body: "hi".into(),
        from: None,
    };
    let err = orch.send(msg).await.unwrap_err();
    assert!(matches!(err, CommsError::UnsupportedChannel(_)));
}

#[tokio::test]
async fn orchestrator_skips_wrong_channel_sender() {
    let orch = CommsOrchestrator::new(vec![Arc::new(telegram_sender())]);
    let msg = Message {
        channel: Channel::Sms,
        to: "+1555".into(),
        body: "txt".into(),
        from: None,
    };
    let err = orch.send(msg).await.unwrap_err();
    assert!(matches!(err, CommsError::UnsupportedChannel(_)));
}

#[tokio::test]
async fn comms_tool_invokes_via_tool_trait() {
    let orch = Arc::new(CommsOrchestrator::new(vec![Arc::new(sms_sender())]));
    let tool = CommsTool::new(orch);
    let payload =
        json!({ "channel": "Sms", "to": "+15550001234", "body": "test sms", "from": null });
    let out = tool.invoke(ToolInput::new("comms", payload)).await.unwrap();
    assert_eq!(out.payload["message_id"], "stub-id");
    assert_eq!(out.payload["status"], "sent");
}

#[tokio::test]
async fn comms_tool_rejects_invalid_payload() {
    let orch = Arc::new(CommsOrchestrator::new(vec![]));
    let tool = CommsTool::new(orch);
    let err = tool
        .invoke(ToolInput::new("comms", json!({ "bad": true })))
        .await
        .unwrap_err();
    assert!(matches!(err, ToolError::InvalidInput { .. }));
}
