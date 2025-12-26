//! Property-based tests for DPM module format

use proptest::prelude::*;
use dx_py_modules::{
    ExportTable, DpmCompiler, DpmLoader,
    format::{ExportKind, DpmHeader},
    compiler::{ModuleDefinition, ExportDef, ImportDef},
};

/// Property 14: Perfect Hash Export Lookup
/// Verifies O(1) lookup for all symbols
mod perfect_hash_tests {
    use super::*;
    
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]
        
        /// All inserted symbols must be retrievable
        #[test]
        fn prop_all_symbols_retrievable(
            symbols in prop::collection::vec("[a-z][a-z0-9_]{0,20}", 1..50)
        ) {
            // Deduplicate symbols
            let unique: Vec<_> = symbols.into_iter()
                .collect::<std::collections::HashSet<_>>()
                .into_iter()
                .collect();
            
            let exports: Vec<_> = unique.iter()
                .enumerate()
                .map(|(i, name)| (name.clone(), ExportKind::Function, i as u32))
                .collect();
            
            let table = ExportTable::build(&exports).unwrap();
            
            // All symbols must be found
            for (name, _, offset) in &exports {
                let entry = table.get(name);
                prop_assert!(entry.is_some(), "Symbol {} not found", name);
                prop_assert_eq!(entry.unwrap().value_offset, *offset);
            }
        }
        
        /// Non-existent symbols must not be found
        #[test]
        fn prop_nonexistent_not_found(
            symbols in prop::collection::vec("[a-z][a-z0-9_]{0,10}", 1..20),
            queries in prop::collection::vec("[A-Z][A-Z0-9_]{0,10}", 1..20)
        ) {
            let exports: Vec<_> = symbols.iter()
                .enumerate()
                .map(|(i, name)| (name.clone(), ExportKind::Function, i as u32))
                .collect();
            
            let table = ExportTable::build(&exports).unwrap();
            
            // Uppercase queries should not match lowercase symbols
            for query in &queries {
                let entry = table.get(query);
                // Should only find if there's an exact match (unlikely with different cases)
                if entry.is_some() {
                    prop_assert!(symbols.contains(query));
                }
            }
        }
        
        /// Lookup is deterministic
        #[test]
        fn prop_lookup_deterministic(
            symbols in prop::collection::vec("[a-z][a-z0-9_]{0,15}", 1..30)
        ) {
            let unique: Vec<_> = symbols.into_iter()
                .collect::<std::collections::HashSet<_>>()
                .into_iter()
                .collect();
            
            let exports: Vec<_> = unique.iter()
                .enumerate()
                .map(|(i, name)| (name.clone(), ExportKind::Variable, i as u32))
                .collect();
            
            let table = ExportTable::build(&exports).unwrap();
            
            // Multiple lookups return same result
            for name in &unique {
                let first = table.get(name).map(|e| e.value_offset);
                let second = table.get(name).map(|e| e.value_offset);
                let third = table.get(name).map(|e| e.value_offset);
                prop_assert_eq!(first, second);
                prop_assert_eq!(second, third);
            }
        }
    }
}

/// Property 2: DPM Module Round-Trip Consistency
mod roundtrip_tests {
    use super::*;
    use tempfile::NamedTempFile;
    use std::io::Write;
    
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(50))]
        
        /// Compiled module can be loaded and exports match
        #[test]
        fn prop_compile_load_roundtrip(
            module_name in "[a-z][a-z0-9_]{0,20}",
            export_names in prop::collection::vec("[a-z][a-z0-9_]{0,15}", 1..10),
            is_package in any::<bool>()
        ) {
            let unique_exports: Vec<_> = export_names.into_iter()
                .collect::<std::collections::HashSet<_>>()
                .into_iter()
                .collect();
            
            let module = ModuleDefinition {
                name: module_name,
                is_package,
                imports: vec![],
                exports: unique_exports.iter().enumerate().map(|(i, name)| {
                    ExportDef {
                        name: name.clone(),
                        kind: ExportKind::Function,
                        data: vec![i as u8; 4],
                    }
                }).collect(),
                init_bytecode: vec![0xF0], // NOP
                type_annotations: vec![],
            };
            
            let mut compiler = DpmCompiler::new();
            let binary = compiler.compile(&module).unwrap();
            
            // Write to temp file
            let mut temp = NamedTempFile::new().unwrap();
            temp.write_all(&binary).unwrap();
            temp.flush().unwrap();
            
            // Load and verify
            let loader = DpmLoader::new();
            let loaded = loader.load(temp.path()).unwrap();
            
            // All exports should be found
            for name in &unique_exports {
                let entry = loaded.get_symbol(name);
                prop_assert!(entry.is_some(), "Export {} not found after roundtrip", name);
            }
        }
        
        /// Header fields are preserved
        #[test]
        fn prop_header_preserved(
            is_package in any::<bool>(),
            num_imports in 0usize..5,
            num_exports in 1usize..10
        ) {
            let module = ModuleDefinition {
                name: "test".to_string(),
                is_package,
                imports: (0..num_imports).map(|i| ImportDef {
                    module_name: format!("mod{}", i),
                    symbol_name: None,
                    alias: None,
                    is_star: false,
                    level: 0,
                }).collect(),
                exports: (0..num_exports).map(|i| ExportDef {
                    name: format!("export{}", i),
                    kind: ExportKind::Function,
                    data: vec![],
                }).collect(),
                init_bytecode: vec![],
                type_annotations: vec![],
            };
            
            let mut compiler = DpmCompiler::new();
            let binary = compiler.compile(&module).unwrap();
            
            let mut temp = NamedTempFile::new().unwrap();
            temp.write_all(&binary).unwrap();
            temp.flush().unwrap();
            
            let loader = DpmLoader::new();
            let loaded = loader.load(temp.path()).unwrap();
            let header = loaded.header();
            
            prop_assert_eq!(header.imports_count as usize, num_imports);
            prop_assert_eq!(header.exports_count as usize, num_exports);
        }
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;
    
    #[test]
    fn test_header_size_and_alignment() {
        assert_eq!(std::mem::align_of::<DpmHeader>(), 64);
        // Header should be at least 64 bytes for cache line alignment
        assert!(DpmHeader::size() >= 64);
    }
    
    #[test]
    fn test_empty_export_table() {
        let table = ExportTable::build(&[]).unwrap();
        assert!(table.is_empty());
        assert!(table.get("anything").is_none());
    }
    
    #[test]
    fn test_single_export() {
        let exports = vec![("single".to_string(), ExportKind::Function, 42)];
        let table = ExportTable::build(&exports).unwrap();
        
        let entry = table.get("single").unwrap();
        assert_eq!(entry.value_offset, 42);
        assert_eq!(entry.kind, ExportKind::Function);
    }
}
