//! JSON Serialization Benchmark: dx-serializer vs serde_json
//!
//! Run with: cargo bench --manifest-path benchmarks/json/Cargo.toml

use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use serde::{Deserialize, Serialize};

/// Test data structure - mimics typical API response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub email: String,
    pub age: u32,
    pub active: bool,
    pub tags: Vec<String>,
}

/// Complex nested structure for stress testing
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ApiResponse {
    pub success: bool,
    pub data: Vec<User>,
    pub meta: Meta,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Meta {
    pub total: u64,
    pub page: u32,
    pub per_page: u32,
    pub total_pages: u32,
}

impl User {
    pub fn sample() -> Self {
        Self {
            id: 12345,
            name: "John Doe".to_string(),
            email: "john.doe@example.com".to_string(),
            age: 30,
            active: true,
            tags: vec!["rust".to_string(), "web".to_string(), "performance".to_string()],
        }
    }
}

impl ApiResponse {
    pub fn sample(user_count: usize) -> Self {
        Self {
            success: true,
            data: (0..user_count).map(|i| {
                User {
                    id: i as u64,
                    name: format!("User {}", i),
                    email: format!("user{}@example.com", i),
                    age: 25 + (i % 40) as u32,
                    active: i % 3 != 0,
                    tags: vec!["tag1".to_string(), "tag2".to_string()],
                }
            }).collect(),
            meta: Meta {
                total: user_count as u64,
                page: 1,
                per_page: user_count as u32,
                total_pages: 1,
            },
        }
    }
}

fn benchmark_serde_json_serialize(c: &mut Criterion) {
    let mut group = c.benchmark_group("Serialize/serde_json");
    
    // Single user
    let user = User::sample();
    let json_size = serde_json::to_vec(&user).unwrap().len();
    group.throughput(Throughput::Bytes(json_size as u64));
    group.bench_function("single_user", |b| {
        b.iter(|| serde_json::to_vec(black_box(&user)).unwrap())
    });
    
    // 10 users
    let response_10 = ApiResponse::sample(10);
    let json_size_10 = serde_json::to_vec(&response_10).unwrap().len();
    group.throughput(Throughput::Bytes(json_size_10 as u64));
    group.bench_function("10_users", |b| {
        b.iter(|| serde_json::to_vec(black_box(&response_10)).unwrap())
    });
    
    // 100 users
    let response_100 = ApiResponse::sample(100);
    let json_size_100 = serde_json::to_vec(&response_100).unwrap().len();
    group.throughput(Throughput::Bytes(json_size_100 as u64));
    group.bench_function("100_users", |b| {
        b.iter(|| serde_json::to_vec(black_box(&response_100)).unwrap())
    });
    
    group.finish();
}

fn benchmark_serde_json_deserialize(c: &mut Criterion) {
    let mut group = c.benchmark_group("Deserialize/serde_json");
    
    // Single user
    let user = User::sample();
    let json_bytes = serde_json::to_vec(&user).unwrap();
    group.throughput(Throughput::Bytes(json_bytes.len() as u64));
    group.bench_function("single_user", |b| {
        b.iter(|| serde_json::from_slice::<User>(black_box(&json_bytes)).unwrap())
    });
    
    // 10 users
    let response_10 = ApiResponse::sample(10);
    let json_bytes_10 = serde_json::to_vec(&response_10).unwrap();
    group.throughput(Throughput::Bytes(json_bytes_10.len() as u64));
    group.bench_function("10_users", |b| {
        b.iter(|| serde_json::from_slice::<ApiResponse>(black_box(&json_bytes_10)).unwrap())
    });
    
    // 100 users
    let response_100 = ApiResponse::sample(100);
    let json_bytes_100 = serde_json::to_vec(&response_100).unwrap();
    group.throughput(Throughput::Bytes(json_bytes_100.len() as u64));
    group.bench_function("100_users", |b| {
        b.iter(|| serde_json::from_slice::<ApiResponse>(black_box(&json_bytes_100)).unwrap())
    });
    
    group.finish();
}

fn benchmark_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("Comparison");
    
    // Compare serialization with string output
    let user = User::sample();
    
    group.bench_function("serde_json_to_string", |b| {
        b.iter(|| serde_json::to_string(black_box(&user)).unwrap())
    });
    
    group.bench_function("serde_json_to_vec", |b| {
        b.iter(|| serde_json::to_vec(black_box(&user)).unwrap())
    });
    
    // Pre-allocated buffer serialization
    group.bench_function("serde_json_to_writer", |b| {
        let mut buffer = Vec::with_capacity(256);
        b.iter(|| {
            buffer.clear();
            serde_json::to_writer(&mut buffer, black_box(&user)).unwrap();
            buffer.len()
        })
    });
    
    group.finish();
}

criterion_group!(
    benches,
    benchmark_serde_json_serialize,
    benchmark_serde_json_deserialize,
    benchmark_comparison,
);

criterion_main!(benches);
