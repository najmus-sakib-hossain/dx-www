//! OXC parser integration

use crate::error::{DxError, DxResult};
use oxc_allocator::Allocator;
use oxc_parser::Parser;
use oxc_span::SourceType;

/// Parsed AST wrapper
pub struct ParsedAST {
    /// Source code (owned copy)
    pub source: String,
    /// Filename
    pub filename: String,
    /// Whether this is TypeScript
    pub is_typescript: bool,
    /// Number of statements parsed
    pub statement_count: usize,
}

/// Parse JavaScript/TypeScript source code using OXC
pub fn parse(source: &str, filename: &str) -> DxResult<ParsedAST> {
    // Determine source type from filename
    let source_type = SourceType::from_path(filename).unwrap_or_default();
    let is_typescript = filename.ends_with(".ts") || filename.ends_with(".tsx");

    // Create allocator for this parse session
    let allocator = Allocator::default();

    // Parse with OXC
    let parser_result = Parser::new(&allocator, source, source_type).parse();

    // Check for parse errors
    if !parser_result.errors.is_empty() {
        let error_messages: Vec<String> = parser_result.errors.iter().map(|e| e.to_string()).collect();
        return Err(DxError::ParseError(error_messages.join("\n")));
    }

    // Get statement count for basic info
    let statement_count = parser_result.program.body.len();

    Ok(ParsedAST {
        source: source.to_string(),
        filename: filename.to_string(),
        is_typescript,
        statement_count,
    })
}

/// Get basic information about parsed code (for debugging)
pub fn get_ast_info(ast: &ParsedAST) -> AstInfo {
    AstInfo {
        filename: ast.filename.clone(),
        is_typescript: ast.is_typescript,
        source_len: ast.source.len(),
        statement_count: ast.statement_count,
    }
}

#[derive(Debug)]
pub struct AstInfo {
    pub filename: String,
    pub is_typescript: bool,
    pub source_len: usize,
    pub statement_count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_js() {
        let source = "const x = 1 + 2;";
        let ast = parse(source, "test.js").unwrap();
        assert_eq!(ast.statement_count, 1);
        assert!(!ast.is_typescript);
    }

    #[test]
    fn test_parse_typescript() {
        let source = "const x: number = 42;";
        let ast = parse(source, "test.ts").unwrap();
        assert!(ast.is_typescript);
    }

    #[test]
    fn test_parse_error() {
        let source = "const x = {";
        let result = parse(source, "test.js");
        assert!(result.is_err());
    }
}
