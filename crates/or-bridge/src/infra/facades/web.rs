//! Bridge entry point for `or-tools-web` (HTTP fetch + scraping).

use super::helpers::{
    block_on, from_field, get_str, invocation, json_value, required_str, unsupported,
    unsupported_provider,
};
use crate::domain::errors::BridgeError;
use or_tools_web::infra::{
    agentql::AgentQlScraper, brightdata::BrightDataScraper, http_client::RequestsClient,
    hyperbrowser::HyperbrowserClient, oxylabs::OxylabsScraper, playwright::PlaywrightBrowser,
};
use or_tools_web::{FetchRequest, Scraper, WebBrowser, WebOrchestrator};
use serde_json::Value;
use std::sync::Arc;

pub(crate) fn invoke(operation: &str, payload: Value) -> Result<Value, BridgeError> {
    match operation {
        "fetch" => {
            let provider_name = required_str(&payload, "provider", "or-tools-web", operation)?;
            let request: FetchRequest = from_field(&payload, "request", "or-tools-web", operation)?;
            let browser = build_web_browser(provider_name, payload.get("config"))?;
            let orchestrator = WebOrchestrator::new(browser);
            block_on("or-tools-web", operation, orchestrator.fetch(request)).and_then(json_value)
        }
        "scrape" => {
            let provider_name = required_str(&payload, "provider", "or-tools-web", operation)?;
            let url = required_str(&payload, "url", "or-tools-web", operation)?.to_owned();
            let scraper = build_scraper(provider_name, payload.get("config"))?;
            block_on("or-tools-web", operation, scraper.scrape(&url)).and_then(json_value)
        }
        _ => Err(unsupported("or-tools-web", operation)),
    }
}

fn build_web_browser(
    provider: &str,
    config: Option<&Value>,
) -> Result<Arc<dyn WebBrowser>, BridgeError> {
    let cfg = config.and_then(Value::as_object);
    let client = reqwest::Client::new();
    let browser = match provider {
        "requests" => Arc::new(RequestsClient::new()) as Arc<dyn WebBrowser>,
        "playwright" => Arc::new(
            if let Some(endpoint) = cfg.and_then(|v| get_str(v, "endpoint")) {
                PlaywrightBrowser::with_endpoint(client, endpoint)
            } else {
                PlaywrightBrowser::from_env()
                    .map_err(|error| invocation("or-tools-web", "fetch", error))?
            },
        ),
        "brightdata" => Arc::new(
            if let (Some(endpoint), Some(token)) = (
                cfg.and_then(|v| get_str(v, "endpoint")),
                cfg.and_then(|v| get_str(v, "token")),
            ) {
                BrightDataScraper::with_endpoint(
                    client,
                    endpoint,
                    token,
                    cfg.and_then(|v| get_str(v, "zone"))
                        .unwrap_or("web_unlocker"),
                )
            } else {
                BrightDataScraper::from_env()
                    .map_err(|error| invocation("or-tools-web", "fetch", error))?
            },
        ),
        "hyperbrowser" => Arc::new(
            if let (Some(endpoint), Some(api_key)) = (
                cfg.and_then(|v| get_str(v, "endpoint")),
                cfg.and_then(|v| get_str(v, "api_key")),
            ) {
                HyperbrowserClient::with_endpoint(client, endpoint, api_key)
            } else {
                HyperbrowserClient::from_env()
                    .map_err(|error| invocation("or-tools-web", "fetch", error))?
            },
        ),
        "oxylabs" => Arc::new(
            if let (Some(endpoint), Some(username), Some(password)) = (
                cfg.and_then(|v| get_str(v, "endpoint")),
                cfg.and_then(|v| get_str(v, "username")),
                cfg.and_then(|v| get_str(v, "password")),
            ) {
                OxylabsScraper::with_credentials(client, endpoint, username, password)
            } else {
                OxylabsScraper::from_env()
                    .map_err(|error| invocation("or-tools-web", "fetch", error))?
            },
        ),
        other => return Err(unsupported_provider("or-tools-web", other)),
    };
    Ok(browser)
}

fn build_scraper(provider: &str, config: Option<&Value>) -> Result<Arc<dyn Scraper>, BridgeError> {
    let cfg = config.and_then(Value::as_object);
    match provider {
        "agentql" => {
            let scraper = if let Some(prompt) = cfg.and_then(|v| get_str(v, "prompt")) {
                AgentQlScraper::from_env()
                    .map_err(|error| invocation("or-tools-web", "scrape", error))?
                    .with_prompt(prompt)
            } else {
                AgentQlScraper::from_env()
                    .map_err(|error| invocation("or-tools-web", "scrape", error))?
            };
            Ok(Arc::new(scraper))
        }
        other => Err(unsupported_provider("or-tools-web", other)),
    }
}
