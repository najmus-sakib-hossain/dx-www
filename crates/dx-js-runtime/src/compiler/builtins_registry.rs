//! Built-in JavaScript Objects and Functions
//!
//! This module provides native implementations of core JavaScript built-ins:
//! - Object (keys, values, entries, assign, freeze, etc.)
//! - Array (map, filter, reduce, sort, etc.)
//! - String (split, join, slice, replace, match, etc.)
//! - Number (toFixed, toString, parseInt, parseFloat)
//! - Math (floor, ceil, sqrt, sin, cos, random, etc.)
//! - JSON (parse, stringify)
//! - console (log, warn, error, time, etc.)
//! - Date, RegExp, Map, Set, etc.

use crate::value::Value;
use std::collections::HashMap;

/// Built-in function registry
pub struct BuiltinRegistry {
    functions: HashMap<String, BuiltinFunction>,
}

/// A built-in function pointer
pub type BuiltinFunction = fn(&[Value]) -> Value;

impl BuiltinRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            functions: HashMap::new(),
        };
        registry.register_all();
        registry
    }

    /// Register all built-in functions
    fn register_all(&mut self) {
        // Console methods
        self.register("console.log", builtin_console_log);
        self.register("console.warn", builtin_console_warn);
        self.register("console.error", builtin_console_error);
        self.register("console.time", builtin_console_time);
        self.register("console.timeEnd", builtin_console_time_end);

        // Math methods
        self.register("Math.floor", builtin_math_floor);
        self.register("Math.ceil", builtin_math_ceil);
        self.register("Math.round", builtin_math_round);
        self.register("Math.sqrt", builtin_math_sqrt);
        self.register("Math.abs", builtin_math_abs);
        self.register("Math.sin", builtin_math_sin);
        self.register("Math.cos", builtin_math_cos);
        self.register("Math.tan", builtin_math_tan);
        self.register("Math.min", builtin_math_min);
        self.register("Math.max", builtin_math_max);
        self.register("Math.pow", builtin_math_pow);
        self.register("Math.random", builtin_math_random);

        // Object methods
        self.register("Object.keys", builtin_object_keys);
        self.register("Object.values", builtin_object_values);
        self.register("Object.entries", builtin_object_entries);
        self.register("Object.assign", builtin_object_assign);
        self.register("Object.freeze", builtin_object_freeze);
        self.register("Object.seal", builtin_object_seal);

        // Array methods
        self.register("Array.isArray", builtin_array_is_array);
        self.register("Array.from", builtin_array_from);
        self.register("Array.of", builtin_array_of);

        // String methods
        self.register("String.fromCharCode", builtin_string_from_char_code);

        // Number methods
        self.register("Number.isNaN", builtin_number_is_nan);
        self.register("Number.isFinite", builtin_number_is_finite);
        self.register("Number.parseInt", builtin_number_parse_int);
        self.register("Number.parseFloat", builtin_number_parse_float);

        // JSON methods
        self.register("JSON.parse", builtin_json_parse);
        self.register("JSON.stringify", builtin_json_stringify);

        // Global functions
        self.register("parseInt", builtin_parse_int);
        self.register("parseFloat", builtin_parse_float);
        self.register("isNaN", builtin_is_nan);
        self.register("isFinite", builtin_is_finite);
    }

    fn register(&mut self, name: &str, func: BuiltinFunction) {
        self.functions.insert(name.to_string(), func);
    }

    pub fn get(&self, name: &str) -> Option<BuiltinFunction> {
        self.functions.get(name).copied()
    }
}

// ============================================================================
// Console Methods
// ============================================================================

fn builtin_console_log(args: &[Value]) -> Value {
    for (i, arg) in args.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", arg.to_string());
    }
    println!();
    Value::Undefined
}

fn builtin_console_warn(args: &[Value]) -> Value {
    eprint!("Warning: ");
    for (i, arg) in args.iter().enumerate() {
        if i > 0 {
            eprint!(" ");
        }
        eprint!("{}", arg.to_string());
    }
    eprintln!();
    Value::Undefined
}

fn builtin_console_error(args: &[Value]) -> Value {
    eprint!("Error: ");
    for (i, arg) in args.iter().enumerate() {
        if i > 0 {
            eprint!(" ");
        }
        eprint!("{}", arg.to_string());
    }
    eprintln!();
    Value::Undefined
}

fn builtin_console_time(_args: &[Value]) -> Value {
    // TODO: Implement timing
    Value::Undefined
}

fn builtin_console_time_end(_args: &[Value]) -> Value {
    // TODO: Implement timing
    Value::Undefined
}

// ============================================================================
// Math Methods
// ============================================================================

fn builtin_math_floor(args: &[Value]) -> Value {
    if let Some(Value::Number(n)) = args.first() {
        Value::Number(n.floor())
    } else {
        Value::Number(f64::NAN)
    }
}

fn builtin_math_ceil(args: &[Value]) -> Value {
    if let Some(Value::Number(n)) = args.first() {
        Value::Number(n.ceil())
    } else {
        Value::Number(f64::NAN)
    }
}

fn builtin_math_round(args: &[Value]) -> Value {
    if let Some(Value::Number(n)) = args.first() {
        Value::Number(n.round())
    } else {
        Value::Number(f64::NAN)
    }
}

fn builtin_math_sqrt(args: &[Value]) -> Value {
    if let Some(Value::Number(n)) = args.first() {
        Value::Number(n.sqrt())
    } else {
        Value::Number(f64::NAN)
    }
}

fn builtin_math_abs(args: &[Value]) -> Value {
    if let Some(Value::Number(n)) = args.first() {
        Value::Number(n.abs())
    } else {
        Value::Number(f64::NAN)
    }
}

fn builtin_math_sin(args: &[Value]) -> Value {
    if let Some(Value::Number(n)) = args.first() {
        Value::Number(n.sin())
    } else {
        Value::Number(f64::NAN)
    }
}

fn builtin_math_cos(args: &[Value]) -> Value {
    if let Some(Value::Number(n)) = args.first() {
        Value::Number(n.cos())
    } else {
        Value::Number(f64::NAN)
    }
}

fn builtin_math_tan(args: &[Value]) -> Value {
    if let Some(Value::Number(n)) = args.first() {
        Value::Number(n.tan())
    } else {
        Value::Number(f64::NAN)
    }
}

fn builtin_math_min(args: &[Value]) -> Value {
    let mut min = f64::INFINITY;
    for arg in args {
        if let Value::Number(n) = arg {
            if n < &min {
                min = *n;
            }
        }
    }
    Value::Number(min)
}

fn builtin_math_max(args: &[Value]) -> Value {
    let mut max = f64::NEG_INFINITY;
    for arg in args {
        if let Value::Number(n) = arg {
            if n > &max {
                max = *n;
            }
        }
    }
    Value::Number(max)
}

fn builtin_math_pow(args: &[Value]) -> Value {
    if args.len() >= 2 {
        if let (Value::Number(base), Value::Number(exp)) = (&args[0], &args[1]) {
            return Value::Number(base.powf(*exp));
        }
    }
    Value::Number(f64::NAN)
}

fn builtin_math_random(_args: &[Value]) -> Value {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    let random = ((nanos % 1_000_000) as f64) / 1_000_000.0;
    Value::Number(random)
}

// ============================================================================
// Object Methods
// ============================================================================

fn builtin_object_keys(_args: &[Value]) -> Value {
    // TODO: Implement Object.keys
    Value::Undefined
}

fn builtin_object_values(_args: &[Value]) -> Value {
    // TODO: Implement Object.values
    Value::Undefined
}

fn builtin_object_entries(_args: &[Value]) -> Value {
    // TODO: Implement Object.entries
    Value::Undefined
}

fn builtin_object_assign(_args: &[Value]) -> Value {
    // TODO: Implement Object.assign
    Value::Undefined
}

fn builtin_object_freeze(_args: &[Value]) -> Value {
    // TODO: Implement Object.freeze
    Value::Undefined
}

fn builtin_object_seal(_args: &[Value]) -> Value {
    // TODO: Implement Object.seal
    Value::Undefined
}

// ============================================================================
// Array Methods
// ============================================================================

fn builtin_array_is_array(_args: &[Value]) -> Value {
    // TODO: Implement Array.isArray
    Value::Boolean(false)
}

fn builtin_array_from(_args: &[Value]) -> Value {
    // TODO: Implement Array.from
    Value::Undefined
}

fn builtin_array_of(_args: &[Value]) -> Value {
    // TODO: Implement Array.of
    Value::Undefined
}

// ============================================================================
// String Methods
// ============================================================================

fn builtin_string_from_char_code(args: &[Value]) -> Value {
    let mut result = String::new();
    for arg in args {
        if let Value::Number(n) = arg {
            if let Some(ch) = char::from_u32(*n as u32) {
                result.push(ch);
            }
        }
    }
    Value::String(result)
}

// ============================================================================
// Number Methods
// ============================================================================

fn builtin_number_is_nan(args: &[Value]) -> Value {
    if let Some(Value::Number(n)) = args.first() {
        Value::Boolean(n.is_nan())
    } else {
        Value::Boolean(false)
    }
}

fn builtin_number_is_finite(args: &[Value]) -> Value {
    if let Some(Value::Number(n)) = args.first() {
        Value::Boolean(n.is_finite())
    } else {
        Value::Boolean(false)
    }
}

fn builtin_number_parse_int(args: &[Value]) -> Value {
    builtin_parse_int(args)
}

fn builtin_number_parse_float(args: &[Value]) -> Value {
    builtin_parse_float(args)
}

// ============================================================================
// JSON Methods
// ============================================================================

fn builtin_json_parse(args: &[Value]) -> Value {
    if let Some(Value::String(s)) = args.first() {
        // TODO: Implement proper JSON parsing
        // For now, just return undefined
        let _ = s;
        Value::Undefined
    } else {
        Value::Undefined
    }
}

fn builtin_json_stringify(args: &[Value]) -> Value {
    if let Some(val) = args.first() {
        // Simple stringification
        Value::String(val.to_string())
    } else {
        Value::Undefined
    }
}

// ============================================================================
// Global Functions
// ============================================================================

fn builtin_parse_int(args: &[Value]) -> Value {
    if let Some(val) = args.first() {
        let s = val.to_string();
        if let Ok(n) = s.trim().parse::<i64>() {
            return Value::Number(n as f64);
        }
    }
    Value::Number(f64::NAN)
}

fn builtin_parse_float(args: &[Value]) -> Value {
    if let Some(val) = args.first() {
        let s = val.to_string();
        if let Ok(n) = s.trim().parse::<f64>() {
            return Value::Number(n);
        }
    }
    Value::Number(f64::NAN)
}

fn builtin_is_nan(args: &[Value]) -> Value {
    if let Some(Value::Number(n)) = args.first() {
        Value::Boolean(n.is_nan())
    } else {
        Value::Boolean(true)
    }
}

fn builtin_is_finite(args: &[Value]) -> Value {
    if let Some(Value::Number(n)) = args.first() {
        Value::Boolean(n.is_finite())
    } else {
        Value::Boolean(false)
    }
}
