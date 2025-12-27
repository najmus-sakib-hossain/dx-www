//! JIT Integration for the Interpreter
//!
//! This module wires the interpreter to the JIT compiler, enabling
//! tiered compilation based on execution profiles.

use dx_py_jit::{TieredJit, CompilationTier, FunctionProfile, OsrManager};
use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;

/// JIT integration for the interpreter
pub struct JitIntegration {
    /// The tiered JIT compiler
    jit: Arc<TieredJit>,
    /// Function profiles for tier promotion decisions
    profiles: RwLock<HashMap<String, FunctionProfile>>,
    /// OSR manager for on-stack replacement
    osr: Arc<OsrManager>,
    /// Whether JIT is enabled
    enabled: bool,
    /// Tier thresholds
    tier1_threshold: u64,
    tier2_threshold: u64,
    tier3_threshold: u64,
}

impl JitIntegration {
    /// Create a new JIT integration
    pub fn new() -> Self {
        Self {
            jit: Arc::new(TieredJit::new()),
            profiles: RwLock::new(HashMap::new()),
            osr: Arc::new(OsrManager::new()),
            enabled: true,
            tier1_threshold: 100,
            tier2_threshold: 1000,
            tier3_threshold: 10000,
        }
    }
    
    /// Create with custom thresholds
    pub fn with_thresholds(tier1: u64, tier2: u64, tier3: u64) -> Self {
        Self {
            jit: Arc::new(TieredJit::new()),
            profiles: RwLock::new(HashMap::new()),
            osr: Arc::new(OsrManager::new()),
            enabled: true,
            tier1_threshold: tier1,
            tier2_threshold: tier2,
            tier3_threshold: tier3,
        }
    }
    
    /// Enable or disable JIT
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
    
    /// Check if JIT is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
    
    /// Record a function call and check for tier promotion
    pub fn record_call(&self, func_name: &str) -> Option<CompilationTier> {
        if !self.enabled {
            return None;
        }
        
        let mut profiles = self.profiles.write();
        let profile = profiles.entry(func_name.to_string())
            .or_insert_with(|| FunctionProfile::new(func_name.to_string()));
        
        profile.record_call();
        let call_count = profile.call_count();
        
        // Check for tier promotion
        let current_tier = profile.current_tier();
        let new_tier = if call_count >= self.tier3_threshold {
            CompilationTier::Tier3
        } else if call_count >= self.tier2_threshold {
            CompilationTier::Tier2
        } else if call_count >= self.tier1_threshold {
            CompilationTier::Tier1
        } else {
            CompilationTier::Tier0
        };
        
        if new_tier > current_tier {
            profile.set_tier(new_tier);
            Some(new_tier)
        } else {
            None
        }
    }
    
    /// Get the current tier for a function
    pub fn get_tier(&self, func_name: &str) -> CompilationTier {
        self.profiles.read()
            .get(func_name)
            .map(|p| p.current_tier())
            .unwrap_or(CompilationTier::Tier0)
    }
    
    /// Get the call count for a function
    pub fn get_call_count(&self, func_name: &str) -> u64 {
        self.profiles.read()
            .get(func_name)
            .map(|p| p.call_count())
            .unwrap_or(0)
    }
    
    /// Request compilation at a specific tier
    pub fn compile(&self, func_name: &str, tier: CompilationTier) -> Result<(), JitError> {
        if !self.enabled {
            return Err(JitError::Disabled);
        }
        
        self.jit.compile(func_name, tier)
            .map_err(|e| JitError::CompilationFailed(e.to_string()))
    }
    
    /// Check if a function has compiled code at the given tier
    pub fn has_compiled(&self, func_name: &str, tier: CompilationTier) -> bool {
        self.jit.has_compiled(func_name, tier)
    }
    
    /// Trigger deoptimization for a function
    pub fn deoptimize(&self, func_name: &str) -> Result<(), JitError> {
        self.jit.deoptimize(func_name)
            .map_err(|e| JitError::DeoptFailed(e.to_string()))
    }
    
    /// Check if OSR is available at the given bytecode offset
    pub fn can_osr(&self, func_name: &str, bytecode_offset: usize) -> bool {
        self.osr.can_osr(func_name, bytecode_offset)
    }
    
    /// Perform OSR transition
    pub fn do_osr(&self, func_name: &str, bytecode_offset: usize) -> Result<(), JitError> {
        self.osr.transition(func_name, bytecode_offset)
            .map_err(|e| JitError::OsrFailed(e.to_string()))
    }
    
    /// Get JIT statistics
    pub fn stats(&self) -> JitStats {
        let profiles = self.profiles.read();
        let mut stats = JitStats::default();
        
        for profile in profiles.values() {
            stats.total_calls += profile.call_count();
            match profile.current_tier() {
                CompilationTier::Tier0 => stats.tier0_functions += 1,
                CompilationTier::Tier1 => stats.tier1_functions += 1,
                CompilationTier::Tier2 => stats.tier2_functions += 1,
                CompilationTier::Tier3 => stats.tier3_functions += 1,
            }
        }
        
        stats.total_functions = profiles.len();
        stats
    }
    
    /// Reset all profiles
    pub fn reset(&self) {
        self.profiles.write().clear();
    }
}

impl Default for JitIntegration {
    fn default() -> Self {
        Self::new()
    }
}

/// JIT integration errors
#[derive(Debug, thiserror::Error)]
pub enum JitError {
    #[error("JIT is disabled")]
    Disabled,
    
    #[error("Compilation failed: {0}")]
    CompilationFailed(String),
    
    #[error("Deoptimization failed: {0}")]
    DeoptFailed(String),
    
    #[error("OSR failed: {0}")]
    OsrFailed(String),
}

/// JIT statistics
#[derive(Debug, Default, Clone)]
pub struct JitStats {
    pub total_functions: usize,
    pub total_calls: u64,
    pub tier0_functions: usize,
    pub tier1_functions: usize,
    pub tier2_functions: usize,
    pub tier3_functions: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_jit_integration_creation() {
        let jit = JitIntegration::new();
        assert!(jit.is_enabled());
    }
    
    #[test]
    fn test_tier_promotion() {
        let jit = JitIntegration::with_thresholds(10, 100, 1000);
        
        // Initial tier is 0
        assert_eq!(jit.get_tier("test_func"), CompilationTier::Tier0);
        
        // Record calls until tier 1
        for _ in 0..9 {
            assert!(jit.record_call("test_func").is_none());
        }
        
        // 10th call should trigger tier 1
        assert_eq!(jit.record_call("test_func"), Some(CompilationTier::Tier1));
        assert_eq!(jit.get_tier("test_func"), CompilationTier::Tier1);
    }
    
    #[test]
    fn test_disabled_jit() {
        let mut jit = JitIntegration::new();
        jit.set_enabled(false);
        
        assert!(!jit.is_enabled());
        assert!(jit.record_call("test_func").is_none());
    }
    
    #[test]
    fn test_stats() {
        let jit = JitIntegration::with_thresholds(5, 50, 500);
        
        for _ in 0..10 {
            jit.record_call("func1");
        }
        for _ in 0..3 {
            jit.record_call("func2");
        }
        
        let stats = jit.stats();
        assert_eq!(stats.total_functions, 2);
        assert_eq!(stats.total_calls, 13);
        assert_eq!(stats.tier1_functions, 1);
        assert_eq!(stats.tier0_functions, 1);
    }
    
    #[test]
    fn test_reset() {
        let jit = JitIntegration::new();
        jit.record_call("test_func");
        
        assert_eq!(jit.get_call_count("test_func"), 1);
        
        jit.reset();
        assert_eq!(jit.get_call_count("test_func"), 0);
    }
}
