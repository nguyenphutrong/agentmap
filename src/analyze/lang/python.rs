use crate::analyze::lang::LanguageParser;
use crate::types::{Symbol, SymbolKind, Visibility};
use once_cell::sync::Lazy;
use regex::Regex;

pub struct PythonParser;

static DEF_PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?m)^([ \t]*)(async\s+)?def\s+(\w+)\s*\(").unwrap());

static CLASS_PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?m)^([ \t]*)class\s+(\w+)").unwrap());

static IMPORT_PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?m)^[ \t]*import\s+([\w.]+)").unwrap());

static FROM_IMPORT_PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?m)^[ \t]*from\s+([\w.]+)\s+import").unwrap());

impl LanguageParser for PythonParser {
    fn parse_symbols(&self, content: &str) -> Vec<Symbol> {
        let mut symbols = Vec::new();
        let lines: Vec<&str> = content.lines().collect();

        for cap in DEF_PATTERN.captures_iter(content) {
            let indent = cap.get(1).map(|m| m.as_str().len()).unwrap_or(0);
            let name = cap.get(3).map(|m| m.as_str()).unwrap_or("");
            let line = line_number_at_offset(content, cap.get(0).unwrap().start());
            let signature = lines.get(line - 1).map(|s| s.trim().to_string());
            let end_line = find_indent_end(&lines, line, indent);

            let is_private = name.starts_with('_') && !name.starts_with("__");
            let is_dunder = name.starts_with("__") && name.ends_with("__");

            let visibility = if is_dunder {
                Visibility::Public
            } else if is_private {
                Visibility::Private
            } else {
                Visibility::Public
            };

            let mut sym = Symbol::new(SymbolKind::Function, name.to_string(), line, visibility);
            if let Some(sig) = signature {
                sym = sym.with_signature(sig);
            }
            sym = sym.with_line_range(line, end_line);
            symbols.push(sym);
        }

        for cap in CLASS_PATTERN.captures_iter(content) {
            let indent = cap.get(1).map(|m| m.as_str().len()).unwrap_or(0);
            let name = cap.get(2).map(|m| m.as_str()).unwrap_or("");
            let line = line_number_at_offset(content, cap.get(0).unwrap().start());
            let signature = lines.get(line - 1).map(|s| s.trim().to_string());
            let end_line = find_indent_end(&lines, line, indent);

            let is_private = name.starts_with('_');

            let mut sym = Symbol::new(
                SymbolKind::Class,
                name.to_string(),
                line,
                if is_private {
                    Visibility::Private
                } else {
                    Visibility::Public
                },
            );
            if let Some(sig) = signature {
                sym = sym.with_signature(sig);
            }
            sym = sym.with_line_range(line, end_line);
            symbols.push(sym);
        }

        symbols.sort_by_key(|s| s.line_range.start);
        symbols
    }

    fn parse_imports(&self, content: &str) -> Vec<String> {
        let mut imports = Vec::new();

        for cap in IMPORT_PATTERN.captures_iter(content) {
            if let Some(m) = cap.get(1) {
                let module = m.as_str().split('.').next().unwrap_or("").to_string();
                if !module.is_empty() && !imports.contains(&module) {
                    imports.push(module);
                }
            }
        }

        for cap in FROM_IMPORT_PATTERN.captures_iter(content) {
            if let Some(m) = cap.get(1) {
                let module = m.as_str().split('.').next().unwrap_or("").to_string();
                if !module.is_empty() && !imports.contains(&module) {
                    imports.push(module);
                }
            }
        }

        imports
    }
}

fn line_number_at_offset(content: &str, offset: usize) -> usize {
    content[..offset].matches('\n').count() + 1
}

fn find_indent_end(lines: &[&str], start_line: usize, base_indent: usize) -> usize {
    let mut end_line = start_line;

    for (i, line) in lines.iter().enumerate().skip(start_line) {
        let trimmed = line.trim();

        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        let current_indent = line.len() - line.trim_start().len();

        if current_indent <= base_indent {
            break;
        }

        end_line = i + 1;
    }

    end_line
}
