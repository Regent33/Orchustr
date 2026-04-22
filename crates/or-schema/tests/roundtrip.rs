use or_schema::{EdgeSpec, GraphSpec, NodeSpec};
use serde_json::json;

fn sample_spec() -> GraphSpec {
    GraphSpec {
        name: "simple-react-agent".to_owned(),
        version: "0.1.0".to_owned(),
        entry: "think".to_owned(),
        exits: vec!["done".to_owned()],
        nodes: vec![
            NodeSpec {
                id: "think".to_owned(),
                handler: "tests::think".to_owned(),
                metadata: json!({}),
            },
            NodeSpec {
                id: "done".to_owned(),
                handler: "tests::done".to_owned(),
                metadata: json!({"kind": "terminal"}),
            },
        ],
        edges: vec![EdgeSpec {
            from: "think".to_owned(),
            to: "done".to_owned(),
            condition: None,
        }],
    }
}

#[test]
fn graph_spec_roundtrip_json() {
    let spec = sample_spec();
    let encoded = spec.to_json().expect("JSON serialization should succeed");
    let decoded = GraphSpec::from_json(&encoded).expect("JSON deserialization should succeed");
    assert_eq!(decoded, spec);
}

#[cfg(feature = "yaml")]
#[test]
fn graph_spec_roundtrip_yaml() {
    let spec = sample_spec();
    let encoded = spec.to_yaml().expect("YAML serialization should succeed");
    let decoded = GraphSpec::from_yaml(&encoded).expect("YAML deserialization should succeed");
    assert_eq!(decoded, spec);
}
