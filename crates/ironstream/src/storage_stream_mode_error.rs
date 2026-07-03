// FILE: storage_stream_mode_error.rs
// occt: Storage_StreamModeError

//! Exception for stream mode errors
//! Inherits from Standard_Failure in OCCT

use std::fmt;

/// Exception raised when a stream mode error occurs
#[derive(Debug, Clone)]
pub struct StorageStreamModeError {
    message: String,
}

impl StorageStreamModeError {
    /// Create a new stream mode error with a message
    pub fn new(message: impl Into<String>) -> Self {
        StorageStreamModeError {
            message: message.into(),
        }
    }

    /// Get the error message
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for StorageStreamModeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Storage stream mode error: {}", self.message)
    }
}

impl std::error::Error for StorageStreamModeError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storage_stream_mode_error_creation() {
        let err = StorageStreamModeError::new("invalid mode");
        assert_eq!(err.message(), "invalid mode");
    }

    #[test]
    fn test_storage_stream_mode_error_display() {
        let err = StorageStreamModeError::new("mode not allowed");
        let s = format!("{}", err);
        assert!(s.contains("mode not allowed"));
    }

    #[test]
    fn test_storage_stream_mode_error_clone() {
        let err = StorageStreamModeError::new("bad mode");
        let err_clone = err.clone();
        assert_eq!(err.message(), err_clone.message());
    }
}
