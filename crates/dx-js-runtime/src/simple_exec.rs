// Minimal working JS interpreter - no fancy types, just works!
use std::collections::HashMap;
use crate::simd::console::BatchConsole;

pub fn execute_js(source: &str) -> String {
    let mut vars: HashMap<String, f64> = HashMap::new();
    let mut output = Vec::new();
    
    for line in source.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with("//") { continue; }
        
        // Variable declaration: const x = 10;
        if line.starts_with("const ") || line.starts_with("let ") {
            if let Some((name, val)) = parse_var(line, &vars) {
                vars.insert(name, val);
            }
        }
        // console.log
        else if line.contains("console.log(") {
            if let Some(val) = extract_log_value(line, &vars) {
                output.push(format_value(val));
            }
        }
    }
    
    output.join("\n")
}

fn parse_var(line: &str, vars: &HashMap<String, f64>) -> Option<(String, f64)> {
    let parts: Vec<&str> = line.split('=').collect();
    if parts.len() < 2 { return None; }
    
    let name = parts[0].split_whitespace().nth(1)?.trim().to_string();
    let expr = parts[1].trim().trim_end_matches(';').trim();
    let val = eval_expr(expr, vars)?;
    
    Some((name, val))
}

fn extract_log_value(line: &str, vars: &HashMap<String, f64>) -> Option<f64> {
    let start = line.find("console.log(")? + 12;
    
    // Find matching closing paren by counting
    let mut depth = 1;
    let mut end = start;
    for (i, ch) in line[start..].chars().enumerate() {
        if ch == '(' { depth += 1; }
        else if ch == ')' {
            depth -= 1;
            if depth == 0 {
                end = start + i;
                break;
            }
        }
    }
    
    let arg = &line[start..end].trim();
    eval_expr(arg, vars)
}

fn eval_expr(expr: &str, vars: &HashMap<String, f64>) -> Option<f64> {
    let expr = expr.trim();
    
    // Boolean
    if expr == "true" { return Some(1.0); }
    if expr == "false" { return Some(0.0); }
    
    // Math functions
    if expr.starts_with("Math.") {
        let p = expr.find('(')?;
        let func = &expr[5..p];
        let end = expr.rfind(')')?;
        let arg_str = &expr[p+1..end];
        let arg = eval_expr(arg_str, vars)?;
        
        return Some(match func {
            "sqrt" => arg.sqrt(),
            "floor" => arg.floor(),
            "ceil" => arg.ceil(),
            "abs" => arg.abs(),
            "round" => arg.round(),
            _ => arg,
        });
    }
    
    // Binary ops
    if let Some(pos) = expr.find(" + ") {
        return Some(eval_expr(expr[..pos].trim(), vars)? + eval_expr(expr[pos+3..].trim(), vars)?);
    }
    if let Some(pos) = expr.find(" - ") {
        return Some(eval_expr(expr[..pos].trim(), vars)? - eval_expr(expr[pos+3..].trim(), vars)?);
    }
    if let Some(pos) = expr.find(" * ") {
        return Some(eval_expr(expr[..pos].trim(), vars)? * eval_expr(expr[pos+3..].trim(), vars)?);
    }
    if let Some(pos) = expr.find(" / ") {
        return Some(eval_expr(expr[..pos].trim(), vars)? / eval_expr(expr[pos+3..].trim(), vars)?);
    }
    
    // Comparison
    if let Some(pos) = expr.find(" < ") {
        return Some(if eval_expr(expr[..pos].trim(), vars)? < eval_expr(expr[pos+3..].trim(), vars)? { 1.0 } else { 0.0 });
    }
    if let Some(pos) = expr.find(" > ") {
        return Some(if eval_expr(expr[..pos].trim(), vars)? > eval_expr(expr[pos+3..].trim(), vars)? { 1.0 } else { 0.0 });
    }
    
    // Variable
    if let Some(&v) = vars.get(expr) {
        return Some(v);
    }
    
    // Number literal  
    expr.trim().parse::<f64>().ok()
}

fn format_value(val: f64) -> String {
    if val == 1.0 { "true".to_string() }
    else if val == 0.0 && val.is_sign_positive() { "false".to_string() }
    else if val.fract() == 0.0 && val.abs() < 1e15 {
        format!("{}", val as i64)
    } else {
        format!("{}", val)
    }
}
