//! Configuration serialization and validation
//!
//! Custom serialization and validation for configuration.

/// Configuration errors
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    /// Failed to read config file
    #[error("Failed to read config file: {0}")]
    IoError(#[from] std::io::Error),
    
    /// Failed to parse TOML
    #[error("Failed to parse TOML: {0}")]
    TomlParseError(#[from] toml::de::Error),
    
    /// Failed to serialize TOML
    #[error("Failed to serialize TOML: {0}")]
    TomlSerializeError(#[from] toml::ser::Error),
    
    /// Invalid configuration value
    #[error("Invalid configuration value: {field} - {message}")]
    ValidationError {
        field: String,
        message: String,
    },
}

impl ConfigError {
    /// Create a validation error
    pub fn validation(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self::ValidationError {
            field: field.into(),
            message: message.into(),
        }
    }
}

/// Validate a configuration value
pub fn validate_index_url(url: &str) -> Result<(), ConfigError> {
    if url.is_empty() {
        return Err(ConfigError::validation("index_url", "URL cannot be empty"));
    }
    
    if !url.starts_with("http://") && !url.starts_with("https://") {
        return Err(ConfigError::validation(
            "index_url",
            "URL must start with http:// or https://",
        ));
    }
    
    Ok(())
}

/// Validate max concurrent downloads
pub fn validate_max_concurrent_downloads(max: u32) -> Result<(), ConfigError> {
    if max == 0 {
        return Err(ConfigError::validation(
            "max_concurrent_downloads",
            "Must be at least 1",
        ));
    }
    
    if max > 100 {
        return Err(ConfigError::validation(
            "max_concurrent_downloads",
            "Must be at most 100",
        ));
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_index_url() {
        assert!(validate_index_url("https://pypi.org/simple").is_ok());
        assert!(validate_index_url("http://localhost:8080").is_ok());
        assert!(validate_index_url("").is_err());
        assert!(validate_index_url("ftp://invalid.com").is_err());
    }

    #[test]
    fn test_validate_max_concurrent_downloads() {
        assert!(validate_max_concurrent_downloads(1).is_ok());
        assert!(validate_max_concurrent_downloads(50).is_ok());
        assert!(validate_max_concurrent_downloads(100).is_ok());
        assert!(validate_max_concurrent_downloads(0).is_err());
        assert!(validate_max_concurrent_downloads(101).is_err());
    }
}
