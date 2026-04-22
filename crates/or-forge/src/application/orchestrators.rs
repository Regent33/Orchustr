use crate::domain::entities::ForgeTool;
use crate::domain::errors::ForgeError;
use crate::infra::implementations::{RegistryStore, invoke_tool, local_handler, proxy_handler};
use or_mcp::{MultiMcpClient, MultiMcpSession, NexusClient, NexusClientTrait};
use serde::{Deserialize, Serialize};

/// Summary metadata returned by additive MCP import helpers in `or-forge`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ImportSummary {
    /// Number of tools imported by the current operation.
    pub tools_imported: usize,
    /// Names registered into the `ForgeRegistry`.
    pub tool_names: Vec<String>,
    /// Optional server name discovered during import.
    pub server_name: Option<String>,
}

#[derive(Clone, Default)]
pub struct ForgeRegistry {
    store: RegistryStore,
}

impl ForgeRegistry {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register<F, Fut>(&mut self, tool: ForgeTool, handler: F) -> Result<(), ForgeError>
    where
        F: Fn(serde_json::Value) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = Result<serde_json::Value, ForgeError>> + Send + 'static,
    {
        self.store.insert(tool, local_handler(handler))
    }

    pub async fn import_from_mcp<C>(&mut self, client: &C) -> Result<usize, ForgeError>
    where
        C: NexusClientTrait + Clone + Send + Sync + 'static,
    {
        let _ = self.import_tools_from_client(client.clone()).await?;
        Ok(self.len())
    }

    /// Discovers and imports all tools from a single HTTP MCP server.
    pub async fn import_all_from_mcp(
        &mut self,
        server_url: &str,
    ) -> Result<ImportSummary, ForgeError> {
        let client = NexusClient::connect_http(server_url.to_owned());
        let server_name = client
            .initialize()
            .await
            .ok()
            .and_then(|value| {
                value
                    .get("serverInfo")
                    .and_then(|info| info.get("name"))
                    .and_then(serde_json::Value::as_str)
                    .map(ToOwned::to_owned)
            });
        let tool_names = self.import_tools_from_client(client).await?;
        Ok(ImportSummary {
            tools_imported: tool_names.len(),
            tool_names,
            server_name,
        })
    }

    /// Connects a `MultiMcpClient` and imports all discovered tools into this registry.
    pub async fn import_all_from_multi_mcp(
        &mut self,
        client: MultiMcpClient,
    ) -> Result<ImportSummary, ForgeError> {
        let session = client
            .connect_all()
            .await
            .map_err(|error| ForgeError::Invocation(error.to_string()))?;
        self.import_all_from_multi_session(session).await
    }

    /// Imports all tools from an already-connected `MultiMcpSession`.
    pub async fn import_all_from_multi_session(
        &mut self,
        session: MultiMcpSession,
    ) -> Result<ImportSummary, ForgeError> {
        let mut tool_names = Vec::new();
        for discovered in session.tools() {
            let registered_name = discovered.registered_name.clone();
            let handler_name = registered_name.clone();
            let handler_session = session.clone();
            self.register(
                ForgeTool {
                    name: registered_name.clone(),
                    description: discovered.tool.description.clone(),
                    input_schema: discovered.tool.input_schema.clone(),
                },
                move |args| {
                    let session = handler_session.clone();
                    let name = handler_name.clone();
                    async move {
                        session
                            .invoke(&name, args)
                            .await
                            .map_err(|error| ForgeError::Invocation(error.to_string()))
                    }
                },
            )?;
            tool_names.push(registered_name);
        }
        Ok(ImportSummary {
            tools_imported: tool_names.len(),
            tool_names,
            server_name: None,
        })
    }

    pub async fn invoke(
        &self,
        name: &str,
        args: serde_json::Value,
    ) -> Result<serde_json::Value, ForgeError> {
        let span = tracing::info_span!(
            "forge.invoke",
            otel.name = "forge.invoke",
            tool = name,
            status = tracing::field::Empty
        );
        let _guard = span.enter();
        let result = invoke_tool(&self.store, name, args).await;
        span.record("status", if result.is_ok() { "success" } else { "failure" });
        result
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.store.tools.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.store.tools.is_empty()
    }

    async fn import_tools_from_client<C>(&mut self, client: C) -> Result<Vec<String>, ForgeError>
    where
        C: NexusClientTrait + Clone + Send + Sync + 'static,
    {
        let tools = client
            .list_tools()
            .await
            .map_err(|error| ForgeError::Invocation(error.to_string()))?;
        let mut tool_names = Vec::with_capacity(tools.len());
        for tool in tools {
            let forge_tool = ForgeTool::from_mcp(tool);
            let name = forge_tool.name.clone();
            self.store
                .insert(forge_tool, proxy_handler(client.clone(), name.clone()))?;
            tool_names.push(name);
        }
        Ok(tool_names)
    }
}
