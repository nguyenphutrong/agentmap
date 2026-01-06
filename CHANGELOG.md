# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2026-01-06

### Added

- **Hierarchical content architecture** - Three-level documentation structure (L0/L1/L2) for better scalability
- **Module detection** - Automatic boundary detection via `mod.rs`, `__init__.py`, `index.{js,ts}`
- **9 language support** - Rust, Python, JavaScript/TypeScript, Go, Swift, Dart, Ruby, C#, Java
- **Hub file detection** - Identify high-impact files imported by 3+ others
- **Import graph visualization** - Show file dependencies in `imports.md`
- **Git diff mode** (`--diff`) - Filter output to changed files only
- **JSON output** (`--json`) - Machine-readable output for tooling integration
- **Remote repository support** - Analyze GitHub repositories directly via URL
- **Depth limiting** (`--depth`) - Control output tree depth for large codebases
- **Complex file threshold** (`--complex-threshold`) - Configure L2 file generation

### Changed

- Output structure from flat files to hierarchical `.agentmap/` directory
- `AGENTS.md` replaced with `INDEX.md` as global routing table
- Documentation now scoped per module (`modules/{slug}/`)

### Output Structure

```
.agentmap/
├── INDEX.md              # L0: Global routing table
├── modules/
│   └── {module-slug}/
│       ├── MODULE.md     # L1: Module summary
│       ├── outline.md    # L1: Symbol maps
│       ├── memory.md     # L1: Warnings/TODOs
│       └── imports.md    # L1: Dependencies
└── files/
    └── {file-slug}.md    # L2: Deep docs (optional)
```

## [0.1.0] - 2026-01-06

### Added

- Initial implementation
- Core symbol extraction for Rust, Python, JavaScript/TypeScript, Go
- Memory marker detection (TODO, WARNING, SAFETY, RULE, etc.)
- `outline.md` generation for large files (>500 lines)
- `memory.md` generation for warnings and business rules
- `AGENTS.md` generation with reading protocol
- CLI with basic options (`-o`, `-t`, `-v`, `--dry-run`)
