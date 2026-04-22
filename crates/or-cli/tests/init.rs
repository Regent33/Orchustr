use or_cli::{InitOptions, ProjectLanguage, ProviderKind, TopologyKind, init_project};
use tempfile::tempdir;

#[test]
fn init_rust_creates_expected_files() {
    let temp = tempdir().unwrap();
    let root = init_project(&InitOptions {
        project_name: "my-agent".to_owned(),
        language: ProjectLanguage::Rust,
        topology: TopologyKind::React,
        provider: ProviderKind::Anthropic,
        target_dir: temp.path().to_path_buf(),
    })
    .unwrap();

    assert!(root.join("orchustr.yaml").exists());
    assert!(root.join(".env.example").exists());
    assert!(root.join("Cargo.toml").exists());
    assert!(root.join("src/main.rs").exists());
    assert!(root.join("src/nodes/mod.rs").exists());
    assert!(root.join("src/nodes/think.rs").exists());
    assert!(root.join("src/nodes/act.rs").exists());
    assert!(root.join("tests/integration_test.rs").exists());
}

#[test]
fn init_python_requirements_contains_orchustr() {
    let temp = tempdir().unwrap();
    let root = init_project(&InitOptions {
        project_name: "py-agent".to_owned(),
        language: ProjectLanguage::Python,
        topology: TopologyKind::React,
        provider: ProviderKind::Anthropic,
        target_dir: temp.path().to_path_buf(),
    })
    .unwrap();

    let requirements = std::fs::read_to_string(root.join("requirements.txt")).unwrap();
    assert!(requirements.contains("orchustr"));
}
