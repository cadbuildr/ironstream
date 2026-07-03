// FILE: standard_macro.rs
// occt: Standard_Macro

//! Platform-specific pre-processor macros for Open CASCADE.
//! In Rust, we provide compile-time constants and deprecation helpers.

/// Compile-time version identifier for compatibility checking.
pub const RUST_IRONSTREAM_VERSION: &str = "1.0.0";

/// Helper macro-like function to emit compile-time warnings about deprecated usage.
/// In Rust, we use the #[deprecated] attribute for functions/types.
pub fn deprecated_warning(message: &str) {
    eprintln!("Deprecation Warning: {}", message);
}

/// Helper to mark functions as deprecated.
/// Example: #[deprecated = "Use new_function instead"]
/// pub fn old_function() { }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_defined() {
        assert!(!RUST_IRONSTREAM_VERSION.is_empty());
        assert_eq!(RUST_IRONSTREAM_VERSION, "1.0.0");
    }

    #[test]
    fn test_deprecated_warning() {
        // Just verify the function can be called
        deprecated_warning("Test deprecation message");
    }
}
