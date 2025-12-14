/// DX Serializer: Mapping Management
/// 
/// Loads and manages variable name mappings from .dx/serializer/mappings.dx
/// Enables bidirectional conversion between machine and human formats

use std::collections::HashMap;
use std::path::PathBuf;
use std::fs;
use std::sync::OnceLock;

static MAPPINGS: OnceLock<Mappings> = OnceLock::new();

pub struct Mappings {
    /// Short key → Full name (machine → human)
    pub expand: HashMap<String, String>,
    /// Full name → Short key (human → machine)
    pub compress: HashMap<String, String>,
}

impl Mappings {
    /// Load mappings from .dx/serializer/mappings.dx
    pub fn load() -> Result<Self, String> {
        let mapping_path = Self::find_mappings_file()?;
        let content = fs::read_to_string(&mapping_path)
            .map_err(|e| format!("Failed to read mappings file: {}", e))?;
        
        Self::parse(&content)
    }
    
    /// Parse mappings from content
    fn parse(content: &str) -> Result<Self, String> {
        let mut expand = HashMap::new();
        let mut compress = HashMap::new();
        
        for line in content.lines() {
            let line = line.trim();
            
            // Skip comments and empty lines
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            
            // Parse mapping: short_key=full_name
            if let Some((short, full)) = line.split_once('=') {
                let short = short.trim().to_string();
                let full = full.trim().to_string();
                
                expand.insert(short.clone(), full.clone());
                compress.insert(full, short);
            }
        }
        
        Ok(Self { expand, compress })
    }
    
    /// Find the mappings file (.dx/serializer/mappings.dx)
    fn find_mappings_file() -> Result<PathBuf, String> {
        // Start from current directory and search upwards
        let mut current = std::env::current_dir()
            .map_err(|e| format!("Failed to get current directory: {}", e))?;
        
        loop {
            let mappings_path = current.join(".dx").join("serializer").join("mappings.dx");
            if mappings_path.exists() {
                return Ok(mappings_path);
            }
            
            // Try parent directory
            if !current.pop() {
                break;
            }
        }
        
        Err("Could not find .dx/serializer/mappings.dx in current directory or parents".to_string())
    }
    
    /// Get global mappings instance (lazy load)
    pub fn get() -> &'static Mappings {
        MAPPINGS.get_or_init(|| {
            Self::load().unwrap_or_else(|e| {
                eprintln!("Warning: Failed to load mappings: {}. Using defaults.", e);
                Self::default()
            })
        })
    }
    
    /// Expand short key to full name (machine → human)
    /// 
    /// **Smart Logic:**
    /// - If key is POPULAR (exists in mappings): expand it (n → name)
    /// - If key is CUSTOM (not in mappings): keep as-is (myKey → myKey)
    /// 
    /// Examples:
    /// - `expand_key("n")` → "name" (popular)
    /// - `expand_key("myCustomKey")` → "myCustomKey" (custom, preserved)
    pub fn expand_key(&self, key: &str) -> String {
        self.expand.get(key).cloned().unwrap_or_else(|| key.to_string())
    }
    
    /// Compress full name to short key (human → machine)
    /// 
    /// **Smart Logic:**
    /// - If key is POPULAR (exists in mappings): abbreviate it (name → n)
    /// - If key is CUSTOM (not in mappings): keep as-is (myKey → myKey)
    /// 
    /// Examples:
    /// - `compress_key("name")` → "n" (popular)
    /// - `compress_key("myCustomKey")` → "myCustomKey" (custom, preserved)
    pub fn compress_key(&self, key: &str) -> String {
        self.compress.get(key).cloned().unwrap_or_else(|| key.to_string())
    }
}

impl Default for Mappings {
    fn default() -> Self {
        // Fallback mappings if file can't be loaded
        let mut expand = HashMap::new();
        let mut compress = HashMap::new();
        
        let defaults = [
            ("n", "name"),
            ("v", "version"),
            ("t", "title"),
            ("d", "description"),
            ("a", "author"),
            ("c", "context"),
            ("l", "languages"),
            ("f", "forge"),
            ("s", "style"),
            ("m", "media"),
            ("i", "i18n"),
            ("u", "ui"),
        ];
        
        for (short, full) in defaults {
            expand.insert(short.to_string(), full.to_string());
            compress.insert(full.to_string(), short.to_string());
        }
        
        Self { expand, compress }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_mappings() {
        let content = r#"
# Comment
n=name
v=version
c=context
"#;
        
        let mappings = Mappings::parse(content).unwrap();
        assert_eq!(mappings.expand_key("n"), "name");
        assert_eq!(mappings.expand_key("v"), "version");
        assert_eq!(mappings.compress_key("name"), "n");
        assert_eq!(mappings.compress_key("version"), "v");
    }
    
    #[test]
    fn test_roundtrip() {
        let mappings = Mappings::default();
        let short = "n";
        let full = mappings.expand_key(short);
        let back = mappings.compress_key(&full);
        assert_eq!(short, back);
    }
}
