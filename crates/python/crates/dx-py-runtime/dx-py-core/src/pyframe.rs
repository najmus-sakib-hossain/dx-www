//! PyFrame - Python stack frame

use crate::header::{PyObjectHeader, TypeTag, ObjectFlags};
use crate::pylist::PyValue;
use crate::pyfunction::PyFunction;
use crate::PyDict;
use std::sync::Arc;

/// Stack frame for function execution
pub struct PyFrame {
    /// Object header
    pub header: PyObjectHeader,
    /// Function being executed
    pub function: Arc<PyFunction>,
    /// Instruction pointer (bytecode offset)
    pub ip: usize,
    /// Local variables
    pub locals: Vec<PyValue>,
    /// Operand stack
    pub stack: Vec<PyValue>,
    /// Block stack (for loops, try/except)
    pub block_stack: Vec<Block>,
    /// Previous frame (caller)
    pub back: Option<Arc<PyFrame>>,
    /// Local namespace (for exec/eval)
    pub local_ns: Option<Arc<PyDict>>,
    /// Line number (for debugging)
    pub lineno: u32,
}

/// Block type for control flow
#[derive(Debug, Clone, Copy)]
pub enum BlockType {
    Loop,
    Except,
    Finally,
    With,
}

/// Block stack entry
#[derive(Debug, Clone, Copy)]
pub struct Block {
    pub block_type: BlockType,
    pub handler: usize,  // Target IP for break/exception
    pub level: usize,    // Stack level when block was entered
}

impl PyFrame {
    /// Create a new frame for a function call
    pub fn new(function: Arc<PyFunction>, back: Option<Arc<PyFrame>>) -> Self {
        let num_locals = function.code.num_locals as usize;
        let stack_size = function.code.stack_size as usize;
        
        Self {
            header: PyObjectHeader::new(TypeTag::Frame, ObjectFlags::NONE),
            function,
            ip: 0,
            locals: vec![PyValue::None; num_locals],
            stack: Vec::with_capacity(stack_size),
            block_stack: Vec::new(),
            back,
            local_ns: None,
            lineno: 0,
        }
    }
    
    /// Push a value onto the stack
    #[inline]
    pub fn push(&mut self, value: PyValue) {
        self.stack.push(value);
    }
    
    /// Pop a value from the stack
    #[inline]
    pub fn pop(&mut self) -> PyValue {
        self.stack.pop().unwrap_or(PyValue::None)
    }
    
    /// Peek at the top of the stack
    #[inline]
    pub fn peek(&self) -> &PyValue {
        self.stack.last().unwrap_or(&PyValue::None)
    }
    
    /// Peek at a value n positions from the top
    #[inline]
    pub fn peek_n(&self, n: usize) -> &PyValue {
        self.stack.get(self.stack.len().saturating_sub(n + 1))
            .unwrap_or(&PyValue::None)
    }
    
    /// Get a local variable
    #[inline]
    pub fn get_local(&self, index: usize) -> &PyValue {
        self.locals.get(index).unwrap_or(&PyValue::None)
    }
    
    /// Set a local variable
    #[inline]
    pub fn set_local(&mut self, index: usize, value: PyValue) {
        if index < self.locals.len() {
            self.locals[index] = value;
        }
    }
    
    /// Push a block onto the block stack
    pub fn push_block(&mut self, block_type: BlockType, handler: usize) {
        self.block_stack.push(Block {
            block_type,
            handler,
            level: self.stack.len(),
        });
    }
    
    /// Pop a block from the block stack
    pub fn pop_block(&mut self) -> Option<Block> {
        self.block_stack.pop()
    }
    
    /// Unwind the stack to a block level
    pub fn unwind_to(&mut self, level: usize) {
        while self.stack.len() > level {
            self.stack.pop();
        }
    }
    
    /// Get the current block
    pub fn current_block(&self) -> Option<&Block> {
        self.block_stack.last()
    }
    
    /// Find a handler for an exception
    pub fn find_exception_handler(&self) -> Option<&Block> {
        self.block_stack
            .iter()
            .rev()
            .find(|b| matches!(b.block_type, BlockType::Except | BlockType::Finally))
    }
    
    /// Get stack depth
    #[inline]
    pub fn stack_depth(&self) -> usize {
        self.stack.len()
    }
    
    /// Clear the stack
    pub fn clear_stack(&mut self) {
        self.stack.clear();
    }
    
    /// Get the function name
    pub fn func_name(&self) -> &str {
        &self.function.name
    }
    
    /// Get the qualified name
    pub fn qualname(&self) -> &str {
        &self.function.qualname
    }
    
    /// Get the module name
    pub fn module_name(&self) -> Option<&str> {
        self.function.module.as_deref()
    }
}

impl std::fmt::Debug for PyFrame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<frame {} at ip={}, stack_depth={}>",
            self.function.qualname,
            self.ip,
            self.stack.len()
        )
    }
}

/// Frame iterator for traceback
pub struct FrameIterator {
    current: Option<Arc<PyFrame>>,
}

impl FrameIterator {
    pub fn new(frame: Arc<PyFrame>) -> Self {
        Self { current: Some(frame) }
    }
}

impl Iterator for FrameIterator {
    type Item = Arc<PyFrame>;
    
    fn next(&mut self) -> Option<Self::Item> {
        let frame = self.current.take()?;
        self.current = frame.back.clone();
        Some(frame)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pyfunction::{CodeRef, Parameter};
    
    fn make_test_function() -> Arc<PyFunction> {
        Arc::new(PyFunction::new(
            "test_func",
            CodeRef {
                bytecode_offset: 0,
                num_locals: 4,
                stack_size: 8,
                num_args: 2,
                num_kwonly_args: 0,
            },
            vec![
                Parameter {
                    name: "a".into(),
                    kind: crate::pyfunction::ParameterKind::PositionalOrKeyword,
                    default: None,
                    annotation: None,
                },
                Parameter {
                    name: "b".into(),
                    kind: crate::pyfunction::ParameterKind::PositionalOrKeyword,
                    default: None,
                    annotation: None,
                },
            ],
        ))
    }
    
    #[test]
    fn test_frame_creation() {
        let func = make_test_function();
        let frame = PyFrame::new(func, None);
        
        assert_eq!(frame.ip, 0);
        assert_eq!(frame.locals.len(), 4);
        assert!(frame.stack.is_empty());
    }
    
    #[test]
    fn test_frame_stack_ops() {
        let func = make_test_function();
        let mut frame = PyFrame::new(func, None);
        
        frame.push(PyValue::Int(1));
        frame.push(PyValue::Int(2));
        
        assert_eq!(frame.stack_depth(), 2);
        
        if let PyValue::Int(v) = frame.pop() {
            assert_eq!(v, 2);
        }
        
        if let PyValue::Int(v) = frame.peek() {
            assert_eq!(*v, 1);
        }
    }
    
    #[test]
    fn test_frame_locals() {
        let func = make_test_function();
        let mut frame = PyFrame::new(func, None);
        
        frame.set_local(0, PyValue::Int(42));
        
        if let PyValue::Int(v) = frame.get_local(0) {
            assert_eq!(*v, 42);
        }
    }
    
    #[test]
    fn test_frame_blocks() {
        let func = make_test_function();
        let mut frame = PyFrame::new(func, None);
        
        frame.push(PyValue::Int(1));
        frame.push_block(BlockType::Loop, 100);
        frame.push(PyValue::Int(2));
        frame.push(PyValue::Int(3));
        
        let block = frame.pop_block().unwrap();
        assert_eq!(block.level, 1);
        
        frame.unwind_to(block.level);
        assert_eq!(frame.stack_depth(), 1);
    }
}
