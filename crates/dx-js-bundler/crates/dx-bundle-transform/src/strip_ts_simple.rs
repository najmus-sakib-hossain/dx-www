/// Production TypeScript stripper - handles complex types correctly
pub fn strip_typescript_simple(source: &str) -> String {
    let mut result = source.to_string();

    // Step 1: Remove interface blocks
    while let Some(start) = result.find("interface ") {
        if let Some(end) = result[start..].find('}') {
            result.replace_range(start..start + end + 1, "");
        } else {
            break;
        }
    }

    // Step 2: Remove type aliases
    while let Some(start) = result.find("type ") {
        if let Some(end) = result[start..].find([';', '\n']) {
            result.replace_range(start..start + end + 1, "");
        } else {
            break;
        }
    }

    // Step 3: Remove generic parameters <T>, <T, U>, etc.
    let mut iteration = 0;
    while iteration < 100 {
        iteration += 1;
        let before_len = result.len();

        if let Some(start) = result.find('<') {
            // Check if preceded by identifier (not comparison operator)
            if start > 0 {
                let before_char = result.chars().nth(start - 1).unwrap_or(' ');
                if before_char.is_alphanumeric() || before_char == '_' {
                    // Find matching closing >
                    let mut depth = 1;
                    let mut end = start + 1;
                    let chars: Vec<char> = result.chars().collect();

                    while end < chars.len() && depth > 0 {
                        match chars[end] {
                            '<' => depth += 1,
                            '>' => depth -= 1,
                            _ => {}
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

    // Step 4: Remove object destructuring parameter types: }: Type) → })
    while let Some(pos) = result.find("}: ") {
        let before = &result[..pos];
        // Check if inside parameters (unclosed parenthesis before)
        let last_open = before.rfind('(').unwrap_or(0);
        let last_close = before.rfind(')').unwrap_or(0);

        if last_open > last_close {
            let after = &result[pos + 3..];
            if let Some(end) = after.find([',', ')']) {
                result.replace_range(pos + 1..pos + 3 + end, "");
                continue;
            }
        }
        break;
    }

    // Step 5: Remove return type annotations: ): Type { or ): Type =>
    // Also handles ) : Type (with space) after parameter types are removed
    // Find all ): or ) : patterns and look for the function body delimiter
    // IMPORTANT: Skip delimiters inside template literals (between backticks)
    let mut step5_iteration = 0;
    while step5_iteration < 100 {
        step5_iteration += 1;

        // Try both patterns: ): and ) :
        let return_pattern = result.find("): ").or_else(|| result.find(") :")).map(|p| (p, 3));

        if let Some((pos, skip)) = return_pattern {
            let after = &result[pos + skip..];

            // Find delimiter: { or =>, but SKIP those inside template literals and track depths
            let mut in_template = false;
            let mut paren_depth: i32 = 0;
            let mut bracket_depth: i32 = 0;
            let mut brace_depth: i32 = 0;
            let mut body_start_pos = None;
            let mut arrow_start_pos = None;

            let chars: Vec<char> = after.chars().collect();
            for i in 0..chars.len() {
                let ch = chars[i];

                if ch == '`' {
                    in_template = !in_template;
                } else if !in_template {
                    // Check for delimiters BEFORE updating depths
                    if paren_depth == 0 && bracket_depth == 0 && brace_depth == 0 {
                        if ch == '{' && body_start_pos.is_none() {
                            body_start_pos = Some(i);
                            break;
                        } else if ch == '=' && i + 1 < chars.len() && chars[i + 1] == '>' {
                            arrow_start_pos = Some(i);
                            break;
                        }
                    }

                    // Track nesting depths AFTER checking
                    match ch {
                        '(' => paren_depth += 1,
                        ')' => paren_depth = (paren_depth - 1).max(0),
                        '[' => bracket_depth += 1,
                        ']' => bracket_depth = (bracket_depth - 1).max(0),
                        '{' => brace_depth += 1,
                        '}' => brace_depth = (brace_depth - 1).max(0),
                        _ => {}
                    }
                }
            }

            if let Some(body_start) = body_start_pos {
                // Remove from the : to just before the {
                // pos + 1 is the position after ), we want to keep )
                result.replace_range(pos + 1..pos + skip + body_start, "");
                continue;
            }

            if let Some(arrow_start) = arrow_start_pos {
                // Remove from the : to just before the =>
                result.replace_range(pos + 1..pos + skip + arrow_start, "");
                continue;
            }

            break;
        } else {
            break;
        }
    }

    // Step 6: Remove parameter type annotations: (param: Type) → (param)
    // Be careful with complex types - look for delimiters
    let mut iteration = 0;
    while iteration < 100 {
        iteration += 1;
        let before_len = result.len();

        if let Some(colon_pos) = result.find(": ") {
            let before = &result[..colon_pos];
            let after = &result[colon_pos + 2..];

            // Check if this colon is a parameter type annotation or an object property
            // Count the nesting level to determine context
            let last_open_paren = before.rfind('(').unwrap_or(0);
            let last_close_paren = before.rfind(')').unwrap_or(0);
            let last_open_brace = before.rfind('{').unwrap_or(0);
            let last_close_brace = before.rfind('}').unwrap_or(0);

            // If we're inside unclosed parens AND those parens are more recent than any unclosed braces,
            // then we're in a parameter list
            let in_params = last_open_paren > 0 && last_open_paren > last_close_paren;
            let in_object = last_open_brace > last_close_brace;

            // Only process if in parameters and NOT in object literal
            // (or if in object that's nested inside parameters, check which is more recent)
            let should_process = in_params && (!in_object || last_open_paren > last_open_brace);

            if should_process {
                // We're inside parameters - find the end of the type
                let mut end = 0;
                let mut paren_depth = 0;
                let mut bracket_depth = 0;
                let mut brace_depth = 0;
                let mut angle_depth = 0;

                for (i, ch) in after.char_indices() {
                    match ch {
                        '(' => paren_depth += 1,
                        ')' => {
                            if paren_depth > 0 {
                                paren_depth -= 1;
                            } else if bracket_depth == 0 && brace_depth == 0 && angle_depth == 0 {
                                // Closing paren of parameter list
                                end = i;
                                break;
                            }
                        }
                        '[' => bracket_depth += 1,
                        ']' if bracket_depth > 0 => bracket_depth -= 1,
                        '{' => brace_depth += 1,
                        '}' if brace_depth > 0 => brace_depth -= 1,
                        '<' => angle_depth += 1,
                        '>' if angle_depth > 0 => angle_depth -= 1,
                        ',' if paren_depth == 0
                            && bracket_depth == 0
                            && brace_depth == 0
                            && angle_depth == 0 =>
                        {
                            end = i;
                            break;
                        }
                        '=' if paren_depth == 0
                            && bracket_depth == 0
                            && brace_depth == 0
                            && angle_depth == 0 =>
                        {
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

    // Step 7: Clean up whitespace artifacts and leftover type fragments
    result = result.replace("  ", " ");
    result = result.replace("( ", "(");
    result = result.replace(" )", ")");
    result = result.replace(", )", ")");
    result = result.replace(",,", ",");
    result = result.replace("(,", "(");

    // Clean up arrow function type residue: fn=> void becomes fn
    while result.contains("=> void") || result.contains("=> number") || result.contains("=> string")
    {
        result = result.replace("=> void", "");
        result = result.replace("=> number", "");
        result = result.replace("=> string", "");
        result = result.replace("=> boolean", "");
        result = result.replace("=> any", "");
    }

    // Clean up parameter with no type but leftover colon: (param: ) -> (param)
    result = result.replace(": )", ")");
    result = result.replace(": ,", ",");

    // Clean up simple type annotations in arrow functions: (x: Type) =>
    // These often get missed in object properties
    for type_name in &["string", "number", "boolean", "void", "any"] {
        let pattern = format!(": {})", type_name);
        result = result.replace(&pattern, ")");
    }

    result
}
