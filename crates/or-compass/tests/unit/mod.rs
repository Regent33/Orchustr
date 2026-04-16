mod routing;

use or_compass::{CompassError, CompassOrchestrator, CompassRouterBuilder};
use or_core::OrchState;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct RouteState {
    importance: u8,
}

impl OrchState for RouteState {}

#[tokio::test]
async fn builder_rejects_missing_default_routes() {
    let result = CompassRouterBuilder::<RouteState>::new()
        .add_route("low", |_| true)
        .set_default("missing")
        .build();
    assert!(matches!(
        result,
        Err(CompassError::MissingDefaultRoute(name)) if name == "missing"
    ));
}

#[tokio::test]
async fn select_route_returns_first_matching_route() {
    let router = CompassRouterBuilder::new()
        .add_route("high", |state: &RouteState| state.importance > 5)
        .add_route("low", |_| true)
        .build()
        .unwrap();
    let result = CompassOrchestrator
        .select_route(&router, &RouteState { importance: 9 })
        .unwrap();
    assert_eq!(result.route, "high");
}

#[tokio::test]
async fn select_route_uses_default_when_no_route_matches() {
    let router = CompassRouterBuilder::new()
        .add_route("high", |state: &RouteState| state.importance > 5)
        .add_route("fallback", |_| false)
        .set_default("fallback")
        .build()
        .unwrap();
    let result = CompassOrchestrator
        .select_route(&router, &RouteState { importance: 1 })
        .unwrap();
    assert_eq!(result.route, "fallback");
}

#[tokio::test]
async fn select_route_returns_error_without_match_or_default() {
    let router = CompassRouterBuilder::new()
        .add_route("high", |state: &RouteState| state.importance > 5)
        .build()
        .unwrap();
    let result = CompassOrchestrator.select_route(&router, &RouteState { importance: 1 });
    assert_eq!(result, Err(CompassError::NoMatchingRoute));
}
