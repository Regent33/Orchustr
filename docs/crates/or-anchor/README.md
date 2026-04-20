# or-anchor

**Status**: 🟡 Partial | **Version**: `0.1.1` | **Deps**: serde, serde_json, thiserror, tracing

Retrieval-oriented crate that chunks documents, creates simple hashed embeddings, and retrieves relevant chunks from an in-memory vector store.

## Position in the Workspace

```mermaid
graph LR
  OR_CORE[or-core] --> THIS[or-anchor]
  THIS --> CALLERS[Callers]
```

## Implementation Status

| Component | Status | Notes |
|---|---|---|
| Chunking | 🟢 | Documents are split into deterministic fixed-size chunks. |
| Embedding and retrieval | 🟡 | Hashed bag-of-words embeddings are useful locally but are not production-grade semantic vectors. |
| Orchestrator surface | 🟢 | `AnchorOrchestrator` wraps index and retrieve entry points. |

## Public Surface

- `AnchorChunk` (struct): Represents a chunked document segment.
- `RetrievedChunk` (struct): Represents a retrieved chunk with a score.
- `AnchorPipeline` (struct): Concrete indexing and retrieval implementation built on `InMemoryVectorStore`.
- `AnchorOrchestrator` (struct): Application wrapper around indexing and retrieval.
- `AnchorError` (enum): Error type for indexing and retrieval failures.

## Dependencies

- Internal crates: or-core
- External crates: serde, serde_json, thiserror, tracing

⚠️ Known Gaps & Limitations
- Embeddings are simple hashed vectors, not model-backed embeddings.
- No external vector database integration exists in this crate.
