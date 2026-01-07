use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryEvent {
    pub timestamp: DateTime<Utc>,
    pub session_id: String,
    pub tool_name: String,
    pub params_preview: String,
    pub tokens_in: usize,
    pub tokens_out: usize,
    pub bytes_out: usize,
    pub duration_ms: u64,
}

impl TelemetryEvent {
    pub fn new(
        session_id: &str,
        tool_name: &str,
        params: &str,
        tokens_in: usize,
        tokens_out: usize,
        bytes_out: usize,
        duration_ms: u64,
    ) -> Self {
        let params_preview = if params.len() > 100 {
            format!("{}...", &params[..100])
        } else {
            params.to_string()
        };

        Self {
            timestamp: Utc::now(),
            session_id: session_id.to_string(),
            tool_name: tool_name.to_string(),
            params_preview,
            tokens_in,
            tokens_out,
            bytes_out,
            duration_ms,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ToolCall {
    pub tool_name: String,
    pub call_count: usize,
    pub total_tokens_out: usize,
}
