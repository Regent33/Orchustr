use crate::domain::entities::PromptTemplate;
use crate::domain::errors::BeaconError;
use crate::infra::adapters::{extract_variables, sanitize_text};
use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Clone, Default)]
pub struct PromptBuilder {
    template: Option<String>,
}

impl PromptBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn template(mut self, template: impl Into<String>) -> Self {
        self.template = Some(template.into());
        self
    }

    pub fn build(self) -> Result<PromptTemplate, BeaconError> {
        let template = self.template.ok_or(BeaconError::MissingTemplate)?;
        let cleaned = sanitize_text(&template);
        let variables = extract_variables(&cleaned)?;
        Ok(PromptTemplate {
            template: cleaned,
            variables,
        })
    }
}

impl PromptTemplate {
    pub fn render<T: Serialize>(&self, context: &T) -> Result<String, BeaconError> {
        let context = serde_json::to_value(context)
            .map_err(|error| BeaconError::InvalidContext(error.to_string()))?;
        let object = context.as_object().ok_or_else(|| {
            BeaconError::InvalidContext("prompt context must serialize to a JSON object".to_owned())
        })?;
        let mut rendered = self.template.clone();
        for variable in &self.variables {
            let value = object
                .get(variable)
                .ok_or_else(|| BeaconError::MissingVariable(variable.clone()))?;
            let replacement = sanitize_text(&value_to_string(value));
            rendered = rendered.replace(&format!("{{{{{variable}}}}}"), &replacement);
        }
        Ok(rendered)
    }
}

fn value_to_string(value: &Value) -> String {
    match value {
        Value::String(text) => text.clone(),
        other => serde_json::to_string(other).unwrap_or_default(),
    }
}
