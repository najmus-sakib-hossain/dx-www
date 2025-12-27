//! Built-in functions (console, JSON, etc.)

use crate::value::Value;

/// Console.log implementation
pub fn console_log(args: &[Value]) {
    for (i, arg) in args.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", arg);
    }
    println!();
}

/// JSON.parse implementation (simplified)
pub fn json_parse(s: &str) -> Result<Value, String> {
    let s = s.trim();

    if s == "null" {
        return Ok(Value::Null);
    }
    if s == "true" {
        return Ok(Value::Boolean(true));
    }
    if s == "false" {
        return Ok(Value::Boolean(false));
    }
    if let Ok(n) = s.parse::<f64>() {
        return Ok(Value::Number(n));
    }
    if s.starts_with('"') && s.ends_with('"') {
        return Ok(Value::String(s[1..s.len() - 1].to_string()));
    }

    Err("Unsupported JSON".to_string())
}

/// JSON.stringify implementation
pub fn json_stringify(value: &Value) -> String {
    match value {
        Value::Null => "null".to_string(),
        Value::Undefined => "undefined".to_string(),
        Value::Boolean(b) => b.to_string(),
        Value::Number(n) => n.to_string(),
        Value::String(s) => format!("\"{}\"", s),
        Value::Object(_) => "[object Object]".to_string(),
        Value::Array(_) => "[]".to_string(),
        Value::Function(_) => "[function]".to_string(),
    }
}
