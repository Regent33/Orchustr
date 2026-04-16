use crate::domain::errors::McpError;
use schemars::schema::{InstanceType, RootSchema, Schema, SingleOrVec};

pub(crate) fn validate_input(
    schema: &RootSchema,
    value: &serde_json::Value,
) -> Result<(), McpError> {
    validate_schema(&Schema::Object(schema.schema.clone()), value)
}

fn validate_schema(schema: &Schema, value: &serde_json::Value) -> Result<(), McpError> {
    match schema {
        Schema::Bool(true) => Ok(()),
        Schema::Bool(false) => Err(McpError::ToolExecution(
            "input rejected by schema".to_owned(),
        )),
        Schema::Object(object) => {
            if let Some(kind) = &object.instance_type {
                validate_type(kind, value)?;
            }
            if let Some(validation) = &object.object {
                let map = value
                    .as_object()
                    .ok_or_else(|| McpError::ToolExecution("expected object input".to_owned()))?;
                for key in &validation.required {
                    if !map.contains_key(key) {
                        return Err(McpError::ToolExecution(format!(
                            "missing required input: {key}"
                        )));
                    }
                }
            }
            Ok(())
        }
    }
}

fn validate_type(
    types: &SingleOrVec<InstanceType>,
    value: &serde_json::Value,
) -> Result<(), McpError> {
    let matches = match types {
        SingleOrVec::Single(kind) => instance_matches(kind.as_ref(), value),
        SingleOrVec::Vec(kinds) => kinds.iter().any(|kind| instance_matches(kind, value)),
    };
    if matches {
        Ok(())
    } else {
        Err(McpError::ToolExecution(
            "input does not match schema type".to_owned(),
        ))
    }
}

fn instance_matches(kind: &InstanceType, value: &serde_json::Value) -> bool {
    match kind {
        InstanceType::Null => value.is_null(),
        InstanceType::Boolean => value.is_boolean(),
        InstanceType::Object => value.is_object(),
        InstanceType::Array => value.is_array(),
        InstanceType::Number => value.is_number(),
        InstanceType::Integer => value.as_i64().is_some() || value.as_u64().is_some(),
        InstanceType::String => value.is_string(),
    }
}
