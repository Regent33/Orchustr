use crate::domain::entities::ForgeTool;
use crate::domain::errors::ForgeError;
use crate::infra::adapters::validate_tool_args;
use or_mcp::NexusClientTrait;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

type ToolFuture = Pin<Box<dyn Future<Output = Result<serde_json::Value, ForgeError>> + 'static>>;
type ToolHandler = Arc<dyn Fn(serde_json::Value) -> ToolFuture + Send + Sync + 'static>;

#[derive(Clone)]
pub(crate) struct RegisteredTool {
    pub tool: ForgeTool,
    pub handler: ToolHandler,
}

#[derive(Clone, Default)]
pub struct RegistryStore {
    pub(crate) tools: std::collections::HashMap<String, RegisteredTool>,
}

impl RegistryStore {
    pub fn insert(&mut self, tool: ForgeTool, handler: ToolHandler) -> Result<(), ForgeError> {
        if self.tools.contains_key(&tool.name) {
            return Err(ForgeError::DuplicateTool(tool.name));
        }
        self.tools
            .insert(tool.name.clone(), RegisteredTool { tool, handler });
        Ok(())
    }
}

pub(crate) fn local_handler<F, Fut>(handler: F) -> ToolHandler
where
    F: Fn(serde_json::Value) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<serde_json::Value, ForgeError>> + Send + 'static,
{
    Arc::new(move |args| Box::pin(handler(args)))
}

pub(crate) fn proxy_handler<C>(client: C, tool_name: String) -> ToolHandler
where
    C: NexusClientTrait + Clone + Send + Sync + 'static,
{
    Arc::new(move |args| {
        let client = client.clone();
        let tool_name = tool_name.clone();
        Box::pin(async move {
            client
                .invoke_tool(&tool_name, args)
                .await
                .map_err(|error| ForgeError::Invocation(error.to_string()))
        })
    })
}

pub(crate) async fn invoke_tool(
    store: &RegistryStore,
    name: &str,
    args: serde_json::Value,
) -> Result<serde_json::Value, ForgeError> {
    const MAX_ARG_BYTES: usize = 1_048_576; // 1 MB
    let arg_size = serde_json::to_string(&args)
        .map(|s| s.len())
        .unwrap_or(0);
    if arg_size > MAX_ARG_BYTES {
        return Err(ForgeError::InvalidArguments(format!(
            "argument payload too large: {arg_size} bytes (max {MAX_ARG_BYTES})"
        )));
    }
    let entry = store
        .tools
        .get(name)
        .ok_or_else(|| ForgeError::UnknownTool(name.to_owned()))?;
    validate_tool_args(&entry.tool.input_schema, &args)?;
    (entry.handler)(args).await
}
