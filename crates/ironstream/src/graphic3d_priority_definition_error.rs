// FILE: graphic3d_priority_definition_error.rs
// occt: Graphic3d_PriorityDefinitionError

use std::fmt;

/// Exception raised when an invalid priority is provided for a graphic structure.
/// Derived from Standard_OutOfRange in OCCT.
#[derive(Debug, Clone)]
pub struct PriorityDefinitionError {
    message: String,
}

impl PriorityDefinitionError {
    /// Creates a new priority definition error with the given message.
    pub fn new(message: impl Into<String>) -> Self {
        PriorityDefinitionError {
            message: message.into(),
        }
    }

    /// Returns the error message.
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for PriorityDefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Graphic3d_PriorityDefinitionError: {}", self.message)
    }
}

impl std::error::Error for PriorityDefinitionError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority_definition_error_creation() {
        let error = PriorityDefinitionError::new("Invalid priority value");
        assert_eq!(error.message(), "Invalid priority value");
    }

    #[test]
    fn test_priority_definition_error_display() {
        let error = PriorityDefinitionError::new("Priority out of range");
        let display_str = format!("{}", error);
        assert!(display_str.contains("Graphic3d_PriorityDefinitionError"));
        assert!(display_str.contains("Priority out of range"));
    }

    #[test]
    fn test_priority_definition_error_is_error() {
        let error = PriorityDefinitionError::new("Test error");
        let _: &dyn std::error::Error = &error;
    }

    #[test]
    fn test_priority_definition_error_clone() {
        let error1 = PriorityDefinitionError::new("Priority error");
        let error2 = error1.clone();
        assert_eq!(error1.message(), error2.message());
    }

    #[test]
    fn test_priority_definition_error_from_string() {
        let error = PriorityDefinitionError::new("Error message".to_string());
        assert_eq!(error.message(), "Error message");
    }
}
