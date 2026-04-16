use crate::domain::errors::SieveError;

pub trait JsonSchemaOutput:
    serde::de::DeserializeOwned + schemars::JsonSchema + Send + Sync + 'static
{
    fn output_schema() -> schemars::schema::RootSchema
    where
        Self: Sized,
    {
        schemars::schema_for!(Self)
    }
}

impl<T> JsonSchemaOutput for T where
    T: serde::de::DeserializeOwned + schemars::JsonSchema + Send + Sync + 'static
{
}

#[cfg_attr(test, mockall::automock)]
pub trait StructuredParser<T: JsonSchemaOutput>: Send + Sync {
    fn parse(&self, raw: &str) -> Result<T, SieveError>;
}
