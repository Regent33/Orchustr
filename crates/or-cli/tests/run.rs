use async_trait::async_trait;
use or_cli::domain::entities::RunRequest;
use or_cli::{CliError, ProjectRunner, run_project};
use std::sync::{Arc, Mutex};
use tempfile::tempdir;

#[derive(Clone, Default)]
struct RecordingRunner {
    requests: Arc<Mutex<Vec<RunRequest>>>,
}

#[async_trait]
impl ProjectRunner for RecordingRunner {
    async fn run(&self, request: RunRequest) -> Result<(), CliError> {
        self.requests.lock().unwrap().push(request);
        Ok(())
    }
}

#[tokio::test]
async fn run_command_reads_orchustr_yaml() {
    let temp = tempdir().unwrap();
    std::fs::write(
        temp.path().join("orchustr.yaml"),
        "orchustr_version: \"0.1.0\"\nproject:\n  name: run-agent\n  language: python\n  provider: anthropic\ngraph:\n  $ref: ./graph.yaml\nobservability:\n  enabled: true\n  dashboard_port: 7700\nmcp_servers: []\n",
    )
    .unwrap();
    std::fs::write(
        temp.path().join("graph.yaml"),
        "name: ok\nversion: \"0.1.0\"\nentry: think\nexits: [done]\nnodes:\n  - id: think\n    handler: nodes::think\n    metadata: {}\n  - id: done\n    handler: nodes::done\n    metadata: {}\nedges:\n  - from: think\n    to: done\n",
    )
    .unwrap();

    let runner = RecordingRunner::default();
    let summary = run_project(temp.path(), &runner).await.unwrap();

    let requests = runner.requests.lock().unwrap();
    assert_eq!(summary.project_name, "run-agent");
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].config.project.name, "run-agent");
    assert_eq!(requests[0].graph.entry, "think");
}
