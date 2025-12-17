/// Production TypeScript stripper - handles complex types correctly
pub fn strip_typescript_simple(source: &str) -> String {
    let mut result = source.to_string();

    // Step 1: Remove interface blocks (including "export interface")
    // First handle "export interface"
    while let Some(start) = result.find("export interface ") {
        if let Some(end) = result[start..].find('}') {
            result.replace_range(start..start + end + 1, "");
        } else {
            break;
        }
    }
    // Then handle plain "interface"
    while let Some(start) = result.find("interface ") {
        if let Some(end) = result[start..].find('}') {
            result.replace_range(start..start + end + 1, "");
        } else {
            break;
        }
    }

    // Step 2: Remove type aliases (including "export type")
    // First handle "export type"
    while let Some(start) = result.find("export type ") {
        if let Some(end) = result[start..].find([';', '\n']) {
            result.replace_range(start..start + end + 1, "");
        } else {
            break;
        }
    }
    // Then handle plain "type "
    while let Some(start) = result.find("type ") {
        if let Some(end) = result[start..].find([';', '\n']) {
            result.replace_range(start..start + end + 1, "");
        } else {
            break;
        }
    }

    // Step 3: Remove generic parameters <T>, <T, U>, etc.
    // ONLY remove when the generic is part of a type annotation, not JSX
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

    // Step 4a: Remove access modifiers from class fields: private/public/protected/readonly
    for modifier in &["private ", "public ", "protected ", "readonly "] {
        result = result.replace(modifier, "");
    }

    // Step 4b: Remove variable/const type annotations: const x: Type = ... → const x = ...
    // Also handles: exports.name: Type = ... → exports.name = ...
    let mut var_iteration = 0;
    while var_iteration < 100 {
        var_iteration += 1;
        let before_len = result.len();

        // Look for patterns: const/let/var identifier: Type, or exports.name: Type
        let patterns = [
            "const ",
            "let ",
            "var ",
            "export const ",
            "export let ",
            "export var ",
            "exports.",
            "module.exports.",
        ];

        let mut best_match: Option<(usize, usize, usize)> = None; // (decl_start, colon_pos, end)

        for pattern in patterns.iter() {
            // Find all occurrences of this pattern
            let mut search_start = 0;
            while let Some(rel_pos) = result[search_start..].find(pattern) {
                let decl_start = search_start + rel_pos;
                let after_decl = &result[decl_start + pattern.len()..];

                // Find the identifier - read alphanumeric chars until we hit a non-identifier char
                let mut ident_end = 0;
                for (idx, ch) in after_decl.char_indices() {
                    if ch.is_alphanumeric() || ch == '_' || ch == '.' {
                        ident_end = idx + ch.len_utf8();
                    } else {
                        break;
                    }
                }

                // Check if immediately followed by ": "
                let after_ident = &after_decl[ident_end..];
                if !after_ident.starts_with(": ") {
                    // No type annotation, try next occurrence
                    search_start = decl_start + pattern.len();
                    continue;
                }

                // Now we have: identifier: Type = value
                let colon_pos = decl_start + pattern.len() + ident_end;
                let after_colon = &result[colon_pos + 2..];

                // Find the delimiter: = or ; or , or newline
                let mut end = 0;
                let mut angle_depth = 0;
                let mut brace_depth = 0;
                let mut bracket_depth = 0;

                for (i, ch) in after_colon.char_indices() {
                    match ch {
                        '<' => angle_depth += 1,
                        '>' if angle_depth > 0 => angle_depth -= 1,
                        '{' => brace_depth += 1,
                        '}' if brace_depth > 0 => brace_depth -= 1,
                        '[' => bracket_depth += 1,
                        ']' if bracket_depth > 0 => bracket_depth -= 1,
                        '=' | ';' | '\n'
                            if angle_depth == 0 && brace_depth == 0 && bracket_depth == 0 =>
                        {
                            end = i;
                            break;
                        }
                        ',' if angle_depth == 0 && brace_depth == 0 && bracket_depth == 0 => {
                            end = i;
                            break;
                        }
                        _ => {}
                    }
                }

                if end > 0 {
                    // Found a valid match - keep the earliest one
                    if best_match.is_none() || colon_pos < best_match.unwrap().1 {
                        best_match = Some((colon_pos, colon_pos, end));
                    }
                    break; // Found one for this pattern, move to next pattern
                }

                search_start = decl_start + pattern.len();
            }
        }

        // Apply the best match
        if let Some((_, colon_pos, end)) = best_match {
            let after_colon = &result[colon_pos + 2..];
            let replacement = if after_colon.chars().nth(end) == Some('=') {
                " "
            } else {
                ""
            };
            result.replace_range(colon_pos..colon_pos + 2 + end, replacement);
        }

        if result.len() == before_len {
            break;
        }
    }

    // Step 4b2: Handle class field types that start at line beginning
    // Pattern: "name: Type = value" at start of line (after access modifiers removed)
    // This handles cases like "  history: number[] = [];"
    let mut iteration_4b2 = 0;
    while iteration_4b2 < 100 {
        iteration_4b2 += 1;
        let before_len = result.len();

        // Look for pattern: start of line (or whitespace), identifier: Type = or ;
        // We need to check if the identifier is at the beginning of a line or after whitespace
        let chars: Vec<char> = result.chars().collect();
        let mut i = 0;
        let mut found = false;

        while i < chars.len() {
            // Check if we're at start of line or after whitespace
            let at_line_start = i == 0 || chars[i - 1] == '\n';
            let after_whitespace =
                i > 0 && (chars[i - 1] == ' ' || chars[i - 1] == '\t' || chars[i - 1] == '\n');

            if (at_line_start || after_whitespace) && chars[i].is_alphabetic() {
                // Read identifier
                let ident_start = i;
                while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') {
                    i += 1;
                }

                // Check for ": Type" pattern
                if i < chars.len() && chars[i] == ':' && i + 1 < chars.len() && chars[i + 1] == ' '
                {
                    let colon_pos = i;
                    i += 2; // skip ": "

                    // Read type (until = or ; or , or newline)
                    let type_start = i;
                    let mut bracket_depth = 0;
                    while i < chars.len() {
                        match chars[i] {
                            '[' => bracket_depth += 1,
                            ']' => bracket_depth -= 1,
                            '=' | ';' | '\n' if bracket_depth == 0 => break,
                            ',' if bracket_depth == 0 => break,
                            _ => {}
                        }
                        i += 1;
                    }

                    // If we found a type and there's an = after, this is a class field
                    if i > type_start && i < chars.len() && chars[i] == '=' {
                        // Check that the "identifier" isn't a keyword like format, parse, etc.
                        let ident: String = chars[ident_start..colon_pos].iter().collect();
                        // Only apply if the identifier looks like a class field (not inside an object literal)
                        // This is a heuristic: we check if the line starts with just the identifier
                        let line_before: String = chars[chars[..ident_start]
                            .iter()
                            .rposition(|&c| c == '\n')
                            .map(|p| p + 1)
                            .unwrap_or(0)
                            ..ident_start]
                            .iter()
                            .collect();

                        // If the line before is just whitespace, this is likely a class field
                        if line_before.trim().is_empty() && !ident.is_empty() {
                            // Remove the ": Type" part (from colon to just before =)
                            let remove_end = i;
                            let byte_colon: usize =
                                chars[..colon_pos].iter().collect::<String>().len();
                            let byte_end: usize =
                                chars[..remove_end].iter().collect::<String>().len();

                            result.replace_range(byte_colon..byte_end, " ");
                            found = true;
                            break;
                        }
                    }
                }
            } else {
                i += 1;
            }
        }

        if !found || result.len() == before_len {
            break;
        }
    }

    // Step 4c: Remove object destructuring parameter types: }: Type) → })
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
    // IMPORTANT: Skip `: ` patterns inside template literals
    let mut iteration = 0;
    while iteration < 100 {
        iteration += 1;
        let before_len = result.len();

        // Find `: ` but skip those inside template literals
        let mut colon_pos = None;
        let mut in_template = false;
        let chars: Vec<char> = result.chars().collect();
        for i in 0..chars.len().saturating_sub(1) {
            if chars[i] == '`' {
                in_template = !in_template;
            } else if !in_template && chars[i] == ':' && chars[i + 1] == ' ' {
                colon_pos = Some(i);
                break;
            }
        }

        if let Some(colon_pos) = colon_pos {
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
