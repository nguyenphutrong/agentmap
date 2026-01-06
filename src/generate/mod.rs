mod agents;
mod memory;
mod outline;

pub use agents::{detect_entry_points, generate_agents_md};
pub use memory::{generate_memory, get_critical_files};
pub use outline::generate_outline;
