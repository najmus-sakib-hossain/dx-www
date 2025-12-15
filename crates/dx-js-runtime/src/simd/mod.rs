//! SIMD operations for vectorized computation

pub mod console;

pub use console::{BatchConsole, console_log_number, console_flush};

use crate::error::{DxError, DxResult};

#[derive(Debug, Clone)]
pub struct SimdF32x4(pub [f32; 4]);

#[derive(Debug, Clone)]
pub struct SimdI32x4(pub [i32; 4]);

impl SimdF32x4 {
    pub fn new(a: f32, b: f32, c: f32, d: f32) -> Self {
        Self([a, b, c, d])
    }

    pub fn splat(value: f32) -> Self {
        Self([value; 4])
    }

    pub fn add(&self, other: &Self) -> Self {
        Self([
            self.0[0] + other.0[0],
            self.0[1] + other.0[1],
            self.0[2] + other.0[2],
            self.0[3] + other.0[3],
        ])
    }

    pub fn mul(&self, other: &Self) -> Self {
        Self([
            self.0[0] * other.0[0],
            self.0[1] * other.0[1],
            self.0[2] * other.0[2],
            self.0[3] * other.0[3],
        ])
    }

    pub fn sum(&self) -> f32 {
        self.0.iter().sum()
    }
}

impl SimdI32x4 {
    pub fn new(a: i32, b: i32, c: i32, d: i32) -> Self {
        Self([a, b, c, d])
    }

    pub fn splat(value: i32) -> Self {
        Self([value; 4])
    }

    pub fn add(&self, other: &Self) -> Self {
        Self([
            self.0[0] + other.0[0],
            self.0[1] + other.0[1],
            self.0[2] + other.0[2],
            self.0[3] + other.0[3],
        ])
    }

    pub fn mul(&self, other: &Self) -> Self {
        Self([
            self.0[0] * other.0[0],
            self.0[1] * other.0[1],
            self.0[2] * other.0[2],
            self.0[3] * other.0[3],
        ])
    }
}

pub fn vector_add_f32(a: &[f32], b: &[f32], result: &mut [f32]) -> DxResult<()> {
    if a.len() != b.len() || a.len() != result.len() {
        return Err(DxError::RuntimeError("Vector length mismatch".to_string()));
    }

    let chunks = a.len() / 4;
    for i in 0..chunks {
        let offset = i * 4;
        let va = SimdF32x4::new(a[offset], a[offset+1], a[offset+2], a[offset+3]);
        let vb = SimdF32x4::new(b[offset], b[offset+1], b[offset+2], b[offset+3]);
        let vr = va.add(&vb);
        result[offset..offset+4].copy_from_slice(&vr.0);
    }

    // Handle remainder
    for i in (chunks * 4)..a.len() {
        result[i] = a[i] + b[i];
    }

    Ok(())
}
