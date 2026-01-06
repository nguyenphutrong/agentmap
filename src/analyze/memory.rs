use crate::types::{MemoryEntry, MemoryKind};
use once_cell::sync::Lazy;
use regex::Regex;

static STANDARD_ANNOTATION: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?im)(?://|#|/\*+|\*)\s*\b(TODO|FIXME|XXX|BUG|HACK|WARNING|NOTE|WARN)\b[:\s]*(.*)")
        .unwrap()
});

static SAFETY_MARKER: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?im)(?://|#|/\*+|\*)\s*\b(SAFETY|INVARIANT|GUARANTEES?)\b[:\s]*(.*)").unwrap()
});

static BUSINESS_RULE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?im)^\s*(?://|#|/\*+|\*)\s*\b(RULE|POLICY|ACCORDING\s+TO)[:!]?\s+(.+)").unwrap()
});

static DEPRECATED_MARKER: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?im)(?://|#|/\*+|\*)\s*\b(DEPRECATED|@deprecated)\b[:\s]*(.*)").unwrap()
});

pub fn extract_memory_markers(content: &str, source_file: &str) -> Vec<MemoryEntry> {
    let mut entries = Vec::new();

    for cap in STANDARD_ANNOTATION.captures_iter(content) {
        let keyword = cap
            .get(1)
            .map(|m| m.as_str().to_uppercase())
            .unwrap_or_default();
        let message = cap
            .get(2)
            .map(|m| m.as_str().trim())
            .unwrap_or("")
            .to_string();
        let line = line_number_at_offset(content, cap.get(0).unwrap().start());

        if message.is_empty() {
            continue;
        }

        let kind = match keyword.as_str() {
            "TODO" => MemoryKind::Todo,
            "FIXME" => MemoryKind::Fixme,
            "XXX" | "BUG" => MemoryKind::Fixme,
            "HACK" => MemoryKind::Hack,
            "WARNING" | "WARN" => MemoryKind::Warning,
            "NOTE" => MemoryKind::Note,
            _ => MemoryKind::Note,
        };

        entries.push(MemoryEntry::new(
            kind,
            message,
            source_file.to_string(),
            line,
        ));
    }

    for cap in SAFETY_MARKER.captures_iter(content) {
        let keyword = cap
            .get(1)
            .map(|m| m.as_str().to_uppercase())
            .unwrap_or_default();
        let message = cap
            .get(2)
            .map(|m| m.as_str().trim())
            .unwrap_or("")
            .to_string();
        let line = line_number_at_offset(content, cap.get(0).unwrap().start());

        if message.is_empty() {
            continue;
        }

        let kind = match keyword.as_str() {
            "SAFETY" => MemoryKind::Safety,
            "INVARIANT" | "GUARANTEES" | "GUARANTEE" => MemoryKind::Invariant,
            _ => MemoryKind::Safety,
        };

        entries.push(MemoryEntry::new(
            kind,
            message,
            source_file.to_string(),
            line,
        ));
    }

    for cap in BUSINESS_RULE.captures_iter(content) {
        let message = cap
            .get(2)
            .map(|m| m.as_str().trim())
            .unwrap_or("")
            .to_string();
        let line = line_number_at_offset(content, cap.get(0).unwrap().start());

        if message.is_empty() {
            continue;
        }

        entries.push(MemoryEntry::new(
            MemoryKind::BusinessRule,
            message,
            source_file.to_string(),
            line,
        ));
    }

    for cap in DEPRECATED_MARKER.captures_iter(content) {
        let message = cap
            .get(2)
            .map(|m| m.as_str().trim())
            .unwrap_or("")
            .to_string();
        let line = line_number_at_offset(content, cap.get(0).unwrap().start());

        let msg = if message.is_empty() {
            "Deprecated".to_string()
        } else {
            message
        };

        entries.push(MemoryEntry::new(
            MemoryKind::Deprecated,
            msg,
            source_file.to_string(),
            line,
        ));
    }

    entries.sort_by_key(|e| e.line_number);
    entries
}

fn line_number_at_offset(content: &str, offset: usize) -> usize {
    content[..offset].matches('\n').count() + 1
}
