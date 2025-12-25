//! Property-based tests for DPL lock file operations
//!
//! **Feature: dx-py-package-manager, Property 6: Hash Table O(1) Lookup Correctness**
//! **Validates: Requirements 2.1**
//!
//! **Feature: dx-py-package-manager, Property 4: DPL Round-Trip Consistency**
//! **Validates: Requirements 2.8**

use dx_py_package_manager::{DplBuilder, DplLockFile};
use proptest::prelude::*;

/// Generate a valid package name (lowercase, alphanumeric with hyphens/underscores)
fn arb_package_name() -> impl Strategy<Value = String> {
    "[a-z][a-z0-9_-]{0,30}".prop_map(|s| s.to_string())
}

/// Generate a valid version string
fn arb_version() -> impl Strategy<Value = String> {
    (1u32..100, 0u32..100, 0u32..100).prop_map(|(major, minor, patch)| {
        format!("{}.{}.{}", major, minor, patch)
    })
}

/// Generate a source hash
fn arb_hash() -> impl Strategy<Value = [u8; 32]> {
    proptest::collection::vec(any::<u8>(), 32).prop_map(|v| {
        let mut arr = [0u8; 32];
        arr.copy_from_slice(&v);
        arr
    })
}

/// Generate a package entry (name, version, hash)
fn arb_package_entry() -> impl Strategy<Value = (String, String, [u8; 32])> {
    (arb_package_name(), arb_version(), arb_hash())
}

/// Generate a list of unique package entries
fn arb_unique_packages(min: usize, max: usize) -> impl Strategy<Value = Vec<(String, String, [u8; 32])>> {
    proptest::collection::vec(arb_package_entry(), min..max).prop_map(|entries| {
        // Deduplicate by name
        let mut seen = std::collections::HashSet::new();
        entries
            .into_iter()
            .filter(|(name, _, _)| seen.insert(name.clone()))
            .collect()
    })
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    /// Property 6: Hash table lookup returns correct entry for all packages
    ///
    /// *For any* DPL lock file with N packages, looking up any package by name
    /// SHALL return the correct entry with matching name and version.
    /// **Validates: Requirements 2.1**
    #[test]
    fn prop_hash_table_lookup_correct(
        packages in arb_unique_packages(1, 50),
        python_version in "[0-9]{1,2}\\.[0-9]{1,2}\\.[0-9]{1,2}",
        platform in "[a-z_0-9]{5,20}"
    ) {
        let mut builder = DplBuilder::new(&python_version, &platform);

        for (name, version, hash) in &packages {
            builder.add_package(name, version, *hash);
        }

        let data = builder.build();
        let lock_file = DplLockFile::from_bytes(data).unwrap();

        // Verify all packages can be looked up correctly
        for (name, version, hash) in &packages {
            let entry = lock_file.lookup(name);
            prop_assert!(entry.is_some(), "Package '{}' not found", name);

            let entry = entry.unwrap();
            prop_assert_eq!(entry.name_str(), name.as_str(),
                "Name mismatch for '{}': got '{}'", name, entry.name_str());
            prop_assert_eq!(entry.version_str(), version.as_str(),
                "Version mismatch for '{}': expected '{}', got '{}'", name, version, entry.version_str());
            prop_assert_eq!(&entry.source_hash, hash,
                "Hash mismatch for '{}'", name);
        }
    }

    /// Property 6: Hash table lookup returns None for non-existent packages
    ///
    /// *For any* DPL lock file, looking up a non-existent package
    /// SHALL return None.
    /// **Validates: Requirements 2.1**
    #[test]
    fn prop_hash_table_lookup_nonexistent(
        packages in arb_unique_packages(1, 20),
        nonexistent_name in "[a-z][a-z0-9_-]{31,40}"  // Names longer than typical to avoid collision
    ) {
        let mut builder = DplBuilder::new("3.12.0", "linux");

        for (name, version, hash) in &packages {
            builder.add_package(name, version, *hash);
        }

        let data = builder.build();
        let lock_file = DplLockFile::from_bytes(data).unwrap();

        // Verify non-existent package returns None
        let result = lock_file.lookup(&nonexistent_name);
        prop_assert!(result.is_none(),
            "Expected None for non-existent package '{}', got Some", nonexistent_name);
    }

    /// Property 6: Package count matches number of added packages
    ///
    /// *For any* DPL lock file, the package_count SHALL equal the number
    /// of packages added to the builder.
    /// **Validates: Requirements 2.1**
    #[test]
    fn prop_package_count_correct(packages in arb_unique_packages(0, 100)) {
        let mut builder = DplBuilder::new("3.11.0", "win_amd64");

        for (name, version, hash) in &packages {
            builder.add_package(name, version, *hash);
        }

        let data = builder.build();
        let lock_file = DplLockFile::from_bytes(data).unwrap();

        prop_assert_eq!(lock_file.package_count() as usize, packages.len(),
            "Package count mismatch: expected {}, got {}", packages.len(), lock_file.package_count());
    }

    /// Property 4: DPL round-trip produces equivalent data
    ///
    /// *For any* valid DPL lock file, building it and then iterating over
    /// all entries SHALL produce the same packages in the same order.
    /// **Validates: Requirements 2.8**
    #[test]
    fn prop_dpl_roundtrip_entries(
        packages in arb_unique_packages(1, 50),
        python_version in "[0-9]{1,2}\\.[0-9]{1,2}\\.[0-9]{1,2}",
        platform in "[a-z_0-9]{5,20}"
    ) {
        let mut builder = DplBuilder::new(&python_version, &platform);

        for (name, version, hash) in &packages {
            builder.add_package(name, version, *hash);
        }

        let data = builder.build();
        let lock_file = DplLockFile::from_bytes(data).unwrap();

        // Verify metadata
        prop_assert_eq!(lock_file.python_version(), python_version.as_str());
        prop_assert_eq!(lock_file.platform(), platform.as_str());

        // Verify all entries via iteration
        let entries: Vec<_> = lock_file.iter().collect();
        prop_assert_eq!(entries.len(), packages.len());

        for (i, (name, version, hash)) in packages.iter().enumerate() {
            prop_assert_eq!(entries[i].name_str(), name.as_str());
            prop_assert_eq!(entries[i].version_str(), version.as_str());
            prop_assert_eq!(&entries[i].source_hash, hash);
        }
    }

    /// Property 4: DPL integrity verification passes for valid files
    ///
    /// *For any* valid DPL lock file built by DplBuilder,
    /// verify() SHALL return true.
    /// **Validates: Requirements 2.8**
    #[test]
    fn prop_dpl_integrity_valid(packages in arb_unique_packages(1, 30)) {
        let mut builder = DplBuilder::new("3.12.0", "manylinux_2_17_x86_64");

        for (name, version, hash) in &packages {
            builder.add_package(name, version, *hash);
        }

        let data = builder.build();
        let lock_file = DplLockFile::from_bytes(data).unwrap();

        prop_assert!(lock_file.verify(), "Integrity verification failed");
    }

    /// Property 6: Empty lock file handles lookups correctly
    ///
    /// *For any* empty DPL lock file, all lookups SHALL return None.
    /// **Validates: Requirements 2.1**
    #[test]
    fn prop_empty_lockfile_lookup(name in arb_package_name()) {
        let builder = DplBuilder::new("3.12.0", "linux");
        let data = builder.build();
        let lock_file = DplLockFile::from_bytes(data).unwrap();

        prop_assert_eq!(lock_file.package_count(), 0);
        prop_assert!(lock_file.lookup(&name).is_none());
    }
}
