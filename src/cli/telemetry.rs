use anyhow::Result;
use std::path::Path;

use crate::telemetry::TokenCounter;

pub fn run_telemetry_summary(output_path: &Path) -> Result<()> {
    let storage = crate::telemetry::TelemetryStorage::new(output_path.join("telemetry"));

    let sessions = storage.get_all_sessions_summary();

    if sessions.is_empty() {
        eprintln!("No telemetry data found.");
        eprintln!("Telemetry is collected when using MCP server tools.");
        eprintln!("\nTo analyze token costs of existing docs, use:");
        eprintln!("  agentlens telemetry module <SLUG>");
        return Ok(());
    }

    println!("\n📊 AgentLens Telemetry Summary");
    println!("═══════════════════════════════════════════════════");
    println!("Sessions: {}", sessions.len());

    let total_tokens_in: usize = sessions.iter().map(|s| s.total_tokens_in).sum();
    let total_tokens_out: usize = sessions.iter().map(|s| s.total_tokens_out).sum();
    let total_calls: usize = sessions.iter().map(|s| s.total_calls).sum();

    println!("\nToken Usage:");
    println!("  Total tokens served: {}", total_tokens_out);
    println!("  Total tokens in requests: {}", total_tokens_in);
    println!("  Total tool calls: {}", total_calls);

    if !sessions.is_empty() {
        println!(
            "  Avg tokens/session: {}",
            total_tokens_out / sessions.len()
        );
    }

    Ok(())
}

pub fn run_telemetry_module(output_path: &Path, slug: &str) -> Result<()> {
    let counter = TokenCounter::new();
    let module_dir = output_path.join("modules").join(slug);

    if !module_dir.exists() {
        anyhow::bail!("Module '{}' not found at {}", slug, module_dir.display());
    }

    println!("\n📊 Token Analysis: {}", slug);
    println!("═══════════════════════════════════════════════════");

    let mut total_tokens = 0;
    let mut total_bytes = 0;
    let mut file_stats = Vec::new();

    for file in &["MODULE.md", "outline.md", "memory.md", "imports.md"] {
        let file_path = module_dir.join(file);
        if file_path.exists() {
            if let Ok(content) = std::fs::read_to_string(&file_path) {
                let tokens = counter.count(&content);
                let bytes = content.len();
                total_tokens += tokens;
                total_bytes += bytes;
                file_stats.push((file.to_string(), tokens, bytes));
            }
        }
    }

    println!("\n| File | Tokens | Bytes |");
    println!("|------|--------|-------|");
    for (file, tokens, bytes) in &file_stats {
        println!("| {} | {} | {} |", file, tokens, bytes);
    }
    println!("|------|--------|-------|");
    println!("| **TOTAL** | **{}** | **{}** |", total_tokens, total_bytes);

    println!("\n📈 Estimated Cost (GPT-5.1-codex-mini):");
    let cost_per_million = 0.25;
    let cost = (total_tokens as f64 / 1_000_000.0) * cost_per_million;
    println!("  ${:.6} per module read", cost);

    Ok(())
}

pub fn run_telemetry_all_modules(output_path: &Path) -> Result<()> {
    let counter = TokenCounter::new();
    let modules_dir = output_path.join("modules");

    if !modules_dir.exists() {
        anyhow::bail!("No modules found at {}", modules_dir.display());
    }

    println!("\n📊 Token Analysis: All Modules");
    println!("═══════════════════════════════════════════════════");

    let mut module_stats = Vec::new();
    let mut grand_total_tokens = 0;
    let mut grand_total_bytes = 0;

    let entries: Vec<_> = std::fs::read_dir(&modules_dir)?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_dir())
        .collect();

    for entry in entries {
        let slug = entry.file_name().to_string_lossy().to_string();
        let module_dir = entry.path();

        let mut module_tokens = 0;
        let mut module_bytes = 0;

        for file in &["MODULE.md", "outline.md", "memory.md", "imports.md"] {
            let file_path = module_dir.join(file);
            if file_path.exists() {
                if let Ok(content) = std::fs::read_to_string(&file_path) {
                    module_tokens += counter.count(&content);
                    module_bytes += content.len();
                }
            }
        }

        grand_total_tokens += module_tokens;
        grand_total_bytes += module_bytes;
        module_stats.push((slug, module_tokens, module_bytes));
    }

    module_stats.sort_by(|a, b| b.1.cmp(&a.1));

    println!("\n| Module | Tokens | Bytes |");
    println!("|--------|--------|-------|");
    for (slug, tokens, bytes) in &module_stats {
        println!("| {} | {} | {} |", slug, tokens, bytes);
    }
    println!("|--------|--------|-------|");
    println!(
        "| **TOTAL ({} modules)** | **{}** | **{}** |",
        module_stats.len(),
        grand_total_tokens,
        grand_total_bytes
    );

    let index_path = output_path.join("INDEX.md");
    let index_tokens = if index_path.exists() {
        std::fs::read_to_string(&index_path)
            .map(|c| counter.count(&c))
            .unwrap_or(0)
    } else {
        0
    };

    println!("\n📋 INDEX.md tokens: {}", index_tokens);
    println!(
        "📋 Grand total (INDEX + all modules): {}",
        grand_total_tokens + index_tokens
    );

    println!("\n📈 Estimated Cost (GPT-5.1-codex-mini @ $0.25/1M tokens):");
    let full_read_tokens = grand_total_tokens + index_tokens;
    let full_cost = (full_read_tokens as f64 / 1_000_000.0) * 0.25;
    let hierarchical_cost = (index_tokens as f64 / 1_000_000.0) * 0.25
        + (module_stats.first().map(|m| m.1).unwrap_or(0) as f64 / 1_000_000.0) * 0.25;

    println!("  Full codebase read: ${:.6}", full_cost);
    println!(
        "  Hierarchical (INDEX + 1 module): ${:.6}",
        hierarchical_cost
    );

    if full_cost > 0.0 {
        let savings = ((full_cost - hierarchical_cost) / full_cost) * 100.0;
        println!("  Savings: {:.1}%", savings);
    }

    Ok(())
}
