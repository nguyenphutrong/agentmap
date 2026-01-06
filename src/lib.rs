pub mod analyze;
pub mod cli;
pub mod emit;
pub mod generate;
pub mod runner;
pub mod scan;
pub mod types;

pub use cli::Args;
pub use runner::run_analysis as run_analysis_for_watch;
pub use types::{FileEntry, Language, MemoryEntry, MemoryKind, Priority, Symbol, SymbolKind};
