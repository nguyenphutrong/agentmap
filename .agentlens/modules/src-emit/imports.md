# Imports

[← Back to MODULE](MODULE.md) | [← Back to INDEX](../../INDEX.md)

## Dependency Graph

```mermaid
graph TD
    src_emit[src-emit] --> analyze[analyze]
    src_emit[src-emit] --> anyhow[anyhow]
    src_emit[src-emit] --> chrono[chrono]
    src_emit[src-emit] --> scan[scan]
    src_emit[src-emit] --> serde[serde]
    src_emit[src-emit] --> std[std]
    src_emit[src-emit] --> super[super]
    src_emit[src-emit] --> types[types]
```

## Internal Dependencies

Dependencies within this module:

- `json`
- `manifest`
- `writer`

## External Dependencies

Dependencies from other modules:

- `analyze`
- `anyhow`
- `chrono`
- `scan`
- `serde`
- `std`
- `super`
- `types`

