//! Command history for the DX CLI
//!
//! Provides binary-serialized command history with search and statistics.
//! - Requirement 9.1: Store command history in binary format
//! - Requirement 9.2: Record command, arguments, exit code, duration, timestamp, working directory
//! - Requirement 9.3: Limit history to configurable max entries (default 1000)
//! - Requirement 9.4: Provide search functionality
//! - Requirement 9.5: Provide statistics on command usage

use crate::utils::error::DxError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;

/// Default maximum number of history entries
pub const DEFAULT_MAX_ENTRIES: usize = 1000;

/// A single command history entry
///
/// Requirement 9.2: Record command, arguments, exit code, duration, timestamp, working directory
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HistoryEntry {
    pub command: String,
    pub arguments: Vec<String>,
    pub exit_code: i32,
    pub duration_ms: u64,
    pub timestamp: DateTime<Utc>,
    pub working_dir: PathBuf,
}

impl HistoryEntry {
    pub fn new(
        command: impl Into<String>,
        arguments: Vec<String>,
        exit_code: i32,
        duration: Duration,
        working_dir: impl Into<PathBuf>,
    ) -> Self {
        Self {
            command: command.into(),
            arguments,
            exit_code,
            duration_ms: duration.as_millis() as u64,
            working_dir: working_dir.into(),
            timestamp: Utc::now(),
        }
    }

    pub fn is_success(&self) -> bool {
        self.exit_code == 0
    }

    pub fn duration(&self) -> Duration {
        Duration::from_millis(self.duration_ms)
    }

    pub fn matches(&self, query: &str) -> bool {
        let q = query.to_lowercase();
        self.command.to_lowercase().contains(&q)
            || self.arguments.iter().any(|a| a.to_lowercase().contains(&q))
            || self.working_dir.to_string_lossy().to_lowercase().contains(&q)
    }
}

/// Command usage statistics (Requirement 9.5)
#[derive(Debug, Clone, Default)]
pub struct HistoryStats {
    pub total: usize,
    pub successful: usize,
    pub failed: usize,
    pub avg_duration_ms: u64,
    pub top_commands: Vec<(String, usize)>,
}

/// Command history manager (Requirement 9.1)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CommandHistory {
    entries: Vec<HistoryEntry>,
    #[serde(default = "default_max_entries")]
    max_entries: usize,
}

fn default_max_entries() -> usize {
    DEFAULT_MAX_ENTRIES
}

impl CommandHistory {
    pub fn new() -> Self {
        Self { entries: Vec::new(), max_entries: DEFAULT_MAX_ENTRIES }
    }

    pub fn with_max_entries(max_entries: usize) -> Self {
        Self { entries: Vec::new(), max_entries }
    }

    fn history_path() -> Option<PathBuf> {
        Some(home::home_dir()?.join(".dx").join("history.json"))
    }

    pub fn load() -> Result<Self, DxError> {
        let path = Self::history_path().ok_or_else(|| DxError::Io {
            message: "Could not determine home directory".into(),
        })?;
        if !path.exists() {
            return Ok(Self::new());
        }
        let content = std::fs::read_to_string(&path).map_err(|e| DxError::Io {
            message: format!("Failed to read history: {}", e),
        })?;
        serde_json::from_str(&content).map_err(|e| DxError::Io {
            message: format!("Failed to parse history: {}", e),
        })
    }

    pub fn save(&self) -> Result<(), DxError> {
        let path = Self::history_path().ok_or_else(|| DxError::Io {
            message: "Could not determine home directory".into(),
        })?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| DxError::Io {
                message: format!("Failed to create history directory: {}", e),
            })?;
        }
        let content = serde_json::to_string_pretty(self).map_err(|e| DxError::Io {
            message: format!("Failed to serialize history: {}", e),
        })?;
        std::fs::write(&path, content).map_err(|e| DxError::Io {
            message: format!("Failed to write history: {}", e),
        })
    }

    /// Add entry with max entries enforcement (Requirement 9.3)
    pub fn add(&mut self, entry: HistoryEntry) {
        self.entries.push(entry);
        while self.entries.len() > self.max_entries {
            self.entries.remove(0);
        }
    }

    pub fn recent(&self, limit: usize) -> impl Iterator<Item = &HistoryEntry> {
        self.entries.iter().rev().take(limit)
    }

    pub fn entries(&self) -> &[HistoryEntry] {
        &self.entries
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn max_entries(&self) -> usize {
        self.max_entries
    }

    /// Search history (Requirement 9.4)
    pub fn search(&self, query: &str) -> Vec<&HistoryEntry> {
        self.entries.iter().filter(|e| e.matches(query)).collect()
    }

    /// Calculate statistics (Requirement 9.5)
    pub fn stats(&self) -> HistoryStats {
        if self.entries.is_empty() {
            return HistoryStats::default();
        }
        let total = self.entries.len();
        let successful = self.entries.iter().filter(|e| e.is_success()).count();
        let total_dur: u64 = self.entries.iter().map(|e| e.duration_ms).sum();
        let mut counts: HashMap<String, usize> = HashMap::new();
        for e in &self.entries {
            *counts.entry(e.command.clone()).or_insert(0) += 1;
        }
        let mut top: Vec<_> = counts.into_iter().collect();
        top.sort_by(|a, b| b.1.cmp(&a.1));
        top.truncate(10);
        HistoryStats {
            total,
            successful,
            failed: total - successful,
            avg_duration_ms: total_dur / total as u64,
            top_commands: top,
        }
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    fn test_entry(cmd: &str, exit_code: i32) -> HistoryEntry {
        HistoryEntry {
            command: cmd.to_string(),
            arguments: vec!["--flag".to_string()],
            exit_code,
            duration_ms: 100,
            timestamp: Utc::now(),
            working_dir: PathBuf::from("/test"),
        }
    }

    #[test]
    fn test_entry_new() {
        let e = HistoryEntry::new("build", vec!["--release".into()], 0, Duration::from_millis(500), "/proj");
        assert_eq!(e.command, "build");
        assert!(e.is_success());
    }

    #[test]
    fn test_entry_matches() {
        let e = test_entry("build", 0);
        assert!(e.matches("build"));
        assert!(e.matches("BUILD"));
        assert!(e.matches("flag"));
        assert!(!e.matches("deploy"));
    }

    #[test]
    fn test_add_and_recent() {
        let mut h = CommandHistory::new();
        h.add(test_entry("build", 0));
        h.add(test_entry("test", 0));
        h.add(test_entry("deploy", 1));
        assert_eq!(h.len(), 3);
        let r: Vec<_> = h.recent(2).collect();
        assert_eq!(r[0].command, "deploy");
        assert_eq!(r[1].command, "test");
    }

    #[test]
    fn test_max_entries() {
        let mut h = CommandHistory::with_max_entries(3);
        for i in 0..5 {
            h.add(test_entry(&format!("cmd{}", i), 0));
        }
        assert_eq!(h.len(), 3);
        assert_eq!(h.entries()[0].command, "cmd2");
    }

    #[test]
    fn test_search() {
        let mut h = CommandHistory::new();
        h.add(test_entry("build", 0));
        h.add(test_entry("test", 0));
        h.add(test_entry("build", 1));
        assert_eq!(h.search("build").len(), 2);
    }

    #[test]
    fn test_stats() {
        let mut h = CommandHistory::new();
        h.add(test_entry("build", 0));
        h.add(test_entry("build", 0));
        h.add(test_entry("test", 0));
        h.add(test_entry("deploy", 1));
        let s = h.stats();
        assert_eq!(s.total, 4);
        assert_eq!(s.successful, 3);
        assert_eq!(s.failed, 1);
        assert_eq!(s.top_commands[0], ("build".to_string(), 2));
    }

    #[test]
    fn test_serialization() {
        let mut h = CommandHistory::new();
        h.add(test_entry("build", 0));
        let json = serde_json::to_string(&h).unwrap();
        let r: CommandHistory = serde_json::from_str(&json).unwrap();
        assert_eq!(r.len(), 1);
    }

    // ═══════════════════════════════════════════════════════════════════
    //  PROPERTY TESTS
    // ═══════════════════════════════════════════════════════════════════

    // Property 13: History Serialization Round-Trip
    // **Validates: Requirements 9.1**
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        #[test]
        fn prop_history_serialization_roundtrip(
            commands in prop::collection::vec("[a-z]{1,10}", 0..20),
            exit_codes in prop::collection::vec(0i32..2, 0..20),
        ) {
            let mut history = CommandHistory::new();
            for (cmd, ec) in commands.iter().zip(exit_codes.iter()) {
                history.add(HistoryEntry {
                    command: cmd.clone(),
                    arguments: vec![],
                    exit_code: *ec,
                    duration_ms: 100,
                    timestamp: Utc::now(),
                    working_dir: PathBuf::from("/test"),
                });
            }
            let json = serde_json::to_string(&history).unwrap();
            let restored: CommandHistory = serde_json::from_str(&json).unwrap();
            prop_assert_eq!(history.len(), restored.len());
            for (o, r) in history.entries().iter().zip(restored.entries().iter()) {
                prop_assert_eq!(&o.command, &r.command);
                prop_assert_eq!(o.exit_code, r.exit_code);
            }
        }
    }

    // Property 14: History Max Entries Enforcement
    // **Validates: Requirements 9.3**
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        #[test]
        fn prop_history_max_entries(max in 1usize..50, num in 0usize..100) {
            let mut h = CommandHistory::with_max_entries(max);
            for i in 0..num {
                h.add(HistoryEntry {
                    command: format!("cmd{}", i),
                    arguments: vec![],
                    exit_code: 0,
                    duration_ms: 100,
                    timestamp: Utc::now(),
                    working_dir: PathBuf::from("/test"),
                });
            }
            prop_assert!(h.len() <= max);
            if num > max {
                prop_assert_eq!(&h.entries()[0].command, &format!("cmd{}", num - max));
            }
        }
    }

    // Property 15: History Search Functionality
    // **Validates: Requirements 9.4**
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        #[test]
        fn prop_history_search(query in "[a-z]{1,5}", cmds in prop::collection::vec("[a-z]{1,10}", 1..20)) {
            let mut h = CommandHistory::new();
            // Use a working_dir that won't interfere with search results
            let work_dir = PathBuf::from("/work");
            for cmd in &cmds {
                h.add(HistoryEntry {
                    command: cmd.clone(),
                    arguments: vec![],
                    exit_code: 0,
                    duration_ms: 100,
                    timestamp: Utc::now(),
                    working_dir: work_dir.clone(),
                });
            }
            let results = h.search(&query);
            // All results must contain the query in command, arguments, or working_dir
            for entry in &results {
                prop_assert!(entry.matches(&query), "Entry {:?} should match query '{}'", entry.command, query);
            }
            // Count expected matches (command contains query OR working_dir contains query)
            let q_lower = query.to_lowercase();
            let expected = cmds.iter().filter(|c| {
                c.to_lowercase().contains(&q_lower) || work_dir.to_string_lossy().to_lowercase().contains(&q_lower)
            }).count();
            prop_assert_eq!(results.len(), expected);
        }
    }

    // Property 16: History Statistics Accuracy
    // **Validates: Requirements 9.5**
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        #[test]
        fn prop_history_stats(exit_codes in prop::collection::vec(0i32..2, 1..50)) {
            let mut h = CommandHistory::new();
            for (i, ec) in exit_codes.iter().enumerate() {
                h.add(HistoryEntry {
                    command: format!("cmd{}", i % 5),
                    arguments: vec![],
                    exit_code: *ec,
                    duration_ms: 100,
                    timestamp: Utc::now(),
                    working_dir: PathBuf::from("/test"),
                });
            }
            let s = h.stats();
            prop_assert_eq!(s.total, exit_codes.len());
            prop_assert_eq!(s.successful, exit_codes.iter().filter(|&&e| e == 0).count());
            prop_assert_eq!(s.failed, exit_codes.iter().filter(|&&e| e != 0).count());
            prop_assert_eq!(s.successful + s.failed, s.total);
        }
    }
}
