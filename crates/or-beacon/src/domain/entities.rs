use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PromptTemplate {
    pub template: String,
    pub variables: Vec<String>,
}
