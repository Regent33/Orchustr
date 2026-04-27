use crate::domain::contracts::DocumentLoader;
use crate::domain::entities::{Document, DocumentKind, LoaderRequest, LoaderSource};
use crate::domain::errors::LoaderError;
use async_trait::async_trait;
use or_tools_core::{Tool, ToolCapability, ToolError, ToolInput, ToolMeta, ToolOutput};
use std::collections::HashMap;
use std::sync::Arc;

/// Routes a [`LoaderRequest`] to the correct [`DocumentLoader`] by document kind.
pub struct LoaderOrchestrator {
    loaders: HashMap<String, Arc<dyn DocumentLoader>>,
}

impl LoaderOrchestrator {
    #[must_use]
    pub fn new() -> Self {
        Self {
            loaders: HashMap::new(),
        }
    }

    pub fn register(&mut self, loader: Arc<dyn DocumentLoader>) {
        self.loaders.insert(loader.name().to_string(), loader);
    }

    pub async fn load(&self, req: LoaderRequest) -> Result<Vec<Document>, LoaderError> {
        let kind = resolve_kind(&req);
        let name = kind_to_loader_name(kind);
        let loader = self
            .loaders
            .get(name)
            .ok_or_else(|| LoaderError::UnsupportedFormat(format!("{kind:?}")))?;
        let span = tracing::info_span!(
            "tools.loader.load",
            otel.name = "tools.loader.load",
            loader = %name,
            status = tracing::field::Empty,
        );
        let _guard = span.enter();
        let result = loader.load(req).await;
        span.record("status", if result.is_ok() { "success" } else { "failure" });
        result
    }
}

impl Default for LoaderOrchestrator {
    fn default() -> Self {
        Self::new()
    }
}

fn resolve_kind(req: &LoaderRequest) -> DocumentKind {
    if let Some(hint) = req.kind_hint {
        return hint;
    }
    if let LoaderSource::Path { path } = &req.source {
        let ext = path.rsplit('.').next().unwrap_or("");
        return DocumentKind::from_extension(ext);
    }
    DocumentKind::Text
}

fn kind_to_loader_name(kind: DocumentKind) -> &'static str {
    match kind {
        DocumentKind::Text | DocumentKind::Unknown => "text",
        DocumentKind::Markdown => "markdown",
        DocumentKind::Html => "html",
        DocumentKind::Json => "json",
        DocumentKind::Csv => "csv",
        DocumentKind::Pdf => "pdf",
        DocumentKind::Docx => "docx",
    }
}

pub struct LoaderTool {
    orchestrator: Arc<LoaderOrchestrator>,
}

impl LoaderTool {
    #[must_use]
    pub fn new(orchestrator: Arc<LoaderOrchestrator>) -> Self {
        Self { orchestrator }
    }
}

#[async_trait]
impl Tool for LoaderTool {
    fn meta(&self) -> ToolMeta {
        ToolMeta::new(
            "loader",
            "Document loader — text, markdown, HTML, JSON, CSV, PDF",
        )
        .with_capability(ToolCapability::Filesystem)
    }

    async fn invoke(&self, input: ToolInput) -> Result<ToolOutput, ToolError> {
        let req: LoaderRequest = serde_json::from_value(input.payload)
            .map_err(|e| ToolError::invalid_input(&input.tool, e.to_string()))?;
        let docs = self.orchestrator.load(req).await?;
        let payload = serde_json::to_value(&docs).map_err(|e| ToolError::Serialization {
            tool: input.tool.clone(),
            reason: e.to_string(),
        })?;
        Ok(ToolOutput::new(input.tool, payload))
    }
}
