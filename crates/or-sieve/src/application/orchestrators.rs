use crate::domain::contracts::{JsonSchemaOutput, StructuredParser};
use crate::domain::entities::PlainText;
use crate::domain::errors::SieveError;
use crate::infra::implementations::TextParser;

#[derive(Debug, Clone, Default)]
pub struct SieveOrchestrator;

impl SieveOrchestrator {
    pub fn parse_structured<T: JsonSchemaOutput, P: StructuredParser<T>>(
        &self,
        parser: &P,
        raw: &str,
    ) -> Result<T, SieveError> {
        let span = tracing::info_span!(
            "sieve.parse_structured",
            otel.name = "sieve.parse_structured",
            status = tracing::field::Empty,
        );
        let _guard = span.enter();
        let result = parser.parse(raw);
        span.record("status", if result.is_ok() { "success" } else { "failure" });
        result
    }

    pub fn parse_text(&self, parser: &TextParser, raw: &str) -> Result<PlainText, SieveError> {
        let span = tracing::info_span!(
            "sieve.parse_text",
            otel.name = "sieve.parse_text",
            status = tracing::field::Empty,
        );
        let _guard = span.enter();
        let result = parser.parse(raw);
        span.record("status", if result.is_ok() { "success" } else { "failure" });
        result
    }
}
