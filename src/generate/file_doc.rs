//! L2 file-level documentation generator.
//!
//! Generates deep documentation for exceptionally complex files
//! that exceed the complexity threshold (>1000 lines OR >50 public symbols).

use crate::analyze::path_to_slug;
use crate::types::{FileEntry, MemoryEntry, Symbol, Visibility};

/// Default threshold for L2 file generation
pub const DEFAULT_COMPLEX_LINES_THRESHOLD: usize = 1000;
pub const DEFAULT_COMPLEX_SYMBOLS_THRESHOLD: usize = 50;

/// Check if a file is complex enough for L2 documentation
pub fn is_complex_file(
    file: &FileEntry,
    symbols: &[Symbol],
    lines_threshold: usize,
    symbols_threshold: usize,
) -> bool {
    if file.line_count >= lines_threshold {
        return true;
    }

    let public_symbols = symbols
        .iter()
        .filter(|s| matches!(s.visibility, Visibility::Public))
        .count();

    public_symbols >= symbols_threshold
}

/// Generate L2 file documentation
pub fn generate_file_doc(
    file: &FileEntry,
    symbols: &[Symbol],
    memory: &[MemoryEntry],
    module_slug: &str,
) -> String {
    let mut output = String::new();

    // Header
    output.push_str(&format!("# {}\n\n", file.relative_path));

    // Navigation
    output.push_str(&format!(
        "[← Back to Module](../modules/{}/MODULE.md) | [← Back to INDEX](../INDEX.md)\n\n",
        module_slug
    ));

    // File info
    output.push_str("## Overview\n\n");
    output.push_str(&format!("- **Lines:** {}\n", file.line_count));
    output.push_str(&format!("- **Language:** {:?}\n", file.language));
    output.push_str(&format!("- **Symbols:** {}\n", symbols.len()));

    let public_count = symbols
        .iter()
        .filter(|s| matches!(s.visibility, Visibility::Public))
        .count();
    output.push_str(&format!("- **Public symbols:** {}\n\n", public_count));

    // Complete symbol table
    if !symbols.is_empty() {
        output.push_str("## Symbol Table\n\n");
        output.push_str("| Line | Kind | Name | Visibility | Signature |\n");
        output.push_str("| ---- | ---- | ---- | ---------- | --------- |\n");

        for sym in symbols {
            let sig = sym
                .signature
                .as_ref()
                .map(|s| format!("`{}`", truncate_signature(s, 50)))
                .unwrap_or_else(|| "-".to_string());

            output.push_str(&format!(
                "| {} | {} | {} | {} | {} |\n",
                sym.line_range.start, sym.kind, sym.name, sym.visibility, sig
            ));
        }
        output.push('\n');
    }

    // Public API section
    let public_symbols: Vec<_> = symbols
        .iter()
        .filter(|s| matches!(s.visibility, Visibility::Public))
        .collect();

    if !public_symbols.is_empty() {
        output.push_str("## Public API\n\n");

        for sym in public_symbols {
            if let Some(ref sig) = sym.signature {
                output.push_str(&format!("### `{}`\n\n", sym.name));
                output.push_str(&format!("```\n{}\n```\n\n", sig));
                output.push_str(&format!(
                    "**Line:** {} | **Kind:** {}\n\n",
                    sym.line_range.start, sym.kind
                ));
            }
        }
    }

    // Memory markers for this file
    let file_memory: Vec<_> = memory
        .iter()
        .filter(|e| e.source_file == file.relative_path)
        .collect();

    if !file_memory.is_empty() {
        output.push_str("## Memory Markers\n\n");

        for entry in file_memory {
            let priority_badge = match entry.priority {
                crate::types::Priority::High => "🔴",
                crate::types::Priority::Medium => "🟡",
                crate::types::Priority::Low => "🟢",
            };

            output.push_str(&format!(
                "### {} `{}` (line {})\n\n> {}\n\n",
                priority_badge, entry.kind, entry.line_number, entry.content
            ));
        }
    }

    output
}

/// Convert file path to L2 slug
pub fn file_path_to_slug(path: &str) -> String {
    path_to_slug(path)
        .replace('.', "-")
        .trim_end_matches('-')
        .to_string()
}

/// Truncate signature for table display
fn truncate_signature(sig: &str, max_len: usize) -> String {
    let sig = sig.replace('\n', " ").replace("  ", " ");
    if sig.len() <= max_len {
        sig
    } else {
        format!("{}...", &sig[..max_len - 3])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Language, LineRange, SymbolKind};
    use std::path::PathBuf;

    fn make_file(path: &str, lines: usize) -> FileEntry {
        FileEntry {
            path: PathBuf::from(path),
            relative_path: path.to_string(),
            extension: Some("rs".to_string()),
            language: Language::Rust,
            size_bytes: 1000,
            line_count: lines,
            is_large: lines > 500,
        }
    }

    fn make_symbol(name: &str, visibility: Visibility) -> Symbol {
        Symbol {
            name: name.to_string(),
            kind: SymbolKind::Function,
            visibility,
            line_range: LineRange { start: 1, end: 10 },
            signature: Some(format!("fn {}()", name)),
            doc_comment: None,
        }
    }

    #[test]
    fn test_is_complex_by_lines() {
        let file = make_file("big.rs", 1500);
        let symbols: Vec<Symbol> = vec![];

        assert!(is_complex_file(&file, &symbols, 1000, 50));
    }

    #[test]
    fn test_is_complex_by_symbols() {
        let file = make_file("many_symbols.rs", 500);
        let symbols: Vec<Symbol> = (0..60)
            .map(|i| make_symbol(&format!("func{}", i), Visibility::Public))
            .collect();

        assert!(is_complex_file(&file, &symbols, 1000, 50));
    }

    #[test]
    fn test_not_complex() {
        let file = make_file("simple.rs", 100);
        let symbols: Vec<Symbol> = (0..10)
            .map(|i| make_symbol(&format!("func{}", i), Visibility::Public))
            .collect();

        assert!(!is_complex_file(&file, &symbols, 1000, 50));
    }

    #[test]
    fn test_file_path_to_slug() {
        assert_eq!(
            file_path_to_slug("src/analyze/parser.rs"),
            "src-analyze-parser-rs"
        );
        assert_eq!(file_path_to_slug("main.rs"), "main-rs");
    }

    #[test]
    fn test_generate_file_doc() {
        let file = make_file("src/big.rs", 1500);
        let symbols = vec![make_symbol("my_func", Visibility::Public)];
        let memory: Vec<MemoryEntry> = vec![];

        let result = generate_file_doc(&file, &symbols, &memory, "src");

        assert!(result.contains("# src/big.rs"));
        assert!(result.contains("Lines:** 1500"));
        assert!(result.contains("## Symbol Table"));
        assert!(result.contains("## Public API"));
        assert!(result.contains("my_func"));
    }

    #[test]
    fn test_truncate_signature() {
        let short = "fn foo()";
        assert_eq!(truncate_signature(short, 50), short);

        let long = "fn very_long_function_name_with_many_parameters(a: i32, b: i32, c: String, d: Vec<u8>) -> Result<(), Error>";
        let truncated = truncate_signature(long, 50);
        assert!(truncated.len() <= 50);
        assert!(truncated.ends_with("..."));
    }
}
