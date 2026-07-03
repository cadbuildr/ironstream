// FILE: standard_warnings_restore.rs
// occt: Standard_WarningsRestore

//! Compiler warnings restoration marker.
//! In Rust, warnings are controlled via attributes and clippy directives.
//! This module serves as a placeholder for C++ compatibility.

/// Marker to restore compiler warnings.
/// Use with warning control attributes in Rust.
pub struct WarningsRestore;

impl WarningsRestore {
    /// Create a new warning restoration marker.
    pub fn new() -> Self {
        WarningsRestore
    }
}

impl Default for WarningsRestore {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_warnings_restore_creation() {
        let _marker = WarningsRestore::new();
    }

    #[test]
    fn test_warnings_restore_default() {
        let _marker = WarningsRestore::default();
    }
}
