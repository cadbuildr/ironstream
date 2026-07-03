// FILE: storage_stream_read_error.rs
// occt: Storage_StreamReadError

//! Exception for stream read errors
//! Inherits from Standard_Failure in OCCT

use std::fmt;

/// Exception raised when a stream read error occurs
#[derive(Debug, Clone)]
pub struct StorageStreamReadError {
    message: String,
}

impl StorageStreamReadError {
    /// Create a new stream read error with a message
    pub fn new(message: impl Into<String>) -> Self {
        StorageStreamReadError {
            message: message.into(),
        }
    }

    /// Get the error message
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for StorageStreamReadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Storage stream read error: {}", self.message)
    }
}

impl std::error::Error for StorageStreamReadError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storage_stream_read_error_creation() {
        let err = StorageStreamReadError::new("test error");
        assert_eq!(err.message(), "test error");
    }

    #[test]
    fn test_storage_stream_read_error_display() {
        let err = StorageStreamReadError::new("read failed");
        let s = format!("{}", err);
        assert!(s.contains("read failed"));
    }

    #[test]
    fn test_storage_stream_read_error_clone() {
        let err = StorageStreamReadError::new("original");
        let err_clone = err.clone();
        assert_eq!(err.message(), err_clone.message());
    }

    #[test]
    fn test_storage_stream_read_error_is_error() {
        let err: Box<dyn std::error::Error> = Box::new(StorageStreamReadError::new("test"));
        assert!(!err.to_string().is_empty());
    }
}
