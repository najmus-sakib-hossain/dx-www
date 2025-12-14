/// TOON to DX ULTRA converter
///
/// Converts TOON format to ultra-optimized DX SINGULARITY format.
use crate::optimizer::{format_array, optimize_key};

/// Convert TOON string to DX ULTRA format
pub fn toon_to_dx(toon_str: &str) -> Result<String, String> {
    let mut output = String::new();
    let lines: Vec<&str> = toon_str.lines().collect();

    let mut i = 0;
    while i < lines.len() {
        let line = lines[i].trim();

        if line.is_empty() {
            i += 1;
            continue;
        }

        // Detect if it's a simple key-value
        if let Some(space_pos) = line.find(' ') {
            let key = &line[..space_pos];
            let value = line[space_pos + 1..].trim_matches('"');

            // Check if next lines are indented (nested object)
            if i + 1 < lines.len() && lines[i + 1].starts_with("  ") {
                // It's a parent key, process nested
                let key_opt = optimize_key(key);

                // Collect nested items
                let mut nested_props = Vec::new();
                let mut j = i + 1;

                while j < lines.len() && lines[j].starts_with("  ") {
                    let nested_line = lines[j].trim();
                    if let Some(nested_space) = nested_line.find(' ') {
                        let nested_key = &nested_line[..nested_space];
                        let nested_val = nested_line[nested_space + 1..].trim_matches('"');
                        nested_props.push((optimize_key(nested_key), nested_val.to_string()));
                    }
                    j += 1;
                }

                // Output as inline or multi-line
                if nested_props.len() <= 4 {
                    output.push_str(&key_opt);
                    output.push('.');
                    for (idx, (k, v)) in nested_props.iter().enumerate() {
                        if idx > 0 {
                            output.push('^');
                        }
                        output.push_str(k);
                        output.push(':');
                        output.push_str(v);
                    }
                    output.push('\n');
                } else {
                    for (k, v) in nested_props {
                        output.push_str(&key_opt);
                        output.push('.');
                        output.push_str(&k);
                        output.push(':');
                        output.push_str(&v);
                        output.push('\n');
                    }
                }

                i = j;
                continue;
            } else {
                // Simple key-value
                let key_opt = optimize_key(key);
                output.push_str(&key_opt);
                output.push(':');
                output.push_str(value);
                output.push('\n');
            }
        }

        i += 1;
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toon_to_dx() {
        let toon = r#"
name "test"
version "1.0.0"
"#;
        let dx = toon_to_dx(toon).unwrap();
        assert!(dx.contains("n:test"));
    }
}
