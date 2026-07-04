// FILE: interface_interface_error.rs
// occt: Interface_InterfaceError

use std::fmt;

/// Exception thrown for Interface errors
#[derive(Debug, Clone)]
pub struct InterfaceInterfaceError {
    message: String,
}

impl InterfaceInterfaceError {
    /// Create a new InterfaceError with a message
    pub fn new(message: impl Into<String>) -> Self {
        InterfaceInterfaceError {
            message: message.into(),
        }
    }

    /// Get the error message
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for InterfaceInterfaceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Interface Error: {}", self.message)
    }
}

impl std::error::Error for InterfaceInterfaceError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_error() {
        let err = InterfaceInterfaceError::new("Test error");
        assert_eq!(err.message(), "Test error");
    }

    #[test]
    fn test_display() {
        let err = InterfaceInterfaceError::new("Test error");
        let msg = format!("{}", err);
        assert!(msg.contains("Interface Error"));
    }

    #[test]
    fn test_clone() {
        let err1 = InterfaceInterfaceError::new("Test");
        let err2 = err1.clone();
        assert_eq!(err1.message(), err2.message());
    }
}
