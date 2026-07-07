// FILE: v3d_bad_value.rs
// occt: V3d_BadValue

use std::fmt;

/// Exception raised when a bad value is passed to V3d operations.
/// Inherits from OutOfRange domain error.
#[derive(Debug, Clone)]
pub struct V3dBadValue {
    message: String,
}

impl V3dBadValue {
    /// Create a new V3dBadValue exception with a message
    pub fn new(message: impl Into<String>) -> Self {
        V3dBadValue {
            message: message.into(),
        }
    }

    /// Get the error message
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for V3dBadValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "V3d_BadValue: {}", self.message)
    }
}

impl std::error::Error for V3dBadValue {}

/// Macro to raise V3dBadValue if a condition is true
#[macro_export]
macro_rules! v3d_bad_value_raise_if {
    ($condition:expr, $message:expr) => {
        if $condition {
            return Err($crate::v3d_bad_value::V3dBadValue::new($message));
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_exception() {
        let exc = V3dBadValue::new("test message");
        assert_eq!(exc.message(), "test message");
    }

    #[test]
    fn test_display() {
        let exc = V3dBadValue::new("bad parameter");
        assert_eq!(
            format!("{}", exc),
            "V3d_BadValue: bad parameter"
        );
    }

    #[test]
    fn test_is_error() {
        let exc = V3dBadValue::new("test");
        let _: Box<dyn std::error::Error> = Box::new(exc);
    }

    #[test]
    fn test_macro_raises() {
        fn test_fn() -> Result<(), V3dBadValue> {
            let value = -5.0;
            v3d_bad_value_raise_if!(value < 0.0, "value must be non-negative");
            Ok(())
        }

        let result = test_fn();
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().message(),
            "value must be non-negative"
        );
    }

    #[test]
    fn test_macro_no_raise() {
        fn test_fn() -> Result<(), V3dBadValue> {
            let value = 5.0;
            v3d_bad_value_raise_if!(value < 0.0, "value must be non-negative");
            Ok(())
        }

        let result = test_fn();
        assert!(result.is_ok());
    }
}
