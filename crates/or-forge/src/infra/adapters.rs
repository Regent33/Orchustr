use crate::domain::errors::ForgeError;
use schemars::schema::{InstanceType, RootSchema, Schema, SingleOrVec};

pub(crate) fn validate_tool_args(
    schema: &RootSchema,
    value: &serde_json::Value,
) -> Result<(), ForgeError> {
    validate_schema(&Schema::Object(schema.schema.clone()), value)
}

fn validate_schema(schema: &Schema, value: &serde_json::Value) -> Result<(), ForgeError> {
    match schema {
        Schema::Bool(true) => Ok(()),
        Schema::Bool(false) => Err(ForgeError::InvalidArguments(
            "schema rejected value".to_owned(),
        )),
        Schema::Object(object) => {
            if let Some(instance_type) = &object.instance_type {
                validate_type(instance_type, value)?;
            }
            if let Some(validation) = &object.object {
                let map = value.as_object().ok_or_else(|| {
                    ForgeError::InvalidArguments("expected object arguments".to_owned())
                })?;
                for key in &validation.required {
                    if !map.contains_key(key) {
                        return Err(ForgeError::InvalidArguments(format!(
                            "missing required argument: {key}"
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
) -> Result<(), ForgeError> {
    let matches = match types {
        SingleOrVec::Single(kind) => instance_matches(kind.as_ref(), value),
        SingleOrVec::Vec(kinds) => kinds.iter().any(|kind| instance_matches(kind, value)),
    };
    if matches {
        Ok(())
    } else {
        Err(ForgeError::InvalidArguments(
            "argument value does not match schema type".to_owned(),
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
