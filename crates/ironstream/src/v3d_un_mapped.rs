// FILE: v3d_un_mapped.rs
// occt: V3d_UnMapped

use std::fmt;

/// Exception raised when an unmapped operation is attempted on a V3d object.
/// Inherits from DomainError domain error.
#[derive(Debug, Clone)]
pub struct V3dUnMapped {
    message: String,
}

impl V3dUnMapped {
    /// Create a new V3dUnMapped exception with a message
    pub fn new(message: impl Into<String>) -> Self {
        V3dUnMapped {
            message: message.into(),
        }
    }

    /// Get the error message
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for V3dUnMapped {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "V3d_UnMapped: {}", self.message)
    }
}

impl std::error::Error for V3dUnMapped {}

/// Macro to raise V3dUnMapped if a condition is true
#[macro_export]
macro_rules! v3d_un_mapped_raise_if {
    ($condition:expr, $message:expr) => {
        if $condition {
            return Err($crate::v3d_un_mapped::V3dUnMapped::new($message));
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_exception() {
        let exc = V3dUnMapped::new("object not mapped");
        assert_eq!(exc.message(), "object not mapped");
    }

    #[test]
    fn test_display() {
        let exc = V3dUnMapped::new("viewer not mapped");
        assert_eq!(format!("{}", exc), "V3d_UnMapped: viewer not mapped");
    }

    #[test]
    fn test_is_error() {
        let exc = V3dUnMapped::new("test");
        let _: Box<dyn std::error::Error> = Box::new(exc);
    }

    #[test]
    fn test_macro_raises() {
        fn test_fn() -> Result<(), V3dUnMapped> {
            let is_mapped = false;
            v3d_un_mapped_raise_if!(!is_mapped, "viewer is not mapped");
            Ok(())
        }

        let result = test_fn();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().message(), "viewer is not mapped");
    }

    #[test]
    fn test_macro_no_raise() {
        fn test_fn() -> Result<(), V3dUnMapped> {
            let is_mapped = true;
            v3d_un_mapped_raise_if!(!is_mapped, "viewer is not mapped");
            Ok(())
        }

        let result = test_fn();
        assert!(result.is_ok());
    }
}
