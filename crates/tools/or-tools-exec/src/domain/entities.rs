use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    Python,
    Shell,
    JavaScript,
    TypeScript,
    Ruby,
}

impl Language {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Python => "python",
            Self::Shell => "sh",
            Self::JavaScript => "javascript",
            Self::TypeScript => "typescript",
            Self::Ruby => "ruby",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExecRequest {
    pub code: String,
    pub language: Language,
    #[serde(default = "default_timeout")]
    pub timeout_ms: u64,
    #[serde(default)]
    pub env: std::collections::HashMap<String, String>,
}

fn default_timeout() -> u64 {
    30_000
}

impl ExecRequest {
    #[must_use]
    pub fn new(code: impl Into<String>, language: Language) -> Self {
        Self {
            code: code.into(),
            language,
            timeout_ms: default_timeout(),
            env: Default::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExecResult {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
    #[serde(default)]
    pub duration_ms: u64,
}

impl ExecResult {
    #[must_use]
    pub fn success(stdout: impl Into<String>) -> Self {
        Self {
            stdout: stdout.into(),
            stderr: String::new(),
            exit_code: 0,
            duration_ms: 0,
        }
    }

    #[must_use]
    pub fn is_success(&self) -> bool {
        self.exit_code == 0
    }
}
