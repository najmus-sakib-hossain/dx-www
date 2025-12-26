//! PyDict - Python dictionary type

use crate::header::{PyObjectHeader, TypeTag, ObjectFlags};
use crate::pylist::PyValue;
use crate::{CoreError, CoreResult};
use dashmap::DashMap;
use std::hash::{Hash, Hasher};
use std::sync::Arc;

/// Hashable key for dictionary
#[derive(Clone, Eq)]
pub enum PyKey {
    None,
    Bool(bool),
    Int(i64),
    Str(Arc<str>),
    Tuple(Vec<PyKey>),
}

impl PartialEq for PyKey {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (PyKey::None, PyKey::None) => true,
            (PyKey::Bool(a), PyKey::Bool(b)) => a == b,
            (PyKey::Int(a), PyKey::Int(b)) => a == b,
            (PyKey::Str(a), PyKey::Str(b)) => a == b,
            (PyKey::Tuple(a), PyKey::Tuple(b)) => a == b,
            // Cross-type comparisons for bool/int
            (PyKey::Bool(b), PyKey::Int(i)) | (PyKey::Int(i), PyKey::Bool(b)) => {
                *i == (*b as i64)
            }
            _ => false,
        }
    }
}

impl Hash for PyKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            PyKey::None => 0u8.hash(state),
            PyKey::Bool(b) => (*b as i64).hash(state),
            PyKey::Int(i) => i.hash(state),
            PyKey::Str(s) => s.hash(state),
            PyKey::Tuple(t) => {
                for item in t {
                    item.hash(state);
                }
            }
        }
    }
}

impl std::fmt::Debug for PyKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PyKey::None => write!(f, "None"),
            PyKey::Bool(b) => write!(f, "{}", if *b { "True" } else { "False" }),
            PyKey::Int(i) => write!(f, "{}", i),
            PyKey::Str(s) => write!(f, "'{}'", s),
            PyKey::Tuple(t) => write!(f, "{:?}", t),
        }
    }
}

impl PyKey {
    /// Try to convert a PyValue to a PyKey
    pub fn from_value(value: &PyValue) -> CoreResult<Self> {
        match value {
            PyValue::None => Ok(PyKey::None),
            PyValue::Bool(b) => Ok(PyKey::Bool(*b)),
            PyValue::Int(i) => Ok(PyKey::Int(*i)),
            PyValue::Str(s) => Ok(PyKey::Str(Arc::clone(s))),
            PyValue::Tuple(t) => {
                let keys: CoreResult<Vec<PyKey>> = t.to_vec()
                    .iter()
                    .map(PyKey::from_value)
                    .collect();
                Ok(PyKey::Tuple(keys?))
            }
            _ => Err(CoreError::TypeError(format!(
                "unhashable type: '{}'", value.type_name()
            ))),
        }
    }
    
    /// Convert key to value
    pub fn to_value(&self) -> PyValue {
        match self {
            PyKey::None => PyValue::None,
            PyKey::Bool(b) => PyValue::Bool(*b),
            PyKey::Int(i) => PyValue::Int(*i),
            PyKey::Str(s) => PyValue::Str(Arc::clone(s)),
            PyKey::Tuple(t) => {
                let values: Vec<PyValue> = t.iter().map(|k| k.to_value()).collect();
                PyValue::Tuple(Arc::new(crate::PyTuple::from_values(values)))
            }
        }
    }
}

/// Python dictionary object
pub struct PyDict {
    /// Object header
    pub header: PyObjectHeader,
    /// Dictionary entries (thread-safe)
    entries: DashMap<PyKey, PyValue>,
}

impl PyDict {
    /// Create a new empty dictionary
    pub fn new() -> Self {
        Self {
            header: PyObjectHeader::new(TypeTag::Dict, ObjectFlags::NONE),
            entries: DashMap::new(),
        }
    }
    
    /// Create with capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            header: PyObjectHeader::new(TypeTag::Dict, ObjectFlags::NONE),
            entries: DashMap::with_capacity(capacity),
        }
    }
    
    /// Get length
    #[inline]
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    
    /// Check if empty
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    
    /// Get item by key
    pub fn getitem(&self, key: &PyKey) -> CoreResult<PyValue> {
        self.entries
            .get(key)
            .map(|v| v.clone())
            .ok_or_else(|| CoreError::KeyError(format!("{:?}", key)))
    }
    
    /// Get item with default
    pub fn get(&self, key: &PyKey, default: PyValue) -> PyValue {
        self.entries
            .get(key)
            .map(|v| v.clone())
            .unwrap_or(default)
    }
    
    /// Set item
    pub fn setitem(&self, key: PyKey, value: PyValue) {
        self.entries.insert(key, value);
    }
    
    /// Delete item
    pub fn delitem(&self, key: &PyKey) -> CoreResult<()> {
        self.entries
            .remove(key)
            .map(|_| ())
            .ok_or_else(|| CoreError::KeyError(format!("{:?}", key)))
    }
    
    /// Check if contains key
    pub fn contains(&self, key: &PyKey) -> bool {
        self.entries.contains_key(key)
    }
    
    /// Clear the dictionary
    pub fn clear(&self) {
        self.entries.clear();
    }
    
    /// Get all keys
    pub fn keys(&self) -> Vec<PyKey> {
        self.entries.iter().map(|r| r.key().clone()).collect()
    }
    
    /// Get all values
    pub fn values(&self) -> Vec<PyValue> {
        self.entries.iter().map(|r| r.value().clone()).collect()
    }
    
    /// Get all items as (key, value) pairs
    pub fn items(&self) -> Vec<(PyKey, PyValue)> {
        self.entries
            .iter()
            .map(|r| (r.key().clone(), r.value().clone()))
            .collect()
    }
    
    /// Pop item with key
    pub fn pop(&self, key: &PyKey, default: Option<PyValue>) -> CoreResult<PyValue> {
        match self.entries.remove(key) {
            Some((_, v)) => Ok(v),
            None => default.ok_or_else(|| CoreError::KeyError(format!("{:?}", key))),
        }
    }
    
    /// Pop arbitrary item
    pub fn popitem(&self) -> CoreResult<(PyKey, PyValue)> {
        // Get first key
        let key = self.entries
            .iter()
            .next()
            .map(|r| r.key().clone());
        
        match key {
            Some(k) => {
                let v = self.entries.remove(&k).unwrap().1;
                Ok((k, v))
            }
            None => Err(CoreError::KeyError("dictionary is empty".into())),
        }
    }
    
    /// Set default value if key doesn't exist
    pub fn setdefault(&self, key: PyKey, default: PyValue) -> PyValue {
        self.entries
            .entry(key)
            .or_insert(default)
            .clone()
    }
    
    /// Update with items from another dict
    pub fn update(&self, other: &PyDict) {
        for item in other.entries.iter() {
            self.entries.insert(item.key().clone(), item.value().clone());
        }
    }
    
    /// Create a shallow copy
    pub fn copy(&self) -> PyDict {
        let new_dict = PyDict::new();
        for item in self.entries.iter() {
            new_dict.entries.insert(item.key().clone(), item.value().clone());
        }
        new_dict
    }
}

impl Default for PyDict {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for PyDict {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PyDict({{")?;
        let items: Vec<_> = self.entries.iter().collect();
        for (i, item) in items.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{:?}: {:?}", item.key(), item.value())?;
        }
        write!(f, "}})")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_dict_creation() {
        let dict = PyDict::new();
        assert!(dict.is_empty());
        assert_eq!(dict.header.type_tag(), TypeTag::Dict);
    }
    
    #[test]
    fn test_dict_set_get() {
        let dict = PyDict::new();
        dict.setitem(PyKey::Str(Arc::from("key")), PyValue::Int(42));
        
        let value = dict.getitem(&PyKey::Str(Arc::from("key"))).unwrap();
        if let PyValue::Int(v) = value {
            assert_eq!(v, 42);
        } else {
            panic!("Expected Int");
        }
    }
    
    #[test]
    fn test_dict_contains() {
        let dict = PyDict::new();
        dict.setitem(PyKey::Int(1), PyValue::Str(Arc::from("one")));
        
        assert!(dict.contains(&PyKey::Int(1)));
        assert!(!dict.contains(&PyKey::Int(2)));
    }
    
    #[test]
    fn test_dict_delete() {
        let dict = PyDict::new();
        dict.setitem(PyKey::Int(1), PyValue::Int(100));
        
        assert!(dict.delitem(&PyKey::Int(1)).is_ok());
        assert!(dict.is_empty());
    }
    
    #[test]
    fn test_dict_keys_values() {
        let dict = PyDict::new();
        dict.setitem(PyKey::Int(1), PyValue::Str(Arc::from("a")));
        dict.setitem(PyKey::Int(2), PyValue::Str(Arc::from("b")));
        
        assert_eq!(dict.keys().len(), 2);
        assert_eq!(dict.values().len(), 2);
    }
    
    #[test]
    fn test_dict_update() {
        let dict1 = PyDict::new();
        dict1.setitem(PyKey::Int(1), PyValue::Int(1));
        
        let dict2 = PyDict::new();
        dict2.setitem(PyKey::Int(2), PyValue::Int(2));
        
        dict1.update(&dict2);
        assert_eq!(dict1.len(), 2);
    }
}
