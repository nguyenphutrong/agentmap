use crate::types::{FileEntry, Symbol};

pub fn generate_outline(files: &[(FileEntry, Vec<Symbol>)]) -> String {
    if files.is_empty() {
        return "# Outline\n\nNo large files found in this repository.".to_string();
    }

    let mut output = String::new();

    output.push_str("# Code Outline\n\n");
    output.push_str("This file contains symbol maps for large files in the codebase.\n\n");

    output.push_str("## Table of Contents\n\n");
    for (file, symbols) in files {
        let anchor = file.relative_path.replace(['/', '.'], "-").to_lowercase();
        output.push_str(&format!(
            "- [{}](#{}) ({} lines, {} symbols)\n",
            file.relative_path,
            anchor,
            file.line_count,
            symbols.len()
        ));
    }
    output.push_str("\n---\n\n");

    for (file, symbols) in files {
        output.push_str(&format!(
            "## {} ({} lines)\n\n",
            file.relative_path, file.line_count
        ));

        if symbols.is_empty() {
            output.push_str("_No symbols extracted._\n\n");
        } else {
            output.push_str("| Line | Kind | Name | Visibility |\n");
            output.push_str("| ---- | ---- | ---- | ---------- |\n");

            for sym in symbols {
                output.push_str(&format!(
                    "| {} | {} | {} | {} |\n",
                    sym.line_range.start, sym.kind, sym.name, sym.visibility
                ));
            }
            output.push('\n');

            let key_entries: Vec<_> = symbols
                .iter()
                .filter(|s| {
                    matches!(s.visibility, crate::types::Visibility::Public)
                        && (matches!(s.kind, crate::types::SymbolKind::Function)
                            || matches!(s.kind, crate::types::SymbolKind::Class)
                            || matches!(s.kind, crate::types::SymbolKind::Struct))
                })
                .take(5)
                .collect();

            if !key_entries.is_empty() {
                output.push_str("### Key Entry Points\n\n");
                for sym in key_entries {
                    let sig = sym.signature.as_deref().unwrap_or(&sym.name);
                    output.push_str(&format!("- `{}` (L{})\n", sig, sym.line_range.start));
                }
                output.push('\n');
            }
        }

        output.push_str("---\n\n");
    }

    output
}
