//! Zero-copy output generation
//!
//! Emit final bundle with minimal allocations

use dx_bundle_core::{BundleConfig, ModuleFormat, TransformedModule};
use dx_bundle_core::error::BundleResult;
use std::io::Write;

/// Bundle emitter for zero-copy concatenation
pub struct BundleEmitter<'a> {
    config: &'a BundleConfig,
}

impl<'a> BundleEmitter<'a> {
    /// Create new emitter
    pub fn new(config: &'a BundleConfig) -> Self {
        Self { config }
    }
    
    /// Emit complete bundle
    pub fn emit(&self, modules: &[TransformedModule]) -> BundleResult<Vec<u8>> {
        // Pre-calculate total size
        let total_size = self.calculate_size(modules);
        let mut output = Vec::with_capacity(total_size);
        
        // Emit format wrapper prefix
        output.extend_from_slice(self.config.format.prefix());
        
        // Emit runtime header (if IIFE/CJS)
        if matches!(self.config.format, ModuleFormat::CJS | ModuleFormat::IIFE) {
            self.emit_runtime(&mut output)?;
        }
        
        // Emit each module
        for module in modules {
            self.emit_module(&mut output, module)?;
        }
        
        // Emit format wrapper suffix
        output.extend_from_slice(self.config.format.suffix());
        
        Ok(output)
    }
    
    /// Calculate total output size (for pre-allocation)
    fn calculate_size(&self, modules: &[TransformedModule]) -> usize {
        let mut size = 0;
        
        // Format wrappers
        size += self.config.format.prefix().len();
        size += self.config.format.suffix().len();
        
        // Runtime (if needed)
        if matches!(self.config.format, ModuleFormat::CJS | ModuleFormat::IIFE) {
            size += RUNTIME_HEADER.len();
        }
        
        // Modules
        for module in modules {
            size += 50; // Wrapper overhead
            size += module.content.len();
        }
        
        size
    }
    
    /// Emit runtime header
    fn emit_runtime(&self, output: &mut Vec<u8>) -> BundleResult<()> {
        output.extend_from_slice(RUNTIME_HEADER);
        Ok(())
    }
    
    /// Emit single module
    fn emit_module(&self, output: &mut Vec<u8>, module: &TransformedModule) -> BundleResult<()> {
        match self.config.format {
            ModuleFormat::ESM => {
                // ESM: Just emit content directly
                output.extend_from_slice(&module.content);
                output.push(b'\n');
            }
            ModuleFormat::CJS | ModuleFormat::IIFE => {
                // CJS/IIFE: Wrap in __dx_define
                output.extend_from_slice(b"__dx_define(");
                output.extend_from_slice(module.id.to_string().as_bytes());
                output.extend_from_slice(b",function(exports,require,module){\n");
                output.extend_from_slice(&module.content);
                output.extend_from_slice(b"\n});\n");
            }
            ModuleFormat::UMD => {
                // UMD: Similar to IIFE
                output.extend_from_slice(b"__dx_define(");
                output.extend_from_slice(module.id.to_string().as_bytes());
                output.extend_from_slice(b",function(exports,require,module){\n");
                output.extend_from_slice(&module.content);
                output.extend_from_slice(b"\n});\n");
            }
        }
        
        Ok(())
    }
    
    /// Emit entry point bootstrap
    pub fn emit_entry(&self, output: &mut Vec<u8>, entry_id: u64) -> BundleResult<()> {
        output.extend_from_slice(b"__dx_require(");
        output.extend_from_slice(entry_id.to_string().as_bytes());
        output.extend_from_slice(b");\n");
        Ok(())
    }
}

/// Minimal runtime for CJS/IIFE bundles
const RUNTIME_HEADER: &[u8] = b"(function(){
'use strict';
var __dx_modules={};
var __dx_cache={};
function __dx_define(id,factory){__dx_modules[id]=factory;}
function __dx_require(id){
if(__dx_cache[id])return __dx_cache[id].exports;
var module={exports:{}};
__dx_cache[id]=module;
__dx_modules[id](module.exports,__dx_require,module);
return module.exports;
}
";

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_emit_esm() {
        let config = BundleConfig {
            format: ModuleFormat::ESM,
            ..Default::default()
        };
        
        let emitter = BundleEmitter::new(&config);
        let modules = vec![
            TransformedModule {
                id: 0,
                content: b"console.log('test');".to_vec(),
                source_map: None,
                imports: vec![],
            }
        ];
        
        let result = emitter.emit(&modules).unwrap();
        assert!(result.len() > 0);
    }
    
    #[test]
    fn test_emit_cjs() {
        let config = BundleConfig {
            format: ModuleFormat::CJS,
            ..Default::default()
        };
        
        let emitter = BundleEmitter::new(&config);
        let modules = vec![
            TransformedModule {
                id: 0,
                content: b"console.log('test');".to_vec(),
                source_map: None,
                imports: vec![],
            }
        ];
        
        let result = emitter.emit(&modules).unwrap();
        assert!(result.contains(&b"__dx_define"[..]));
    }
}
