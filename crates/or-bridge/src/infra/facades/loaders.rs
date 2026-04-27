//! Bridge entry point for `or-tools-loaders` (text/markdown/HTML/JSON
//! /CSV/PDF document loading).

use super::helpers::{block_on, from_field, json_value, unsupported};
use crate::domain::errors::BridgeError;
use or_tools_loaders::infra::{
    csv_loader::CsvLoader, html::HtmlLoader, json::JsonLoader, markdown::MarkdownLoader,
    pdf::PdfLoader, text::TextLoader,
};
use or_tools_loaders::{LoaderOrchestrator, LoaderRequest};
use serde_json::Value;
use std::sync::Arc;

pub(crate) fn invoke(operation: &str, payload: Value) -> Result<Value, BridgeError> {
    if operation != "load" {
        return Err(unsupported("or-tools-loaders", operation));
    }
    let request: LoaderRequest = from_field(&payload, "request", "or-tools-loaders", operation)?;
    let mut orchestrator = LoaderOrchestrator::new();
    orchestrator.register(Arc::new(TextLoader));
    orchestrator.register(Arc::new(MarkdownLoader));
    orchestrator.register(Arc::new(HtmlLoader));
    orchestrator.register(Arc::new(JsonLoader));
    orchestrator.register(Arc::new(CsvLoader));
    orchestrator.register(Arc::new(PdfLoader));
    block_on("or-tools-loaders", operation, orchestrator.load(request)).and_then(json_value)
}
