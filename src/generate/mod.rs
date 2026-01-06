mod agents;
mod file_doc;
mod imports;
mod index;
mod memory;
mod module_content;
mod outline;

pub use agents::{detect_entry_points, generate_agents_md, AgentsConfig};
pub use file_doc::{
    file_path_to_slug, generate_file_doc, is_complex_file, DEFAULT_COMPLEX_LINES_THRESHOLD,
    DEFAULT_COMPLEX_SYMBOLS_THRESHOLD,
};
pub use imports::generate_imports;
pub use index::{generate_index_md, IndexConfig};
pub use memory::{generate_memory, get_critical_files};
pub use module_content::generate_module_content;
pub use outline::generate_outline;
