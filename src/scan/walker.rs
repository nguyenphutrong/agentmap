use crate::types::{FileEntry, Language};
use anyhow::{Context, Result};
use ignore::WalkBuilder;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;

const BINARY_CHECK_SIZE: usize = 8192;
const MINIFIED_LINE_LENGTH_THRESHOLD: usize = 200;

pub fn scan_directory(
    root: &Path,
    threshold: usize,
    respect_gitignore: bool,
    max_depth: Option<usize>,
) -> Result<Vec<FileEntry>> {
    let mut entries = Vec::new();
    let root = root
        .canonicalize()
        .context("Failed to canonicalize root path")?;

    let mut builder = WalkBuilder::new(&root);
    builder
        .hidden(true)
        .git_ignore(respect_gitignore)
        .git_global(respect_gitignore)
        .git_exclude(respect_gitignore);

    if let Some(depth) = max_depth {
        builder.max_depth(Some(depth));
    }

    let walker = builder.build();

    for result in walker {
        let entry = match result {
            Ok(e) => e,
            Err(err) => {
                eprintln!("Warning: {}", err);
                continue;
            }
        };

        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        let extension = match path.extension().and_then(|e| e.to_str()) {
            Some(ext) => ext,
            None => continue,
        };

        let language = Language::from_extension(extension);
        if matches!(language, Language::Unknown) {
            continue;
        }

        if is_binary_file(path)? {
            continue;
        }

        let (line_count, is_minified) = count_lines_and_check_minified(path)?;
        if is_minified {
            continue;
        }

        let relative_path = path
            .strip_prefix(&root)
            .unwrap_or(path)
            .to_string_lossy()
            .to_string();

        let size_bytes = path.metadata().map(|m| m.len()).unwrap_or(0);

        entries.push(FileEntry::new(
            path.to_path_buf(),
            relative_path,
            size_bytes,
            line_count,
            threshold,
        ));
    }

    entries.sort_by(|a, b| a.relative_path.cmp(&b.relative_path));

    Ok(entries)
}

fn is_binary_file(path: &Path) -> Result<bool> {
    let file = File::open(path).context("Failed to open file for binary check")?;
    let mut reader = BufReader::new(file);
    let mut buffer = [0u8; BINARY_CHECK_SIZE];

    let bytes_read = reader.read(&mut buffer)?;

    Ok(buffer[..bytes_read].contains(&0))
}

fn count_lines_and_check_minified(path: &Path) -> Result<(usize, bool)> {
    let file = File::open(path).context("Failed to open file for line count")?;
    let reader = BufReader::new(file);

    let mut line_count = 0;
    let mut total_chars = 0;
    let mut non_empty_lines = 0;

    for line in reader.lines() {
        let line = line.context("Failed to read line")?;
        line_count += 1;
        let len = line.len();
        if len > 0 {
            total_chars += len;
            non_empty_lines += 1;
        }
    }

    let avg_line_length = if non_empty_lines > 0 {
        total_chars / non_empty_lines
    } else {
        0
    };

    let is_minified = avg_line_length > MINIFIED_LINE_LENGTH_THRESHOLD;

    Ok((line_count, is_minified))
}
