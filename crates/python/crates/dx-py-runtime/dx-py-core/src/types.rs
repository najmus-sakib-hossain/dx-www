//! PyType - Type object and type system

use crate::header::{PyObjectHeader, TypeTag, ObjectFlags};
use std::sync::Arc;
use dashmap::DashMap;

/// Type object representing a Python type
pub struct PyType {
    /// Object header
    pub header: PyObjectHeader,
    /// Type name
    pub name: String,
    /// Base types (for MRO)
    pub bases: Vec<Arc<PyType>>,
    /// Method resolution order
    pub mro: Vec<Arc<PyType>>,
    /// Type attributes/methods
    pub dict: DashMap<String, PyTypeSlot>,
    /// Type flags
    pub type_flags: TypeFlags,
}

/// Type-specific flags
#[derive(Debug, Clone, Copy, Default)]
pub struct TypeFlags {
    pub is_abstract: bool,
    pub is_final: bool,
    pub has_gc: bool,
    pub has_dict: bool,
    pub has_slots: bool,
    pub is_basetype: bool,
}

/// Slot in a type's dictionary
#[derive(Clone)]
pub enum PyTypeSlot {
    /// A method (function)
    Method(Arc<dyn Fn() + Send + Sync>),
    /// A class method
    ClassMethod(Arc<dyn Fn() + Send + Sync>),
    /// A static method
    StaticMethod(Arc<dyn Fn() + Send + Sync>),
    /// A property
    Property {
        getter: Option<Arc<dyn Fn() + Send + Sync>>,
        setter: Option<Arc<dyn Fn() + Send + Sync>>,
        deleter: Option<Arc<dyn Fn() + Send + Sync>>,
    },
    /// A data descriptor
    Data(Arc<dyn std::any::Any + Send + Sync>),
}

impl PyType {
    /// Create a new type
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            header: PyObjectHeader::new(TypeTag::Type, ObjectFlags::NONE),
            name: name.into(),
            bases: Vec::new(),
            mro: Vec::new(),
            dict: DashMap::new(),
            type_flags: TypeFlags::default(),
        }
    }
    
    /// Create a new type with bases
    pub fn with_bases(name: impl Into<String>, bases: Vec<Arc<PyType>>) -> Self {
        let mut ty = Self::new(name);
        ty.bases = bases;
        ty.compute_mro();
        ty
    }
    
    /// Compute method resolution order (C3 linearization)
    pub fn compute_mro(&mut self) {
        // Simplified MRO: just self + bases in order
        // Full C3 linearization would be more complex
        self.mro.clear();
        // Note: We'd need Arc<Self> here, simplified for now
    }
    
    /// Check if this type is a subtype of another
    pub fn is_subtype(&self, other: &PyType) -> bool {
        if std::ptr::eq(self, other) {
            return true;
        }
        self.bases.iter().any(|base| base.is_subtype(other))
    }
    
    /// Get an attribute from the type
    pub fn get_attr(&self, name: &str) -> Option<PyTypeSlot> {
        self.dict.get(name).map(|v| v.clone())
    }
    
    /// Set an attribute on the type
    pub fn set_attr(&self, name: impl Into<String>, slot: PyTypeSlot) {
        self.dict.insert(name.into(), slot);
    }
}

/// Built-in type singletons
pub mod builtin_types {
    use super::*;
    use std::sync::OnceLock;
    
    static TYPE_NONE: OnceLock<Arc<PyType>> = OnceLock::new();
    static TYPE_BOOL: OnceLock<Arc<PyType>> = OnceLock::new();
    static TYPE_INT: OnceLock<Arc<PyType>> = OnceLock::new();
    static TYPE_FLOAT: OnceLock<Arc<PyType>> = OnceLock::new();
    static TYPE_STR: OnceLock<Arc<PyType>> = OnceLock::new();
    static TYPE_LIST: OnceLock<Arc<PyType>> = OnceLock::new();
    static TYPE_TUPLE: OnceLock<Arc<PyType>> = OnceLock::new();
    static TYPE_DICT: OnceLock<Arc<PyType>> = OnceLock::new();
    
    pub fn none_type() -> Arc<PyType> {
        TYPE_NONE.get_or_init(|| Arc::new(PyType::new("NoneType"))).clone()
    }
    
    pub fn bool_type() -> Arc<PyType> {
        TYPE_BOOL.get_or_init(|| Arc::new(PyType::new("bool"))).clone()
    }
    
    pub fn int_type() -> Arc<PyType> {
        TYPE_INT.get_or_init(|| Arc::new(PyType::new("int"))).clone()
    }
    
    pub fn float_type() -> Arc<PyType> {
        TYPE_FLOAT.get_or_init(|| Arc::new(PyType::new("float"))).clone()
    }
    
    pub fn str_type() -> Arc<PyType> {
        TYPE_STR.get_or_init(|| Arc::new(PyType::new("str"))).clone()
    }
    
    pub fn list_type() -> Arc<PyType> {
        TYPE_LIST.get_or_init(|| Arc::new(PyType::new("list"))).clone()
    }
    
    pub fn tuple_type() -> Arc<PyType> {
        TYPE_TUPLE.get_or_init(|| Arc::new(PyType::new("tuple"))).clone()
    }
    
    pub fn dict_type() -> Arc<PyType> {
        TYPE_DICT.get_or_init(|| Arc::new(PyType::new("dict"))).clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_type_creation() {
        let ty = PyType::new("MyClass");
        assert_eq!(ty.name, "MyClass");
        assert_eq!(ty.header.type_tag(), TypeTag::Type);
    }
    
    #[test]
    fn test_builtin_types() {
        let int_type = builtin_types::int_type();
        assert_eq!(int_type.name, "int");
        
        let str_type = builtin_types::str_type();
        assert_eq!(str_type.name, "str");
    }
}
