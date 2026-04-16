//! Tests for compass route name trimming, duplicate detection, and edge cases.

use or_compass::{CompassError, CompassOrchestrator, CompassRouterBuilder};
use or_core::OrchState;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct TestState {
    value: u8,
}

impl OrchState for TestState {}

#[test]
fn route_name_whitespace_is_trimmed() {
    let router = CompassRouterBuilder::new()
        .add_route("  alpha  ", |_: &TestState| true)
        .build()
        .unwrap();
    let result = CompassOrchestrator
        .select_route(&router, &TestState { value: 1 })
        .unwrap();
    assert_eq!(result.route, "alpha", "route name should be trimmed");
}

#[test]
fn blank_route_name_rejected() {
    let result = CompassRouterBuilder::new()
        .add_route("   ", |_: &TestState| true)
        .build();
    assert!(matches!(result, Err(CompassError::BlankRouteName)));
}

#[test]
fn duplicate_route_names_rejected() {
    let result = CompassRouterBuilder::new()
        .add_route("same", |_: &TestState| true)
        .add_route("same", |_: &TestState| false)
        .build();
    assert!(matches!(
        result,
        Err(CompassError::DuplicateRoute(ref name)) if name == "same"
    ));
}

#[test]
fn empty_router_rejected() {
    let result = CompassRouterBuilder::<TestState>::new().build();
    assert!(matches!(result, Err(CompassError::EmptyRouter)));
}

#[test]
fn trimmed_duplicates_are_caught() {
    let result = CompassRouterBuilder::new()
        .add_route("alpha", |_: &TestState| true)
        .add_route(" alpha ", |_: &TestState| false)
        .build();
    assert!(
        matches!(result, Err(CompassError::DuplicateRoute(ref name)) if name == "alpha"),
        "whitespace-padded duplicate route should be caught"
    );
}
