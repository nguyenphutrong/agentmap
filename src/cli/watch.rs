use anyhow::{Context, Result};
use notify::RecursiveMode;
use notify_debouncer_mini::{new_debouncer, DebouncedEventKind};
use std::path::Path;
use std::sync::mpsc::channel;
use std::time::Duration;

use crate::cli::Args;

pub fn run_watch(args: &Args, debounce_ms: u64) -> Result<()> {
    let work_path = args.path.canonicalize().context("Failed to resolve path")?;

    eprintln!("Watching: {}", work_path.display());
    eprintln!("Press Ctrl+C to stop\n");

    crate::run_analysis_for_watch(args, &work_path)?;

    let (tx, rx) = channel();

    let debounce_duration = Duration::from_millis(debounce_ms);
    let mut debouncer =
        new_debouncer(debounce_duration, tx).context("Failed to create file watcher")?;

    debouncer
        .watcher()
        .watch(&work_path, RecursiveMode::Recursive)
        .context("Failed to start watching directory")?;

    let output_path = if args.output.is_absolute() {
        args.output.clone()
    } else {
        work_path.join(&args.output)
    };

    loop {
        match rx.recv() {
            Ok(Ok(events)) => {
                let relevant_events: Vec<_> = events
                    .iter()
                    .filter(|e| e.kind == DebouncedEventKind::Any)
                    .filter(|e| !is_output_path(&e.path, &output_path))
                    .filter(|e| !is_hidden_or_git(&e.path))
                    .collect();

                if relevant_events.is_empty() {
                    continue;
                }

                if args.verbosity() > 1 {
                    for event in &relevant_events {
                        eprintln!("  Changed: {}", event.path.display());
                    }
                }

                eprintln!("\n[{}] Changes detected, regenerating...", timestamp());

                match crate::run_analysis_for_watch(args, &work_path) {
                    Ok(()) => eprintln!("[{}] Done\n", timestamp()),
                    Err(e) => eprintln!("[{}] Error: {}\n", timestamp(), e),
                }
            }
            Ok(Err(error)) => {
                eprintln!("Watch error: {:?}", error);
            }
            Err(e) => {
                eprintln!("Channel error: {}", e);
                break;
            }
        }
    }

    Ok(())
}

fn is_output_path(path: &Path, output_path: &Path) -> bool {
    path.starts_with(output_path)
}

fn is_hidden_or_git(path: &Path) -> bool {
    path.components().any(|c| {
        c.as_os_str()
            .to_str()
            .map(|s| s.starts_with('.'))
            .unwrap_or(false)
    })
}

fn timestamp() -> String {
    chrono::Local::now().format("%H:%M:%S").to_string()
}
