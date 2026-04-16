#![allow(async_fn_in_trait)]

use crate::domain::entities::{CompletionMessage, CompletionResponse, MessageRole};
use crate::domain::errors::ConduitError;
use futures::{Stream, stream};
use std::pin::Pin;

pub type TextStream = Pin<Box<dyn Stream<Item = Result<String, ConduitError>> + Send>>;

#[cfg_attr(test, mockall::automock)]
pub trait ConduitProvider: Send + Sync + 'static {
    async fn complete_messages(
        &self,
        messages: Vec<CompletionMessage>,
    ) -> Result<CompletionResponse, ConduitError>;

    async fn complete_text(&self, prompt: &str) -> Result<CompletionResponse, ConduitError> {
        self.complete_messages(vec![CompletionMessage::single_text(
            MessageRole::User,
            prompt,
        )])
        .await
    }

    async fn stream_text(
        &self,
        messages: Vec<CompletionMessage>,
    ) -> Result<TextStream, ConduitError> {
        let response = self.complete_messages(messages).await?;
        let chunks = response
            .text
            .split_whitespace()
            .map(ToOwned::to_owned)
            .collect::<Vec<_>>();
        let output = if chunks.is_empty() {
            vec![String::new()]
        } else {
            chunks
        };
        Ok(Box::pin(stream::iter(output.into_iter().map(Ok))))
    }
}
