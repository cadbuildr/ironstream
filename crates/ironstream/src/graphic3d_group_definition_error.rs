// FILE: graphic3d_group_definition_error.rs
// occt: Graphic3d_GroupDefinitionError

use core::fmt;

/// Exception thrown when a group definition error occurs
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GroupDefinitionError {
    message: String,
}

impl GroupDefinitionError {
    /// Creates a new GroupDefinitionError with the given message
    pub fn new(message: &str) -> Self {
        GroupDefinitionError {
            message: message.to_string(),
        }
    }

    /// Returns the error message
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for GroupDefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Group Definition Error: {}", self.message)
    }
}

impl std::error::Error for GroupDefinitionError {}

/// Macro for raising GroupDefinitionError if condition is true
#[macro_export]
macro_rules! raise_group_definition_error_if {
    ($condition:expr, $message:expr) => {
        if $condition {
            return Err($crate::graphic3d_group_definition_error::GroupDefinitionError::new($message));
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_definition_error_creation() {
        let error = GroupDefinitionError::new("Test error message");
        assert_eq!(error.message(), "Test error message");
    }

    #[test]
    fn test_group_definition_error_display() {
        let error = GroupDefinitionError::new("Invalid group definition");
        let display_str = format!("{}", error);
        assert!(display_str.contains("Invalid group definition"));
        assert!(display_str.contains("Group Definition Error"));
    }

    #[test]
    fn test_group_definition_error_clone() {
        let error = GroupDefinitionError::new("Original error");
        let cloned = error.clone();
        assert_eq!(error, cloned);
    }

    #[test]
    fn test_group_definition_error_debug() {
        let error = GroupDefinitionError::new("Debug test");
        let debug_str = format!("{:?}", error);
        assert!(debug_str.contains("GroupDefinitionError"));
        assert!(debug_str.contains("Debug test"));
    }

    #[test]
    fn test_group_definition_error_empty_message() {
        let error = GroupDefinitionError::new("");
        assert_eq!(error.message(), "");
    }

    #[test]
    fn test_group_definition_error_is_error() {
        let error = GroupDefinitionError::new("Test");
        let _: &dyn std::error::Error = &error;
    }
}
