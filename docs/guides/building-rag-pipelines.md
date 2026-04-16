# Building RAG Pipelines

`or-anchor` currently provides the retrieval-oriented pieces that exist in the repository: text chunking, hashed local embeddings, and in-memory vector retrieval.

## Rust Example

```rust
use or_anchor::AnchorPipeline;

# async fn example() -> anyhow::Result<()> {
let pipeline = AnchorPipeline::new();
pipeline.index_document("doc-1", "retrieval augmented generation starts with chunking text").await?;
let chunks = pipeline.retrieve("chunking retrieval", 3).await?;
println!("{}", chunks.len());
# Ok(()) }
```

## What to Expect

- Retrieval is deterministic and local.
- Embeddings are hashed vectors, not model-generated embeddings.
- `or-core`'s in-memory vector store backs the current implementation.

⚠️ Known Gaps & Limitations
- No external vector database or embedding model integration exists in the current code.
