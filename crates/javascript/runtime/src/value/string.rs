//! Interned string implementation

use std::collections::HashMap;
use std::sync::RwLock;

/// Global string interner
static STRING_INTERNER: RwLock<Option<StringInterner>> = RwLock::new(None);

/// String interner for deduplication
pub struct StringInterner {
    strings: HashMap<String, u32>,
    by_id: Vec<String>,
}

impl StringInterner {
    /// Create a new interner
    pub fn new() -> Self {
        Self {
            strings: HashMap::new(),
            by_id: Vec::new(),
        }
    }

    /// Intern a string
    pub fn intern(&mut self, s: &str) -> u32 {
        if let Some(&id) = self.strings.get(s) {
            return id;
        }

        let id = self.by_id.len() as u32;
        self.strings.insert(s.to_string(), id);
        self.by_id.push(s.to_string());
        id
    }

    /// Get string by ID
    pub fn get(&self, id: u32) -> Option<&str> {
        self.by_id.get(id as usize).map(|s| s.as_str())
    }
}

impl Default for StringInterner {
    fn default() -> Self {
        Self::new()
    }
}

/// Initialize the global interner
pub fn init_interner() {
    let mut guard = STRING_INTERNER.write().unwrap();
    if guard.is_none() {
        *guard = Some(StringInterner::new());
    }
}

/// Intern a string globally
pub fn intern(s: &str) -> u32 {
    init_interner();
    let mut guard = STRING_INTERNER.write().unwrap();
    guard.as_mut().unwrap().intern(s)
}

/// Get interned string
pub fn get_interned(id: u32) -> Option<String> {
    let guard = STRING_INTERNER.read().unwrap();
    guard.as_ref().and_then(|i| i.get(id).map(|s| s.to_string()))
}
