# or-tools-loaders

**Status**: Implemented | **Version**: `0.1.3` | **Default features**: `text`, `markdown`, `json`, `csv`, `html` | **Feature flags**: `text`, `markdown`, `json`, `csv`, `html`, `pdf`, `docx`, `all`

Document loading and normalization tools for Orchustr. The crate turns local paths or raw input into normalized `Document` chunks, routing requests by document kind through registered loaders and exposing the workflow through a generic `Tool`.

## In Plain Language

This crate is the document intake layer. Its job is to take raw content or a file path, understand what kind of document it is, and turn that input into normalized `Document` chunks that the rest of the system can index, store, summarize, or retrieve later.

For non-specialists, this is the crate that "cleans a document up enough for the AI system to use it." For contributors, this is where format-specific parsing and chunking logic live. It is intentionally separate from vector storage and retrieval so file parsing can evolve without changing the RAG backend.

## Responsibilities

- Define the shared loader contract and normalized document entities.
- Route a load request by explicit kind or by file extension.
- Parse supported formats such as text, markdown, JSON, CSV, HTML, and PDF.
- Chunk normalized output into `Document` records that downstream systems can consume.
- Stop at loading and chunking; indexing, embedding, and retrieval are handled by other crates.

## Position in the Workspace

```mermaid
graph LR
  CORE[or-tools-core] --> THIS[or-tools-loaders]
```

## Implementation Status

| Component | Status | Notes |
|---|---|---|
| Domain contracts | Implemented | `DocumentLoader`, `DocumentKind`, `LoaderRequest`, `LoaderSource`, `Document`, and `LoaderError` are present and re-exported. |
| Orchestration | Implemented | `LoaderOrchestrator` routes by explicit kind hint or path extension. |
| Tool adapter | Implemented | `LoaderTool` exposes loading through `Tool`. |
| Loader modules | Implemented | `text`, `markdown`, `json`, `csv`, `html`, and `pdf` modules are present under `src/infra/`. |
| Unit tests | Implemented | `tests/unit_suite.rs` covers text, markdown, JSON validation, chunking, routing, and tool dispatch. |

## Public Surface

- `DocumentLoader` (trait): async contract implemented by each loader backend.
- `DocumentKind` (enum): normalized document-kind taxonomy.
- `LoaderSource` (enum): input source as a path or raw content.
- `LoaderRequest` (struct): loader request with source, optional kind hint, chunk size, and metadata.
- `Document` (struct): normalized document chunk with kind and chunk index.
- `LoaderError` (enum): format, source, IO, and parse failure model.
- `LoaderOrchestrator` (struct): routing layer for registered loaders.

## Feature Flags and Loader Modules

| Feature | Module | Main type | Notes |
|---|---|---|---|
| `text` | `infra/text.rs` | `TextLoader` | Reads text and chunks it directly. |
| `markdown` | `infra/markdown.rs` | `MarkdownLoader` | Strips YAML front matter before chunking. |
| `json` | `infra/json.rs` | `JsonLoader` | Validates JSON and pretty-prints it before chunking. |
| `csv` | `infra/csv_loader.rs` | `CsvLoader` | Converts rows into JSON objects and emits chunked row groups. |
| `html` | `infra/html.rs` | `HtmlLoader` | Strips tags and collapses whitespace into text. |
| `pdf` | `infra/pdf.rs` | `PdfLoader` | Extracts text with `lopdf`; raw PDF input expects base64 bytes. |
| `docx` | `(declared in Cargo.toml)` | `(no module wired)` | The feature flag exists, but `src/infra/mod.rs` does not currently wire a `docx` module. |

## Dependencies

- Internal crates: `or-tools-core`
- External crates: async-trait, serde, serde_json, thiserror, tokio, tracing, base64
- Optional external crates: `csv` behind `csv`, `lopdf` behind `pdf`

## Known Gaps & Limitations

- The `docx` feature flag is declared in `Cargo.toml`, but no `infra::docx` implementation is currently present in the source tree.
- `PdfLoader` expects raw binary input to arrive as base64-encoded content.
- The HTML loader uses a zero-dependency tag stripper rather than a full DOM parser.
