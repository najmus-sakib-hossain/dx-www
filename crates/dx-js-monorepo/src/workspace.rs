//! Workspace Manager
//!
//! Loads and maintains the Binary Workspace Manifest.

use std::path::{Path, PathBuf};
use memmap2::Mmap;
use crate::error::WorkspaceError;
use crate::types::PackageEntry;
use crate::bwm::{BwmHeader, BwmSerializer, WorkspaceData};

/// Workspace Manager for loading and querying workspace manifests
pub struct WorkspaceManager {
    /// Memory-mapped manifest data
    mmap: Option<Mmap>,
    /// Parsed workspace data (for non-mmap access)
    data: Option<WorkspaceData>,
    /// Path to manifest file
    manifest_path: Option<PathBuf>,
    /// Package name to index lookup
    name_index: rustc_hash::FxHashMap<String, u32>,
}

impl WorkspaceManager {
    /// Create a new workspace manager
    pub fn new() -> Self {
        Self {
            mmap: None,
            data: None,
            manifest_path: None,
            name_index: rustc_hash::FxHashMap::default(),
        }
    }

    /// Load workspace manifest from memory-mapped file
    pub fn load(&mut self, path: &Path) -> Result<(), WorkspaceError> {
        let file = std::fs::File::open(path)
            .map_err(|_| WorkspaceError::ManifestNotFound { 
                path: path.to_path_buf() 
            })?;

        // Memory-map the file
        let mmap = unsafe { Mmap::map(&file) }?;

        // Validate header
        if mmap.len() < BwmHeader::SIZE {
            return Err(WorkspaceError::ManifestCorrupted {
                reason: "file too small".to_string(),
                hash_mismatch: false,
            });
        }

        let header: &BwmHeader = bytemuck::from_bytes(&mmap[..BwmHeader::SIZE]);
        header.validate_magic()?;
        header.validate_version()?;

        // Verify content hash
        let computed_hash = blake3::hash(&mmap[BwmHeader::SIZE..]);
        if computed_hash.as_bytes() != &header.content_hash {
            return Err(WorkspaceError::ManifestCorrupted {
                reason: "content hash mismatch".to_string(),
                hash_mismatch: true,
            });
        }

        // Parse and build index
        let data = BwmSerializer::deserialize(&mmap)?;
        self.build_name_index(&data);

        self.mmap = Some(mmap);
        self.data = Some(data);
        self.manifest_path = Some(path.to_path_buf());

        Ok(())
    }

    /// Load from raw bytes (for testing)
    pub fn load_from_bytes(&mut self, bytes: &[u8]) -> Result<(), WorkspaceError> {
        let data = BwmSerializer::deserialize(bytes)?;
        self.build_name_index(&data);
        self.data = Some(data);
        Ok(())
    }

    /// Build name-to-index lookup table
    fn build_name_index(&mut self, data: &WorkspaceData) {
        self.name_index.clear();
        for (idx, pkg) in data.packages.iter().enumerate() {
            self.name_index.insert(pkg.name.clone(), idx as u32);
        }
    }

    /// Get package by name with O(1) lookup
    pub fn get_package(&self, name: &str) -> Option<&crate::bwm::PackageData> {
        let idx = *self.name_index.get(name)?;
        self.data.as_ref()?.packages.get(idx as usize)
    }

    /// Get package by index
    pub fn get_package_by_index(&self, idx: u32) -> Option<&crate::bwm::PackageData> {
        self.data.as_ref()?.packages.get(idx as usize)
    }

    /// Get all packages in topological order
    pub fn topological_order(&self) -> &[u32] {
        self.data.as_ref()
            .map(|d| d.topological_order.as_slice())
            .unwrap_or(&[])
    }

    /// Get direct dependencies of a package
    pub fn dependencies(&self, pkg_idx: u32) -> Vec<u32> {
        let data = match &self.data {
            Some(d) => d,
            None => return Vec::new(),
        };

        data.dependency_edges.iter()
            .filter(|(from, _)| *from == pkg_idx)
            .map(|(_, to)| *to)
            .collect()
    }

    /// Get number of packages
    pub fn package_count(&self) -> usize {
        self.data.as_ref()
            .map(|d| d.packages.len())
            .unwrap_or(0)
    }

    /// Incrementally update manifest when package.json changes
    pub fn update_package(&mut self, _path: &Path) -> Result<(), WorkspaceError> {
        // TODO: Implement incremental update
        // For now, regenerate entire manifest
        self.regenerate()
    }

    /// Regenerate entire manifest from source files
    pub fn regenerate(&mut self) -> Result<(), WorkspaceError> {
        // TODO: Implement regeneration from package.json files
        Ok(())
    }

    /// Check if manifest is loaded
    pub fn is_loaded(&self) -> bool {
        self.data.is_some()
    }
}

impl Default for WorkspaceManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bwm::PackageData;

    fn create_test_workspace() -> WorkspaceData {
        let mut data = WorkspaceData {
            packages: vec![
                PackageData {
                    name: "pkg-a".to_string(),
                    path: "packages/a".to_string(),
                    version: (1, 0, 0),
                    dependencies: vec![],
                    is_private: false,
                },
                PackageData {
                    name: "pkg-b".to_string(),
                    path: "packages/b".to_string(),
                    version: (1, 0, 0),
                    dependencies: vec!["pkg-a".to_string()],
                    is_private: false,
                },
                PackageData {
                    name: "pkg-c".to_string(),
                    path: "packages/c".to_string(),
                    version: (1, 0, 0),
                    dependencies: vec!["pkg-b".to_string()],
                    is_private: false,
                },
            ],
            dependency_edges: vec![(0, 1), (1, 2)],
            topological_order: vec![],
        };
        data.compute_topological_order().unwrap();
        data
    }

    #[test]
    fn test_workspace_manager_load_from_bytes() {
        let data = create_test_workspace();
        let bytes = BwmSerializer::serialize(&data).unwrap();

        let mut manager = WorkspaceManager::new();
        manager.load_from_bytes(&bytes).unwrap();

        assert_eq!(manager.package_count(), 3);
        assert!(manager.get_package("pkg-a").is_some());
        assert!(manager.get_package("pkg-b").is_some());
        assert!(manager.get_package("pkg-c").is_some());
        assert!(manager.get_package("pkg-d").is_none());
    }

    #[test]
    fn test_package_lookup_by_index() {
        let data = create_test_workspace();
        let bytes = BwmSerializer::serialize(&data).unwrap();

        let mut manager = WorkspaceManager::new();
        manager.load_from_bytes(&bytes).unwrap();

        let pkg = manager.get_package_by_index(0).unwrap();
        assert_eq!(pkg.name, "pkg-a");

        let pkg = manager.get_package_by_index(1).unwrap();
        assert_eq!(pkg.name, "pkg-b");

        assert!(manager.get_package_by_index(100).is_none());
    }

    #[test]
    fn test_topological_order() {
        let data = create_test_workspace();
        let bytes = BwmSerializer::serialize(&data).unwrap();

        let mut manager = WorkspaceManager::new();
        manager.load_from_bytes(&bytes).unwrap();

        let order = manager.topological_order();
        assert_eq!(order.len(), 3);

        // pkg-a should come before pkg-b, pkg-b before pkg-c
        let pos_a = order.iter().position(|&x| x == 0).unwrap();
        let pos_b = order.iter().position(|&x| x == 1).unwrap();
        let pos_c = order.iter().position(|&x| x == 2).unwrap();

        assert!(pos_a < pos_b);
        assert!(pos_b < pos_c);
    }

    #[test]
    fn test_dependencies() {
        let data = create_test_workspace();
        let bytes = BwmSerializer::serialize(&data).unwrap();

        let mut manager = WorkspaceManager::new();
        manager.load_from_bytes(&bytes).unwrap();

        // pkg-a depends on pkg-b (edge 0 -> 1)
        let deps = manager.dependencies(0);
        assert_eq!(deps, vec![1]);

        // pkg-b depends on pkg-c (edge 1 -> 2)
        let deps = manager.dependencies(1);
        assert_eq!(deps, vec![2]);

        // pkg-c has no dependencies
        let deps = manager.dependencies(2);
        assert!(deps.is_empty());
    }
}
