use crate::domain::entities::CompletionResponse;
use crate::domain::errors::ConduitError;
use crate::infra::adapters::gemini::parse_gemini_response;
use serde_json::Value;

/// Parses a Google Vertex AI response.
/// Uses the same format as the Gemini API.
pub(crate) fn parse_vertex_response(body: &Value) -> Result<CompletionResponse, ConduitError> {
    parse_gemini_response(body)
}
