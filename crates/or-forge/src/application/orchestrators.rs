use crate::domain::entities::ForgeTool;
use crate::domain::errors::ForgeError;
use crate::infra::implementations::{RegistryStore, invoke_tool, local_handler, proxy_handler};
use or_mcp::NexusClientTrait;

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
        let tools = client
            .list_tools()
            .await
            .map_err(|error| ForgeError::Invocation(error.to_string()))?;
        for tool in tools {
            let forge_tool = ForgeTool::from_mcp(tool);
            let name = forge_tool.name.clone();
            self.store
                .insert(forge_tool, proxy_handler(client.clone(), name))?;
        }
        Ok(self.len())
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
}
