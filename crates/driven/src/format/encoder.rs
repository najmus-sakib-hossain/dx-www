//! Binary encoder for .drv format

use super::{
    schema::{
        ContextSection, DrvHeader, PersonaSection, RuleEntry, StandardsSection, WorkflowSection,
    },
    SectionType, DRV_VERSION,
};
use crate::{parser::UnifiedRule, DrivenError, Result};
use bytes::{BufMut, BytesMut};
use std::collections::HashMap;

/// Encoder for creating .drv binary files
#[derive(Debug)]
pub struct DrvEncoder {
    /// String deduplication table
    string_table: Vec<String>,
    /// String to index mapping
    string_map: HashMap<String, u32>,
    /// Output buffer
    buffer: BytesMut,
}

impl DrvEncoder {
    /// Create a new encoder
    pub fn new() -> Self {
        Self {
            string_table: Vec::new(),
            string_map: HashMap::new(),
            buffer: BytesMut::with_capacity(4096),
        }
    }

    /// Intern a string and return its index
    fn intern(&mut self, s: &str) -> u32 {
        if let Some(&idx) = self.string_map.get(s) {
            return idx;
        }
        let idx = self.string_table.len() as u32;
        self.string_table.push(s.to_string());
        self.string_map.insert(s.to_string(), idx);
        idx
    }

    /// Encode rules to binary format
    pub fn encode(&self, rules: &[UnifiedRule]) -> Result<Vec<u8>> {
        let mut encoder = DrvEncoder::new();
        encoder.encode_rules(rules)
    }

    /// Internal encoding implementation
    fn encode_rules(&mut self, rules: &[UnifiedRule]) -> Result<Vec<u8>> {
        // First pass: build string table and sections
        let mut persona: Option<PersonaSection> = None;
        let mut standards = StandardsSection::default();
        let mut context = ContextSection::default();
        let mut workflow: Option<WorkflowSection> = None;

        for rule in rules {
            match rule {
                UnifiedRule::Persona {
                    name,
                    role,
                    identity,
                    style,
                    traits,
                    principles,
                } => {
                    persona = Some(PersonaSection {
                        name_idx: self.intern(name),
                        role_idx: self.intern(role),
                        identity_idx: identity.as_ref().map(|s| self.intern(s)).unwrap_or(0),
                        style_idx: style.as_ref().map(|s| self.intern(s)).unwrap_or(0),
                        traits: traits.iter().map(|t| self.intern(t)).collect(),
                        principles: principles.iter().map(|p| self.intern(p)).collect(),
                    });
                }
                UnifiedRule::Standard {
                    category,
                    priority,
                    description,
                    pattern,
                } => {
                    standards.rules.push(RuleEntry {
                        category: *category,
                        priority: *priority,
                        description_idx: self.intern(description),
                        pattern_idx: pattern.as_ref().map(|p| self.intern(p)).unwrap_or(0),
                    });
                }
                UnifiedRule::Context {
                    includes,
                    excludes,
                    focus,
                } => {
                    context.include_patterns = includes.iter().map(|s| self.intern(s)).collect();
                    context.exclude_patterns = excludes.iter().map(|s| self.intern(s)).collect();
                    context.focus_areas = focus.iter().map(|s| self.intern(s)).collect();
                }
                UnifiedRule::Workflow { name, steps } => {
                    let mut ws = WorkflowSection {
                        name_idx: self.intern(name),
                        steps: Vec::new(),
                    };
                    for step in steps {
                        ws.steps.push(super::schema::WorkflowStep {
                            name_idx: self.intern(&step.name),
                            description_idx: self.intern(&step.description),
                            condition_idx: step
                                .condition
                                .as_ref()
                                .map(|c| self.intern(c))
                                .unwrap_or(0),
                            actions: step.actions.iter().map(|a| self.intern(a)).collect(),
                        });
                    }
                    workflow = Some(ws);
                }
                UnifiedRule::Raw { content } => {
                    // Store raw content as a standard rule
                    standards.rules.push(RuleEntry {
                        category: super::schema::RuleCategory::Other,
                        priority: 100,
                        description_idx: self.intern(content),
                        pattern_idx: 0,
                    });
                }
            }
        }

        // Count sections
        let mut section_count = 1; // String table always present
        if persona.is_some() {
            section_count += 1;
        }
        if !standards.rules.is_empty() {
            section_count += 1;
        }
        if !context.include_patterns.is_empty() || !context.exclude_patterns.is_empty() {
            section_count += 1;
        }
        if workflow.is_some() {
            section_count += 1;
        }

        // Write header (will update checksum at end)
        let header = DrvHeader::new(section_count);
        self.buffer.put_slice(bytemuck::bytes_of(&header));

        // Write string table
        self.write_string_table()?;

        // Write sections
        if let Some(ref p) = persona {
            self.write_persona_section(p)?;
        }
        if !standards.rules.is_empty() {
            self.write_standards_section(&standards)?;
        }
        if !context.include_patterns.is_empty() || !context.exclude_patterns.is_empty() {
            self.write_context_section(&context)?;
        }
        if let Some(ref w) = workflow {
            self.write_workflow_section(w)?;
        }

        // Calculate and update checksum
        let checksum = self.calculate_checksum();
        let header_bytes = &mut self.buffer[..16];
        header_bytes[12..16].copy_from_slice(&checksum.to_le_bytes());

        Ok(self.buffer.to_vec())
    }

    fn write_string_table(&mut self) -> Result<()> {
        self.buffer.put_u8(SectionType::StringTable as u8);
        self.buffer.put_u32_le(self.string_table.len() as u32);

        for s in &self.string_table {
            let bytes = s.as_bytes();
            if bytes.len() > u16::MAX as usize {
                return Err(DrivenError::Format(format!(
                    "String too long: {} bytes",
                    bytes.len()
                )));
            }
            self.buffer.put_u16_le(bytes.len() as u16);
            self.buffer.put_slice(bytes);
        }

        Ok(())
    }

    fn write_persona_section(&mut self, persona: &PersonaSection) -> Result<()> {
        self.buffer.put_u8(SectionType::Persona as u8);
        self.buffer.put_u32_le(persona.name_idx);
        self.buffer.put_u32_le(persona.role_idx);
        self.buffer.put_u32_le(persona.identity_idx);
        self.buffer.put_u32_le(persona.style_idx);

        self.buffer.put_u16_le(persona.traits.len() as u16);
        for &idx in &persona.traits {
            self.buffer.put_u32_le(idx);
        }

        self.buffer.put_u16_le(persona.principles.len() as u16);
        for &idx in &persona.principles {
            self.buffer.put_u32_le(idx);
        }

        Ok(())
    }

    fn write_standards_section(&mut self, standards: &StandardsSection) -> Result<()> {
        self.buffer.put_u8(SectionType::Standards as u8);
        self.buffer.put_u32_le(standards.rules.len() as u32);

        for rule in &standards.rules {
            self.buffer.put_u8(rule.category as u8);
            self.buffer.put_u8(rule.priority);
            self.buffer.put_u32_le(rule.description_idx);
            self.buffer.put_u32_le(rule.pattern_idx);
        }

        Ok(())
    }

    fn write_context_section(&mut self, context: &ContextSection) -> Result<()> {
        self.buffer.put_u8(SectionType::Context as u8);

        self.buffer
            .put_u16_le(context.include_patterns.len() as u16);
        for &idx in &context.include_patterns {
            self.buffer.put_u32_le(idx);
        }

        self.buffer
            .put_u16_le(context.exclude_patterns.len() as u16);
        for &idx in &context.exclude_patterns {
            self.buffer.put_u32_le(idx);
        }

        self.buffer.put_u16_le(context.focus_areas.len() as u16);
        for &idx in &context.focus_areas {
            self.buffer.put_u32_le(idx);
        }

        self.buffer.put_u16_le(context.dependencies.len() as u16);
        for &(name, version) in &context.dependencies {
            self.buffer.put_u32_le(name);
            self.buffer.put_u32_le(version);
        }

        Ok(())
    }

    fn write_workflow_section(&mut self, workflow: &WorkflowSection) -> Result<()> {
        self.buffer.put_u8(SectionType::Workflow as u8);
        self.buffer.put_u32_le(workflow.name_idx);
        self.buffer.put_u16_le(workflow.steps.len() as u16);

        for step in &workflow.steps {
            self.buffer.put_u32_le(step.name_idx);
            self.buffer.put_u32_le(step.description_idx);
            self.buffer.put_u32_le(step.condition_idx);
            self.buffer.put_u16_le(step.actions.len() as u16);
            for &idx in &step.actions {
                self.buffer.put_u32_le(idx);
            }
        }

        Ok(())
    }

    fn calculate_checksum(&self) -> u32 {
        let hash = blake3::hash(&self.buffer[16..]);
        let bytes = hash.as_bytes();
        u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
    }
}

impl Default for DrvEncoder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encoder_new() {
        let encoder = DrvEncoder::new();
        assert!(encoder.string_table.is_empty());
        assert!(encoder.string_map.is_empty());
    }

    #[test]
    fn test_string_interning() {
        let mut encoder = DrvEncoder::new();
        let idx1 = encoder.intern("hello");
        let idx2 = encoder.intern("world");
        let idx3 = encoder.intern("hello"); // duplicate

        assert_eq!(idx1, 0);
        assert_eq!(idx2, 1);
        assert_eq!(idx3, 0); // Same as first
        assert_eq!(encoder.string_table.len(), 2);
    }

    #[test]
    fn test_encode_empty() {
        let encoder = DrvEncoder::new();
        let result = encoder.encode(&[]).unwrap();

        // Should have at least header + string table section
        assert!(result.len() >= 16);
        // Check magic bytes
        assert_eq!(&result[0..4], b"DRV\0");
    }
}
