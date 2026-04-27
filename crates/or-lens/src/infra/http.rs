use crate::application::orchestrators::snapshot_from_spans;
use crate::domain::contracts::TraceRepository;
use crate::domain::entities::{ExecutionSnapshot, TraceSummary};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};
use axum::{Json, Router, routing::get};

const DASHBOARD_HTML: &str = include_str!("../../assets/dashboard.html");

#[derive(Clone)]
pub(crate) struct AppState<R: TraceRepository> {
    pub(crate) repository: R,
}

pub(crate) fn router<R: TraceRepository>(repository: R) -> Router {
    Router::new()
        .route("/", get(index))
        .route("/api/traces", get(list_traces::<R>))
        .route("/api/traces/{trace_id}", get(get_trace::<R>))
        .with_state(AppState { repository })
}

async fn index() -> Html<&'static str> {
    Html(DASHBOARD_HTML)
}

async fn list_traces<R: TraceRepository>(
    State(state): State<AppState<R>>,
) -> Json<Vec<TraceSummary>> {
    Json(state.repository.traces())
}

async fn get_trace<R: TraceRepository>(
    Path(trace_id): Path<String>,
    State(state): State<AppState<R>>,
) -> Result<Json<ExecutionSnapshot>, impl IntoResponse> {
    state
        .repository
        .trace(&trace_id)
        .map(|spans| Json(snapshot_from_spans(&trace_id, &spans)))
        .ok_or((StatusCode::NOT_FOUND, "trace not found"))
}
