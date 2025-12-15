//! Typed Middle Intermediate Representation

use crate::compiler::type_solver::TypedAST;
use crate::error::DxResult;
use std::collections::HashMap;

/// A complete typed MIR program
#[derive(Debug, Clone)]
pub struct TypedMIR {
    pub functions: Vec<TypedFunction>,
    pub globals: Vec<TypedGlobal>,
    pub entry_point: Option<FunctionId>,
    pub type_layouts: HashMap<TypeId, TypeLayout>,
}

/// A function with all types resolved
#[derive(Debug, Clone)]
pub struct TypedFunction {
    pub id: FunctionId,
    pub name: String,
    pub params: Vec<TypedParam>,
    pub return_type: Type,
    pub blocks: Vec<TypedBlock>,
    pub locals: Vec<TypedLocal>,
    pub is_pure: bool,
}

#[derive(Debug, Clone)]
pub struct TypedParam {
    pub name: String,
    pub ty: Type,
    pub index: u32,
}

#[derive(Debug, Clone)]
pub struct TypedLocal {
    pub name: String,
    pub ty: Type,
    pub index: u32,
}

/// A basic block with typed instructions
#[derive(Debug, Clone)]
pub struct TypedBlock {
    pub id: BlockId,
    pub instructions: Vec<TypedInstruction>,
    pub terminator: Terminator,
}

/// A typed instruction
#[derive(Debug, Clone)]
pub enum TypedInstruction {
    /// Constant value
    Const { dest: LocalId, value: Constant },

    /// Binary operation
    BinOp {
        dest: LocalId,
        op: BinOpKind,
        left: LocalId,
        right: LocalId,
        op_type: PrimitiveType,
    },

    /// Property access with known offset
    GetProperty {
        dest: LocalId,
        object: LocalId,
        offset: u32,
        prop_type: Type,
    },

    /// Property write
    SetProperty {
        object: LocalId,
        offset: u32,
        value: LocalId,
    },

    /// Function call
    Call {
        dest: Option<LocalId>,
        function: FunctionId,
        args: Vec<LocalId>,
    },

    /// Allocate object
    Allocate { dest: LocalId, layout: TypeId },

    /// Copy value
    Copy { dest: LocalId, src: LocalId },
}

#[derive(Debug, Clone)]
pub enum Terminator {
    Return(Option<LocalId>),
    Goto(BlockId),
    Branch {
        condition: LocalId,
        then_block: BlockId,
        else_block: BlockId,
    },
    Unreachable,
}

#[derive(Debug, Clone)]
pub enum Type {
    Primitive(PrimitiveType),
    Object(TypeId),
    Array(Box<Type>),
    Function(FunctionSignature),
    Any,
    Never,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrimitiveType {
    I32,
    I64,
    F64,
    Bool,
    String,
    Null,
    Undefined,
}

#[derive(Debug, Clone)]
pub struct TypeLayout {
    pub size: u32,
    pub alignment: u32,
    pub fields: Vec<FieldLayout>,
}

#[derive(Debug, Clone)]
pub struct FieldLayout {
    pub name: String,
    pub offset: u32,
    pub ty: Type,
}

#[derive(Debug, Clone)]
pub struct FunctionSignature {
    pub params: Vec<Type>,
    pub return_type: Box<Type>,
}

#[derive(Debug, Clone)]
pub enum Constant {
    I32(i32),
    I64(i64),
    F64(f64),
    Bool(bool),
    String(String),
    Null,
    Undefined,
}

#[derive(Debug, Clone, Copy)]
pub enum BinOpKind {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    And,
    Or,
}

// ID types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FunctionId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LocalId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TypeId(pub u32);

#[derive(Debug, Clone)]
pub struct TypedGlobal {
    pub name: String,
    pub ty: Type,
}

/// Lower typed AST to MIR
pub fn lower_to_mir(_typed_ast: &TypedAST) -> DxResult<TypedMIR> {
    // Create a simple entry point for now
    let entry_function = TypedFunction {
        id: FunctionId(0),
        name: "__dx_main__".to_string(),
        params: vec![],
        return_type: Type::Primitive(PrimitiveType::I64),
        blocks: vec![TypedBlock {
            id: BlockId(0),
            instructions: vec![],
            terminator: Terminator::Return(None),
        }],
        locals: vec![],
        is_pure: true,
    };

    Ok(TypedMIR {
        functions: vec![entry_function],
        globals: vec![],
        entry_point: Some(FunctionId(0)),
        type_layouts: HashMap::new(),
    })
}
