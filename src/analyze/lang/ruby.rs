use crate::analyze::lang::LanguageParser;
use crate::types::{Symbol, SymbolKind, Visibility};
use once_cell::sync::Lazy;
use regex::Regex;

pub struct RubyParser;

// class ClassName < ParentClass or class ClassName
static CLASS_PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?m)^class\s+([A-Z]\w*)(?:\s*<\s*\w+)?").unwrap());

// module ModuleName
static MODULE_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?m)^module\s+([A-Z]\w*)").unwrap());

// def method_name or def method_name(params)
static METHOD_PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?m)^\s*def\s+(\w+[?!=]?)(?:\s*\(|$|\s)").unwrap());

// def self.method_name (class method)
static CLASS_METHOD_PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?m)^\s*def\s+self\.(\w+[?!=]?)(?:\s*\(|$|\s)").unwrap());

// attr_reader :name, :other or attr_accessor :name
static ATTR_PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?m)^\s*attr_(reader|writer|accessor)\s+(.+)").unwrap());

impl LanguageParser for RubyParser {
    fn parse_symbols(&self, content: &str) -> Vec<Symbol> {
        let mut symbols = Vec::new();
        let lines: Vec<&str> = content.lines().collect();

        for cap in CLASS_PATTERN.captures_iter(content) {
            let name = cap.get(1).map(|m| m.as_str()).unwrap_or("");
            let line = line_number_at_offset(content, cap.get(0).unwrap().start());
            let end_line = find_ruby_end(&lines, line);

            let mut sym = Symbol::new(
                SymbolKind::Class,
                name.to_string(),
                line,
                Visibility::Public,
            );
            sym = sym.with_line_range(line, end_line);
            sym = sym.with_signature(format!("class {}", name));
            symbols.push(sym);
        }

        for cap in MODULE_PATTERN.captures_iter(content) {
            let name = cap.get(1).map(|m| m.as_str()).unwrap_or("");
            let line = line_number_at_offset(content, cap.get(0).unwrap().start());
            let end_line = find_ruby_end(&lines, line);

            let mut sym = Symbol::new(
                SymbolKind::Module,
                name.to_string(),
                line,
                Visibility::Public,
            );
            sym = sym.with_line_range(line, end_line);
            sym = sym.with_signature(format!("module {}", name));
            symbols.push(sym);
        }

        for cap in CLASS_METHOD_PATTERN.captures_iter(content) {
            let name = cap.get(1).map(|m| m.as_str()).unwrap_or("");
            let line = line_number_at_offset(content, cap.get(0).unwrap().start());
            let end_line = find_ruby_end(&lines, line);
            let visibility = get_visibility_at_line(&lines, line);

            let mut sym = Symbol::new(
                SymbolKind::Function,
                format!("self.{}", name),
                line,
                visibility,
            );
            sym = sym.with_line_range(line, end_line);
            sym = sym.with_signature(format!("def self.{}", name));
            symbols.push(sym);
        }

        for cap in METHOD_PATTERN.captures_iter(content) {
            let full_match = cap.get(0).unwrap().as_str();
            if full_match.contains("self.") {
                continue;
            }

            let name = cap.get(1).map(|m| m.as_str()).unwrap_or("");
            let line = line_number_at_offset(content, cap.get(0).unwrap().start());
            let end_line = find_ruby_end(&lines, line);
            let visibility = get_visibility_at_line(&lines, line);

            let mut sym = Symbol::new(SymbolKind::Method, name.to_string(), line, visibility);
            sym = sym.with_line_range(line, end_line);
            sym = sym.with_signature(format!("def {}", name));
            symbols.push(sym);
        }

        for cap in ATTR_PATTERN.captures_iter(content) {
            let attr_type = cap.get(1).map(|m| m.as_str()).unwrap_or("");
            let attrs_str = cap.get(2).map(|m| m.as_str()).unwrap_or("");
            let line = line_number_at_offset(content, cap.get(0).unwrap().start());

            for attr in attrs_str.split(',') {
                let attr_name = attr.trim().trim_start_matches(':').trim();
                if attr_name.is_empty() {
                    continue;
                }
                let sym = Symbol::new(
                    SymbolKind::Const,
                    attr_name.to_string(),
                    line,
                    Visibility::Public,
                );
                symbols.push(sym);
            }

            let _ = attr_type;
        }

        symbols.sort_by_key(|s| s.line_range.start);
        symbols.dedup_by(|a, b| a.name == b.name && a.line_range.start == b.line_range.start);
        symbols
    }
}

fn line_number_at_offset(content: &str, offset: usize) -> usize {
    content[..offset].matches('\n').count() + 1
}

fn find_ruby_end(lines: &[&str], start_line: usize) -> usize {
    let mut depth = 0;
    for (i, line) in lines.iter().enumerate().skip(start_line - 1) {
        let trimmed = line.trim();

        if trimmed.starts_with("class ")
            || trimmed.starts_with("module ")
            || trimmed.starts_with("def ")
            || trimmed.starts_with("do")
            || trimmed.starts_with("if ")
            || trimmed.starts_with("unless ")
            || trimmed.starts_with("case ")
            || trimmed.starts_with("begin")
            || trimmed == "begin"
        {
            depth += 1;
        }

        if trimmed == "end" || trimmed.starts_with("end ") || trimmed.starts_with("end#") {
            depth -= 1;
            if depth == 0 {
                return i + 1;
            }
        }
    }
    start_line
}

fn get_visibility_at_line(lines: &[&str], target_line: usize) -> Visibility {
    for i in (0..target_line.saturating_sub(1)).rev() {
        let trimmed = lines[i].trim();
        if trimmed == "private" || trimmed.starts_with("private ") {
            return Visibility::Private;
        }
        if trimmed == "protected" || trimmed.starts_with("protected ") {
            return Visibility::Internal;
        }
        if trimmed == "public" || trimmed.starts_with("public ") {
            return Visibility::Public;
        }
        if trimmed.starts_with("class ") || trimmed.starts_with("module ") {
            break;
        }
    }
    Visibility::Public
}
