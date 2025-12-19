//! Dynamic library loading.

use crate::error::{FfiError, FfiResult};
use libloading::Library;

/// Load a dynamic library.
pub fn dlopen(path: &str) -> FfiResult<DynamicLibrary> {
    let lib = unsafe { Library::new(path)? };
    Ok(DynamicLibrary { lib })
}

/// Dynamic library handle.
pub struct DynamicLibrary {
    lib: Library,
}

impl DynamicLibrary {
    /// Get a function symbol.
    ///
    /// # Safety
    /// The caller must ensure the symbol type matches the actual function signature.
    pub unsafe fn get<T>(&self, name: &str) -> FfiResult<libloading::Symbol<T>> {
        unsafe {
            self.lib
                .get(name.as_bytes())
                .map_err(|_| FfiError::SymbolNotFound(name.to_string()))
        }
    }

    /// Close the library.
    pub fn close(self) {
        drop(self.lib);
    }
}
