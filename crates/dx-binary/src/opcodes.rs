//! # HTIP v1 Opcodes
//!
//! The 11 operations that define the entire web rendering protocol.
//!
//! ## Design Philosophy
//!
//! Every web UI can be expressed as:
//! 1. Define templates (static HTML structure)
//! 2. Instantiate templates (cloneNode)
//! 3. Patch dynamic slots (text, attributes, events)
//!
//! That's it. No VDOM. No diffing. O(1) updates.

use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

/// HTIP v1 Opcode (11 total - locked forever)
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Encode, Decode)]
pub enum OpcodeV1 {
    /// Define a new template in the dictionary
    /// Payload: TemplateDef { id: u16, html: String, bindings: Vec<Binding> }
    TemplateDef = 0x01,

    /// Instantiate a template (cloneNode)
    /// Payload: Instantiate { instance_id: u32, template_id: u16, parent_id: u32 }
    Instantiate = 0x02,

    /// Patch text content at slot
    /// Payload: PatchText { instance_id: u32, slot_id: u16, string_id: u32 }
    PatchText = 0x03,

    /// Patch attribute at slot
    /// Payload: PatchAttr { instance_id: u32, slot_id: u16, attr_name: u32, value: u32 }
    PatchAttr = 0x04,

    /// Toggle CSS class
    /// Payload: PatchClassToggle { instance_id: u32, class_name: u32, enabled: bool }
    PatchClassToggle = 0x05,

    /// Attach event listener
    /// Payload: AttachEvent { instance_id: u32, event_type: u32, handler_id: u32 }
    AttachEvent = 0x06,

    /// Remove node
    /// Payload: RemoveNode { instance_id: u32 }
    RemoveNode = 0x07,

    /// Batch operation start (for transaction grouping)
    /// Payload: BatchStart { batch_id: u32 }
    BatchStart = 0x08,

    /// Batch operation commit
    /// Payload: BatchCommit { batch_id: u32 }
    BatchCommit = 0x09,

    /// Set property (e.g., input.value, checked)
    /// Payload: SetProperty { instance_id: u32, prop_name: u32, value: PropertyValue }
    SetProperty = 0x0A,

    /// Append child node
    /// Payload: AppendChild { parent_id: u32, child_id: u32 }
    AppendChild = 0x0B,
}

impl OpcodeV1 {
    /// Convert from u8 (for deserialization)
    pub fn from_u8(byte: u8) -> Option<Self> {
        match byte {
            0x01 => Some(OpcodeV1::TemplateDef),
            0x02 => Some(OpcodeV1::Instantiate),
            0x03 => Some(OpcodeV1::PatchText),
            0x04 => Some(OpcodeV1::PatchAttr),
            0x05 => Some(OpcodeV1::PatchClassToggle),
            0x06 => Some(OpcodeV1::AttachEvent),
            0x07 => Some(OpcodeV1::RemoveNode),
            0x08 => Some(OpcodeV1::BatchStart),
            0x09 => Some(OpcodeV1::BatchCommit),
            0x0A => Some(OpcodeV1::SetProperty),
            0x0B => Some(OpcodeV1::AppendChild),
            _ => None,
        }
    }

    /// Convert to u8 (for serialization)
    pub fn to_u8(self) -> u8 {
        self as u8
    }
}

/// Template definition with binding slots
#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct TemplateDef {
    pub id: u16,
    pub html_string_id: u32, // Reference to string table
    pub bindings: Vec<Binding>,
}

/// Binding slot definition
#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct Binding {
    pub slot_id: u16,
    pub binding_type: BindingType,
    pub path: Vec<u8>, // DOM path (e.g., [0, 2, 1] = firstChild.childNodes[2].firstChild)
}

#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub enum BindingType {
    Text,
    Attribute { attr_name_id: u32 },
    Property { prop_name_id: u32 },
    Event { event_type_id: u32 },
    Class,
}

/// Instantiate template
#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct Instantiate {
    pub instance_id: u32,
    pub template_id: u16,
    pub parent_id: u32,
}

/// Patch text content
#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct PatchText {
    pub instance_id: u32,
    pub slot_id: u16,
    pub string_id: u32,
}

/// Patch attribute
#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct PatchAttr {
    pub instance_id: u32,
    pub slot_id: u16,
    pub attr_name_id: u32,
    pub value_id: u32,
}

/// Toggle CSS class
#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct PatchClassToggle {
    pub instance_id: u32,
    pub class_name_id: u32,
    pub enabled: bool,
}

/// Attach event listener
#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct AttachEvent {
    pub instance_id: u32,
    pub event_type_id: u32,
    pub handler_id: u32,
}

/// Remove node
#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct RemoveNode {
    pub instance_id: u32,
}

/// Batch operation start
#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct BatchStart {
    pub batch_id: u32,
}

/// Batch operation commit
#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct BatchCommit {
    pub batch_id: u32,
}

/// Set property
#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct SetProperty {
    pub instance_id: u32,
    pub prop_name_id: u32,
    pub value: PropertyValue,
}

/// Property value types
#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub enum PropertyValue {
    String(u32), // String ID
    Number(f64),
    Boolean(bool),
    Null,
}

/// Append child
#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct AppendChild {
    pub parent_id: u32,
    pub child_id: u32,
}

/// Combined operation payload
#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub enum Operation {
    TemplateDef(TemplateDef),
    Instantiate(Instantiate),
    PatchText(PatchText),
    PatchAttr(PatchAttr),
    PatchClassToggle(PatchClassToggle),
    AttachEvent(AttachEvent),
    RemoveNode(RemoveNode),
    BatchStart(BatchStart),
    BatchCommit(BatchCommit),
    SetProperty(SetProperty),
    AppendChild(AppendChild),
}

impl Operation {
    /// Get opcode for this operation
    pub fn opcode(&self) -> OpcodeV1 {
        match self {
            Operation::TemplateDef(_) => OpcodeV1::TemplateDef,
            Operation::Instantiate(_) => OpcodeV1::Instantiate,
            Operation::PatchText(_) => OpcodeV1::PatchText,
            Operation::PatchAttr(_) => OpcodeV1::PatchAttr,
            Operation::PatchClassToggle(_) => OpcodeV1::PatchClassToggle,
            Operation::AttachEvent(_) => OpcodeV1::AttachEvent,
            Operation::RemoveNode(_) => OpcodeV1::RemoveNode,
            Operation::BatchStart(_) => OpcodeV1::BatchStart,
            Operation::BatchCommit(_) => OpcodeV1::BatchCommit,
            Operation::SetProperty(_) => OpcodeV1::SetProperty,
            Operation::AppendChild(_) => OpcodeV1::AppendChild,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opcode_conversion() {
        for i in 0x01u8..=0x0Bu8 {
            let opcode = OpcodeV1::from_u8(i).unwrap();
            assert_eq!(opcode.to_u8(), i);
        }
    }

    #[test]
    fn test_opcode_size() {
        // Ensure opcode is 1 byte
        assert_eq!(std::mem::size_of::<OpcodeV1>(), 1);
    }

    #[test]
    fn test_invalid_opcode() {
        assert!(OpcodeV1::from_u8(0x00).is_none());
        assert!(OpcodeV1::from_u8(0xFF).is_none());
    }
}
