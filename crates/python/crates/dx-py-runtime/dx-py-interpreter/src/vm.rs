//! Virtual Machine for DX-Py runtime

use crate::{InterpreterError, InterpreterResult};
use dx_py_core::pylist::PyValue;
use dx_py_core::pyframe::PyFrame;
use dx_py_core::pyfunction::{PyFunction, PyBuiltinFunction};
use dx_py_core::builtins::get_builtins;
use dx_py_core::PyDict;
use dashmap::DashMap;
use std::sync::Arc;

/// Virtual Machine state
pub struct VirtualMachine {
    /// Global namespace
    pub globals: Arc<PyDict>,
    /// Built-in functions
    pub builtins: DashMap<String, Arc<PyBuiltinFunction>>,
    /// Loaded modules
    pub modules: DashMap<String, Arc<PyDict>>,
    /// Current frame (if any)
    pub current_frame: Option<Arc<PyFrame>>,
}

impl VirtualMachine {
    /// Create a new VM
    pub fn new() -> Self {
        let vm = Self {
            globals: Arc::new(PyDict::new()),
            builtins: DashMap::new(),
            modules: DashMap::new(),
            current_frame: None,
        };
        
        // Initialize builtins
        for builtin in get_builtins() {
            vm.builtins.insert(builtin.name.clone(), Arc::new(builtin));
        }
        
        vm
    }
    
    /// Get a global variable
    pub fn get_global(&self, name: &str) -> Option<PyValue> {
        use dx_py_core::pydict::PyKey;
        self.globals.getitem(&PyKey::Str(Arc::from(name))).ok()
    }
    
    /// Set a global variable
    pub fn set_global(&self, name: &str, value: PyValue) {
        use dx_py_core::pydict::PyKey;
        self.globals.setitem(PyKey::Str(Arc::from(name)), value);
    }
    
    /// Get a builtin function
    pub fn get_builtin(&self, name: &str) -> Option<Arc<PyBuiltinFunction>> {
        self.builtins.get(name).map(|r| r.clone())
    }
    
    /// Call a builtin function
    pub fn call_builtin(&self, name: &str, args: &[PyValue]) -> InterpreterResult<PyValue> {
        let func = self.get_builtin(name)
            .ok_or_else(|| InterpreterError::NameError(
                format!("name '{}' is not defined", name)
            ))?;
        
        func.call(args).map_err(|e| InterpreterError::Core(e))
    }
    
    /// Call a function
    pub fn call_function(
        &self,
        _func: &PyFunction,
        _args: &[PyValue],
    ) -> InterpreterResult<PyValue> {
        // TODO: Implement full function call
        // This would create a new frame and execute the function's bytecode
        Err(InterpreterError::Runtime("Function calls not yet implemented".into()))
    }
    
    /// Import a module
    pub fn import_module(&self, name: &str) -> InterpreterResult<Arc<PyDict>> {
        // Check if already loaded
        if let Some(module) = self.modules.get(name) {
            return Ok(module.clone());
        }
        
        // TODO: Implement module loading from DPM files
        Err(InterpreterError::ImportError(format!(
            "No module named '{}'", name
        )))
    }
    
    /// Execute a simple expression (for REPL)
    pub fn eval_expr(&self, expr: &str) -> InterpreterResult<PyValue> {
        // Very simple expression evaluator for testing
        let expr = expr.trim();
        
        // Try to parse as integer
        if let Ok(i) = expr.parse::<i64>() {
            return Ok(PyValue::Int(i));
        }
        
        // Try to parse as float
        if let Ok(f) = expr.parse::<f64>() {
            return Ok(PyValue::Float(f));
        }
        
        // Check for string literal
        if (expr.starts_with('"') && expr.ends_with('"'))
            || (expr.starts_with('\'') && expr.ends_with('\''))
        {
            let s = &expr[1..expr.len()-1];
            return Ok(PyValue::Str(Arc::from(s)));
        }
        
        // Check for None, True, False
        match expr {
            "None" => return Ok(PyValue::None),
            "True" => return Ok(PyValue::Bool(true)),
            "False" => return Ok(PyValue::Bool(false)),
            _ => {}
        }
        
        // Check for builtin function call
        if let Some(paren_pos) = expr.find('(') {
            if expr.ends_with(')') {
                let func_name = &expr[..paren_pos];
                let args_str = &expr[paren_pos+1..expr.len()-1];
                
                // Parse arguments (very simplified)
                let args: Vec<PyValue> = if args_str.is_empty() {
                    vec![]
                } else {
                    args_str
                        .split(',')
                        .map(|s| self.eval_expr(s.trim()))
                        .collect::<Result<Vec<_>, _>>()?
                };
                
                return self.call_builtin(func_name, &args);
            }
        }
        
        // Check for variable
        if let Some(value) = self.get_global(expr) {
            return Ok(value);
        }
        
        Err(InterpreterError::NameError(format!(
            "name '{}' is not defined", expr
        )))
    }
}

impl Default for VirtualMachine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_vm_creation() {
        let vm = VirtualMachine::new();
        assert!(vm.builtins.contains_key("print"));
        assert!(vm.builtins.contains_key("len"));
    }
    
    #[test]
    fn test_vm_globals() {
        let vm = VirtualMachine::new();
        vm.set_global("x", PyValue::Int(42));
        
        let value = vm.get_global("x").unwrap();
        if let PyValue::Int(v) = value {
            assert_eq!(v, 42);
        } else {
            panic!("Expected Int");
        }
    }
    
    #[test]
    fn test_vm_builtin_call() {
        let vm = VirtualMachine::new();
        
        let result = vm.call_builtin("len", &[PyValue::Str(Arc::from("hello"))]).unwrap();
        if let PyValue::Int(len) = result {
            assert_eq!(len, 5);
        } else {
            panic!("Expected Int");
        }
    }
    
    #[test]
    fn test_vm_eval_expr() {
        let vm = VirtualMachine::new();
        
        // Integer
        let result = vm.eval_expr("42").unwrap();
        assert!(matches!(result, PyValue::Int(42)));
        
        // Float
        let result = vm.eval_expr("3.14").unwrap();
        if let PyValue::Float(f) = result {
            assert!((f - 3.14).abs() < 0.001);
        }
        
        // String
        let result = vm.eval_expr("'hello'").unwrap();
        if let PyValue::Str(s) = result {
            assert_eq!(&*s, "hello");
        }
        
        // Builtin call
        let result = vm.eval_expr("len('test')").unwrap();
        assert!(matches!(result, PyValue::Int(4)));
    }
}
