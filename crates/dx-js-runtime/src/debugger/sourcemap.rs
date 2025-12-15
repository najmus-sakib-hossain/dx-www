//! Source map generation and parsing


#[derive(Debug, Clone)]
pub struct SourceMap {
    pub version: u32,
    pub sources: Vec<String>,
    pub mappings: String,
    pub names: Vec<String>,
}

impl SourceMap {
    pub fn new() -> Self {
        Self {
            version: 3,
            sources: Vec::new(),
            mappings: String::new(),
            names: Vec::new(),
        }
    }

    pub fn add_source(&mut self, source: String) {
        if !self.sources.contains(&source) {
            self.sources.push(source);
        }
    }

    pub fn add_mapping(&mut self, generated_line: usize, generated_column: usize,
                       source_line: usize, source_column: usize) {
        let mapping = format!("{},{},{},{}", 
            generated_line, generated_column, source_line, source_column);
        if !self.mappings.is_empty() { self.mappings.push(';'); }
        self.mappings.push_str(&mapping);
    }

    pub fn to_json(&self) -> String {
        format!(r#"{{"version":{},"sources":{:?},"mappings":"{}","names":{:?}}}"#,
            self.version, self.sources, self.mappings, self.names)
    }

    pub fn lookup(&self, generated_line: usize, generated_column: usize) -> Option<SourceLocation> {
        // Simplified lookup - would use VLQ decoding in production
        Some(SourceLocation {
            source: self.sources.first()?.clone(),
            line: generated_line,
            column: generated_column,
            name: None,
        })
    }
}

impl Default for SourceMap {
    fn default() -> Self { Self::new() }
}

#[derive(Debug, Clone)]
pub struct SourceLocation {
    pub source: String,
    pub line: usize,
    pub column: usize,
    pub name: Option<String>,
}
