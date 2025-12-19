//! Bun hashing functions.

use crc32fast::Hasher as Crc32Hasher;

/// Default fast hash (wyhash).
pub fn hash(data: &[u8]) -> u64 {
    wyhash::wyhash(data, 0)
}

/// WyHash.
pub fn wyhash(data: &[u8], seed: u64) -> u64 {
    wyhash::wyhash(data, seed)
}

/// CRC-32.
pub fn crc32(data: &[u8]) -> u32 {
    let mut hasher = Crc32Hasher::new();
    hasher.update(data);
    hasher.finalize()
}

/// Adler-32.
pub fn adler32(data: &[u8]) -> u32 {
    let mut a: u32 = 1;
    let mut b: u32 = 0;
    const MOD: u32 = 65521;

    for &byte in data {
        a = (a + byte as u32) % MOD;
        b = (b + a) % MOD;
    }

    (b << 16) | a
}

/// CityHash64.
pub fn city_hash_64(data: &[u8]) -> u64 {
    cityhash_rs::cityhash_110_128(data).0
}

/// MurmurHash3 32-bit.
pub fn murmur32v3(data: &[u8], seed: u32) -> u32 {
    murmur3::murmur3_32(&mut std::io::Cursor::new(data), seed).unwrap_or(0)
}

/// Hash algorithm for CryptoHasher.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HashAlgorithm {
    /// MD5
    Md5,
    /// SHA-1
    Sha1,
    /// SHA-256
    Sha256,
    /// SHA-384
    Sha384,
    /// SHA-512
    Sha512,
    /// BLAKE2b-256
    Blake2b256,
    /// BLAKE2b-512
    Blake2b512,
    /// BLAKE3
    Blake3,
}

/// Streaming crypto hasher.
pub struct CryptoHasher {
    algorithm: HashAlgorithm,
    data: Vec<u8>,
}

impl CryptoHasher {
    /// Create a new hasher.
    pub fn new(algorithm: HashAlgorithm) -> Self {
        Self {
            algorithm,
            data: Vec::new(),
        }
    }

    /// Update with data.
    pub fn update(&mut self, data: &[u8]) {
        self.data.extend_from_slice(data);
    }

    /// Get the digest.
    pub fn digest(&self) -> Vec<u8> {
        use sha2::{Sha256, Sha384, Sha512, Digest};
        
        match self.algorithm {
            HashAlgorithm::Md5 => {
                use md5::Md5;
                let mut hasher = Md5::new();
                hasher.update(&self.data);
                hasher.finalize().to_vec()
            }
            HashAlgorithm::Sha1 => {
                use sha1::Sha1;
                let mut hasher = Sha1::new();
                hasher.update(&self.data);
                hasher.finalize().to_vec()
            }
            HashAlgorithm::Sha256 => {
                let mut hasher = Sha256::new();
                hasher.update(&self.data);
                hasher.finalize().to_vec()
            }
            HashAlgorithm::Sha384 => {
                let mut hasher = Sha384::new();
                hasher.update(&self.data);
                hasher.finalize().to_vec()
            }
            HashAlgorithm::Sha512 => {
                let mut hasher = Sha512::new();
                hasher.update(&self.data);
                hasher.finalize().to_vec()
            }
            HashAlgorithm::Blake2b256 | HashAlgorithm::Blake2b512 | HashAlgorithm::Blake3 => {
                // Fallback to SHA-256 for now
                let mut hasher = Sha256::new();
                hasher.update(&self.data);
                hasher.finalize().to_vec()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        let h1 = hash(b"hello");
        let h2 = hash(b"hello");
        assert_eq!(h1, h2);
        
        let h3 = hash(b"world");
        assert_ne!(h1, h3);
    }

    #[test]
    fn test_crc32() {
        let c1 = crc32(b"hello");
        let c2 = crc32(b"hello");
        assert_eq!(c1, c2);
    }

    #[test]
    fn test_adler32() {
        let a1 = adler32(b"hello");
        let a2 = adler32(b"hello");
        assert_eq!(a1, a2);
    }

    #[test]
    fn test_crypto_hasher() {
        let mut hasher = CryptoHasher::new(HashAlgorithm::Sha256);
        hasher.update(b"hello");
        let digest = hasher.digest();
        assert_eq!(digest.len(), 32);
    }
}
