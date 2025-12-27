//! Dependency graph for smart change detection
//!
//! This crate builds and maintains an import graph of Python files
//! to identify which tests are affected by file changes.

pub use dx_py_core::{GraphError, TestId};

use dx_py_core::TestCase;
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::Direction;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};
use tree_sitter::{Node, Parser};

/// Magic bytes for graph cache files
const GRAPH_MAGIC: &[u8; 4] = b"DXGR";
const GRAPH_VERSION: u16 = 1;

/// A node in the import graph representing a Python file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileNode {
    pub path: PathBuf,
    pub content_hash: [u8; 32],
    pub tests: Vec<TestId>,
}

/// Import graph for tracking file dependencies
#[derive(Debug)]
pub struct ImportGraph {
    graph: DiGraph<PathBuf, ()>,
    path_to_node: HashMap<PathBuf, NodeIndex>,
    file_hashes: HashMap<PathBuf, [u8; 32]>,
    file_tests: HashMap<PathBuf, Vec<TestId>>,
}

impl ImportGraph {
    /// Create a new empty import graph
    pub fn new() -> Self {
        Self {
            graph: DiGraph::new(),
            path_to_node: HashMap::new(),
            file_hashes: HashMap::new(),
            file_tests: HashMap::new(),
        }
    }

    /// Add a file to the graph
    pub fn add_file(&mut self, path: &Path) -> NodeIndex {
        if let Some(&idx) = self.path_to_node.get(path) {
            return idx;
        }

        let idx = self.graph.add_node(path.to_owned());
        self.path_to_node.insert(path.to_owned(), idx);
        idx
    }

    /// Add an import edge from importer to imported
    pub fn add_import(&mut self, importer: &Path, imported: &Path) {
        let from_idx = self.add_file(importer);
        let to_idx = self.add_file(imported);
        
        // Check if edge already exists
        if !self.graph.contains_edge(from_idx, to_idx) {
            self.graph.add_edge(from_idx, to_idx, ());
        }
    }

    /// Associate tests with a file
    pub fn set_file_tests(&mut self, path: &Path, tests: Vec<TestId>) {
        self.file_tests.insert(path.to_owned(), tests);
    }

    /// Set the content hash for a file
    pub fn set_file_hash(&mut self, path: &Path, hash: [u8; 32]) {
        self.file_hashes.insert(path.to_owned(), hash);
    }

    /// Get all files that depend on the given file (transitively)
    pub fn get_dependents(&self, path: &Path) -> HashSet<PathBuf> {
        let mut dependents = HashSet::new();

        if let Some(&start_idx) = self.path_to_node.get(path) {
            // Use BFS to find all nodes that can reach this node
            // We need to traverse incoming edges
            let mut visited = HashSet::new();
            let mut stack = vec![start_idx];

            while let Some(current) = stack.pop() {
                if visited.contains(&current) {
                    continue;
                }
                visited.insert(current);

                // Get all nodes that import this one (incoming edges)
                for neighbor in self.graph.neighbors_directed(current, Direction::Incoming) {
                    if !visited.contains(&neighbor) {
                        stack.push(neighbor);
                        if let Some(path) = self.graph.node_weight(neighbor) {
                            dependents.insert(path.clone());
                        }
                    }
                }
            }
        }

        dependents
    }

    /// Get all tests affected by changes to the given file
    pub fn get_affected_tests(&self, changed_file: &Path) -> Vec<TestId> {
        let mut affected = Vec::new();

        // Include tests from the changed file itself
        if let Some(tests) = self.file_tests.get(changed_file) {
            affected.extend(tests.iter().cloned());
        }

        // Include tests from all dependent files
        for dependent in self.get_dependents(changed_file) {
            if let Some(tests) = self.file_tests.get(&dependent) {
                affected.extend(tests.iter().cloned());
            }
        }

        affected
    }

    /// Get the number of files in the graph
    pub fn file_count(&self) -> usize {
        self.graph.node_count()
    }

    /// Get the number of import edges in the graph
    pub fn edge_count(&self) -> usize {
        self.graph.edge_count()
    }

    /// Check if a file needs re-scanning based on content hash
    pub fn needs_rescan(&self, path: &Path, current_hash: &[u8; 32]) -> bool {
        match self.file_hashes.get(path) {
            None => true,
            Some(stored_hash) => stored_hash != current_hash,
        }
    }

    /// Save the graph to a binary cache file
    pub fn save(&self, path: &Path) -> Result<(), GraphError> {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);

        // Write magic and version
        writer.write_all(GRAPH_MAGIC)?;
        writer.write_all(&GRAPH_VERSION.to_le_bytes())?;

        // Serialize graph data
        let data = GraphData {
            nodes: self.graph.node_weights().cloned().collect(),
            edges: self.graph.edge_indices()
                .filter_map(|e| {
                    let (a, b) = self.graph.edge_endpoints(e)?;
                    Some((a.index(), b.index()))
                })
                .collect(),
            file_hashes: self.file_hashes.clone(),
            file_tests: self.file_tests.clone(),
        };

        bincode::serialize_into(&mut writer, &data)
            .map_err(|e| GraphError::CacheCorrupted(e.to_string()))?;

        writer.flush()?;
        Ok(())
    }

    /// Load the graph from a binary cache file
    pub fn load(path: &Path) -> Result<Self, GraphError> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);

        // Read and verify magic
        let mut magic = [0u8; 4];
        reader.read_exact(&mut magic)?;
        if &magic != GRAPH_MAGIC {
            return Err(GraphError::CacheCorrupted("Invalid magic bytes".into()));
        }

        // Read version
        let mut version_bytes = [0u8; 2];
        reader.read_exact(&mut version_bytes)?;
        let version = u16::from_le_bytes(version_bytes);
        if version != GRAPH_VERSION {
            return Err(GraphError::CacheCorrupted(format!(
                "Unsupported version: {}",
                version
            )));
        }

        // Deserialize graph data
        let data: GraphData = bincode::deserialize_from(reader)
            .map_err(|e| GraphError::CacheCorrupted(e.to_string()))?;

        // Rebuild graph
        let mut graph = DiGraph::new();
        let mut path_to_node = HashMap::new();

        for node_path in &data.nodes {
            let idx = graph.add_node(node_path.clone());
            path_to_node.insert(node_path.clone(), idx);
        }

        for (from, to) in &data.edges {
            if *from < data.nodes.len() && *to < data.nodes.len() {
                let from_idx = NodeIndex::new(*from);
                let to_idx = NodeIndex::new(*to);
                graph.add_edge(from_idx, to_idx, ());
            }
        }

        Ok(Self {
            graph,
            path_to_node,
            file_hashes: data.file_hashes,
            file_tests: data.file_tests,
        })
    }
}

impl Default for ImportGraph {
    fn default() -> Self {
        Self::new()
    }
}

/// Serializable graph data
#[derive(Serialize, Deserialize)]
struct GraphData {
    nodes: Vec<PathBuf>,
    edges: Vec<(usize, usize)>,
    file_hashes: HashMap<PathBuf, [u8; 32]>,
    file_tests: HashMap<PathBuf, Vec<TestId>>,
}

/// Parser for extracting imports from Python files
pub struct ImportExtractor {
    parser: Parser,
}

impl ImportExtractor {
    /// Create a new import extractor
    pub fn new() -> Result<Self, GraphError> {
        let mut parser = Parser::new();
        let language = tree_sitter_python::language();
        parser
            .set_language(language)
            .map_err(|e| GraphError::ParseError(e.to_string()))?;
        Ok(Self { parser })
    }

    /// Extract imports from Python source code
    pub fn extract_imports(&mut self, source: &str) -> Result<Vec<ImportInfo>, GraphError> {
        let tree = self.parser
            .parse(source, None)
            .ok_or_else(|| GraphError::ParseError("Failed to parse source".into()))?;

        let mut imports = Vec::new();
        self.walk_tree(tree.root_node(), source.as_bytes(), &mut imports);
        Ok(imports)
    }

    fn walk_tree(&self, node: Node, source: &[u8], imports: &mut Vec<ImportInfo>) {
        match node.kind() {
            "import_statement" => {
                if let Some(import) = self.parse_import(node, source) {
                    imports.push(import);
                }
            }
            "import_from_statement" => {
                if let Some(import) = self.parse_import_from(node, source) {
                    imports.push(import);
                }
            }
            _ => {
                for child in node.children(&mut node.walk()) {
                    self.walk_tree(child, source, imports);
                }
            }
        }
    }

    fn parse_import(&self, node: Node, source: &[u8]) -> Option<ImportInfo> {
        for child in node.children(&mut node.walk()) {
            if child.kind() == "dotted_name" {
                let module = self.node_text(child, source);
                return Some(ImportInfo {
                    module,
                    is_relative: false,
                    level: 0,
                });
            }
        }
        None
    }

    fn parse_import_from(&self, node: Node, source: &[u8]) -> Option<ImportInfo> {
        let mut module = String::new();
        let mut level = 0;
        let mut found_module = false;

        for child in node.children(&mut node.walk()) {
            match child.kind() {
                "import_prefix" => {
                    // Count dots for relative imports
                    let text = self.node_text(child, source);
                    level = text.chars().filter(|c| *c == '.').count();
                }
                "dotted_name" => {
                    if !found_module {
                        module = self.node_text(child, source);
                        found_module = true;
                    }
                }
                "relative_import" => {
                    // Handle relative imports
                    for subchild in child.children(&mut child.walk()) {
                        match subchild.kind() {
                            "import_prefix" => {
                                let text = self.node_text(subchild, source);
                                level = text.chars().filter(|c| *c == '.').count();
                            }
                            "dotted_name" => {
                                module = self.node_text(subchild, source);
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }

        Some(ImportInfo {
            module,
            is_relative: level > 0,
            level,
        })
    }

    fn node_text(&self, node: Node, source: &[u8]) -> String {
        let start = node.start_byte();
        let end = node.end_byte();
        String::from_utf8_lossy(&source[start..end]).to_string()
    }
}

impl Default for ImportExtractor {
    fn default() -> Self {
        Self::new().expect("Failed to create ImportExtractor")
    }
}

/// Information about an import statement
#[derive(Debug, Clone)]
pub struct ImportInfo {
    pub module: String,
    pub is_relative: bool,
    pub level: usize,
}

impl ImportInfo {
    /// Resolve a relative import to an absolute module path
    pub fn resolve(&self, current_file: &Path) -> Option<PathBuf> {
        if self.is_relative {
            let mut base = current_file.parent()?.to_owned();
            for _ in 1..self.level {
                base = base.parent()?.to_owned();
            }
            if self.module.is_empty() {
                Some(base.join("__init__.py"))
            } else {
                let module_path = self.module.replace('.', "/");
                Some(base.join(format!("{}.py", module_path)))
            }
        } else {
            let module_path = self.module.replace('.', "/");
            Some(PathBuf::from(format!("{}.py", module_path)))
        }
    }
}

/// Builder for constructing an ImportGraph from a directory
pub struct ImportGraphBuilder {
    graph: ImportGraph,
    extractor: ImportExtractor,
    root: PathBuf,
}

impl ImportGraphBuilder {
    pub fn new(root: impl Into<PathBuf>) -> Result<Self, GraphError> {
        Ok(Self {
            graph: ImportGraph::new(),
            extractor: ImportExtractor::new()?,
            root: root.into(),
        })
    }

    /// Add a file and its imports to the graph
    pub fn add_file(&mut self, path: &Path, tests: Vec<TestCase>) -> Result<(), GraphError> {
        let content = std::fs::read(path)?;
        let hash = blake3::hash(&content);
        let source = String::from_utf8_lossy(&content);

        self.graph.add_file(path);
        self.graph.set_file_hash(path, *hash.as_bytes());
        self.graph.set_file_tests(path, tests.iter().map(|t| t.id).collect());

        let imports = self.extractor.extract_imports(&source)?;
        for import in imports {
            if let Some(imported_path) = import.resolve(path) {
                // Try to find the actual file
                let full_path = self.root.join(&imported_path);
                if full_path.exists() {
                    self.graph.add_import(path, &full_path);
                }
            }
        }

        Ok(())
    }

    /// Build the final graph
    pub fn build(self) -> ImportGraph {
        self.graph
    }
}

#[cfg(test)]
mod tests;

