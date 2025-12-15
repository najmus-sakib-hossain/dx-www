//! Compiler module - OXC parser + Cranelift JIT

pub mod ast_lowering;
pub mod codegen;
pub mod expressions;
pub mod mir;
pub mod optimize;
pub mod parser;
pub mod statements;
pub mod type_solver;

use crate::error::DxResult;

pub use codegen::CompiledModule;
pub use mir::{Type, TypeId, TypedMIR};

/// Compiler configuration
#[derive(Clone, Debug)]
pub struct CompilerConfig {
    /// Enable TypeScript type checking
    pub type_check: bool,
    /// Optimization level
    pub optimization_level: OptLevel,
}

impl Default for CompilerConfig {
    fn default() -> Self {
        Self {
            type_check: true,
            optimization_level: OptLevel::Basic,
        }
    }
}

/// Optimization level
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OptLevel {
    /// No optimizations
    None,
    /// Basic optimizations (fast compile)
    Basic,
    /// Aggressive optimizations (slower compile, faster code)
    Aggressive,
}

/// The main compiler
pub struct Compiler {
    config: CompilerConfig,
    codegen: codegen::CodeGenerator,
}

impl Compiler {
    /// Create a new compiler
    pub fn new(config: CompilerConfig) -> DxResult<Self> {
        Ok(Self {
            config: config.clone(),
            codegen: codegen::CodeGenerator::new(config.optimization_level)?,
        })
    }

    /// Compile source code to native machine code
    pub fn compile(&mut self, source: &str, filename: &str) -> DxResult<CompiledModule> {
        // Phase 1: Parse with OXC
        let ast = parser::parse(source, filename)?;

        // Phase 2: Lower AST to Typed MIR (includes type solving)
        let mir = ast_lowering::lower_ast_to_mir(source, &ast)?;

        // Phase 3: Optimizations
        let optimized_mir = match self.config.optimization_level {
            OptLevel::None => mir,
            OptLevel::Basic => optimize::basic_optimize(mir),
            OptLevel::Aggressive => {
                let mir = optimize::basic_optimize(mir);
                let mir = optimize::inline_small_functions(mir);
                optimize::dead_code_elimination(mir)
            }
        };

        // Phase 4: Cranelift codegen
        let compiled = self.codegen.generate(&optimized_mir)?;

        Ok(compiled)
    }
}
