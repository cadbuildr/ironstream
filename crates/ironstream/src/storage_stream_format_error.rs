// FILE: storage_stream_format_error.rs
// occt: Storage_StreamFormatError

//! Exception for stream format errors
//! Inherits from Standard_Failure in OCCT

use std::fmt;

/// Exception raised when a stream format error occurs
#[derive(Debug, Clone)]
pub struct StorageStreamFormatError {
    message: String,
}

impl StorageStreamFormatError {
    /// Create a new stream format error with a message
    pub fn new(message: impl Into<String>) -> Self {
        StorageStreamFormatError {
            message: message.into(),
        }
    }

    /// Get the error message
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for StorageStreamFormatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Storage stream format error: {}", self.message)
    }
}

impl std::error::Error for StorageStreamFormatError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storage_stream_format_error_creation() {
        let err = StorageStreamFormatError::new("invalid format");
        assert_eq!(err.message(), "invalid format");
    }

    #[test]
    fn test_storage_stream_format_error_display() {
        let err = StorageStreamFormatError::new("format mismatch");
        let s = format!("{}", err);
        assert!(s.contains("format mismatch"));
    }

    #[test]
    fn test_storage_stream_format_error_clone() {
        let err = StorageStreamFormatError::new("bad format");
        let err_clone = err.clone();
        assert_eq!(err.message(), err_clone.message());
    }
}
