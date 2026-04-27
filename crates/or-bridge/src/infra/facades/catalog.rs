//! Static catalog of every crate exposed through the bridge plus the
//! `workspace_catalog()` entry point that serializes it to JSON for
//! cross-language consumers.

use crate::domain::errors::BridgeError;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub(crate) struct CrateBinding {
    pub(crate) crate_name: &'static str,
    pub(crate) binding_mode: &'static str,
    pub(crate) description: &'static str,
    pub(crate) operations: &'static [&'static str],
}

pub(crate) const CRATE_BINDINGS: &[CrateBinding] = &[
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
