//! DX Bundle Concat - Zero-Copy Concatenation

use std::io::{self, Write};
use std::path::Path;

pub struct ZeroCopyBundler {
    output: std::fs::File,
}

impl ZeroCopyBundler {
    pub fn new(output_path: &Path) -> io::Result<Self> {
        let output = std::fs::File::create(output_path)?;
        Ok(Self { output })
    }

    pub fn write_runtime_header(&mut self) -> io::Result<()> {
        // Minimal CommonJS runtime (338 bytes unminified)
        static RUNTIME: &[u8] = br#"(function(){
var __dx_modules={};
var __dx_cache={};
function __dx_define(id,factory){__dx_modules[id]=factory;}
function __dx_require(id){
if(__dx_cache[id])return __dx_cache[id].exports;
var module=__dx_cache[id]={exports:{}};
__dx_modules[id](module.exports,__dx_require,module);
return module.exports;
}
"#;
        self.output.write_all(RUNTIME)?;
        Ok(())
    }

    pub fn write_module(&mut self, id: u32, content: &[u8]) -> io::Result<()> {
        write!(self.output, "__dx_define({},function(exports,require,module){{", id)?;
        self.output.write_all(content)?;
        self.output.write_all(b"});\n")?;
        Ok(())
    }

    pub fn write_entry(&mut self, id: u32) -> io::Result<()> {
        write!(self.output, "__dx_require({});", id)?;
        Ok(())
    }

    pub fn write_footer(&mut self) -> io::Result<()> {
        self.output.write_all(b"})();")?;
        Ok(())
    }
}
