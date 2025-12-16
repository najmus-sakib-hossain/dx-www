//! Core data types for DX format

use rustc_hash::FxHashMap;
use std::fmt;

/// The root value type in DX format
#[derive(Debug, Clone, PartialEq)]
pub enum DxValue {
    /// Null value (~)
    Null,
    /// Boolean (+ or -)
    Bool(bool),
    /// Integer
    Int(i64),
    /// Float
    Float(f64),
    /// String (no quotes in machine format)
    String(String),
    /// Array/List
    Array(DxArray),
    /// Object/Map
    Object(DxObject),
    /// Table (schema-defined array of objects)
    Table(DxTable),
    /// Reference to an anchor (@N)
    Ref(usize),
}

/// A DX array (inline or vertical)
#[derive(Debug, Clone, PartialEq)]
pub struct DxArray {
    pub values: Vec<DxValue>,
    /// Whether this was a stream (>)
    pub is_stream: bool,
}

impl DxArray {
    pub fn new() -> Self {
        Self {
            values: Vec::new(),
            is_stream: false,
        }
    }

    pub fn with_capacity(cap: usize) -> Self {
        Self {
            values: Vec::with_capacity(cap),
            is_stream: false,
        }
    }

    pub fn stream(values: Vec<DxValue>) -> Self {
        Self {
            values,
            is_stream: true,
        }
    }
}

impl Default for DxArray {
    fn default() -> Self {
        Self::new()
    }
}

/// A DX object (key-value pairs)
#[derive(Debug, Clone, PartialEq)]
pub struct DxObject {
    /// Ordered key-value pairs
    pub fields: Vec<(String, DxValue)>,
    /// Fast lookup map (key index)
    lookup: FxHashMap<String, usize>,
}

impl DxObject {
    pub fn new() -> Self {
        Self {
            fields: Vec::new(),
            lookup: FxHashMap::default(),
        }
    }

    pub fn with_capacity(cap: usize) -> Self {
        Self {
            fields: Vec::with_capacity(cap),
            lookup: FxHashMap::with_capacity_and_hasher(cap, Default::default()),
        }
    }

    pub fn insert(&mut self, key: String, value: DxValue) {
        if let Some(&idx) = self.lookup.get(&key) {
            self.fields[idx].1 = value;
        } else {
            let idx = self.fields.len();
            self.fields.push((key.clone(), value));
            self.lookup.insert(key, idx);
        }
    }

    pub fn get(&self, key: &str) -> Option<&DxValue> {
        self.lookup.get(key).map(|&idx| &self.fields[idx].1)
    }

    pub fn iter(&self) -> impl Iterator<Item = &(String, DxValue)> {
        self.fields.iter()
    }
}

impl Default for DxObject {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for DxObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        for (i, (k, v)) in self.fields.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}: {:?}", k, v)?;
        }
        write!(f, "}}")
    }
}

/// A table with schema-defined columns
#[derive(Debug, Clone, PartialEq)]
pub struct DxTable {
    pub schema: crate::schema::Schema,
    pub rows: Vec<Vec<DxValue>>,
}

impl DxTable {
    pub fn new(schema: crate::schema::Schema) -> Self {
        Self {
            schema,
            rows: Vec::new(),
        }
    }

    pub fn add_row(&mut self, row: Vec<DxValue>) -> Result<(), String> {
        if row.len() != self.schema.columns.len() {
            return Err(format!(
                "Row length {} doesn't match schema length {}",
                row.len(),
                self.schema.columns.len()
            ));
        }
        self.rows.push(row);
        Ok(())
    }

    pub fn row_count(&self) -> usize {
        self.rows.len()
    }

    pub fn column_count(&self) -> usize {
        self.schema.columns.len()
    }
}

impl DxValue {
    /// Check if this value is "empty" for ditto logic
    pub fn is_empty(&self) -> bool {
        matches!(self, DxValue::Null)
    }

    /// Get type name for error messages
    pub fn type_name(&self) -> &'static str {
        match self {
            DxValue::Null => "null",
            DxValue::Bool(_) => "bool",
            DxValue::Int(_) => "int",
            DxValue::Float(_) => "float",
            DxValue::String(_) => "string",
            DxValue::Array(_) => "array",
            DxValue::Object(_) => "object",
            DxValue::Table(_) => "table",
            DxValue::Ref(_) => "ref",
        }
    }

    /// Convert to boolean if possible
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            DxValue::Bool(b) => Some(*b),
            _ => None,
        }
    }

    /// Convert to integer if possible
    pub fn as_int(&self) -> Option<i64> {
        match self {
            DxValue::Int(i) => Some(*i),
            DxValue::Float(f) => Some(*f as i64),
            _ => None,
        }
    }

    /// Convert to float if possible
    pub fn as_float(&self) -> Option<f64> {
        match self {
            DxValue::Float(f) => Some(*f),
            DxValue::Int(i) => Some(*i as f64),
            _ => None,
        }
    }

    /// Convert to string if possible
    pub fn as_str(&self) -> Option<&str> {
        match self {
            DxValue::String(s) => Some(s),
            _ => None,
        }
    }
}
