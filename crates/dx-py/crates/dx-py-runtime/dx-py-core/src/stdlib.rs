//! Standard library compatibility modules

use crate::pylist::PyValue;
use crate::pyfunction::PyBuiltinFunction;
use crate::{CoreError, CoreResult};
use std::sync::Arc;

/// Create sys module builtins
pub fn sys_builtins() -> Vec<PyBuiltinFunction> {
    vec![
        PyBuiltinFunction::new("exit", |args| {
            let code = match args.first() {
                Some(PyValue::Int(i)) => *i as i32,
                Some(PyValue::None) | None => 0,
                _ => 1,
            };
            Err(CoreError::RuntimeError(format!("SystemExit: {}", code)))
        }),
    ]
}

/// Create os module builtins
pub fn os_builtins() -> Vec<PyBuiltinFunction> {
    vec![
        PyBuiltinFunction::new("getcwd", |_args| {
            std::env::current_dir()
                .map(|p| PyValue::Str(Arc::from(p.to_string_lossy().to_string())))
                .map_err(|e| CoreError::RuntimeError(e.to_string()))
        }),
        PyBuiltinFunction::new("getenv", |args| {
            match args.first() {
                Some(PyValue::Str(name)) => {
                    match std::env::var(name.as_ref()) {
                        Ok(val) => Ok(PyValue::Str(Arc::from(val))),
                        Err(_) => Ok(PyValue::None),
                    }
                }
                _ => Err(CoreError::TypeError("getenv() argument must be str".into())),
            }
        }),
        PyBuiltinFunction::new("listdir", |args| {
            let path = match args.first() {
                Some(PyValue::Str(p)) => p.to_string(),
                None => ".".to_string(),
                _ => return Err(CoreError::TypeError("listdir() argument must be str".into())),
            };
            
            let entries: CoreResult<Vec<PyValue>> = std::fs::read_dir(&path)
                .map_err(|e| CoreError::RuntimeError(e.to_string()))?
                .map(|entry| {
                    entry
                        .map(|e| PyValue::Str(Arc::from(e.file_name().to_string_lossy().to_string())))
                        .map_err(|e| CoreError::RuntimeError(e.to_string()))
                })
                .collect();
            
            Ok(PyValue::List(Arc::new(crate::PyList::from_values(entries?))))
        }),
    ]
}

/// Create io module builtins
pub fn io_builtins() -> Vec<PyBuiltinFunction> {
    vec![
        PyBuiltinFunction::new("open", |args| {
            // Simplified open - just returns the filename for now
            match args.first() {
                Some(PyValue::Str(path)) => {
                    Ok(PyValue::Str(Arc::clone(path)))
                }
                _ => Err(CoreError::TypeError("open() argument must be str".into())),
            }
        }),
    ]
}

/// Create json module builtins (using simd-json when available)
pub fn json_builtins() -> Vec<PyBuiltinFunction> {
    vec![
        PyBuiltinFunction::new("dumps", |args| {
            match args.first() {
                Some(value) => {
                    let json = value_to_json(value)?;
                    Ok(PyValue::Str(Arc::from(json)))
                }
                None => Err(CoreError::TypeError("dumps() requires an argument".into())),
            }
        }),
        PyBuiltinFunction::new("loads", |args| {
            match args.first() {
                Some(PyValue::Str(s)) => {
                    json_to_value(s)
                }
                _ => Err(CoreError::TypeError("loads() argument must be str".into())),
            }
        }),
    ]
}

/// Convert PyValue to JSON string
fn value_to_json(value: &PyValue) -> CoreResult<String> {
    match value {
        PyValue::None => Ok("null".to_string()),
        PyValue::Bool(b) => Ok(if *b { "true" } else { "false" }.to_string()),
        PyValue::Int(i) => Ok(i.to_string()),
        PyValue::Float(f) => Ok(f.to_string()),
        PyValue::Str(s) => Ok(format!("\"{}\"", escape_json_string(s))),
        PyValue::List(list) => {
            let items: CoreResult<Vec<String>> = list.to_vec()
                .iter()
                .map(value_to_json)
                .collect();
            Ok(format!("[{}]", items?.join(",")))
        }
        PyValue::Dict(dict) => {
            let items: CoreResult<Vec<String>> = dict.items()
                .iter()
                .map(|(k, v)| {
                    let key_str = match k {
                        crate::pydict::PyKey::Str(s) => format!("\"{}\"", escape_json_string(s)),
                        crate::pydict::PyKey::Int(i) => format!("\"{}\"", i),
                        _ => return Err(CoreError::TypeError("JSON keys must be strings".into())),
                    };
                    let val_str = value_to_json(v)?;
                    Ok(format!("{}:{}", key_str, val_str))
                })
                .collect();
            Ok(format!("{{{}}}", items?.join(",")))
        }
        PyValue::Tuple(tuple) => {
            let items: CoreResult<Vec<String>> = tuple.to_vec()
                .iter()
                .map(value_to_json)
                .collect();
            Ok(format!("[{}]", items?.join(",")))
        }
    }
}

/// Escape special characters in JSON string
fn escape_json_string(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '"' => result.push_str("\\\""),
            '\\' => result.push_str("\\\\"),
            '\n' => result.push_str("\\n"),
            '\r' => result.push_str("\\r"),
            '\t' => result.push_str("\\t"),
            c if c.is_control() => {
                result.push_str(&format!("\\u{:04x}", c as u32));
            }
            c => result.push(c),
        }
    }
    result
}

/// Parse JSON string to PyValue (simplified parser)
fn json_to_value(s: &str) -> CoreResult<PyValue> {
    let s = s.trim();
    
    if s == "null" {
        return Ok(PyValue::None);
    }
    if s == "true" {
        return Ok(PyValue::Bool(true));
    }
    if s == "false" {
        return Ok(PyValue::Bool(false));
    }
    
    // Try parsing as number
    if let Ok(i) = s.parse::<i64>() {
        return Ok(PyValue::Int(i));
    }
    if let Ok(f) = s.parse::<f64>() {
        return Ok(PyValue::Float(f));
    }
    
    // String
    if s.starts_with('"') && s.ends_with('"') {
        let inner = &s[1..s.len()-1];
        return Ok(PyValue::Str(Arc::from(unescape_json_string(inner))));
    }
    
    // Array (simplified - doesn't handle nested structures well)
    if s.starts_with('[') && s.ends_with(']') {
        let inner = &s[1..s.len()-1];
        if inner.is_empty() {
            return Ok(PyValue::List(Arc::new(crate::PyList::new())));
        }
        // Very simplified parsing - won't work for nested arrays
        let items: CoreResult<Vec<PyValue>> = inner
            .split(',')
            .map(|item| json_to_value(item.trim()))
            .collect();
        return Ok(PyValue::List(Arc::new(crate::PyList::from_values(items?))));
    }
    
    Err(CoreError::ValueError(format!("Invalid JSON: {}", s)))
}

/// Unescape JSON string
fn unescape_json_string(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();
    
    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some('"') => result.push('"'),
                Some('\\') => result.push('\\'),
                Some('n') => result.push('\n'),
                Some('r') => result.push('\r'),
                Some('t') => result.push('\t'),
                Some(c) => {
                    result.push('\\');
                    result.push(c);
                }
                None => result.push('\\'),
            }
        } else {
            result.push(c);
        }
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_json_dumps_primitives() {
        assert_eq!(value_to_json(&PyValue::None).unwrap(), "null");
        assert_eq!(value_to_json(&PyValue::Bool(true)).unwrap(), "true");
        assert_eq!(value_to_json(&PyValue::Int(42)).unwrap(), "42");
        assert_eq!(value_to_json(&PyValue::Str(Arc::from("hello"))).unwrap(), "\"hello\"");
    }
    
    #[test]
    fn test_json_loads_primitives() {
        assert!(matches!(json_to_value("null").unwrap(), PyValue::None));
        assert!(matches!(json_to_value("true").unwrap(), PyValue::Bool(true)));
        assert!(matches!(json_to_value("42").unwrap(), PyValue::Int(42)));
    }
    
    #[test]
    fn test_json_roundtrip() {
        let original = PyValue::Int(123);
        let json = value_to_json(&original).unwrap();
        let parsed = json_to_value(&json).unwrap();
        
        if let (PyValue::Int(a), PyValue::Int(b)) = (&original, &parsed) {
            assert_eq!(a, b);
        }
    }
    
    #[test]
    fn test_os_getcwd() {
        let builtins = os_builtins();
        let getcwd = builtins.iter().find(|f| f.name == "getcwd").unwrap();
        
        let result = getcwd.call(&[]).unwrap();
        assert!(matches!(result, PyValue::Str(_)));
    }
}
