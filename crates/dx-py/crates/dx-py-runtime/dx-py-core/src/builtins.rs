//! Built-in functions for DX-Py runtime

use crate::pylist::PyValue;
use crate::pyfunction::PyBuiltinFunction;
use crate::{CoreError, CoreResult};
use std::sync::Arc;

/// Create the print builtin
pub fn builtin_print() -> PyBuiltinFunction {
    PyBuiltinFunction::new("print", |args| {
        let output: Vec<String> = args.iter().map(|v| format_value(v)).collect();
        println!("{}", output.join(" "));
        Ok(PyValue::None)
    })
}

/// Create the len builtin
pub fn builtin_len() -> PyBuiltinFunction {
    PyBuiltinFunction::new("len", |args| {
        match args.first() {
            Some(PyValue::Str(s)) => Ok(PyValue::Int(s.chars().count() as i64)),
            Some(PyValue::List(l)) => Ok(PyValue::Int(l.len() as i64)),
            Some(PyValue::Tuple(t)) => Ok(PyValue::Int(t.len() as i64)),
            Some(PyValue::Dict(d)) => Ok(PyValue::Int(d.len() as i64)),
            Some(v) => Err(CoreError::TypeError(format!(
                "object of type '{}' has no len()", v.type_name()
            ))),
            None => Err(CoreError::TypeError("len() takes exactly one argument".into())),
        }
    })
}


/// Create the type builtin
pub fn builtin_type() -> PyBuiltinFunction {
    PyBuiltinFunction::new("type", |args| {
        match args.first() {
            Some(v) => Ok(PyValue::Str(Arc::from(v.type_name()))),
            None => Err(CoreError::TypeError("type() takes 1 or 3 arguments".into())),
        }
    })
}

/// Create the int builtin
pub fn builtin_int() -> PyBuiltinFunction {
    PyBuiltinFunction::new("int", |args| {
        match args.first() {
            Some(PyValue::Int(i)) => Ok(PyValue::Int(*i)),
            Some(PyValue::Float(f)) => Ok(PyValue::Int(*f as i64)),
            Some(PyValue::Bool(b)) => Ok(PyValue::Int(*b as i64)),
            Some(PyValue::Str(s)) => {
                s.parse::<i64>()
                    .map(PyValue::Int)
                    .map_err(|_| CoreError::ValueError(format!(
                        "invalid literal for int(): '{}'", s
                    )))
            }
            Some(v) => Err(CoreError::TypeError(format!(
                "int() argument must be a string or number, not '{}'", v.type_name()
            ))),
            None => Ok(PyValue::Int(0)),
        }
    })
}

/// Create the float builtin
pub fn builtin_float() -> PyBuiltinFunction {
    PyBuiltinFunction::new("float", |args| {
        match args.first() {
            Some(PyValue::Float(f)) => Ok(PyValue::Float(*f)),
            Some(PyValue::Int(i)) => Ok(PyValue::Float(*i as f64)),
            Some(PyValue::Bool(b)) => Ok(PyValue::Float(*b as i64 as f64)),
            Some(PyValue::Str(s)) => {
                s.parse::<f64>()
                    .map(PyValue::Float)
                    .map_err(|_| CoreError::ValueError(format!(
                        "could not convert string to float: '{}'", s
                    )))
            }
            Some(v) => Err(CoreError::TypeError(format!(
                "float() argument must be a string or number, not '{}'", v.type_name()
            ))),
            None => Ok(PyValue::Float(0.0)),
        }
    })
}

/// Create the str builtin
pub fn builtin_str() -> PyBuiltinFunction {
    PyBuiltinFunction::new("str", |args| {
        match args.first() {
            Some(v) => Ok(PyValue::Str(Arc::from(format_value(v)))),
            None => Ok(PyValue::Str(Arc::from(""))),
        }
    })
}

/// Create the bool builtin
pub fn builtin_bool() -> PyBuiltinFunction {
    PyBuiltinFunction::new("bool", |args| {
        match args.first() {
            Some(v) => Ok(PyValue::Bool(v.to_bool())),
            None => Ok(PyValue::Bool(false)),
        }
    })
}

/// Create the abs builtin
pub fn builtin_abs() -> PyBuiltinFunction {
    PyBuiltinFunction::new("abs", |args| {
        match args.first() {
            Some(PyValue::Int(i)) => Ok(PyValue::Int(i.abs())),
            Some(PyValue::Float(f)) => Ok(PyValue::Float(f.abs())),
            Some(v) => Err(CoreError::TypeError(format!(
                "bad operand type for abs(): '{}'", v.type_name()
            ))),
            None => Err(CoreError::TypeError("abs() takes exactly one argument".into())),
        }
    })
}

/// Create the min builtin
pub fn builtin_min() -> PyBuiltinFunction {
    PyBuiltinFunction::new("min", |args| {
        if args.is_empty() {
            return Err(CoreError::TypeError("min expected at least 1 argument".into()));
        }
        
        let mut min_val = args[0].clone();
        for arg in &args[1..] {
            if compare_values(arg, &min_val)? < 0 {
                min_val = arg.clone();
            }
        }
        Ok(min_val)
    })
}

/// Create the max builtin
pub fn builtin_max() -> PyBuiltinFunction {
    PyBuiltinFunction::new("max", |args| {
        if args.is_empty() {
            return Err(CoreError::TypeError("max expected at least 1 argument".into()));
        }
        
        let mut max_val = args[0].clone();
        for arg in &args[1..] {
            if compare_values(arg, &max_val)? > 0 {
                max_val = arg.clone();
            }
        }
        Ok(max_val)
    })
}

/// Create the sum builtin
pub fn builtin_sum() -> PyBuiltinFunction {
    PyBuiltinFunction::new("sum", |args| {
        match args.first() {
            Some(PyValue::List(list)) => {
                let mut total: i64 = 0;
                for item in list.to_vec() {
                    match item {
                        PyValue::Int(i) => total += i,
                        _ => return Err(CoreError::TypeError(
                            "unsupported operand type for sum".into()
                        )),
                    }
                }
                Ok(PyValue::Int(total))
            }
            _ => Err(CoreError::TypeError("sum() argument must be iterable".into())),
        }
    })
}

/// Create the range builtin (returns a list for simplicity)
pub fn builtin_range() -> PyBuiltinFunction {
    PyBuiltinFunction::new("range", |args| {
        let (start, stop, step) = match args.len() {
            1 => match &args[0] {
                PyValue::Int(stop) => (0, *stop, 1),
                _ => return Err(CoreError::TypeError("range() integer expected".into())),
            },
            2 => match (&args[0], &args[1]) {
                (PyValue::Int(start), PyValue::Int(stop)) => (*start, *stop, 1),
                _ => return Err(CoreError::TypeError("range() integer expected".into())),
            },
            3 => match (&args[0], &args[1], &args[2]) {
                (PyValue::Int(start), PyValue::Int(stop), PyValue::Int(step)) => {
                    if *step == 0 {
                        return Err(CoreError::ValueError("range() step cannot be zero".into()));
                    }
                    (*start, *stop, *step)
                }
                _ => return Err(CoreError::TypeError("range() integer expected".into())),
            },
            _ => return Err(CoreError::TypeError(
                "range expected at most 3 arguments".into()
            )),
        };
        
        let mut result = Vec::new();
        let mut i = start;
        if step > 0 {
            while i < stop {
                result.push(PyValue::Int(i));
                i += step;
            }
        } else {
            while i > stop {
                result.push(PyValue::Int(i));
                i += step;
            }
        }
        
        Ok(PyValue::List(Arc::new(crate::PyList::from_values(result))))
    })
}

/// Format a value for display
fn format_value(value: &PyValue) -> String {
    match value {
        PyValue::None => "None".to_string(),
        PyValue::Bool(b) => if *b { "True" } else { "False" }.to_string(),
        PyValue::Int(i) => i.to_string(),
        PyValue::Float(f) => format!("{}", f),
        PyValue::Str(s) => s.to_string(),
        PyValue::List(l) => {
            let items: Vec<String> = l.to_vec().iter().map(repr_value).collect();
            format!("[{}]", items.join(", "))
        }
        PyValue::Tuple(t) => {
            let items: Vec<String> = t.to_vec().iter().map(repr_value).collect();
            if items.len() == 1 {
                format!("({},)", items[0])
            } else {
                format!("({})", items.join(", "))
            }
        }
        PyValue::Dict(d) => {
            let items: Vec<String> = d.items()
                .iter()
                .map(|(k, v)| format!("{}: {}", format!("{:?}", k), repr_value(v)))
                .collect();
            format!("{{{}}}", items.join(", "))
        }
    }
}

/// Repr a value (with quotes for strings)
fn repr_value(value: &PyValue) -> String {
    match value {
        PyValue::Str(s) => format!("'{}'", s),
        _ => format_value(value),
    }
}

/// Compare two values, returns -1, 0, or 1
fn compare_values(a: &PyValue, b: &PyValue) -> CoreResult<i32> {
    match (a, b) {
        (PyValue::Int(x), PyValue::Int(y)) => Ok(x.cmp(y) as i32),
        (PyValue::Float(x), PyValue::Float(y)) => {
            Ok(x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal) as i32)
        }
        (PyValue::Int(x), PyValue::Float(y)) => {
            Ok((*x as f64).partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal) as i32)
        }
        (PyValue::Float(x), PyValue::Int(y)) => {
            Ok(x.partial_cmp(&(*y as f64)).unwrap_or(std::cmp::Ordering::Equal) as i32)
        }
        (PyValue::Str(x), PyValue::Str(y)) => Ok(x.cmp(y) as i32),
        _ => Err(CoreError::TypeError(format!(
            "'<' not supported between '{}' and '{}'",
            a.type_name(), b.type_name()
        ))),
    }
}

/// Get all builtin functions
pub fn get_builtins() -> Vec<PyBuiltinFunction> {
    vec![
        builtin_print(),
        builtin_len(),
        builtin_type(),
        builtin_int(),
        builtin_float(),
        builtin_str(),
        builtin_bool(),
        builtin_abs(),
        builtin_min(),
        builtin_max(),
        builtin_sum(),
        builtin_range(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_builtin_len() {
        let len_fn = builtin_len();
        
        let result = len_fn.call(&[PyValue::Str(Arc::from("hello"))]).unwrap();
        assert!(matches!(result, PyValue::Int(5)));
    }
    
    #[test]
    fn test_builtin_int() {
        let int_fn = builtin_int();
        
        let result = int_fn.call(&[PyValue::Str(Arc::from("42"))]).unwrap();
        assert!(matches!(result, PyValue::Int(42)));
        
        let result = int_fn.call(&[PyValue::Float(3.14)]).unwrap();
        assert!(matches!(result, PyValue::Int(3)));
    }
    
    #[test]
    fn test_builtin_abs() {
        let abs_fn = builtin_abs();
        
        let result = abs_fn.call(&[PyValue::Int(-42)]).unwrap();
        assert!(matches!(result, PyValue::Int(42)));
    }
    
    #[test]
    fn test_builtin_range() {
        let range_fn = builtin_range();
        
        let result = range_fn.call(&[PyValue::Int(5)]).unwrap();
        if let PyValue::List(list) = result {
            assert_eq!(list.len(), 5);
        } else {
            panic!("Expected list");
        }
    }
}
