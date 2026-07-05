// FILE: draw_failure.rs
// occt: Draw_Failure

//! Exception class for Draw application errors.

use std::fmt;

/// Draw application failure exception
#[derive(Debug, Clone)]
pub struct DrawFailure {
    message: String,
}

impl DrawFailure {
    /// Create a new DrawFailure with a message
    pub fn new(message: impl Into<String>) -> Self {
        DrawFailure {
            message: message.into(),
        }
    }

    /// Get the error message
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for DrawFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Draw_Failure: {}", self.message)
    }
}

impl std::error::Error for DrawFailure {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_draw_failure_creation() {
        let failure = DrawFailure::new("Test error");
        assert_eq!(failure.message(), "Test error");
    }

    #[test]
    fn test_draw_failure_display() {
        let failure = DrawFailure::new("Something went wrong");
        assert_eq!(
            format!("{}", failure),
            "Draw_Failure: Something went wrong"
        );
    }

    #[test]
    fn test_draw_failure_string_conversion() {
        let failure = DrawFailure::new("String message".to_string());
        assert_eq!(failure.message(), "String message");
    }

    #[test]
    fn test_draw_failure_error_trait() {
        let failure = DrawFailure::new("Error trait test");
        let _err: &dyn std::error::Error = &failure;
        // Verify it implements Error trait
    }
}
