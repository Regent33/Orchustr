mod retrieval;

use or_anchor::{AnchorOrchestrator, AnchorPipeline};

#[tokio::test]
async fn index_and_retrieve_relevant_chunks() {
    let pipeline = AnchorPipeline::new().with_chunk_size(3);
    AnchorOrchestrator
        .index_document(
            &pipeline,
            "doc-1",
            "orchustr builds reliable agents with memory and retrieval",
        )
        .await
        .unwrap();
    let results = AnchorOrchestrator
        .retrieve(&pipeline, "memory retrieval", 2)
        .await
        .unwrap();
    assert!(!results.is_empty());
    assert!(results[0].text.contains("memory") || results[0].text.contains("retrieval"));
}

#[tokio::test]
async fn retrieve_returns_empty_when_no_documents_are_indexed() {
    let pipeline = AnchorPipeline::new();
    let results = AnchorOrchestrator
        .retrieve(&pipeline, "nothing here", 3)
        .await
        .unwrap();
    assert!(results.is_empty());
}
