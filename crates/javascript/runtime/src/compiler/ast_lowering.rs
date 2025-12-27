//! Complete AST to MIR lowering
//!
//! This module walks the OXC AST and generates Typed MIR that can be
//! compiled to native code by Cranelift.

use crate::compiler::mir::*;
use crate::compiler::parser::ParsedAST;
use crate::compiler::statements::StatementLowerer;
use crate::error::{DxError, DxResult};
use oxc_allocator::Allocator;
use oxc_parser::Parser;
use oxc_span::SourceType;
use std::collections::HashMap;

/// AST to MIR lowering context
pub struct AstLowering<'a> {
    /// Source code
    source: &'a str,
    /// All lowered functions
    functions: Vec<TypedFunction>,
    /// Global variables
    globals: Vec<TypedGlobal>,
    /// Type layouts
    type_layouts: HashMap<TypeId, TypeLayout>,
    /// Next IDs
    next_function_id: u32,
    next_type_id: u32,
    /// String constants
    string_constants: Vec<String>,
}

impl<'a> AstLowering<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            functions: Vec::new(),
            globals: Vec::new(),
            type_layouts: HashMap::new(),
            next_function_id: 0,
            next_type_id: 0,
            string_constants: Vec::new(),
        }
    }

    /// Lower the entire program
    pub fn lower(&mut self, _ast: &ParsedAST) -> DxResult<TypedMIR> {
        // Create main function
        let main_id = FunctionId(self.next_function_id);
        self.next_function_id += 1;
        let builder = FunctionBuilder::new(main_id, "__dx_main__".to_string());

        // Parse and lower the AST using OXC
        let builder = self.lower_source(builder)?;

        // Finalize main function
        self.functions.push(builder.build());

        Ok(TypedMIR {
            functions: std::mem::take(&mut self.functions),
            globals: std::mem::take(&mut self.globals),
            entry_point: Some(main_id),
            type_layouts: std::mem::take(&mut self.type_layouts),
        })
    }

    fn lower_source(&mut self, builder: FunctionBuilder) -> DxResult<FunctionBuilder> {
        // Parse source with OXC
        let allocator = Allocator::default();
        let source_type = SourceType::default();
        let parser_result = Parser::new(&allocator, self.source, source_type).parse();

        if !parser_result.errors.is_empty() {
            return Err(DxError::ParseError(
                parser_result
                    .errors
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<_>>()
                    .join("\n"),
            ));
        }

        let program = parser_result.program;

        // Create statement lowerer
        let mut lowerer = StatementLowerer::new(builder);

        // Lower each statement
        for stmt in &program.body {
            lowerer.lower_statement(stmt)?;
        }

        // Return the finished builder
        Ok(lowerer.finish())
    }
}

/// Lower parsed AST to MIR
pub fn lower_ast_to_mir(source: &str, ast: &ParsedAST) -> DxResult<TypedMIR> {
    let mut lowering = AstLowering::new(source);
    lowering.lower(ast)
}
