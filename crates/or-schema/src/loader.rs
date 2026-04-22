use crate::GraphSpec;
use std::error::Error;
use std::fmt::{Display, Formatter};

/// Errors returned while loading or serializing `or-schema` graph descriptors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SchemaError {
    /// JSON serialization or deserialization failed.
    Json(String),
    /// YAML support is not enabled for this build.
    YamlFeatureDisabled,
    /// YAML serialization or deserialization failed.
    Yaml(String),
}

impl Display for SchemaError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Json(message) => write!(f, "graph schema JSON error: {message}"),
            Self::YamlFeatureDisabled => {
                write!(f, "graph schema YAML support requires the `yaml` feature")
            }
            Self::Yaml(message) => write!(f, "graph schema YAML error: {message}"),
        }
    }
}

impl Error for SchemaError {}

impl GraphSpec {
    /// Parses a graph spec from JSON text for the `or-schema` crate.
    pub fn from_json(input: &str) -> Result<Self, SchemaError> {
        serde_json::from_str(input).map_err(|error| SchemaError::Json(error.to_string()))
    }

    /// Parses a graph spec from YAML text for the `or-schema` crate.
    pub fn from_yaml(input: &str) -> Result<Self, SchemaError> {
        #[cfg(feature = "yaml")]
        {
            serde_yaml::from_str(input).map_err(|error| SchemaError::Yaml(error.to_string()))
        }
        #[cfg(not(feature = "yaml"))]
        {
            let _ = input;
            Err(SchemaError::YamlFeatureDisabled)
        }
    }

    /// Serializes a graph spec to JSON text for the `or-schema` crate.
    pub fn to_json(&self) -> Result<String, SchemaError> {
        serde_json::to_string_pretty(self).map_err(|error| SchemaError::Json(error.to_string()))
    }

    /// Serializes a graph spec to YAML text for the `or-schema` crate.
    pub fn to_yaml(&self) -> Result<String, SchemaError> {
        #[cfg(feature = "yaml")]
        {
            serde_yaml::to_string(self).map_err(|error| SchemaError::Yaml(error.to_string()))
        }
        #[cfg(not(feature = "yaml"))]
        {
            Err(SchemaError::YamlFeatureDisabled)
        }
    }
}
