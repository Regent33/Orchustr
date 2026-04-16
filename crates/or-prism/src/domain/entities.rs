use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PrismConfig {
    pub otlp_endpoint: String,
    pub service_name: String,
}
