use crate::analyze::lang::LanguageParser;
use crate::types::{Symbol, SymbolKind, Visibility};
use once_cell::sync::Lazy;
use regex::Regex;

pub struct PhpParser;

static CLASS_PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?m)^\s*(abstract\s+)?(final\s+)?class\s+(\w+)").unwrap());

static INTERFACE_PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?m)^\s*interface\s+(\w+)").unwrap());

static TRAIT_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?m)^\s*trait\s+(\w+)").unwrap());

static FUNCTION_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?m)^\s*(public|private|protected|static|\s)*\s*function\s+(\w+)\s*\(").unwrap()
});

static CONST_PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?m)^\s*(public|private|protected)?\s*const\s+(\w+)\s*=").unwrap());

impl LanguageParser for PhpParser {
    fn parse_symbols(&self, content: &str) -> Vec<Symbol> {
        let mut symbols = Vec::new();

        for cap in CLASS_PATTERN.captures_iter(content) {
            let name = cap.get(3).map(|m| m.as_str()).unwrap_or("");
            let line = line_number_at_offset(content, cap.get(0).unwrap().start());
            let is_abstract = cap.get(1).is_some();

            let full_match = cap.get(0).unwrap().as_str().trim();
            let end_line = find_brace_end(content, cap.get(0).unwrap().end())
                .map(|pos| line_number_at_offset(content, pos))
                .unwrap_or(line);

            let mut sym = Symbol::new(
                SymbolKind::Class,
                name.to_string(),
                line,
                Visibility::Public,
            );
            sym = sym.with_line_range(line, end_line);
            if is_abstract {
                sym = sym.with_signature(format!("abstract class {}", name));
            } else {
                sym = sym.with_signature(full_match.to_string());
            }
            symbols.push(sym);
        }

        for cap in INTERFACE_PATTERN.captures_iter(content) {
            let name = cap.get(1).map(|m| m.as_str()).unwrap_or("");
            let line = line_number_at_offset(content, cap.get(0).unwrap().start());
            let end_line = find_brace_end(content, cap.get(0).unwrap().end())
                .map(|pos| line_number_at_offset(content, pos))
                .unwrap_or(line);

            let mut sym = Symbol::new(
                SymbolKind::Interface,
                name.to_string(),
                line,
                Visibility::Public,
            );
            sym = sym.with_line_range(line, end_line);
            sym = sym.with_signature(format!("interface {}", name));
            symbols.push(sym);
        }

        for cap in TRAIT_PATTERN.captures_iter(content) {
            let name = cap.get(1).map(|m| m.as_str()).unwrap_or("");
            let line = line_number_at_offset(content, cap.get(0).unwrap().start());
            let end_line = find_brace_end(content, cap.get(0).unwrap().end())
                .map(|pos| line_number_at_offset(content, pos))
                .unwrap_or(line);

            let mut sym = Symbol::new(
                SymbolKind::Trait,
                name.to_string(),
                line,
                Visibility::Public,
            );
            sym = sym.with_line_range(line, end_line);
            sym = sym.with_signature(format!("trait {}", name));
            symbols.push(sym);
        }

        for cap in FUNCTION_PATTERN.captures_iter(content) {
            let modifiers = cap.get(1).map(|m| m.as_str()).unwrap_or("");
            let name = cap.get(2).map(|m| m.as_str()).unwrap_or("");
            let line = line_number_at_offset(content, cap.get(0).unwrap().start());

            let visibility = if modifiers.contains("private") {
                Visibility::Private
            } else if modifiers.contains("protected") {
                Visibility::Protected
            } else {
                Visibility::Public
            };

            let end_line = find_brace_end(content, cap.get(0).unwrap().end())
                .map(|pos| line_number_at_offset(content, pos))
                .unwrap_or(line);

            let full_match = cap.get(0).unwrap().as_str().trim();
            let signature = full_match.trim_end_matches('(').to_string() + "(...)";

            let mut sym = Symbol::new(SymbolKind::Function, name.to_string(), line, visibility);
            sym = sym.with_line_range(line, end_line);
            sym = sym.with_signature(signature);
            symbols.push(sym);
        }

        for cap in CONST_PATTERN.captures_iter(content) {
            let visibility_str = cap.get(1).map(|m| m.as_str()).unwrap_or("public");
            let name = cap.get(2).map(|m| m.as_str()).unwrap_or("");
            let line = line_number_at_offset(content, cap.get(0).unwrap().start());

            let visibility = match visibility_str {
                "private" => Visibility::Private,
                "protected" => Visibility::Protected,
                _ => Visibility::Public,
            };

            let sym = Symbol::new(SymbolKind::Const, name.to_string(), line, visibility);
            symbols.push(sym);
        }

        symbols.sort_by_key(|s| s.line_range.start);
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
