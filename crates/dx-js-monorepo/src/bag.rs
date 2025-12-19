//! Binary Affected Graph (BAG) format
//!
//! Pre-computed change propagation paths for instant impact detection.

use bytemuck::{Pod, Zeroable};
use crate::{BAG_MAGIC, FORMAT_VERSION};

/// Binary Affected Graph header
#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct BagHeader {
    /// Magic bytes: "DXAG"
    pub magic: [u8; 4],
    /// Format version
    pub version: u32,
    /// Number of packages
    pub package_count: u32,
    /// Offset to inverse dependency index
    pub inverse_deps_offset: u64,
    /// Offset to transitive closure cache
    pub transitive_offset: u64,
    /// Offset to file-to-package mapping
    pub file_map_offset: u64,
    /// Blake3 hash of content
    pub content_hash: [u8; 32],
}

impl BagHeader {
    /// Size of header in bytes
    pub const SIZE: usize = std::mem::size_of::<Self>();

    /// Create a new header
    pub fn new(package_count: u32) -> Self {
        Self {
            magic: BAG_MAGIC,
            version: FORMAT_VERSION,
            package_count,
            inverse_deps_offset: Self::SIZE as u64,
            transitive_offset: 0,
            file_map_offset: 0,
            content_hash: [0; 32],
        }
    }
}

/// Inverse dependency entry
#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct InverseDepsEntry {
    /// Package index
    pub package_idx: u32,
    /// Offset to list of dependents
    pub dependents_offset: u32,
    /// Number of direct dependents
    pub dependents_count: u16,
    /// Padding
    _padding: u16,
}

/// File-to-package mapping entry
#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct FileMapEntry {
    /// Hash of file path
    pub path_hash: u64,
    /// Owning package index
    pub package_idx: u32,
    /// Padding
    _padding: u32,
}

/// Affected graph data for serialization
#[derive(Debug, Clone, Default)]
pub struct AffectedGraphData {
    /// Number of packages
    pub package_count: u32,
    /// Inverse dependencies: package_idx -> list of packages that depend on it
    pub inverse_deps: Vec<Vec<u32>>,
    /// Transitive closure: package_idx -> all transitive dependents
    pub transitive_closure: Vec<Vec<u32>>,
    /// File path hash -> package index
    pub file_map: Vec<(u64, u32)>,
}

impl AffectedGraphData {
    /// Create from dependency edges
    pub fn from_edges(package_count: u32, edges: &[(u32, u32)]) -> Self {
        let n = package_count as usize;
        
        // Build inverse dependency index
        let mut inverse_deps = vec![Vec::new(); n];
        for &(from, to) in edges {
            inverse_deps[to as usize].push(from);
        }
        
        // Compute transitive closure using Floyd-Warshall-like approach
        let mut transitive_closure = inverse_deps.clone();
        
        // For each package, compute all transitive dependents
        for i in 0..n {
            let mut visited = vec![false; n];
            let mut stack = inverse_deps[i].clone();
            
            while let Some(pkg) = stack.pop() {
                if visited[pkg as usize] {
                    continue;
                }
                visited[pkg as usize] = true;
                
                for &dep in &inverse_deps[pkg as usize] {
                    if !visited[dep as usize] {
                        stack.push(dep);
                    }
                }
            }
            
            transitive_closure[i] = (0..n as u32)
                .filter(|&j| visited[j as usize])
                .collect();
        }
        
        Self {
            package_count,
            inverse_deps,
            transitive_closure,
            file_map: Vec::new(),
        }
    }

    /// Add file-to-package mapping
    pub fn add_file_mapping(&mut self, path: &str, package_idx: u32) {
        let path_hash = xxhash_rust::xxh3::xxh3_64(path.as_bytes());
        self.file_map.push((path_hash, package_idx));
    }

    /// Get direct dependents of a package
    pub fn dependents(&self, package_idx: u32) -> &[u32] {
        self.inverse_deps.get(package_idx as usize)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    /// Get all transitive dependents of a package
    pub fn transitive_dependents(&self, package_idx: u32) -> &[u32] {
        self.transitive_closure.get(package_idx as usize)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    /// Find package that owns a file
    pub fn file_to_package(&self, path: &str) -> Option<u32> {
        let path_hash = xxhash_rust::xxh3::xxh3_64(path.as_bytes());
        self.file_map.iter()
            .find(|(h, _)| *h == path_hash)
            .map(|(_, idx)| *idx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bag_header_size() {
        assert_eq!(BagHeader::SIZE, 72);
    }

    #[test]
    fn test_inverse_deps() {
        // a -> b -> c
        let edges = vec![(0, 1), (1, 2)];
        let graph = AffectedGraphData::from_edges(3, &edges);
        
        // b depends on a, so a is in inverse_deps[1]
        assert!(graph.dependents(1).contains(&0));
        // c depends on b, so b is in inverse_deps[2]
        assert!(graph.dependents(2).contains(&1));
        // a has no dependents
        assert!(graph.dependents(0).is_empty());
    }

    #[test]
    fn test_transitive_closure() {
        // a -> b -> c
        let edges = vec![(0, 1), (1, 2)];
        let graph = AffectedGraphData::from_edges(3, &edges);
        
        // Changing c affects nothing
        assert!(graph.transitive_dependents(2).is_empty());
        // Changing b affects c
        assert!(graph.transitive_dependents(1).contains(&2));
        // Changing a affects b and c
        let a_deps = graph.transitive_dependents(0);
        assert!(a_deps.contains(&1));
        assert!(a_deps.contains(&2));
    }

    #[test]
    fn test_file_mapping() {
        let mut graph = AffectedGraphData::from_edges(3, &[]);
        graph.add_file_mapping("packages/a/src/index.ts", 0);
        graph.add_file_mapping("packages/b/src/index.ts", 1);
        
        assert_eq!(graph.file_to_package("packages/a/src/index.ts"), Some(0));
        assert_eq!(graph.file_to_package("packages/b/src/index.ts"), Some(1));
        assert_eq!(graph.file_to_package("packages/c/src/index.ts"), None);
    }
}
