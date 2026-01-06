use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::scan::DiffStat;
use crate::types::{FileEntry, MemoryEntry, Symbol};

#[derive(Serialize)]
pub struct JsonOutput {
    pub version: String,
    pub generated_at: DateTime<Utc>,
    pub project: ProjectInfo,
    pub files: Vec<FileEntry>,
    pub large_files: Vec<LargeFileEntry>,
    pub memory: Vec<MemoryEntry>,
    pub entry_points: Vec<String>,
    pub critical_files: Vec<CriticalFile>,
    pub hub_files: Vec<HubFile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diff: Option<DiffInfo>,
}

#[derive(Serialize)]
pub struct ProjectInfo {
    pub path: String,
    pub files_scanned: usize,
    pub large_files_count: usize,
    pub memory_markers_count: usize,
}

#[derive(Serialize)]
pub struct LargeFileEntry {
    pub path: String,
    pub line_count: usize,
    pub language: String,
    pub symbols: Vec<Symbol>,
}

#[derive(Serialize)]
pub struct CriticalFile {
    pub path: String,
    pub high_priority_markers: usize,
}

#[derive(Serialize)]
pub struct HubFile {
    pub path: String,
    pub imported_by: usize,
}

#[derive(Serialize)]
pub struct DiffInfo {
    pub base_ref: String,
    pub files: Vec<DiffStat>,
}

impl JsonOutput {
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_else(|_| "{}".to_string())
    }
}
