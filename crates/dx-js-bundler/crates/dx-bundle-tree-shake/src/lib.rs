//! DX Bundle Tree Shake - Binary Tree Shaking

use dx_bundle_core::*;
use std::collections::{HashMap, HashSet, VecDeque};

pub struct TreeShaker {
    used_modules: HashSet<u32>,
    _used_exports: HashMap<u32, HashSet<String>>,
}

impl Default for TreeShaker {
    fn default() -> Self {
        Self::new()
    }
}

impl TreeShaker {
    pub fn new() -> Self {
        Self {
            used_modules: HashSet::new(),
            _used_exports: HashMap::new(),
        }
    }

    /// Analyze module usage starting from entry points
    pub fn analyze(&mut self, entries: &[u32], module_count: usize) -> Vec<ShakeModule> {
        let mut results = vec![
            ShakeModule {
                module: 0,
                used_exports: 0,
                all_used: 0,
                has_side_effects: 1, // Conservative: assume side effects
                can_remove: 0,
                _padding: 0,
            };
            module_count
        ];

        // Mark entry modules as fully used
        let mut queue = VecDeque::new();
        for &entry in entries {
            self.used_modules.insert(entry);
            queue.push_back(entry);

            if (entry as usize) < module_count {
                results[entry as usize].all_used = 1;
                results[entry as usize].can_remove = 0;
            }
        }

        // BFS to mark all reachable modules
        while let Some(module_idx) = queue.pop_front() {
            if (module_idx as usize) >= module_count {
                continue;
            }

            // Mark this module as used
            self.used_modules.insert(module_idx);
            results[module_idx as usize].can_remove = 0;

            // In a full implementation, we would traverse imports here
            // For now, conservatively mark everything as used
        }

        // Mark unused modules
        for (idx, result) in results.iter_mut().enumerate() {
            result.module = idx as u32;
            if !self.used_modules.contains(&(idx as u32)) {
                result.can_remove = 1;
                result.all_used = 0;
            }
        }

        results
    }

    /// Check if a module can be removed
    pub fn can_remove_module(&self, module_idx: u32) -> bool {
        !self.used_modules.contains(&module_idx)
    }
}
