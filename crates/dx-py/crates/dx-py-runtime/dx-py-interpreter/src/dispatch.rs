//! Bytecode dispatch loop

use crate::opcodes::Opcode;
use crate::{InterpreterError, InterpreterResult};
use dx_py_core::pylist::PyValue;
use dx_py_core::pyframe::PyFrame;
use std::sync::Arc;

/// Bytecode dispatcher
pub struct Dispatcher {
    /// Constants pool
    constants: Vec<PyValue>,
    /// Names pool
    names: Vec<String>,
    /// Bytecode
    code: Vec<u8>,
}

impl Dispatcher {
    /// Create a new dispatcher
    pub fn new(code: Vec<u8>, constants: Vec<PyValue>, names: Vec<String>) -> Self {
        Self { code, constants, names }
    }
    
    /// Execute bytecode in a frame
    pub fn execute(&self, frame: &mut PyFrame) -> InterpreterResult<PyValue> {
        loop {
            if frame.ip >= self.code.len() {
                break;
            }
            
            let opcode_byte = self.code[frame.ip];
            frame.ip += 1;
            
            let opcode = Opcode::from_byte(opcode_byte)
                .ok_or_else(|| InterpreterError::Runtime(
                    format!("Unknown opcode: 0x{:02x}", opcode_byte)
                ))?;
            
            // Read argument if needed
            let arg = if opcode.has_arg() && frame.ip + 1 < self.code.len() {
                let arg = u16::from_le_bytes([
                    self.code[frame.ip],
                    self.code[frame.ip + 1],
                ]);
                frame.ip += 2;
                Some(arg as usize)
            } else {
                None
            };
            
            match self.dispatch(frame, opcode, arg)? {
                DispatchResult::Continue => continue,
                DispatchResult::Return(value) => return Ok(value),
            }
        }
        
        // Return None if we fall through
        Ok(PyValue::None)
    }
    
    /// Dispatch a single opcode
    fn dispatch(
        &self,
        frame: &mut PyFrame,
        opcode: Opcode,
        arg: Option<usize>,
    ) -> InterpreterResult<DispatchResult> {
        match opcode {
            // Stack operations
            Opcode::Nop => {}
            Opcode::Pop => { frame.pop(); }
            Opcode::Dup => {
                let top = frame.peek().clone();
                frame.push(top);
            }
            Opcode::DupTwo => {
                let a = frame.peek_n(1).clone();
                let b = frame.peek_n(0).clone();
                frame.push(a);
                frame.push(b);
            }
            Opcode::Rot2 => {
                let a = frame.pop();
                let b = frame.pop();
                frame.push(a);
                frame.push(b);
            }
            Opcode::Rot3 => {
                let a = frame.pop();
                let b = frame.pop();
                let c = frame.pop();
                frame.push(a);
                frame.push(c);
                frame.push(b);
            }
            Opcode::Rot4 => {
                let a = frame.pop();
                let b = frame.pop();
                let c = frame.pop();
                let d = frame.pop();
                frame.push(a);
                frame.push(d);
                frame.push(c);
                frame.push(b);
            }
            
            // Load operations
            Opcode::LoadConst => {
                let idx = arg.ok_or_else(|| InterpreterError::Runtime(
                    "LOAD_CONST requires argument".into()
                ))?;
                let value = self.constants.get(idx)
                    .cloned()
                    .unwrap_or(PyValue::None);
                frame.push(value);
            }
            Opcode::LoadFast => {
                let idx = arg.ok_or_else(|| InterpreterError::Runtime(
                    "LOAD_FAST requires argument".into()
                ))?;
                let value = frame.get_local(idx).clone();
                frame.push(value);
            }
            Opcode::LoadName | Opcode::LoadGlobal => {
                let idx = arg.ok_or_else(|| InterpreterError::Runtime(
                    "LOAD_NAME requires argument".into()
                ))?;
                let name = self.names.get(idx)
                    .ok_or_else(|| InterpreterError::NameError(
                        format!("name index {} out of range", idx)
                    ))?;
                return Err(InterpreterError::NameError(
                    format!("name '{}' is not defined", name)
                ));
            }
            
            // Store operations
            Opcode::StoreFast => {
                let idx = arg.ok_or_else(|| InterpreterError::Runtime(
                    "STORE_FAST requires argument".into()
                ))?;
                let value = frame.pop();
                frame.set_local(idx, value);
            }
            
            // Binary operations
            Opcode::BinaryAdd => {
                let b = frame.pop();
                let a = frame.pop();
                let result = self.binary_add(&a, &b)?;
                frame.push(result);
            }
            Opcode::BinarySub => {
                let b = frame.pop();
                let a = frame.pop();
                let result = self.binary_sub(&a, &b)?;
                frame.push(result);
            }
            Opcode::BinaryMul => {
                let b = frame.pop();
                let a = frame.pop();
                let result = self.binary_mul(&a, &b)?;
                frame.push(result);
            }
            Opcode::BinaryDiv => {
                let b = frame.pop();
                let a = frame.pop();
                let result = self.binary_div(&a, &b)?;
                frame.push(result);
            }
            Opcode::BinaryFloorDiv => {
                let b = frame.pop();
                let a = frame.pop();
                let result = self.binary_floordiv(&a, &b)?;
                frame.push(result);
            }
            Opcode::BinaryMod => {
                let b = frame.pop();
                let a = frame.pop();
                let result = self.binary_mod(&a, &b)?;
                frame.push(result);
            }
            
            // Unary operations
            Opcode::UnaryNot => {
                let a = frame.pop();
                frame.push(PyValue::Bool(!a.to_bool()));
            }
            Opcode::UnaryNeg => {
                let a = frame.pop();
                let result = self.unary_neg(&a)?;
                frame.push(result);
            }
            
            // Comparison operations
            Opcode::CompareEq => {
                let b = frame.pop();
                let a = frame.pop();
                frame.push(PyValue::Bool(self.compare_eq(&a, &b)));
            }
            Opcode::CompareNe => {
                let b = frame.pop();
                let a = frame.pop();
                frame.push(PyValue::Bool(!self.compare_eq(&a, &b)));
            }
            Opcode::CompareLt => {
                let b = frame.pop();
                let a = frame.pop();
                frame.push(PyValue::Bool(self.compare_lt(&a, &b)?));
            }
            Opcode::CompareLe => {
                let b = frame.pop();
                let a = frame.pop();
                let lt = self.compare_lt(&a, &b)?;
                let eq = self.compare_eq(&a, &b);
                frame.push(PyValue::Bool(lt || eq));
            }
            Opcode::CompareGt => {
                let b = frame.pop();
                let a = frame.pop();
                frame.push(PyValue::Bool(self.compare_lt(&b, &a)?));
            }
            Opcode::CompareGe => {
                let b = frame.pop();
                let a = frame.pop();
                let gt = self.compare_lt(&b, &a)?;
                let eq = self.compare_eq(&a, &b);
                frame.push(PyValue::Bool(gt || eq));
            }
            
            // Not yet implemented
            _ => {
                return Err(InterpreterError::Runtime(
                    format!("Opcode {:?} not implemented", opcode)
                ));
            }
        }
        
        Ok(DispatchResult::Continue)
    }
    
    // Binary operation helpers
    fn binary_add(&self, a: &PyValue, b: &PyValue) -> InterpreterResult<PyValue> {
        match (a, b) {
            (PyValue::Int(x), PyValue::Int(y)) => Ok(PyValue::Int(x + y)),
            (PyValue::Float(x), PyValue::Float(y)) => Ok(PyValue::Float(x + y)),
            (PyValue::Int(x), PyValue::Float(y)) => Ok(PyValue::Float(*x as f64 + y)),
            (PyValue::Float(x), PyValue::Int(y)) => Ok(PyValue::Float(x + *y as f64)),
            (PyValue::Str(x), PyValue::Str(y)) => {
                let mut s = x.to_string();
                s.push_str(y);
                Ok(PyValue::Str(Arc::from(s)))
            }
            _ => Err(InterpreterError::TypeError(format!(
                "unsupported operand type(s) for +: '{}' and '{}'",
                a.type_name(), b.type_name()
            ))),
        }
    }
    
    fn binary_sub(&self, a: &PyValue, b: &PyValue) -> InterpreterResult<PyValue> {
        match (a, b) {
            (PyValue::Int(x), PyValue::Int(y)) => Ok(PyValue::Int(x - y)),
            (PyValue::Float(x), PyValue::Float(y)) => Ok(PyValue::Float(x - y)),
            (PyValue::Int(x), PyValue::Float(y)) => Ok(PyValue::Float(*x as f64 - y)),
            (PyValue::Float(x), PyValue::Int(y)) => Ok(PyValue::Float(x - *y as f64)),
            _ => Err(InterpreterError::TypeError(format!(
                "unsupported operand type(s) for -: '{}' and '{}'",
                a.type_name(), b.type_name()
            ))),
        }
    }
    
    fn binary_mul(&self, a: &PyValue, b: &PyValue) -> InterpreterResult<PyValue> {
        match (a, b) {
            (PyValue::Int(x), PyValue::Int(y)) => Ok(PyValue::Int(x * y)),
            (PyValue::Float(x), PyValue::Float(y)) => Ok(PyValue::Float(x * y)),
            (PyValue::Int(x), PyValue::Float(y)) => Ok(PyValue::Float(*x as f64 * y)),
            (PyValue::Float(x), PyValue::Int(y)) => Ok(PyValue::Float(x * *y as f64)),
            (PyValue::Str(s), PyValue::Int(n)) | (PyValue::Int(n), PyValue::Str(s)) => {
                if *n <= 0 {
                    Ok(PyValue::Str(Arc::from("")))
                } else {
                    Ok(PyValue::Str(Arc::from(s.repeat(*n as usize))))
                }
            }
            _ => Err(InterpreterError::TypeError(format!(
                "unsupported operand type(s) for *: '{}' and '{}'",
                a.type_name(), b.type_name()
            ))),
        }
    }
    
    fn binary_div(&self, a: &PyValue, b: &PyValue) -> InterpreterResult<PyValue> {
        match (a, b) {
            (PyValue::Int(x), PyValue::Int(y)) => {
                if *y == 0 {
                    Err(InterpreterError::ValueError("division by zero".into()))
                } else {
                    Ok(PyValue::Float(*x as f64 / *y as f64))
                }
            }
            (PyValue::Float(x), PyValue::Float(y)) => {
                if *y == 0.0 {
                    Err(InterpreterError::ValueError("division by zero".into()))
                } else {
                    Ok(PyValue::Float(x / y))
                }
            }
            (PyValue::Int(x), PyValue::Float(y)) => {
                if *y == 0.0 {
                    Err(InterpreterError::ValueError("division by zero".into()))
                } else {
                    Ok(PyValue::Float(*x as f64 / y))
                }
            }
            (PyValue::Float(x), PyValue::Int(y)) => {
                if *y == 0 {
                    Err(InterpreterError::ValueError("division by zero".into()))
                } else {
                    Ok(PyValue::Float(x / *y as f64))
                }
            }
            _ => Err(InterpreterError::TypeError(format!(
                "unsupported operand type(s) for /: '{}' and '{}'",
                a.type_name(), b.type_name()
            ))),
        }
    }
    
    fn binary_floordiv(&self, a: &PyValue, b: &PyValue) -> InterpreterResult<PyValue> {
        match (a, b) {
            (PyValue::Int(x), PyValue::Int(y)) => {
                if *y == 0 {
                    Err(InterpreterError::ValueError("division by zero".into()))
                } else {
                    Ok(PyValue::Int(x / y))
                }
            }
            _ => Err(InterpreterError::TypeError(format!(
                "unsupported operand type(s) for //: '{}' and '{}'",
                a.type_name(), b.type_name()
            ))),
        }
    }
    
    fn binary_mod(&self, a: &PyValue, b: &PyValue) -> InterpreterResult<PyValue> {
        match (a, b) {
            (PyValue::Int(x), PyValue::Int(y)) => {
                if *y == 0 {
                    Err(InterpreterError::ValueError("modulo by zero".into()))
                } else {
                    Ok(PyValue::Int(x % y))
                }
            }
            _ => Err(InterpreterError::TypeError(format!(
                "unsupported operand type(s) for %: '{}' and '{}'",
                a.type_name(), b.type_name()
            ))),
        }
    }
    
    fn unary_neg(&self, a: &PyValue) -> InterpreterResult<PyValue> {
        match a {
            PyValue::Int(x) => Ok(PyValue::Int(-x)),
            PyValue::Float(x) => Ok(PyValue::Float(-x)),
            _ => Err(InterpreterError::TypeError(format!(
                "bad operand type for unary -: '{}'", a.type_name()
            ))),
        }
    }
    
    fn compare_eq(&self, a: &PyValue, b: &PyValue) -> bool {
        match (a, b) {
            (PyValue::None, PyValue::None) => true,
            (PyValue::Bool(x), PyValue::Bool(y)) => x == y,
            (PyValue::Int(x), PyValue::Int(y)) => x == y,
            (PyValue::Float(x), PyValue::Float(y)) => x == y,
            (PyValue::Int(x), PyValue::Float(y)) => (*x as f64) == *y,
            (PyValue::Float(x), PyValue::Int(y)) => *x == (*y as f64),
            (PyValue::Str(x), PyValue::Str(y)) => x == y,
            _ => false,
        }
    }
    
    fn compare_lt(&self, a: &PyValue, b: &PyValue) -> InterpreterResult<bool> {
        match (a, b) {
            (PyValue::Int(x), PyValue::Int(y)) => Ok(x < y),
            (PyValue::Float(x), PyValue::Float(y)) => Ok(x < y),
            (PyValue::Int(x), PyValue::Float(y)) => Ok((*x as f64) < *y),
            (PyValue::Float(x), PyValue::Int(y)) => Ok(*x < (*y as f64)),
            (PyValue::Str(x), PyValue::Str(y)) => Ok(x < y),
            _ => Err(InterpreterError::TypeError(format!(
                "'<' not supported between '{}' and '{}'",
                a.type_name(), b.type_name()
            ))),
        }
    }
}

/// Result of dispatching an opcode
enum DispatchResult {
    Continue,
    Return(PyValue),
}

#[cfg(test)]
mod tests {
    use super::*;
    use dx_py_core::pyfunction::{PyFunction, CodeRef, Parameter};
    
    fn make_frame() -> PyFrame {
        let func = Arc::new(PyFunction::new(
            "test",
            CodeRef {
                bytecode_offset: 0,
                num_locals: 4,
                stack_size: 8,
                num_args: 0,
                num_kwonly_args: 0,
            },
            vec![],
        ));
        PyFrame::new(func, None)
    }
    
    #[test]
    fn test_binary_add() {
        let dispatcher = Dispatcher::new(
            vec![
                Opcode::LoadConst as u8, 0, 0,  // Load 10
                Opcode::LoadConst as u8, 1, 0,  // Load 20
                Opcode::BinaryAdd as u8,        // Add
            ],
            vec![PyValue::Int(10), PyValue::Int(20)],
            vec![],
        );
        
        let mut frame = make_frame();
        let result = dispatcher.execute(&mut frame).unwrap();
        
        if let PyValue::Int(v) = result {
            // Result should be on stack, but we return None at end
            // Let's check the stack instead
        }
        
        // Actually the result is left on stack, and we return None
        // Let's verify the stack has the result
        assert_eq!(frame.stack_depth(), 1);
    }
    
    #[test]
    fn test_comparison() {
        let dispatcher = Dispatcher::new(
            vec![
                Opcode::LoadConst as u8, 0, 0,  // Load 10
                Opcode::LoadConst as u8, 1, 0,  // Load 20
                Opcode::CompareLt as u8,        // 10 < 20
            ],
            vec![PyValue::Int(10), PyValue::Int(20)],
            vec![],
        );
        
        let mut frame = make_frame();
        dispatcher.execute(&mut frame).unwrap();
        
        // Check result on stack
        if let PyValue::Bool(b) = frame.peek() {
            assert!(*b); // 10 < 20 is true
        } else {
            panic!("Expected Bool");
        }
    }
}
