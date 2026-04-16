use crate::domain::contracts::ConduitProvider;
use crate::domain::entities::{CompletionMessage, CompletionResponse, MessageRole};
use crate::domain::errors::ConduitError;

#[derive(Debug, Clone, Default)]
pub struct ConduitOrchestrator;

impl ConduitOrchestrator {
    #[must_use = "request preparation errors should be handled"]
    pub fn prepare_text_request(
        &self,
        prompt: &str,
    ) -> Result<Vec<CompletionMessage>, ConduitError> {
        let span = tracing::info_span!(
            "conduit.prepare_text_request",
            otel.name = "conduit.prepare_text_request",
            status = tracing::field::Empty,
        );
        let _guard = span.enter();
        let result = if prompt.trim().is_empty() {
            Err(ConduitError::InvalidRequest(
                "prompt must not be empty".to_owned(),
            ))
        } else {
            Ok(vec![CompletionMessage::single_text(
                MessageRole::User,
                prompt,
            )])
        };
        span.record("status", if result.is_ok() { "success" } else { "failure" });
        result
    }

    pub async fn execute_completion<P: ConduitProvider>(
        &self,
        provider: &P,
        messages: Vec<CompletionMessage>,
    ) -> Result<CompletionResponse, ConduitError> {
        let span = tracing::info_span!(
            "conduit.execute_completion",
            otel.name = "conduit.execute_completion",
            message_count = messages.len(),
            status = tracing::field::Empty,
        );
        let _guard = span.enter();
        let result = provider.complete_messages(messages).await;
        span.record("status", if result.is_ok() { "success" } else { "failure" });
        result
    }
}
