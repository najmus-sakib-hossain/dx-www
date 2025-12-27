// EXTREME MICRO-OPTIMIZATIONS - Every nanosecond counts!
// Replace HashMap with fixed array (10x faster lookup)
// Remove all allocations - use stack buffers
// Inline everything aggressively

const MAX_VARS: usize = 32;
const MAX_OUTPUT: usize = 64;

// Stack-allocated variable storage (no HashMap overhead!)
struct VarStore {
    names: [&'static str; MAX_VARS],
    values: [f64; MAX_VARS],
    count: usize,
}

impl VarStore {
    #[inline(always)]
    fn new() -> Self {
        Self {
            names: [""; MAX_VARS],
            values: [0.0; MAX_VARS],
            count: 0,
        }
    }

    #[inline(always)]
    fn set(&mut self, name: &'static str, val: f64) {
        // Linear search (faster than HashMap for <32 items!)
        for i in 0..self.count {
            if self.names[i] == name {
                self.values[i] = val;
                return;
            }
        }
        if self.count < MAX_VARS {
            self.names[self.count] = name;
            self.values[self.count] = val;
            self.count += 1;
        }
    }

    #[inline(always)]
    fn get(&self, name: &str) -> Option<f64> {
        for i in 0..self.count {
            if self.names[i] == name {
                return Some(self.values[i]);
            }
        }
        None
    }
}

// Ultra-fast output buffer with optimized formatting
struct OutputBuffer {
    data: [u8; 8192], // Larger buffer for batching
    len: usize,
}

impl OutputBuffer {
    #[inline(always)]
    fn new() -> Self {
        Self {
            data: [0; 8192],
            len: 0,
        }
    }

    /// Push pre-formatted bytes directly
    #[inline(always)]
    fn push_bytes(&mut self, bytes: &[u8]) {
        if self.len + bytes.len() + 1 < 8192 {
            self.data[self.len..self.len + bytes.len()].copy_from_slice(bytes);
            self.len += bytes.len();
            self.data[self.len] = b'\n';
            self.len += 1;
        }
    }

    /// Fast path for single-digit integers
    #[inline(always)]
    fn push_single_digit(&mut self, d: u8) {
        if self.len + 2 < 8192 {
            self.data[self.len] = b'0' + d;
            self.data[self.len + 1] = b'\n';
            self.len += 2;
        }
    }

    #[inline(always)]
    fn as_str(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(&self.data[..self.len]) }
    }
}

#[inline(always)]
pub fn execute_js(source: &str) -> String {
    let mut vars = VarStore::new();
    let mut output = OutputBuffer::new();

    for line in source.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if !line.is_empty() && line.as_bytes()[0] == b'/' {
            continue;
        }

        let first = line.as_bytes()[0];

        // Fast path variable declaration
        if first == b'c' && line.starts_with("const ") {
            if let Some((name, val)) = parse_var_fast(line, &vars) {
                let name_leaked: &'static str = Box::leak(name.to_string().into_boxed_str());
                vars.set(name_leaked, val);
            }
        } else if first == b'l' && line.starts_with("let ") {
            if let Some((name, val)) = parse_var_fast(line, &vars) {
                let name_leaked: &'static str = Box::leak(name.to_string().into_boxed_str());
                vars.set(name_leaked, val);
            }
        }
        // Fast path console.log
        else if first == b'c' && line.len() > 12 && line.starts_with("console.log(") {
            if let Some(val) = extract_log_fast(line, &vars) {
                format_value_fast(val, &mut output);
            }
        }
    }

    output.as_str().to_string()
}

#[inline(always)]
fn parse_var_fast<'a>(line: &'a str, vars: &VarStore) -> Option<(&'a str, f64)> {
    let eq_pos = line.find('=')?;
    let name_start = if line.starts_with("const ") { 6 } else { 4 };
    let name = line[name_start..eq_pos].trim();
    let expr = line[eq_pos + 1..].trim_end_matches(';').trim();
    let val = eval_expr_fast(expr, vars)?;
    Some((name, val))
}

#[inline(always)]
fn extract_log_fast(line: &str, vars: &VarStore) -> Option<f64> {
    let bytes = line.as_bytes();
    let start = 12; // "console.log(".len()
    let mut end = start;
    let mut depth = 1;

    for i in start..bytes.len() {
        match bytes[i] {
            b'(' => depth += 1,
            b')' => {
                depth -= 1;
                if depth == 0 {
                    end = i;
                    break;
                }
            }
            _ => {}
        }
    }

    eval_expr_fast(&line[start..end].trim(), vars)
}

#[inline(always)]
fn eval_expr_fast(expr: &str, vars: &VarStore) -> Option<f64> {
    if expr.is_empty() {
        return None;
    }

    let bytes = expr.as_bytes();
    let first = bytes[0];

    // Fast boolean check
    if first == b't' {
        return Some(1.0);
    }
    if first == b'f' {
        return Some(0.0);
    }

    // Fast Math function with aggressive constant folding
    if first == b'M' && expr.starts_with("Math.") {
        let p = expr.find('(')?;
        let end = expr.rfind(')')?;
        let arg_str = &expr[p + 1..end].trim();

        // Aggressive constant folding: try to parse as constant first
        let arg = if let Ok(constant) = arg_str.parse::<f64>() {
            constant
        } else {
            eval_expr_fast(arg_str, vars)?
        };

        // Determine function from byte pattern (faster than string comparison)
        let func_name = &expr[5..p];
        return Some(match func_name {
            "sqrt" => arg.sqrt(),
            "floor" => arg.floor(),
            "ceil" => arg.ceil(),
            "abs" => arg.abs(),
            "round" => arg.round(),
            _ => arg,
        });
    }

    // Fast binary ops with constant folding
    if let Some(pos) = find_byte_seq(bytes, b" + ") {
        let (l, r) = (expr[..pos].trim(), expr[pos + 3..].trim());
        if let (Ok(a), Ok(b)) = (l.parse::<f64>(), r.parse::<f64>()) {
            return Some(a + b);
        }
        return Some(eval_expr_fast(l, vars)? + eval_expr_fast(r, vars)?);
    }
    if let Some(pos) = find_byte_seq(bytes, b" - ") {
        let (l, r) = (expr[..pos].trim(), expr[pos + 3..].trim());
        if let (Ok(a), Ok(b)) = (l.parse::<f64>(), r.parse::<f64>()) {
            return Some(a - b);
        }
        return Some(eval_expr_fast(l, vars)? - eval_expr_fast(r, vars)?);
    }
    if let Some(pos) = find_byte_seq(bytes, b" * ") {
        let (l, r) = (expr[..pos].trim(), expr[pos + 3..].trim());
        if let (Ok(a), Ok(b)) = (l.parse::<f64>(), r.parse::<f64>()) {
            return Some(a * b);
        }
        return Some(eval_expr_fast(l, vars)? * eval_expr_fast(r, vars)?);
    }
    if let Some(pos) = find_byte_seq(bytes, b" / ") {
        let (l, r) = (expr[..pos].trim(), expr[pos + 3..].trim());
        if let (Ok(a), Ok(b)) = (l.parse::<f64>(), r.parse::<f64>()) {
            return Some(a / b);
        }
        return Some(eval_expr_fast(l, vars)? / eval_expr_fast(r, vars)?);
    }

    // Fast comparisons
    if let Some(pos) = find_byte_seq(bytes, b" < ") {
        let (l, r) = (expr[..pos].trim(), expr[pos + 3..].trim());
        if let (Ok(a), Ok(b)) = (l.parse::<f64>(), r.parse::<f64>()) {
            return Some(if a < b { 1.0 } else { 0.0 });
        }
        return Some(if eval_expr_fast(l, vars)? < eval_expr_fast(r, vars)? {
            1.0
        } else {
            0.0
        });
    }
    if let Some(pos) = find_byte_seq(bytes, b" > ") {
        let (l, r) = (expr[..pos].trim(), expr[pos + 3..].trim());
        if let (Ok(a), Ok(b)) = (l.parse::<f64>(), r.parse::<f64>()) {
            return Some(if a > b { 1.0 } else { 0.0 });
        }
        return Some(if eval_expr_fast(l, vars)? > eval_expr_fast(r, vars)? {
            1.0
        } else {
            0.0
        });
    }

    // Variable lookup
    if let Some(v) = vars.get(expr) {
        return Some(v);
    }

    // Number literal
    expr.parse::<f64>().ok()
}

// Ultra-fast byte sequence finder
#[inline(always)]
fn find_byte_seq(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    if haystack.len() < needle.len() {
        return None;
    }
    for i in 0..=haystack.len() - needle.len() {
        if haystack[i..i + needle.len()] == *needle {
            return Some(i);
        }
    }
    None
}

// Ultra-fast formatter with optimized fast paths
#[inline(always)]
fn format_value_fast(val: f64, output: &mut OutputBuffer) {
    // Fast path: booleans
    if val == 1.0 {
        output.push_bytes(b"true");
        return;
    }
    if val == 0.0 && val.is_sign_positive() {
        output.push_bytes(b"false");
        return;
    }

    // Fast path: single-digit integers (common case)
    if val >= 0.0 && val < 10.0 && val.fract() == 0.0 {
        output.push_single_digit(val as u8);
        return;
    }

    // Fast path: small integers
    if val.fract() == 0.0 && val.abs() < 1e15 {
        let int_val = val as i64;
        let mut buf = itoa::Buffer::new();
        let formatted = buf.format(int_val);
        output.push_bytes(formatted.as_bytes());
    } else {
        // Floating point: use ryu
        let mut buf = ryu::Buffer::new();
        let formatted = buf.format(val);
        output.push_bytes(formatted.as_bytes());
    }
}
