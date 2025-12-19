//! Cryptographic operations.

use sha2::{Sha256, Sha512, Digest};
use md5::Md5;  // md-5 crate re-exports as md5

/// Hash algorithm.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HashAlgorithm {
    /// MD5
    Md5,
    /// SHA-1
    Sha1,
    /// SHA-256
    Sha256,
    /// SHA-512
    Sha512,
}

/// Create a hash of data.
pub fn create_hash(algorithm: HashAlgorithm, data: &[u8]) -> Vec<u8> {
    match algorithm {
        HashAlgorithm::Md5 => {
            let mut hasher = Md5::new();
            hasher.update(data);
            hasher.finalize().to_vec()
        }
        HashAlgorithm::Sha1 => {
            use sha1::Sha1;
            let mut hasher = Sha1::new();
            hasher.update(data);
            hasher.finalize().to_vec()
        }
        HashAlgorithm::Sha256 => {
            let mut hasher = Sha256::new();
            hasher.update(data);
            hasher.finalize().to_vec()
        }
        HashAlgorithm::Sha512 => {
            let mut hasher = Sha512::new();
            hasher.update(data);
            hasher.finalize().to_vec()
        }
    }
}

/// Generate cryptographically secure random bytes.
pub fn random_bytes(size: usize) -> Vec<u8> {
    let mut buf = vec![0u8; size];
    getrandom::getrandom(&mut buf).expect("Failed to generate random bytes");
    buf
}

/// Generate random UUID v4.
pub fn random_uuid() -> String {
    let bytes = random_bytes(16);
    format!(
        "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        bytes[0], bytes[1], bytes[2], bytes[3],
        bytes[4], bytes[5],
        (bytes[6] & 0x0f) | 0x40, bytes[7],
        (bytes[8] & 0x3f) | 0x80, bytes[9],
        bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15]
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha256() {
        let hash = create_hash(HashAlgorithm::Sha256, b"hello");
        assert_eq!(hash.len(), 32);
    }

    #[test]
    fn test_random_bytes() {
        let bytes = random_bytes(32);
        assert_eq!(bytes.len(), 32);
    }

    #[test]
    fn test_random_uuid() {
        let uuid = random_uuid();
        assert_eq!(uuid.len(), 36);
        assert!(uuid.contains('-'));
    }
}
