use crate::types::{MemoryEntry, Priority};
use std::collections::BTreeMap;

/// Generates memory.md content from extracted memory entries.
///
/// Groups entries by category (Warnings, Business Rules, Technical Debt, Notes),
/// sorts by priority within categories, and formats with emoji headers.
pub fn generate_memory(entries: &[MemoryEntry]) -> String {
    if entries.is_empty() {
        return "# Memory\n\nNo memory markers found in this repository.".to_string();
    }

    let mut output = String::new();

    output.push_str("# Memory\n\n");
    output.push_str("This file contains extracted knowledge markers from the codebase.\n\n");

    // Group by category
    let mut by_category: BTreeMap<&'static str, Vec<&MemoryEntry>> = BTreeMap::new();
    for entry in entries {
        by_category
            .entry(entry.kind.category())
            .or_default()
            .push(entry);
    }

    // Category order: Warnings first, then Business Rules, Tech Debt, Notes
    let category_order = ["Warnings", "Business Rules", "Technical Debt", "Notes"];

    // Summary table
    output.push_str("## Summary\n\n");
    output.push_str("| Category | Count | High | Medium | Low |\n");
    output.push_str("| -------- | ----- | ---- | ------ | --- |\n");

    for cat in &category_order {
        if let Some(items) = by_category.get(*cat) {
            let high = items
                .iter()
                .filter(|e| e.priority == Priority::High)
                .count();
            let med = items
                .iter()
                .filter(|e| e.priority == Priority::Medium)
                .count();
            let low = items.iter().filter(|e| e.priority == Priority::Low).count();
            let emoji = items.first().map(|e| e.kind.emoji()).unwrap_or("");
            output.push_str(&format!(
                "| {} {} | {} | {} | {} | {} |\n",
                emoji,
                cat,
                items.len(),
                high,
                med,
                low
            ));
        }
    }
    output.push_str("\n---\n\n");

    // Detailed sections
    for cat in &category_order {
        if let Some(items) = by_category.get(*cat) {
            let emoji = items.first().map(|e| e.kind.emoji()).unwrap_or("");
            output.push_str(&format!("## {} {}\n\n", emoji, cat));

            // Sort by priority (High first), then by file/line
            let mut sorted: Vec<_> = items.iter().collect();
            sorted.sort_by(|a, b| {
                a.priority
                    .cmp(&b.priority)
                    .then_with(|| a.source_file.cmp(&b.source_file))
                    .then_with(|| a.line_number.cmp(&b.line_number))
            });

            for entry in sorted {
                let priority_badge = match entry.priority {
                    Priority::High => "🔴",
                    Priority::Medium => "🟡",
                    Priority::Low => "🟢",
                };

                output.push_str(&format!(
                    "### {} `{}` ({}:{})\n\n",
                    priority_badge, entry.kind, entry.source_file, entry.line_number
                ));
                output.push_str(&format!("> {}\n\n", entry.content));
            }

            output.push_str("---\n\n");
        }
    }

    output
}

/// Returns files with high-priority memory entries for AGENTS.md critical files section.
/// Returns tuples of (file_path, count_of_high_priority_entries)
pub fn get_critical_files(entries: &[MemoryEntry]) -> Vec<(String, usize)> {
    let mut file_counts: BTreeMap<&str, usize> = BTreeMap::new();

    for entry in entries.iter().filter(|e| e.priority == Priority::High) {
        *file_counts.entry(&entry.source_file).or_default() += 1;
    }

    let mut result: Vec<_> = file_counts
        .into_iter()
        .map(|(f, c)| (f.to_string(), c))
        .collect();

    // Sort by count descending
    result.sort_by(|a, b| b.1.cmp(&a.1));
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::MemoryKind;

    #[test]
    fn test_empty_entries() {
        let result = generate_memory(&[]);
        assert!(result.contains("No memory markers found"));
    }

    #[test]
    fn test_generates_summary() {
        let entries = vec![
            MemoryEntry::new(
                MemoryKind::Warning,
                "This is dangerous".to_string(),
                "src/lib.rs".to_string(),
                10,
            ),
            MemoryEntry::new(
                MemoryKind::Todo,
                "Implement this".to_string(),
                "src/main.rs".to_string(),
                20,
            ),
        ];

        let result = generate_memory(&entries);
        assert!(result.contains("## Summary"));
        assert!(result.contains("Warnings"));
        assert!(result.contains("Technical Debt"));
    }

    #[test]
    fn test_critical_files() {
        let entries = vec![
            MemoryEntry::new(
                MemoryKind::Warning,
                "Warning 1".to_string(),
                "src/danger.rs".to_string(),
                10,
            ),
            MemoryEntry::new(
                MemoryKind::Safety,
                "Safety note".to_string(),
                "src/danger.rs".to_string(),
                20,
            ),
            MemoryEntry::new(
                MemoryKind::Todo,
                "Todo item".to_string(),
                "src/other.rs".to_string(),
                5,
            ),
        ];

        let critical = get_critical_files(&entries);
        assert_eq!(critical.len(), 1); // Only danger.rs has high-priority
        assert_eq!(critical[0].0, "src/danger.rs");
        assert_eq!(critical[0].1, 2);
    }
}
