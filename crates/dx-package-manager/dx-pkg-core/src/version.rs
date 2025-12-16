use crate::error::{Error, Result};
use std::fmt;

/// Semantic version
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl Version {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self { major, minor, patch }
    }

    pub fn parse(s: &str) -> Result<Self> {
        let s = s.trim_start_matches('v');
        let parts: Vec<&str> = s.split('.').collect();
        
        if parts.len() != 3 {
            return Err(Error::InvalidVersion(s.to_string()));
        }

        Ok(Self {
            major: parts[0].parse().map_err(|_| Error::InvalidVersion(s.to_string()))?,
            minor: parts[1].parse().map_err(|_| Error::InvalidVersion(s.to_string()))?,
            patch: parts[2].parse().map_err(|_| Error::InvalidVersion(s.to_string()))?,
        })
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

/// Version constraint
#[derive(Debug, Clone, PartialEq)]
pub enum VersionConstraint {
    Exact(Version),
    Range { min: Version, max: Version },
    Caret(Version),  // ^1.2.3 = >=1.2.3 <2.0.0
    Tilde(Version),  // ~1.2.3 = >=1.2.3 <1.3.0
}

/// Encode version to u64 (major << 40 | minor << 20 | patch)
pub fn encode_version(version: &Version) -> u64 {
    ((version.major as u64) << 40) | ((version.minor as u64) << 20) | (version.patch as u64)
}

/// Decode version from u64
pub fn decode_version(encoded: u64) -> Version {
    Version {
        major: ((encoded >> 40) & 0xFFFFFF) as u32,
        minor: ((encoded >> 20) & 0xFFFFF) as u32,
        patch: (encoded & 0xFFFFF) as u32,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_version() {
        let v = Version::parse("1.2.3").unwrap();
        assert_eq!(v.major, 1);
        assert_eq!(v.minor, 2);
        assert_eq!(v.patch, 3);
    }

    #[test]
    fn test_parse_version_with_v() {
        let v = Version::parse("v1.2.3").unwrap();
        assert_eq!(v, Version::new(1, 2, 3));
    }

    #[test]
    fn test_encode_decode() {
        let v = Version::new(4, 17, 21);
        let encoded = encode_version(&v);
        let decoded = decode_version(encoded);
        assert_eq!(v, decoded);
    }

    #[test]
    fn test_version_ordering() {
        let v1 = Version::new(1, 0, 0);
        let v2 = Version::new(1, 2, 0);
        let v3 = Version::new(2, 0, 0);
        assert!(v1 < v2);
        assert!(v2 < v3);
    }
}
