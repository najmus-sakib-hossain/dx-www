//! Property-based tests for Tiered JIT
//!
//! Property 18: JIT Tier Promotion Threshold
//! Validates: Requirements 4.2, 4.3, 4.4

use dx_py_jit::*;
use dx_py_jit::compiler::FunctionId;
use dx_py_jit::profile::PyType;
use proptest::prelude::*;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]
    
    /// Property 18: JIT Tier Promotion Threshold
    /// Validates: Requirements 4.2, 4.3, 4.4
    ///
    /// Functions should be promoted to:
    /// - Tier 1 (Baseline JIT) at 100 calls
    /// - Tier 2 (Optimizing JIT) at 1000 calls
    /// - Tier 3 (AOT Optimized) at 10000 calls
    #[test]
    fn prop_tier_promotion_thresholds(calls in 0u64..15000) {
        let jit = TieredJit::new();
        let func_id = FunctionId(1);
        
        let profile = jit.get_profile(func_id, 100, 5);
        
        // Record the specified number of calls
        for _ in 0..calls {
            profile.record_call();
        }
        
        let promotion = jit.check_promotion(func_id);
        
        if calls < 100 {
            // Should not be promoted yet
            prop_assert!(
                promotion.is_none(),
                "Should not promote at {} calls (< 100)",
                calls
            );
        } else if calls < 1000 {
            // Should be promoted to Baseline JIT
            prop_assert_eq!(
                promotion,
                Some(CompilationTier::BaselineJit),
                "Should promote to BaselineJit at {} calls",
                calls
            );
        }
        // Note: Higher tiers require the previous tier to be compiled first
    }
    
    /// Property: Type feedback correctly tracks monomorphic sites
    #[test]
    fn prop_type_feedback_monomorphic(type_idx in 0u8..15) {
        let feedback = TypeFeedback::new();
        let py_type = PyType::from_u8(type_idx);
        
        // Record single type multiple times
        for _ in 0..10 {
            feedback.record(py_type);
        }
        
        prop_assert!(feedback.is_monomorphic());
        prop_assert_eq!(feedback.get_types().len(), 1);
        prop_assert_eq!(feedback.get_primary_type(), Some(py_type));
    }
    
    /// Property: Type feedback correctly tracks polymorphic sites
    #[test]
    fn prop_type_feedback_polymorphic(
        type1 in 1u8..15,
        type2 in 1u8..15,
        type3 in 1u8..15
    ) {
        prop_assume!(type1 != type2 && type2 != type3 && type1 != type3);
        
        let feedback = TypeFeedback::new();
        
        feedback.record(PyType::from_u8(type1));
        feedback.record(PyType::from_u8(type2));
        feedback.record(PyType::from_u8(type3));
        
        prop_assert!(feedback.is_polymorphic());
        prop_assert_eq!(feedback.get_types().len(), 3);
    }
    
    /// Property: Deoptimization count prevents promotion
    #[test]
    fn prop_deopt_prevents_promotion(
        calls in 100u64..1000,
        deopts in 11u32..100
    ) {
        let jit = TieredJit::with_settings(true, 10);
        let func_id = FunctionId(1);
        
        let profile = jit.get_profile(func_id, 100, 5);
        
        // Record calls
        for _ in 0..calls {
            profile.record_call();
        }
        
        // Record too many deopts
        for _ in 0..deopts {
            profile.record_deopt();
        }
        
        // Should not be promoted due to deopts
        prop_assert!(
            jit.check_promotion(func_id).is_none(),
            "Should not promote with {} deopts (> 10)",
            deopts
        );
    }
    
    /// Property: Branch probability is always in [0, 1]
    #[test]
    fn prop_branch_probability_range(
        taken in 0u64..10000,
        not_taken in 0u64..10000
    ) {
        let profile = FunctionProfile::new(10, 1);
        
        for _ in 0..taken {
            profile.record_branch_taken(0);
        }
        for _ in 0..not_taken {
            profile.record_branch_not_taken(0);
        }
        
        if let Some(prob) = profile.get_branch_probability(0) {
            prop_assert!(prob >= 0.0 && prob <= 1.0);
            
            if taken + not_taken > 0 {
                let expected = taken as f64 / (taken + not_taken) as f64;
                prop_assert!((prob - expected).abs() < 0.001);
            }
        }
    }
    
    /// Property: Tier ordering is consistent
    #[test]
    fn prop_tier_ordering(_seed in any::<u64>()) {
        prop_assert!(CompilationTier::Interpreter < CompilationTier::BaselineJit);
        prop_assert!(CompilationTier::BaselineJit < CompilationTier::OptimizingJit);
        prop_assert!(CompilationTier::OptimizingJit < CompilationTier::AotOptimized);
        
        // Thresholds should be increasing
        prop_assert!(CompilationTier::Interpreter.threshold() < CompilationTier::BaselineJit.threshold());
        prop_assert!(CompilationTier::BaselineJit.threshold() < CompilationTier::OptimizingJit.threshold());
        prop_assert!(CompilationTier::OptimizingJit.threshold() < CompilationTier::AotOptimized.threshold());
    }
    
    /// Property: Tier progression is correct
    #[test]
    fn prop_tier_progression(_seed in any::<u64>()) {
        let mut tier = CompilationTier::Interpreter;
        let mut count = 0;
        
        while let Some(next) = tier.next() {
            prop_assert!(next > tier);
            tier = next;
            count += 1;
        }
        
        prop_assert_eq!(count, 3); // 3 promotions possible
        prop_assert_eq!(tier, CompilationTier::AotOptimized);
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;
    use dx_py_jit::osr::{OsrManager, FrameSnapshot};
    
    #[test]
    fn test_osr_hot_detection() {
        let manager = OsrManager::with_threshold(1000);
        
        assert!(!manager.is_hot(999));
        assert!(manager.is_hot(1000));
        assert!(manager.is_hot(10000));
    }
    
    #[test]
    fn test_profile_concurrent_access() {
        use std::sync::Arc;
        use std::thread;
        
        let jit = Arc::new(TieredJit::new());
        let func_id = FunctionId(1);
        
        let profile = jit.get_profile(func_id, 100, 5);
        let profile = Arc::new(profile);
        
        let mut handles = vec![];
        
        for _ in 0..4 {
            let profile_clone = Arc::clone(&profile);
            handles.push(thread::spawn(move || {
                for _ in 0..1000 {
                    profile_clone.record_call();
                }
            }));
        }
        
        for handle in handles {
            handle.join().unwrap();
        }
        
        assert_eq!(profile.get_call_count(), 4000);
    }
    
    #[test]
    fn test_type_feedback_deduplication() {
        let feedback = TypeFeedback::new();
        
        // Record same type multiple times
        for _ in 0..100 {
            feedback.record(PyType::Int);
        }
        
        // Should only have one type
        assert_eq!(feedback.get_types().len(), 1);
        assert!(feedback.is_monomorphic());
    }
}
