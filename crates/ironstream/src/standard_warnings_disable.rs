// FILE: standard_warnings_disable.rs
// occt: Standard_WarningsDisable

//! Compiler warnings suppression marker.
//! In Rust, warnings are controlled via attributes and clippy directives.
//! This module serves as a placeholder for C++ compatibility.

/// Marker to suppress compiler warnings.
/// Use with `#![allow(...)]` attributes in Rust.
pub struct WarningsDisable;

impl WarningsDisable {
    /// Create a new warning suppression marker.
    pub fn new() -> Self {
        WarningsDisable
    }
}

impl Default for WarningsDisable {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_warnings_disable_creation() {
        let _marker = WarningsDisable::new();
    }

    #[test]
    fn test_warnings_disable_default() {
        let _marker = WarningsDisable::default();
    }
}
