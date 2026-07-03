// FILE: standard_warning_disable_function_cast.rs
// occt: Standard_WarningDisableFunctionCast

//! Disable function cast warning marker.
//! Used for suppressing warnings about function pointer casts.

/// Marker to disable function cast warnings.
/// In Rust, use with unsafe function pointer casts.
pub struct WarningDisableFunctionCast;

impl WarningDisableFunctionCast {
    /// Create a new function cast warning disable marker.
    pub fn new() -> Self {
        WarningDisableFunctionCast
    }
}

impl Default for WarningDisableFunctionCast {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_warning_disable_function_cast_creation() {
        let _marker = WarningDisableFunctionCast::new();
    }

    #[test]
    fn test_warning_disable_function_cast_default() {
        let _marker = WarningDisableFunctionCast::default();
    }
}
