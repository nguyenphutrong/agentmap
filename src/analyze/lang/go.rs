use crate::analyze::lang::LanguageParser;
use crate::types::{Symbol, SymbolKind, Visibility};
use once_cell::sync::Lazy;
use regex::Regex;

pub struct GoParser;

static FUNC_PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?m)^func\s+(?:\([^)]+\)\s+)?(\w+)\s*\(").unwrap());

static STRUCT_PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?m)^type\s+(\w+)\s+struct\b").unwrap());

static INTERFACE_PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?m)^type\s+(\w+)\s+interface\b").unwrap());

static CONST_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?m)^const\s+(\w+)\s*=").unwrap());

static IMPORT_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r#"(?m)^\s*"([^"]+)"\s*$"#).unwrap());

static IMPORT_BLOCK_PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?s)import\s*\(([^)]+)\)").unwrap());

static SINGLE_IMPORT_PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"(?m)^import\s+"([^"]+)""#).unwrap());

impl LanguageParser for GoParser {
    fn parse_symbols(&self, content: &str) -> Vec<Symbol> {
        let mut symbols = Vec::new();
        let lines: Vec<&str> = content.lines().collect();

        for cap in FUNC_PATTERN.captures_iter(content) {
            let name = cap.get(1).map(|m| m.as_str()).unwrap_or("");
            let line = line_number_at_offset(content, cap.get(0).unwrap().start());
            let signature = lines.get(line - 1).map(|s| s.trim().to_string());
            let end_line = find_brace_end(content, cap.get(0).unwrap().end());

            let is_exported = name
                .chars()
                .next()
                .map(|c| c.is_uppercase())
                .unwrap_or(false);

            let mut sym = Symbol::new(
                SymbolKind::Function,
                name.to_string(),
                line,
                if is_exported {
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

        for cap in STRUCT_PATTERN.captures_iter(content) {
            let name = cap.get(1).map(|m| m.as_str()).unwrap_or("");
            let line = line_number_at_offset(content, cap.get(0).unwrap().start());
            let end_line = find_brace_end(content, cap.get(0).unwrap().end());

            let is_exported = name
                .chars()
                .next()
                .map(|c| c.is_uppercase())
                .unwrap_or(false);

            let mut sym = Symbol::new(
                SymbolKind::Struct,
                name.to_string(),
                line,
                if is_exported {
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

        for cap in INTERFACE_PATTERN.captures_iter(content) {
            let name = cap.get(1).map(|m| m.as_str()).unwrap_or("");
            let line = line_number_at_offset(content, cap.get(0).unwrap().start());
            let end_line = find_brace_end(content, cap.get(0).unwrap().end());

            let is_exported = name
                .chars()
                .next()
                .map(|c| c.is_uppercase())
                .unwrap_or(false);

            let mut sym = Symbol::new(
                SymbolKind::Interface,
                name.to_string(),
                line,
                if is_exported {
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

        for cap in CONST_PATTERN.captures_iter(content) {
            let name = cap.get(1).map(|m| m.as_str()).unwrap_or("");
            let line = line_number_at_offset(content, cap.get(0).unwrap().start());

            let is_exported = name
                .chars()
                .next()
                .map(|c| c.is_uppercase())
                .unwrap_or(false);

            symbols.push(Symbol::new(
                SymbolKind::Const,
                name.to_string(),
                line,
                if is_exported {
                    Visibility::Public
                } else {
                    Visibility::Private
                },
            ));
        }

        symbols.sort_by_key(|s| s.line_range.start);
        symbols
    }

    fn parse_imports(&self, content: &str) -> Vec<String> {
        let mut imports = Vec::new();

        for cap in SINGLE_IMPORT_PATTERN.captures_iter(content) {
            if let Some(m) = cap.get(1) {
                let path = extract_go_package_name(m.as_str());
                if !imports.contains(&path) {
                    imports.push(path);
                }
            }
        }

        for cap in IMPORT_BLOCK_PATTERN.captures_iter(content) {
            if let Some(block) = cap.get(1) {
                for line_cap in IMPORT_PATTERN.captures_iter(block.as_str()) {
                    if let Some(m) = line_cap.get(1) {
                        let path = extract_go_package_name(m.as_str());
                        if !imports.contains(&path) {
                            imports.push(path);
                        }
                    }
                }
            }
        }

        imports
    }
}

fn extract_go_package_name(import_path: &str) -> String {
    import_path
        .rsplit('/')
        .next()
        .unwrap_or(import_path)
        .to_string()
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
