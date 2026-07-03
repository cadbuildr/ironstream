// FILE: standard_program_error.rs
// occt: Standard_ProgramError

//! Exception thrown for program logic errors.

use std::fmt;

#[derive(Debug, Clone)]
pub struct StandardProgramError {
    message: String,
}

impl StandardProgramError {
    pub fn new(message: impl Into<String>) -> Self {
        StandardProgramError {
            message: message.into(),
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn exception_type(&self) -> &str {
        "Standard_ProgramError"
    }
}

impl fmt::Display for StandardProgramError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.exception_type(), self.message)
    }
}

impl std::error::Error for StandardProgramError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_program_error_creation() {
        let err = StandardProgramError::new("Program logic error");
        assert_eq!(err.message(), "Program logic error");
    }

    #[test]
    fn test_program_error_display() {
        let err = StandardProgramError::new("Invalid state");
        assert_eq!(err.to_string(), "Standard_ProgramError: Invalid state");
    }

    #[test]
    fn test_program_error_clone() {
        let err = StandardProgramError::new("Test");
        let err2 = err.clone();
        assert_eq!(err.message(), err2.message());
    }
}
