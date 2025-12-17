//! DX Bundle Graph - O(1) Module Graph Cache
//!
//! This crate implements the O(1) module graph cache that avoids
//! re-parsing and re-resolving unchanged dependency graphs.

use dashmap::DashMap;
use dx_bundle_core::*;
use memmap2::Mmap;
use std::collections::{HashMap, VecDeque};
use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// Module graph cache manager
pub struct ModuleGraphCache {
    /// Cache directory for storing graphs
    cache_dir: PathBuf,
    /// In-memory cached graphs
    graphs: DashMap<u128, Arc<CachedGraph>>,
}

/// A cached module graph
pub struct CachedGraph {
    mmap: Mmap,
    _hash: u128,
}

impl ModuleGraphCache {
    /// Create a new module graph cache
    pub fn new(cache_dir: &Path) -> io::Result<Self> {
        std::fs::create_dir_all(cache_dir)?;
        Ok(Self {
            cache_dir: cache_dir.to_path_buf(),
            graphs: DashMap::new(),
        })
    }

    /// Compute project hash from all source files
    pub fn compute_hash(project_root: &Path, entries: &[PathBuf]) -> u128 {
        let mut hasher = blake3::Hasher::new();

        // Hash entry points first
        for entry in entries {
            hasher.update(entry.to_string_lossy().as_bytes());
        }

        // Walk source files
        for entry in walkdir::WalkDir::new(project_root)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| Self::is_source_file(e.path()))
        {
            // Hash path
            hasher.update(entry.path().to_string_lossy().as_bytes());

            // Hash mtime (not content - too slow for large projects)
            if let Ok(meta) = entry.metadata()
                && let Ok(mtime) = meta.modified()
            {
                let mtime_secs =
                    mtime.duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs();
                hasher.update(&mtime_secs.to_le_bytes());
            }
        }

        // Also hash configuration files
        for config in &["package.json", "tsconfig.json", "dx.toml"] {
            if let Ok(content) = std::fs::read(project_root.join(config)) {
                hasher.update(&content);
            }
        }

        let hash = hasher.finalize();
        u128::from_le_bytes(hash.as_bytes()[..16].try_into().unwrap())
    }

    /// Get cached graph or build new one
    pub fn get_or_build(
        &self,
        project_root: &Path,
        entries: &[PathBuf],
    ) -> io::Result<Arc<CachedGraph>> {
        let hash = Self::compute_hash(project_root, entries);

        // Check in-memory cache
        if let Some(cached) = self.graphs.get(&hash) {
            return Ok(Arc::clone(&cached));
        }

        // Check disk cache
        let graph_path = self.cache_dir.join(format!("{:032x}.dxmg", hash));
        if graph_path.exists() {
            let file = std::fs::File::open(&graph_path)?;
            let mmap = unsafe { Mmap::map(&file)? };

            // Validate magic
            if mmap.len() >= 4 && mmap[0..4] == magic::MODULE_GRAPH {
                let cached = Arc::new(CachedGraph { mmap, _hash: hash });
                self.graphs.insert(hash, Arc::clone(&cached));
                return Ok(cached);
            }
        }

        // Build new graph
        let graph_data = self.build_graph(project_root, entries)?;

        // Write to disk
        std::fs::write(&graph_path, &graph_data)?;

        // Memory-map
        let file = std::fs::File::open(&graph_path)?;
        let mmap = unsafe { Mmap::map(&file)? };
        let cached = Arc::new(CachedGraph { mmap, _hash: hash });

        self.graphs.insert(hash, Arc::clone(&cached));
        Ok(cached)
    }

    /// Build module graph from scratch
    fn build_graph(&self, project_root: &Path, entries: &[PathBuf]) -> io::Result<Vec<u8>> {
        let mut builder = GraphBuilder::new();

        // Use work-stealing parallelism
        let (tx, rx) = crossbeam_channel::unbounded();

        // Queue entry points
        for entry in entries {
            tx.send(entry.clone()).ok();
        }

        // Process in parallel
        let processed = DashMap::new();
        let module_map: DashMap<u64, (PathBuf, Vec<String>)> = DashMap::new();

        rayon::scope(|s| {
            for _ in 0..num_cpus::get() {
                let tx = tx.clone();
                let rx = rx.clone();
                let processed = &processed;
                let module_map = &module_map;

                s.spawn(move |_| {
                    while let Ok(path) = rx.recv_timeout(std::time::Duration::from_millis(10)) {
                        let abs_path = if path.is_absolute() {
                            path.clone()
                        } else {
                            project_root.join(&path)
                        };

                        // Skip if already processed
                        let path_hash =
                            xxhash_rust::xxh64::xxh64(abs_path.to_string_lossy().as_bytes(), 0);
                        if processed.contains_key(&path_hash) {
                            continue;
                        }
                        processed.insert(path_hash, ());

                        // Parse and extract imports
                        if let Ok(source) = std::fs::read_to_string(&abs_path) {
                            let imports = Self::extract_imports(&source, &abs_path);

                            // Store module data
                            module_map.insert(path_hash, (abs_path.clone(), imports.clone()));

                            // Queue imports for processing
                            for import in imports {
                                if let Some(resolved_path) =
                                    Self::resolve_import(&import, &abs_path, project_root)
                                {
                                    tx.send(resolved_path).ok();
                                }
                            }
                        }
                    }
                });
            }
        });

        // Build binary graph from processed modules
        // Create module ID mapping
        let mut path_to_id: HashMap<PathBuf, u32> = HashMap::new();

        for (id, entry) in module_map.iter().enumerate() {
            let (path, _) = entry.value();
            path_to_id.insert(path.clone(), id as u32);
        }

        // Add modules to builder
        for entry in module_map.iter() {
            let (path, imports) = entry.value();
            let module_id = path_to_id[path];

            // Resolve import edges
            let mut resolved_imports: Vec<u32> = Vec::new();
            for import_spec in imports {
                if let Some(import_path) = Self::resolve_import(import_spec, path, project_root)
                    && let Some(&import_id) = path_to_id.get(&import_path)
                {
                    resolved_imports.push(import_id);
                }
            }

            // Add module entry
            let path_str = path.to_string_lossy();
            let path_hash = *entry.key();
            let string_offset = builder.strings.len() as u32;
            builder.strings.extend_from_slice(path_str.as_bytes());
            builder.strings.push(0); // Null terminator

            builder.modules.push(ModuleEntry {
                id: path_hash as u128, // Use path hash as module ID
                path_hash,
                path_offset: string_offset,
                path_len: path_str.len() as u16,
                kind: ModuleKind::ESM as u8,
                ast_offset: 0,
                ast_size: 0,
                first_import: builder.edges.len() as u32,
                import_count: resolved_imports.len() as u16,
                first_export: 0,
                export_count: 0,
                has_side_effects: 1,
                tree_shakeable: 0,
                source_mtime: 0,
            });

            // Add edges
            for &import_id in &resolved_imports {
                builder.edges.push(ImportEdge {
                    from_module: module_id,
                    to_module: import_id,
                    kind: ImportKind::Named as u8,
                    is_dynamic: 0,
                    specifier_offset: 0,
                    specifier_len: 0,
                });
            }
        }

        builder.build()
    }

    /// Fast import extraction using OXC
    fn extract_imports(source: &str, path: &Path) -> Vec<String> {
        use oxc_ast::ast::Statement;
        use oxc_parser::Parser;
        use oxc_span::SourceType;

        let source_type = match path.extension().and_then(|e| e.to_str()) {
            Some("ts") => SourceType::ts(),
            Some("tsx") => SourceType::tsx(),
            Some("jsx") => SourceType::jsx(),
            Some("mjs") => SourceType::mjs(),
            Some("cjs") => SourceType::cjs(),
            _ => SourceType::mjs(),
        };

        let allocator = oxc_allocator::Allocator::default();
        let ret = Parser::new(&allocator, source, source_type).parse();

        let mut imports = Vec::new();

        // Extract imports from AST
        for stmt in &ret.program.body {
            match stmt {
                Statement::ImportDeclaration(decl) => {
                    imports.push(decl.source.value.to_string());
                }
                Statement::ExportNamedDeclaration(decl) => {
                    if let Some(source) = &decl.source {
                        imports.push(source.value.to_string());
                    }
                }
                Statement::ExportAllDeclaration(decl) => {
                    imports.push(decl.source.value.to_string());
                }
                _ => {}
            }
        }

        imports
    }

    /// Simple import resolution (Node.js algorithm)
    fn resolve_import(specifier: &str, from: &Path, project_root: &Path) -> Option<PathBuf> {
        // Relative import
        if specifier.starts_with('.') {
            let base = from.parent()?;
            let resolved = base.join(specifier);

            // Try exact path
            if resolved.exists() {
                return Some(resolved);
            }

            // Try with extensions
            for ext in &[".js", ".ts", ".tsx", ".jsx"] {
                let with_ext = resolved.with_extension(&ext[1..]);
                if with_ext.exists() {
                    return Some(with_ext);
                }
            }

            // Try as directory with index
            for ext in &[".js", ".ts", ".tsx", ".jsx"] {
                let index = resolved.join(format!("index{}", ext));
                if index.exists() {
                    return Some(index);
                }
            }
        }
        // Absolute or node_modules import
        else {
            // Try node_modules
            let mut current = from.parent()?;
            loop {
                let node_modules = current.join("node_modules").join(specifier);
                if node_modules.exists() {
                    return Some(node_modules);
                }

                current = current.parent()?;
                if current == project_root {
                    break;
                }
            }
        }

        None
    }

    fn is_source_file(path: &Path) -> bool {
        matches!(
            path.extension().and_then(|e| e.to_str()),
            Some("js" | "jsx" | "ts" | "tsx" | "mjs" | "cjs")
        )
    }
}

impl CachedGraph {
    /// Get header
    #[inline(always)]
    pub fn header(&self) -> &ModuleGraphHeader {
        unsafe { &*(self.mmap.as_ptr() as *const ModuleGraphHeader) }
    }

    /// Get raw memory map (for string table access)
    #[inline(always)]
    pub fn mmap(&self) -> &Mmap {
        &self.mmap
    }

    /// Get all modules - zero-copy slice
    #[inline(always)]
    pub fn modules(&self) -> &[ModuleEntry] {
        let header = self.header();
        unsafe {
            std::slice::from_raw_parts(
                self.mmap.as_ptr().add(header.modules_offset as usize) as *const ModuleEntry,
                header.module_count as usize,
            )
        }
    }

    /// Get import edges
    #[inline(always)]
    pub fn edges(&self) -> &[ImportEdge] {
        let header = self.header();
        let edge_count: usize = self.modules().iter().map(|m| m.import_count as usize).sum();

        unsafe {
            std::slice::from_raw_parts(
                self.mmap.as_ptr().add(header.edges_offset as usize) as *const ImportEdge,
                edge_count,
            )
        }
    }

    /// Get topologically sorted module order (for bundling)
    pub fn topo_order(&self) -> Vec<u32> {
        let mut order = Vec::new();
        let mut in_degree = vec![0u32; self.modules().len()];

        // Calculate in-degrees
        for edge in self.edges() {
            in_degree[edge.to_module as usize] += 1;
        }

        // Find nodes with no incoming edges
        let mut queue: VecDeque<u32> = in_degree
            .iter()
            .enumerate()
            .filter(|&(_, &d)| d == 0)
            .map(|(i, _)| i as u32)
            .collect();

        while let Some(node) = queue.pop_front() {
            order.push(node);

            let module = &self.modules()[node as usize];
            let start = module.first_import as usize;
            let end = start + module.import_count as usize;

            for edge in &self.edges()[start..end] {
                in_degree[edge.to_module as usize] -= 1;
                if in_degree[edge.to_module as usize] == 0 {
                    queue.push_back(edge.to_module);
                }
            }
        }

        order
    }
}

/// Graph builder helper
struct GraphBuilder {
    modules: Vec<ModuleEntry>,
    edges: Vec<ImportEdge>,
    strings: Vec<u8>,
}

impl GraphBuilder {
    fn new() -> Self {
        Self {
            modules: Vec::new(),
            edges: Vec::new(),
            strings: Vec::new(),
        }
    }

    fn build(self) -> io::Result<Vec<u8>> {
        let mut output = Vec::new();

        // Calculate offsets
        let header_size = std::mem::size_of::<ModuleGraphHeader>();
        let modules_offset = header_size as u64;
        let modules_size = self.modules.len() * std::mem::size_of::<ModuleEntry>();
        let edges_offset = modules_offset + modules_size as u64;
        let edges_size = self.edges.len() * std::mem::size_of::<ImportEdge>();
        let strings_offset = edges_offset + edges_size as u64;

        // Write header
        let header = ModuleGraphHeader {
            magic: magic::MODULE_GRAPH,
            version: 1,
            source_hash: 0, // Filled by caller
            entry_count: 0,
            module_count: self.modules.len() as u32,
            modules_offset,
            edges_offset,
            strings_offset,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        };
        output.extend_from_slice(bytemuck::bytes_of(&header));

        // Write modules
        for module in &self.modules {
            output.extend_from_slice(bytemuck::bytes_of(module));
        }

        // Write edges
        for edge in &self.edges {
            output.extend_from_slice(bytemuck::bytes_of(edge));
        }

        // Write strings
        output.extend_from_slice(&self.strings);

        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_computation() {
        let temp_dir = tempfile::tempdir().unwrap();
        let hash = ModuleGraphCache::compute_hash(temp_dir.path(), &[]);
        assert_ne!(hash, 0);
    }
}
