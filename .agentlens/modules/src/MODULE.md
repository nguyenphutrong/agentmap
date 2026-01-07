# Module: src

[← Back to INDEX](../../INDEX.md)

**Type:** rust | **Files:** 4

**Entry point:** `src/lib.rs`

## Files

| File | Lines | Large |
| ---- | ----- | ----- |
| `src/config.rs` | 165 |  |
| `src/lib.rs` | 15 |  |
| `src/main.rs` | 551 | 📊 |
| `src/runner.rs` | 325 |  |

## Child Modules

- [src-analyze](../src-analyze/MODULE.md)
- [src-cli](../src-cli/MODULE.md)
- [src-emit](../src-emit/MODULE.md)
- [src-generate](../src-generate/MODULE.md)
- [src-mcp](../src-mcp/MODULE.md)
- [src-scan](../src-scan/MODULE.md)
- [src-telemetry](../src-telemetry/MODULE.md)
- [src-types](../src-types/MODULE.md)

---

Symbol maps for 1 large files in this module.

## src/main.rs (551 lines)

| Line | Kind | Name | Visibility |
| ---- | ---- | ---- | ---------- |
| 31 | fn | main | (private) |
| 128 | fn | run_analysis | (private) |
| 262 | fn | run_json_output | (private) |
| 326 | fn | run_hierarchical_output | (private) |
| 509 | fn | run_init | (private) |
---

Dependencies within this module:

- `config`
- `runner`

## External Dependencies

Dependencies from other modules:

- `agentlens`
- `analyze`
- `anyhow`
- `chrono`
- `clap`
- `cli`
- `emit`
- `generate`
- `mcp`
- `scan`
- `serde`
- `std`
- `super`
- `telemetry`
- `tempfile`
- `types`
