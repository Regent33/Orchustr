use crate::domain::errors::BridgeError;
use crate::infra::adapters::{dyn_state_from_json, dyn_state_to_json};
use or_beacon::PromptBuilder;

pub(crate) fn render(template: &str, context_json: &str) -> Result<String, BridgeError> {
    let context = dyn_state_from_json(context_json)?;
    PromptBuilder::new()
        .template(template)
        .build()
        .map_err(|error| BridgeError::Prompt(error.to_string()))?
        .render(&context)
        .map_err(|error| BridgeError::Prompt(error.to_string()))
}

pub(crate) fn normalize(raw_state: &str) -> Result<String, BridgeError> {
    dyn_state_to_json(&dyn_state_from_json(raw_state)?)
}
