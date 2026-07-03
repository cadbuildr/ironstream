// FILE: storage_stream_ext_char_parity_error.rs
// occt: Storage_StreamExtCharParityError

//! Exception for extended character parity errors in streams
//! Inherits from Storage_StreamReadError in OCCT

use std::fmt;

/// Exception raised when an extended character parity error occurs during stream read
#[derive(Debug, Clone)]
pub struct StorageStreamExtCharParityError {
    message: String,
}

impl StorageStreamExtCharParityError {
    /// Create a new extended character parity error with a message
    pub fn new(message: impl Into<String>) -> Self {
        StorageStreamExtCharParityError {
            message: message.into(),
        }
    }

    /// Get the error message
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for StorageStreamExtCharParityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Storage stream extended character parity error: {}", self.message)
    }
}

impl std::error::Error for StorageStreamExtCharParityError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ext_char_parity_error_creation() {
        let err = StorageStreamExtCharParityError::new("parity check failed");
        assert_eq!(err.message(), "parity check failed");
    }

    #[test]
    fn test_ext_char_parity_error_display() {
        let err = StorageStreamExtCharParityError::new("invalid parity");
        let s = format!("{}", err);
        assert!(s.contains("invalid parity"));
    }

    #[test]
    fn test_ext_char_parity_error_clone() {
        let err = StorageStreamExtCharParityError::new("parity error");
        let err_clone = err.clone();
        assert_eq!(err.message(), err_clone.message());
    }
}
