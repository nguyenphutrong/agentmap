# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.0] - 2026-01-06

### Added

- **Smart file generation** - Reduce file clutter for small/medium projects:
  - Skip empty `outline.md` when no large files exist
  - Skip empty `memory.md` when no TODO/WARNING markers exist
  - Auto-merge small content (<500 bytes) inline into `MODULE.md`
- **CI/CD** - Automatic publish to crates.io on release

### Changed

- `MODULE.md` now conditionally shows Documentation section (only if separate files exist)
- Small imports/outline/memory content inlined with `---` separator

### Performance

- Up to **70% reduction** in generated files for typical projects (33 → 10 files)

## [0.2.0] - 2026-01-06

Initial release.

### Added

- **Hierarchical content architecture** - Three-level documentation structure (L0/L1/L2) for scalability
- **Module detection** - Automatic boundary detection via `mod.rs`, `__init__.py`, `index.{js,ts}`, or 5+ files
- **9 language support** - Rust, Python, JavaScript/TypeScript, Go, Swift, Dart, Ruby, C#, Java
- **Symbol extraction** - Functions, classes, structs, traits, enums, interfaces
- **Memory marker detection** - TODO, FIXME, WARNING, SAFETY, RULE, DEPRECATED, etc.
- **Hub file detection** - Identify high-impact files imported by 3+ others
- **Import graph visualization** - Show intra/inter-module dependencies
- **Git diff mode** (`--diff`) - Filter output to changed files only
- **JSON output** (`--json`) - Machine-readable output for tooling integration
- **Remote repository support** - Analyze GitHub repositories directly via URL
- **Incremental regeneration** - Only update modules that changed
- **CLI options**: `-o`, `-t`, `-c`, `-d`, `-i`, `-l`, `-v`, `--dry-run`, `--no-gitignore`

### Output Structure

```
.agentmap/
├── INDEX.md              # L0: Global routing table
├── modules/
│   └── {module-slug}/
│       ├── MODULE.md     # L1: Module summary
│       ├── outline.md    # L1: Symbol maps (if large files exist)
│       ├── memory.md     # L1: Warnings/TODOs (if markers exist)
│       └── imports.md    # L1: Dependencies (inline if small)
└── files/
    └── {file-slug}.md    # L2: Deep docs for complex files
```
