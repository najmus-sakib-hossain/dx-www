//! DX Bundle Core - Binary formats and core types
//!
//! This crate defines all the binary formats and core types used across
//! the DX bundler ecosystem.

#![warn(missing_docs)]

use bytemuck::{Pod, Zeroable};

/// Magic numbers for binary formats
pub mod magic {
    /// Module graph magic: "DXMG"
    pub const MODULE_GRAPH: [u8; 4] = *b"DXMG";
    /// AST cache magic: "DXAC"
    pub const AST_CACHE: [u8; 4] = *b"DXAC";
    /// Tree shake result magic: "DXTS"
    pub const TREE_SHAKE: [u8; 4] = *b"DXTS";
    /// Source map magic: "DXSM"
    pub const SOURCE_MAP: [u8; 4] = *b"DXSM";
    /// Bundle magic: "DXBN"
    pub const BUNDLE: [u8; 4] = *b"DXBN";
}

/// Binary module graph format header
#[repr(C, packed)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct ModuleGraphHeader {
    /// Magic: "DXMG"
    pub magic: [u8; 4],
    /// Format version
    pub version: u32,
    /// Project source hash (for invalidation)
    pub source_hash: u128,
    /// Entry point count
    pub entry_count: u32,
    /// Total module count
    pub module_count: u32,
    /// Modules offset
    pub modules_offset: u64,
    /// Edges offset (import relationships)
    pub edges_offset: u64,
    /// Strings offset
    pub strings_offset: u64,
    /// Created timestamp
    pub created_at: u64,
}

/// Pre-computed module entry in the graph
#[repr(C, packed)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct ModuleEntry {
    /// Module ID (content hash for deduplication)
    pub id: u128,
    /// File path hash
    pub path_hash: u64,
    /// Path string offset
    pub path_offset: u32,
    /// Path length
    pub path_len: u16,
    /// Module kind
    pub kind: u8,
    /// Pre-compiled AST offset (in AST cache)
    pub ast_offset: u64,
    /// AST size
    pub ast_size: u32,
    /// First import index
    pub first_import: u32,
    /// Import count
    pub import_count: u16,
    /// First export index
    pub first_export: u32,
    /// Export count
    pub export_count: u16,
    /// Side effects flag
    pub has_side_effects: u8,
    /// Tree-shakeable flag
    pub tree_shakeable: u8,
    /// Source file mtime
    pub source_mtime: u64,
}

/// Module kind enumeration
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModuleKind {
    /// ES Module
    ESM = 0,
    /// CommonJS
    CJS = 1,
    /// JSON
    JSON = 2,
    /// CSS
    CSS = 3,
    /// Asset (images, fonts, etc.)
    Asset = 4,
    /// TypeScript
    TypeScript = 5,
    /// TSX (TypeScript + JSX)
    TSX = 6,
    /// JSX
    JSX = 7,
}

impl ModuleKind {
    /// Convert from u8
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::ESM),
            1 => Some(Self::CJS),
            2 => Some(Self::JSON),
            3 => Some(Self::CSS),
            4 => Some(Self::Asset),
            5 => Some(Self::TypeScript),
            6 => Some(Self::TSX),
            7 => Some(Self::JSX),
            _ => None,
        }
    }
}

/// Import edge in the dependency graph
#[repr(C, packed)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct ImportEdge {
    /// Source module index
    pub from_module: u32,
    /// Target module index
    pub to_module: u32,
    /// Import kind
    pub kind: u8,
    /// Is dynamic import flag
    pub is_dynamic: u8,
    /// Import specifier offset
    pub specifier_offset: u32,
    /// Specifier length
    pub specifier_len: u16,
}

/// Import kind enumeration
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImportKind {
    /// import x from 'y'
    Default = 0,
    /// import { x } from 'y'
    Named = 1,
    /// import * as x from 'y'
    Namespace = 2,
    /// import 'y' (side effect only)
    SideEffect = 3,
    /// require('y')
    CommonJS = 4,
    /// import('y')
    Dynamic = 5,
}

impl ImportKind {
    /// Convert from u8
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Default),
            1 => Some(Self::Named),
            2 => Some(Self::Namespace),
            3 => Some(Self::SideEffect),
            4 => Some(Self::CommonJS),
            5 => Some(Self::Dynamic),
            _ => None,
        }
    }
}

/// Export entry in the graph
#[repr(C, packed)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct ExportEntry {
    /// Module index
    pub module: u32,
    /// Export name hash
    pub name_hash: u64,
    /// Name offset
    pub name_offset: u32,
    /// Name length
    pub name_len: u16,
    /// Export kind
    pub kind: u8,
    /// Local binding offset (for re-exports)
    pub local_offset: u32,
}

/// Export kind enumeration
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportKind {
    /// export const x = ...
    Named = 0,
    /// export default ...
    Default = 1,
    /// export { x } from 'y'
    ReExport = 2,
    /// export * from 'y'
    ReExportAll = 3,
}

impl ExportKind {
    /// Convert from u8
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Named),
            1 => Some(Self::Default),
            2 => Some(Self::ReExport),
            3 => Some(Self::ReExportAll),
            _ => None,
        }
    }
}

/// Binary AST cache header
#[repr(C, packed)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct AstCacheHeader {
    /// Magic: "DXAC"
    pub magic: [u8; 4],
    /// Format version
    pub version: u32,
    /// Entry count
    pub entry_count: u32,
    /// Entries offset
    pub entries_offset: u64,
    /// AST data offset
    pub data_offset: u64,
}

/// AST cache entry
#[repr(C, packed)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct AstCacheEntry {
    /// File path hash
    pub path_hash: u64,
    /// Content hash (for invalidation)
    pub content_hash: u128,
    /// AST data offset
    pub ast_offset: u64,
    /// AST data size
    pub ast_size: u32,
    /// Source type
    pub source_type: u8,
    /// Parse flags
    pub flags: u8,
}

/// Binary AST representation
#[repr(C, packed)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct BinaryAst {
    /// Node count
    pub node_count: u32,
    /// Nodes offset
    pub nodes_offset: u32,
    /// Strings offset
    pub strings_offset: u32,
    /// Source positions offset
    pub positions_offset: u32,
}

/// AST node in binary format
#[repr(C, packed)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct AstNode {
    /// Node kind
    pub kind: u16,
    /// Parent index
    pub parent: u32,
    /// First child index
    pub first_child: u32,
    /// Next sibling index
    pub next_sibling: u32,
    /// Data offset (kind-specific data)
    pub data_offset: u32,
    /// Source start position
    pub start: u32,
    /// Source end position
    pub end: u32,
}

/// Tree shaking result header
#[repr(C, packed)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct TreeShakeHeader {
    /// Magic: "DXTS"
    pub magic: [u8; 4],
    /// Format version
    pub version: u32,
    /// Number of modules
    pub module_count: u32,
    /// Modules offset
    pub modules_offset: u64,
    /// Exports offset
    pub exports_offset: u64,
}

/// Tree shake result for a module
#[repr(C, packed)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct ShakeModule {
    /// Module index
    pub module: u32,
    /// Used exports bitmap (up to 64 exports)
    pub used_exports: u64,
    /// All exports used flag
    pub all_used: u8,
    /// Has side effects flag
    pub has_side_effects: u8,
    /// Can be completely removed flag
    pub can_remove: u8,
    /// Padding for alignment
    pub _padding: u8,
}

/// Binary source map header
#[repr(C, packed)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct SourceMapHeader {
    /// Magic: "DXSM"
    pub magic: [u8; 4],
    /// Format version
    pub version: u32,
    /// Number of sources
    pub source_count: u32,
    /// Number of names
    pub name_count: u32,
    /// Number of mappings
    pub mapping_count: u32,
    /// Sources offset
    pub sources_offset: u64,
    /// Names offset
    pub names_offset: u64,
    /// Mappings offset
    pub mappings_offset: u64,
    /// Strings offset
    pub strings_offset: u64,
}

/// Binary source map mapping entry
#[repr(C, packed)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct BinaryMapping {
    /// Generated line
    pub gen_line: u32,
    /// Generated column
    pub gen_column: u32,
    /// Original source index
    pub source: u16,
    /// Original line
    pub orig_line: u32,
    /// Original column
    pub orig_column: u32,
    /// Name index (u16::MAX if none)
    pub name: u16,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_kind_conversion() {
        assert_eq!(ModuleKind::from_u8(0), Some(ModuleKind::ESM));
        assert_eq!(ModuleKind::from_u8(6), Some(ModuleKind::TSX));
        assert_eq!(ModuleKind::from_u8(255), None);
    }

    #[test]
    fn test_import_kind_conversion() {
        assert_eq!(ImportKind::from_u8(0), Some(ImportKind::Default));
        assert_eq!(ImportKind::from_u8(5), Some(ImportKind::Dynamic));
        assert_eq!(ImportKind::from_u8(255), None);
    }

    #[test]
    fn test_magic_numbers() {
        assert_eq!(&magic::MODULE_GRAPH, b"DXMG");
        assert_eq!(&magic::AST_CACHE, b"DXAC");
        assert_eq!(&magic::TREE_SHAKE, b"DXTS");
        assert_eq!(&magic::SOURCE_MAP, b"DXSM");
    }
}
