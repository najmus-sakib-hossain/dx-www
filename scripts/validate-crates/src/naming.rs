//! Naming convention validation module
//!
//! Validates crate directory names and ensures consistency with Cargo.toml package names.

use regex::Regex;
use std::path::Path;

/// Result of naming validation
#[derive(Debug, Clone)]
pub struct NamingValidation {
    /// Whether the directory name is valid kebab-case
    pub is_valid_kebab_case: bool,
    /// Whether the directory name matches the package name
    pub names_match: bool,
    /// The directory name
    pub dir_name: String,
    /// The package name from Cargo.toml
    pub package_name: String,
    /// Suggested fix if names don't match
    pub suggested_name: Option<String>,
    /// Whether the name contains typos
    pub has_typo: bool,
    /// Description of the typo if found
    pub typo_description: Option<String>,
}

/// Known typos to check for
const KNOWN_TYPOS: &[(&str, &str, &str)] = &[
    ("compability", "compatibility", "COMPABILITY should be COMPATIBILITY"),
    ("compatability", "compatibility", "COMPATABILITY should be COMPATIBILITY"),
    ("dependancies", "dependencies", "DEPENDANCIES should be DEPENDENCIES"),
    ("seperate", "separate", "SEPERATE should be SEPARATE"),
    ("occured", "occurred", "OCCURED should be OCCURRED"),
    ("recieve", "receive", "RECIEVE should be RECEIVE"),
];

/// Validate naming conventions for a crate
pub fn validate_naming(dir_path: &Path, package_name: &str) -> NamingValidation {
    let dir_name = dir_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("")
        .to_string();

    let is_valid_kebab_case = is_valid_kebab_case_name(&dir_name);
    let normalized_dir = normalize_name(&dir_name);
    let normalized_pkg = normalize_name(package_name);
    let names_match = normalized_dir == normalized_pkg;

    let suggested_name = if !names_match {
        Some(suggest_name(&dir_name, package_name))
    } else {
        None
    };

    let (has_typo, typo_description) = check_for_typos(&dir_name);

    NamingValidation {
        is_valid_kebab_case,
        names_match,
        dir_name,
        package_name: package_name.to_string(),
        suggested_name,
        has_typo,
        typo_description,
    }
}

/// Check if a name follows kebab-case convention
/// Valid: starts with lowercase letter, contains only lowercase letters, digits, and hyphens
pub fn is_valid_kebab_case_name(name: &str) -> bool {
    let kebab_case_pattern = Regex::new(r"^[a-z][a-z0-9-]*$").unwrap();
    kebab_case_pattern.is_match(name)
}

/// Normalize a name for comparison (convert underscores to hyphens, lowercase)
fn normalize_name(name: &str) -> String {
    name.to_lowercase().replace('_', "-")
}

/// Suggest a corrected name based on directory and package names
fn suggest_name(_dir_name: &str, package_name: &str) -> String {
    // If package name is valid kebab-case, suggest using it
    if is_valid_kebab_case_name(package_name) {
        return package_name.to_string();
    }
    
    // Otherwise, convert package name to kebab-case
    package_name.to_lowercase().replace('_', "-")
}

/// Check for known typos in the name
fn check_for_typos(name: &str) -> (bool, Option<String>) {
    let lower_name = name.to_lowercase();
    
    for (typo, _correct, description) in KNOWN_TYPOS {
        if lower_name.contains(typo) {
            return (true, Some(description.to_string()));
        }
    }
    
    // Check for malformed names (concatenated words without separators)
    if looks_malformed(&lower_name) {
        return (true, Some(format!("Name '{}' appears malformed (missing separators)", name)));
    }
    
    (false, None)
}

/// Check if a name looks malformed (e.g., concatenated paths or words)
fn looks_malformed(name: &str) -> bool {
    // Check for patterns that suggest concatenated paths
    let suspicious_patterns = [
        "crates",      // Likely a path fragment
        "runtime",     // Common word that might be concatenated
        "benches",     // Common directory name
        "core",        // Common module name
    ];
    
    // Count how many suspicious patterns appear
    let pattern_count = suspicious_patterns
        .iter()
        .filter(|p| name.contains(*p))
        .count();
    
    // If multiple patterns appear, it's likely malformed
    if pattern_count >= 2 {
        return true;
    }
    
    // Check for very long names without hyphens (likely concatenated)
    if name.len() > 30 && !name.contains('-') {
        return true;
    }
    
    false
}

/// Validate a file name for typos
pub fn validate_filename(filename: &str) -> Option<String> {
    let lower_name = filename.to_lowercase();
    
    for (typo, correct, _description) in KNOWN_TYPOS {
        if lower_name.contains(typo) {
            // Preserve original case pattern by replacing in the original string
            let typo_start = lower_name.find(typo)?;
            let typo_end = typo_start + typo.len();
            
            // Check if original was uppercase
            let original_segment = &filename[typo_start..typo_end];
            let is_uppercase = original_segment.chars().all(|c| c.is_uppercase() || !c.is_alphabetic());
            
            let replacement = if is_uppercase {
                correct.to_uppercase()
            } else if original_segment.chars().next().map(|c| c.is_uppercase()).unwrap_or(false) {
                // Title case
                let mut chars: Vec<char> = correct.chars().collect();
                if let Some(first) = chars.first_mut() {
                    *first = first.to_uppercase().next().unwrap_or(*first);
                }
                chars.into_iter().collect()
            } else {
                correct.to_string()
            };
            
            let mut result = filename.to_string();
            result.replace_range(typo_start..typo_end, &replacement);
            return Some(result);
        }
    }
    
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_kebab_case() {
        // Valid names
        assert!(is_valid_kebab_case_name("my-crate"));
        assert!(is_valid_kebab_case_name("dx-www"));
        assert!(is_valid_kebab_case_name("a"));
        assert!(is_valid_kebab_case_name("crate123"));
        assert!(is_valid_kebab_case_name("my-crate-2"));
        
        // Invalid names
        assert!(!is_valid_kebab_case_name("My-Crate"));      // Uppercase
        assert!(!is_valid_kebab_case_name("my_crate"));      // Underscore
        assert!(!is_valid_kebab_case_name("123crate"));      // Starts with number
        assert!(!is_valid_kebab_case_name("-my-crate"));     // Starts with hyphen
        assert!(!is_valid_kebab_case_name(""));              // Empty
        assert!(!is_valid_kebab_case_name("my crate"));      // Space
    }

    #[test]
    fn test_validate_naming_match() {
        let result = validate_naming(Path::new("/crates/my-crate"), "my-crate");
        assert!(result.is_valid_kebab_case);
        assert!(result.names_match);
        assert!(result.suggested_name.is_none());
    }

    #[test]
    fn test_validate_naming_underscore_conversion() {
        let result = validate_naming(Path::new("/crates/my-crate"), "my_crate");
        assert!(result.is_valid_kebab_case);
        assert!(result.names_match); // Should match after normalization
    }

    #[test]
    fn test_validate_naming_mismatch() {
        let result = validate_naming(Path::new("/crates/old-name"), "new-name");
        assert!(result.is_valid_kebab_case);
        assert!(!result.names_match);
        assert!(result.suggested_name.is_some());
    }

    #[test]
    fn test_check_for_typos() {
        let (has_typo, desc) = check_for_typos("compability");
        assert!(has_typo);
        assert!(desc.is_some());
        
        let (has_typo, _) = check_for_typos("compatibility");
        assert!(!has_typo);
    }

    #[test]
    fn test_looks_malformed() {
        assert!(looks_malformed("cratesdx-py-runtimedx-py-corebenches"));
        assert!(!looks_malformed("dx-runtime"));
        assert!(!looks_malformed("my-crate"));
    }

    #[test]
    fn test_validate_filename() {
        assert_eq!(
            validate_filename("COMPABILITY.md"),
            Some("COMPATIBILITY.md".to_string())
        );
        assert_eq!(validate_filename("README.md"), None);
    }
}
