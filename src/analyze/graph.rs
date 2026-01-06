use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Default)]
pub struct FileGraph {
    pub imports: HashMap<String, Vec<String>>,
    pub importers: HashMap<String, Vec<String>>,
}

impl FileGraph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_file(&mut self, file_path: &str, imports: Vec<String>) {
        self.imports.insert(file_path.to_string(), imports.clone());

        for import in imports {
            self.importers
                .entry(import)
                .or_default()
                .push(file_path.to_string());
        }
    }

    pub fn resolve_import(&self, from_file: &str, import_path: &str) -> Option<String> {
        let from_dir = Path::new(from_file).parent()?;

        if import_path.starts_with("./") || import_path.starts_with("../") {
            let resolved = from_dir.join(import_path);
            return Some(normalize_path(&resolved.to_string_lossy()));
        }

        if !import_path.contains('/') && !import_path.contains('.') {
            let potential = from_dir.join(import_path);
            return Some(normalize_path(&potential.to_string_lossy()));
        }

        Some(import_path.to_string())
    }

    pub fn hub_files(&self) -> Vec<(String, usize)> {
        let mut hubs: Vec<(String, usize)> = self
            .importers
            .iter()
            .filter(|(_, importers)| importers.len() >= 3)
            .map(|(file, importers)| (file.clone(), importers.len()))
            .collect();

        hubs.sort_by(|a, b| b.1.cmp(&a.1));
        hubs
    }

    pub fn is_hub(&self, file: &str) -> bool {
        self.importers
            .get(file)
            .map(|i| i.len() >= 3)
            .unwrap_or(false)
    }
}

fn normalize_path(path: &str) -> String {
    path.replace("\\", "/")
        .replace("//", "/")
        .trim_start_matches("./")
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hub_detection() {
        let mut graph = FileGraph::new();

        graph.add_file("a.rs", vec!["utils".to_string()]);
        graph.add_file("b.rs", vec!["utils".to_string()]);
        graph.add_file("c.rs", vec!["utils".to_string()]);
        graph.add_file("d.rs", vec!["utils".to_string(), "config".to_string()]);

        let hubs = graph.hub_files();
        assert_eq!(hubs.len(), 1);
        assert_eq!(hubs[0].0, "utils");
        assert_eq!(hubs[0].1, 4);
    }

    #[test]
    fn test_not_hub_with_few_importers() {
        let mut graph = FileGraph::new();

        graph.add_file("a.rs", vec!["utils".to_string()]);
        graph.add_file("b.rs", vec!["utils".to_string()]);

        let hubs = graph.hub_files();
        assert!(hubs.is_empty());
    }
}
