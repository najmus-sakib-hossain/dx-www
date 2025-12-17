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

/// Builder for a single function
pub struct FunctionBuilder {
    pub id: FunctionId,
    pub name: String,
    pub params: Vec<TypedParam>,
    pub return_type: Type,
    pub blocks: Vec<TypedBlock>,
    pub locals: Vec<TypedLocal>,
    pub current_block: BlockId,
    next_local_id: u32,
    next_block_id: u32,
}

impl FunctionBuilder {
    pub fn new(id: FunctionId, name: String) -> Self {
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

    pub fn add_local(&mut self, name: String, ty: Type) -> LocalId {
        let id = LocalId(self.next_local_id);
        self.next_local_id += 1;
        self.locals.push(TypedLocal {
            name,
            ty,
            index: id.0,
        });
        id
    }

    pub fn add_param(&mut self, name: String, ty: Type) -> LocalId {
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

    pub fn emit(&mut self, inst: TypedInstruction) {
        if let Some(block) = self.blocks.iter_mut().find(|b| b.id == self.current_block) {
            block.instructions.push(inst);
        }
    }

    pub fn new_block(&mut self) -> BlockId {
        let id = BlockId(self.next_block_id);
        self.next_block_id += 1;
        self.blocks.push(TypedBlock {
            id,
            instructions: Vec::new(),
            terminator: Terminator::Unreachable,
        });
        id
    }

    pub fn set_terminator(&mut self, term: Terminator) {
        if let Some(block) = self.blocks.iter_mut().find(|b| b.id == self.current_block) {
            block.terminator = term;
        }
    }

    pub fn switch_to_block(&mut self, id: BlockId) {
        self.current_block = id;
    }

    pub fn build(self) -> TypedFunction {
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
