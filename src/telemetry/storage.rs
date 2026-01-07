use std::collections::HashMap;
use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;

use super::{SessionSummary, TelemetryEvent, ToolCall};

pub struct TelemetryStorage {
    dir: PathBuf,
    events_file: PathBuf,
}

impl TelemetryStorage {
    pub fn new(dir: PathBuf) -> Self {
        if let Err(e) = fs::create_dir_all(&dir) {
            eprintln!("[telemetry] Failed to create dir: {}", e);
        }

        let events_file = dir.join("events.jsonl");

        Self { dir, events_file }
    }

    pub fn append(&mut self, event: &TelemetryEvent) -> anyhow::Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.events_file)?;

        let json = serde_json::to_string(event)?;
        writeln!(file, "{}", json)?;

        Ok(())
    }

    pub fn get_session_summary(&self, session_id: &str) -> SessionSummary {
        let mut summary = SessionSummary {
            session_id: session_id.to_string(),
            ..Default::default()
        };

        let file = match File::open(&self.events_file) {
            Ok(f) => f,
            Err(_) => return summary,
        };

        let reader = BufReader::new(file);
        let mut tool_stats: HashMap<String, ToolCall> = HashMap::new();

        for line in reader.lines().map_while(Result::ok) {
            if let Ok(event) = serde_json::from_str::<TelemetryEvent>(&line) {
                if event.session_id == session_id {
                    summary.total_calls += 1;
                    summary.total_tokens_in += event.tokens_in;
                    summary.total_tokens_out += event.tokens_out;
                    summary.total_bytes += event.bytes_out;

                    let stat = tool_stats
                        .entry(event.tool_name.clone())
                        .or_insert_with(|| ToolCall {
                            tool_name: event.tool_name.clone(),
                            call_count: 0,
                            total_tokens_out: 0,
                        });
                    stat.call_count += 1;
                    stat.total_tokens_out += event.tokens_out;
                }
            }
        }

        summary.tools_used = tool_stats.into_values().collect();
        summary
            .tools_used
            .sort_by(|a, b| b.call_count.cmp(&a.call_count));

        summary
    }

    pub fn get_all_sessions_summary(&self) -> Vec<SessionSummary> {
        let file = match File::open(&self.events_file) {
            Ok(f) => f,
            Err(_) => return vec![],
        };

        let reader = BufReader::new(file);
        let mut sessions: HashMap<String, SessionSummary> = HashMap::new();
        let mut tool_stats: HashMap<String, HashMap<String, ToolCall>> = HashMap::new();

        for line in reader.lines().map_while(Result::ok) {
            if let Ok(event) = serde_json::from_str::<TelemetryEvent>(&line) {
                let summary =
                    sessions
                        .entry(event.session_id.clone())
                        .or_insert_with(|| SessionSummary {
                            session_id: event.session_id.clone(),
                            ..Default::default()
                        });

                summary.total_calls += 1;
                summary.total_tokens_in += event.tokens_in;
                summary.total_tokens_out += event.tokens_out;
                summary.total_bytes += event.bytes_out;

                let session_tools = tool_stats.entry(event.session_id.clone()).or_default();

                let stat = session_tools
                    .entry(event.tool_name.clone())
                    .or_insert_with(|| ToolCall {
                        tool_name: event.tool_name.clone(),
                        call_count: 0,
                        total_tokens_out: 0,
                    });
                stat.call_count += 1;
                stat.total_tokens_out += event.tokens_out;
            }
        }

        for (session_id, summary) in sessions.iter_mut() {
            if let Some(tools) = tool_stats.get(session_id) {
                summary.tools_used = tools.values().cloned().collect();
                summary
                    .tools_used
                    .sort_by(|a, b| b.call_count.cmp(&a.call_count));
            }
        }

        sessions.into_values().collect()
    }

    pub fn events_file_path(&self) -> &PathBuf {
        &self.events_file
    }

    pub fn dir(&self) -> &PathBuf {
        &self.dir
    }
}
