use crate::analyze::lang::LanguageParser;
use crate::types::{Symbol, SymbolKind, Visibility};
use once_cell::sync::Lazy;
use regex::Regex;

pub struct CppParser;

// class ClassName : public Base { or class ClassName {
static CLASS_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?m)^(?:template\s*<[^>]*>\s*)?(class|struct)\s+(\w+)(?:\s*:\s*[^{]+)?\s*\{")
        .unwrap()
});

// namespace Name {
static NAMESPACE_PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?m)^namespace\s+(\w+)\s*\{").unwrap());

// enum class EnumName { or enum EnumName {
static ENUM_PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?m)^enum(?:\s+class)?\s+(\w+)\s*\{").unwrap());

// ReturnType FunctionName(params) { or ReturnType ClassName::MethodName(params) {
static FUNCTION_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?m)^(virtual\s+|static\s+|inline\s+|constexpr\s+|explicit\s+)*(\w+(?:\s*[*&])?(?:\s*<[^>]+>)?)\s+(\w+(?:::\w+)?)\s*\([^)]*\)(?:\s*(?:const|override|noexcept|final))*\s*\{")
        .unwrap()
});

// virtual ReturnType methodName(params) const; (declaration without body)
static METHOD_DECL_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?m)^\s*(virtual\s+|static\s+|inline\s+|constexpr\s+|explicit\s+)*(\w+(?:\s*[*&])?(?:\s*<[^>]+>)?)\s+(\w+)\s*\([^)]*\)(?:\s*(?:const|override|noexcept|final))*\s*;")
        .unwrap()
});

#[allow(dead_code)]
static VISIBILITY_PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?m)^(public|private|protected)\s*:").unwrap());

impl LanguageParser for CppParser {
    fn parse_symbols(&self, content: &str) -> Vec<Symbol> {
        let mut symbols = Vec::new();

        for cap in CLASS_PATTERN.captures_iter(content) {
            let kind_str = cap.get(1).map(|m| m.as_str()).unwrap_or("class");
            let name = cap.get(2).map(|m| m.as_str()).unwrap_or("");
            let line = line_number_at_offset(content, cap.get(0).unwrap().start());

            let kind = if kind_str == "struct" {
                SymbolKind::Struct
            } else {
                SymbolKind::Class
            };

            let end_line = find_brace_end(content, cap.get(0).unwrap().end() - 1)
                .map(|pos| line_number_at_offset(content, pos))
                .unwrap_or(line);

            let mut sym = Symbol::new(kind, name.to_string(), line, Visibility::Public);
            sym = sym.with_line_range(line, end_line);
            sym = sym.with_signature(format!("{} {}", kind_str, name));
            symbols.push(sym);
        }

        for cap in NAMESPACE_PATTERN.captures_iter(content) {
            let name = cap.get(1).map(|m| m.as_str()).unwrap_or("");
            let line = line_number_at_offset(content, cap.get(0).unwrap().start());

            let end_line = find_brace_end(content, cap.get(0).unwrap().end() - 1)
                .map(|pos| line_number_at_offset(content, pos))
                .unwrap_or(line);

            let mut sym = Symbol::new(
                SymbolKind::Module,
                name.to_string(),
                line,
                Visibility::Public,
            );
            sym = sym.with_line_range(line, end_line);
            sym = sym.with_signature(format!("namespace {}", name));
            symbols.push(sym);
        }

        for cap in ENUM_PATTERN.captures_iter(content) {
            let name = cap.get(1).map(|m| m.as_str()).unwrap_or("");
            let line = line_number_at_offset(content, cap.get(0).unwrap().start());

            let end_line = find_brace_end(content, cap.get(0).unwrap().end() - 1)
                .map(|pos| line_number_at_offset(content, pos))
                .unwrap_or(line);

            let mut sym = Symbol::new(SymbolKind::Enum, name.to_string(), line, Visibility::Public);
            sym = sym.with_line_range(line, end_line);
            sym = sym.with_signature(format!("enum {}", name));
            symbols.push(sym);
        }

        for cap in FUNCTION_PATTERN.captures_iter(content) {
            let modifiers = cap.get(1).map(|m| m.as_str()).unwrap_or("");
            let name = cap.get(3).map(|m| m.as_str()).unwrap_or("");
            let line = line_number_at_offset(content, cap.get(0).unwrap().start());

            if is_control_flow_keyword(name) {
                continue;
            }

            let is_static = modifiers.contains("static");
            let visibility = if is_static {
                Visibility::Private
            } else {
                Visibility::Public
            };

            let end_line = find_brace_end(content, cap.get(0).unwrap().end() - 1)
                .map(|pos| line_number_at_offset(content, pos))
                .unwrap_or(line);

            let kind = if name.contains("::") {
                SymbolKind::Method
            } else {
                SymbolKind::Function
            };

            let full_match = cap.get(0).unwrap().as_str().trim();
            let signature = full_match.trim_end_matches('{').trim().to_string();

            let mut sym = Symbol::new(kind, name.to_string(), line, visibility);
            sym = sym.with_line_range(line, end_line);
            sym = sym.with_signature(signature);
            symbols.push(sym);
        }

        for cap in METHOD_DECL_PATTERN.captures_iter(content) {
            let modifiers = cap.get(1).map(|m| m.as_str()).unwrap_or("");
            let name = cap.get(3).map(|m| m.as_str()).unwrap_or("");
            let line = line_number_at_offset(content, cap.get(0).unwrap().start());

            if is_control_flow_keyword(name) || name == "return" {
                continue;
            }

            let is_virtual = modifiers.contains("virtual");
            let visibility = if is_virtual {
                Visibility::Public
            } else {
                Visibility::Internal
            };

            let full_match = cap.get(0).unwrap().as_str().trim();
            let signature = full_match.trim_end_matches(';').trim().to_string();

            let mut sym = Symbol::new(SymbolKind::Method, name.to_string(), line, visibility);
            sym = sym.with_signature(signature);
            symbols.push(sym);
        }

        symbols.sort_by_key(|s| s.line_range.start);
        symbols.dedup_by(|a, b| a.name == b.name && a.line_range.start == b.line_range.start);
        symbols
    }
}

fn is_control_flow_keyword(name: &str) -> bool {
    matches!(name, "if" | "for" | "while" | "switch" | "catch")
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
