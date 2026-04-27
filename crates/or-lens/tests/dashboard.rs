#![cfg(feature = "dashboard")]

use or_lens::server::start_dashboard_server_with_collector;
use or_lens::snapshot::snapshot_from_spans;
use or_lens::{LensSpan, LensSpanStatus, SpanCollector, TraceSummary};
use serde_json::json;
use std::time::Duration;

fn sample_span(name: &str, started_at_ms: u64) -> LensSpan {
    LensSpan {
        trace_id: "trace-1".to_owned(),
        span_id: format!("span-{name}"),
        parent_span_id: None,
        name: name.to_owned(),
        started_at_ms,
        ended_at_ms: Some(started_at_ms + 5),
        status: LensSpanStatus::Completed,
        state_before: json!({ "step": name }),
        state_after: json!({ "step": name, "done": true }),
    }
}

#[test]
fn collector_receives_span_and_stores() {
    let collector = SpanCollector::new();
    collector.record_span(sample_span("think", 10));

    let traces = collector.traces();
    assert_eq!(traces.len(), 1);
    assert_eq!(traces[0].trace_id, "trace-1");

    let stored = collector.trace("trace-1").expect("trace should exist");
    assert_eq!(stored.len(), 1);
    assert_eq!(stored[0].name, "think");
}

#[test]
fn snapshot_from_span_tree_correct_order() {
    let spans = vec![
        sample_span("done", 30),
        sample_span("think", 10),
        sample_span("act", 20),
    ];

    let snapshot = snapshot_from_spans("trace-1", &spans);
    let ordered = snapshot
        .nodes
        .into_iter()
        .map(|node| node.name)
        .collect::<Vec<_>>();

    assert_eq!(ordered, vec!["think", "act", "done"]);
}

#[tokio::test]
async fn server_traces_endpoint_returns_json() {
    let collector = SpanCollector::new();
    collector.record_span(sample_span("think", 10));

    let handle = start_dashboard_server_with_collector(collector, 0)
        .await
        .expect("dashboard server should start");
    tokio::time::sleep(Duration::from_millis(50)).await;

    let url = format!("http://127.0.0.1:{}/api/traces", handle.port());
    let response = reqwest::get(url)
        .await
        .expect("request should succeed")
        .json::<Vec<TraceSummary>>()
        .await
        .expect("response should decode");

    handle.shutdown();

    assert_eq!(response.len(), 1);
    assert_eq!(response[0].trace_id, "trace-1");
}

#[test]
fn collector_caps_spans_per_trace_and_evicts_oldest() {
    // Per-trace cap of 3: any insert beyond 3 should drop the oldest
    // span (lowest `started_at_ms`).
    let collector = SpanCollector::with_capacity(3, 16);
    for ts in [10, 20, 30, 40, 50] {
        collector.record_span(sample_span(&format!("step-{ts}"), ts));
    }
    let stored = collector
        .trace("trace-1")
        .expect("trace should still exist");
    assert_eq!(stored.len(), 3);
    let timestamps = stored
        .iter()
        .map(|span| span.started_at_ms)
        .collect::<Vec<_>>();
    assert_eq!(
        timestamps,
        vec![30, 40, 50],
        "oldest spans (10, 20) should have been evicted"
    );
}

#[test]
fn collector_caps_total_traces_and_evicts_least_recent() {
    // Trace cap of 2: a third trace forces eviction of the
    // least-recently-active trace.
    let collector = SpanCollector::with_capacity(8, 2);

    let mut span_a = sample_span("a", 10);
    span_a.trace_id = "trace-a".to_owned();
    collector.record_span(span_a);

    let mut span_b = sample_span("b", 50);
    span_b.trace_id = "trace-b".to_owned();
    collector.record_span(span_b);

    let mut span_c = sample_span("c", 100);
    span_c.trace_id = "trace-c".to_owned();
    collector.record_span(span_c);

    let traces = collector.traces();
    let ids: Vec<_> = traces.into_iter().map(|t| t.trace_id).collect();
    assert!(
        !ids.contains(&"trace-a".to_owned()),
        "oldest trace 'trace-a' should have been evicted; got {ids:?}"
    );
    assert!(ids.contains(&"trace-b".to_owned()));
    assert!(ids.contains(&"trace-c".to_owned()));
}
