//! Stack capability definitions
//!
//! Defines what capabilities a language stack can provide.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Individual stack capabilities
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StackCapability {
    /// Code execution engine (e.g., dx-js-runtime)
    Runtime,

    /// Dependency management (e.g., dx-js-package-manager)
    PackageManager,

    /// Code bundling/compilation (e.g., dx-js-bundler)
    Bundler,

    /// Multi-package workspace management (e.g., dx-js-monorepo)
    Monorepo,

    /// Cross-version/platform compatibility (e.g., dx-js-compatibility)
    Compatibility,

    /// Test execution framework (e.g., dx-js-test-runner)
    TestRunner,
}

impl StackCapability {
    /// All available capabilities
    pub const ALL: [StackCapability; 6] = [
        StackCapability::Runtime,
        StackCapability::PackageManager,
        StackCapability::Bundler,
        StackCapability::Monorepo,
        StackCapability::Compatibility,
        StackCapability::TestRunner,
    ];

    /// Returns the CLI command name for this capability
    pub fn command_name(&self) -> &'static str {
        match self {
            StackCapability::Runtime => "run",
            StackCapability::PackageManager => "pkg",
            StackCapability::Bundler => "bundle",
            StackCapability::Monorepo => "mono",
            StackCapability::Compatibility => "compat",
            StackCapability::TestRunner => "test",
        }
    }

    /// Returns a human-readable description
    pub fn description(&self) -> &'static str {
        match self {
            StackCapability::Runtime => "Code execution engine",
            StackCapability::PackageManager => "Dependency management",
            StackCapability::Bundler => "Code bundling/compilation",
            StackCapability::Monorepo => "Multi-package workspace management",
            StackCapability::Compatibility => "Cross-version/platform compatibility",
            StackCapability::TestRunner => "Test execution framework",
        }
    }
}

impl fmt::Display for StackCapability {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StackCapability::Runtime => write!(f, "Runtime"),
            StackCapability::PackageManager => write!(f, "Package Manager"),
            StackCapability::Bundler => write!(f, "Bundler"),
            StackCapability::Monorepo => write!(f, "Monorepo"),
            StackCapability::Compatibility => write!(f, "Compatibility"),
            StackCapability::TestRunner => write!(f, "Test Runner"),
        }
    }
}

/// A set of stack capabilities, implemented as a bitset for efficiency.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct StackCapabilitySet(u8);

impl StackCapabilitySet {
    const RUNTIME: u8 = 1 << 0;
    const PACKAGE_MANAGER: u8 = 1 << 1;
    const BUNDLER: u8 = 1 << 2;
    const MONOREPO: u8 = 1 << 3;
    const COMPATIBILITY: u8 = 1 << 4;
    const TEST_RUNNER: u8 = 1 << 5;

    /// Creates an empty capability set
    pub const fn empty() -> Self {
        Self(0)
    }

    /// Creates a capability set with all capabilities
    pub const fn all() -> Self {
        Self(
            Self::RUNTIME
                | Self::PACKAGE_MANAGER
                | Self::BUNDLER
                | Self::MONOREPO
                | Self::COMPATIBILITY
                | Self::TEST_RUNNER,
        )
    }

    /// Converts a capability to its bit flag
    const fn to_bit(cap: StackCapability) -> u8 {
        match cap {
            StackCapability::Runtime => Self::RUNTIME,
            StackCapability::PackageManager => Self::PACKAGE_MANAGER,
            StackCapability::Bundler => Self::BUNDLER,
            StackCapability::Monorepo => Self::MONOREPO,
            StackCapability::Compatibility => Self::COMPATIBILITY,
            StackCapability::TestRunner => Self::TEST_RUNNER,
        }
    }

    /// Check if a capability is present
    pub const fn contains(&self, cap: StackCapability) -> bool {
        self.0 & Self::to_bit(cap) != 0
    }

    /// Add a capability to the set
    pub fn insert(&mut self, cap: StackCapability) {
        self.0 |= Self::to_bit(cap);
    }

    /// Remove a capability from the set
    pub fn remove(&mut self, cap: StackCapability) {
        self.0 &= !Self::to_bit(cap);
    }

    /// Check if the set is empty
    pub const fn is_empty(&self) -> bool {
        self.0 == 0
    }

    /// Count the number of capabilities
    pub const fn count(&self) -> u32 {
        self.0.count_ones()
    }

    /// Iterate over all capabilities in this set
    pub fn iter(&self) -> impl Iterator<Item = StackCapability> + '_ {
        StackCapability::ALL.iter().copied().filter(|cap| self.contains(*cap))
    }

    /// Union of two capability sets
    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    /// Intersection of two capability sets
    pub const fn intersection(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }

    /// Difference of two capability sets (self - other)
    pub const fn difference(self, other: Self) -> Self {
        Self(self.0 & !other.0)
    }
}

impl FromIterator<StackCapability> for StackCapabilitySet {
    fn from_iter<T: IntoIterator<Item = StackCapability>>(iter: T) -> Self {
        let mut set = Self::empty();
        for cap in iter {
            set.insert(cap);
        }
        set
    }
}

impl fmt::Display for StackCapabilitySet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let caps: Vec<_> = self.iter().map(|c| c.to_string()).collect();
        if caps.is_empty() {
            write!(f, "(none)")
        } else {
            write!(f, "{}", caps.join(", "))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_set() {
        let set = StackCapabilitySet::empty();
        assert!(set.is_empty());
        assert!(!set.contains(StackCapability::Runtime));
    }

    #[test]
    fn test_all_set() {
        let set = StackCapabilitySet::all();
        assert!(!set.is_empty());
        assert_eq!(set.count(), 6);
        assert!(set.contains(StackCapability::Runtime));
        assert!(set.contains(StackCapability::TestRunner));
    }

    #[test]
    fn test_insert_remove() {
        let mut set = StackCapabilitySet::empty();
        set.insert(StackCapability::Runtime);
        assert!(set.contains(StackCapability::Runtime));
        assert!(!set.contains(StackCapability::Bundler));

        set.remove(StackCapability::Runtime);
        assert!(!set.contains(StackCapability::Runtime));
    }

    #[test]
    fn test_from_iter() {
        let set =
            StackCapabilitySet::from_iter([StackCapability::Runtime, StackCapability::Bundler]);
        assert!(set.contains(StackCapability::Runtime));
        assert!(set.contains(StackCapability::Bundler));
        assert!(!set.contains(StackCapability::TestRunner));
    }

    #[test]
    fn test_set_operations() {
        let a = StackCapabilitySet::from_iter([StackCapability::Runtime, StackCapability::Bundler]);
        let b =
            StackCapabilitySet::from_iter([StackCapability::Bundler, StackCapability::TestRunner]);

        let union = a.union(b);
        assert!(union.contains(StackCapability::Runtime));
        assert!(union.contains(StackCapability::Bundler));
        assert!(union.contains(StackCapability::TestRunner));

        let intersection = a.intersection(b);
        assert!(!intersection.contains(StackCapability::Runtime));
        assert!(intersection.contains(StackCapability::Bundler));
        assert!(!intersection.contains(StackCapability::TestRunner));
    }
}
