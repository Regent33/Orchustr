use or_schema::GraphSpec;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::{Path, PathBuf};

/// Languages supported by `orchustr init`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProjectLanguage {
    Rust,
    Python,
    Typescript,
    Dart,
}

impl ProjectLanguage {
    /// Returns the default source file extension for the language.
    #[must_use]
    pub fn file_extension(&self) -> &'static str {
        match self {
            Self::Rust => "rs",
            Self::Python => "py",
            Self::Typescript => "ts",
            Self::Dart => "dart",
        }
    }

    /// Returns a simple generated node template for `orchustr new node`.
    #[must_use]
    pub fn node_template(&self, name: &str) -> String {
        match self {
            Self::Rust => format!("pub async fn {name}() {{}}\n"),
            Self::Python => format!("async def {name}(state):\n    return state\n"),
            Self::Typescript => format!("export async function {name}(state: unknown) {{\n  return state;\n}}\n"),
            Self::Dart => format!("Future<dynamic> {name}(dynamic state) async => state;\n"),
        }
    }
}

/// Loop topologies supported by `orchustr init`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TopologyKind {
    React,
    PlanExecute,
    Reflection,
}

/// Provider presets supported by `orchustr init`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderKind {
    Anthropic,
    Openai,
    Ollama,
}

impl ProviderKind {
    /// Returns the lowercase provider label stored in generated config files.
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Anthropic => "anthropic",
            Self::Openai => "openai",
            Self::Ollama => "ollama",
        }
    }

    /// Returns the `.env.example` key for the provider.
    #[must_use]
    pub fn env_key(&self) -> &'static str {
        match self {
            Self::Anthropic => "ANTHROPIC_API_KEY",
            Self::Openai => "OPENAI_API_KEY",
            Self::Ollama => "OLLAMA_BASE_URL",
        }
    }
}

/// User-facing options for generating a new Orchustr project scaffold.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InitOptions {
    pub project_name: String,
    pub language: ProjectLanguage,
    pub topology: TopologyKind,
    pub provider: ProviderKind,
    pub target_dir: PathBuf,
}

impl InitOptions {
    /// Returns the final root directory for the generated project.
    #[must_use]
    pub fn project_root(&self) -> PathBuf {
        self.target_dir.join(&self.project_name)
    }
}

/// Root `orchustr.yaml` project configuration file parsed by `or-cli`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchustrConfig {
    pub orchustr_version: String,
    pub project: ProjectMetadata,
    pub graph: GraphReference,
    #[serde(default)]
    pub observability: ObservabilityConfig,
    #[serde(default)]
    pub mcp_servers: Vec<Value>,
}

impl OrchustrConfig {
    /// Resolves the referenced graph file relative to the project root.
    #[must_use]
    pub fn graph_path(&self, project_dir: &Path) -> PathBuf {
        project_dir.join(&self.graph.path)
    }
}

/// Project metadata nested inside `orchustr.yaml`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectMetadata {
    pub name: String,
    pub language: ProjectLanguage,
    pub provider: String,
}

/// Graph file reference nested inside `orchustr.yaml`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphReference {
    #[serde(rename = "$ref")]
    pub path: String,
}

/// Observability settings nested inside `orchustr.yaml`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservabilityConfig {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default = "default_dashboard_port")]
    pub dashboard_port: u16,
}

impl Default for ObservabilityConfig {
    fn default() -> Self {
        Self {
            enabled: default_enabled(),
            dashboard_port: default_dashboard_port(),
        }
    }
}

/// Parsed project input handed to a `ProjectRunner`.
#[derive(Debug, Clone)]
pub struct RunRequest {
    pub project_dir: PathBuf,
    pub config: OrchustrConfig,
    pub graph: GraphSpec,
}

/// Result metadata returned by `run_project`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunSummary {
    pub project_name: String,
    pub dashboard_port: Option<u16>,
}

fn default_enabled() -> bool {
    true
}

fn default_dashboard_port() -> u16 {
    7700
}
