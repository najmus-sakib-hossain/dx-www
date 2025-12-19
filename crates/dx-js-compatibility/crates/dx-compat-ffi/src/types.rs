//! FFI type definitions.

/// FFI type definitions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FfiType {
    /// Void type
    Void,
    /// Boolean type
    Bool,
    /// Signed 8-bit integer
    I8,
    /// Signed 16-bit integer
    I16,
    /// Signed 32-bit integer
    I32,
    /// Signed 64-bit integer
    I64,
    /// Unsigned 8-bit integer
    U8,
    /// Unsigned 16-bit integer
    U16,
    /// Unsigned 32-bit integer
    U32,
    /// Unsigned 64-bit integer
    U64,
    /// 32-bit float
    F32,
    /// 64-bit float
    F64,
    /// Pointer type
    Pointer,
    /// C string type
    CString,
}
