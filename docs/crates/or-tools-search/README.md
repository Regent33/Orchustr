# or-tools-search

**Status**: Implemented | **Version**: `0.1.3` | **Default features**: `(none)` | **Feature flags**: `tavily`, `exa`, `brave`, `serper`, `searxng`, `youcom`, `bing`, `all`

Feature-gated search-provider integrations for Orchustr tools. The crate defines the shared search query and response model, a fallback-oriented `SearchOrchestrator`, and a `SearchProviderTool` adapter that exposes a provider as an `or-tools-core::Tool`.

## In Plain Language

This crate is the web-search layer for Orchustr. Its job is to ask search providers for results, normalize what comes back, and give the rest of the system one consistent search response shape no matter which provider answered.

If you are choosing tools at a product or workflow level, `or-tools-search` answers questions like "find pages about this topic" or "look up current sources." If you are implementing or extending the system, this is where you add a new search provider or define how fallback between providers should work.

## Responsibilities

- Define the common search query, result, response, and error types.
- Provide the `SearchProvider` contract that each backend must implement.
- Orchestrate fallback across registered providers until one returns usable results.
- Expose provider-backed search as a generic Orchustr `Tool`.
- Stop at search results; page fetching, browser automation, and scraping belong to `or-tools-web`.

## Position in the Workspace

```mermaid
graph LR
  CORE[or-tools-core] --> THIS[or-tools-search]
```

## Implementation Status

| Component | Status | Notes |
|---|---|---|
| Domain contracts | Implemented | `SearchProvider`, `SearchQuery`, `SearchResult`, `SearchResponse`, and `SearchError` are present and re-exported. |
| Orchestration | Implemented | `SearchOrchestrator` validates queries and falls back across registered providers until one returns results. |
| Tool adapter | Implemented | `SearchProviderTool<P>` exposes a single provider instance through the generic `Tool` trait. |
| Provider modules | Implemented | `tavily`, `exa`, `brave`, `serper`, `searxng`, `youcom`, and `bing` are feature-gated under `src/infra/`. |
| Unit tests | Implemented | `tests/unit_suite.rs` covers empty queries, missing providers, fallback, tool wrapping, and invalid payloads. |

## Public Surface

- `SearchProvider` (trait): contract implemented by each search backend.
- `SearchQuery` (struct): request model with query text, optional locale hints, safety flag, and result limit.
- `SearchResult` (struct): one normalized search hit.
- `SearchResponse` (struct): normalized provider response with provider name and result list.
- `SearchError` (enum): provider-neutral error model and conversion into `ToolError`.
- `SearchOrchestrator` (struct): fallback search runtime across a list of `SearchProvider`s.
- `SearchProviderTool` (struct): `Tool` adapter for a single provider.

## Feature Flags and Providers

| Feature | Module | Main type | Config from env |
|---|---|---|---|
| `tavily` | `infra/tavily.rs` | `TavilySearch` | `TAVILY_API_KEY` |
| `exa` | `infra/exa.rs` | `ExaSearch` | `EXA_API_KEY` |
| `brave` | `infra/brave.rs` | `BraveSearch` | `BRAVE_SEARCH_API_KEY` |
| `serper` | `infra/serper.rs` | `SerperSearch` | `SERPER_API_KEY` |
| `searxng` | `infra/searxng.rs` | `SearxngSearch` | `SEARXNG_ENDPOINT` |
| `youcom` | `infra/youcom.rs` | `YouComSearch` | `YOUCOM_API_KEY` |
| `bing` | `infra/bing.rs` | `BingSearch` | `BING_SEARCH_API_KEY` |

## Dependencies

- Internal crates: `or-tools-core`
- External crates: async-trait, reqwest, serde, serde_json, thiserror, tokio, tracing, url

## Known Gaps & Limitations

- The crate compiles no provider by default; callers must opt into one or more provider features.
- `SearchOrchestrator` only returns the first non-empty response; it does not merge result sets across providers.
- Unit coverage uses stub providers rather than live network calls.
