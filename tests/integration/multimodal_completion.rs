use or_conduit::{
    CompletionMessage, CompletionResponse, ConduitOrchestrator, ConduitProvider, ContentPart,
    FinishReason, ImageDetail, MessageRole,
};
use or_core::TokenUsage;
use tokio::sync::Mutex;

#[derive(Debug, Default)]
struct RecordingProvider {
    seen: Mutex<Vec<CompletionMessage>>,
}

impl ConduitProvider for RecordingProvider {
    async fn complete_messages(
        &self,
        messages: Vec<CompletionMessage>,
    ) -> Result<CompletionResponse, or_conduit::ConduitError> {
        *self.seen.lock().await = messages;
        Ok(CompletionResponse {
            text: "processed multimodal request".into(),
            usage: TokenUsage::default(),
            finish_reason: FinishReason::Stop,
        })
    }
}

#[tokio::test]
async fn conduit_orchestrator_passes_multimodal_messages_to_provider() {
    let provider = RecordingProvider::default();
    let messages = vec![CompletionMessage {
        role: MessageRole::User,
        content: vec![
            ContentPart::Text {
                text: "Describe this asset".into(),
            },
            ContentPart::Image {
                url: "https://example.test/image.png".into(),
                detail: ImageDetail::High,
            },
            ContentPart::Document {
                data: "ZmFrZS1wZGY=".into(),
                media_type: "application/pdf".into(),
            },
        ],
    }];

    let response = ConduitOrchestrator
        .execute_completion(&provider, messages.clone())
        .await
        .expect("completion should succeed");

    let seen = provider.seen.lock().await.clone();
    assert_eq!(response.text, "processed multimodal request");
    assert_eq!(seen, messages);
}
