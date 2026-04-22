# or-schema

**Status**: Complete | **Version**: `0.1.2` | **Deps**: serde, serde_json, serde_yaml(feature=`yaml`)

`or-schema` provides serializable graph descriptors for Orchustr runtimes. It defines `GraphSpec`, `NodeSpec`, and `EdgeSpec` plus JSON/YAML loader helpers that keep descriptor parsing separate from runtime execution.

## Position in the Workspace

```mermaid
graph LR
  THIS[or-schema] --> OR_LOOM[or-loom feature=serde]
```

## Implementation Status

| Component | Status | Notes |
|---|---|---|
| Descriptor types | Complete | `GraphSpec`, `NodeSpec`, and `EdgeSpec` are implemented and tested. |
| JSON helpers | Complete | `from_json` and `to_json` roundtrip descriptor text. |
| YAML helpers | Complete | `from_yaml` and `to_yaml` are available behind feature `yaml`. |
| Runtime coupling | Complete | The crate deliberately does not depend on `or-loom`. |

## Public Surface

- `GraphSpec` (struct): Serializable graph definition with nodes, edges, entry, and exits.
- `NodeSpec` (struct): Serializable node descriptor with a registered handler name.
- `EdgeSpec` (struct): Serializable edge descriptor with an optional condition name.
- `SchemaError` (enum): Error type for JSON/YAML parsing and feature availability.

## Known Gaps & Limitations

- `or-schema` only defines descriptors; it does not execute graphs itself.
- YAML support is optional and requires feature `yaml`.
