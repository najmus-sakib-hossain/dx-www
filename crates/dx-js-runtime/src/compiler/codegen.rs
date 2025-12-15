//! Cranelift code generation

use crate::compiler::mir::{FunctionId, PrimitiveType, TypedMIR};
use crate::compiler::mir::Type as MirType;
use crate::compiler::OptLevel;
use crate::error::{DxError, DxResult};
use cranelift::prelude::*;
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{FuncId, Linkage, Module};
use std::collections::HashMap;

/// A compiled module ready for execution
pub struct CompiledModule {
    /// The JIT module (must be kept alive)
    _jit_module: JITModule,
    /// Function pointers by ID
    functions: HashMap<FunctionId, *const u8>,
    /// Entry point
    entry_point: Option<*const u8>,
    /// Source hash for caching
    pub source_hash: [u8; 32],
}

// Safety: We control access to the function pointers
unsafe impl Send for CompiledModule {}
unsafe impl Sync for CompiledModule {}

impl CompiledModule {
    /// Execute the module's entry point
    pub unsafe fn execute(&self) -> i64 {
        if let Some(entry) = self.entry_point {
            let func: fn() -> i64 = std::mem::transmute(entry);
            func()
        } else {
            0
        }
    }

    /// Get a function pointer by ID
    #[allow(dead_code)]
    pub fn get_function(&self, id: FunctionId) -> Option<*const u8> {
        self.functions.get(&id).copied()
    }
}

/// Code generator using Cranelift
pub struct CodeGenerator {
    #[allow(dead_code)]
    opt_level: OptLevel,
}

impl CodeGenerator {
    pub fn new(opt_level: OptLevel) -> DxResult<Self> {
        Ok(Self { opt_level })
    }

    /// Generate native code from MIR
    pub fn generate(&mut self, mir: &TypedMIR) -> DxResult<CompiledModule> {
        // Create JIT module
        let builder = JITBuilder::new(cranelift_module::default_libcall_names())
            .map_err(|e| DxError::CompileError(e.to_string()))?;

        let mut jit_module = JITModule::new(builder);
        let mut ctx = jit_module.make_context();
        let mut func_ctx = FunctionBuilderContext::new();

        let mut func_ids: HashMap<FunctionId, FuncId> = HashMap::new();
        let mut signatures: HashMap<FunctionId, Signature> = HashMap::new();

        // First pass: declare all functions
        for func in &mir.functions {
            let mut sig = jit_module.make_signature();

            // Add return type
            match &func.return_type {
                MirType::Primitive(PrimitiveType::I64) => {
                    sig.returns.push(AbiParam::new(types::I64));
                }
                MirType::Primitive(PrimitiveType::F64) => {
                    sig.returns.push(AbiParam::new(types::F64));
                }
                MirType::Primitive(PrimitiveType::I32) => {
                    sig.returns.push(AbiParam::new(types::I32));
                }
                _ => {
                    // Default to I64 for other types (boxed)
                    sig.returns.push(AbiParam::new(types::I64));
                }
            }

            // Add parameters
            for param in &func.params {
                let ty = self.type_to_cranelift(&param.ty);
                sig.params.push(AbiParam::new(ty));
            }

            let func_id = jit_module
                .declare_function(&func.name, Linkage::Local, &sig)
                .map_err(|e| DxError::CompileError(e.to_string()))?;

            func_ids.insert(func.id, func_id);
            signatures.insert(func.id, sig);
        }

        // Second pass: define all functions
        for func in &mir.functions {
            let func_id = func_ids[&func.id];
            let sig = &signatures[&func.id];

            ctx.func.signature = sig.clone();

            let mut builder = FunctionBuilder::new(&mut ctx.func, &mut func_ctx);

            // Create entry block
            let entry_block = builder.create_block();
            builder.append_block_params_for_function_params(entry_block);
            builder.switch_to_block(entry_block);
            builder.seal_block(entry_block);

            // For now, just return 0
            let zero = builder.ins().iconst(types::I64, 0);
            builder.ins().return_(&[zero]);

            builder.finalize();

            jit_module
                .define_function(func_id, &mut ctx)
                .map_err(|e| DxError::CompileError(e.to_string()))?;

            jit_module.clear_context(&mut ctx);
        }

        // Finalize
        jit_module
            .finalize_definitions()
            .map_err(|e| DxError::CompileError(e.to_string()))?;

        // Collect function pointers
        let mut functions = HashMap::new();
        for (mir_id, cl_id) in &func_ids {
            let ptr = jit_module.get_finalized_function(*cl_id);
            functions.insert(*mir_id, ptr);
        }

        let entry_point = mir.entry_point.and_then(|id| functions.get(&id).copied());

        Ok(CompiledModule {
            _jit_module: jit_module,
            functions,
            entry_point,
            source_hash: [0; 32],
        })
    }

    fn type_to_cranelift(&self, ty: &MirType) -> types::Type {
        match ty {
            MirType::Primitive(PrimitiveType::I32) => types::I32,
            MirType::Primitive(PrimitiveType::I64) => types::I64,
            MirType::Primitive(PrimitiveType::F64) => types::F64,
            MirType::Primitive(PrimitiveType::Bool) => types::I8,
            _ => types::I64, // Default to pointer-sized
        }
    }
}
