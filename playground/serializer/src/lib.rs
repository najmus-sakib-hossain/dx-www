//! Comprehensive serializer benchmarks
//! 
//! Compares DX-Zero against:
//! - rkyv
//! - Cap'n Proto
//! - FlatBuffers
//! - Protobuf
//! - Bincode
//! - JSON
//! - TOON

use serde::{Deserialize, Serialize};

/// Test data structure
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub id: u64,
    pub age: u32,
    pub active: bool,
    pub score: f64,
    pub name: String,
    pub email: String,
    pub bio: String,
}

impl User {
    pub fn sample() -> Self {
        Self {
            id: 12345,
            age: 30,
            active: true,
            score: 98.5,
            name: "John Doe".to_string(),
            email: "john@example.com".to_string(),
            bio: "Software engineer with 10 years of experience in Rust and systems programming.".to_string(),
        }
    }

    pub fn small_sample() -> Self {
        Self {
            id: 999,
            age: 25,
            active: true,
            score: 85.0,
            name: "Alice".to_string(),
            email: "a@b.com".to_string(),
            bio: "Short".to_string(),
        }
    }
}

// Rkyv support
#[derive(
    rkyv::Archive, 
    rkyv::Serialize, 
    rkyv::Deserialize,
    Debug, Clone, PartialEq
)]
#[rkyv(derive(Debug))]
pub struct UserRkyv {
    pub id: u64,
    pub age: u32,
    pub active: bool,
    pub score: f64,
    pub name: String,
    pub email: String,
    pub bio: String,
}

impl From<&User> for UserRkyv {
    fn from(u: &User) -> Self {
        Self {
            id: u.id,
            age: u.age,
            active: u.active,
            score: u.score,
            name: u.name.clone(),
            email: u.email.clone(),
            bio: u.bio.clone(),
        }
    }
}

// Cap'n Proto support (optional - only if schema compiled)
#[cfg(feature = "capnproto")]
pub mod capnp_user {
    include!(concat!(env!("OUT_DIR"), "/user_capnp.rs"));
}
