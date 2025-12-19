//! Property tests for Node.js error code correctness.
//!
//! Feature: dx-js-compatibility, Property 19: Error Code Correctness
//! Validates: Requirements 29.1, 29.4, 29.5
//!
//! Property: For any file operation on a non-existent path, the error SHALL have code ENOENT.
//! For any file operation with insufficient permissions, the error SHALL have code EACCES.

use dx_compat_node::error::{ErrorCode, NodeError};
use dx_compat_node::fs;
use proptest::prelude::*;
use std::io;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    /// Property 19: Error Code Correctness - ENOENT for non-existent paths
    /// For any file operation on a non-existent path, the error SHALL have code ENOENT.
    #[test]
    fn error_code_enoent_for_nonexistent_path(
        filename in "[a-zA-Z0-9_]{1,20}\\.(txt|json|rs|md)"
    ) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            // Generate a path that definitely doesn't exist
            let nonexistent_path = format!("/nonexistent_dir_12345/{}", filename);
            
            let result = fs::read_file(&nonexistent_path).await;
            
            prop_assert!(result.is_err(), "Expected error for non-existent path");
            let err = result.unwrap_err();
            prop_assert_eq!(
                err.code, 
                ErrorCode::ENOENT,
                "Expected ENOENT error code for non-existent path, got {:?}",
                err.code
            );
        });
    }

    /// Property 19: Error Code Correctness - ErrorCode from IO error kind
    /// Verifies that IO error kinds are correctly mapped to Node.js error codes.
    #[test]
    fn error_code_from_io_error_kind_mapping(kind_idx in 0usize..8) {
        let test_cases = [
            (io::ErrorKind::NotFound, ErrorCode::ENOENT),
            (io::ErrorKind::PermissionDenied, ErrorCode::EACCES),
            (io::ErrorKind::AlreadyExists, ErrorCode::EEXIST),
            (io::ErrorKind::TimedOut, ErrorCode::ETIMEDOUT),
            (io::ErrorKind::ConnectionRefused, ErrorCode::ECONNREFUSED),
            (io::ErrorKind::ConnectionReset, ErrorCode::ECONNRESET),
            (io::ErrorKind::ConnectionAborted, ErrorCode::ECONNABORTED),
            (io::ErrorKind::InvalidInput, ErrorCode::EINVAL),
        ];
        
        let (io_kind, expected_code) = test_cases[kind_idx];
        let actual_code = ErrorCode::from_io_error_kind(io_kind);
        
        prop_assert_eq!(
            actual_code,
            expected_code,
            "IO error kind {:?} should map to {:?}, got {:?}",
            io_kind,
            expected_code,
            actual_code
        );
    }

    /// Property 19: Error Code Correctness - Error message contains code
    /// Verifies that error messages include the error code string.
    #[test]
    fn error_message_contains_code(path in "[a-zA-Z0-9_/]{1,50}") {
        let err = NodeError::enoent(&path);
        prop_assert!(
            err.message.contains("ENOENT"),
            "ENOENT error message should contain 'ENOENT'"
        );
        prop_assert_eq!(err.code, ErrorCode::ENOENT);
        prop_assert_eq!(err.path, Some(path.clone()));

        let err = NodeError::eacces(&path);
        prop_assert!(
            err.message.contains("EACCES"),
            "EACCES error message should contain 'EACCES'"
        );
        prop_assert_eq!(err.code, ErrorCode::EACCES);
        prop_assert_eq!(err.path, Some(path.clone()));

        let err = NodeError::eexist(&path);
        prop_assert!(
            err.message.contains("EEXIST"),
            "EEXIST error message should contain 'EEXIST'"
        );
        prop_assert_eq!(err.code, ErrorCode::EEXIST);
    }

    /// Property 19: Error Code Correctness - Error code string representation
    /// Verifies that error codes have correct string representations.
    #[test]
    fn error_code_string_representation(code_idx in 0usize..17) {
        let codes = [
            (ErrorCode::ENOENT, "ENOENT"),
            (ErrorCode::EACCES, "EACCES"),
            (ErrorCode::EEXIST, "EEXIST"),
            (ErrorCode::EISDIR, "EISDIR"),
            (ErrorCode::ENOTDIR, "ENOTDIR"),
            (ErrorCode::ENOTEMPTY, "ENOTEMPTY"),
            (ErrorCode::ETIMEDOUT, "ETIMEDOUT"),
            (ErrorCode::ECONNREFUSED, "ECONNREFUSED"),
            (ErrorCode::EINVAL, "EINVAL"),
            (ErrorCode::EBADF, "EBADF"),
            (ErrorCode::EBUSY, "EBUSY"),
            (ErrorCode::ECANCELED, "ECANCELED"),
            (ErrorCode::ECHILD, "ECHILD"),
            (ErrorCode::ECONNABORTED, "ECONNABORTED"),
            (ErrorCode::ECONNRESET, "ECONNRESET"),
            (ErrorCode::EXDEV, "EXDEV"),
            (ErrorCode::UNKNOWN, "UNKNOWN"),
        ];
        
        let (code, expected_str) = codes[code_idx];
        prop_assert_eq!(
            code.as_str(),
            expected_str,
            "Error code {:?} should have string representation '{}'",
            code,
            expected_str
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_enoent_on_read_nonexistent_file() {
        let result = fs::read_file("/this/path/does/not/exist.txt").await;
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.code, ErrorCode::ENOENT);
    }

    #[tokio::test]
    async fn test_enoent_on_stat_nonexistent_file() {
        let result = fs::stat("/this/path/does/not/exist.txt").await;
        assert!(result.is_err());
    }

    #[test]
    fn test_error_code_numeric_values() {
        // Verify error codes have expected negative values (POSIX convention)
        assert_eq!(ErrorCode::ENOENT as i32, -2);
        assert_eq!(ErrorCode::EACCES as i32, -13);
        assert_eq!(ErrorCode::EEXIST as i32, -17);
    }
}
