# Imports

[← Back to MODULE](MODULE.md) | [← Back to INDEX](../../INDEX.md)

## Dependency Graph

```mermaid
graph TD
    src_cli[src-cli] --> analyze[analyze]
    src_cli[src-cli] --> anyhow[anyhow]
    src_cli[src-cli] --> clap[clap]
    src_cli[src-cli] --> config[config]
    src_cli[src-cli] --> emit[emit]
    src_cli[src-cli] --> generate[generate]
    src_cli[src-cli] --> mcp[mcp]
    src_cli[src-cli] --> notify[notify]
    src_cli[src-cli] --> notify_debouncer_mini[notify_debouncer_mini]
    src_cli[src-cli] --> rmcp[rmcp]
    src_cli[src-cli] --> scan[scan]
    src_cli[src-cli] --> std[std]
    src_cli[src-cli] --> super[super]
    src_cli[src-cli] --> tempfile[tempfile]
    src_cli[src-cli] --> tui[tui]
    src_cli[src-cli] --> types[types]
```

## Internal Dependencies

Dependencies within this module:

- `args`
- `check`
- `cli`
- `hooks`
- `serve`
- `telemetry`
- `templates`
- `update`
- `watch`

## External Dependencies

Dependencies from other modules:

- `analyze`
- `anyhow`
- `clap`
- `config`
- `emit`
- `generate`
- `mcp`
- `notify`
- `notify_debouncer_mini`
- `rmcp`
- `scan`
- `std`
- `super`
- `tempfile`
- `tui`
- `types`

