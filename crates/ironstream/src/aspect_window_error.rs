// FILE: aspect_window_error.rs
// occt: Aspect_WindowError

use std::fmt;

/// Exception raised when an error occurs with window operations.
/// In Rust, we represent this as a custom error type that can be thrown.
#[derive(Debug, Clone)]
pub struct AspectWindowError {
    message: String,
}

impl AspectWindowError {
    /// Create a new window error with the given message.
    pub fn new(message: impl Into<String>) -> Self {
        AspectWindowError {
            message: message.into(),
        }
    }

    /// Get the error message.
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for AspectWindowError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Aspect_WindowError: {}", self.message)
    }
}

impl std::error::Error for AspectWindowError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_window_error_creation() {
        let err = AspectWindowError::new("Invalid window handle");
        assert_eq!(err.message(), "Invalid window handle");
    }

    #[test]
    fn test_window_error_display() {
        let err = AspectWindowError::new("Window not found");
        let msg = format!("{}", err);
        assert!(msg.contains("Aspect_WindowError"));
        assert!(msg.contains("Window not found"));
    }

    #[test]
    fn test_window_error_is_error() {
        let err = AspectWindowError::new("Test error");
        let _: &dyn std::error::Error = &err;
    }
}
