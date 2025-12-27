//! Tests for fixture cache

use super::*;
use proptest::prelude::*;
use tempfile::TempDir;

#[test]
fn test_cache_creation() {
    let temp_dir = TempDir::new().unwrap();
    let cache = FixtureCache::new(temp_dir.path()).unwrap();
    assert!(cache.is_empty());
    assert_eq!(cache.len(), 0);
}

#[test]
fn test_store_and_load() {
    let temp_dir = TempDir::new().unwrap();
    let mut cache = FixtureCache::new(temp_dir.path()).unwrap();

    let id = FixtureId::new(12345);
    let source = "def my_fixture(): return 42";
    let source_hash = FixtureCache::hash_source(source);
    let value: i32 = 42;

    // Store
    cache.store(id, source_hash, &value).unwrap();
    assert!(cache.contains(id));

    // Load
    let loaded: i32 = cache.load(id).unwrap();
    assert_eq!(loaded, value);
}

#[test]
fn test_get_or_create_cached() {
    let temp_dir = TempDir::new().unwrap();
    let mut cache = FixtureCache::new(temp_dir.path()).unwrap();

    let id = FixtureId::new(12345);
    let source = "def my_fixture(): return 42";
    let mut call_count = 0;

    // First call - creates value
    let value1: i32 = cache
        .get_or_create(id, source, || {
            call_count += 1;
            42
        })
        .unwrap();
    assert_eq!(value1, 42);
    assert_eq!(call_count, 1);

    // Second call - uses cached value
    let value2: i32 = cache
        .get_or_create(id, source, || {
            call_count += 1;
            99
        })
        .unwrap();
    assert_eq!(value2, 42);
    assert_eq!(call_count, 1); // Not called again
}

#[test]
fn test_cache_invalidation_on_source_change() {
    let temp_dir = TempDir::new().unwrap();
    let mut cache = FixtureCache::new(temp_dir.path()).unwrap();

    let id = FixtureId::new(12345);
    let source1 = "def my_fixture(): return 42";
    let source2 = "def my_fixture(): return 99"; // Changed source

    // First call with source1
    let value1: i32 = cache.get_or_create(id, source1, || 42).unwrap();
    assert_eq!(value1, 42);

    // Second call with different source - should recreate
    let value2: i32 = cache.get_or_create(id, source2, || 99).unwrap();
    assert_eq!(value2, 99);
}

#[test]
fn test_invalidate() {
    let temp_dir = TempDir::new().unwrap();
    let mut cache = FixtureCache::new(temp_dir.path()).unwrap();

    let id = FixtureId::new(12345);
    let source = "def my_fixture(): return 42";
    let source_hash = FixtureCache::hash_source(source);

    cache.store(id, source_hash, &42i32).unwrap();
    assert!(cache.contains(id));

    cache.invalidate(id).unwrap();
    assert!(!cache.contains(id));
}

#[test]
fn test_clear() {
    let temp_dir = TempDir::new().unwrap();
    let mut cache = FixtureCache::new(temp_dir.path()).unwrap();

    let source = "def fixture(): pass";
    let hash = FixtureCache::hash_source(source);

    cache.store(FixtureId::new(1), hash, &1i32).unwrap();
    cache.store(FixtureId::new(2), hash, &2i32).unwrap();
    cache.store(FixtureId::new(3), hash, &3i32).unwrap();

    assert_eq!(cache.len(), 3);

    cache.clear().unwrap();
    assert!(cache.is_empty());
}

#[test]
fn test_complex_value() {
    let temp_dir = TempDir::new().unwrap();
    let mut cache = FixtureCache::new(temp_dir.path()).unwrap();

    #[derive(Debug, Clone, PartialEq, Serialize, serde::Deserialize)]
    struct ComplexFixture {
        name: String,
        values: Vec<i32>,
        nested: Option<Box<ComplexFixture>>,
    }

    let id = FixtureId::new(12345);
    let source = "def complex_fixture(): ...";
    let source_hash = FixtureCache::hash_source(source);

    let value = ComplexFixture {
        name: "test".to_string(),
        values: vec![1, 2, 3, 4, 5],
        nested: Some(Box::new(ComplexFixture {
            name: "nested".to_string(),
            values: vec![10, 20],
            nested: None,
        })),
    };

    cache.store(id, source_hash, &value).unwrap();
    let loaded: ComplexFixture = cache.load(id).unwrap();
    assert_eq!(loaded, value);
}

#[test]
fn test_persistence_across_instances() {
    let temp_dir = TempDir::new().unwrap();
    let id = FixtureId::new(12345);
    let source = "def my_fixture(): return 42";
    let source_hash = FixtureCache::hash_source(source);

    // Store with first instance
    {
        let mut cache = FixtureCache::new(temp_dir.path()).unwrap();
        cache.store(id, source_hash, &42i32).unwrap();
    }

    // Load with new instance
    {
        let mut cache = FixtureCache::new(temp_dir.path()).unwrap();
        assert!(cache.is_valid(id, source_hash));
        let loaded: i32 = cache.load(id).unwrap();
        assert_eq!(loaded, 42);
    }
}

// Property tests

/// Feature: dx-py-test-runner, Property 14: Fixture Cache Round-Trip
/// Validates: Requirements 6.1, 6.3
///
/// For any serializable fixture value, storing it in the Fixture_Cache
/// and retrieving it SHALL produce an equivalent value. When the fixture
/// function changes (different hash), the cache SHALL be invalidated
/// and the fixture recreated.
proptest! {
    #[test]
    fn prop_fixture_cache_roundtrip(
        value in any::<i64>(),
        source in "[a-z]{10,50}",
    ) {
        let temp_dir = TempDir::new().unwrap();
        let mut cache = FixtureCache::new(temp_dir.path()).unwrap();

        let id = FixtureId::new(blake3::hash(source.as_bytes()).as_bytes()[0..8]
            .iter()
            .fold(0u64, |acc, &b| (acc << 8) | b as u64));
        let source_hash = FixtureCache::hash_source(&source);

        // Store and load
        cache.store(id, source_hash, &value).unwrap();
        let loaded: i64 = cache.load(id).unwrap();
        prop_assert_eq!(loaded, value);
    }

    #[test]
    fn prop_fixture_invalidation_on_source_change(
        value1 in any::<i32>(),
        value2 in any::<i32>(),
        source1 in "[a-z]{10,30}",
        source2 in "[A-Z]{10,30}", // Different pattern to ensure different source
    ) {
        let temp_dir = TempDir::new().unwrap();
        let mut cache = FixtureCache::new(temp_dir.path()).unwrap();

        let id = FixtureId::new(12345);

        // Store with source1
        let result1: i32 = cache.get_or_create(id, &source1, || value1).unwrap();
        prop_assert_eq!(result1, value1);

        // Get with source2 - should create new value
        let result2: i32 = cache.get_or_create(id, &source2, || value2).unwrap();
        prop_assert_eq!(result2, value2);
    }

    #[test]
    fn prop_fixture_cache_consistency(
        values in prop::collection::vec(any::<i32>(), 1..10),
        source in "[a-z]{10,30}",
    ) {
        let temp_dir = TempDir::new().unwrap();
        let mut cache = FixtureCache::new(temp_dir.path()).unwrap();
        let source_hash = FixtureCache::hash_source(&source);

        // Store multiple fixtures
        for (i, &value) in values.iter().enumerate() {
            let id = FixtureId::new(i as u64);
            cache.store(id, source_hash, &value).unwrap();
        }

        // Verify all can be loaded correctly
        for (i, &expected) in values.iter().enumerate() {
            let id = FixtureId::new(i as u64);
            let loaded: i32 = cache.load(id).unwrap();
            prop_assert_eq!(loaded, expected);
        }
    }
}
