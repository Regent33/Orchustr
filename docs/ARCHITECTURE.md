# Architecture

Orchustr is structured as a layered Cargo workspace rather than a monolithic crate. `or-core` anchors shared contracts, execution crates build upward from those contracts, and integration crates such as `or-conduit`, `or-forge`, and `or-mcp` connect the runtime to external systems.

The repository now exposes three maintained binding surfaces: Python, TypeScript, and Dart. All three sit above `or-bridge`, but only a narrow helper surface is native today, so each language package still carries part of its own runtime behavior.

## Read This Next

- [Architecture Overview](./architecture/overview.md)
- [Crate Dependency Graph](./architecture/crate-dependency-graph.md)
- [Data Flow](./architecture/data-flow.md)
- [Execution Model](./architecture/execution-model.md)
- [Binding Architecture](./architecture/binding-architecture.md)
- [Design Decisions](./architecture/design-decisions.md)
- [Multi-Language Strategy](./architecture/multi-language-strategy.md)

## High-Level Layers

- **Foundation**: `or-core`
- **Execution**: `or-pipeline`, `or-relay`, `or-loom`, `or-compass`
- **Integration**: `or-conduit`, `or-forge`, `or-mcp`, `or-sieve`, `or-recall`, `or-anchor`
- **Agents**: `or-sentinel`, `or-colony`
- **Cross-cutting**: `or-prism`, `or-bridge`

⚠️ Known Gaps & Limitations
- The architecture pages document the current repository state only.
- Some planned capabilities implied by crate names are still partial in implementation.
