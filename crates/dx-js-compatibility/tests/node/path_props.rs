//! Property tests for Node.js path module.
//!
//! Feature: dx-js-compatibility, Property 11: Path Operations Correctness
//! Validates: Requirements 3.1, 3.2, 3.6, 3.7
//!
//! Property: For any array of path segments, `path.join()` followed by `path.normalize()`
//! SHALL produce a valid path, and `path.isAbsolute(path.resolve(segments))` SHALL return true.

use dx_compat_node::path;
use proptest::prelude::*;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    /// Property 11: Path Operations Correctness - resolve produces absolute paths
    /// For any array of path segments, path.resolve() SHALL produce an absolute path.
    #[test]
    fn path_resolve_produces_absolute(
        segments in prop::collection::vec("[a-zA-Z0-9_]{1,20}", 1..5)
    ) {
        let segment_refs: Vec<&str> = segments.iter().map(|s| s.as_str()).collect();
        let resolved = path::resolve(&segment_refs);
        let resolved_str = resolved.to_string_lossy();

        prop_assert!(
            path::is_absolute(&resolved_str),
            "path.resolve() should produce an absolute path, got: {}",
            resolved_str
        );
    }

    /// Property 11: Path Operations Correctness - join then normalize is idempotent
    /// For any path segments, normalize(join(segments)) should not contain . or ..
    #[test]
    fn path_join_normalize_removes_dots(
        segments in prop::collection::vec("[a-zA-Z0-9_]{1,10}", 1..5)
    ) {
        let segment_refs: Vec<&str> = segments.iter().map(|s| s.as_str()).collect();
        let joined = path::join(&segment_refs);
        let normalized = path::normalize(&joined.to_string_lossy());
        let normalized_str = normalized.to_string_lossy();

        // Normalized path should not contain standalone . or .. components
        let components: Vec<_> = normalized_str.split(path::SEP).collect();
        for component in &components {
            prop_assert!(
                *component != "." && *component != "..",
                "Normalized path should not contain . or .., got: {}",
                normalized_str
            );
        }
    }

    /// Property 11: Path Operations Correctness - dirname/basename round-trip
    /// For any path with a file, join(dirname(path), basename(path)) should reconstruct the path.
    #[test]
    fn path_dirname_basename_round_trip(
        dir_parts in prop::collection::vec("[a-zA-Z0-9_]{1,10}", 1..3),
        filename in "[a-zA-Z0-9_]{1,10}\\.[a-zA-Z]{1,4}"
    ) {
        let mut full_parts = dir_parts.clone();
        full_parts.push(filename.clone());
        let part_refs: Vec<&str> = full_parts.iter().map(|s| s.as_str()).collect();
        
        let full_path = path::join(&part_refs);
        let full_path_str = full_path.to_string_lossy();

        let dir = path::dirname(&full_path_str);
        let base = path::basename(&full_path_str, None);

        // Reconstruct path
        let reconstructed = path::join(&[dir, &base]);
        let reconstructed_str = reconstructed.to_string_lossy();

        // Normalize both for comparison
        let original_normalized = path::normalize(&full_path_str);
        let reconstructed_normalized = path::normalize(&reconstructed_str);

        prop_assert_eq!(
            original_normalized,
            reconstructed_normalized,
            "join(dirname, basename) should reconstruct the original path"
        );
    }

    /// Property 11: Path Operations Correctness - extname extracts extension
    /// For any filename with extension, extname should return the extension with dot.
    #[test]
    fn path_extname_extracts_extension(
        name in "[a-zA-Z0-9_]{1,10}",
        ext in "[a-zA-Z]{1,4}"
    ) {
        let filename = format!("{}.{}", name, ext);
        let extracted_ext = path::extname(&filename);

        prop_assert_eq!(
            extracted_ext,
            format!(".{}", ext),
            "extname should return extension with dot"
        );
    }

    /// Property 11: Path Operations Correctness - basename with extension removal
    /// basename(path, ext) should remove the extension from the result.
    #[test]
    fn path_basename_removes_extension(
        name in "[a-zA-Z0-9_]{1,10}",
        ext in "[a-zA-Z]{1,4}"
    ) {
        let filename = format!("{}.{}", name, ext);
        let ext_with_dot = format!(".{}", ext);
        
        let base_without_ext = path::basename(&filename, Some(&ext_with_dot));

        prop_assert_eq!(
            base_without_ext,
            name,
            "basename with extension should return name without extension"
        );
    }

    /// Property 11: Path Operations Correctness - normalize is idempotent
    /// normalize(normalize(path)) should equal normalize(path).
    #[test]
    fn path_normalize_is_idempotent(
        segments in prop::collection::vec("[a-zA-Z0-9_]{1,10}", 1..5)
    ) {
        let segment_refs: Vec<&str> = segments.iter().map(|s| s.as_str()).collect();
        let joined = path::join(&segment_refs);
        let joined_str = joined.to_string_lossy();

        let normalized_once = path::normalize(&joined_str);
        let normalized_twice = path::normalize(&normalized_once.to_string_lossy());

        prop_assert_eq!(
            normalized_once,
            normalized_twice,
            "normalize should be idempotent"
        );
    }

    /// Property 11: Path Operations Correctness - join with empty segments
    /// join should handle empty segments gracefully.
    #[test]
    fn path_join_handles_empty_segments(
        non_empty in "[a-zA-Z0-9_]{1,10}",
        empty_positions in prop::collection::vec(0usize..3, 0..3)
    ) {
        let mut segments: Vec<&str> = vec![&non_empty];
        for pos in empty_positions {
            if pos < segments.len() {
                segments.insert(pos, "");
            }
        }

        let joined = path::join(&segments);
        let joined_str = joined.to_string_lossy();

        // Result should contain the non-empty segment
        prop_assert!(
            joined_str.contains(&non_empty),
            "join result should contain non-empty segment"
        );
    }

    /// Property 11: Path Operations Correctness - parse produces consistent components
    /// parse(path) should produce components that can reconstruct the path.
    #[test]
    fn path_parse_produces_consistent_components(
        dir in "[a-zA-Z0-9_]{1,10}",
        name in "[a-zA-Z0-9_]{1,10}",
        ext in "[a-zA-Z]{1,4}"
    ) {
        let filename = format!("{}.{}", name, ext);
        let full_path = path::join(&[&dir, &filename]);
        let full_path_str = full_path.to_string_lossy();

        let parsed = path::parse(&full_path_str);

        prop_assert_eq!(
            parsed.base,
            filename,
            "parsed.base should equal the filename"
        );
        prop_assert_eq!(
            parsed.ext,
            format!(".{}", ext),
            "parsed.ext should equal the extension with dot"
        );
        prop_assert_eq!(
            parsed.name,
            name,
            "parsed.name should equal the name without extension"
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sep_is_platform_specific() {
        #[cfg(unix)]
        assert_eq!(path::SEP, '/');
        #[cfg(windows)]
        assert_eq!(path::SEP, '\\');
    }

    #[test]
    fn test_delimiter_is_platform_specific() {
        #[cfg(unix)]
        assert_eq!(path::DELIMITER, ':');
        #[cfg(windows)]
        assert_eq!(path::DELIMITER, ';');
    }

    #[test]
    fn test_is_absolute() {
        #[cfg(unix)]
        {
            assert!(path::is_absolute("/foo/bar"));
            assert!(!path::is_absolute("foo/bar"));
            assert!(!path::is_absolute("./foo"));
        }
        #[cfg(windows)]
        {
            assert!(path::is_absolute("C:\\foo\\bar"));
            assert!(!path::is_absolute("foo\\bar"));
        }
    }

    #[test]
    fn test_normalize_removes_dots() {
        let result = path::normalize("foo/./bar/../baz");
        let result_str = result.to_string_lossy();
        assert!(!result_str.contains(".."));
        assert!(result_str.contains("foo"));
        assert!(result_str.contains("baz"));
    }

    #[test]
    fn test_extname_no_extension() {
        assert_eq!(path::extname("filename"), "");
        assert_eq!(path::extname(".gitignore"), "");
    }

    #[test]
    fn test_dirname_root() {
        #[cfg(unix)]
        assert_eq!(path::dirname("/"), "");
    }
}
