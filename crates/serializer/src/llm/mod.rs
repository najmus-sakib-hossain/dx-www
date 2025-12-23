//! LLM and Human format serialization module
//!
//! This module provides support for three interconvertible formats:
//! - **LLM Format**: Token-optimized using sigils (#c, #:, #<letter>), references (^key), and abbreviated keys
//! - **Human Format**: Beautiful TOML-like display with Unicode tables, expanded keys, and summaries
//! - **Machine Format**: Binary format for runtime (already implemented in parent crate)
//!
//! The architecture follows a "hub and spoke" model where all formats convert through
//! a common internal representation (`DxDocument`), ensuring consistent round-trip behavior.

pub mod abbrev;
pub mod cache_generator;
pub mod convert;
pub mod human_formatter;
pub mod human_formatter_v2;
pub mod human_parser;
pub mod parser;
pub mod pretty_printer;
pub mod serializer;
pub mod table_wrapper;
pub mod types;

#[cfg(test)]
mod abbrev_props;
#[cfg(test)]
mod convert_props;
#[cfg(test)]
mod human_props;
#[cfg(test)]
mod llm_props;

// Re-export main types
pub use abbrev::AbbrevDict;
pub use cache_generator::{CacheConfig, CacheGenerator, CacheError, CachePaths, CacheResult};
pub use convert::{
    document_to_human, document_to_human_v2, document_to_human_v2_with_config,
    document_to_llm, document_to_machine, human_to_document, human_to_llm,
    human_to_llm_v2, human_to_machine, human_v2_to_document, human_v2_to_machine,
    llm_to_document, llm_to_human, llm_to_human_v2, llm_to_human_v2_with_config,
    machine_to_document, machine_to_human, machine_to_human_v2, machine_to_llm,
    ConvertError, MachineFormat,
};
pub use human_formatter::{HumanFormatConfig, HumanFormatter, TableStyle};
pub use human_formatter_v2::{box_chars, HumanFormatV2Config, HumanFormatterV2};
pub use human_parser::{HumanParseError, HumanParser};
pub use parser::{LlmParser, ParseError};
pub use pretty_printer::{PrettyPrinter, PrettyPrinterConfig, PrettyPrintError};
pub use serializer::LlmSerializer;
pub use table_wrapper::{TableWrapper, TableWrapperConfig};
pub use types::{DxDocument, DxLlmValue, DxSection};
