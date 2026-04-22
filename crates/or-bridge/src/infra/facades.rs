use crate::domain::errors::BridgeError;
use serde::Serialize;
use serde_json::{Value, json};
use std::future::Future;
use std::sync::{Arc, OnceLock};

use or_beacon::PromptOrchestrator;
use or_conduit::ConduitOrchestrator;
use or_core::{CoreOrchestrator, RetryPolicy, TokenBudget};
use or_prism::install_global_subscriber;
use or_sieve::{SieveOrchestrator, TextParser};

use or_tools_search::infra::{
    bing::BingSearch, brave::BraveSearch, exa::ExaSearch, searxng::SearxngSearch,
    serper::SerperSearch, tavily::TavilySearch, youcom::YouComSearch,
};
use or_tools_search::{SearchOrchestrator, SearchProvider, SearchQuery};

use or_tools_web::infra::{
    agentql::AgentQlScraper, brightdata::BrightDataScraper, http_client::RequestsClient,
    hyperbrowser::HyperbrowserClient, oxylabs::OxylabsScraper, playwright::PlaywrightBrowser,
};
use or_tools_web::{FetchRequest, Scraper, WebBrowser, WebOrchestrator};

use or_tools_vector::infra::{
    chroma::ChromaClient, milvus::MilvusClient, pgvector::PgVectorClient,
    pinecone::PineconeClient, qdrant::QdrantClient, weaviate::WeaviateClient,
};
use or_tools_vector::{
    CollectionConfig, DeleteRequest, QueryFilter, UpsertBatch, VectorStoreClient,
};

use or_tools_loaders::infra::{
    csv_loader::CsvLoader, html::HtmlLoader, json::JsonLoader, markdown::MarkdownLoader,
    pdf::PdfLoader, text::TextLoader,
};
use or_tools_loaders::{LoaderOrchestrator, LoaderRequest};

use or_tools_exec::infra::{
    bearly::BearlyExecutor, daytona::DaytonaExecutor, e2b::E2BExecutor,
    python::PythonExecutor, shell::ShellExecutor,
};
use or_tools_exec::{CodeExecutor, ExecOrchestrator, ExecRequest};

use or_tools_file::infra::{
    arxiv::ArxivSource, financial::FinancialDatasetsSource, gdrive::GoogleDriveStore,
    json_toolkit::JsonToolkit, local_fs::LocalFileSystem,
};
use or_tools_file::{DataSource, FileOrchestrator, FileStore};

use or_tools_comms::infra::{
    discord::DiscordSender, facebook::FacebookSender, messenger::MessengerSender,
    telegram::TelegramSender, twilio::TwilioSender, whatsapp::WhatsAppSender,
};
use or_tools_comms::{Channel, CommsOrchestrator, Message, MessageSender};

use or_tools_productivity::infra::{
    clickup::ClickUpTracker, gcalendar::GoogleCalendarClient, github::GitHubTracker,
    gmail::GmailClient, jira::JiraTracker, notion::NotionBase,
    office365::{OutlookCalendarClient, OutlookEmailClient},
    slack::SlackMessenger, trello::TrelloTracker,
};
use or_tools_productivity::{
    CalendarClient, CalendarEvent, Email, EmailClient, Issue, KnowledgeBase, Page,
    ProjectTracker, TeamMessenger,
};

#[derive(Debug, Clone, Serialize)]
struct CrateBinding {
    crate_name: &'static str,
    binding_mode: &'static str,
    description: &'static str,
    operations: &'static [&'static str],
}

const CRATE_BINDINGS: &[CrateBinding] = &[
    CrateBinding {
        crate_name: "or-core",
        binding_mode: "native",
        description: "Core orchestration utilities for token budgets and retry planning.",
        operations: &["enforce_completion_budget", "next_retry_delay"],
    },
    CrateBinding {
        crate_name: "or-beacon",
        binding_mode: "mixed",
        description: "Prompt construction and rendering. Bindings also expose local PromptBuilder helpers.",
        operations: &["render_template"],
    },
    CrateBinding {
        crate_name: "or-bridge",
        binding_mode: "native",
        description: "Native JSON bridge entry points for prompt rendering, state normalization, and crate invocation.",
        operations: &["render_prompt_json", "normalize_state_json"],
    },
    CrateBinding {
        crate_name: "or-checkpoint",
        binding_mode: "language-runtime",
        description: "Checkpoint/pause-resume semantics are exposed through binding-local workflow helpers.",
        operations: &["pause", "resume"],
    },
    CrateBinding {
        crate_name: "or-colony",
        binding_mode: "language-runtime",
        description: "Multi-agent colony coordination is exposed through binding-local workflow helpers.",
        operations: &["coordinate"],
    },
    CrateBinding {
        crate_name: "or-compass",
        binding_mode: "language-runtime",
        description: "Route selection is exposed through binding-local workflow helpers.",
        operations: &["select_route"],
    },
    CrateBinding {
        crate_name: "or-conduit",
        binding_mode: "mixed",
        description: "LLM request preparation plus binding-local provider clients.",
        operations: &["prepare_text_request"],
    },
    CrateBinding {
        crate_name: "or-forge",
        binding_mode: "language-runtime",
        description: "Tool registry and MCP import flows are exposed through binding-local helpers.",
        operations: &["register", "import_from_mcp", "invoke"],
    },
    CrateBinding {
        crate_name: "or-loom",
        binding_mode: "language-runtime",
        description: "Execution-graph building is exposed through binding-local GraphBuilder APIs.",
        operations: &["execute_graph", "resume_graph"],
    },
    CrateBinding {
        crate_name: "or-mcp",
        binding_mode: "language-runtime",
        description: "MCP client helpers are exposed directly in the language bindings.",
        operations: &["connect_http", "list_tools", "invoke_tool"],
    },
    CrateBinding {
        crate_name: "or-pipeline",
        binding_mode: "language-runtime",
        description: "Pipeline composition is exposed through binding-local workflow helpers.",
        operations: &["execute_pipeline"],
    },
    CrateBinding {
        crate_name: "or-prism",
        binding_mode: "native",
        description: "Tracing subscriber setup through the Rust Prism crate.",
        operations: &["install_global_subscriber"],
    },
    CrateBinding {
        crate_name: "or-recall",
        binding_mode: "language-runtime",
        description: "In-memory recall helpers are provided in the bindings.",
        operations: &["remember", "recall"],
    },
    CrateBinding {
        crate_name: "or-relay",
        binding_mode: "language-runtime",
        description: "Parallel branch execution is exposed through binding-local workflow helpers.",
        operations: &["execute_parallel"],
    },
    CrateBinding {
        crate_name: "or-sentinel",
        binding_mode: "language-runtime",
        description: "Agent planning/execution helpers are exposed through binding-local workflow helpers.",
        operations: &["run_agent"],
    },
    CrateBinding {
        crate_name: "or-sieve",
        binding_mode: "mixed",
        description: "Structured/text parsing via the Rust crate plus language-local convenience helpers.",
        operations: &["parse_text"],
    },
    CrateBinding {
        crate_name: "or-tools-core",
        binding_mode: "language-runtime",
        description: "Tool registry and dispatcher helpers are exposed directly in the language bindings.",
        operations: &["register", "invoke", "dispatch"],
    },
    CrateBinding {
        crate_name: "or-tools-search",
        binding_mode: "native",
        description: "Search providers routed through the Rust tool crate.",
        operations: &["search"],
    },
    CrateBinding {
        crate_name: "or-tools-web",
        binding_mode: "native",
        description: "Web fetch and scraping providers routed through the Rust tool crate.",
        operations: &["fetch", "scrape"],
    },
    CrateBinding {
        crate_name: "or-tools-vector",
        binding_mode: "native",
        description: "Vector store operations routed through the Rust tool crate.",
        operations: &["ensure_collection", "upsert", "delete", "query"],
    },
    CrateBinding {
        crate_name: "or-tools-loaders",
        binding_mode: "native",
        description: "Document loading and chunking routed through the Rust tool crate.",
        operations: &["load"],
    },
    CrateBinding {
        crate_name: "or-tools-exec",
        binding_mode: "native",
        description: "Local and remote execution providers routed through the Rust tool crate.",
        operations: &["execute"],
    },
    CrateBinding {
        crate_name: "or-tools-file",
        binding_mode: "native",
        description: "Filesystem and external file/data sources routed through the Rust tool crate.",
        operations: &["read", "write", "list", "delete", "fetch"],
    },
    CrateBinding {
        crate_name: "or-tools-comms",
        binding_mode: "native",
        description: "Outbound messaging providers routed through the Rust tool crate.",
        operations: &["send"],
    },
    CrateBinding {
        crate_name: "or-tools-productivity",
        binding_mode: "native",
        description: "Productivity providers routed through the Rust tool crate.",
        operations: &[
            "list_emails",
            "send_email",
            "list_events",
            "create_event",
            "list_issues",
            "create_issue",
            "search_pages",
            "create_page",
            "post_message",
            "search_messages",
        ],
    },
];

pub(crate) fn workspace_catalog() -> Result<String, BridgeError> {
    serde_json::to_string(CRATE_BINDINGS)
        .map_err(|error| BridgeError::InvalidJson(error.to_string()))
}

pub(crate) fn invoke(crate_name: &str, operation: &str, payload: Value) -> Result<Value, BridgeError> {
    match crate_name {
        "or-core" => invoke_core(operation, payload),
        "or-beacon" => invoke_beacon(operation, payload),
        "or-bridge" => invoke_bridge(operation, payload),
        "or-conduit" => invoke_conduit(operation, payload),
        "or-prism" => invoke_prism(operation, payload),
        "or-sieve" => invoke_sieve(operation, payload),
        "or-tools-search" => invoke_search(operation, payload),
        "or-tools-web" => invoke_web(operation, payload),
        "or-tools-vector" => invoke_vector(operation, payload),
        "or-tools-loaders" => invoke_loaders(operation, payload),
        "or-tools-exec" => invoke_exec(operation, payload),
        "or-tools-file" => invoke_file(operation, payload),
        "or-tools-comms" => invoke_comms(operation, payload),
        "or-tools-productivity" => invoke_productivity(operation, payload),
        other => Err(BridgeError::UnsupportedCrate(other.to_owned())),
    }
}

fn invoke_core(operation: &str, payload: Value) -> Result<Value, BridgeError> {
    let orchestrator = CoreOrchestrator::new();
    match operation {
        "enforce_completion_budget" => {
            let budget: TokenBudget = from_field(&payload, "budget", "or-core", operation)?;
            let prompt_tokens = required_u64(&payload, "prompt_tokens", "or-core", operation)? as u32;
            orchestrator
                .enforce_completion_budget(&budget, prompt_tokens)
                .map_err(|error| invocation("or-core", operation, error))?;
            Ok(json!({ "status": "ok" }))
        }
        "next_retry_delay" => {
            let policy: RetryPolicy = from_field(&payload, "policy", "or-core", operation)?;
            let attempt = required_u64(&payload, "attempt", "or-core", operation)? as u32;
            let delay = orchestrator
                .next_retry_delay(&policy, attempt)
                .map_err(|error| invocation("or-core", operation, error))?;
            Ok(json!({ "delay_ms": delay.as_millis() as u64 }))
        }
        _ => Err(unsupported("or-core", operation)),
    }
}

fn invoke_beacon(operation: &str, payload: Value) -> Result<Value, BridgeError> {
    let orchestrator = PromptOrchestrator;
    match operation {
        "render_template" => {
            let template = required_str(&payload, "template", "or-beacon", operation)?;
            let context = payload
                .get("context")
                .cloned()
                .unwrap_or_else(|| Value::Object(Default::default()));
            let built = orchestrator
                .build_template(template)
                .map_err(|error| invocation("or-beacon", operation, error))?;
            let rendered = orchestrator
                .render_template(&built, &context)
                .map_err(|error| invocation("or-beacon", operation, error))?;
            Ok(json!({ "text": rendered }))
        }
        _ => Err(unsupported("or-beacon", operation)),
    }
}

fn invoke_bridge(operation: &str, payload: Value) -> Result<Value, BridgeError> {
    match operation {
        "render_prompt_json" => {
            let template = required_str(&payload, "template", "or-bridge", operation)?;
            let context = payload
                .get("context")
                .cloned()
                .unwrap_or_else(|| Value::Object(Default::default()));
            let rendered = crate::render_prompt_json(
                template,
                &serde_json::to_string(&context)
                    .map_err(|error| BridgeError::InvalidJson(error.to_string()))?,
            )?;
            Ok(json!({ "text": rendered }))
        }
        "normalize_state_json" => {
            let state = payload
                .get("state")
                .cloned()
                .unwrap_or_else(|| Value::Object(Default::default()));
            let normalized = crate::normalize_state_json(
                &serde_json::to_string(&state)
                    .map_err(|error| BridgeError::InvalidJson(error.to_string()))?,
            )?;
            Ok(json!({ "state": serde_json::from_str::<Value>(&normalized).map_err(|error| BridgeError::InvalidJson(error.to_string()))? }))
        }
        _ => Err(unsupported("or-bridge", operation)),
    }
}

fn invoke_conduit(operation: &str, payload: Value) -> Result<Value, BridgeError> {
    match operation {
        "prepare_text_request" => {
            let prompt = required_str(&payload, "prompt", "or-conduit", operation)?;
            let messages = ConduitOrchestrator
                .prepare_text_request(prompt)
                .map_err(|error| invocation("or-conduit", operation, error))?;
            serde_json::to_value(messages).map_err(|error| BridgeError::InvalidJson(error.to_string()))
        }
        _ => Err(unsupported("or-conduit", operation)),
    }
}

fn invoke_prism(operation: &str, payload: Value) -> Result<Value, BridgeError> {
    match operation {
        "install_global_subscriber" => {
            let endpoint = required_str(&payload, "otlp_endpoint", "or-prism", operation)?;
            install_global_subscriber(endpoint).map_err(|error| invocation("or-prism", operation, error))?;
            Ok(json!({ "status": "ok" }))
        }
        _ => Err(unsupported("or-prism", operation)),
    }
}

fn invoke_sieve(operation: &str, payload: Value) -> Result<Value, BridgeError> {
    match operation {
        "parse_text" => {
            let raw = required_str(&payload, "raw", "or-sieve", operation)?;
            let parsed = SieveOrchestrator
                .parse_text(&TextParser, raw)
                .map_err(|error| invocation("or-sieve", operation, error))?;
            serde_json::to_value(parsed).map_err(|error| BridgeError::InvalidJson(error.to_string()))
        }
        _ => Err(unsupported("or-sieve", operation)),
    }
}

fn invoke_search(operation: &str, payload: Value) -> Result<Value, BridgeError> {
    if operation != "search" {
        return Err(unsupported("or-tools-search", operation));
    }
    let provider_name = required_str(&payload, "provider", "or-tools-search", operation)?;
    let query: SearchQuery = from_field(&payload, "query", "or-tools-search", operation)?;
    let provider = build_search_provider(provider_name, payload.get("config"))?;
    let orchestrator = SearchOrchestrator::new(vec![provider]);
    block_on("or-tools-search", operation, orchestrator.search(query))
        .and_then(json_value)
}

fn invoke_web(operation: &str, payload: Value) -> Result<Value, BridgeError> {
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

fn invoke_vector(operation: &str, payload: Value) -> Result<Value, BridgeError> {
    let provider_name = required_str(&payload, "provider", "or-tools-vector", operation)?;
    let client = build_vector_client(provider_name, payload.get("config"))?;
    match operation {
        "ensure_collection" => {
            let cfg: CollectionConfig = from_field(&payload, "data", "or-tools-vector", operation)?;
            block_on("or-tools-vector", operation, client.ensure_collection(cfg))?;
            Ok(json!({ "status": "ok" }))
        }
        "upsert" => {
            let batch: UpsertBatch = from_field(&payload, "data", "or-tools-vector", operation)?;
            block_on("or-tools-vector", operation, client.upsert(batch))?;
            Ok(json!({ "status": "ok" }))
        }
        "delete" => {
            let req: DeleteRequest = from_field(&payload, "data", "or-tools-vector", operation)?;
            block_on("or-tools-vector", operation, client.delete(req))?;
            Ok(json!({ "status": "ok" }))
        }
        "query" => {
            let filter: QueryFilter = from_field(&payload, "data", "or-tools-vector", operation)?;
            block_on("or-tools-vector", operation, client.query(filter)).and_then(json_value)
        }
        _ => Err(unsupported("or-tools-vector", operation)),
    }
}

fn invoke_loaders(operation: &str, payload: Value) -> Result<Value, BridgeError> {
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

fn invoke_exec(operation: &str, payload: Value) -> Result<Value, BridgeError> {
    if operation != "execute" {
        return Err(unsupported("or-tools-exec", operation));
    }
    let request: ExecRequest = from_field(&payload, "request", "or-tools-exec", operation)?;
    let providers = payload
        .get("providers")
        .and_then(Value::as_array)
        .map(|items| {
            items
                .iter()
                .filter_map(Value::as_str)
                .map(str::to_owned)
                .collect::<Vec<_>>()
        })
        .unwrap_or_else(|| vec!["python".into(), "shell".into()]);
    let config = payload.get("config").and_then(Value::as_object);
    let executors = providers
        .into_iter()
        .map(|provider| build_exec_executor(&provider, config.and_then(|cfg| cfg.get(&provider))))
        .collect::<Result<Vec<_>, _>>()?;
    let orchestrator = ExecOrchestrator::new(executors);
    block_on("or-tools-exec", operation, orchestrator.execute(request)).and_then(json_value)
}

fn invoke_file(operation: &str, payload: Value) -> Result<Value, BridgeError> {
    match operation {
        "read" | "write" | "list" | "delete" => {
            let provider_name = payload
                .get("provider")
                .and_then(Value::as_str)
                .unwrap_or("local");
            let store = build_file_store(provider_name, payload.get("config"))?;
            let orchestrator = FileOrchestrator::new(store.clone());
            match operation {
                "read" => {
                    let path = required_str(&payload, "path", "or-tools-file", operation)?;
                    block_on("or-tools-file", operation, orchestrator.read(path)).and_then(json_value)
                }
                "write" => {
                    let path = required_str(&payload, "path", "or-tools-file", operation)?;
                    let content = required_str(&payload, "content", "or-tools-file", operation)?;
                    block_on("or-tools-file", operation, store.write(path, content))?;
                    Ok(json!({ "status": "ok" }))
                }
                "list" => {
                    let path = required_str(&payload, "path", "or-tools-file", operation)?;
                    block_on("or-tools-file", operation, store.list(path)).and_then(json_value)
                }
                "delete" => {
                    let path = required_str(&payload, "path", "or-tools-file", operation)?;
                    block_on("or-tools-file", operation, store.delete(path))?;
                    Ok(json!({ "status": "ok" }))
                }
                _ => unreachable!(),
            }
        }
        "fetch" => {
            let provider_name = required_str(&payload, "provider", "or-tools-file", operation)?;
            let query = payload.get("query").cloned().unwrap_or(Value::Null);
            let source = build_data_source(provider_name, payload.get("config"))?;
            block_on("or-tools-file", operation, source.fetch(query))
        }
        _ => Err(unsupported("or-tools-file", operation)),
    }
}

fn invoke_comms(operation: &str, payload: Value) -> Result<Value, BridgeError> {
    if operation != "send" {
        return Err(unsupported("or-tools-comms", operation));
    }
    let provider_name = required_str(&payload, "provider", "or-tools-comms", operation)?;
    let message = build_message(provider_name, &payload)?;
    let sender = build_message_sender(provider_name, payload.get("config"))?;
    let orchestrator = CommsOrchestrator::new(vec![sender]);
    block_on("or-tools-comms", operation, orchestrator.send(message)).and_then(json_value)
}

fn invoke_productivity(operation: &str, payload: Value) -> Result<Value, BridgeError> {
    let provider_name = required_str(&payload, "provider", "or-tools-productivity", operation)?;
    match operation {
        "list_emails" => {
            let client = build_email_client(provider_name, payload.get("config"))?;
            let query = payload.get("query").cloned().unwrap_or(Value::Null);
            block_on("or-tools-productivity", operation, client.list(query)).and_then(json_value)
        }
        "send_email" => {
            let client = build_email_client(provider_name, payload.get("config"))?;
            let email: Email = from_field(&payload, "item", "or-tools-productivity", operation)?;
            let id = block_on("or-tools-productivity", operation, client.send_email(email))?;
            Ok(json!({ "id": id }))
        }
        "list_events" => {
            let client = build_calendar_client(provider_name, payload.get("config"))?;
            let query = payload.get("query").cloned().unwrap_or(Value::Null);
            block_on("or-tools-productivity", operation, client.list_events(query)).and_then(json_value)
        }
        "create_event" => {
            let client = build_calendar_client(provider_name, payload.get("config"))?;
            let event: CalendarEvent = from_field(&payload, "item", "or-tools-productivity", operation)?;
            let id = block_on("or-tools-productivity", operation, client.create_event(event))?;
            Ok(json!({ "id": id }))
        }
        "list_issues" => {
            let tracker = build_project_tracker(provider_name, payload.get("config"))?;
            let query = payload.get("query").cloned().unwrap_or(Value::Null);
            block_on("or-tools-productivity", operation, tracker.list_issues(query)).and_then(json_value)
        }
        "create_issue" => {
            let tracker = build_project_tracker(provider_name, payload.get("config"))?;
            let issue: Issue = from_field(&payload, "item", "or-tools-productivity", operation)?;
            let id = block_on("or-tools-productivity", operation, tracker.create_issue(issue))?;
            Ok(json!({ "id": id }))
        }
        "search_pages" => {
            let kb = build_knowledge_base(provider_name, payload.get("config"))?;
            let query = payload.get("query").cloned().unwrap_or(Value::Null);
            block_on("or-tools-productivity", operation, kb.search(query)).and_then(json_value)
        }
        "create_page" => {
            let kb = build_knowledge_base(provider_name, payload.get("config"))?;
            let page: Page = from_field(&payload, "item", "or-tools-productivity", operation)?;
            let id = block_on("or-tools-productivity", operation, kb.create_page(page))?;
            Ok(json!({ "id": id }))
        }
        "post_message" => {
            let messenger = build_team_messenger(provider_name, payload.get("config"))?;
            let channel = required_str(&payload, "channel", "or-tools-productivity", operation)?;
            let text = required_str(&payload, "text", "or-tools-productivity", operation)?;
            let id = block_on("or-tools-productivity", operation, messenger.post(channel, text))?;
            Ok(json!({ "id": id }))
        }
        "search_messages" => {
            let messenger = build_team_messenger(provider_name, payload.get("config"))?;
            let query = payload.get("query").cloned().unwrap_or(Value::Null);
            block_on("or-tools-productivity", operation, messenger.search_messages(query))
                .and_then(json_value)
        }
        _ => Err(unsupported("or-tools-productivity", operation)),
    }
}

fn build_search_provider(provider: &str, config: Option<&Value>) -> Result<Arc<dyn SearchProvider>, BridgeError> {
    let cfg = config.and_then(Value::as_object);
    let client = reqwest::Client::new();
    let provider = match provider {
        "tavily" => Arc::new(
            if let (Some(endpoint), Some(api_key)) = (cfg.and_then(|v| get_str(v, "endpoint")), cfg.and_then(|v| get_str(v, "api_key"))) {
                TavilySearch::with_endpoint(client, endpoint, api_key)
            } else {
                TavilySearch::from_env().map_err(|error| invocation("or-tools-search", "search", error))?
            },
        ) as Arc<dyn SearchProvider>,
        "exa" => Arc::new(
            if let (Some(endpoint), Some(api_key)) = (cfg.and_then(|v| get_str(v, "endpoint")), cfg.and_then(|v| get_str(v, "api_key"))) {
                ExaSearch::with_endpoint(client, endpoint, api_key)
            } else {
                ExaSearch::from_env().map_err(|error| invocation("or-tools-search", "search", error))?
            },
        ),
        "brave" => Arc::new(
            if let (Some(endpoint), Some(api_key)) = (cfg.and_then(|v| get_str(v, "endpoint")), cfg.and_then(|v| get_str(v, "api_key"))) {
                BraveSearch::with_endpoint(client, endpoint, api_key)
            } else {
                BraveSearch::from_env().map_err(|error| invocation("or-tools-search", "search", error))?
            },
        ),
        "serper" => Arc::new(
            if let (Some(endpoint), Some(api_key)) = (cfg.and_then(|v| get_str(v, "endpoint")), cfg.and_then(|v| get_str(v, "api_key"))) {
                SerperSearch::with_endpoint(client, endpoint, api_key)
            } else {
                SerperSearch::from_env().map_err(|error| invocation("or-tools-search", "search", error))?
            },
        ),
        "searxng" => Arc::new(
            if let Some(endpoint) = cfg.and_then(|v| get_str(v, "endpoint")) {
                SearxngSearch::with_endpoint(client, endpoint)
            } else {
                SearxngSearch::from_env().map_err(|error| invocation("or-tools-search", "search", error))?
            },
        ),
        "youcom" => Arc::new(
            if let (Some(endpoint), Some(api_key)) = (cfg.and_then(|v| get_str(v, "endpoint")), cfg.and_then(|v| get_str(v, "api_key"))) {
                YouComSearch::with_endpoint(client, endpoint, api_key)
            } else {
                YouComSearch::from_env().map_err(|error| invocation("or-tools-search", "search", error))?
            },
        ),
        "bing" => Arc::new(
            if let (Some(endpoint), Some(api_key)) = (cfg.and_then(|v| get_str(v, "endpoint")), cfg.and_then(|v| get_str(v, "api_key"))) {
                BingSearch::with_endpoint(client, endpoint, api_key)
            } else {
                BingSearch::from_env().map_err(|error| invocation("or-tools-search", "search", error))?
            },
        ),
        other => return Err(unsupported_provider("or-tools-search", other)),
    };
    Ok(provider)
}

fn build_web_browser(provider: &str, config: Option<&Value>) -> Result<Arc<dyn WebBrowser>, BridgeError> {
    let cfg = config.and_then(Value::as_object);
    let client = reqwest::Client::new();
    let browser = match provider {
        "requests" => Arc::new(RequestsClient::new()) as Arc<dyn WebBrowser>,
        "playwright" => Arc::new(
            if let Some(endpoint) = cfg.and_then(|v| get_str(v, "endpoint")) {
                PlaywrightBrowser::with_endpoint(client, endpoint)
            } else {
                PlaywrightBrowser::from_env().map_err(|error| invocation("or-tools-web", "fetch", error))?
            },
        ),
        "brightdata" => Arc::new(
            if let (Some(endpoint), Some(token)) = (cfg.and_then(|v| get_str(v, "endpoint")), cfg.and_then(|v| get_str(v, "token"))) {
                BrightDataScraper::with_endpoint(
                    client,
                    endpoint,
                    token,
                    cfg.and_then(|v| get_str(v, "zone")).unwrap_or("web_unlocker"),
                )
            } else {
                BrightDataScraper::from_env().map_err(|error| invocation("or-tools-web", "fetch", error))?
            },
        ),
        "hyperbrowser" => Arc::new(
            if let (Some(endpoint), Some(api_key)) = (cfg.and_then(|v| get_str(v, "endpoint")), cfg.and_then(|v| get_str(v, "api_key"))) {
                HyperbrowserClient::with_endpoint(client, endpoint, api_key)
            } else {
                HyperbrowserClient::from_env().map_err(|error| invocation("or-tools-web", "fetch", error))?
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
                OxylabsScraper::from_env().map_err(|error| invocation("or-tools-web", "fetch", error))?
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

fn build_vector_client(provider: &str, config: Option<&Value>) -> Result<Box<dyn VectorStoreClient>, BridgeError> {
    let cfg = config.and_then(Value::as_object);
    let client = reqwest::Client::new();
    let store: Box<dyn VectorStoreClient> = match provider {
        "pinecone" => Box::new(
            if let (Some(host), Some(api_key)) = (cfg.and_then(|v| get_str(v, "host")), cfg.and_then(|v| get_str(v, "api_key"))) {
                PineconeClient::with_config(client, host, api_key)
            } else {
                PineconeClient::from_env().map_err(|error| invocation("or-tools-vector", "invoke", error))?
            },
        ),
        "weaviate" => Box::new(
            if let Some(base_url) = cfg.and_then(|v| get_str(v, "base_url")) {
                WeaviateClient::with_config(client, base_url, cfg.and_then(|v| get_str(v, "api_key")).map(str::to_owned))
            } else {
                WeaviateClient::from_env().map_err(|error| invocation("or-tools-vector", "invoke", error))?
            },
        ),
        "qdrant" => Box::new(
            if let Some(base_url) = cfg.and_then(|v| get_str(v, "base_url")) {
                QdrantClient::with_config(client, base_url, cfg.and_then(|v| get_str(v, "api_key")).map(str::to_owned))
            } else {
                QdrantClient::from_env().map_err(|error| invocation("or-tools-vector", "invoke", error))?
            },
        ),
        "chroma" => Box::new(
            if let Some(base_url) = cfg.and_then(|v| get_str(v, "base_url")) {
                ChromaClient::with_config(client, base_url)
            } else {
                ChromaClient::from_env()
            },
        ),
        "milvus" => Box::new(
            if let Some(base_url) = cfg.and_then(|v| get_str(v, "base_url")) {
                MilvusClient::with_config(client, base_url, cfg.and_then(|v| get_str(v, "token")).map(str::to_owned))
            } else {
                MilvusClient::from_env().map_err(|error| invocation("or-tools-vector", "invoke", error))?
            },
        ),
        "pgvector" => {
            if config.is_some() {
                return Err(BridgeError::InvalidInput(
                    "pgvector currently uses environment-based connection setup only".into(),
                ));
            }
            Box::new(block_on("or-tools-vector", "connect", PgVectorClient::from_env())?)
        }
        other => return Err(unsupported_provider("or-tools-vector", other)),
    };
    Ok(store)
}

fn build_exec_executor(provider: &str, config: Option<&Value>) -> Result<Arc<dyn CodeExecutor>, BridgeError> {
    let cfg = config.and_then(Value::as_object);
    let client = reqwest::Client::new();
    let executor: Arc<dyn CodeExecutor> = match provider {
        "python" => Arc::new(PythonExecutor),
        "shell" => Arc::new(ShellExecutor),
        "e2b" => Arc::new(
            if let (Some(endpoint), Some(api_key)) = (cfg.and_then(|v| get_str(v, "endpoint")), cfg.and_then(|v| get_str(v, "api_key"))) {
                E2BExecutor::with_config(client, endpoint, api_key)
            } else {
                E2BExecutor::from_env().map_err(|error| invocation("or-tools-exec", "execute", error))?
            },
        ),
        "bearly" => Arc::new(
            if let (Some(endpoint), Some(api_key)) = (cfg.and_then(|v| get_str(v, "endpoint")), cfg.and_then(|v| get_str(v, "api_key"))) {
                BearlyExecutor::with_config(client, endpoint, api_key)
            } else {
                BearlyExecutor::from_env().map_err(|error| invocation("or-tools-exec", "execute", error))?
            },
        ),
        "daytona" => Arc::new(
            if cfg.is_some() {
                return Err(BridgeError::InvalidInput(
                    "daytona currently uses environment-based connection setup only".into(),
                ));
            } else {
                DaytonaExecutor::from_env().map_err(|error| invocation("or-tools-exec", "execute", error))?
            },
        ),
        other => return Err(unsupported_provider("or-tools-exec", other)),
    };
    Ok(executor)
}

fn build_file_store(provider: &str, config: Option<&Value>) -> Result<Arc<dyn FileStore>, BridgeError> {
    let cfg = config.and_then(Value::as_object);
    let client = reqwest::Client::new();
    let store: Arc<dyn FileStore> = match provider {
        "local" => Arc::new(LocalFileSystem),
        "gdrive" => Arc::new(
            if let Some(access_token) = cfg.and_then(|v| get_str(v, "access_token")) {
                GoogleDriveStore::with_token(client, access_token)
            } else {
                GoogleDriveStore::from_env().map_err(|error| invocation("or-tools-file", "store", error))?
            },
        ),
        other => return Err(unsupported_provider("or-tools-file", other)),
    };
    Ok(store)
}

fn build_data_source(provider: &str, config: Option<&Value>) -> Result<Arc<dyn DataSource>, BridgeError> {
    let cfg = config.and_then(Value::as_object);
    let client = reqwest::Client::new();
    let source: Arc<dyn DataSource> = match provider {
        "json" => Arc::new(JsonToolkit),
        "arxiv" => Arc::new(
            if let Some(endpoint) = cfg.and_then(|v| get_str(v, "endpoint")) {
                ArxivSource::with_endpoint(client, endpoint)
            } else {
                ArxivSource::new()
            },
        ),
        "financial" => Arc::new(
            if let (Some(endpoint), Some(api_key)) = (cfg.and_then(|v| get_str(v, "endpoint")), cfg.and_then(|v| get_str(v, "api_key"))) {
                FinancialDatasetsSource::with_config(client, endpoint, api_key)
            } else {
                FinancialDatasetsSource::from_env().map_err(|error| invocation("or-tools-file", "fetch", error))?
            },
        ),
        other => return Err(unsupported_provider("or-tools-file", other)),
    };
    Ok(source)
}

fn build_message(provider: &str, payload: &Value) -> Result<Message, BridgeError> {
    Ok(Message {
        channel: match provider {
            "sms" | "twilio" => Channel::Sms,
            "telegram" => Channel::Telegram,
            "discord" => Channel::Discord,
            "whatsapp" => Channel::WhatsApp,
            "facebook" => Channel::Facebook,
            "messenger" => Channel::Messenger,
            other => return Err(unsupported_provider("or-tools-comms", other)),
        },
        to: required_str(payload, "to", "or-tools-comms", "send")?.to_owned(),
        body: required_str(payload, "body", "or-tools-comms", "send")?.to_owned(),
        from: payload.get("from").and_then(Value::as_str).map(str::to_owned),
    })
}

fn build_message_sender(provider: &str, config: Option<&Value>) -> Result<Arc<dyn MessageSender>, BridgeError> {
    let cfg = config.and_then(Value::as_object);
    let client = reqwest::Client::new();
    let sender: Arc<dyn MessageSender> = match provider {
        "sms" | "twilio" => Arc::new(
            if let (Some(account_sid), Some(auth_token), Some(from)) = (
                cfg.and_then(|v| get_str(v, "account_sid")),
                cfg.and_then(|v| get_str(v, "auth_token")),
                cfg.and_then(|v| get_str(v, "from")),
            ) {
                TwilioSender::with_config(client, account_sid, auth_token, from)
            } else {
                TwilioSender::from_env().map_err(|error| invocation("or-tools-comms", "send", error))?
            },
        ),
        "telegram" => Arc::new(
            if let (Some(base_url), Some(bot_token)) = (
                cfg.and_then(|v| get_str(v, "base_url")),
                cfg.and_then(|v| get_str(v, "bot_token")),
            ) {
                TelegramSender::with_config(client, base_url, bot_token)
            } else {
                TelegramSender::from_env().map_err(|error| invocation("or-tools-comms", "send", error))?
            },
        ),
        "discord" => Arc::new(
            if let (Some(base_url), Some(bot_token)) = (
                cfg.and_then(|v| get_str(v, "base_url")),
                cfg.and_then(|v| get_str(v, "bot_token")),
            ) {
                DiscordSender::with_config(client, base_url, bot_token)
            } else {
                DiscordSender::from_env().map_err(|error| invocation("or-tools-comms", "send", error))?
            },
        ),
        "whatsapp" => Arc::new(
            if let (Some(account_sid), Some(auth_token), Some(from)) = (
                cfg.and_then(|v| get_str(v, "account_sid")),
                cfg.and_then(|v| get_str(v, "auth_token")),
                cfg.and_then(|v| get_str(v, "from")),
            ) {
                WhatsAppSender::with_config(client, account_sid, auth_token, from)
            } else {
                WhatsAppSender::from_env().map_err(|error| invocation("or-tools-comms", "send", error))?
            },
        ),
        "facebook" => Arc::new(
            if let (Some(base_url), Some(page_access_token)) = (
                cfg.and_then(|v| get_str(v, "base_url")),
                cfg.and_then(|v| get_str(v, "page_token").or_else(|| get_str(v, "page_access_token"))),
            ) {
                FacebookSender::with_config(client, base_url, page_access_token)
            } else {
                FacebookSender::from_env().map_err(|error| invocation("or-tools-comms", "send", error))?
            },
        ),
        "messenger" => Arc::new(
            if let (Some(base_url), Some(page_access_token)) = (
                cfg.and_then(|v| get_str(v, "base_url")),
                cfg.and_then(|v| get_str(v, "page_token").or_else(|| get_str(v, "page_access_token"))),
            ) {
                MessengerSender::with_config(client, base_url, page_access_token)
            } else {
                MessengerSender::from_env().map_err(|error| invocation("or-tools-comms", "send", error))?
            },
        ),
        other => return Err(unsupported_provider("or-tools-comms", other)),
    };
    Ok(sender)
}

fn build_email_client(provider: &str, config: Option<&Value>) -> Result<Box<dyn EmailClient>, BridgeError> {
    let cfg = config.and_then(Value::as_object);
    let client = reqwest::Client::new();
    let email: Box<dyn EmailClient> = match provider {
        "gmail" => Box::new(
            if let (Some(base_url), Some(access_token)) = (
                cfg.and_then(|v| get_str(v, "base_url")),
                cfg.and_then(|v| get_str(v, "access_token")),
            ) {
                GmailClient::with_config(client, base_url, access_token)
            } else {
                GmailClient::from_env().map_err(|error| invocation("or-tools-productivity", "email", error))?
            },
        ),
        "office365" => Box::new(
            if let (Some(base_url), Some(access_token)) = (
                cfg.and_then(|v| get_str(v, "base_url")),
                cfg.and_then(|v| get_str(v, "access_token")),
            ) {
                OutlookEmailClient::with_config(client, base_url, access_token)
            } else {
                OutlookEmailClient::from_env().map_err(|error| invocation("or-tools-productivity", "email", error))?
            },
        ),
        other => return Err(unsupported_provider("or-tools-productivity", other)),
    };
    Ok(email)
}

fn build_calendar_client(provider: &str, config: Option<&Value>) -> Result<Box<dyn CalendarClient>, BridgeError> {
    let cfg = config.and_then(Value::as_object);
    let client = reqwest::Client::new();
    let calendar: Box<dyn CalendarClient> = match provider {
        "gcalendar" => Box::new(
            if let (Some(base_url), Some(access_token)) = (
                cfg.and_then(|v| get_str(v, "base_url")),
                cfg.and_then(|v| get_str(v, "access_token")),
            ) {
                GoogleCalendarClient::with_config(client, base_url, access_token)
            } else {
                GoogleCalendarClient::from_env().map_err(|error| invocation("or-tools-productivity", "calendar", error))?
            },
        ),
        "office365" => Box::new(
            if let (Some(base_url), Some(access_token)) = (
                cfg.and_then(|v| get_str(v, "base_url")),
                cfg.and_then(|v| get_str(v, "access_token")),
            ) {
                OutlookCalendarClient::with_config(client, base_url, access_token)
            } else {
                OutlookCalendarClient::from_env().map_err(|error| invocation("or-tools-productivity", "calendar", error))?
            },
        ),
        other => return Err(unsupported_provider("or-tools-productivity", other)),
    };
    Ok(calendar)
}

fn build_project_tracker(provider: &str, config: Option<&Value>) -> Result<Box<dyn ProjectTracker>, BridgeError> {
    let cfg = config.and_then(Value::as_object);
    let client = reqwest::Client::new();
    let tracker: Box<dyn ProjectTracker> = match provider {
        "jira" => Box::new(
            if let (Some(base_url), Some(auth_header)) = (
                cfg.and_then(|v| get_str(v, "base_url")),
                cfg.and_then(|v| get_str(v, "auth_header")),
            ) {
                JiraTracker::with_config(client, base_url, auth_header)
            } else {
                JiraTracker::from_env().map_err(|error| invocation("or-tools-productivity", "tracker", error))?
            },
        ),
        "github" => Box::new(
            if let (Some(base_url), Some(token), Some(owner), Some(repo)) = (
                cfg.and_then(|v| get_str(v, "base_url")),
                cfg.and_then(|v| get_str(v, "token")),
                cfg.and_then(|v| get_str(v, "owner")),
                cfg.and_then(|v| get_str(v, "repo")),
            ) {
                GitHubTracker::with_config(client, base_url, token, owner, repo)
            } else {
                GitHubTracker::from_env().map_err(|error| invocation("or-tools-productivity", "tracker", error))?
            },
        ),
        "trello" => Box::new(
            if let (Some(base_url), Some(api_key), Some(token), Some(list_id)) = (
                cfg.and_then(|v| get_str(v, "base_url")),
                cfg.and_then(|v| get_str(v, "api_key")),
                cfg.and_then(|v| get_str(v, "token")),
                cfg.and_then(|v| get_str(v, "list_id")),
            ) {
                TrelloTracker::with_config(client, base_url, api_key, token, list_id)
            } else {
                TrelloTracker::from_env().map_err(|error| invocation("or-tools-productivity", "tracker", error))?
            },
        ),
        "clickup" => Box::new(
            if let (Some(base_url), Some(api_key), Some(list_id)) = (
                cfg.and_then(|v| get_str(v, "base_url")),
                cfg.and_then(|v| get_str(v, "api_key")),
                cfg.and_then(|v| get_str(v, "list_id")),
            ) {
                ClickUpTracker::with_config(client, base_url, api_key, list_id)
            } else {
                ClickUpTracker::from_env().map_err(|error| invocation("or-tools-productivity", "tracker", error))?
            },
        ),
        other => return Err(unsupported_provider("or-tools-productivity", other)),
    };
    Ok(tracker)
}

fn build_knowledge_base(provider: &str, config: Option<&Value>) -> Result<Box<dyn KnowledgeBase>, BridgeError> {
    let cfg = config.and_then(Value::as_object);
    let client = reqwest::Client::new();
    let knowledge: Box<dyn KnowledgeBase> = match provider {
        "notion" => Box::new(
            if let (Some(base_url), Some(api_key), Some(database_id)) = (
                cfg.and_then(|v| get_str(v, "base_url")),
                cfg.and_then(|v| get_str(v, "api_key")),
                cfg.and_then(|v| get_str(v, "database_id")),
            ) {
                NotionBase::with_config(client, base_url, api_key, database_id)
            } else {
                NotionBase::from_env().map_err(|error| invocation("or-tools-productivity", "knowledge", error))?
            },
        ),
        other => return Err(unsupported_provider("or-tools-productivity", other)),
    };
    Ok(knowledge)
}

fn build_team_messenger(provider: &str, config: Option<&Value>) -> Result<Box<dyn TeamMessenger>, BridgeError> {
    let cfg = config.and_then(Value::as_object);
    let client = reqwest::Client::new();
    let messenger: Box<dyn TeamMessenger> = match provider {
        "slack" => Box::new(
            if let (Some(base_url), Some(bot_token)) = (
                cfg.and_then(|v| get_str(v, "base_url")),
                cfg.and_then(|v| get_str(v, "bot_token")),
            ) {
                SlackMessenger::with_config(client, base_url, bot_token)
            } else {
                SlackMessenger::from_env().map_err(|error| invocation("or-tools-productivity", "messenger", error))?
            },
        ),
        other => return Err(unsupported_provider("or-tools-productivity", other)),
    };
    Ok(messenger)
}

fn from_field<T>(
    payload: &Value,
    key: &str,
    crate_name: &str,
    operation: &str,
) -> Result<T, BridgeError>
where
    T: serde::de::DeserializeOwned,
{
    serde_json::from_value(
        payload
            .get(key)
            .cloned()
            .ok_or_else(|| BridgeError::InvalidInput(format!("missing `{key}` for `{crate_name}` / `{operation}`")))?,
    )
    .map_err(|error| BridgeError::InvalidInput(error.to_string()))
}

fn required_str<'a>(
    payload: &'a Value,
    key: &str,
    crate_name: &str,
    operation: &str,
) -> Result<&'a str, BridgeError> {
    payload
        .get(key)
        .and_then(Value::as_str)
        .ok_or_else(|| {
            BridgeError::InvalidInput(format!(
                "missing string `{key}` for `{crate_name}` / `{operation}`"
            ))
        })
}

fn required_u64(
    payload: &Value,
    key: &str,
    crate_name: &str,
    operation: &str,
) -> Result<u64, BridgeError> {
    payload
        .get(key)
        .and_then(Value::as_u64)
        .ok_or_else(|| {
            BridgeError::InvalidInput(format!(
                "missing integer `{key}` for `{crate_name}` / `{operation}`"
            ))
        })
}

fn get_str<'a>(payload: &'a serde_json::Map<String, Value>, key: &str) -> Option<&'a str> {
    payload.get(key).and_then(Value::as_str)
}

fn json_value<T: Serialize>(value: T) -> Result<Value, BridgeError> {
    serde_json::to_value(value).map_err(|error| BridgeError::InvalidJson(error.to_string()))
}

fn invocation(
    crate_name: &str,
    operation: &str,
    error: impl std::fmt::Display,
) -> BridgeError {
    BridgeError::Invocation {
        crate_name: crate_name.to_owned(),
        operation: operation.to_owned(),
        reason: error.to_string(),
    }
}

fn unsupported(crate_name: &str, operation: &str) -> BridgeError {
    BridgeError::UnsupportedOperation {
        crate_name: crate_name.to_owned(),
        operation: operation.to_owned(),
    }
}

fn unsupported_provider(crate_name: &str, provider: &str) -> BridgeError {
    BridgeError::InvalidInput(format!(
        "unsupported provider `{provider}` for `{crate_name}`"
    ))
}

fn runtime() -> &'static tokio::runtime::Runtime {
    static RUNTIME: OnceLock<tokio::runtime::Runtime> = OnceLock::new();
    RUNTIME.get_or_init(|| {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("or-bridge runtime")
    })
}

fn block_on<T, E>(
    crate_name: &str,
    operation: &str,
    future: impl Future<Output = Result<T, E>>,
) -> Result<T, BridgeError>
where
    E: std::fmt::Display,
{
    match tokio::runtime::Handle::try_current() {
        Ok(handle) => tokio::task::block_in_place(|| handle.block_on(future)),
        Err(_) => runtime().block_on(future),
    }
    .map_err(|error| invocation(crate_name, operation, error))
}
