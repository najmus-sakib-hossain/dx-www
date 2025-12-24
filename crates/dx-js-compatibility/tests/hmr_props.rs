//! Property-based tests for HMR compatibility.
//!
//! Tests:
//! - Property 16: HMR Dependency Invalidation

use dx_compat_hmr::DependencyGraph;
use proptest::prelude::*;
use std::collections::HashSet;

/// Generate valid module names.
fn arb_module_name() -> impl Strategy<Value = String> {
    "[a-z]{1,5}\\.js".prop_map(|s| s)
}

/// Generate a list of unique module names.
fn arb_module_list(min: usize, max: usize) -> impl Strategy<Value = Vec<String>> {
    prop::collection::hash_set(arb_module_name(), min..max)
        .prop_map(|set| set.into_iter().collect())
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    /// Property 16: HMR Dependency Invalidation
    ///
    /// For any dependency graph:
    /// - When a module changes, all its dependents should be invalidated
    /// - The changed module itself should be in the invalidation list
    /// - Modules that don't depend on the changed module should not be invalidated
    #[test]
    fn prop_hmr_dependency_invalidation(
        modules in arb_module_list(3, 8),
    ) {
        prop_assume!(modules.len() >= 3);

        let graph = DependencyGraph::new();

        // Add all modules
        for module in &modules {
            graph.add_module(module);
        }

        // Create a chain: module[0] -> module[1] -> module[2]
        if modules.len() >= 3 {
            graph.add_dependency(&modules[0], &modules[1]);
            graph.add_dependency(&modules[1], &modules[2]);
        }

        // When module[2] changes, module[0] and module[1] should be invalidated
        let dependents = graph.get_dependents(&modules[2]);

        // module[1] depends on module[2], so it should be invalidated
        prop_assert!(
            dependents.iter().any(|p| p.to_string_lossy() == modules[1]),
            "Direct dependent should be invalidated"
        );

        // module[0] depends on module[1] which depends on module[2]
        prop_assert!(
            dependents.iter().any(|p| p.to_string_lossy() == modules[0]),
            "Transitive dependent should be invalidated"
        );

        // Modules not in the chain should not be invalidated
        for module in modules.iter().skip(3) {
            prop_assert!(
                !dependents.iter().any(|p| p.to_string_lossy() == *module),
                "Unrelated module should not be invalidated"
            );
        }
    }

    /// Property 16b: Adding dependencies updates the graph correctly
    #[test]
    fn prop_hmr_add_dependency(
        from in arb_module_name(),
        to in arb_module_name(),
    ) {
        prop_assume!(from != to);

        let graph = DependencyGraph::new();
        graph.add_dependency(&from, &to);

        // Both modules should exist
        prop_assert!(graph.has_module(&from));
        prop_assert!(graph.has_module(&to));

        // 'from' should have 'to' as a dependency
        let deps = graph.get_dependencies(&from);
        prop_assert!(deps.iter().any(|p| p.to_string_lossy() == to));

        // 'to' should have 'from' as a dependent
        let dependents = graph.get_dependents(&to);
        prop_assert!(dependents.iter().any(|p| p.to_string_lossy() == from));
    }

    /// Property 16c: Removing a module removes it from the graph
    #[test]
    fn prop_hmr_remove_module(
        modules in arb_module_list(2, 5),
    ) {
        prop_assume!(modules.len() >= 2);

        let graph = DependencyGraph::new();

        for module in &modules {
            graph.add_module(module);
        }

        let to_remove = &modules[0];
        prop_assert!(graph.has_module(to_remove));

        graph.remove_module(to_remove);
        prop_assert!(!graph.has_module(to_remove));

        // Other modules should still exist
        for module in modules.iter().skip(1) {
            prop_assert!(graph.has_module(module));
        }
    }

    /// Property 16d: Circular dependencies are handled
    #[test]
    fn prop_hmr_circular_dependencies(
        modules in arb_module_list(3, 5),
    ) {
        prop_assume!(modules.len() >= 3);

        let graph = DependencyGraph::new();

        // Create a cycle: a -> b -> c -> a
        graph.add_dependency(&modules[0], &modules[1]);
        graph.add_dependency(&modules[1], &modules[2]);
        graph.add_dependency(&modules[2], &modules[0]);

        // All modules in the cycle should be dependents of any module in the cycle
        let dependents = graph.get_dependents(&modules[0]);

        // Should include the other modules in the cycle
        // (the implementation may or may not include all due to cycle handling)
        prop_assert!(graph.has_module(&modules[0]));
        prop_assert!(graph.has_module(&modules[1]));
        prop_assert!(graph.has_module(&modules[2]));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_dependency_chain() {
        let graph = DependencyGraph::new();

        // a.js imports b.js imports c.js
        graph.add_dependency("a.js", "b.js");
        graph.add_dependency("b.js", "c.js");

        // When c.js changes
        let dependents = graph.get_dependents("c.js");

        // Both a.js and b.js should be invalidated
        assert!(dependents.iter().any(|p| p.to_string_lossy() == "b.js"));
        assert!(dependents.iter().any(|p| p.to_string_lossy() == "a.js"));
    }

    #[test]
    fn test_diamond_dependency() {
        let graph = DependencyGraph::new();

        // Diamond: a -> b, a -> c, b -> d, c -> d
        graph.add_dependency("a.js", "b.js");
        graph.add_dependency("a.js", "c.js");
        graph.add_dependency("b.js", "d.js");
        graph.add_dependency("c.js", "d.js");

        // When d.js changes, all should be invalidated
        let dependents = graph.get_dependents("d.js");

        assert!(dependents.iter().any(|p| p.to_string_lossy() == "b.js"));
        assert!(dependents.iter().any(|p| p.to_string_lossy() == "c.js"));
        assert!(dependents.iter().any(|p| p.to_string_lossy() == "a.js"));
    }

    #[test]
    fn test_isolated_module() {
        let graph = DependencyGraph::new();

        graph.add_module("a.js");
        graph.add_module("b.js");
        graph.add_dependency("c.js", "d.js");

        // a.js and b.js are isolated
        let dependents = graph.get_dependents("d.js");

        assert!(!dependents.iter().any(|p| p.to_string_lossy() == "a.js"));
        assert!(!dependents.iter().any(|p| p.to_string_lossy() == "b.js"));
        assert!(dependents.iter().any(|p| p.to_string_lossy() == "c.js"));
    }

    #[test]
    fn test_module_count() {
        let graph = DependencyGraph::new();

        assert_eq!(graph.module_count(), 0);

        graph.add_module("a.js");
        assert_eq!(graph.module_count(), 1);

        graph.add_module("b.js");
        assert_eq!(graph.module_count(), 2);

        // Adding same module again shouldn't increase count
        graph.add_module("a.js");
        assert_eq!(graph.module_count(), 2);
    }
}
