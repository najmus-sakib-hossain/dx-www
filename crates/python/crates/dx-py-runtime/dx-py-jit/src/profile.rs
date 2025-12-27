//! Function profiling for JIT compilation decisions

use std::sync::atomic::{AtomicU32, AtomicU64, AtomicU8, Ordering};

/// Python type tags for type feedback
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PyType {
    Unknown = 0,
    None = 1,
    Bool = 2,
    Int = 3,
    Float = 4,
    Str = 5,
    Bytes = 6,
    List = 7,
    Tuple = 8,
    Dict = 9,
    Set = 10,
    Function = 11,
    Class = 12,
    Object = 13,
    Module = 14,
    // Add more as needed
}

impl PyType {
    pub fn from_u8(value: u8) -> Self {
        match value {
            0 => Self::Unknown,
            1 => Self::None,
            2 => Self::Bool,
            3 => Self::Int,
            4 => Self::Float,
            5 => Self::Str,
            6 => Self::Bytes,
            7 => Self::List,
            8 => Self::Tuple,
            9 => Self::Dict,
            10 => Self::Set,
            11 => Self::Function,
            12 => Self::Class,
            13 => Self::Object,
            14 => Self::Module,
            _ => Self::Unknown,
        }
    }
}

/// Function profile collected during interpretation
#[derive(Default)]
pub struct FunctionProfile {
    /// Number of times this function has been called
    pub call_count: AtomicU64,
    /// Type feedback for each bytecode location
    pub type_feedback: Vec<TypeFeedback>,
    /// Branch counts (taken, not_taken) for each branch
    pub branch_counts: Vec<(AtomicU64, AtomicU64)>,
    /// Number of deoptimizations
    pub deopt_count: AtomicU32,
}

impl FunctionProfile {
    /// Create a new profile for a function with the given number of bytecode locations
    pub fn new(bytecode_len: usize, branch_count: usize) -> Self {
        Self {
            call_count: AtomicU64::new(0),
            type_feedback: (0..bytecode_len).map(|_| TypeFeedback::new()).collect(),
            branch_counts: (0..branch_count).map(|_| (AtomicU64::new(0), AtomicU64::new(0))).collect(),
            deopt_count: AtomicU32::new(0),
        }
    }
    
    /// Record a function call
    #[inline]
    pub fn record_call(&self) {
        self.call_count.fetch_add(1, Ordering::Relaxed);
    }
    
    /// Get the call count
    #[inline]
    pub fn get_call_count(&self) -> u64 {
        self.call_count.load(Ordering::Relaxed)
    }
    
    /// Record a type observation at a bytecode location
    #[inline]
    pub fn record_type(&self, bc_offset: usize, py_type: PyType) {
        if let Some(feedback) = self.type_feedback.get(bc_offset) {
            feedback.record(py_type);
        }
    }
    
    /// Record a branch taken
    #[inline]
    pub fn record_branch_taken(&self, branch_idx: usize) {
        if let Some((taken, _)) = self.branch_counts.get(branch_idx) {
            taken.fetch_add(1, Ordering::Relaxed);
        }
    }
    
    /// Record a branch not taken
    #[inline]
    pub fn record_branch_not_taken(&self, branch_idx: usize) {
        if let Some((_, not_taken)) = self.branch_counts.get(branch_idx) {
            not_taken.fetch_add(1, Ordering::Relaxed);
        }
    }
    
    /// Record a deoptimization
    #[inline]
    pub fn record_deopt(&self) {
        self.deopt_count.fetch_add(1, Ordering::Relaxed);
    }
    
    /// Get the deoptimization count
    #[inline]
    pub fn get_deopt_count(&self) -> u32 {
        self.deopt_count.load(Ordering::Relaxed)
    }
    
    /// Get branch probability (taken / total)
    pub fn get_branch_probability(&self, branch_idx: usize) -> Option<f64> {
        self.branch_counts.get(branch_idx).map(|(taken, not_taken)| {
            let t = taken.load(Ordering::Relaxed) as f64;
            let n = not_taken.load(Ordering::Relaxed) as f64;
            let total = t + n;
            if total > 0.0 { t / total } else { 0.5 }
        })
    }
}

/// Type feedback for a single bytecode location
pub struct TypeFeedback {
    /// Observed types (up to 4)
    observed_types: [AtomicU8; 4],
    /// Number of types observed
    type_count: AtomicU8,
}

impl TypeFeedback {
    /// Create new type feedback
    pub fn new() -> Self {
        Self {
            observed_types: [
                AtomicU8::new(PyType::Unknown as u8),
                AtomicU8::new(PyType::Unknown as u8),
                AtomicU8::new(PyType::Unknown as u8),
                AtomicU8::new(PyType::Unknown as u8),
            ],
            type_count: AtomicU8::new(0),
        }
    }
    
    /// Record an observed type
    pub fn record(&self, py_type: PyType) {
        let type_byte = py_type as u8;
        let count = self.type_count.load(Ordering::Relaxed) as usize;
        
        // Check if we already have this type
        for i in 0..count.min(4) {
            if self.observed_types[i].load(Ordering::Relaxed) == type_byte {
                return; // Already recorded
            }
        }
        
        // Add new type if we have room
        if count < 4 {
            self.observed_types[count].store(type_byte, Ordering::Relaxed);
            self.type_count.fetch_add(1, Ordering::Relaxed);
        }
    }
    
    /// Check if this site is monomorphic (single type)
    pub fn is_monomorphic(&self) -> bool {
        self.type_count.load(Ordering::Relaxed) == 1
    }
    
    /// Check if this site is polymorphic (2-4 types)
    pub fn is_polymorphic(&self) -> bool {
        let count = self.type_count.load(Ordering::Relaxed);
        count >= 2 && count <= 4
    }
    
    /// Check if this site is megamorphic (too many types)
    pub fn is_megamorphic(&self) -> bool {
        self.type_count.load(Ordering::Relaxed) > 4
    }
    
    /// Get the observed types
    pub fn get_types(&self) -> Vec<PyType> {
        let count = self.type_count.load(Ordering::Relaxed) as usize;
        (0..count.min(4))
            .map(|i| PyType::from_u8(self.observed_types[i].load(Ordering::Relaxed)))
            .collect()
    }
    
    /// Get the primary type (most likely to be observed)
    pub fn get_primary_type(&self) -> Option<PyType> {
        if self.type_count.load(Ordering::Relaxed) > 0 {
            Some(PyType::from_u8(self.observed_types[0].load(Ordering::Relaxed)))
        } else {
            None
        }
    }
}

impl Default for TypeFeedback {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_function_profile() {
        let profile = FunctionProfile::new(10, 2);
        
        assert_eq!(profile.get_call_count(), 0);
        
        profile.record_call();
        profile.record_call();
        
        assert_eq!(profile.get_call_count(), 2);
    }
    
    #[test]
    fn test_type_feedback() {
        let feedback = TypeFeedback::new();
        
        assert!(!feedback.is_monomorphic());
        
        feedback.record(PyType::Int);
        assert!(feedback.is_monomorphic());
        assert_eq!(feedback.get_types(), vec![PyType::Int]);
        
        feedback.record(PyType::Float);
        assert!(feedback.is_polymorphic());
        assert_eq!(feedback.get_types(), vec![PyType::Int, PyType::Float]);
        
        // Recording same type again shouldn't add it
        feedback.record(PyType::Int);
        assert_eq!(feedback.get_types().len(), 2);
    }
    
    #[test]
    fn test_branch_probability() {
        let profile = FunctionProfile::new(10, 1);
        
        // 75% taken
        for _ in 0..75 {
            profile.record_branch_taken(0);
        }
        for _ in 0..25 {
            profile.record_branch_not_taken(0);
        }
        
        let prob = profile.get_branch_probability(0).unwrap();
        assert!((prob - 0.75).abs() < 0.01);
    }
}
