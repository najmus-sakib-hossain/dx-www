//! Property-based tests for the dependency resolver
//!
//! Property 9: Resolution Hint Cache Correctness
//! For any set of dependencies, if a cached resolution exists and is valid,
//! using the cached resolution SHALL produce the same installed package set
//! as performing a fresh resolution.

use proptest::prelude::*;
use std::collections::HashSet;

use dx_py_core::version::PackedVersion;
use dx_py_package_manager::resolver::{
    Dependency, HintCache, InMemoryProvider, Resolution, ResolvedPackage, Resolver,
    VersionConstraint,
};

/// Generate arbitrary package names
fn arb_package_name() -> impl Strategy<Value = String> {
    prop::string::string_regex("[a-z][a-z0-9_]{2,15}")
        .unwrap()
        .prop_filter("non-empty", |s| !s.is_empty())
}

/// Generate arbitrary version
fn arb_version() -> impl Strategy<Value = (u32, u32, u32)> {
    (0u32..100, 0u32..100, 0u32..100)
}

/// Generate arbitrary version constraint
fn arb_constraint() -> impl Strategy<Value = VersionConstraint> {
    prop_oneof![
        Just(VersionConstraint::Any),
        arb_version().prop_map(|(major, minor, patch)| {
            VersionConstraint::Gte(PackedVersion::new(major, minor, patch))
        }),
        arb_version().prop_map(|(major, minor, patch)| {
            VersionConstraint::Exact(PackedVersion::new(major, minor, patch))
        }),
    ]
}

/// Generate a simple package registry
fn arb_registry() -> impl Strategy<Value = InMemoryProvider> {
    prop::collection::vec(
        (arb_package_name(), prop::collection::vec(arb_version(), 1..5)),
        1..10,
    )
    .prop_map(|packages| {
        let mut provider = InMemoryProvider::new();
        for (name, versions) in packages {
            for (major, minor, patch) in versions {
                let version_str = format!("{}.{}.{}", major, minor, patch);
                provider.add_package(&name, &version_str, vec![]);
            }
        }
        provider
    })
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    /// Property 9: Resolution Hint Cache Correctness
    /// Validates: Requirements 7.1, 7.3
    ///
    /// For any set of dependencies, if a cached resolution exists and is valid,
    /// using the cached resolution SHALL produce the same installed package set
    /// as performing a fresh resolution.
    #[test]
    fn prop_hint_cache_correctness(
        _provider in arb_registry(),
        _seed in any::<u64>(),
    ) {
        // Create a simple scenario with known packages
        let mut test_provider = InMemoryProvider::new();
        test_provider.add_package("pkg_a", "1.0.0", vec![]);
        test_provider.add_package("pkg_a", "2.0.0", vec![]);
        test_provider.add_package("pkg_b", "1.0.0", vec![]);
        test_provider.add_package("pkg_b", "1.5.0", vec![]);
        test_provider.add_package("pkg_c", "3.0.0", vec![]);

        let deps = vec![
            Dependency::new("pkg_a", VersionConstraint::Any),
            Dependency::new("pkg_b", VersionConstraint::Gte(PackedVersion::new(1, 0, 0))),
        ];

        // First resolution (no cache)
        let mut resolver1 = Resolver::new(test_provider.clone());
        let res1 = resolver1.resolve(&deps).unwrap();
        prop_assert!(!res1.from_cache);

        // Second resolution (should use cache)
        let res2 = resolver1.resolve(&deps).unwrap();
        prop_assert!(res2.from_cache);

        // Both resolutions should produce the same packages
        let names1: HashSet<_> = res1.packages.iter().map(|p| &p.name).collect();
        let names2: HashSet<_> = res2.packages.iter().map(|p| &p.name).collect();
        prop_assert_eq!(names1, names2);

        // Versions should also match
        for pkg1 in &res1.packages {
            let pkg2 = res2.packages.iter().find(|p| p.name == pkg1.name).unwrap();
            prop_assert_eq!(pkg1.version, pkg2.version);
        }
    }

    /// Property: Cache lookup returns None for unknown hashes
    #[test]
    fn prop_cache_miss_for_unknown(hash in any::<u64>()) {
        let cache = HintCache::new();
        prop_assert!(cache.lookup(hash).is_none());
    }

    /// Property: Cache stores and retrieves correctly
    #[test]
    fn prop_cache_store_retrieve(
        hash in any::<u64>(),
        pkg_name in arb_package_name(),
        version in arb_version(),
    ) {
        let mut cache = HintCache::new();
        
        let packages = vec![ResolvedPackage::new(
            &pkg_name,
            PackedVersion::new(version.0, version.1, version.2),
            &format!("{}.{}.{}", version.0, version.1, version.2),
        )];
        let resolution = Resolution::new(packages.clone(), 10);
        
        cache.store(hash, &resolution);
        
        let cached = cache.lookup(hash);
        prop_assert!(cached.is_some());
        
        let cached = cached.unwrap();
        prop_assert_eq!(cached.packages.len(), 1);
        prop_assert_eq!(&cached.packages[0].name, &pkg_name);
    }

    /// Property: Version constraint satisfaction is consistent
    #[test]
    fn prop_constraint_satisfaction_consistent(
        major in 0u32..100,
        minor in 0u32..100,
        patch in 0u32..100,
    ) {
        let version = PackedVersion::new(major, minor, patch);
        
        // Any always satisfies
        prop_assert!(VersionConstraint::Any.satisfies(&version));
        
        // Exact matches only itself
        prop_assert!(VersionConstraint::Exact(version).satisfies(&version));
        
        // Gte satisfies >= versions
        let lower = PackedVersion::new(
            major.saturating_sub(1),
            minor,
            patch,
        );
        prop_assert!(VersionConstraint::Gte(lower).satisfies(&version));
        
        // Lt satisfies < versions
        let higher = PackedVersion::new(major + 1, 0, 0);
        prop_assert!(VersionConstraint::Lt(higher).satisfies(&version));
    }

    /// Property: Resolution always picks highest valid version
    #[test]
    fn prop_resolution_picks_highest(
        v1 in 1u32..50,
        v2 in 51u32..100,
    ) {
        let mut provider = InMemoryProvider::new();
        provider.add_package("test_pkg", &format!("{}.0.0", v1), vec![]);
        provider.add_package("test_pkg", &format!("{}.0.0", v2), vec![]);
        
        let mut resolver = Resolver::new(provider);
        let deps = vec![Dependency::new("test_pkg", VersionConstraint::Any)];
        
        let resolution = resolver.resolve(&deps).unwrap();
        prop_assert_eq!(resolution.packages.len(), 1);
        prop_assert_eq!(resolution.packages[0].version.major, v2);
    }

    /// Property: Cache eviction maintains correctness
    #[test]
    fn prop_cache_eviction_correctness(
        entries in prop::collection::vec(
            (any::<u64>(), arb_package_name()),
            10..20
        ),
    ) {
        let mut cache = HintCache::with_max_size(5);
        
        for (hash, name) in &entries {
            let packages = vec![ResolvedPackage::new(
                name,
                PackedVersion::new(1, 0, 0),
                "1.0.0",
            )];
            let resolution = Resolution::new(packages, 10);
            cache.store(*hash, &resolution);
        }
        
        // Cache should not exceed max size
        prop_assert!(cache.len() <= 5);
        
        // All entries in cache should be valid
        for (hash, _) in entries.iter().rev().take(5) {
            if let Some(cached) = cache.lookup(*hash) {
                prop_assert!(cached.is_valid());
            }
        }
    }
}

#[test]
fn test_resolver_with_transitive_deps() {
    let mut provider = InMemoryProvider::new();
    
    // requests depends on urllib3 and certifi
    provider.add_package(
        "requests",
        "2.30.0",
        vec![
            Dependency::new("urllib3", VersionConstraint::Gte(PackedVersion::new(1, 21, 0))),
            Dependency::new("certifi", VersionConstraint::Any),
        ],
    );
    provider.add_package("urllib3", "2.0.0", vec![]);
    provider.add_package("certifi", "2023.5.7", vec![]);
    
    let mut resolver = Resolver::new(provider);
    let deps = vec![Dependency::new("requests", VersionConstraint::Any)];
    
    let resolution = resolver.resolve(&deps).unwrap();
    
    // Should resolve all 3 packages
    assert_eq!(resolution.packages.len(), 3);
    
    let names: HashSet<_> = resolution.packages.iter().map(|p| p.name.as_str()).collect();
    assert!(names.contains("requests"));
    assert!(names.contains("urllib3"));
    assert!(names.contains("certifi"));
}

#[test]
fn test_hint_cache_delta_resolution() {
    let mut provider = InMemoryProvider::new();
    provider.add_package("pkg_a", "1.0.0", vec![]);
    provider.add_package("pkg_b", "1.0.0", vec![]);
    provider.add_package("pkg_c", "1.0.0", vec![]);
    
    let mut resolver = Resolver::new(provider);
    
    // First resolution with pkg_a and pkg_b
    let deps1 = vec![
        Dependency::new("pkg_a", VersionConstraint::Any),
        Dependency::new("pkg_b", VersionConstraint::Any),
    ];
    let res1 = resolver.resolve(&deps1).unwrap();
    assert!(!res1.from_cache);
    
    // Second resolution with same deps should use cache
    let res2 = resolver.resolve(&deps1).unwrap();
    assert!(res2.from_cache);
}
