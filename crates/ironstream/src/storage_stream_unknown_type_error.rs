// FILE: storage_stream_unknown_type_error.rs
// occt: Storage_StreamUnknownTypeError

//! Exception for unknown type errors in streams
//! Inherits from Storage_StreamReadError in OCCT

use std::fmt;

/// Exception raised when an unknown type error occurs during stream read
#[derive(Debug, Clone)]
pub struct StorageStreamUnknownTypeError {
    message: String,
}

impl StorageStreamUnknownTypeError {
    /// Create a new unknown type error with a message
    pub fn new(message: impl Into<String>) -> Self {
        StorageStreamUnknownTypeError {
            message: message.into(),
        }
    }

    /// Get the error message
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for StorageStreamUnknownTypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Storage stream unknown type error: {}", self.message)
    }
}

impl std::error::Error for StorageStreamUnknownTypeError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unknown_type_error_creation() {
        let err = StorageStreamUnknownTypeError::new("unknown type 'Foo'");
        assert_eq!(err.message(), "unknown type 'Foo'");
    }

    #[test]
    fn test_unknown_type_error_display() {
        let err = StorageStreamUnknownTypeError::new("unregistered type");
        let s = format!("{}", err);
        assert!(s.contains("unregistered type"));
    }

    #[test]
    fn test_unknown_type_error_clone() {
        let err = StorageStreamUnknownTypeError::new("unknown type");
        let err_clone = err.clone();
        assert_eq!(err.message(), err_clone.message());
    }
}
