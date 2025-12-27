//! # Splitter Module - The Holographic Engine
//!
//! The secret sauce. Separates the "Bone" from the "Muscle."
//!
//! ## Algorithm
//! Scan JSX: `<div class="box">Count: {state.count}</div>`
//! - **Extraction 1 (Template):** `<div class="box">Count: <!--SLOT_0--></div>` -> Saved to `template_map`
//! - **Extraction 2 (Binding):** `SLOT_0` maps to `state.count`
//!
//! ## Output
//! - `templates`: A list of unique DOM structures
//! - `bindings`: A mapping of Slot IDs to Rust expressions

use anyhow::{anyhow, Result};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::parser::{Component, ParsedModule};

// Re-export shared types from dx-www-packet
pub use dx_www_packet::{SlotDef, SlotType, Template};

/// Binding from Slot to Rust expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Binding {
    pub slot_id: u32,
    pub component: String,
    pub expression: String, // Rust expression (e.g., "self.count")
    pub dirty_bit: u8,      // Which bit in dirty_mask
}

/// State schema for a component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateSchema {
    pub component: String,
    pub fields: Vec<StateField>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateField {
    pub name: String,
    pub type_name: String,
    pub initial_value: String,
    pub dirty_bit: u8,
}

/// Split components into templates and bindings
pub fn split_components(
    modules: Vec<ParsedModule>,
    verbose: bool,
) -> Result<(Vec<Template>, Vec<Binding>, Vec<StateSchema>)> {
    if verbose {
        println!("  Splitting components...");
    }

    let mut templates = Vec::new();
    let mut bindings = Vec::new();
    let mut schemas = Vec::new();
    let mut template_dedup: HashMap<String, u32> = HashMap::new();
    let mut next_template_id = 0u32;
    let mut next_slot_id = 0u32;

    for module in &modules {
        for component in &module.components {
            if verbose {
                println!("    Processing component: {}", component.name);
            }

            // Extract state schema
            let schema = extract_state_schema(component)?;
            schemas.push(schema);

            // Parse JSX body and split
            let (template, component_bindings) = split_jsx(
                &component.jsx_body,
                &component.name,
                &mut next_template_id,
                &mut next_slot_id,
                &mut template_dedup,
            )?;

            if let Some(template) = template {
                templates.push(template);
            }
            bindings.extend(component_bindings);
        }
    }

    if verbose {
        println!("  Extracted {} templates, {} bindings", templates.len(), bindings.len());
    }

    Ok((templates, bindings, schemas))
}

/// Extract state schema from component
fn extract_state_schema(component: &Component) -> Result<StateSchema> {
    let mut fields = Vec::new();
    let mut dirty_bit = 0u8;

    for state_def in &component.state {
        fields.push(StateField {
            name: state_def.name.clone(),
            type_name: state_def.type_annotation.clone(),
            initial_value: state_def.initial_value.clone(),
            dirty_bit,
        });

        dirty_bit += 1;
        if dirty_bit >= 64 {
            return Err(anyhow!(
                "Component {} has more than 64 state fields (dirty_mask overflow)",
                component.name
            ));
        }
    }

    Ok(StateSchema {
        component: component.name.clone(),
        fields,
    })
}

/// Split JSX into template and bindings
fn split_jsx(
    jsx_body: &str,
    component_name: &str,
    next_template_id: &mut u32,
    next_slot_id: &mut u32,
    template_dedup: &mut HashMap<String, u32>,
) -> Result<(Option<Template>, Vec<Binding>)> {
    // This is a simplified implementation
    // Production would need full JSX parser

    if jsx_body.is_empty() {
        return Ok((None, Vec::new()));
    }

    // TODO: Parse JSX properly
    // For now, create a dummy template for demo purposes
    let mut html = jsx_body.to_string();
    let mut slots = Vec::new();
    let mut bindings = Vec::new();

    // Example transformation:
    // <div>Count: {state.count}</div>
    // becomes:
    // <div>Count: <!--SLOT_0--></div>
    // with binding: SLOT_0 -> self.count

    // Simplified regex-based approach (production would use proper AST)
    let expression_pattern = Regex::new(r"\{([^}]+)\}").unwrap();

    // html initialized above

    for capture in expression_pattern.captures_iter(jsx_body) {
        let expression = capture.get(1).unwrap().as_str().trim();
        let slot_id = *next_slot_id;
        *next_slot_id += 1;

        // Replace with slot marker
        let marker = format!("<!--SLOT_{}-->", slot_id);
        html = html.replace(&format!("{{{}}}", expression), &marker);

        // Create slot definition
        slots.push(SlotDef {
            slot_id,
            slot_type: SlotType::Text,
            path: vec![0], // Simplified - would need proper DOM path calculation
        });

        // Create binding
        // Convert "state.count" to "self.count"
        let rust_expr = expression.replace("state.", "self.");

        bindings.push(Binding {
            slot_id,
            component: component_name.to_string(),
            expression: rust_expr,
            dirty_bit: 0, // TODO: Map to actual dirty bit
        });
    }

    // Deduplicate templates
    let hash = blake3::hash(html.as_bytes()).to_hex().to_string();

    let template_id = if let Some(&existing_id) = template_dedup.get(&hash) {
        existing_id
    } else {
        let id = *next_template_id;
        *next_template_id += 1;
        template_dedup.insert(hash.clone(), id);
        id
    };

    let template = Template {
        id: template_id,
        html,
        slots,
        hash,
    };

    Ok((Some(template), bindings))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jsx_splitting() {
        let jsx = r#"<div>Count: {state.count}</div>"#;
        let mut next_template_id = 0;
        let mut next_slot_id = 0;
        let mut dedup = HashMap::new();

        let (template, bindings) =
            split_jsx(jsx, "TestComponent", &mut next_template_id, &mut next_slot_id, &mut dedup)
                .unwrap();

        assert!(template.is_some());
        let template = template.unwrap();
        assert!(template.html.contains("<!--SLOT_0-->"));
        assert_eq!(bindings.len(), 1);
        assert_eq!(bindings[0].expression, "self.count");
    }
}
