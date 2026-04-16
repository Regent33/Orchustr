use crate::domain::contracts::{JsonSchemaOutput, StructuredParser};
use crate::domain::entities::PlainText;
use crate::domain::errors::SieveError;
use crate::infra::adapters::validate_against_schema;
use std::marker::PhantomData;

#[derive(Debug, Clone, Default)]
pub struct JsonParser<T: JsonSchemaOutput> {
    _marker: PhantomData<T>,
}

impl<T: JsonSchemaOutput> JsonParser<T> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl<T: JsonSchemaOutput> StructuredParser<T> for JsonParser<T> {
    fn parse(&self, raw: &str) -> Result<T, SieveError> {
        let value = serde_json::from_str(raw)
            .map_err(|error| SieveError::InvalidJson(error.to_string()))?;
        validate_against_schema::<T>(&value)?;
        serde_json::from_value(value)
            .map_err(|error| SieveError::Deserialization(error.to_string()))
    }
}

#[derive(Debug, Clone, Default)]
pub struct TextParser;

impl TextParser {
    pub fn parse(&self, raw: &str) -> Result<PlainText, SieveError> {
        if raw.trim().is_empty() {
            Err(SieveError::EmptyText)
        } else {
            Ok(PlainText {
                text: raw.trim().to_owned(),
            })
        }
    }
}
