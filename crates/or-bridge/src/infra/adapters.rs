use crate::domain::errors::BridgeError;
use or_core::DynState;

pub(crate) fn dyn_state_from_json(raw: &str) -> Result<DynState, BridgeError> {
    let value: serde_json::Value =
        serde_json::from_str(raw).map_err(|error| BridgeError::InvalidJson(error.to_string()))?;
    let object = value.as_object().ok_or(BridgeError::InvalidState)?;
    Ok(object
        .iter()
        .map(|(key, value)| (key.clone(), value.clone()))
        .collect())
}

pub(crate) fn dyn_state_to_json(state: &DynState) -> Result<String, BridgeError> {
    serde_json::to_string(state).map_err(|error| BridgeError::InvalidJson(error.to_string()))
}
