// FILE: topo_ds_frozen_shape.rs
// occt: TopoDS_FrozenShape

use std::fmt;

/// Exception raised when attempting to modify a shape that is frozen or protected.
/// A frozen shape is one that is already shared or in use and cannot be modified.
#[derive(Clone, Debug)]
pub struct TopoDsFrozenShape {
    message: String,
}

impl TopoDsFrozenShape {
    /// Creates a new FrozenShape exception with the given message.
    pub fn new(message: impl Into<String>) -> Self {
        TopoDsFrozenShape {
            message: message.into(),
        }
    }

    /// Returns the exception message.
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for TopoDsFrozenShape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TopoDS_FrozenShape: {}", self.message)
    }
}

impl std::error::Error for TopoDsFrozenShape {}

/// Helper macro to raise FrozenShape exception conditionally.
/// Equivalent to C++ TopoDS_FrozenShape_Raise_if.
#[macro_export]
macro_rules! frozen_shape_raise_if {
    ($condition:expr, $message:expr) => {
        if $condition {
            return Err($crate::topo_ds_frozen_shape::TopoDsFrozenShape::new($message));
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_frozen_shape() {
        let ex = TopoDsFrozenShape::new("Shape is frozen");
        assert_eq!(ex.message(), "Shape is frozen");
    }

    #[test]
    fn test_display() {
        let ex = TopoDsFrozenShape::new("Cannot modify");
        let formatted = format!("{}", ex);
        assert!(formatted.contains("TopoDS_FrozenShape"));
        assert!(formatted.contains("Cannot modify"));
    }

    #[test]
    fn test_is_error() {
        let ex = TopoDsFrozenShape::new("Test error");
        let _error: Box<dyn std::error::Error> = Box::new(ex);
    }

    #[test]
    fn test_clone() {
        let ex1 = TopoDsFrozenShape::new("Clone test");
        let ex2 = ex1.clone();
        assert_eq!(ex1.message(), ex2.message());
    }

    #[test]
    fn test_macro_condition_true() {
        let result: Result<(), TopoDsFrozenShape> = (|| {
            let is_frozen = true;
            frozen_shape_raise_if!(is_frozen, "Shape is protected");
            Ok(())
        })();

        assert!(result.is_err());
        assert!(result.unwrap_err().message().contains("protected"));
    }

    #[test]
    fn test_macro_condition_false() {
        let result: Result<(), TopoDsFrozenShape> = (|| {
            let is_frozen = false;
            frozen_shape_raise_if!(is_frozen, "Shape is protected");
            Ok(())
        })();

        assert!(result.is_ok());
    }
}
