//! Bun compression functions.

use crate::error::{BunError, BunResult};
use flate2::read::{GzDecoder, GzEncoder, DeflateDecoder, DeflateEncoder};
use flate2::Compression;
use std::io::{Read, Write};

/// Gzip compress.
pub fn gzip_sync(data: &[u8], level: Option<u32>) -> BunResult<Vec<u8>> {
    let level = Compression::new(level.unwrap_or(6));
    let mut encoder = GzEncoder::new(data, level);
    let mut result = Vec::new();
    encoder.read_to_end(&mut result)?;
    Ok(result)
}

/// Gzip decompress.
pub fn gunzip_sync(data: &[u8]) -> BunResult<Vec<u8>> {
    let mut decoder = GzDecoder::new(data);
    let mut result = Vec::new();
    decoder.read_to_end(&mut result)?;
    Ok(result)
}

/// Deflate compress.
pub fn deflate_sync(data: &[u8], level: Option<u32>) -> BunResult<Vec<u8>> {
    let level = Compression::new(level.unwrap_or(6));
    let mut encoder = DeflateEncoder::new(data, level);
    let mut result = Vec::new();
    encoder.read_to_end(&mut result)?;
    Ok(result)
}

/// Deflate decompress.
pub fn inflate_sync(data: &[u8]) -> BunResult<Vec<u8>> {
    let mut decoder = DeflateDecoder::new(data);
    let mut result = Vec::new();
    decoder.read_to_end(&mut result)?;
    Ok(result)
}

/// Brotli compress.
pub fn brotli_compress_sync(data: &[u8], level: Option<u32>) -> BunResult<Vec<u8>> {
    let mut result = Vec::new();
    let params = brotli::enc::BrotliEncoderParams {
        quality: level.unwrap_or(6) as i32,
        ..Default::default()
    };
    brotli::BrotliCompress(&mut std::io::Cursor::new(data), &mut result, &params)
        .map_err(|e| BunError::Compression(e.to_string()))?;
    Ok(result)
}

/// Brotli decompress.
pub fn brotli_decompress_sync(data: &[u8]) -> BunResult<Vec<u8>> {
    let mut result = Vec::new();
    brotli::BrotliDecompress(&mut std::io::Cursor::new(data), &mut result)
        .map_err(|e| BunError::Compression(e.to_string()))?;
    Ok(result)
}

/// Zstd compress.
pub fn zstd_compress_sync(data: &[u8], level: Option<i32>) -> BunResult<Vec<u8>> {
    let level = level.unwrap_or(3);
    zstd::encode_all(std::io::Cursor::new(data), level)
        .map_err(|e| BunError::Compression(e.to_string()))
}

/// Zstd decompress.
pub fn zstd_decompress_sync(data: &[u8]) -> BunResult<Vec<u8>> {
    zstd::decode_all(std::io::Cursor::new(data))
        .map_err(|e| BunError::Compression(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gzip_round_trip() {
        let data = b"hello world";
        let compressed = gzip_sync(data, None).unwrap();
        let decompressed = gunzip_sync(&compressed).unwrap();
        assert_eq!(decompressed, data);
    }

    #[test]
    fn test_deflate_round_trip() {
        let data = b"hello world";
        let compressed = deflate_sync(data, None).unwrap();
        let decompressed = inflate_sync(&compressed).unwrap();
        assert_eq!(decompressed, data);
    }

    #[test]
    fn test_brotli_round_trip() {
        let data = b"hello world";
        let compressed = brotli_compress_sync(data, None).unwrap();
        let decompressed = brotli_decompress_sync(&compressed).unwrap();
        assert_eq!(decompressed, data);
    }

    #[test]
    fn test_zstd_round_trip() {
        let data = b"hello world";
        let compressed = zstd_compress_sync(data, None).unwrap();
        let decompressed = zstd_decompress_sync(&compressed).unwrap();
        assert_eq!(decompressed, data);
    }
}
