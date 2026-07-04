// FILE: interface_interface_mismatch.rs
// occt: Interface_InterfaceMismatch

use std::fmt;

/// Exception thrown for Interface mismatch errors
#[derive(Debug, Clone)]
pub struct InterfaceInterfaceMismatch {
    message: String,
}

impl InterfaceInterfaceMismatch {
    /// Create a new InterfaceMismatch with a message
    pub fn new(message: impl Into<String>) -> Self {
        InterfaceInterfaceMismatch {
            message: message.into(),
        }
    }

    /// Get the error message
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for InterfaceInterfaceMismatch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Interface Mismatch: {}", self.message)
    }
}

impl std::error::Error for InterfaceInterfaceMismatch {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_error() {
        let err = InterfaceInterfaceMismatch::new("Mismatch error");
        assert_eq!(err.message(), "Mismatch error");
    }

    #[test]
    fn test_display() {
        let err = InterfaceInterfaceMismatch::new("Test mismatch");
        let msg = format!("{}", err);
        assert!(msg.contains("Interface Mismatch"));
    }

    #[test]
    fn test_clone() {
        let err1 = InterfaceInterfaceMismatch::new("Test");
        let err2 = err1.clone();
        assert_eq!(err1.message(), err2.message());
    }
}
