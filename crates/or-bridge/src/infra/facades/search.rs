//! Bridge entry point for `or-tools-search`. Each provider is wired
//! through its own `from_env` / `with_endpoint` constructor and wrapped
//! in an `Arc<dyn SearchProvider>` for `SearchOrchestrator`.

use super::helpers::{
    block_on, from_field, get_str, invocation, json_value, required_str, unsupported,
    unsupported_provider,
};
use crate::domain::errors::BridgeError;
use or_tools_search::infra::{
    bing::BingSearch, brave::BraveSearch, exa::ExaSearch, searxng::SearxngSearch,
    serper::SerperSearch, tavily::TavilySearch, youcom::YouComSearch,
};
use or_tools_search::{SearchOrchestrator, SearchProvider, SearchQuery};
use serde_json::Value;
use std::sync::Arc;

pub(crate) fn invoke(operation: &str, payload: Value) -> Result<Value, BridgeError> {
    if operation != "search" {
        return Err(unsupported("or-tools-search", operation));
    }
    let provider_name = required_str(&payload, "provider", "or-tools-search", operation)?;
    let query: SearchQuery = from_field(&payload, "query", "or-tools-search", operation)?;
    let provider = build_search_provider(provider_name, payload.get("config"))?;
    let orchestrator = SearchOrchestrator::new(vec![provider]);
    block_on("or-tools-search", operation, orchestrator.search(query)).and_then(json_value)
}

fn build_search_provider(
    provider: &str,
    config: Option<&Value>,
) -> Result<Arc<dyn SearchProvider>, BridgeError> {
    let cfg = config.and_then(Value::as_object);
    let client = reqwest::Client::new();
    let provider = match provider {
        "tavily" => Arc::new(
            if let (Some(endpoint), Some(api_key)) = (
                cfg.and_then(|v| get_str(v, "endpoint")),
                cfg.and_then(|v| get_str(v, "api_key")),
            ) {
                TavilySearch::with_endpoint(client, endpoint, api_key)
            } else {
                TavilySearch::from_env()
                    .map_err(|error| invocation("or-tools-search", "search", error))?
            },
        ) as Arc<dyn SearchProvider>,
        "exa" => Arc::new(
            if let (Some(endpoint), Some(api_key)) = (
                cfg.and_then(|v| get_str(v, "endpoint")),
                cfg.and_then(|v| get_str(v, "api_key")),
            ) {
                ExaSearch::with_endpoint(client, endpoint, api_key)
            } else {
                ExaSearch::from_env()
                    .map_err(|error| invocation("or-tools-search", "search", error))?
            },
        ),
        "brave" => Arc::new(
            if let (Some(endpoint), Some(api_key)) = (
                cfg.and_then(|v| get_str(v, "endpoint")),
                cfg.and_then(|v| get_str(v, "api_key")),
            ) {
                BraveSearch::with_endpoint(client, endpoint, api_key)
            } else {
                BraveSearch::from_env()
                    .map_err(|error| invocation("or-tools-search", "search", error))?
            },
        ),
        "serper" => Arc::new(
            if let (Some(endpoint), Some(api_key)) = (
                cfg.and_then(|v| get_str(v, "endpoint")),
                cfg.and_then(|v| get_str(v, "api_key")),
            ) {
                SerperSearch::with_endpoint(client, endpoint, api_key)
            } else {
                SerperSearch::from_env()
                    .map_err(|error| invocation("or-tools-search", "search", error))?
            },
        ),
        "searxng" => Arc::new(
            if let Some(endpoint) = cfg.and_then(|v| get_str(v, "endpoint")) {
                SearxngSearch::with_endpoint(client, endpoint)
            } else {
                SearxngSearch::from_env()
                    .map_err(|error| invocation("or-tools-search", "search", error))?
            },
        ),
        "youcom" => Arc::new(
            if let (Some(endpoint), Some(api_key)) = (
                cfg.and_then(|v| get_str(v, "endpoint")),
                cfg.and_then(|v| get_str(v, "api_key")),
            ) {
                YouComSearch::with_endpoint(client, endpoint, api_key)
            } else {
                YouComSearch::from_env()
                    .map_err(|error| invocation("or-tools-search", "search", error))?
            },
        ),
        "bing" => Arc::new(
            if let (Some(endpoint), Some(api_key)) = (
                cfg.and_then(|v| get_str(v, "endpoint")),
                cfg.and_then(|v| get_str(v, "api_key")),
            ) {
                BingSearch::with_endpoint(client, endpoint, api_key)
            } else {
                BingSearch::from_env()
                    .map_err(|error| invocation("or-tools-search", "search", error))?
            },
        ),
        other => return Err(unsupported_provider("or-tools-search", other)),
    };
    Ok(provider)
}
