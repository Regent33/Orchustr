use or_mcp::McpTool;
use schemars::schema::RootSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForgeTool {
    pub name: String,
    pub description: String,
    pub input_schema: RootSchema,
}

impl ForgeTool {
    #[must_use]
    pub fn from_mcp(tool: McpTool) -> Self {
        Self {
            name: tool.name,
            description: tool.description,
            input_schema: tool.input_schema,
        }
    }
}
