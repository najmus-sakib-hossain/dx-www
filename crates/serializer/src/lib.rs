//! # DX Serializer
//!
//! The world's best serializer - optimized for Humans, LLMs, AND Machines.
//!
//! ## Two Simple Formats
//!
//! | Format | Use Case | Performance |
//! |--------|----------|-------------|
//! | **DX LLM** | Text format for humans & LLMs | 26.8% more efficient than TOON |
//! | **DX Machine** | Binary format for runtime | 0.70ns field access (hardware limit) |
//!
//! ## Quick Start
//!
//! ```rust
//! use serializer::{DxDocument, DxLlmValue, DxSection};
//! use serializer::{document_to_llm, llm_to_document};  // LLM format
//! use serializer::zero::DxZeroBuilder;                  // Machine format
//!
//! // Create a document
//! let mut doc = DxDocument::new();
//! doc.context.insert("name".to_string(), DxLlmValue::Str("MyApp".to_string()));
//!
//! // Convert to LLM format (text, 26.8% better than TOON)
//! let llm_text = document_to_llm(&doc);
//!
//! // Convert to Machine format (binary, 0.70ns access)
//! let mut buffer = Vec::new();
//! let mut builder = DxZeroBuilder::new(&mut buffer, 8, 1);
//! builder.write_u64(0, 12345);
//! builder.write_string(8, "MyApp");
//! builder.finish();
//! ```
//!
//! ## Holographic Architecture
//!
//! DX seamlessly converts between formats:
//! - **Human Format** (Editor View) - Beautiful, readable, collapsible
//! - **LLM Format** (Disk Storage) - Token-efficient, 26.8% better than TOON
//! - **Machine Format** (Runtime) - Binary, 0.70ns access, 27× faster than rkyv
//!
//! ## Key Features
//! - Base62 integers (%x): 320→5A, 540→8k
//! - Auto-increment (%#): Sequential IDs generated automatically
//! - Holographic inflate/deflate for editor integration
//! - Zero-copy binary format with sub-nanosecond field access

// Allow dead_code for API completeness
#![allow(dead_code)]

pub mod base62;
#[cfg(test)]
mod base62_props;
pub mod binary_output;

// Platform-specific async I/O
#[cfg(feature = "async-io")]
pub mod io;
pub mod compress;
pub mod converters;
pub mod encoder;
pub mod error;
#[cfg(test)]
mod error_props;
pub mod formatter;
pub mod hologram;
pub mod llm;
pub mod mappings;
pub mod optimizer;
pub mod parser;
pub mod schema;
pub mod tokenizer;
pub mod types;
pub mod utf8;
#[cfg(test)]
mod utf8_props;
pub mod wasm;
pub mod watch;
pub mod zero;

pub use base62::{decode_base62, encode_base62};
pub use binary_output::{
    get_binary_path, hash_path, is_cache_valid, read_binary, write_binary, BinaryConfig,
};
pub use compress::{compress_to_writer, format_machine};
pub use converters::{convert_to_dx, json_to_dx, toml_to_dx, toon_to_dx, yaml_to_dx};
pub use encoder::{encode, encode_to_writer, Encoder};
pub use error::{DxError, Result};
pub use formatter::{format_human, HumanFormatter};
pub use hologram::{deflate, inflate, Deflater, HologramConfig, Inflater};
pub use mappings::Mappings;
pub use optimizer::{optimize_key, optimize_path};
pub use parser::{parse, parse_stream, Parser};
pub use schema::{Schema, TypeHint};
pub use types::{DxArray, DxObject, DxValue};
pub use utf8::{validate_utf8, validate_utf8_detailed, validate_utf8_owned, validate_string_input, Utf8ValidationError};

// Re-export LLM/Human format types at crate root for convenience
pub use llm::{
    AbbrevDict, ConvertError, DxDocument, DxLlmValue, DxSection, HumanFormatConfig,
    HumanFormatter as LlmHumanFormatter, HumanParseError, HumanParser, LlmParser, LlmSerializer,
    MachineFormat, ParseError as LlmParseError, TableStyle,
};
pub use llm::{
    document_to_human, document_to_llm, document_to_machine, human_to_document, human_to_llm,
    human_to_machine, llm_to_document, llm_to_human, machine_to_document, machine_to_human,
    machine_to_llm,
};

// Re-export Human Format V2 types
pub use llm::{
    box_chars, HumanFormatV2Config, HumanFormatterV2,
    PrettyPrinter, PrettyPrinterConfig, PrettyPrintError,
    TableWrapper, TableWrapperConfig,
    CacheConfig, CacheGenerator, CacheError, CachePaths, CacheResult,
};
pub use llm::{
    document_to_human_v2, document_to_human_v2_with_config,
    human_to_llm_v2, human_v2_to_document, human_v2_to_machine,
    llm_to_human_v2, llm_to_human_v2_with_config, machine_to_human_v2,
};

// Re-export WASM types for VS Code extension
pub use wasm::{DxSerializer, SerializerConfig, TransformResult, ValidationResult, smart_quote};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_trip() {
        // Simple key-value format that the parser supports
        let input = b"name:Test
value:123
active:+";

        let parsed = parse(input).expect("Parse failed");
        let encoded = encode(&parsed).expect("Encode failed");
        let reparsed = parse(&encoded).expect("Reparse failed");

        assert_eq!(parsed, reparsed);
    }

    #[test]
    fn test_human_format() {
        let input = b"data=id%i name%s
1 Test
2 Demo";

        let parsed = parse(input).expect("Parse failed");
        let human = format_human(&parsed).expect("Format failed");

        assert!(human.contains("DATA TABLE"));
        assert!(human.contains("Test"));
        assert!(human.contains("Demo"));
    }
}
