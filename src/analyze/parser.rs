use crate::analyze::lang::get_parser;
use crate::types::{FileEntry, Symbol};

pub fn extract_symbols(file: &FileEntry, content: &str) -> Vec<Symbol> {
    match get_parser(file.language) {
        Some(parser) => parser.parse_symbols(content),
        None => Vec::new(),
    }
}
