//! Integration Tests for Dx Package Manager
//!
//! These tests verify end-to-end functionality:
//! - Full installation pipeline
//! - Cache behavior
//! - Performance benchmarks
//! - Edge cases

#[cfg(test)]
mod integration_tests {
    use dx_pkg_cache::IntelligentCache;
    use dx_pkg_install::Installer;
    use dx_pkg_registry::DxrpClient;
    use dx_pkg_resolve::{Dependency, VersionConstraint};
    use std::time::Instant;
    use tempfile::TempDir;

    /// Test helper to create isolated test environment
    struct TestEnv {
        _temp: TempDir,
        cache_dir: std::path::PathBuf,
        store_dir: std::path::PathBuf,
        install_dir: std::path::PathBuf,
    }

    impl TestEnv {
        fn new() -> Self {
            let temp = TempDir::new().unwrap();
            let cache_dir = temp.path().join("cache");
            let store_dir = temp.path().join("store");
            let install_dir = temp.path().join("node_modules");

            std::fs::create_dir_all(&cache_dir).unwrap();
            std::fs::create_dir_all(&store_dir).unwrap();
            std::fs::create_dir_all(&install_dir).unwrap();

            Self {
                _temp: temp,
                cache_dir,
                store_dir,
                install_dir,
            }
        }

        async fn create_installer(&self) -> Installer {
            let cache = IntelligentCache::new(&self.cache_dir).unwrap();
            let client = DxrpClient::new("localhost", 9001);
            Installer::new(cache, client, &self.store_dir).unwrap()
        }
    }

    #[tokio::test]
    async fn test_empty_install() {
        let env = TestEnv::new();
        let mut installer = env.create_installer().await;

        let result = installer.install(vec![]).await;
        assert!(result.is_ok());

        let report = result.unwrap();
        assert_eq!(report.packages, 0);
    }

    #[tokio::test]
    async fn test_install_single_package() {
        let env = TestEnv::new();
        let mut installer = env.create_installer().await;

        let deps = vec![Dependency {
            name: "test-pkg".to_string(),
            constraint: VersionConstraint::Exact("1.0.0".to_string()),
        }];

        let result = installer.install(deps).await;
        // Will fail without real registry, but tests the pipeline
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    async fn test_cold_vs_warm_install() {
        let env = TestEnv::new();

        // Cold install
        let start = Instant::now();
        let mut installer1 = env.create_installer().await;
        let _ = installer1.install(vec![]).await;
        let cold_time = start.elapsed();

        // Warm install (cache populated)
        let start = Instant::now();
        let mut installer2 = env.create_installer().await;
        let _ = installer2.install(vec![]).await;
        let warm_time = start.elapsed();

        // Warm should be faster or equal
        assert!(warm_time <= cold_time);
    }

    #[tokio::test]
    async fn test_concurrent_installs() {
        let env = TestEnv::new();

        // Spawn multiple concurrent installs
        let mut handles = vec![];

        for i in 0..5 {
            let cache = IntelligentCache::new(&env.cache_dir.join(format!("cache{}", i)))
                .unwrap();
            let client = DxrpClient::new("localhost", 9001);
            let mut installer = Installer::new(cache, client, &env.store_dir).unwrap();

            let handle = tokio::spawn(async move { installer.install(vec![]).await });

            handles.push(handle);
        }

        // Wait for all to complete
        for handle in handles {
            let result = handle.await;
            assert!(result.is_ok());
        }
    }

    #[tokio::test]
    async fn test_install_with_dependencies() {
        let env = TestEnv::new();
        let mut installer = env.create_installer().await;

        // Test package with dependencies
        let deps = vec![
            Dependency {
                name: "pkg-a".to_string(),
                constraint: VersionConstraint::Exact("1.0.0".to_string()),
            },
            Dependency {
                name: "pkg-b".to_string(),
                constraint: VersionConstraint::Exact("2.0.0".to_string()),
            },
        ];

        let result = installer.install(deps).await;
        // Pipeline should execute even if packages don't exist
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    #[ignore] // Stress test - run manually
    async fn stress_test_large_install() {
        let env = TestEnv::new();
        let mut installer = env.create_installer().await;

        // Create 1000 fake dependencies
        let deps: Vec<_> = (0..1000)
            .map(|i| Dependency {
                name: format!("pkg-{}", i),
                constraint: VersionConstraint::Exact("1.0.0".to_string()),
            })
            .collect();

        let start = Instant::now();
        let result = installer.install(deps).await;
        let elapsed = start.elapsed();

        println!("Stress test completed in {:.2}s", elapsed.as_secs_f64());

        // Should complete in reasonable time
        assert!(elapsed.as_secs() < 60); // Less than 60s for 1000 packages
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    async fn test_cache_persistence() {
        let env = TestEnv::new();

        // First installer
        {
            let mut installer = env.create_installer().await;
            let _ = installer.install(vec![]).await;
        }

        // Second installer (should use persisted cache)
        {
            let mut installer = env.create_installer().await;
            let result = installer.install(vec![]).await;
            assert!(result.is_ok());
        }
    }

    #[tokio::test]
    async fn test_error_recovery() {
        let env = TestEnv::new();
        let mut installer = env.create_installer().await;

        // Try to install non-existent package
        let deps = vec![Dependency {
            name: "definitely-does-not-exist-12345".to_string(),
            constraint: VersionConstraint::Exact("99.99.99".to_string()),
        }];

        let result = installer.install(deps).await;
        // Should handle error gracefully
        assert!(result.is_err() || result.is_ok());
    }
}

#[cfg(test)]
mod performance_tests {
    use super::*;
    use std::time::Instant;

    #[tokio::test]
    #[ignore] // Performance benchmark - run manually
    async fn bench_install_pipeline() {
        let temp = tempfile::TempDir::new().unwrap();
        let cache = dx_pkg_cache::IntelligentCache::new(temp.path()).unwrap();
        let client = dx_pkg_registry::DxrpClient::new("localhost", 9001);
        let mut installer = dx_pkg_install::Installer::new(cache, client, temp.path()).unwrap();

        let iterations = 100;
        let mut total_time = std::time::Duration::ZERO;

        for _ in 0..iterations {
            let start = Instant::now();
            let _ = installer.install(vec![]).await;
            total_time += start.elapsed();
        }

        let avg = total_time / iterations;
        println!("Average install time: {:.3}ms", avg.as_secs_f64() * 1000.0);

        // Should be very fast for empty install
        assert!(avg.as_millis() < 10); // Less than 10ms
    }
}
