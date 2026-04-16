mod observability;
mod retry;

use or_core::{
    CoreError, CoreOrchestrator, InMemoryPersistenceBackend, InMemoryVectorStore,
    PersistenceBackend, RetryPolicy, TokenBudget, VectorStore,
};
use serde_json::json;

#[tokio::test]
async fn enforce_completion_budget_accepts_fitting_requests() {
    let orchestrator = CoreOrchestrator::new();
    let budget = TokenBudget {
        max_context_tokens: 1024,
        max_completion_tokens: 256,
    };
    let result = orchestrator.enforce_completion_budget(&budget, 512);
    assert!(result.is_ok());
}

#[tokio::test]
async fn enforce_completion_budget_rejects_overflowing_requests() {
    let orchestrator = CoreOrchestrator::new();
    let budget = TokenBudget {
        max_context_tokens: 512,
        max_completion_tokens: 256,
    };
    let result = orchestrator.enforce_completion_budget(&budget, 400);
    assert_eq!(
        result,
        Err(CoreError::BudgetExceeded {
            requested: 656,
            budget: 512
        })
    );
}

#[tokio::test]
async fn next_retry_delay_returns_duration_for_valid_attempts() {
    let orchestrator = CoreOrchestrator::new();
    let result = orchestrator.next_retry_delay(&RetryPolicy::no_retry(), 1);
    assert!(result.is_ok());
}

#[tokio::test]
async fn next_retry_delay_rejects_invalid_attempts() {
    let orchestrator = CoreOrchestrator::new();
    let result = orchestrator.next_retry_delay(&RetryPolicy::default_llm(), 4);
    assert_eq!(
        result,
        Err(CoreError::InvalidRetryAttempt {
            attempt: 4,
            max_attempts: 3
        })
    );
}

#[tokio::test]
async fn in_memory_persistence_round_trips_state() {
    let backend = InMemoryPersistenceBackend::new();
    backend
        .save_state("graph:entry", json!({"step": 1}))
        .await
        .unwrap();
    let loaded = backend.load_state("graph:entry").await.unwrap();
    assert_eq!(loaded, Some(json!({"step": 1})));
}

#[tokio::test]
async fn in_memory_vector_store_returns_best_match_first() {
    let store = InMemoryVectorStore::new();
    store
        .upsert("a", vec![1.0, 0.0], json!({"kind": "alpha"}))
        .await
        .unwrap();
    store
        .upsert("b", vec![0.0, 1.0], json!({"kind": "beta"}))
        .await
        .unwrap();
    let result = store.query(vec![1.0, 0.0], 1).await.unwrap();
    assert_eq!(result[0].id, "a");
}
