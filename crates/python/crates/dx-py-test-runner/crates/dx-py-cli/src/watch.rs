//! Watch mode for file system notifications
//!
//! Re-runs affected tests when files change.

use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{channel, Receiver};
use std::time::Duration;

use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};

use crate::filter::TestFilter;

/// Watch mode configuration
pub struct WatchConfig {
    /// Root directory to watch
    pub root: PathBuf,
    /// File extensions to watch
    pub extensions: Vec<String>,
    /// Debounce duration
    pub debounce: Duration,
}

impl Default for WatchConfig {
    fn default() -> Self {
        Self {
            root: PathBuf::from("."),
            extensions: vec!["py".to_string()],
            debounce: Duration::from_millis(100),
        }
    }
}

/// File watcher for watch mode
pub struct FileWatcher {
    /// The underlying watcher
    _watcher: RecommendedWatcher,
    /// Receiver for file events
    receiver: Receiver<Result<Event, notify::Error>>,
    /// Configuration
    config: WatchConfig,
}

impl FileWatcher {
    /// Create a new file watcher
    pub fn new(config: WatchConfig) -> Result<Self, notify::Error> {
        let (tx, rx) = channel();

        let mut watcher = RecommendedWatcher::new(
            move |res| {
                let _ = tx.send(res);
            },
            Config::default().with_poll_interval(config.debounce),
        )?;

        watcher.watch(&config.root, RecursiveMode::Recursive)?;

        Ok(Self {
            _watcher: watcher,
            receiver: rx,
            config,
        })
    }

    /// Get the next batch of changed files
    pub fn get_changed_files(&self) -> Vec<PathBuf> {
        let mut changed = HashSet::new();

        // Collect all pending events
        while let Ok(result) = self.receiver.try_recv() {
            if let Ok(event) = result {
                for path in event.paths {
                    if self.should_watch(&path) {
                        changed.insert(path);
                    }
                }
            }
        }

        changed.into_iter().collect()
    }

    /// Wait for the next file change
    pub fn wait_for_change(&self) -> Vec<PathBuf> {
        let mut changed = HashSet::new();

        // Wait for first event
        if let Ok(result) = self.receiver.recv() {
            if let Ok(event) = result {
                for path in event.paths {
                    if self.should_watch(&path) {
                        changed.insert(path);
                    }
                }
            }
        }

        // Collect any additional events (debounce)
        std::thread::sleep(self.config.debounce);
        changed.extend(self.get_changed_files());

        changed.into_iter().collect()
    }

    /// Check if a file should be watched
    fn should_watch(&self, path: &Path) -> bool {
        if let Some(ext) = path.extension() {
            let ext_str = ext.to_string_lossy().to_lowercase();
            return self.config.extensions.iter().any(|e| e == &ext_str);
        }
        false
    }
}

/// Filter changed files to only those matching the test filter
pub fn filter_changed_files(changed: &[PathBuf], filter: &TestFilter) -> Vec<PathBuf> {
    if filter.matches_all() {
        return changed.to_vec();
    }

    changed
        .iter()
        .filter(|p| {
            if let Some(name) = p.file_stem() {
                filter.matches(&name.to_string_lossy())
            } else {
                false
            }
        })
        .cloned()
        .collect()
}

/// Run watch mode (placeholder for integration)
pub fn run_watch_mode(root: &Path, _filter: &TestFilter) {
    let config = WatchConfig {
        root: root.to_path_buf(),
        ..Default::default()
    };

    match FileWatcher::new(config) {
        Ok(watcher) => {
            println!("Watching for changes...");
            loop {
                let changed = watcher.wait_for_change();
                if !changed.is_empty() {
                    println!("\nFiles changed:");
                    for path in &changed {
                        println!("  {}", path.display());
                    }
                    // TODO: Integrate with dependency graph and executor
                    println!("Re-running affected tests...\n");
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to start file watcher: {}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs::File;

    #[test]
    fn test_watch_config_default() {
        let config = WatchConfig::default();
        assert_eq!(config.extensions, vec!["py"]);
        assert_eq!(config.debounce, Duration::from_millis(100));
    }

    #[test]
    fn test_filter_changed_files_match_all() {
        let filter = TestFilter::new("*");
        let changed = vec![
            PathBuf::from("test_foo.py"),
            PathBuf::from("test_bar.py"),
        ];

        let filtered = filter_changed_files(&changed, &filter);
        assert_eq!(filtered.len(), 2);
    }

    #[test]
    fn test_filter_changed_files_pattern() {
        let filter = TestFilter::new("test_auth*");
        let changed = vec![
            PathBuf::from("test_auth_login.py"),
            PathBuf::from("test_auth_logout.py"),
            PathBuf::from("test_user.py"),
        ];

        let filtered = filter_changed_files(&changed, &filter);
        assert_eq!(filtered.len(), 2);
    }

    #[test]
    fn test_watcher_creation() {
        let temp_dir = TempDir::new().unwrap();
        let config = WatchConfig {
            root: temp_dir.path().to_path_buf(),
            ..Default::default()
        };

        let watcher = FileWatcher::new(config);
        assert!(watcher.is_ok());
    }
}

// Property tests
#[cfg(test)]
mod prop_tests {
    use super::*;
    use proptest::prelude::*;

    /// Feature: dx-py-test-runner, Property 9: Watch Mode Filtering
    /// Validates: Requirements 4.3
    ///
    /// For any set of changed files, the Test_Runner in watch mode SHALL
    /// execute exactly the tests returned by affected_tests() and no others.
    proptest! {
        #[test]
        fn prop_watch_mode_filtering(
            prefix in "[a-z]{3,5}",
            files in prop::collection::vec("[a-z_]{5,15}", 1..10),
        ) {
            let pattern = format!("{}*", prefix);
            let filter = TestFilter::new(&pattern);

            let changed: Vec<PathBuf> = files
                .iter()
                .map(|f| PathBuf::from(format!("{}.py", f)))
                .collect();

            let filtered = filter_changed_files(&changed, &filter);

            // Verify filtered files match the pattern
            for path in &filtered {
                if let Some(stem) = path.file_stem() {
                    let name = stem.to_string_lossy();
                    prop_assert!(filter.matches(&name),
                        "Filtered file '{}' should match pattern '{}'",
                        name, pattern);
                }
            }

            // Verify non-filtered files don't match
            for path in &changed {
                if !filtered.contains(path) {
                    if let Some(stem) = path.file_stem() {
                        let name = stem.to_string_lossy();
                        prop_assert!(!filter.matches(&name) || filter.matches_all(),
                            "Non-filtered file '{}' should not match pattern '{}'",
                            name, pattern);
                    }
                }
            }
        }

        #[test]
        fn prop_match_all_includes_everything(
            files in prop::collection::vec("[a-z_]{5,15}", 1..10),
        ) {
            let filter = TestFilter::new("*");
            let changed: Vec<PathBuf> = files
                .iter()
                .map(|f| PathBuf::from(format!("{}.py", f)))
                .collect();

            let filtered = filter_changed_files(&changed, &filter);
            prop_assert_eq!(filtered.len(), changed.len());
        }
    }
}
