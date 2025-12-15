//! Complete AST to MIR lowering
//!
//! This module walks the OXC AST and generates Typed MIR that can be
//! compiled to native code by Cranelift.

use crate::compiler::mir::*;
use crate::compiler::parser::ParsedAST;
use crate::error::{DxError, DxResult};
use oxc_allocator::Allocator;
use oxc_ast::ast::*;
use oxc_parser::Parser;
use oxc_span::SourceType;
use std::collections::HashMap;

/// AST to MIR lowering context
pub struct AstLowering<'a> {
    /// Source code
    source: &'a str,
    /// Current function being lowered
    current_function: Option<FunctionBuilder>,
    /// All lowered functions
    functions: Vec<TypedFunction>,
    /// Global variables
    globals: Vec<TypedGlobal>,
    /// Type layouts
    type_layouts: HashMap<TypeId, TypeLayout>,
    /// Next IDs
    next_function_id: u32,
    next_type_id: u32,
    /// Variable scopes
    scopes: Vec<HashMap<String, LocalId>>,
    /// String constants
    string_constants: Vec<String>,
}

/// Builder for a single function
struct FunctionBuilder {
    id: FunctionId,
    name: String,
    params: Vec<TypedParam>,
    return_type: Type,
    blocks: Vec<TypedBlock>,
    locals: Vec<TypedLocal>,
    current_block: BlockId,
    next_local_id: u32,
    next_block_id: u32,
}

impl FunctionBuilder {
    fn new(id: FunctionId, name: String) -> Self {
        Self {
            id,
            name,
            params: Vec::new(),
            return_type: Type::Primitive(PrimitiveType::F64),
            blocks: vec![TypedBlock {
                id: BlockId(0),
                instructions: Vec::new(),
                terminator: Terminator::Return(None),
            }],
            locals: Vec::new(),
            current_block: BlockId(0),
            next_local_id: 0,
            next_block_id: 1,
        }
    }

    fn add_local(&mut self, name: String, ty: Type) -> LocalId {
        let id = LocalId(self.next_local_id);
        self.next_local_id += 1;
        self.locals.push(TypedLocal {
            name,
            ty,
            index: id.0,
        });
        id
    }

    #[allow(dead_code)]
    fn add_param(&mut self, name: String, ty: Type) -> LocalId {
        let id = LocalId(self.next_local_id);
        self.next_local_id += 1;
        self.params.push(TypedParam {
            name: name.clone(),
            ty: ty.clone(),
            index: id.0,
        });
        self.locals.push(TypedLocal {
            name,
            ty,
            index: id.0,
        });
        id
    }

    fn emit(&mut self, inst: TypedInstruction) {
        if let Some(block) = self.blocks.iter_mut().find(|b| b.id == self.current_block) {
            block.instructions.push(inst);
        }
    }

    #[allow(dead_code)]
    fn new_block(&mut self) -> BlockId {
        let id = BlockId(self.next_block_id);
        self.next_block_id += 1;
        self.blocks.push(TypedBlock {
            id,
            instructions: Vec::new(),
            terminator: Terminator::Unreachable,
        });
        id
    }

    fn set_terminator(&mut self, term: Terminator) {
        if let Some(block) = self.blocks.iter_mut().find(|b| b.id == self.current_block) {
            block.terminator = term;
        }
    }

    #[allow(dead_code)]
    fn switch_to_block(&mut self, id: BlockId) {
        self.current_block = id;
    }

    fn build(self) -> TypedFunction {
        TypedFunction {
            id: self.id,
            name: self.name,
            params: self.params,
            return_type: self.return_type,
            blocks: self.blocks,
            locals: self.locals,
            is_pure: false,
        }
    }
}

impl<'a> AstLowering<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            current_function: None,
            functions: Vec::new(),
            globals: Vec::new(),
            type_layouts: HashMap::new(),
            next_function_id: 0,
            next_type_id: 0,
            scopes: vec![HashMap::new()],
            string_constants: Vec::new(),
        }
    }

    /// Lower the entire program
    pub fn lower(&mut self, _ast: &ParsedAST) -> DxResult<TypedMIR> {
        // Create main function
        let main_id = FunctionId(self.next_function_id);
        self.next_function_id += 1;
        self.current_function = Some(FunctionBuilder::new(main_id, "__dx_main__".to_string()));

        // Parse and lower the AST using OXC
        self.lower_source()?;

        // Finalize main function
        if let Some(builder) = self.current_function.take() {
            self.functions.push(builder.build());
        }

        Ok(TypedMIR {
            functions: std::mem::take(&mut self.functions),
            globals: std::mem::take(&mut self.globals),
            entry_point: Some(main_id),
            type_layouts: std::mem::take(&mut self.type_layouts),
        })
    }

    fn lower_source(&mut self) -> DxResult<()> {
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

        // Lower each statement
        let mut last_result: Option<LocalId> = None;

        for stmt in &program.body {
            last_result = self.lower_statement(stmt)?;
        }

        // Set up return with the last expression result
        let func = self.current_function.as_mut().unwrap();
        if let Some(result) = last_result {
            func.set_terminator(Terminator::Return(Some(result)));
        } else {
            // Return undefined (NaN)
            let undef = func.add_local("__undef__".to_string(), Type::Primitive(PrimitiveType::F64));
            func.emit(TypedInstruction::Const {
                dest: undef,
                value: Constant::F64(f64::NAN),
            });
            func.set_terminator(Terminator::Return(Some(undef)));
        }

        Ok(())
    }

    fn lower_statement(&mut self, stmt: &Statement<'_>) -> DxResult<Option<LocalId>> {
        match stmt {
            Statement::ExpressionStatement(expr_stmt) => {
                let result = self.lower_expression(&expr_stmt.expression)?;
                Ok(Some(result))
            }

            Statement::VariableDeclaration(var_decl) => {
                for decl in &var_decl.declarations {
                    if let Some(init) = &decl.init {
                        let value = self.lower_expression(init)?;

                        // Get variable name
                        if let BindingPatternKind::BindingIdentifier(ident) = &decl.id.kind {
                            let name = ident.name.to_string();
                            self.define_variable(name, value);
                        }
                    }
                }
                Ok(None)
            }

            Statement::ReturnStatement(ret) => {
                if let Some(arg) = &ret.argument {
                    let value = self.lower_expression(arg)?;
                    let func = self.current_function.as_mut().unwrap();
                    func.set_terminator(Terminator::Return(Some(value)));
                } else {
                    let func = self.current_function.as_mut().unwrap();
                    func.set_terminator(Terminator::Return(None));
                }
                Ok(None)
            }

            Statement::BlockStatement(block) => {
                self.push_scope();
                let mut last = None;
                for stmt in &block.body {
                    last = self.lower_statement(stmt)?;
                }
                self.pop_scope();
                Ok(last)
            }

            // For unsupported statements, just continue
            _ => Ok(None),
        }
    }

    fn lower_expression(&mut self, expr: &Expression<'_>) -> DxResult<LocalId> {
        match expr {
            Expression::NumericLiteral(lit) => {
                let func = self.current_function.as_mut().unwrap();
                let dest = func.add_local("".to_string(), Type::Primitive(PrimitiveType::F64));
                func.emit(TypedInstruction::Const {
                    dest,
                    value: Constant::F64(lit.value),
                });
                Ok(dest)
            }

            Expression::BooleanLiteral(lit) => {
                let func = self.current_function.as_mut().unwrap();
                let dest = func.add_local("".to_string(), Type::Primitive(PrimitiveType::F64));
                // Convert bool to f64 (1.0 or 0.0)
                func.emit(TypedInstruction::Const {
                    dest,
                    value: Constant::F64(if lit.value { 1.0 } else { 0.0 }),
                });
                Ok(dest)
            }

            Expression::StringLiteral(lit) => {
                let func = self.current_function.as_mut().unwrap();
                let dest = func.add_local("".to_string(), Type::Primitive(PrimitiveType::String));
                func.emit(TypedInstruction::Const {
                    dest,
                    value: Constant::String(lit.value.to_string()),
                });
                Ok(dest)
            }

            Expression::NullLiteral(_) => {
                let func = self.current_function.as_mut().unwrap();
                let dest = func.add_local("".to_string(), Type::Primitive(PrimitiveType::Null));
                func.emit(TypedInstruction::Const {
                    dest,
                    value: Constant::Null,
                });
                Ok(dest)
            }

            Expression::BinaryExpression(bin) => {
                let left = self.lower_expression(&bin.left)?;
                let right = self.lower_expression(&bin.right)?;

                let op = match bin.operator {
                    BinaryOperator::Addition => BinOpKind::Add,
                    BinaryOperator::Subtraction => BinOpKind::Sub,
                    BinaryOperator::Multiplication => BinOpKind::Mul,
                    BinaryOperator::Division => BinOpKind::Div,
                    BinaryOperator::Remainder => BinOpKind::Mod,
                    BinaryOperator::LessThan => BinOpKind::Lt,
                    BinaryOperator::LessEqualThan => BinOpKind::Le,
                    BinaryOperator::GreaterThan => BinOpKind::Gt,
                    BinaryOperator::GreaterEqualThan => BinOpKind::Ge,
                    BinaryOperator::Equality | BinaryOperator::StrictEquality => BinOpKind::Eq,
                    BinaryOperator::Inequality | BinaryOperator::StrictInequality => BinOpKind::Ne,
                    _ => {
                        return Err(DxError::CompileError(format!(
                            "Unsupported operator: {:?}",
                            bin.operator
                        )))
                    }
                };

                let func = self.current_function.as_mut().unwrap();
                let dest = func.add_local("".to_string(), Type::Primitive(PrimitiveType::F64));
                func.emit(TypedInstruction::BinOp {
                    dest,
                    op,
                    left,
                    right,
                    op_type: PrimitiveType::F64,
                });

                Ok(dest)
            }

            Expression::UnaryExpression(unary) => {
                let arg = self.lower_expression(&unary.argument)?;
                let func = self.current_function.as_mut().unwrap();

                match unary.operator {
                    UnaryOperator::UnaryNegation => {
                        // Negate: 0 - arg
                        let zero =
                            func.add_local("".to_string(), Type::Primitive(PrimitiveType::F64));
                        func.emit(TypedInstruction::Const {
                            dest: zero,
                            value: Constant::F64(0.0),
                        });
                        let dest =
                            func.add_local("".to_string(), Type::Primitive(PrimitiveType::F64));
                        func.emit(TypedInstruction::BinOp {
                            dest,
                            op: BinOpKind::Sub,
                            left: zero,
                            right: arg,
                            op_type: PrimitiveType::F64,
                        });
                        Ok(dest)
                    }
                    UnaryOperator::UnaryPlus => {
                        // Just return the value
                        Ok(arg)
                    }
                    UnaryOperator::LogicalNot => {
                        // !x => x == 0
                        let zero =
                            func.add_local("".to_string(), Type::Primitive(PrimitiveType::F64));
                        func.emit(TypedInstruction::Const {
                            dest: zero,
                            value: Constant::F64(0.0),
                        });
                        let dest =
                            func.add_local("".to_string(), Type::Primitive(PrimitiveType::F64));
                        func.emit(TypedInstruction::BinOp {
                            dest,
                            op: BinOpKind::Eq,
                            left: arg,
                            right: zero,
                            op_type: PrimitiveType::F64,
                        });
                        Ok(dest)
                    }
                    _ => Err(DxError::CompileError(format!(
                        "Unsupported unary operator: {:?}",
                        unary.operator
                    ))),
                }
            }

            Expression::Identifier(ident) => {
                let name = ident.name.as_str();
                for scope in self.scopes.iter().rev() {
                    if let Some(&local_id) = scope.get(name) {
                        return Ok(local_id);
                    }
                }
                Err(DxError::CompileError(format!(
                    "Undefined variable: {}",
                    name
                )))
            }

            Expression::ParenthesizedExpression(paren) => self.lower_expression(&paren.expression),

            Expression::CallExpression(call) => self.lower_call_expression(call),

            Expression::ConditionalExpression(cond) => {
                // Ternary: test ? consequent : alternate
                // For now, we'll just evaluate test and return consequent
                // Full implementation would need phi nodes
                let _test = self.lower_expression(&cond.test)?;
                let consequent = self.lower_expression(&cond.consequent)?;
                let _alternate = self.lower_expression(&cond.alternate)?;

                // TODO: Implement proper branching with phi nodes
                Ok(consequent)
            }

            // For unsupported expressions, return NaN (undefined)
            _ => {
                let func = self.current_function.as_mut().unwrap();
                let dest = func.add_local("".to_string(), Type::Primitive(PrimitiveType::F64));
                func.emit(TypedInstruction::Const {
                    dest,
                    value: Constant::F64(f64::NAN),
                });
                Ok(dest)
            }
        }
    }

    fn lower_call_expression(&mut self, call: &CallExpression<'_>) -> DxResult<LocalId> {
        // Get function name
        let func_name = match &call.callee {
            Expression::Identifier(ident) => ident.name.to_string(),
            Expression::StaticMemberExpression(member) => {
                if let Expression::Identifier(obj) = &member.object {
                    format!("{}.{}", obj.name, member.property.name)
                } else {
                    return Err(DxError::CompileError(
                        "Complex member expressions not supported".into(),
                    ));
                }
            }
            _ => {
                return Err(DxError::CompileError(
                    "Complex call expressions not supported".into(),
                ))
            }
        };

        // Lower arguments
        let mut args = Vec::new();
        for arg in &call.arguments {
            match arg {
                Argument::SpreadElement(_) => {
                    return Err(DxError::CompileError("Spread arguments not supported".into()));
                }
                _ => {
                    let arg_expr = arg.to_expression();
                    args.push(self.lower_expression(arg_expr)?);
                }
            }
        }

        let func = self.current_function.as_mut().unwrap();
        let dest = func.add_local("".to_string(), Type::Primitive(PrimitiveType::F64));

        // Handle built-in functions with magic IDs
        let function_id = match func_name.as_str() {
            "console.log" => FunctionId(u32::MAX - 1),
            "console.warn" => FunctionId(u32::MAX - 2),
            "console.error" => FunctionId(u32::MAX - 3),
            "Math.floor" => FunctionId(u32::MAX - 10),
            "Math.ceil" => FunctionId(u32::MAX - 11),
            "Math.sqrt" => FunctionId(u32::MAX - 12),
            "Math.abs" => FunctionId(u32::MAX - 13),
            "Math.sin" => FunctionId(u32::MAX - 14),
            "Math.cos" => FunctionId(u32::MAX - 15),
            "Math.random" => FunctionId(u32::MAX - 16),
            _ => FunctionId(0), // User function (not implemented yet)
        };

        func.emit(TypedInstruction::Call {
            dest: Some(dest),
            function: function_id,
            args,
        });

        Ok(dest)
    }

    fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    fn define_variable(&mut self, name: String, local_id: LocalId) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name, local_id);
        }
    }
}

/// Lower parsed AST to MIR
pub fn lower_ast_to_mir(source: &str, ast: &ParsedAST) -> DxResult<TypedMIR> {
    let mut lowering = AstLowering::new(source);
    lowering.lower(ast)
}
