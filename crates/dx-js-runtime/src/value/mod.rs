//! JavaScript value representation

pub mod object;
pub mod string;
pub mod tagged;

use std::fmt;

/// JavaScript value
#[derive(Clone, Debug)]
pub enum Value {
    /// Undefined
    Undefined,
    /// Null
    Null,
    /// Boolean
    Boolean(bool),
    /// Number (f64)
    Number(f64),
    /// String
    String(String),
    /// Object
    Object(object::Object),
    /// Array
    Array(Vec<Value>),
    /// Function
    Function(FunctionValue),
}

#[derive(Clone, Debug)]
pub struct FunctionValue {
    pub name: String,
    pub ptr: usize,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Undefined => write!(f, "undefined"),
            Value::Null => write!(f, "null"),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Object(_) => write!(f, "[object Object]"),
            Value::Array(arr) => {
                write!(f, "[")?;
                for (i, v) in arr.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", v)?;
                }
                write!(f, "]")
            }
            Value::Function(func) => write!(f, "[Function: {}]", func.name),
        }
    }
}

impl Value {
    /// Check if value is truthy
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Undefined | Value::Null => false,
            Value::Boolean(b) => *b,
            Value::Number(n) => *n != 0.0 && !n.is_nan(),
            Value::String(s) => !s.is_empty(),
            _ => true,
        }
    }

    /// Convert to number
    pub fn to_number(&self) -> f64 {
        match self {
            Value::Undefined => f64::NAN,
            Value::Null => 0.0,
            Value::Boolean(b) => {
                if *b {
                    1.0
                } else {
                    0.0
                }
            }
            Value::Number(n) => *n,
            Value::String(s) => s.parse().unwrap_or(f64::NAN),
            _ => f64::NAN,
        }
    }

    /// Convert to string
    pub fn to_js_string(&self) -> String {
        format!("{}", self)
    }
}
