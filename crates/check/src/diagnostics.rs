//! Binary Diagnostics
//!
//! Compact binary format for diagnostics - 33 bytes vs 300-500 bytes JSON.
//! Enables real-time 60fps linting updates.

use bytemuck::{Pod, Zeroable};
use std::fmt;
use std::path::PathBuf;

/// Diagnostic severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum DiagnosticSeverity {
    /// Hint - informational suggestion
    Hint = 0,
    /// Info - informational message
    Info = 1,
    /// Warning - potential issue
    Warning = 2,
    /// Error - definite problem
    Error = 3,
}

impl DiagnosticSeverity {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Hint => "hint",
            Self::Info => "info",
            Self::Warning => "warning",
            Self::Error => "error",
        }
    }

    pub fn symbol(&self) -> &'static str {
        match self {
            Self::Hint => "💡",
            Self::Info => "ℹ️",
            Self::Warning => "⚠️",
            Self::Error => "❌",
        }
    }
}

/// Source span (byte offsets)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Pod, Zeroable)]
#[repr(C)]
pub struct Span {
    /// Start byte offset
    pub start: u32,
    /// End byte offset
    pub end: u32,
}

impl Span {
    pub fn new(start: u32, end: u32) -> Self {
        Self { start, end }
    }

    pub fn len(&self) -> u32 {
        self.end - self.start
    }

    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }

    /// Convert to line/column using line index
    pub fn to_line_col(&self, line_index: &LineIndex) -> (LineCol, LineCol) {
        (
            line_index.line_col(self.start),
            line_index.line_col(self.end),
        )
    }
}

impl From<oxc_span::Span> for Span {
    fn from(span: oxc_span::Span) -> Self {
        Self {
            start: span.start,
            end: span.end,
        }
    }
}

/// Line and column (1-indexed for display)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LineCol {
    pub line: u32,
    pub col: u32,
}

/// Index for fast byte offset to line/column conversion
pub struct LineIndex {
    /// Byte offsets of line starts
    line_starts: Vec<u32>,
}

impl LineIndex {
    pub fn new(source: &str) -> Self {
        let mut line_starts = vec![0];
        for (i, c) in source.char_indices() {
            if c == '\n' {
                line_starts.push((i + 1) as u32);
            }
        }
        Self { line_starts }
    }

    pub fn line_col(&self, offset: u32) -> LineCol {
        let line = self
            .line_starts
            .partition_point(|&start| start <= offset)
            .saturating_sub(1);
        let line_start = self.line_starts[line];
        LineCol {
            line: (line + 1) as u32,      // 1-indexed
            col: (offset - line_start) + 1, // 1-indexed
        }
    }
}

/// Binary diagnostic format (33 bytes - compact for network transfer)
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
#[repr(C, packed)]
pub struct BinaryDiagnostic {
    /// File ID (index into file table)
    pub file_id: u32,
    /// Start byte offset
    pub start_byte: u32,
    /// End byte offset
    pub end_byte: u32,
    /// Severity (0=hint, 1=info, 2=warn, 3=error)
    pub severity: u8,
    /// Rule ID (index into rule table)
    pub rule_id: u16,
    /// Message template ID
    pub message_id: u16,
    /// Template parameters (for parameterized messages)
    pub captures: [u32; 4],
}

impl BinaryDiagnostic {
    pub fn span(&self) -> Span {
        Span {
            start: self.start_byte,
            end: self.end_byte,
        }
    }

    pub fn severity(&self) -> DiagnosticSeverity {
        match self.severity {
            0 => DiagnosticSeverity::Hint,
            1 => DiagnosticSeverity::Info,
            2 => DiagnosticSeverity::Warning,
            _ => DiagnosticSeverity::Error,
        }
    }
}

/// Full diagnostic with all context
#[derive(Debug, Clone)]
pub struct Diagnostic {
    /// Source file path
    pub file: PathBuf,
    /// Span in source
    pub span: Span,
    /// Severity level
    pub severity: DiagnosticSeverity,
    /// Rule that produced this diagnostic
    pub rule_id: String,
    /// Human-readable message
    pub message: String,
    /// Optional suggestion for fixing
    pub suggestion: Option<String>,
    /// Related information
    pub related: Vec<RelatedInfo>,
    /// Quick fix if available
    pub fix: Option<Fix>,
}

impl Diagnostic {
    /// Create a new error diagnostic
    pub fn error(file: PathBuf, span: Span, rule_id: &str, message: impl Into<String>) -> Self {
        Self {
            file,
            span,
            severity: DiagnosticSeverity::Error,
            rule_id: rule_id.to_string(),
            message: message.into(),
            suggestion: None,
            related: Vec::new(),
            fix: None,
        }
    }

    /// Create a new warning diagnostic
    pub fn warn(file: PathBuf, span: Span, rule_id: &str, message: impl Into<String>) -> Self {
        Self {
            file,
            span,
            severity: DiagnosticSeverity::Warning,
            rule_id: rule_id.to_string(),
            message: message.into(),
            suggestion: None,
            related: Vec::new(),
            fix: None,
        }
    }

    /// Add a suggestion
    pub fn with_suggestion(mut self, suggestion: impl Into<String>) -> Self {
        self.suggestion = Some(suggestion.into());
        self
    }

    /// Add a fix
    pub fn with_fix(mut self, fix: Fix) -> Self {
        self.fix = Some(fix);
        self
    }

    /// Convert to binary format for network transfer
    pub fn to_binary(&self, file_id: u32, rule_table: &RuleTable) -> BinaryDiagnostic {
        BinaryDiagnostic {
            file_id,
            start_byte: self.span.start,
            end_byte: self.span.end,
            severity: self.severity as u8,
            rule_id: rule_table.get_id(&self.rule_id).unwrap_or(0),
            message_id: 0, // TODO: implement message templates
            captures: [0; 4],
        }
    }

    /// Format for terminal output
    pub fn format(&self, source: &str) -> String {
        let line_index = LineIndex::new(source);
        let (start_lc, _end_lc) = self.span.to_line_col(&line_index);

        format!(
            "{} {}[{}]: {}\n  --> {}:{}:{}\n",
            self.severity.symbol(),
            self.severity.as_str(),
            self.rule_id,
            self.message,
            self.file.display(),
            start_lc.line,
            start_lc.col,
        )
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}[{}]: {} at {:?}",
            self.severity.as_str(),
            self.rule_id,
            self.message,
            self.file
        )
    }
}

/// Related diagnostic information
#[derive(Debug, Clone)]
pub struct RelatedInfo {
    pub file: PathBuf,
    pub span: Span,
    pub message: String,
}

/// A code fix/edit
#[derive(Debug, Clone)]
pub struct Fix {
    /// Description of the fix
    pub description: String,
    /// Edits to apply
    pub edits: Vec<Edit>,
}

/// A single text edit
#[derive(Debug, Clone)]
pub struct Edit {
    /// Span to replace
    pub span: Span,
    /// New text
    pub new_text: String,
}

impl Fix {
    /// Create a fix that replaces the span with new text
    pub fn replace(description: impl Into<String>, span: Span, new_text: impl Into<String>) -> Self {
        Self {
            description: description.into(),
            edits: vec![Edit {
                span,
                new_text: new_text.into(),
            }],
        }
    }

    /// Create a fix that deletes the span
    pub fn delete(description: impl Into<String>, span: Span) -> Self {
        Self::replace(description, span, "")
    }

    /// Create a fix that inserts text at a position
    pub fn insert(description: impl Into<String>, position: u32, text: impl Into<String>) -> Self {
        Self::replace(description, Span::new(position, position), text)
    }
}

/// Rule ID to index mapping for binary format
pub struct RuleTable {
    rules: Vec<String>,
}

impl RuleTable {
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    pub fn register(&mut self, rule_id: &str) -> u16 {
        if let Some(idx) = self.rules.iter().position(|r| r == rule_id) {
            idx as u16
        } else {
            let idx = self.rules.len() as u16;
            self.rules.push(rule_id.to_string());
            idx
        }
    }

    pub fn get_id(&self, rule_id: &str) -> Option<u16> {
        self.rules.iter().position(|r| r == rule_id).map(|i| i as u16)
    }

    pub fn get_name(&self, id: u16) -> Option<&str> {
        self.rules.get(id as usize).map(|s| s.as_str())
    }
}

impl Default for RuleTable {
    fn default() -> Self {
        Self::new()
    }
}

/// Differential diagnostic update for incremental updates
#[derive(Debug, Clone)]
pub struct DiagnosticPatch {
    /// Indices of removed diagnostics
    pub removed: Vec<u32>,
    /// New diagnostics
    pub added: Vec<BinaryDiagnostic>,
}

impl DiagnosticPatch {
    /// Compute minimal patch between old and new diagnostics
    pub fn compute(old: &[BinaryDiagnostic], new: &[BinaryDiagnostic]) -> Self {
        use std::collections::HashSet;

        let old_set: HashSet<_> = old.iter().map(|d| Self::hash_diagnostic(d)).collect();
        let new_set: HashSet<_> = new.iter().map(|d| Self::hash_diagnostic(d)).collect();

        Self {
            removed: old
                .iter()
                .enumerate()
                .filter(|(_, d)| !new_set.contains(&Self::hash_diagnostic(d)))
                .map(|(i, _)| i as u32)
                .collect(),
            added: new
                .iter()
                .filter(|d| !old_set.contains(&Self::hash_diagnostic(d)))
                .copied()
                .collect(),
        }
    }

    fn hash_diagnostic(d: &BinaryDiagnostic) -> u64 {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        // Copy fields to avoid unaligned reference issues with packed struct
        let file_id = d.file_id;
        let start_byte = d.start_byte;
        let end_byte = d.end_byte;
        let rule_id = d.rule_id;
        file_id.hash(&mut hasher);
        start_byte.hash(&mut hasher);
        end_byte.hash(&mut hasher);
        rule_id.hash(&mut hasher);
        hasher.finish()
    }

    /// Serialize to bytes (typically 10-100 bytes vs full array)
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend(&(self.removed.len() as u16).to_le_bytes());
        for idx in &self.removed {
            buf.extend(&idx.to_le_bytes());
        }
        buf.extend(&(self.added.len() as u16).to_le_bytes());
        for diag in &self.added {
            buf.extend(bytemuck::bytes_of(diag));
        }
        buf
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_span() {
        let span = Span::new(10, 20);
        assert_eq!(span.len(), 10);
        assert!(!span.is_empty());
    }

    #[test]
    fn test_line_index() {
        let source = "line1\nline2\nline3";
        let index = LineIndex::new(source);

        assert_eq!(index.line_col(0), LineCol { line: 1, col: 1 });
        assert_eq!(index.line_col(6), LineCol { line: 2, col: 1 });
        assert_eq!(index.line_col(12), LineCol { line: 3, col: 1 });
    }

    #[test]
    fn test_binary_diagnostic_size() {
        assert_eq!(std::mem::size_of::<BinaryDiagnostic>(), 33);
    }
}
