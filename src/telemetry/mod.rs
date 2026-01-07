mod counter;
mod event;
mod storage;

pub use counter::TokenCounter;
pub use event::{TelemetryEvent, ToolCall};
pub use storage::TelemetryStorage;

use std::path::Path;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct Telemetry {
    storage: Arc<RwLock<TelemetryStorage>>,
    counter: TokenCounter,
    session_id: String,
}

impl Telemetry {
    pub fn new(output_path: &Path) -> Self {
        let session_id = uuid::Uuid::new_v4().to_string();
        let storage = TelemetryStorage::new(output_path.join("telemetry"));

        Self {
            storage: Arc::new(RwLock::new(storage)),
            counter: TokenCounter::new(),
            session_id,
        }
    }

    pub async fn record_tool_call(
        &self,
        tool_name: &str,
        params: &str,
        response: &str,
        duration_ms: u64,
    ) {
        let tokens_in = self.counter.count(params);
        let tokens_out = self.counter.count(response);

        let event = TelemetryEvent::new(
            &self.session_id,
            tool_name,
            params,
            tokens_in,
            tokens_out,
            response.len(),
            duration_ms,
        );

        let mut storage = self.storage.write().await;
        if let Err(e) = storage.append(&event) {
            eprintln!("[telemetry] Failed to write event: {}", e);
        }
    }

    pub async fn get_session_summary(&self) -> SessionSummary {
        let storage = self.storage.read().await;
        storage.get_session_summary(&self.session_id)
    }

    pub fn session_id(&self) -> &str {
        &self.session_id
    }
}

#[derive(Debug, Clone, Default)]
pub struct SessionSummary {
    pub session_id: String,
    pub total_calls: usize,
    pub total_tokens_in: usize,
    pub total_tokens_out: usize,
    pub total_bytes: usize,
    pub tools_used: Vec<ToolCall>,
}
