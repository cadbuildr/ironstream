// FILE: storage_stream_type_mismatch_error.rs
// occt: Storage_StreamTypeMismatchError

//! Exception for type mismatch errors in streams
//! Inherits from Storage_StreamReadError in OCCT

use std::fmt;

/// Exception raised when a type mismatch error occurs during stream read
#[derive(Debug, Clone)]
pub struct StorageStreamTypeMismatchError {
    message: String,
}

impl StorageStreamTypeMismatchError {
    /// Create a new type mismatch error with a message
    pub fn new(message: impl Into<String>) -> Self {
        StorageStreamTypeMismatchError {
            message: message.into(),
        }
    }

    /// Get the error message
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for StorageStreamTypeMismatchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Storage stream type mismatch error: {}", self.message)
    }
}

impl std::error::Error for StorageStreamTypeMismatchError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_mismatch_error_creation() {
        let err = StorageStreamTypeMismatchError::new("expected Int, got String");
        assert_eq!(err.message(), "expected Int, got String");
    }

    #[test]
    fn test_type_mismatch_error_display() {
        let err = StorageStreamTypeMismatchError::new("type mismatch");
        let s = format!("{}", err);
        assert!(s.contains("type mismatch"));
    }

    #[test]
    fn test_type_mismatch_error_clone() {
        let err = StorageStreamTypeMismatchError::new("type error");
        let err_clone = err.clone();
        assert_eq!(err.message(), err_clone.message());
    }
}
