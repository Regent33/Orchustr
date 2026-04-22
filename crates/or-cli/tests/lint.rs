use or_cli::lint_path;
use tempfile::tempdir;

#[test]
fn lint_valid_graph_passes() {
    let temp = tempdir().unwrap();
    let graph = temp.path().join("graph.yaml");
    std::fs::write(
        &graph,
        "name: ok\nversion: \"0.1.0\"\nentry: think\nexits: [done]\nnodes:\n  - id: think\n    handler: nodes::think\n    metadata: {}\n  - id: done\n    handler: nodes::done\n    metadata: {}\nedges:\n  - from: think\n    to: done\n",
    )
    .unwrap();

    let result = lint_path(&graph).unwrap();
    assert_eq!(result, vec![graph]);
}

#[test]
fn lint_missing_entry_node_fails() {
    let temp = tempdir().unwrap();
    let graph = temp.path().join("graph.yaml");
    std::fs::write(
        &graph,
        "name: broken\nversion: \"0.1.0\"\nexits: [done]\nnodes:\n  - id: done\n    handler: nodes::done\n    metadata: {}\nedges: []\n",
    )
    .unwrap();

    let error = lint_path(&graph).unwrap_err().to_string();
    assert!(error.contains("entry"));
}
