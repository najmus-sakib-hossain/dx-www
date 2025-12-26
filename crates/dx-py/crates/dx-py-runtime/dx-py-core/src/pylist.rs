//! PyList - Python list type

use crate::header::{PyObjectHeader, TypeTag, ObjectFlags};
use crate::{CoreError, CoreResult};
use parking_lot::RwLock;
use std::sync::Arc;

/// A Python value (simplified for core types)
#[derive(Clone)]
pub enum PyValue {
    None,
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(Arc<str>),
    List(Arc<PyList>),
    Tuple(Arc<crate::PyTuple>),
    Dict(Arc<crate::PyDict>),
}

impl PyValue {
    pub fn type_name(&self) -> &'static str {
        match self {
            PyValue::None => "NoneType",
            PyValue::Bool(_) => "bool",
            PyValue::Int(_) => "int",
            PyValue::Float(_) => "float",
            PyValue::Str(_) => "str",
            PyValue::List(_) => "list",
            PyValue::Tuple(_) => "tuple",
            PyValue::Dict(_) => "dict",
        }
    }
    
    pub fn to_bool(&self) -> bool {
        match self {
            PyValue::None => false,
            PyValue::Bool(b) => *b,
            PyValue::Int(i) => *i != 0,
            PyValue::Float(f) => *f != 0.0,
            PyValue::Str(s) => !s.is_empty(),
            PyValue::List(l) => !l.is_empty(),
            PyValue::Tuple(t) => !t.is_empty(),
            PyValue::Dict(d) => !d.is_empty(),
        }
    }
}

impl std::fmt::Debug for PyValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PyValue::None => write!(f, "None"),
            PyValue::Bool(b) => write!(f, "{}", if *b { "True" } else { "False" }),
            PyValue::Int(i) => write!(f, "{}", i),
            PyValue::Float(fl) => write!(f, "{}", fl),
            PyValue::Str(s) => write!(f, "'{}'", s),
            PyValue::List(_) => write!(f, "[...]"),
            PyValue::Tuple(_) => write!(f, "(...)"),
            PyValue::Dict(_) => write!(f, "{{...}}"),
        }
    }
}

/// Python list object
pub struct PyList {
    /// Object header
    pub header: PyObjectHeader,
    /// List elements (thread-safe)
    elements: RwLock<Vec<PyValue>>,
}

impl PyList {
    /// Create a new empty list
    pub fn new() -> Self {
        Self {
            header: PyObjectHeader::new(TypeTag::List, ObjectFlags::NONE),
            elements: RwLock::new(Vec::new()),
        }
    }
    
    /// Create a list with capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            header: PyObjectHeader::new(TypeTag::List, ObjectFlags::NONE),
            elements: RwLock::new(Vec::with_capacity(capacity)),
        }
    }
    
    /// Create from values
    pub fn from_values(values: Vec<PyValue>) -> Self {
        Self {
            header: PyObjectHeader::new(TypeTag::List, ObjectFlags::NONE),
            elements: RwLock::new(values),
        }
    }
    
    /// Get length
    #[inline]
    pub fn len(&self) -> usize {
        self.elements.read().len()
    }
    
    /// Check if empty
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.elements.read().is_empty()
    }
    
    /// Get item at index
    pub fn getitem(&self, index: i64) -> CoreResult<PyValue> {
        let elements = self.elements.read();
        let len = elements.len() as i64;
        let idx = if index < 0 { len + index } else { index };
        
        if idx < 0 || idx >= len {
            return Err(CoreError::IndexError(format!(
                "list index out of range: {}", index
            )));
        }
        
        Ok(elements[idx as usize].clone())
    }
    
    /// Set item at index
    pub fn setitem(&self, index: i64, value: PyValue) -> CoreResult<()> {
        let mut elements = self.elements.write();
        let len = elements.len() as i64;
        let idx = if index < 0 { len + index } else { index };
        
        if idx < 0 || idx >= len {
            return Err(CoreError::IndexError(format!(
                "list assignment index out of range: {}", index
            )));
        }
        
        elements[idx as usize] = value;
        Ok(())
    }
    
    /// Append item
    pub fn append(&self, value: PyValue) {
        self.elements.write().push(value);
    }
    
    /// Insert item at index
    pub fn insert(&self, index: i64, value: PyValue) {
        let mut elements = self.elements.write();
        let len = elements.len() as i64;
        let idx = if index < 0 {
            (len + index + 1).max(0) as usize
        } else {
            (index as usize).min(elements.len())
        };
        elements.insert(idx, value);
    }
    
    /// Remove and return item at index
    pub fn pop(&self, index: Option<i64>) -> CoreResult<PyValue> {
        let mut elements = self.elements.write();
        if elements.is_empty() {
            return Err(CoreError::IndexError("pop from empty list".into()));
        }
        
        let len = elements.len() as i64;
        let idx = match index {
            Some(i) if i < 0 => (len + i) as usize,
            Some(i) => i as usize,
            None => elements.len() - 1,
        };
        
        if idx >= elements.len() {
            return Err(CoreError::IndexError("pop index out of range".into()));
        }
        
        Ok(elements.remove(idx))
    }
    
    /// Remove first occurrence of value
    pub fn remove(&self, value: &PyValue) -> CoreResult<()> {
        let mut elements = self.elements.write();
        for (i, elem) in elements.iter().enumerate() {
            if Self::values_equal(elem, value) {
                elements.remove(i);
                return Ok(());
            }
        }
        Err(CoreError::ValueError("list.remove(x): x not in list".into()))
    }
    
    /// Clear the list
    pub fn clear(&self) {
        self.elements.write().clear();
    }
    
    /// Extend with items from iterator
    pub fn extend(&self, items: impl IntoIterator<Item = PyValue>) {
        self.elements.write().extend(items);
    }
    
    /// Get slice
    pub fn slice(&self, start: Option<i64>, end: Option<i64>) -> PyList {
        let elements = self.elements.read();
        let len = elements.len() as i64;
        
        let start = match start {
            Some(s) if s < 0 => (len + s).max(0) as usize,
            Some(s) => (s as usize).min(elements.len()),
            None => 0,
        };
        
        let end = match end {
            Some(e) if e < 0 => (len + e).max(0) as usize,
            Some(e) => (e as usize).min(elements.len()),
            None => elements.len(),
        };
        
        if start >= end {
            return PyList::new();
        }
        
        PyList::from_values(elements[start..end].to_vec())
    }
    
    /// Reverse in place
    pub fn reverse(&self) {
        self.elements.write().reverse();
    }
    
    /// Sort in place (simplified - only works for homogeneous numeric lists)
    pub fn sort(&self) -> CoreResult<()> {
        let mut elements = self.elements.write();
        
        // Check if all elements are comparable (simplified: only ints)
        let all_ints = elements.iter().all(|v| matches!(v, PyValue::Int(_)));
        
        if all_ints {
            elements.sort_by(|a, b| {
                if let (PyValue::Int(x), PyValue::Int(y)) = (a, b) {
                    x.cmp(y)
                } else {
                    std::cmp::Ordering::Equal
                }
            });
            Ok(())
        } else {
            Err(CoreError::TypeError("'<' not supported between instances".into()))
        }
    }
    
    /// Count occurrences
    pub fn count(&self, value: &PyValue) -> usize {
        self.elements
            .read()
            .iter()
            .filter(|v| Self::values_equal(v, value))
            .count()
    }
    
    /// Find index of value
    pub fn index(&self, value: &PyValue) -> CoreResult<usize> {
        self.elements
            .read()
            .iter()
            .position(|v| Self::values_equal(v, value))
            .ok_or_else(|| CoreError::ValueError("value not in list".into()))
    }
    
    /// Check if contains value
    pub fn contains(&self, value: &PyValue) -> bool {
        self.elements
            .read()
            .iter()
            .any(|v| Self::values_equal(v, value))
    }
    
    /// Concatenate two lists
    pub fn concat(&self, other: &PyList) -> PyList {
        let mut elements = self.elements.read().clone();
        elements.extend(other.elements.read().iter().cloned());
        PyList::from_values(elements)
    }
    
    /// Repeat list n times
    pub fn repeat(&self, n: usize) -> PyList {
        let elements = self.elements.read();
        let mut result = Vec::with_capacity(elements.len() * n);
        for _ in 0..n {
            result.extend(elements.iter().cloned());
        }
        PyList::from_values(result)
    }
    
    /// Get all elements as a vector
    pub fn to_vec(&self) -> Vec<PyValue> {
        self.elements.read().clone()
    }
    
    /// Simple value equality check
    fn values_equal(a: &PyValue, b: &PyValue) -> bool {
        match (a, b) {
            (PyValue::None, PyValue::None) => true,
            (PyValue::Bool(x), PyValue::Bool(y)) => x == y,
            (PyValue::Int(x), PyValue::Int(y)) => x == y,
            (PyValue::Float(x), PyValue::Float(y)) => x == y,
            (PyValue::Str(x), PyValue::Str(y)) => x == y,
            _ => false,
        }
    }
}

impl Default for PyList {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for PyList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PyList({:?})", self.elements.read())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_list_creation() {
        let list = PyList::new();
        assert!(list.is_empty());
        assert_eq!(list.header.type_tag(), TypeTag::List);
    }
    
    #[test]
    fn test_list_append_get() {
        let list = PyList::new();
        list.append(PyValue::Int(1));
        list.append(PyValue::Int(2));
        list.append(PyValue::Int(3));
        
        assert_eq!(list.len(), 3);
        
        if let PyValue::Int(v) = list.getitem(0).unwrap() {
            assert_eq!(v, 1);
        } else {
            panic!("Expected Int");
        }
        
        if let PyValue::Int(v) = list.getitem(-1).unwrap() {
            assert_eq!(v, 3);
        } else {
            panic!("Expected Int");
        }
    }
    
    #[test]
    fn test_list_slice() {
        let list = PyList::from_values(vec![
            PyValue::Int(1),
            PyValue::Int(2),
            PyValue::Int(3),
            PyValue::Int(4),
            PyValue::Int(5),
        ]);
        
        let slice = list.slice(Some(1), Some(4));
        assert_eq!(slice.len(), 3);
    }
    
    #[test]
    fn test_list_pop() {
        let list = PyList::from_values(vec![
            PyValue::Int(1),
            PyValue::Int(2),
            PyValue::Int(3),
        ]);
        
        let popped = list.pop(None).unwrap();
        if let PyValue::Int(v) = popped {
            assert_eq!(v, 3);
        }
        assert_eq!(list.len(), 2);
    }
    
    #[test]
    fn test_list_sort() {
        let list = PyList::from_values(vec![
            PyValue::Int(3),
            PyValue::Int(1),
            PyValue::Int(2),
        ]);
        
        list.sort().unwrap();
        
        if let PyValue::Int(v) = list.getitem(0).unwrap() {
            assert_eq!(v, 1);
        }
    }
}
