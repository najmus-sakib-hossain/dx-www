//! Comprehensive benchmark: All serializers

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use serializer_benchmark::{User, UserRkyv};
use std::hint::black_box as bb;

// =============================================================================
// DX-ZERO (Our Implementation)
// =============================================================================

use dx_serializer::zero::DxZeroBuilder;

#[repr(C, packed)]
struct UserDxZero {
    _header: [u8; 4],
    id: u64,
    age: u32,
    active: bool,
    score: f64,
    name_slot: [u8; 16],
    email_slot: [u8; 16],
    bio_slot: [u8; 16],
}

impl UserDxZero {
    const FIXED_SIZE: usize = 21;
    const SLOT_COUNT: usize = 3;
    const HEAP_OFFSET: usize = 73;

    #[inline(always)]
    fn from_bytes(bytes: &[u8]) -> &Self {
        unsafe { &*(bytes.as_ptr() as *const Self) }
    }

    #[inline(always)]
    fn id(&self) -> u64 {
        unsafe {
            let ptr = (self as *const Self as *const u8).add(4);
            u64::from_le_bytes(*(ptr as *const [u8; 8]))
        }
    }

    #[inline(always)]
    fn age(&self) -> u32 {
        unsafe {
            let ptr = (self as *const Self as *const u8).add(12);
            u32::from_le_bytes(*(ptr as *const [u8; 4]))
        }
    }
}

fn serialize_dx_zero(user: &User) -> Vec<u8> {
    let mut buffer = Vec::new();
    let mut builder = DxZeroBuilder::new(&mut buffer, UserDxZero::FIXED_SIZE, UserDxZero::SLOT_COUNT);
    
    builder.write_u64(0, user.id);
    builder.write_u32(8, user.age);
    builder.write_bool(12, user.active);
    builder.write_f64(13, user.score);
    builder.write_string(21, &user.name);
    builder.write_string(37, &user.email);
    builder.write_string(53, &user.bio);
    
    builder.finish();
    buffer
}

fn deserialize_dx_zero(bytes: &[u8]) -> &UserDxZero {
    UserDxZero::from_bytes(bytes)
}

// =============================================================================
// RKYV
// =============================================================================

fn serialize_rkyv(user: &User) -> Vec<u8> {
    let user_rkyv = UserRkyv::from(user);
    rkyv::to_bytes::<_, 256>(&user_rkyv).unwrap().to_vec()
}

fn deserialize_rkyv(bytes: &[u8]) -> u64 {
    let archived = unsafe { rkyv::archived_root::<UserRkyv>(bytes) };
    archived.id
}

// =============================================================================
// BINCODE
// =============================================================================

fn serialize_bincode(user: &User) -> Vec<u8> {
    bincode::serialize(user).unwrap()
}

fn deserialize_bincode(bytes: &[u8]) -> User {
    bincode::deserialize(bytes).unwrap()
}

// =============================================================================
// JSON
// =============================================================================

fn serialize_json(user: &User) -> Vec<u8> {
    serde_json::to_vec(user).unwrap()
}

fn deserialize_json(bytes: &[u8]) -> User {
    serde_json::from_slice(bytes).unwrap()
}

// =============================================================================
// DX-INFINITY (Text format)
// =============================================================================

fn serialize_dx_infinity(user: &User) -> Vec<u8> {
    format!(
        "user=id%i age%i active%b score%f name%s email%s bio%s\n{} {} {} {} {} {} {}",
        user.id, user.age, if user.active { '+' } else { '-' }, 
        user.score, user.name, user.email, user.bio
    ).into_bytes()
}

fn deserialize_dx_infinity(bytes: &[u8]) -> Result<(), String> {
    dx_serializer::parse(bytes).map(|_| ()).map_err(|e| format!("{:?}", e))
}

// =============================================================================
// BENCHMARKS
// =============================================================================

fn bench_serialize(c: &mut Criterion) {
    let user = User::sample();
    let mut group = c.benchmark_group("serialize");
    
    group.bench_function("dx_zero", |b| {
        b.iter(|| serialize_dx_zero(black_box(&user)));
    });
    
    group.bench_function("rkyv", |b| {
        b.iter(|| serialize_rkyv(black_box(&user)));
    });
    
    group.bench_function("bincode", |b| {
        b.iter(|| serialize_bincode(black_box(&user)));
    });
    
    group.bench_function("json", |b| {
        b.iter(|| serialize_json(black_box(&user)));
    });
    
    group.bench_function("dx_infinity", |b| {
        b.iter(|| serialize_dx_infinity(black_box(&user)));
    });
    
    group.finish();
}

fn bench_deserialize(c: &mut Criterion) {
    let user = User::sample();
    
    let dx_zero_bytes = serialize_dx_zero(&user);
    let rkyv_bytes = serialize_rkyv(&user);
    let bincode_bytes = serialize_bincode(&user);
    let json_bytes = serialize_json(&user);
    let dx_inf_bytes = serialize_dx_infinity(&user);
    
    let mut group = c.benchmark_group("deserialize");
    
    group.bench_function("dx_zero", |b| {
        b.iter(|| {
            let u = deserialize_dx_zero(black_box(&dx_zero_bytes));
            black_box(u.id());
        });
    });
    
    group.bench_function("rkyv", |b| {
        b.iter(|| deserialize_rkyv(black_box(&rkyv_bytes)));
    });
    
    group.bench_function("bincode", |b| {
        b.iter(|| deserialize_bincode(black_box(&bincode_bytes)));
    });
    
    group.bench_function("json", |b| {
        b.iter(|| deserialize_json(black_box(&json_bytes)));
    });
    
    group.bench_function("dx_infinity", |b| {
        b.iter(|| deserialize_dx_infinity(black_box(&dx_inf_bytes)));
    });
    
    group.finish();
}

fn bench_roundtrip(c: &mut Criterion) {
    let user = User::sample();
    let mut group = c.benchmark_group("roundtrip");
    
    group.bench_function("dx_zero", |b| {
        b.iter(|| {
            let bytes = serialize_dx_zero(black_box(&user));
            let u = deserialize_dx_zero(&bytes);
            black_box(u.id());
        });
    });
    
    group.bench_function("rkyv", |b| {
        b.iter(|| {
            let bytes = serialize_rkyv(black_box(&user));
            black_box(deserialize_rkyv(&bytes));
        });
    });
    
    group.bench_function("bincode", |b| {
        b.iter(|| {
            let bytes = serialize_bincode(black_box(&user));
            black_box(deserialize_bincode(&bytes));
        });
    });
    
    group.bench_function("json", |b| {
        b.iter(|| {
            let bytes = serialize_json(black_box(&user));
            black_box(deserialize_json(&bytes));
        });
    });
    
    group.finish();
}

fn bench_size_comparison(c: &mut Criterion) {
    let user = User::sample();
    
    let dx_zero_bytes = serialize_dx_zero(&user);
    let rkyv_bytes = serialize_rkyv(&user);
    let bincode_bytes = serialize_bincode(&user);
    let json_bytes = serialize_json(&user);
    let dx_inf_bytes = serialize_dx_infinity(&user);
    
    println!("\n=== SIZE COMPARISON (User struct) ===");
    println!("DX-Zero:      {} bytes (baseline)", dx_zero_bytes.len());
    println!("rkyv:         {} bytes ({:.1}× larger)", rkyv_bytes.len(), rkyv_bytes.len() as f64 / dx_zero_bytes.len() as f64);
    println!("Bincode:      {} bytes ({:.1}× larger)", bincode_bytes.len(), bincode_bytes.len() as f64 / dx_zero_bytes.len() as f64);
    println!("JSON:         {} bytes ({:.1}× larger)", json_bytes.len(), json_bytes.len() as f64 / dx_zero_bytes.len() as f64);
    println!("DX-Infinity:  {} bytes ({:.1}× larger)", dx_inf_bytes.len(), dx_inf_bytes.len() as f64 / dx_zero_bytes.len() as f64);
    println!();
    
    c.bench_function("size_baseline", |b| {
        b.iter(|| black_box(dx_zero_bytes.len()));
    });
}

criterion_group!(
    benches,
    bench_serialize,
    bench_deserialize,
    bench_roundtrip,
    bench_size_comparison,
);

criterion_main!(benches);
