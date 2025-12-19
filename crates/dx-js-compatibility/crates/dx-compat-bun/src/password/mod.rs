//! Bun.password hashing.

use crate::error::{BunError, BunResult};

/// Password hashing algorithm.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Algorithm {
    /// Argon2id (default, recommended)
    #[default]
    Argon2id,
    /// bcrypt
    Bcrypt,
}

/// Hash a password.
pub fn hash(password: &str, algorithm: Option<Algorithm>) -> BunResult<String> {
    let algorithm = algorithm.unwrap_or_default();
    
    match algorithm {
        Algorithm::Argon2id => {
            let salt = argon2::password_hash::SaltString::generate(&mut rand::thread_rng());
            let argon2 = argon2::Argon2::default();
            
            argon2::PasswordHasher::hash_password(&argon2, password.as_bytes(), &salt)
                .map(|h| h.to_string())
                .map_err(|e| BunError::Password(e.to_string()))
        }
        Algorithm::Bcrypt => {
            bcrypt::hash(password, bcrypt::DEFAULT_COST)
                .map_err(|e| BunError::Password(e.to_string()))
        }
    }
}

/// Verify a password against a hash.
pub fn verify(password: &str, hash: &str) -> BunResult<bool> {
    // Try Argon2 first
    if hash.starts_with("$argon2") {
        let parsed_hash = argon2::PasswordHash::new(hash)
            .map_err(|e| BunError::Password(e.to_string()))?;
        
        let argon2 = argon2::Argon2::default();
        Ok(argon2::PasswordVerifier::verify_password(&argon2, password.as_bytes(), &parsed_hash).is_ok())
    } else {
        // Try bcrypt
        bcrypt::verify(password, hash)
            .map_err(|e| BunError::Password(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_argon2_hash_verify() {
        let password = "my_secure_password";
        let hashed = hash(password, Some(Algorithm::Argon2id)).unwrap();
        
        assert!(verify(password, &hashed).unwrap());
        assert!(!verify("wrong_password", &hashed).unwrap());
    }

    #[test]
    fn test_bcrypt_hash_verify() {
        let password = "my_secure_password";
        let hashed = hash(password, Some(Algorithm::Bcrypt)).unwrap();
        
        assert!(verify(password, &hashed).unwrap());
        assert!(!verify("wrong_password", &hashed).unwrap());
    }
}
