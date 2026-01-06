use crate::analyze::lang::LanguageParser;
use crate::types::{Symbol, SymbolKind, Visibility};
use once_cell::sync::Lazy;
use regex::Regex;

pub struct CParser;

static FUNCTION_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?m)^(static\s+)?(inline\s+)?(const\s+)?(\w+(?:\s*\*)*)\s+(\w+)\s*\([^)]*\)\s*\{")
        .unwrap()
});

static FUNCTION_DECL_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?m)^(extern\s+)?(static\s+)?(\w+(?:\s*\*)*)\s+(\w+)\s*\([^)]*\)\s*;").unwrap()
});

static STRUCT_PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?m)^(typedef\s+)?struct\s+(\w+)?\s*\{").unwrap());

static ENUM_PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?m)^(typedef\s+)?enum\s+(\w+)?\s*\{").unwrap());

static TYPEDEF_PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?m)^typedef\s+.+?\s+(\w+)\s*;").unwrap());

impl LanguageParser for CParser {
    fn parse_symbols(&self, content: &str) -> Vec<Symbol> {
        let mut symbols = Vec::new();

        for cap in FUNCTION_PATTERN.captures_iter(content) {
            let is_static = cap.get(1).is_some();
            let name = cap.get(5).map(|m| m.as_str()).unwrap_or("");
            let line = line_number_at_offset(content, cap.get(0).unwrap().start());

            if name == "if" || name == "for" || name == "while" || name == "switch" {
                continue;
            }

            let visibility = if is_static {
                Visibility::Private
            } else {
                Visibility::Public
            };

            let end_line = find_brace_end(content, cap.get(0).unwrap().end() - 1)
                .map(|pos| line_number_at_offset(content, pos))
                .unwrap_or(line);

            let full_match = cap.get(0).unwrap().as_str().trim();
            let signature = full_match.trim_end_matches('{').trim().to_string();

            let mut sym = Symbol::new(SymbolKind::Function, name.to_string(), line, visibility);
            sym = sym.with_line_range(line, end_line);
            sym = sym.with_signature(signature);
            symbols.push(sym);
        }

        for cap in FUNCTION_DECL_PATTERN.captures_iter(content) {
            let is_static = cap.get(2).is_some();
            let name = cap.get(4).map(|m| m.as_str()).unwrap_or("");
            let line = line_number_at_offset(content, cap.get(0).unwrap().start());

            let visibility = if is_static {
                Visibility::Private
            } else {
                Visibility::Public
            };

            let full_match = cap.get(0).unwrap().as_str().trim();
            let signature = full_match.trim_end_matches(';').trim().to_string();

            let mut sym = Symbol::new(SymbolKind::Function, name.to_string(), line, visibility);
            sym = sym.with_signature(signature);
            symbols.push(sym);
        }

        for cap in STRUCT_PATTERN.captures_iter(content) {
            let name = cap.get(2).map(|m| m.as_str()).unwrap_or("anonymous");
            let line = line_number_at_offset(content, cap.get(0).unwrap().start());

            let end_line = find_brace_end(content, cap.get(0).unwrap().end() - 1)
                .map(|pos| line_number_at_offset(content, pos))
                .unwrap_or(line);

            let mut sym = Symbol::new(
                SymbolKind::Struct,
                name.to_string(),
                line,
                Visibility::Public,
            );
            sym = sym.with_line_range(line, end_line);
            sym = sym.with_signature(format!("struct {}", name));
            symbols.push(sym);
        }

        for cap in ENUM_PATTERN.captures_iter(content) {
            let name = cap.get(2).map(|m| m.as_str()).unwrap_or("anonymous");
            let line = line_number_at_offset(content, cap.get(0).unwrap().start());

            let end_line = find_brace_end(content, cap.get(0).unwrap().end() - 1)
                .map(|pos| line_number_at_offset(content, pos))
                .unwrap_or(line);

            let mut sym = Symbol::new(SymbolKind::Enum, name.to_string(), line, Visibility::Public);
            sym = sym.with_line_range(line, end_line);
            sym = sym.with_signature(format!("enum {}", name));
            symbols.push(sym);
        }

        for cap in TYPEDEF_PATTERN.captures_iter(content) {
            let name = cap.get(1).map(|m| m.as_str()).unwrap_or("");
            let line = line_number_at_offset(content, cap.get(0).unwrap().start());

            let sym = Symbol::new(SymbolKind::Type, name.to_string(), line, Visibility::Public);
            symbols.push(sym);
        }

        symbols.sort_by_key(|s| s.line_range.start);
        symbols.dedup_by(|a, b| a.name == b.name && a.line_range.start == b.line_range.start);
        symbols
    }
}

fn line_number_at_offset(content: &str, offset: usize) -> usize {
    content[..offset].matches('\n').count() + 1
}

fn find_brace_end(content: &str, start: usize) -> Option<usize> {
    let bytes = content.as_bytes();
    let mut depth = 0;
    let mut in_string = false;
    let mut string_char = b'"';
    let mut i = start;

    while i < bytes.len() {
        let b = bytes[i];

        if in_string {
            if b == string_char && (i == 0 || bytes[i - 1] != b'\\') {
                in_string = false;
            }
            i += 1;
            continue;
        }

        match b {
            b'"' | b'\'' => {
                in_string = true;
                string_char = b;
            }
            b'{' => depth += 1,
            b'}' => {
                depth -= 1;
                if depth == 0 {
                    return Some(i);
                }
            }
            _ => {}
        }
        i += 1;
    }
    None
}
