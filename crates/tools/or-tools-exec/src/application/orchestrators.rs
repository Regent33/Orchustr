use crate::domain::contracts::CodeExecutor;
use crate::domain::entities::{ExecRequest, ExecResult};
use crate::domain::errors::ExecError;
use async_trait::async_trait;
use or_tools_core::{Tool, ToolCapability, ToolError, ToolInput, ToolMeta, ToolOutput};
use std::sync::Arc;

pub struct ExecOrchestrator {
    executors: Vec<Arc<dyn CodeExecutor>>,
}

impl ExecOrchestrator {
    #[must_use]
    pub fn new(executors: Vec<Arc<dyn CodeExecutor>>) -> Self {
        Self { executors }
    }

    pub async fn execute(&self, req: ExecRequest) -> Result<ExecResult, ExecError> {
        let executor = self
            .executors
            .iter()
            .find(|e| e.supports(req.language))
            .ok_or_else(|| ExecError::UnsupportedLanguage(req.language.as_str().into()))?;
        let span = tracing::info_span!(
            "tools.exec.execute",
            otel.name = "tools.exec.execute",
            executor = executor.name(),
            language = req.language.as_str(),
            status = tracing::field::Empty,
        );
        let _guard = span.enter();
        let result = executor.execute(req).await;
        span.record("status", if result.is_ok() { "success" } else { "failure" });
        result
    }
}

pub struct ExecTool {
    orchestrator: Arc<ExecOrchestrator>,
}

impl ExecTool {
    #[must_use]
    pub fn new(orchestrator: Arc<ExecOrchestrator>) -> Self {
        Self { orchestrator }
    }
}

#[async_trait]
impl Tool for ExecTool {
    fn meta(&self) -> ToolMeta {
        ToolMeta::new(
            "exec",
            "Code execution — Python, Shell, and sandboxed runtimes",
        )
        .with_capability(ToolCapability::Subprocess)
    }

    async fn invoke(&self, input: ToolInput) -> Result<ToolOutput, ToolError> {
        let req: ExecRequest = serde_json::from_value(input.payload)
            .map_err(|e| ToolError::invalid_input(&input.tool, e.to_string()))?;
        let result = self.orchestrator.execute(req).await?;
        let payload = serde_json::to_value(&result).map_err(|e| ToolError::Serialization {
            tool: input.tool.clone(),
            reason: e.to_string(),
        })?;
        Ok(ToolOutput::new(input.tool, payload))
    }
}
