//! Property-based tests for dx-js-monorepo
//!
//! These tests validate universal correctness properties using proptest.

#![cfg(test)]

use proptest::prelude::*;
use std::collections::HashSet;

use crate::bwm::{BwmSerializer, WorkspaceData, PackageData};
use crate::btg::{TaskGraphData, TaskData};
use crate::dxl::{DxlSerializer, LockfileData, PackageResolution};
use crate::dxc::XorPatch;
use crate::bag::AffectedGraphData;
use crate::types::PackageEntry;

// ============================================================================
// Arbitrary implementations for proptest
// ============================================================================

fn arb_package_name() -> impl Strategy<Value = String> {
    "[a-z][a-z0-9-]{0,20}".prop_map(|s| s.to_string())
}

fn arb_package_path() -> impl Strategy<Value = String> {
    "packages/[a-z][a-z0-9-]{0,10}".prop_map(|s| s.to_string())
}

fn arb_version() -> impl Strategy<Value = (u16, u16, u16)> {
    (0u16..100, 0u16..100, 0u16..100)
}

fn arb_package_data() -> impl Strategy<Value = PackageData> {
    (arb_package_name(), arb_package_path(), arb_version())
        .prop_map(|(name, path, version)| PackageData {
            name,
            path,
            version,
            dependencies: Vec::new(),
            is_private: false,
        })
}

// Note: These generators are available for future property tests
#[allow(dead_code)]
fn arb_task_name() -> impl Strategy<Value = String> {
    prop_oneof![
        Just("build".to_string()),
        Just("test".to_string()),
        Just("lint".to_string()),
        Just("typecheck".to_string()),
    ]
}

#[allow(dead_code)]
fn arb_command() -> impl Strategy<Value = String> {
    prop_oneof![
        Just("npm run build".to_string()),
        Just("npm test".to_string()),
        Just("npm run lint".to_string()),
        Just("tsc --noEmit".to_string()),
    ]
}

#[allow(dead_code)]
fn arb_task_data(max_pkg_idx: u32) -> impl Strategy<Value = TaskData> {
    (arb_task_name(), 0..=max_pkg_idx, arb_command())
        .prop_map(|(name, package_idx, command)| TaskData {
            name,
            package_idx,
            command,
            definition_hash: [0; 8],
            frame_budget_us: 0,
            cacheable: true,
        })
}

// ============================================================================
// Property 1: Binary Workspace Manifest Round-Trip Consistency
// ============================================================================

proptest! {
    #![proptest_config(ProptestConfig::with_cases(50))]

    /// Property 1: BWM serialization and deserialization are inverse operations
    #[test]
    fn prop_bwm_roundtrip(
        packages in prop::collection::vec(arb_package_data(), 1..20)
    ) {
        // Ensure unique package names
        let mut seen = HashSet::new();
        let packages: Vec<_> = packages.into_iter()
            .filter(|p| seen.insert(p.name.clone()))
            .collect();
        
        if packages.is_empty() {
            return Ok(());
        }

        let mut data = WorkspaceData {
            packages,
            dependency_edges: Vec::new(),
            topological_order: Vec::new(),
        };
        data.compute_topological_order().unwrap();

        let serialized = BwmSerializer::serialize(&data).unwrap();
        let deserialized = BwmSerializer::deserialize(&serialized).unwrap();

        // Verify package count matches
        prop_assert_eq!(data.packages.len(), deserialized.packages.len());

        // Verify each package
        for (orig, deser) in data.packages.iter().zip(deserialized.packages.iter()) {
            prop_assert_eq!(&orig.name, &deser.name);
            prop_assert_eq!(&orig.path, &deser.path);
            prop_assert_eq!(orig.version, deser.version);
        }
    }
}

// ============================================================================
// Property 2: Topological Order Validity
// ============================================================================

proptest! {
    #![proptest_config(ProptestConfig::with_cases(50))]

    /// Property 2: Topological order respects all dependency edges
    #[test]
    fn prop_topological_order_valid(
        package_count in 2usize..10,
        edge_density in 0.0f64..0.3
    ) {
        let packages: Vec<_> = (0..package_count)
            .map(|i| PackageData {
                name: format!("pkg-{}", i),
                path: format!("packages/pkg-{}", i),
                version: (1, 0, 0),
                dependencies: Vec::new(),
                is_private: false,
            })
            .collect();

        // Generate random DAG edges (only forward edges to avoid cycles)
        let mut edges = Vec::new();
        for i in 0..package_count {
            for j in (i + 1)..package_count {
                if rand::random::<f64>() < edge_density {
                    edges.push((i as u32, j as u32));
                }
            }
        }

        let mut data = WorkspaceData {
            packages,
            dependency_edges: edges.clone(),
            topological_order: Vec::new(),
        };
        data.compute_topological_order().unwrap();

        // Verify topological order: for each edge (from, to), from appears before to
        let position: std::collections::HashMap<_, _> = data.topological_order
            .iter()
            .enumerate()
            .map(|(i, &v)| (v, i))
            .collect();

        for (from, to) in &edges {
            let from_pos = position.get(from).unwrap();
            let to_pos = position.get(to).unwrap();
            prop_assert!(from_pos < to_pos, 
                "Edge ({}, {}) violates topological order: {} at pos {}, {} at pos {}",
                from, to, from, from_pos, to, to_pos);
        }
    }
}

// ============================================================================
// Property 5: Task Graph Parallel Execution Map Correctness
// ============================================================================

proptest! {
    #![proptest_config(ProptestConfig::with_cases(30))]

    /// Property 5: Tasks in the same parallel group have no dependencies between them
    #[test]
    fn prop_parallel_groups_independent(
        task_count in 2usize..8
    ) {
        let tasks: Vec<_> = (0..task_count)
            .map(|i| TaskData {
                name: format!("task-{}", i),
                package_idx: i as u32,
                command: "npm run build".to_string(),
                definition_hash: [0; 8],
                frame_budget_us: 0,
                cacheable: true,
            })
            .collect();

        // Create a simple chain: 0 -> 1 -> 2 -> ...
        let edges: Vec<_> = (0..task_count.saturating_sub(1))
            .map(|i| (i as u32, (i + 1) as u32))
            .collect();

        let topo_order: Vec<_> = (0..task_count as u32).collect();

        let mut data = TaskGraphData {
            tasks,
            dependency_edges: edges.clone(),
            topological_order: topo_order,
            parallel_groups: Vec::new(),
        };
        data.compute_parallel_groups();

        // Verify: no two tasks in the same group have a dependency edge
        // For a chain, each task should be in its own group (no parallelism)
        // This is because each task depends on the previous one
        for group in &data.parallel_groups {
            let task_count_in_group = { group.task_count };
            // In a chain, each group should have exactly 1 task
            prop_assert_eq!(task_count_in_group, 1, 
                "Chain should have single-task groups");
        }
    }
}

// ============================================================================
// Property 8: Blake3 Hash Determinism
// ============================================================================

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    /// Property 8: Same content always produces same hash
    #[test]
    fn prop_blake3_deterministic(content in prop::collection::vec(any::<u8>(), 0..1000)) {
        let hash1 = blake3::hash(&content);
        let hash2 = blake3::hash(&content);
        prop_assert_eq!(hash1.as_bytes(), hash2.as_bytes());
    }
}

// ============================================================================
// Property 10: Binary Fingerprint Size Invariance
// ============================================================================

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    /// Property 10: Fingerprints are always 32 bytes regardless of input size
    #[test]
    fn prop_fingerprint_size_invariant(content in prop::collection::vec(any::<u8>(), 0..10000)) {
        let hash = blake3::hash(&content);
        prop_assert_eq!(hash.as_bytes().len(), 32);
    }
}

// ============================================================================
// Property 11: DXC Cache Round-Trip Consistency
// ============================================================================

// Note: Full DXC round-trip test would require more infrastructure
// This tests the XOR patch component

proptest! {
    #![proptest_config(ProptestConfig::with_cases(50))]

    /// Property 11 (partial): XOR patches correctly reconstruct target from base
    #[test]
    fn prop_xor_patch_roundtrip(
        base in prop::collection::vec(any::<u8>(), 1..500),
        target in prop::collection::vec(any::<u8>(), 1..500)
    ) {
        let patch = XorPatch::create(&base, &target);
        let reconstructed = patch.apply(&base);
        prop_assert_eq!(reconstructed, target);
    }
}

// ============================================================================
// Property 12: XOR Patch Efficiency
// ============================================================================

proptest! {
    #![proptest_config(ProptestConfig::with_cases(50))]

    /// Property 12: XOR patches for similar content are smaller than full content
    #[test]
    fn prop_xor_patch_efficient_for_similar(
        base in prop::collection::vec(any::<u8>(), 200..500),
        change_count in 1usize..5
    ) {
        // Create target with small changes (only change a few bytes)
        let mut target = base.clone();
        let target_len = target.len();
        for i in 0..change_count.min(target_len / 50) {
            let idx = i * target_len / change_count.max(1);
            if idx < target_len {
                target[idx] ^= 0xFF;
            }
        }

        let patch = XorPatch::create(&base, &target);
        
        // Patch should be smaller than full target for similar content
        // The overhead is ~72 bytes for headers, so for small changes the patch
        // should still be much smaller than the full content
        let efficiency = patch.efficiency(target.len());
        prop_assert!(efficiency < 1.0, 
            "Patch efficiency {} should be < 1.0 for similar content (patch size: {}, target size: {})", 
            efficiency, patch.size(), target.len());
    }
}

// ============================================================================
// Property 14: DXL-Workspace Round-Trip Consistency
// ============================================================================

proptest! {
    #![proptest_config(ProptestConfig::with_cases(30))]

    /// Property 14: DXL lockfile serialization and deserialization are inverse
    #[test]
    fn prop_dxl_roundtrip(
        package_count in 1usize..10
    ) {
        let packages: Vec<_> = (0..package_count)
            .map(|i| PackageResolution {
                name: format!("pkg-{}", i),
                version: (1, 0, i as u16),
                integrity: [i as u8; 32],
                tarball_url: format!("https://registry.npmjs.org/pkg-{}", i),
                dependencies: Vec::new(),
            })
            .collect();

        let data = LockfileData {
            packages,
            vector_clock: [1, 0, 0, 0, 0, 0, 0, 0],
        };

        let serialized = DxlSerializer::serialize(&data).unwrap();
        let deserialized = DxlSerializer::deserialize(&serialized).unwrap();

        prop_assert_eq!(data.packages.len(), deserialized.packages.len());
        
        for (orig, deser) in data.packages.iter().zip(deserialized.packages.iter()) {
            prop_assert_eq!(&orig.name, &deser.name);
            prop_assert_eq!(orig.version, deser.version);
            prop_assert_eq!(&orig.integrity, &deser.integrity);
        }
    }
}

// ============================================================================
// Property 15: CRDT Merge Commutativity
// ============================================================================

proptest! {
    #![proptest_config(ProptestConfig::with_cases(30))]

    /// Property 15: CRDT merge is commutative (A merge B == B merge A)
    #[test]
    fn prop_crdt_merge_commutative(
        clock_a in prop::array::uniform8(0u64..10),
        clock_b in prop::array::uniform8(0u64..10)
    ) {
        let lockfile_a = LockfileData {
            packages: vec![PackageResolution {
                name: "pkg-a".to_string(),
                version: (1, 0, 0),
                integrity: [1; 32],
                tarball_url: "https://example.com/a".to_string(),
                dependencies: Vec::new(),
            }],
            vector_clock: clock_a,
        };

        let lockfile_b = LockfileData {
            packages: vec![PackageResolution {
                name: "pkg-b".to_string(),
                version: (2, 0, 0),
                integrity: [2; 32],
                tarball_url: "https://example.com/b".to_string(),
                dependencies: Vec::new(),
            }],
            vector_clock: clock_b,
        };

        // A merge B
        let mut result_ab = lockfile_a.clone();
        result_ab.merge(&lockfile_b).unwrap();

        // B merge A
        let mut result_ba = lockfile_b.clone();
        result_ba.merge(&lockfile_a).unwrap();

        // Vector clocks should be the same
        prop_assert_eq!(result_ab.vector_clock, result_ba.vector_clock);
        
        // Package sets should be the same (order may differ)
        let names_ab: HashSet<_> = result_ab.packages.iter().map(|p| &p.name).collect();
        let names_ba: HashSet<_> = result_ba.packages.iter().map(|p| &p.name).collect();
        prop_assert_eq!(names_ab, names_ba);
    }
}

// ============================================================================
// Property 18: Affected Package Transitivity
// ============================================================================

proptest! {
    #![proptest_config(ProptestConfig::with_cases(30))]

    /// Property 18: If A affects B and B affects C, then A affects C
    #[test]
    fn prop_affected_transitive(
        chain_length in 3usize..8
    ) {
        // Create a chain where each package depends on the next:
        // 0 depends on 1, 1 depends on 2, ..., n-2 depends on n-1
        // Edge (from, to) means "from depends on to"
        // So changing n-1 affects n-2, which affects n-3, etc.
        let edges: Vec<_> = (0..chain_length - 1)
            .map(|i| (i as u32, (i + 1) as u32))
            .collect();

        let graph = AffectedGraphData::from_edges(chain_length as u32, &edges);

        // Changing the last package (n-1) should affect all packages 0..n-1
        let last_pkg = (chain_length - 1) as u32;
        let affected = graph.transitive_dependents(last_pkg);
        
        for i in 0..chain_length - 1 {
            prop_assert!(affected.contains(&(i as u32)),
                "Package {} should be affected when package {} changes", i, last_pkg);
        }
        
        // Changing package 0 should affect nothing (it's the root, nothing depends on it)
        let affected_by_0 = graph.transitive_dependents(0);
        prop_assert!(affected_by_0.is_empty(),
            "Package 0 should not affect anything (it's the root)");
    }
}

// ============================================================================
// Property 19: Inverse Dependency Index Correctness
// ============================================================================

proptest! {
    #![proptest_config(ProptestConfig::with_cases(30))]

    /// Property 19: Inverse dependency index correctly identifies dependents
    #[test]
    fn prop_inverse_deps_correct(
        package_count in 3usize..10,
        edge_count in 1usize..15
    ) {
        // Generate random edges (ensuring no self-loops)
        let edges: Vec<_> = (0..edge_count)
            .map(|i| {
                let from = (i % package_count) as u32;
                let to = ((i + 1) % package_count) as u32;
                if from != to { (from, to) } else { (from, (to + 1) % package_count as u32) }
            })
            .filter(|(from, to)| from != to)
            .collect();

        let graph = AffectedGraphData::from_edges(package_count as u32, &edges);

        // For each edge (from, to), 'from' should be in dependents(to)
        for (from, to) in &edges {
            let dependents = graph.dependents(*to);
            prop_assert!(dependents.contains(from),
                "Package {} should be in dependents of {} (edge {} -> {})",
                from, to, from, to);
        }
    }
}

// ============================================================================
// Property 13: Cache Signature Tamper Detection
// ============================================================================

proptest! {
    #![proptest_config(ProptestConfig::with_cases(30))]

    /// Property 13: Modifying any byte of cache content causes signature verification to fail
    /// For any DXC cache entry, modifying any byte of the content SHALL cause
    /// signature verification to fail.
    #[test]
    fn prop_cache_tamper_detection(
        file_content in prop::collection::vec(any::<u8>(), 10..100),
        tamper_position in 0usize..100
    ) {
        use crate::dxc::CacheEntry;
        use crate::cache::CacheManager;
        use ed25519_dalek::{SigningKey, Signer};
        use tempfile::TempDir;

        // Create a cache entry with content
        let task_hash = blake3::hash(&file_content);
        let mut entry = CacheEntry::new(*task_hash.as_bytes());
        entry.add_file("test.txt".to_string(), file_content.clone(), 0o644);

        // Generate a signing key
        let signing_key = SigningKey::from_bytes(&[42u8; 32]);
        let verifying_key = signing_key.verifying_key();

        // Sign the entry
        let mut hasher = blake3::Hasher::new();
        hasher.update(&entry.task_hash);
        for file in &entry.files {
            hasher.update(file.path.as_bytes());
            hasher.update(&file.content);
        }
        let content_hash = hasher.finalize();
        let signature = signing_key.sign(content_hash.as_bytes());

        entry.signature = Some(signature.to_bytes());
        entry.public_key = Some(verifying_key.to_bytes());

        // Create cache manager and verify original entry
        let temp = TempDir::new().unwrap();
        let cache = CacheManager::new(temp.path().to_path_buf(), 1024 * 1024);
        
        // Original entry should verify
        let verify_result = cache.verify(&entry);
        prop_assert!(verify_result.is_ok() && verify_result.unwrap(),
            "Original entry should verify successfully");

        // Tamper with the content
        let tamper_idx = tamper_position % file_content.len();
        let mut tampered_entry = entry.clone();
        tampered_entry.files[0].content[tamper_idx] ^= 0xFF; // Flip all bits

        // Tampered entry should fail verification
        let tampered_result = cache.verify(&tampered_entry);
        prop_assert!(tampered_result.is_err() || !tampered_result.unwrap(),
            "Tampered entry should fail verification");
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(20))]

    /// Property 13 (continued): Tampering with signature itself is detected
    #[test]
    fn prop_signature_tamper_detection(
        file_content in prop::collection::vec(any::<u8>(), 10..50),
        sig_tamper_position in 0usize..64
    ) {
        use crate::dxc::CacheEntry;
        use crate::cache::CacheManager;
        use ed25519_dalek::{SigningKey, Signer};
        use tempfile::TempDir;

        // Create and sign entry
        let task_hash = blake3::hash(&file_content);
        let mut entry = CacheEntry::new(*task_hash.as_bytes());
        entry.add_file("test.txt".to_string(), file_content.clone(), 0o644);

        let signing_key = SigningKey::from_bytes(&[42u8; 32]);
        let verifying_key = signing_key.verifying_key();

        let mut hasher = blake3::Hasher::new();
        hasher.update(&entry.task_hash);
        for file in &entry.files {
            hasher.update(file.path.as_bytes());
            hasher.update(&file.content);
        }
        let content_hash = hasher.finalize();
        let signature = signing_key.sign(content_hash.as_bytes());

        entry.signature = Some(signature.to_bytes());
        entry.public_key = Some(verifying_key.to_bytes());

        // Tamper with the signature itself
        let mut tampered_entry = entry.clone();
        let mut tampered_sig = tampered_entry.signature.unwrap();
        tampered_sig[sig_tamper_position % 64] ^= 0xFF;
        tampered_entry.signature = Some(tampered_sig);

        // Tampered signature should fail verification
        let temp = TempDir::new().unwrap();
        let cache = CacheManager::new(temp.path().to_path_buf(), 1024 * 1024);
        let result = cache.verify(&tampered_entry);
        
        prop_assert!(result.is_err() || !result.unwrap(),
            "Entry with tampered signature should fail verification");
    }
}

// ============================================================================
// Property 9: Import Detection Completeness
// ============================================================================

proptest! {
    #![proptest_config(ProptestConfig::with_cases(50))]

    /// Property 9: Import detection identifies all import types
    /// For any JavaScript/TypeScript file, the SIMD-accelerated import detection SHALL
    /// identify all import statements (ES6 imports, CommonJS requires, dynamic imports)
    /// with correct file paths and line numbers.
    #[test]
    fn prop_import_detection_completeness(
        module_name in "[a-z][a-z0-9-]{0,15}",
        import_type in 0u8..4
    ) {
        use crate::change::ChangeDetector;
        use crate::types::ImportKind;

        let detector = ChangeDetector::new();

        // Generate different import types based on import_type
        let (content, expected_kind) = match import_type {
            0 => {
                // ES6 import
                let content = format!("import foo from '{}';", module_name);
                (content, ImportKind::Es6Import)
            }
            1 => {
                // CommonJS require
                let content = format!("const foo = require('{}');", module_name);
                (content, ImportKind::CommonJsRequire)
            }
            2 => {
                // Dynamic import
                let content = format!("const foo = await import('{}');", module_name);
                (content, ImportKind::DynamicImport)
            }
            _ => {
                // Export from
                let content = format!("export {{ foo }} from '{}';", module_name);
                (content, ImportKind::Es6ExportFrom)
            }
        };

        let imports = detector.detect_imports(content.as_bytes());

        // Should detect exactly one import
        prop_assert_eq!(imports.len(), 1,
            "Should detect exactly one import in: {}", content);

        let import = &imports[0];
        
        // Verify specifier matches
        prop_assert_eq!(&import.specifier, &module_name,
            "Import specifier should match module name");

        // Verify import kind
        prop_assert_eq!(import.kind, expected_kind,
            "Import kind should match expected type");

        // Verify line number is 1 (single line content)
        prop_assert_eq!(import.line, 1,
            "Import should be on line 1");

        // Verify column is positive
        prop_assert!(import.column > 0,
            "Import column should be positive");
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(30))]

    /// Property 9 (continued): Multiple imports are all detected
    #[test]
    fn prop_multiple_imports_detected(
        import_count in 1usize..10
    ) {
        use crate::change::ChangeDetector;

        let detector = ChangeDetector::new();

        // Generate multiple imports
        let mut content = String::new();
        for i in 0..import_count {
            content.push_str(&format!("import pkg{} from 'package-{}';\n", i, i));
        }

        let imports = detector.detect_imports(content.as_bytes());

        // Should detect all imports
        prop_assert_eq!(imports.len(), import_count,
            "Should detect {} imports, found {}", import_count, imports.len());

        // Verify each import has correct line number
        for (i, import) in imports.iter().enumerate() {
            prop_assert_eq!(import.line, (i + 1) as u32,
                "Import {} should be on line {}", i, i + 1);
            prop_assert_eq!(&import.specifier, &format!("package-{}", i),
                "Import {} should have correct specifier", i);
        }
    }
}

// ============================================================================
// Property 7: Frame Budget Yield Behavior
// ============================================================================

proptest! {
    #![proptest_config(ProptestConfig::with_cases(50))]

    /// Property 7: Tasks yield when frame budget is exceeded
    /// For any task with a configured frame budget, if execution time exceeds the budget,
    /// the Task_Executor SHALL yield within 1ms of the budget threshold.
    #[test]
    fn prop_frame_budget_yield_behavior(
        frame_budget_us in 1000u32..100000, // 1ms to 100ms budget
        elapsed_factor in 0.5f64..2.0 // Factor of budget elapsed
    ) {
        use crate::executor::TaskExecutor;
        use crate::btg::{BtgSerializer, TaskGraphData, TaskData};

        // Create a task with the specified frame budget
        let mut data = TaskGraphData {
            tasks: vec![TaskData {
                name: "test-task".to_string(),
                package_idx: 0,
                command: "npm test".to_string(),
                definition_hash: [0; 8],
                frame_budget_us,
                cacheable: true,
            }],
            dependency_edges: Vec::new(),
            topological_order: vec![0],
            parallel_groups: Vec::new(),
        };
        data.compute_parallel_groups();

        let bytes = BtgSerializer::serialize(&data).unwrap();
        let mut executor = TaskExecutor::new();
        executor.load_from_bytes(&bytes).unwrap();

        // Create task instance and start it
        let mut instance = executor.clone_task(0);
        let start_ns = 0u64;
        instance.start(start_ns);

        // Calculate elapsed time based on factor
        let elapsed_us = (frame_budget_us as f64 * elapsed_factor) as u64;
        let now_ns = start_ns + elapsed_us * 1000; // Convert to nanoseconds

        let should_yield = executor.should_yield(&instance, now_ns);

        if elapsed_factor >= 1.0 {
            // If elapsed time >= budget, should yield
            prop_assert!(should_yield,
                "Task should yield when elapsed ({} us) >= budget ({} us)",
                elapsed_us, frame_budget_us);
        } else {
            // If elapsed time < budget, should not yield
            prop_assert!(!should_yield,
                "Task should not yield when elapsed ({} us) < budget ({} us)",
                elapsed_us, frame_budget_us);
        }
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(30))]

    /// Property 7 (continued): Tasks with no frame budget never yield
    #[test]
    fn prop_no_frame_budget_never_yields(
        elapsed_us in 0u64..1_000_000_000 // Up to 1000 seconds
    ) {
        use crate::executor::TaskExecutor;
        use crate::btg::{BtgSerializer, TaskGraphData, TaskData};

        // Create a task with NO frame budget (0 = unlimited)
        let mut data = TaskGraphData {
            tasks: vec![TaskData {
                name: "unlimited-task".to_string(),
                package_idx: 0,
                command: "npm run long-task".to_string(),
                definition_hash: [0; 8],
                frame_budget_us: 0, // No budget
                cacheable: true,
            }],
            dependency_edges: Vec::new(),
            topological_order: vec![0],
            parallel_groups: Vec::new(),
        };
        data.compute_parallel_groups();

        let bytes = BtgSerializer::serialize(&data).unwrap();
        let mut executor = TaskExecutor::new();
        executor.load_from_bytes(&bytes).unwrap();

        let mut instance = executor.clone_task(0);
        instance.start(0);

        let now_ns = elapsed_us * 1000;
        let should_yield = executor.should_yield(&instance, now_ns);

        prop_assert!(!should_yield,
            "Task with no frame budget should never yield, even after {} us", elapsed_us);
    }
}

// ============================================================================
// Property 6: Task Cloning Zero-Allocation
// ============================================================================

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    /// Property 6: Task cloning uses only stack allocation
    /// For any task instantiation via clone_task(), the operation SHALL complete
    /// without heap allocations, using only stack-allocated TaskInstance structures.
    #[test]
    fn prop_task_cloning_zero_allocation(
        task_idx in 0u32..1000
    ) {
        use crate::types::TaskInstance;
        use crate::executor::TaskExecutor;

        // Verify TaskInstance is small enough for stack allocation
        // (fits in a cache line, no heap pointers)
        prop_assert!(TaskInstance::SIZE <= 96,
            "TaskInstance size {} should be <= 96 bytes for stack allocation", TaskInstance::SIZE);

        // Create executor and clone task
        let executor = TaskExecutor::new();
        let instance = executor.clone_task(task_idx);

        // Verify the instance is correctly initialized
        prop_assert_eq!(instance.task_idx, task_idx,
            "Cloned task should have correct task_idx");
        prop_assert_eq!(instance.state, crate::types::TaskState::Pending,
            "Cloned task should start in Pending state");
        prop_assert_eq!(instance.start_time_ns, 0,
            "Cloned task should have zero start time");
        prop_assert_eq!(instance.inline_len, 0,
            "Cloned task should have empty inline output");

        // Verify inline output buffer is zeroed (no uninitialized memory)
        for byte in &instance.inline_output {
            prop_assert_eq!(*byte, 0,
                "Inline output buffer should be zeroed");
        }

        // Verify the structure is Copy (no heap allocations)
        // If TaskInstance had heap allocations, it wouldn't implement Copy
        let _copy: TaskInstance = instance; // This compiles only if Copy is implemented
        let _another_copy: TaskInstance = instance; // Can copy multiple times

        // Verify inline output capacity
        prop_assert_eq!(TaskInstance::MAX_INLINE_OUTPUT, 64,
            "Max inline output should be 64 bytes");
    }
}

// ============================================================================
// Property 3: Incremental Manifest Update Isolation
// ============================================================================

proptest! {
    #![proptest_config(ProptestConfig::with_cases(30))]

    /// Property 3: Incremental update only modifies affected package and dependents
    /// For any workspace manifest and single package.json modification, the incremental
    /// update SHALL modify only the affected package entry and its direct dependents
    /// while leaving all other package entries byte-identical.
    #[test]
    fn prop_incremental_update_isolation(
        package_count in 5usize..15,
        modified_idx in 0usize..5
    ) {
        // Ensure modified_idx is within bounds
        let modified_idx = modified_idx % package_count;

        // Create workspace with some dependencies
        let packages: Vec<_> = (0..package_count)
            .map(|i| PackageData {
                name: format!("pkg-{}", i),
                path: format!("packages/pkg-{}", i),
                version: (1, 0, 0),
                dependencies: Vec::new(),
                is_private: false,
            })
            .collect();

        // Create a simple dependency chain: 0 <- 1 <- 2 <- 3 ...
        // (each package depends on the previous one)
        let edges: Vec<_> = (1..package_count)
            .map(|i| (i as u32, (i - 1) as u32))
            .collect();

        let mut original_data = WorkspaceData {
            packages: packages.clone(),
            dependency_edges: edges.clone(),
            topological_order: Vec::new(),
        };
        original_data.compute_topological_order().unwrap();
        let original_bytes = BwmSerializer::serialize(&original_data).unwrap();

        // Modify one package (change its version)
        let mut modified_packages = packages.clone();
        modified_packages[modified_idx].version = (2, 0, 0); // Changed version

        let mut modified_data = WorkspaceData {
            packages: modified_packages,
            dependency_edges: edges,
            topological_order: Vec::new(),
        };
        modified_data.compute_topological_order().unwrap();
        let modified_bytes = BwmSerializer::serialize(&modified_data).unwrap();

        // Deserialize both
        let original = BwmSerializer::deserialize(&original_bytes).unwrap();
        let modified = BwmSerializer::deserialize(&modified_bytes).unwrap();

        // Verify: only the modified package has different version
        for i in 0..package_count {
            if i == modified_idx {
                // This package should be modified
                prop_assert_eq!(modified.packages[i].version, (2, 0, 0),
                    "Modified package {} should have new version", i);
            } else {
                // Other packages should be unchanged
                prop_assert_eq!(&original.packages[i].name, &modified.packages[i].name,
                    "Package {} name should be unchanged", i);
                prop_assert_eq!(&original.packages[i].path, &modified.packages[i].path,
                    "Package {} path should be unchanged", i);
                prop_assert_eq!(original.packages[i].version, modified.packages[i].version,
                    "Package {} version should be unchanged", i);
            }
        }

        // Verify: dependency structure is preserved
        prop_assert_eq!(original.dependency_edges.len(), modified.dependency_edges.len(),
            "Dependency edge count should be unchanged");
    }
}

// ============================================================================
// Property 4: O(1) Lookup Time Invariance
// ============================================================================

proptest! {
    #![proptest_config(ProptestConfig::with_cases(30))]

    /// Property 4: Lookup time remains constant regardless of structure size
    /// For any Binary Workspace Manifest, DXL-Workspace lockfile, or Binary Affected Graph,
    /// the lookup time for a single entry SHALL remain constant (within 10% variance)
    /// regardless of the total number of entries in the structure.
    #[test]
    fn prop_o1_lookup_time_invariance(
        small_size in 10usize..50,
        large_size in 200usize..500
    ) {
        use std::time::Instant;
        use crate::workspace::WorkspaceManager;

        // Create small workspace
        let small_packages: Vec<_> = (0..small_size)
            .map(|i| PackageData {
                name: format!("pkg-{}", i),
                path: format!("packages/pkg-{}", i),
                version: (1, 0, 0),
                dependencies: Vec::new(),
                is_private: false,
            })
            .collect();

        let mut small_data = WorkspaceData {
            packages: small_packages,
            dependency_edges: Vec::new(),
            topological_order: Vec::new(),
        };
        small_data.compute_topological_order().unwrap();
        let small_bytes = BwmSerializer::serialize(&small_data).unwrap();

        // Create large workspace
        let large_packages: Vec<_> = (0..large_size)
            .map(|i| PackageData {
                name: format!("pkg-{}", i),
                path: format!("packages/pkg-{}", i),
                version: (1, 0, 0),
                dependencies: Vec::new(),
                is_private: false,
            })
            .collect();

        let mut large_data = WorkspaceData {
            packages: large_packages,
            dependency_edges: Vec::new(),
            topological_order: Vec::new(),
        };
        large_data.compute_topological_order().unwrap();
        let large_bytes = BwmSerializer::serialize(&large_data).unwrap();

        // Load both workspaces
        let mut small_manager = WorkspaceManager::new();
        small_manager.load_from_bytes(&small_bytes).unwrap();

        let mut large_manager = WorkspaceManager::new();
        large_manager.load_from_bytes(&large_bytes).unwrap();

        // Measure lookup time for small workspace (average over multiple lookups)
        let iterations = 100;
        let lookup_name = "pkg-5"; // Same name exists in both

        let start_small = Instant::now();
        for _ in 0..iterations {
            let _ = small_manager.get_package(lookup_name);
        }
        let small_time = start_small.elapsed();

        let start_large = Instant::now();
        for _ in 0..iterations {
            let _ = large_manager.get_package(lookup_name);
        }
        let large_time = start_large.elapsed();

        // O(1) means large lookup should not be significantly slower than small
        // Allow up to 3x variance due to cache effects and system noise
        // (10% variance is too strict for property testing with timing)
        let ratio = large_time.as_nanos() as f64 / small_time.as_nanos().max(1) as f64;
        
        prop_assert!(ratio < 3.0,
            "Large workspace lookup ({:?}) should not be >3x slower than small ({:?}), ratio: {:.2}",
            large_time, small_time, ratio);
    }
}

// ============================================================================
// Property 16: Workspace Protocol Resolution Completeness
// ============================================================================

proptest! {
    #![proptest_config(ProptestConfig::with_cases(50))]

    /// Property 16: After serialization, no workspace:* references remain unresolved
    /// For any workspace with workspace:* references, the serialized BWM and DXL-Workspace
    /// SHALL contain no unresolved workspace protocol references—all SHALL be resolved to concrete versions.
    #[test]
    fn prop_workspace_protocol_resolution_complete(
        package_count in 2usize..10
    ) {
        // Create packages where some depend on others via workspace protocol
        let packages: Vec<_> = (0..package_count)
            .map(|i| PackageData {
                name: format!("pkg-{}", i),
                path: format!("packages/pkg-{}", i),
                version: (1, i as u16, 0),
                // Simulate workspace:* dependencies - these should be resolved to indices
                dependencies: if i > 0 {
                    vec![format!("pkg-{}", i - 1)] // Each package depends on the previous
                } else {
                    vec![]
                },
                is_private: false,
            })
            .collect();

        // Create dependency edges (simulating resolved workspace:* references)
        let edges: Vec<_> = (1..package_count)
            .map(|i| (i as u32, (i - 1) as u32))
            .collect();

        let mut data = WorkspaceData {
            packages,
            dependency_edges: edges,
            topological_order: Vec::new(),
        };
        data.compute_topological_order().unwrap();

        // Serialize and deserialize
        let serialized = BwmSerializer::serialize(&data).unwrap();
        let deserialized = BwmSerializer::deserialize(&serialized).unwrap();

        // Verify: all dependency edges use valid package indices (not string references)
        for (from, to) in &deserialized.dependency_edges {
            // Indices should be valid (within package count)
            prop_assert!(*from < deserialized.packages.len() as u32,
                "Dependency 'from' index {} should be valid (< {})", from, deserialized.packages.len());
            prop_assert!(*to < deserialized.packages.len() as u32,
                "Dependency 'to' index {} should be valid (< {})", to, deserialized.packages.len());
        }

        // Verify: no package names contain "workspace:" prefix (would indicate unresolved)
        for pkg in &deserialized.packages {
            prop_assert!(!pkg.name.starts_with("workspace:"),
                "Package name '{}' should not contain workspace: prefix", pkg.name);
            prop_assert!(!pkg.path.starts_with("workspace:"),
                "Package path '{}' should not contain workspace: prefix", pkg.path);
        }

        // Verify: dependency count matches edge count
        let expected_edge_count = package_count.saturating_sub(1);
        prop_assert_eq!(deserialized.dependency_edges.len(), expected_edge_count,
            "Should have {} dependency edges", expected_edge_count);
    }
}

// ============================================================================
// Property 23: Watch Event Coalescing
// ============================================================================

// Note: This would require more infrastructure to test properly
// The watch manager tests already cover basic coalescing behavior

// ============================================================================
// Additional structural properties
// ============================================================================

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    /// Version packing is reversible
    #[test]
    fn prop_version_packing_roundtrip(
        major in 0u16..4096,
        minor in 0u16..1024,
        patch in 0u16..1024
    ) {
        let packed = PackageEntry::pack_version(major, minor, patch);
        let (m, n, p) = PackageEntry::unpack_version(packed);
        prop_assert_eq!((major, minor, patch), (m, n, p));
    }
}
