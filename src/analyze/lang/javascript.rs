use crate::analyze::lang::LanguageParser;
use crate::types::{Symbol, SymbolKind, Visibility};
use once_cell::sync::Lazy;
use regex::Regex;

pub struct JavaScriptParser;

static FUNCTION_PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?m)^[ \t]*(export\s+)?(async\s+)?function\s+(\w+)").unwrap());

static CLASS_PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?m)^[ \t]*(export\s+)?(default\s+)?class\s+(\w+)").unwrap());

static ARROW_CONST_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?m)^[ \t]*(export\s+)?(const|let|var)\s+(\w+)\s*=\s*(async\s*)?\([^)]*\)\s*=>")
        .unwrap()
});

static SIMPLE_ARROW_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?m)^[ \t]*(export\s+)?(const|let|var)\s+(\w+)\s*=\s*(async\s*)?\w+\s*=>").unwrap()
});

static INTERFACE_PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?m)^[ \t]*(export\s+)?interface\s+(\w+)").unwrap());

static TYPE_PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?m)^[ \t]*(export\s+)?type\s+(\w+)\s*=").unwrap());

static CONST_PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?m)^[ \t]*(export\s+)?const\s+(\w+)\s*:\s*[^=]+=\s*[^(]").unwrap());

impl LanguageParser for JavaScriptParser {
    fn parse_symbols(&self, content: &str) -> Vec<Symbol> {
        let mut symbols = Vec::new();
        let lines: Vec<&str> = content.lines().collect();

        for cap in FUNCTION_PATTERN.captures_iter(content) {
            let is_export = cap.get(1).is_some();
            let name = cap.get(3).map(|m| m.as_str()).unwrap_or("");
            let line = line_number_at_offset(content, cap.get(0).unwrap().start());
            let signature = lines.get(line - 1).map(|s| s.trim().to_string());
            let end_line = find_brace_end(content, cap.get(0).unwrap().end());

            let mut sym = Symbol::new(
                SymbolKind::Function,
                name.to_string(),
                line,
                if is_export {
                    Visibility::Public
                } else {
                    Visibility::Private
                },
            );
            if let Some(sig) = signature {
                sym = sym.with_signature(sig);
            }
            if let Some(end) = end_line {
                sym = sym.with_line_range(line, end);
            }
            symbols.push(sym);
        }

        for cap in CLASS_PATTERN.captures_iter(content) {
            let is_export = cap.get(1).is_some();
            let name = cap.get(3).map(|m| m.as_str()).unwrap_or("");
            let line = line_number_at_offset(content, cap.get(0).unwrap().start());
            let end_line = find_brace_end(content, cap.get(0).unwrap().end());

            let mut sym = Symbol::new(
                SymbolKind::Class,
                name.to_string(),
                line,
                if is_export {
                    Visibility::Public
                } else {
                    Visibility::Private
                },
            );
            if let Some(end) = end_line {
                sym = sym.with_line_range(line, end);
            }
            symbols.push(sym);
        }

        for cap in ARROW_CONST_PATTERN.captures_iter(content) {
            let is_export = cap.get(1).is_some();
            let name = cap.get(3).map(|m| m.as_str()).unwrap_or("");
            let line = line_number_at_offset(content, cap.get(0).unwrap().start());
            let signature = lines.get(line - 1).map(|s| s.trim().to_string());
            let end_line = find_brace_end(content, cap.get(0).unwrap().end());

            let mut sym = Symbol::new(
                SymbolKind::Function,
                name.to_string(),
                line,
                if is_export {
                    Visibility::Public
                } else {
                    Visibility::Private
                },
            );
            if let Some(sig) = signature {
                sym = sym.with_signature(sig);
            }
            if let Some(end) = end_line {
                sym = sym.with_line_range(line, end);
            }
            symbols.push(sym);
        }

        for cap in SIMPLE_ARROW_PATTERN.captures_iter(content) {
            let is_export = cap.get(1).is_some();
            let name = cap.get(3).map(|m| m.as_str()).unwrap_or("");
            let line = line_number_at_offset(content, cap.get(0).unwrap().start());

            symbols.push(Symbol::new(
                SymbolKind::Function,
                name.to_string(),
                line,
                if is_export {
                    Visibility::Public
                } else {
                    Visibility::Private
                },
            ));
        }

        for cap in INTERFACE_PATTERN.captures_iter(content) {
            let is_export = cap.get(1).is_some();
            let name = cap.get(2).map(|m| m.as_str()).unwrap_or("");
            let line = line_number_at_offset(content, cap.get(0).unwrap().start());
            let end_line = find_brace_end(content, cap.get(0).unwrap().end());

            let mut sym = Symbol::new(
                SymbolKind::Interface,
                name.to_string(),
                line,
                if is_export {
                    Visibility::Public
                } else {
                    Visibility::Private
                },
            );
            if let Some(end) = end_line {
                sym = sym.with_line_range(line, end);
            }
            symbols.push(sym);
        }

        for cap in TYPE_PATTERN.captures_iter(content) {
            let is_export = cap.get(1).is_some();
            let name = cap.get(2).map(|m| m.as_str()).unwrap_or("");
            let line = line_number_at_offset(content, cap.get(0).unwrap().start());

            symbols.push(Symbol::new(
                SymbolKind::Type,
                name.to_string(),
                line,
                if is_export {
                    Visibility::Public
                } else {
                    Visibility::Private
                },
            ));
        }

        symbols.sort_by_key(|s| s.line_range.start);
        symbols
    }
}

fn line_number_at_offset(content: &str, offset: usize) -> usize {
    content[..offset].matches('\n').count() + 1
}

fn find_brace_end(content: &str, start_offset: usize) -> Option<usize> {
    let bytes = content.as_bytes();
    let mut depth = 0;
    let mut found_open = false;
    let mut i = start_offset;

    while i < bytes.len() {
        match bytes[i] {
            b'{' => {
                depth += 1;
                found_open = true;
            }
            b'}' => {
                depth -= 1;
                if found_open && depth == 0 {
                    return Some(line_number_at_offset(content, i));
                }
            }
            _ => {}
        }
        i += 1;
    }

    None
}
