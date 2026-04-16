//! RAG retrieval tests: relevance ranking, chunk boundaries, empty queries.

use or_anchor::{AnchorOrchestrator, AnchorPipeline};

#[tokio::test]
async fn retrieval_ranks_relevant_chunks_higher() {
    let pipeline = AnchorPipeline::new().with_chunk_size(5);
    AnchorOrchestrator
        .index_document(
            &pipeline,
            "doc-rag-1",
            "Rust is a systems programming language focused on safety and performance",
        )
        .await
        .unwrap();
    AnchorOrchestrator
        .index_document(
            &pipeline,
            "doc-rag-2",
            "Python is an interpreted language popular in data science and machine learning",
        )
        .await
        .unwrap();

    let results = AnchorOrchestrator
        .retrieve(&pipeline, "systems programming safety", 2)
        .await
        .unwrap();
    assert!(!results.is_empty());
    // The Rust doc should rank higher for "systems programming safety"
    assert!(
        results[0].text.contains("Rust") || results[0].text.contains("safety"),
        "expected Rust-related chunk first, got: {:?}",
        results[0].text
    );
}

#[tokio::test]
async fn retrieval_respects_top_k_limit() {
    let pipeline = AnchorPipeline::new().with_chunk_size(3);
    for i in 0..10 {
        AnchorOrchestrator
            .index_document(&pipeline, &format!("doc-k-{i}"), &format!("chunk content {i}"))
            .await
            .unwrap();
    }
    let results = AnchorOrchestrator
        .retrieve(&pipeline, "chunk", 3)
        .await
        .unwrap();
    assert!(
        results.len() <= 3,
        "top_k=3 should return at most 3 results, got {}",
        results.len()
    );
}

#[tokio::test]
async fn retrieval_handles_unicode_documents() {
    let pipeline = AnchorPipeline::new().with_chunk_size(5);
    AnchorOrchestrator
        .index_document(&pipeline, "doc-uni", "日本語のテスト文書です。AIエージェントを構築しています。")
        .await
        .unwrap();
    let results = AnchorOrchestrator
        .retrieve(&pipeline, "テスト", 1)
        .await
        .unwrap();
    assert!(!results.is_empty(), "unicode retrieval should return results");
}

#[tokio::test]
async fn multiple_documents_indexed_independently() {
    let pipeline = AnchorPipeline::new().with_chunk_size(5);
    AnchorOrchestrator
        .index_document(&pipeline, "d1", "alpha beta gamma")
        .await
        .unwrap();
    AnchorOrchestrator
        .index_document(&pipeline, "d2", "delta epsilon zeta")
        .await
        .unwrap();
    let results = AnchorOrchestrator
        .retrieve(&pipeline, "alpha", 5)
        .await
        .unwrap();
    assert!(
        results.iter().any(|r| r.text.contains("alpha")),
        "should find alpha in results"
    );
}
