//! Cranelift code generation with built-in function support

use crate::compiler::mir::{
    BinOpKind, BlockId, Constant, FunctionId, LocalId, PrimitiveType, Terminator,
    Type, TypedFunction, TypedInstruction, TypedMIR,
};
use crate::compiler::OptLevel;
use crate::error::{DxError, DxResult};
use crate::value::Value;
use cranelift::prelude::*;
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{FuncId, Linkage, Module};
use cranelift_codegen::ir::FuncRef;
use std::collections::HashMap;

// Alias to avoid name collision with our mir::FunctionBuilder
use cranelift::prelude::FunctionBuilder as CraneliftFunctionBuilder;

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
    pub fn execute(&self) -> DxResult<Value> {
        if let Some(entry) = self.entry_point {
            // The entry function returns f64
            let func: extern "C" fn() -> f64 = unsafe { std::mem::transmute(entry) };
            let result = func();
            Ok(Value::Number(result))
        } else {
            Ok(Value::Undefined)
        }
    }

    /// Get a function pointer by ID
    #[allow(dead_code)]
    pub fn get_function(&self, id: FunctionId) -> Option<*const u8> {
        self.functions.get(&id).copied()
    }
}

// Built-in function implementations (extern "C" for FFI)
extern "C" fn builtin_console_log(value: f64) -> f64 {
    if value.is_nan() {
        println!("undefined");
    } else if value.is_infinite() {
        if value > 0.0 {
            println!("Infinity");
        } else {
            println!("-Infinity");
        }
    } else if value.fract() == 0.0 && value.abs() < 1e15 {
        println!("{}", value as i64);
    } else {
        println!("{}", value);
    }
    f64::NAN // return undefined
}

extern "C" fn builtin_math_floor(value: f64) -> f64 {
    value.floor()
}

extern "C" fn builtin_math_ceil(value: f64) -> f64 {
    value.ceil()
}

extern "C" fn builtin_math_sqrt(value: f64) -> f64 {
    value.sqrt()
}

extern "C" fn builtin_math_abs(value: f64) -> f64 {
    value.abs()
}

extern "C" fn builtin_math_sin(value: f64) -> f64 {
    value.sin()
}

extern "C" fn builtin_math_cos(value: f64) -> f64 {
    value.cos()
}

extern "C" fn builtin_math_random() -> f64 {
    // Simple random using system time
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    ((now as u64) % 1000000) as f64 / 1000000.0
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
        // Create JIT module with native target
        let mut builder = JITBuilder::new(cranelift_module::default_libcall_names())
            .map_err(|e| DxError::CompileError(e.to_string()))?;

        // Register built-in functions as symbols
        builder.symbol("__dx_console_log", builtin_console_log as *const u8);
        builder.symbol("__dx_math_floor", builtin_math_floor as *const u8);
        builder.symbol("__dx_math_ceil", builtin_math_ceil as *const u8);
        builder.symbol("__dx_math_sqrt", builtin_math_sqrt as *const u8);
        builder.symbol("__dx_math_abs", builtin_math_abs as *const u8);
        builder.symbol("__dx_math_sin", builtin_math_sin as *const u8);
        builder.symbol("__dx_math_cos", builtin_math_cos as *const u8);
        builder.symbol("__dx_math_random", builtin_math_random as *const u8);

        let mut jit_module = JITModule::new(builder);
        let mut ctx = jit_module.make_context();
        let mut func_ctx = FunctionBuilderContext::new();

        let mut func_ids: HashMap<FunctionId, FuncId> = HashMap::new();
        let mut builtin_func_ids: HashMap<u32, FuncId> = HashMap::new();
        let mut signatures: HashMap<FunctionId, Signature> = HashMap::new();

        // Declare built-in functions
        self.declare_builtins(&mut jit_module, &mut builtin_func_ids)?;

        // First pass: declare all user functions
        for func in &mir.functions {
            let mut sig = jit_module.make_signature();

            // Add return type (always f64 for JS values)
            sig.returns.push(AbiParam::new(types::F64));

            // Add parameters
            for _param in &func.params {
                sig.params.push(AbiParam::new(types::F64));
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
            
            // Pre-declare all function references before creating FunctionBuilder
            let mut func_refs: HashMap<u32, FuncRef> = HashMap::new();
            for (&magic_id, &builtin_id) in &builtin_func_ids {
                let func_ref = jit_module.declare_func_in_func(builtin_id, &mut ctx.func);
                func_refs.insert(magic_id, func_ref);
            }
            for (&mir_func_id, &cl_func_id) in &func_ids {
                let func_ref = jit_module.declare_func_in_func(cl_func_id, &mut ctx.func);
                func_refs.insert(mir_func_id.0, func_ref);
            }

            self.compile_function_body(
                &mut ctx,
                &mut func_ctx,
                func,
                &func_refs,
            )?;

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

    fn declare_builtins(
        &self,
        module: &mut JITModule,
        builtin_ids: &mut HashMap<u32, FuncId>,
    ) -> DxResult<()> {
        // Built-in functions with their magic IDs and argument counts
        let builtins: &[(&str, u32, usize)] = &[
            ("__dx_console_log", u32::MAX - 1, 1),
            ("__dx_console_log", u32::MAX - 2, 1), // console.warn
            ("__dx_console_log", u32::MAX - 3, 1), // console.error
            ("__dx_math_floor", u32::MAX - 10, 1),
            ("__dx_math_ceil", u32::MAX - 11, 1),
            ("__dx_math_sqrt", u32::MAX - 12, 1),
            ("__dx_math_abs", u32::MAX - 13, 1),
            ("__dx_math_sin", u32::MAX - 14, 1),
            ("__dx_math_cos", u32::MAX - 15, 1),
            ("__dx_math_random", u32::MAX - 16, 0),
        ];

        for (name, magic_id, arg_count) in builtins {
            let mut sig = module.make_signature();
            for _ in 0..*arg_count {
                sig.params.push(AbiParam::new(types::F64));
            }
            sig.returns.push(AbiParam::new(types::F64));

            let func_id = module
                .declare_function(name, Linkage::Import, &sig)
                .map_err(|e| DxError::CompileError(e.to_string()))?;

            builtin_ids.insert(*magic_id, func_id);
        }

        Ok(())
    }

    fn compile_function_body(
        &self,
        ctx: &mut cranelift::codegen::Context,
        func_ctx: &mut FunctionBuilderContext,
        func: &TypedFunction,
        func_refs: &HashMap<u32, FuncRef>,
    ) -> DxResult<()> {
        let mut builder = CraneliftFunctionBuilder::new(&mut ctx.func, func_ctx);

        // Create block map
        let mut block_map: HashMap<BlockId, Block> = HashMap::new();
        for block in &func.blocks {
            let cl_block = builder.create_block();
            block_map.insert(block.id, cl_block);
        }

        // Set up entry block
        let entry_block = block_map[&BlockId(0)];
        builder.append_block_params_for_function_params(entry_block);
        builder.switch_to_block(entry_block);
        builder.seal_block(entry_block);

        // Map locals to SSA values
        let mut locals: HashMap<LocalId, cranelift::prelude::Value> = HashMap::new();

        // Map parameters
        for (i, param) in func.params.iter().enumerate() {
            let value = builder.block_params(entry_block)[i];
            locals.insert(LocalId(param.index), value);
        }

        // Compile each block
        for block in &func.blocks {
            let cl_block = block_map[&block.id];

            if block.id != BlockId(0) {
                builder.switch_to_block(cl_block);
                builder.seal_block(cl_block);
            }

            // Compile instructions
            for inst in &block.instructions {
                self.compile_instruction(&mut builder, inst, &mut locals, func_refs)?;
            }

            // Compile terminator
            self.compile_terminator(&mut builder, &block.terminator, &locals, &block_map)?;
        }

        builder.finalize();
        Ok(())
    }

    fn compile_instruction(
        &self,
        builder: &mut CraneliftFunctionBuilder,
        inst: &TypedInstruction,
        locals: &mut HashMap<LocalId, cranelift::prelude::Value>,
        func_refs: &HashMap<u32, FuncRef>,
    ) -> DxResult<()> {
        match inst {
            TypedInstruction::Const { dest, value } => {
                let val = match value {
                    Constant::I32(n) => {
                        let i = builder.ins().iconst(types::I32, *n as i64);
                        builder.ins().fcvt_from_sint(types::F64, i)
                    }
                    Constant::I64(n) => {
                        let i = builder.ins().iconst(types::I64, *n);
                        builder.ins().fcvt_from_sint(types::F64, i)
                    }
                    Constant::F64(n) => builder.ins().f64const(*n),
                    Constant::Bool(b) => builder.ins().f64const(if *b { 1.0 } else { 0.0 }),
                    Constant::String(_) => {
                        // Strings represented as NaN for now
                        builder.ins().f64const(f64::NAN)
                    }
                    Constant::Null => builder.ins().f64const(0.0),
                    Constant::Undefined => builder.ins().f64const(f64::NAN),
                };
                locals.insert(*dest, val);
            }

            TypedInstruction::BinOp {
                dest,
                op,
                left,
                right,
                op_type: _,
            } => {
                let lval = locals[left];
                let rval = locals[right];

                let result = match op {
                    BinOpKind::Add => builder.ins().fadd(lval, rval),
                    BinOpKind::Sub => builder.ins().fsub(lval, rval),
                    BinOpKind::Mul => builder.ins().fmul(lval, rval),
                    BinOpKind::Div => builder.ins().fdiv(lval, rval),
                    BinOpKind::Mod => {
                        // x % y = x - floor(x/y) * y
                        let div = builder.ins().fdiv(lval, rval);
                        let floor = builder.ins().floor(div);
                        let mul = builder.ins().fmul(floor, rval);
                        builder.ins().fsub(lval, mul)
                    }
                    BinOpKind::Lt => {
                        let cmp = builder.ins().fcmp(FloatCC::LessThan, lval, rval);
                        let i = builder.ins().uextend(types::I32, cmp);
                        builder.ins().fcvt_from_sint(types::F64, i)
                    }
                    BinOpKind::Le => {
                        let cmp = builder.ins().fcmp(FloatCC::LessThanOrEqual, lval, rval);
                        let i = builder.ins().uextend(types::I32, cmp);
                        builder.ins().fcvt_from_sint(types::F64, i)
                    }
                    BinOpKind::Gt => {
                        let cmp = builder.ins().fcmp(FloatCC::GreaterThan, lval, rval);
                        let i = builder.ins().uextend(types::I32, cmp);
                        builder.ins().fcvt_from_sint(types::F64, i)
                    }
                    BinOpKind::Ge => {
                        let cmp = builder.ins().fcmp(FloatCC::GreaterThanOrEqual, lval, rval);
                        let i = builder.ins().uextend(types::I32, cmp);
                        builder.ins().fcvt_from_sint(types::F64, i)
                    }
                    BinOpKind::Eq => {
                        let cmp = builder.ins().fcmp(FloatCC::Equal, lval, rval);
                        let i = builder.ins().uextend(types::I32, cmp);
                        builder.ins().fcvt_from_sint(types::F64, i)
                    }
                    BinOpKind::Ne => {
                        let cmp = builder.ins().fcmp(FloatCC::NotEqual, lval, rval);
                        let i = builder.ins().uextend(types::I32, cmp);
                        builder.ins().fcvt_from_sint(types::F64, i)
                    }
                    BinOpKind::And => {
                        // Logical AND: both non-zero
                        let zero = builder.ins().f64const(0.0);
                        let l_nz = builder.ins().fcmp(FloatCC::NotEqual, lval, zero);
                        let r_nz = builder.ins().fcmp(FloatCC::NotEqual, rval, zero);
                        let both = builder.ins().band(l_nz, r_nz);
                        let i = builder.ins().uextend(types::I32, both);
                        builder.ins().fcvt_from_sint(types::F64, i)
                    }
                    BinOpKind::Or => {
                        // Logical OR: either non-zero
                        let zero = builder.ins().f64const(0.0);
                        let l_nz = builder.ins().fcmp(FloatCC::NotEqual, lval, zero);
                        let r_nz = builder.ins().fcmp(FloatCC::NotEqual, rval, zero);
                        let either = builder.ins().bor(l_nz, r_nz);
                        let i = builder.ins().uextend(types::I32, either);
                        builder.ins().fcvt_from_sint(types::F64, i)
                    }
                };

                locals.insert(*dest, result);
            }

            TypedInstruction::Call { dest, function, args } => {
                let arg_values: Vec<cranelift::prelude::Value> =
                    args.iter().map(|a| locals[a]).collect();

                // Look up pre-declared function reference
                if let Some(&func_ref) = func_refs.get(&function.0) {
                    let call = builder.ins().call(func_ref, &arg_values);

                    if let Some(dest) = dest {
                        let results = builder.inst_results(call);
                        if !results.is_empty() {
                            locals.insert(*dest, results[0]);
                        } else {
                            let nan = builder.ins().f64const(f64::NAN);
                            locals.insert(*dest, nan);
                        }
                    }
                } else {
                    // Unknown function - return NaN
                    if let Some(dest) = dest {
                        let nan = builder.ins().f64const(f64::NAN);
                        locals.insert(*dest, nan);
                    }
                }
            }

            TypedInstruction::Copy { dest, src } => {
                let val = locals[src];
                locals.insert(*dest, val);
            }

            TypedInstruction::GetProperty { dest, .. } => {
                // Property access not fully implemented yet
                let nan = builder.ins().f64const(f64::NAN);
                locals.insert(*dest, nan);
            }

            TypedInstruction::SetProperty { .. } => {
                // Property write not fully implemented yet
            }

            TypedInstruction::Allocate { dest, .. } => {
                // Object allocation not fully implemented yet
                let zero = builder.ins().f64const(0.0);
                locals.insert(*dest, zero);
            }
        }

        Ok(())
    }

    fn compile_terminator(
        &self,
        builder: &mut CraneliftFunctionBuilder,
        term: &Terminator,
        locals: &HashMap<LocalId, cranelift::prelude::Value>,
        block_map: &HashMap<BlockId, Block>,
    ) -> DxResult<()> {
        match term {
            Terminator::Return(value) => {
                if let Some(val_id) = value {
                    let val = locals[val_id];
                    builder.ins().return_(&[val]);
                } else {
                    let nan = builder.ins().f64const(f64::NAN);
                    builder.ins().return_(&[nan]);
                }
            }

            Terminator::Goto(target) => {
                let target_block = block_map[target];
                builder.ins().jump(target_block, &[]);
            }

            Terminator::Branch {
                condition,
                then_block,
                else_block,
            } => {
                let cond = locals[condition];
                let then_bl = block_map[then_block];
                let else_bl = block_map[else_block];

                // Convert f64 condition to boolean (non-zero = true)
                let zero = builder.ins().f64const(0.0);
                let is_true = builder.ins().fcmp(FloatCC::NotEqual, cond, zero);

                builder.ins().brif(is_true, then_bl, &[], else_bl, &[]);
            }

            Terminator::Unreachable => {
                // Return NaN for unreachable code
                let nan = builder.ins().f64const(f64::NAN);
                builder.ins().return_(&[nan]);
            }
        }

        Ok(())
    }
}
