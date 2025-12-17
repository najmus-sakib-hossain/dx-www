//! DX Bundle Transform - Code Transformations

use std::collections::HashMap;

mod strip_ts_simple;
pub use strip_ts_simple::strip_typescript_simple;

/// Transform JSX to JavaScript - removes JSX while preserving TypeScript generics
pub fn transform_jsx(source: &str) -> String {
    let mut result = String::with_capacity(source.len());
    let chars: Vec<char> = source.chars().collect();
    let mut i = 0;
    let mut jsx_depth: i32 = 0;

    while i < chars.len() {
        if chars[i] == '<' {
            // Check if this is JSX (not comparison, not TypeScript generic)
            if i + 1 < chars.len() {
                let next = chars[i + 1];
                let prev = if i > 0 { chars[i - 1] } else { ' ' };

                // TypeScript generic: <T>, <T, U> etc. after identifier
                // Also covers: Record<string, T>, Array<T>, Map<K, V>, etc.
                let is_generic = (prev.is_alphanumeric() || prev == '_')
                    && (next.is_uppercase() || next == '_' || next.is_lowercase());

                // Additional check: if we're in a type annotation context (after : and before =)
                // then treat any < as a generic, not JSX
                let in_type_context = {
                    // Look backwards for : and make sure no = between : and current position
                    let before = &result;
                    let last_colon = before.rfind(':');
                    let last_eq = before.rfind('=');
                    let last_brace = before.rfind('{');
                    match (last_colon, last_eq, last_brace) {
                        (Some(colon), Some(eq), _) if colon > eq => true,
                        (Some(colon), None, Some(brace)) if colon > brace => true,
                        (Some(_), None, None) => true,
                        _ => false,
                    }
                };

                if is_generic || in_type_context {
                    // Keep the generic - just copy it
                    result.push(chars[i]);
                    i += 1;
                    let mut depth = 1;
                    while i < chars.len() && depth > 0 {
                        if chars[i] == '<' {
                            depth += 1;
                        } else if chars[i] == '>' {
                            depth -= 1;
                        }
                        result.push(chars[i]);
                        i += 1;
                    }
                    continue;
                }

                // Debug: ALWAYS print when we see < that's treated as JSX
                if source.contains("Record") {
                    eprintln!(
                        "JSX < at {}: prev='{}', next='{}', is_generic={}, in_type_context={}",
                        i, prev, next, is_generic, in_type_context
                    );
                }

                // JSX closing tag
                if next == '/' && i + 2 < chars.len() && chars[i + 2].is_alphabetic() {
                    // Skip until >
                    while i < chars.len() && chars[i] != '>' {
                        i += 1;
                    }
                    if i < chars.len() {
                        i += 1; // Skip >
                    }
                    jsx_depth = jsx_depth.saturating_sub(1);
                    continue;
                }
                // JSX opening tag
                else if next.is_alphabetic() {
                    // Check if self-closing
                    let mut j = i + 1;
                    let mut is_self_closing = false;
                    while j < chars.len() && chars[j] != '>' {
                        if chars[j] == '/' && j + 1 < chars.len() && chars[j + 1] == '>' {
                            is_self_closing = true;
                        }
                        j += 1;
                    }

                    // Skip tag
                    i = j + 1;

                    if !is_self_closing {
                        jsx_depth += 1;
                    }
                    continue;
                }
            }
        }

        // Skip everything inside JSX
        if jsx_depth > 0 {
            i += 1;
            continue;
        }

        result.push(chars[i]);
        i += 1;
    }

    // Clean up return statements with empty JSX
    result = result.replace("return (  )", "return null");
    result = result.replace("return ( )", "return null");
    result = result.replace("return ()", "return null");
    result = result.replace("return (  ", "return null");

    result
}

/// Strip TypeScript type annotations and interfaces
pub fn strip_typescript(source: &str) -> String {
    let mut result = source.to_string();

    // Step 1: Remove interface declarations completely
    while let Some(start) = result.find("interface ") {
        let mut brace_count = 0;
        let mut found_opening = false;
        if let Some(opening) = result[start..].find('{') {
            found_opening = true;
            brace_count = 1;
            let search_start = start + opening + 1;

            for (i, ch) in result[search_start..].char_indices() {
                if ch == '{' {
                    brace_count += 1;
                } else if ch == '}' {
                    brace_count -= 1;
                    if brace_count == 0 {
                        result.replace_range(start..search_start + i + 1, "");
                        break;
                    }
                }
            }
        }

        if !found_opening || brace_count != 0 {
            break;
        }
    }

    // Step 2: Remove type aliases
    while let Some(start) = result.find("type ") {
        if let Some(semicolon) = result[start..].find(';') {
            result.replace_range(start..start + semicolon + 1, "");
        } else {
            break;
        }
    }

    // Step 3: Remove generic type parameters <T>, <T, U>
    loop {
        let before_len = result.len();

        if let Some(start) = result.find('<') {
            // Check if this looks like a generic (follows a word character)
            if start > 0 {
                let before = result.chars().nth(start - 1).unwrap_or(' ');
                if before.is_alphanumeric() || before == '_' {
                    // Find matching >
                    let mut depth = 1;
                    let mut end = start + 1;
                    let chars: Vec<char> = result.chars().collect();

                    while end < chars.len() && depth > 0 {
                        if chars[end] == '<' {
                            depth += 1;
                        } else if chars[end] == '>' {
                            depth -= 1;
                        }
                        end += 1;
                    }

                    if depth == 0 {
                        result.replace_range(start..end, "");
                        continue;
                    }
                }
            }
        }

        if result.len() == before_len {
            break;
        }
    }

    // Step 4: Remove return type annotations ): Type =>
    loop {
        let before_len = result.len();

        if let Some(colon_pos) = result.find("):") {
            let after = &result[colon_pos + 2..];

            // Find delimiter (=> or { or ;)
            if let Some(delim) = after.find("=>").or(after.find('{')).or(after.find(';')) {
                let keep = if after[delim..].starts_with("=>") {
                    "=>"
                } else if after[delim..].starts_with('{') {
                    "{"
                } else {
                    ";"
                };

                result.replace_range(colon_pos + 1..colon_pos + 2 + delim, keep);
                continue;
            }
        }

        if result.len() == before_len {
            break;
        }
    }

    // Step 5: Remove parameter type annotations
    loop {
        let before_len = result.len();

        // Look for pattern: (param: Type) or (param: Type,
        if let Some(colon_pos) = result.find(": ") {
            // Check if inside parameters (there's a ( before and ) or , after)
            let before = &result[..colon_pos];
            let after = &result[colon_pos + 2..];

            // Find the param name
            let param_start = before
                .rfind(|c: char| !c.is_alphanumeric() && c != '_')
                .map(|p| p + 1)
                .unwrap_or(0);
            let param = &before[param_start..];

            // Check if this is in a function parameter
            if param.chars().all(|c| c.is_alphanumeric() || c == '_') && !param.is_empty() {
                // Find end of type (look for , or ) or =>)
                let mut end = 0;
                let mut paren_depth = 0;
                let mut bracket_depth = 0;

                for (i, ch) in after.char_indices() {
                    match ch {
                        '(' => paren_depth += 1,
                        ')' if paren_depth > 0 => paren_depth -= 1,
                        ')' if paren_depth == 0 => {
                            end = i;
                            break;
                        }
                        '[' => bracket_depth += 1,
                        ']' if bracket_depth > 0 => bracket_depth -= 1,
                        ',' if paren_depth == 0 && bracket_depth == 0 => {
                            end = i;
                            break;
                        }
                        '=' if paren_depth == 0 && bracket_depth == 0 => {
                            end = i;
                            break;
                        }
                        _ => {}
                    }
                }

                if end > 0 {
                    result.replace_range(colon_pos..colon_pos + 2 + end, "");
                    continue;
                }
            }
        }

        if result.len() == before_len {
            break;
        }
    }

    // Step 5.5: Remove object destructuring type annotations }: Type)
    loop {
        let before_len = result.len();

        if let Some(brace_colon) = result.find("}: ") {
            // Find the next ) or ,
            let after = &result[brace_colon + 3..];
            let mut end = 0;

            for (i, ch) in after.char_indices() {
                if ch == ')' || ch == ',' {
                    end = i;
                    break;
                }
            }

            if end > 0 {
                // Replace }: Type with }
                result.replace_range(brace_colon + 1..brace_colon + 3 + end, "");
                continue;
            }
        }

        if result.len() == before_len {
            break;
        }
    }

    // Step 6: Clean up artifacts
    result = result.replace("  ", " ");
    result = result.replace("( ", "(");
    result = result.replace(" )", ")");
    result = result.replace(", )", ")");
    result = result.replace(",,", ",");
    result = result.replace("(,", "(");

    result
}

/// SIMD whitespace stripper (simplified version)
pub fn strip_whitespace(source: &[u8]) -> Vec<u8> {
    let mut output = Vec::with_capacity(source.len());
    let mut i = 0;
    let len = source.len();
    let mut in_string = false;
    let mut string_char = 0u8;
    let mut last_was_space = false;

    while i < len {
        let byte = source[i];

        // Handle strings
        if in_string {
            output.push(byte);
            if byte == string_char && (i == 0 || source[i - 1] != b'\\') {
                in_string = false;
            }
            i += 1;
            continue;
        }

        if byte == b'"' || byte == b'\'' || byte == b'`' {
            in_string = true;
            string_char = byte;
            output.push(byte);
            last_was_space = false;
            i += 1;
            continue;
        }

        // Strip excessive whitespace
        if byte == b' ' || byte == b'\t' || byte == b'\n' || byte == b'\r' {
            if !last_was_space {
                output.push(b' ');
                last_was_space = true;
            }
        } else {
            output.push(byte);
            last_was_space = false;
        }

        i += 1;
    }

    output
}

/// Simple identifier mangler
pub struct IdentifierMangler {
    mapping: HashMap<String, String>,
    counter: usize,
}

impl Default for IdentifierMangler {
    fn default() -> Self {
        Self::new()
    }
}

impl IdentifierMangler {
    pub fn new() -> Self {
        Self {
            mapping: HashMap::new(),
            counter: 0,
        }
    }

    pub fn mangle(&mut self, name: &str) -> String {
        // Don't mangle short names or reserved words
        if name.len() <= 2 || Self::is_reserved(name) {
            return name.to_string();
        }

        if let Some(mangled) = self.mapping.get(name) {
            return mangled.clone();
        }

        let mangled = Self::generate_name(self.counter);
        self.counter += 1;
        self.mapping.insert(name.to_string(), mangled.clone());
        mangled
    }

    fn generate_name(n: usize) -> String {
        const CHARS: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let mut result = String::new();
        let mut num = n;

        loop {
            result.push(CHARS[num % CHARS.len()] as char);
            num /= CHARS.len();
            if num == 0 {
                break;
            }
            num -= 1;
        }

        result
    }

    fn is_reserved(name: &str) -> bool {
        matches!(
            name,
            "if" | "else"
                | "for"
                | "while"
                | "do"
                | "switch"
                | "case"
                | "break"
                | "continue"
                | "return"
                | "function"
                | "var"
                | "let"
                | "const"
                | "class"
                | "extends"
                | "import"
                | "export"
                | "default"
                | "async"
                | "await"
                | "try"
                | "catch"
                | "finally"
                | "throw"
                | "new"
                | "this"
                | "super"
                | "typeof"
                | "instanceof"
                | "in"
                | "of"
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_typescript() {
        let input = "function foo(x: number): string { return String(x); }";
        let output = strip_typescript(input);
        assert!(output.contains("function foo(x)"));
    }

    #[test]
    fn test_identifier_mangler() {
        let mut mangler = IdentifierMangler::new();
        assert_eq!(mangler.mangle("longVariableName"), "a");
        assert_eq!(mangler.mangle("anotherLongName"), "b");
        assert_eq!(mangler.mangle("longVariableName"), "a"); // Same mapping
    }
}
