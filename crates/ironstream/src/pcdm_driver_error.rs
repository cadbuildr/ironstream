// FILE: pcdm_driver_error.rs
// occt: PCDM_DriverError

use std::fmt;

/// Exception thrown when a persistence driver encounters an error
#[derive(Debug, Clone)]
pub struct PCDMDriverError {
    message: String,
}

impl PCDMDriverError {
    /// Create a new driver error
    pub fn new(message: &str) -> Self {
        PCDMDriverError {
            message: message.to_string(),
        }
    }

    /// Get the error message
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for PCDMDriverError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PCDM_DriverError: {}", self.message)
    }
}

impl std::error::Error for PCDMDriverError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_error() {
        let err = PCDMDriverError::new("test error");
        assert_eq!(err.message(), "test error");
    }

    #[test]
    fn test_error_display() {
        let err = PCDMDriverError::new("driver failed");
        assert_eq!(format!("{}", err), "PCDM_DriverError: driver failed");
    }

    #[test]
    fn test_error_clone() {
        let err1 = PCDMDriverError::new("original");
        let err2 = err1.clone();
        assert_eq!(err2.message(), "original");
    }
}
