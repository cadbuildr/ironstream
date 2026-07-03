// FILE: topo_ds_un_compatible_shapes.rs
// occt: TopoDS_UnCompatibleShapes

use std::fmt;

/// Exception raised when an incorrect shape insertion was attempted.
/// Shapes may be incompatible based on their type or current state.
#[derive(Clone, Debug)]
pub struct TopoDsUnCompatibleShapes {
    message: String,
}

impl TopoDsUnCompatibleShapes {
    /// Creates a new UnCompatibleShapes exception with the given message.
    pub fn new(message: impl Into<String>) -> Self {
        TopoDsUnCompatibleShapes {
            message: message.into(),
        }
    }

    /// Returns the exception message.
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for TopoDsUnCompatibleShapes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TopoDS_UnCompatibleShapes: {}", self.message)
    }
}

impl std::error::Error for TopoDsUnCompatibleShapes {}

/// Helper macro to raise UnCompatibleShapes exception conditionally.
/// Equivalent to C++ TopoDS_UnCompatibleShapes_Raise_if.
#[macro_export]
macro_rules! un_compatible_shapes_raise_if {
    ($condition:expr, $message:expr) => {
        if $condition {
            return Err($crate::topo_ds_un_compatible_shapes::TopoDsUnCompatibleShapes::new($message));
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_exception() {
        let ex = TopoDsUnCompatibleShapes::new("Incompatible shape types");
        assert_eq!(ex.message(), "Incompatible shape types");
    }

    #[test]
    fn test_display() {
        let ex = TopoDsUnCompatibleShapes::new("Cannot insert edge into vertex");
        let formatted = format!("{}", ex);
        assert!(formatted.contains("TopoDS_UnCompatibleShapes"));
        assert!(formatted.contains("Cannot insert"));
    }

    #[test]
    fn test_is_error() {
        let ex = TopoDsUnCompatibleShapes::new("Test error");
        let _error: Box<dyn std::error::Error> = Box::new(ex);
    }

    #[test]
    fn test_clone() {
        let ex1 = TopoDsUnCompatibleShapes::new("Clone test");
        let ex2 = ex1.clone();
        assert_eq!(ex1.message(), ex2.message());
    }

    #[test]
    fn test_macro_condition_true() {
        let result: Result<(), TopoDsUnCompatibleShapes> = (|| {
            let shapes_compatible = false;
            un_compatible_shapes_raise_if!(!shapes_compatible, "Shapes are incompatible");
            Ok(())
        })();

        assert!(result.is_err());
        assert!(result.unwrap_err().message().contains("incompatible"));
    }

    #[test]
    fn test_macro_condition_false() {
        let result: Result<(), TopoDsUnCompatibleShapes> = (|| {
            let shapes_compatible = true;
            un_compatible_shapes_raise_if!(!shapes_compatible, "Shapes are incompatible");
            Ok(())
        })();

        assert!(result.is_ok());
    }
}
