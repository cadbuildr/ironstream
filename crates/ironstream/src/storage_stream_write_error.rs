// FILE: storage_stream_write_error.rs
// occt: Storage_StreamWriteError

//! Exception for stream write errors
//! Inherits from Standard_Failure in OCCT

use std::fmt;

/// Exception raised when a stream write error occurs
#[derive(Debug, Clone)]
pub struct StorageStreamWriteError {
    message: String,
}

impl StorageStreamWriteError {
    /// Create a new stream write error with a message
    pub fn new(message: impl Into<String>) -> Self {
        StorageStreamWriteError {
            message: message.into(),
        }
    }

    /// Get the error message
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for StorageStreamWriteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Storage stream write error: {}", self.message)
    }
}

impl std::error::Error for StorageStreamWriteError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storage_stream_write_error_creation() {
        let err = StorageStreamWriteError::new("write failed");
        assert_eq!(err.message(), "write failed");
    }

    #[test]
    fn test_storage_stream_write_error_display() {
        let err = StorageStreamWriteError::new("disk full");
        let s = format!("{}", err);
        assert!(s.contains("disk full"));
    }

    #[test]
    fn test_storage_stream_write_error_clone() {
        let err = StorageStreamWriteError::new("write error");
        let err_clone = err.clone();
        assert_eq!(err.message(), err_clone.message());
    }
}
