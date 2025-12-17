//! DX Bundle Minify - SIMD Minification

use std::collections::HashMap;

/// Minify JavaScript source code
pub fn minify(source: &[u8]) -> Vec<u8> {
    let mut output = Vec::with_capacity(source.len());
    let len = source.len();
    let mut i = 0;
    let mut in_string = false;
    let mut string_char = 0u8;
    let mut last_was_space = false;

    while i < len {
        let byte = source[i];

        // Handle strings - preserve them exactly
        if in_string {
            output.push(byte);
            if byte == string_char && (i == 0 || source[i - 1] != b'\\') {
                in_string = false;
            }
            i += 1;
            continue;
        }

        // Start of string
        if byte == b'"' || byte == b'\'' || byte == b'`' {
            in_string = true;
            string_char = byte;
            output.push(byte);
            last_was_space = false;
            i += 1;
            continue;
        }

        // Skip comments
        if byte == b'/' && i + 1 < len {
            if source[i + 1] == b'/' {
                // Line comment - skip to end of line
                i += 2;
                while i < len && source[i] != b'\n' {
                    i += 1;
                }
                continue;
            } else if source[i + 1] == b'*' {
                // Block comment - skip to */
                i += 2;
                while i + 1 < len {
                    if source[i] == b'*' && source[i + 1] == b'/' {
                        i += 2;
                        break;
                    }
                    i += 1;
                }
                continue;
            }
        }

        // Compress whitespace
        if byte == b' ' || byte == b'\t' || byte == b'\n' || byte == b'\r' {
            if !last_was_space && !output.is_empty() {
                // Only add space if needed (between identifiers/keywords)
                let prev = *output.last().unwrap();
                if prev.is_ascii_alphanumeric() || prev == b'_' || prev == b'$' {
                    output.push(b' ');
                    last_was_space = true;
                }
            }
        } else {
            output.push(byte);
            last_was_space = false;
        }

        i += 1;
    }

    output
}

/// Mangle variable names
pub fn mangle_identifiers(source: &[u8], reserved: &[&str]) -> Vec<u8> {
    let source_str = String::from_utf8_lossy(source);
    let mut output = String::with_capacity(source.len());
    let mut mapping = HashMap::new();
    let mut counter = 0;
    let reserved_set: std::collections::HashSet<_> = reserved.iter().copied().collect();

    let mut chars = source_str.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch.is_alphabetic() || ch == '_' || ch == '$' {
            // Collect full identifier
            let mut ident = String::new();
            ident.push(ch);

            while let Some(&next_ch) = chars.peek() {
                if next_ch.is_alphanumeric() || next_ch == '_' || next_ch == '$' {
                    ident.push(chars.next().unwrap());
                } else {
                    break;
                }
            }

            // Mangle if not reserved and long enough
            if ident.len() > 3 && !reserved_set.contains(ident.as_str()) {
                let mangled = mapping.entry(ident.clone()).or_insert_with(|| {
                    let name = generate_short_name(counter);
                    counter += 1;
                    name
                });
                output.push_str(mangled);
            } else {
                output.push_str(&ident);
            }
        } else {
            output.push(ch);
        }
    }

    output.into_bytes()
}

fn generate_short_name(n: usize) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minify() {
        let input = b"function   test ( )  {  return   42 ;  }";
        let output = minify(input);
        let result = String::from_utf8_lossy(&output);
        assert!(result.len() < input.len());
    }
}
