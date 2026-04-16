use futures::StreamExt;
use or_conduit::{
    AnthropicConduit, CompletionMessage, CompletionResponse, ConduitError, ConduitOrchestrator,
    ConduitProvider, ContentPart, FinishReason, ImageDetail, MessageRole, OpenAiCompatConduit,
};
use or_core::TokenUsage;

mod providers;

#[derive(Clone)]
struct TestProvider {
    response: Result<CompletionResponse, ConduitError>,
}

impl ConduitProvider for TestProvider {
    async fn complete_messages(
        &self,
        _messages: Vec<CompletionMessage>,
    ) -> Result<CompletionResponse, ConduitError> {
        self.response.clone()
    }
}

#[tokio::test]
async fn prepare_text_request_wraps_user_text() {
    let orchestrator = ConduitOrchestrator;
    let messages = orchestrator.prepare_text_request("hello world").unwrap();
    assert_eq!(messages.len(), 1);
    assert_eq!(messages[0].role, MessageRole::User);
}

#[tokio::test]
async fn prepare_text_request_rejects_blank_prompts() {
    let orchestrator = ConduitOrchestrator;
    let result = orchestrator.prepare_text_request("   ");
    assert_eq!(
        result,
        Err(ConduitError::InvalidRequest(
            "prompt must not be empty".to_owned()
        ))
    );
}

#[tokio::test]
async fn execute_completion_returns_provider_output() {
    let provider = TestProvider {
        response: Ok(CompletionResponse {
            text: "done".to_owned(),
            usage: TokenUsage {
                prompt_tokens: 1,
                completion_tokens: 1,
                total_tokens: 2,
            },
            finish_reason: FinishReason::Stop,
        }),
    };
    let result = ConduitOrchestrator
        .execute_completion(
            &provider,
            vec![CompletionMessage::single_text(MessageRole::User, "hi")],
        )
        .await
        .unwrap();
    assert_eq!(result.text, "done");
}

#[tokio::test]
async fn execute_completion_propagates_provider_errors() {
    let provider = TestProvider {
        response: Err(ConduitError::NotImplemented(
            "downstream unavailable".to_owned(),
        )),
    };
    let result = ConduitOrchestrator
        .execute_completion(
            &provider,
            vec![CompletionMessage::single_text(MessageRole::User, "hi")],
        )
        .await;
    assert_eq!(
        result,
        Err(ConduitError::NotImplemented(
            "downstream unavailable".to_owned()
        ))
    );
}

#[tokio::test]
async fn complete_text_wraps_prompt_for_struct_convenience() {
    let provider = TestProvider {
        response: Ok(CompletionResponse {
            text: "wrapped".to_owned(),
            usage: TokenUsage::default(),
            finish_reason: FinishReason::Stop,
        }),
    };
    let result = provider.complete_text("summarize this").await.unwrap();
    assert_eq!(result.text, "wrapped");
}

#[tokio::test]
async fn openai_and_anthropic_conduits_accept_explicit_construction() {
    let openai = OpenAiCompatConduit::openai("openai-key", "gpt-test").unwrap();
    let anthropic = AnthropicConduit::new("anthropic-key", "claude-test").unwrap();
    let _ = (openai, anthropic);
}

#[tokio::test]
async fn multimodal_message_preserves_image_shape() {
    let message = CompletionMessage {
        role: MessageRole::User,
        content: vec![
            ContentPart::Image {
                url: "https://example.com/image.png".to_owned(),
                detail: ImageDetail::High,
            },
            ContentPart::Text {
                text: "describe this".to_owned(),
            },
        ],
    };
    assert_eq!(message.content.len(), 2);
}

#[tokio::test]
async fn stream_text_yields_real_chunks() {
    let provider = TestProvider {
        response: Ok(CompletionResponse {
            text: "stream this now".to_owned(),
            usage: TokenUsage::default(),
            finish_reason: FinishReason::Stop,
        }),
    };
    let output = provider
        .stream_text(vec![CompletionMessage::single_text(
            MessageRole::User,
            "hi",
        )])
        .await
        .unwrap()
        .collect::<Vec<_>>()
        .await;
    assert_eq!(
        output,
        vec![
            Ok("stream".to_owned()),
            Ok("this".to_owned()),
            Ok("now".to_owned())
        ]
    );
}
