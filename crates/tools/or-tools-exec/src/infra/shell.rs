use crate::domain::contracts::CodeExecutor;
use crate::domain::entities::{ExecRequest, ExecResult, Language};
use crate::domain::errors::ExecError;
use async_trait::async_trait;
use std::time::{Duration, Instant};
use tokio::process::Command;
use tokio::time::timeout;

/// Environment variable callers must set to `1` (or any truthy value)
/// to enable the unsandboxed local-shell executor. Without it,
/// `ShellExecutor::execute` refuses to run and returns
/// [`ExecError::ExecutorNotFound`] with guidance toward sandboxed
/// alternatives (E2B, Daytona, Bearly).
pub const SHELL_OPT_IN_ENV: &str = "ORCHUSTR_ALLOW_UNSANDBOXED_SHELL";

fn shell_opt_in_enabled() -> bool {
    std::env::var(SHELL_OPT_IN_ENV)
        .map(|value| {
            let trimmed = value.trim().to_ascii_lowercase();
            matches!(trimmed.as_str(), "1" | "true" | "yes" | "on")
        })
        .unwrap_or(false)
}

/// Local-shell executor.
///
/// **Security note:** this executor pipes `req.code` directly to the
/// system shell with no sandboxing, command allowlist, or resource
/// limits. It is intended for development and trusted automation only.
/// In any context where `req.code` could be influenced by an LLM or
/// other untrusted input, prefer a sandboxed executor such as
/// `E2BExecutor`, `DaytonaExecutor`, or `BearlyExecutor`.
///
/// To guard against accidental production use, `execute` requires the
/// caller to opt in via the `ORCHUSTR_ALLOW_UNSANDBOXED_SHELL` env var
/// (see [`SHELL_OPT_IN_ENV`]).
pub struct ShellExecutor;

#[async_trait]
impl CodeExecutor for ShellExecutor {
    fn name(&self) -> &'static str {
        "shell"
    }

    fn supports(&self, lang: Language) -> bool {
        matches!(lang, Language::Shell)
    }

    async fn execute(&self, req: ExecRequest) -> Result<ExecResult, ExecError> {
        if !shell_opt_in_enabled() {
            return Err(ExecError::ExecutorNotFound {
                executor: "shell".to_owned(),
                reason: format!(
                    "ShellExecutor is unsandboxed and disabled by default. \
                     Set {SHELL_OPT_IN_ENV}=1 to enable, or use a sandboxed \
                     executor (e2b, daytona, bearly)."
                ),
            });
        }
        let start = Instant::now();
        let shell = if cfg!(target_os = "windows") {
            "cmd"
        } else {
            "sh"
        };
        let flag = if cfg!(target_os = "windows") {
            "/C"
        } else {
            "-c"
        };
        // `kill_on_drop` ensures that if the timeout below fires (or this
        // future is otherwise cancelled), tokio reaps the spawned process
        // instead of leaving an orphan.
        let child = Command::new(shell)
            .args([flag, &req.code])
            .envs(&req.env)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .kill_on_drop(true)
            .spawn()
            .map_err(|e| ExecError::Spawn(e.to_string()))?;

        let output = timeout(
            Duration::from_millis(req.timeout_ms),
            child.wait_with_output(),
        )
        .await
        .map_err(|_| ExecError::Timeout(req.timeout_ms))?
        .map_err(|e| ExecError::Io(e.to_string()))?;

        Ok(ExecResult {
            stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
            exit_code: output.status.code().unwrap_or(-1),
            duration_ms: start.elapsed().as_millis() as u64,
        })
    }
}
