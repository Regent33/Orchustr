use crate::domain::entities::PromptTemplate;
use crate::domain::errors::BeaconError;
use crate::infra::implementations::PromptBuilder;
use serde::Serialize;

#[derive(Debug, Clone, Default)]
pub struct PromptOrchestrator;

impl PromptOrchestrator {
    pub fn build_template(&self, raw_template: &str) -> Result<PromptTemplate, BeaconError> {
        let span = tracing::info_span!(
            "beacon.build_template",
            otel.name = "beacon.build_template",
            status = tracing::field::Empty,
        );
        let _guard = span.enter();
        let result = PromptBuilder::new().template(raw_template).build();
        span.record("status", if result.is_ok() { "success" } else { "failure" });
        result
    }

    pub fn render_template<T: Serialize>(
        &self,
        template: &PromptTemplate,
        context: &T,
    ) -> Result<String, BeaconError> {
        let span = tracing::info_span!(
            "beacon.render_template",
            otel.name = "beacon.render_template",
            status = tracing::field::Empty,
        );
        let _guard = span.enter();
        let result = template.render(context);
        span.record("status", if result.is_ok() { "success" } else { "failure" });
        result
    }
}
