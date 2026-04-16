use crate::domain::contracts::JsonSchemaOutput;
use crate::domain::errors::SieveError;
use schemars::schema::{InstanceType, RootSchema, Schema, SchemaObject, SingleOrVec};
use serde_json::Value;

pub(crate) fn validate_against_schema<T: JsonSchemaOutput>(
    value: &Value,
) -> Result<(), SieveError> {
    let schema = T::output_schema();
    validate_root(&schema, value)
}

fn validate_root(schema: &RootSchema, value: &Value) -> Result<(), SieveError> {
    validate_schema(&Schema::Object(schema.schema.clone()), value, "$")
}

fn validate_schema(schema: &Schema, value: &Value, path: &str) -> Result<(), SieveError> {
    match schema {
        Schema::Bool(true) => Ok(()),
        Schema::Bool(false) => Err(violation(path, "value rejected by schema")),
        Schema::Object(object) => {
            validate_instance_type(object, value, path)?;
            if let Some(validation) = &object.object {
                let Some(map) = value.as_object() else {
                    return Err(violation(path, "expected object"));
                };
                for key in &validation.required {
                    if !map.contains_key(key) {
                        return Err(violation(
                            &format!("{path}.{key}"),
                            "missing required property",
                        ));
                    }
                }
                for (key, nested_schema) in &validation.properties {
                    if let Some(nested_value) = map.get(key) {
                        validate_schema(nested_schema, nested_value, &format!("{path}.{key}"))?;
                    }
                }
            }
            if let Some(validation) = &object.array {
                let Some(items) = value.as_array() else {
                    return Err(violation(path, "expected array"));
                };
                if let Some(nested) = &validation.items {
                    for (index, item) in items.iter().enumerate() {
                        validate_array_item(nested, item, &format!("{path}[{index}]"))?;
                    }
                }
            }
            Ok(())
        }
    }
}

fn validate_instance_type(
    object: &SchemaObject,
    value: &Value,
    path: &str,
) -> Result<(), SieveError> {
    let Some(instance_type) = &object.instance_type else {
        return Ok(());
    };
    if matches_instance_type(value, instance_type) {
        Ok(())
    } else {
        Err(violation(path, "value does not match expected type"))
    }
}

fn matches_instance_type(value: &Value, instance_type: &SingleOrVec<InstanceType>) -> bool {
    match instance_type {
        SingleOrVec::Single(single) => matches_instance(value, single.as_ref()),
        SingleOrVec::Vec(list) => list.iter().any(|item| matches_instance(value, item)),
    }
}

fn matches_instance(value: &Value, instance_type: &InstanceType) -> bool {
    match instance_type {
        InstanceType::Null => value.is_null(),
        InstanceType::Boolean => value.is_boolean(),
        InstanceType::Object => value.is_object(),
        InstanceType::Array => value.is_array(),
        InstanceType::Number => value.is_number(),
        InstanceType::Integer => value.as_i64().is_some() || value.as_u64().is_some(),
        InstanceType::String => value.is_string(),
    }
}

fn validate_array_item(
    schema: &SingleOrVec<Schema>,
    value: &Value,
    path: &str,
) -> Result<(), SieveError> {
    match schema {
        SingleOrVec::Single(single) => validate_schema(single, value, path),
        SingleOrVec::Vec(list) => {
            if let Some(first) = list.first() {
                validate_schema(first, value, path)
            } else {
                Ok(())
            }
        }
    }
}

fn violation(path: &str, message: &str) -> SieveError {
    SieveError::SchemaViolation {
        path: path.to_owned(),
        message: message.to_owned(),
    }
}
