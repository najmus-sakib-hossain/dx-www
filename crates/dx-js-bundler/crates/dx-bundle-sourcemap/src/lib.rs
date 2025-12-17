//! DX Bundle Sourcemap - Binary Source Maps

use dx_bundle_core::*;

pub struct BinarySourceMapBuilder {
    sources: Vec<String>,
    names: Vec<String>,
    mappings: Vec<BinaryMapping>,
}

impl Default for BinarySourceMapBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl BinarySourceMapBuilder {
    pub fn new() -> Self {
        Self {
            sources: Vec::new(),
            names: Vec::new(),
            mappings: Vec::new(),
        }
    }

    pub fn add_mapping(
        &mut self,
        gen_line: u32,
        gen_column: u32,
        source: &str,
        orig_line: u32,
        orig_column: u32,
        name: Option<&str>,
    ) {
        let source_idx = self.get_or_add_source(source);
        let name_idx = name.map(|n| self.get_or_add_name(n)).unwrap_or(u16::MAX);

        self.mappings.push(BinaryMapping {
            gen_line,
            gen_column,
            source: source_idx,
            orig_line,
            orig_column,
            name: name_idx,
        });
    }

    pub fn build(self) -> Vec<u8> {
        let mut output = Vec::new();

        let header = SourceMapHeader {
            magic: magic::SOURCE_MAP,
            version: 1,
            source_count: self.sources.len() as u32,
            name_count: self.names.len() as u32,
            mapping_count: self.mappings.len() as u32,
            sources_offset: 0,
            names_offset: 0,
            mappings_offset: 0,
            strings_offset: 0,
        };
        output.extend_from_slice(bytemuck::bytes_of(&header));

        // TODO: Write sources, names, mappings, strings

        output
    }

    fn get_or_add_source(&mut self, source: &str) -> u16 {
        if let Some(idx) = self.sources.iter().position(|s| s == source) {
            return idx as u16;
        }
        let idx = self.sources.len() as u16;
        self.sources.push(source.to_string());
        idx
    }

    fn get_or_add_name(&mut self, name: &str) -> u16 {
        if let Some(idx) = self.names.iter().position(|n| n == name) {
            return idx as u16;
        }
        let idx = self.names.len() as u16;
        self.names.push(name.to_string());
        idx
    }
}
