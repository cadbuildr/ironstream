// FILE: topo_ds_locked_shape.rs
// occt: TopoDS_LockedShape

use std::fmt;

/// Exception raised when attempting to modify the geometry of a locked shape.
/// A locked shape is one that is already shared or protected and its geometry cannot change.
#[derive(Clone, Debug)]
pub struct TopoDsLockedShape {
    message: String,
}

impl TopoDsLockedShape {
    /// Creates a new LockedShape exception with the given message.
    pub fn new(message: impl Into<String>) -> Self {
        TopoDsLockedShape {
            message: message.into(),
        }
    }

    /// Returns the exception message.
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for TopoDsLockedShape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TopoDS_LockedShape: {}", self.message)
    }
}

impl std::error::Error for TopoDsLockedShape {}

/// Helper macro to raise LockedShape exception conditionally.
/// Equivalent to C++ TopoDS_LockedShape_Raise_if.
#[macro_export]
macro_rules! locked_shape_raise_if {
    ($condition:expr, $message:expr) => {
        if $condition {
            return Err($crate::topo_ds_locked_shape::TopoDsLockedShape::new($message));
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_locked_shape() {
        let ex = TopoDsLockedShape::new("Shape geometry is locked");
        assert_eq!(ex.message(), "Shape geometry is locked");
    }

    #[test]
    fn test_display() {
        let ex = TopoDsLockedShape::new("Cannot modify geometry");
        let formatted = format!("{}", ex);
        assert!(formatted.contains("TopoDS_LockedShape"));
        assert!(formatted.contains("Cannot modify geometry"));
    }

    #[test]
    fn test_is_error() {
        let ex = TopoDsLockedShape::new("Test error");
        let _error: Box<dyn std::error::Error> = Box::new(ex);
    }

    #[test]
    fn test_clone() {
        let ex1 = TopoDsLockedShape::new("Clone test");
        let ex2 = ex1.clone();
        assert_eq!(ex1.message(), ex2.message());
    }

    #[test]
    fn test_macro_condition_true() {
        let result: Result<(), TopoDsLockedShape> = (|| {
            let is_locked = true;
            locked_shape_raise_if!(is_locked, "Shape is locked");
            Ok(())
        })();

        assert!(result.is_err());
        assert!(result.unwrap_err().message().contains("locked"));
    }

    #[test]
    fn test_macro_condition_false() {
        let result: Result<(), TopoDsLockedShape> = (|| {
            let is_locked = false;
            locked_shape_raise_if!(is_locked, "Shape is locked");
            Ok(())
        })();

        assert!(result.is_ok());
    }
}
