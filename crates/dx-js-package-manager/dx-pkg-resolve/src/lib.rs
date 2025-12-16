//! dx-pkg-resolve: Dependency Resolution
//!
//! Now uses npm registry API directly (zero infrastructure!)
//! Still fast through: binary lock file checking, BFS resolution, parallel fetching

use anyhow::{Context, Result};
use dx_pkg_core::{version::Version};
use std::collections::{HashMap, HashSet, VecDeque};

// Re-export npm client
pub use dx_pkg_npm::{NpmClient, AbbreviatedMetadata};

/// Package identifier
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct PackageId {
    pub name: String,
    pub version: Version,
}

/// Package resolution result
#[derive(Debug, Clone, serde::Serialize)]
pub struct ResolvedPackage {
    pub name: String,
    pub version: String,
    pub tarball_url: String,
    pub dependencies: HashMap<String, String>,
}

/// Complete resolved dependency graph
#[derive(Debug, Clone)]
pub struct ResolvedGraph {
    pub packages: Vec<ResolvedPackage>,
    /// Fast lookup: name -> package
    lookup: HashMap<String, ResolvedPackage>,
}

impl ResolvedGraph {
    pub fn new() -> Self {
        Self {
            packages: Vec::new(),
            lookup: HashMap::new(),
        }
    }

    pub fn add(&mut self, package: ResolvedPackage) {
        self.lookup.insert(package.name.clone(), package.clone());
        self.packages.push(package);
    }

    pub fn get(&self, name: &str) -> Option<&ResolvedPackage> {
        self.lookup.get(name)
    }
}

/// Local dependency resolver
pub struct LocalResolver {
    npm: NpmClient,
}

impl LocalResolver {
    pub fn new() -> Self {
        Self {
            npm: NpmClient::new(),
        }
    }

    /// Resolve all dependencies from package.json manifest
    pub async fn resolve(&mut self, dependencies: &HashMap<String, String>) -> Result<ResolvedGraph> {
        let mut resolved = ResolvedGraph::new();
        let mut queue: VecDeque<(String, String)> = VecDeque::new();
        let mut seen: HashSet<String> = HashSet::new();

        // Start with direct dependencies
        for (name, version) in dependencies {
            queue.push_back((name.clone(), version.clone()));
        }

        // BFS resolution
        while let Some((name, constraint)) = queue.pop_front() {
            let key = format!("{}@{}", name, constraint);
            if seen.contains(&key) {
                continue;
            }
            seen.insert(key);

            // Fetch abbreviated metadata from npm (faster than full)
            let metadata = self.npm.get_abbreviated(&name).await
                .with_context(|| format!("Failed to fetch metadata for {}", name))?;

            // Find best matching version
            let version = Self::find_best_version(&metadata, &constraint)?;
            let version_info = metadata.versions.get(&version)
                .ok_or_else(|| anyhow::anyhow!("Version {} not found for {}", version, name))?;

            // Add to resolved graph
            let package = ResolvedPackage {
                name: name.clone(),
                version: version.clone(),
                tarball_url: version_info.dist.tarball.clone(),
                dependencies: version_info.dependencies.clone(),
            };
            
            resolved.add(package.clone());

            // Queue transitive dependencies
            for (dep_name, dep_constraint) in &package.dependencies {
                queue.push_back((dep_name.clone(), dep_constraint.clone()));
            }
        }

        Ok(resolved)
    }

    /// Find the best matching version for a semver constraint
    fn find_best_version(
        metadata: &AbbreviatedMetadata,
        constraint: &str,
    ) -> Result<String> {
        // Handle special cases
        if constraint == "latest" || constraint == "*" {
            return metadata
                .dist_tags
                .get("latest")
                .cloned()
                .ok_or_else(|| anyhow::anyhow!("No 'latest' tag found"));
        }

        // Handle exact versions
        if metadata.versions.contains_key(constraint) {
            return Ok(constraint.to_string());
        }

        // Parse semver constraint
        let req = semver::VersionReq::parse(constraint)
            .with_context(|| format!("Invalid version constraint: {}", constraint))?;

        // Find all matching versions
        let mut matching: Vec<semver::Version> = metadata
            .versions
            .keys()
            .filter_map(|v| semver::Version::parse(v).ok())
            .filter(|v| req.matches(v))
            .collect();

        // Sort descending (prefer newest)
        matching.sort_by(|a, b| b.cmp(a));

        matching
            .first()
            .map(|v| v.to_string())
            .ok_or_else(|| anyhow::anyhow!("No matching version found for constraint: {}", constraint))
    }
}

impl Default for LocalResolver {
    fn default() -> Self {
        Self::new()
    }
}

// Keep old dependency graph types for compatibility
/// Dependency constraint
#[derive(Debug, Clone)]
pub struct Dependency {
    pub name: String,
    pub constraint: VersionConstraint,
}

/// Version constraint types
/// Version constraint types
#[derive(Debug, Clone)]
pub enum VersionConstraint {
    Exact(Version),
    Range { min: Version, max: Version },
    Caret(Version),  // ^1.2.3 (>=1.2.3 <2.0.0)
    Tilde(Version),  // ~1.2.3 (>=1.2.3 <1.3.0)
    Latest,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_resolve_lodash() {
        let mut resolver = LocalResolver::new();
        let mut deps = HashMap::new();
        deps.insert("lodash".to_string(), "^4.17.0".to_string());

        let graph = resolver.resolve(&deps).await.unwrap();
        
        assert_eq!(graph.packages.len(), 1);
        assert_eq!(graph.packages[0].name, "lodash");
        assert!(graph.packages[0].version.starts_with("4.17"));
    }

    #[tokio::test]
    async fn test_resolve_with_deps() {
        let mut resolver = LocalResolver::new();
        let mut deps = HashMap::new();
        deps.insert("express".to_string(), "^4.18.0".to_string());

        let graph = resolver.resolve(&deps).await.unwrap();
        
        // Express has many dependencies
        assert!(graph.packages.len() > 10);
        assert!(graph.get("express").is_some());
    }
}
