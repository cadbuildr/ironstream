// FILE: standard_license_error.rs
// occt: Standard_LicenseError

//! Exception thrown when a license error occurs.
//! Inherits from Standard_Failure.

use std::fmt;

/// Base exception class for all failures
#[derive(Debug, Clone)]
pub struct StandardFailure {
    message: String,
}

impl StandardFailure {
    pub fn new(message: impl Into<String>) -> Self {
        StandardFailure {
            message: message.into(),
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn exception_type(&self) -> &str {
        "Standard_Failure"
    }
}

impl fmt::Display for StandardFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.exception_type(), self.message)
    }
}

impl std::error::Error for StandardFailure {}

/// Exception thrown when a license error occurs.
/// Inherits from Standard_Failure.
#[derive(Debug, Clone)]
pub struct StandardLicenseError {
    message: String,
}

impl StandardLicenseError {
    pub fn new(message: impl Into<String>) -> Self {
        StandardLicenseError {
            message: message.into(),
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn exception_type(&self) -> &str {
        "Standard_LicenseError"
    }
}

impl fmt::Display for StandardLicenseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.exception_type(), self.message)
    }
}

impl std::error::Error for StandardLicenseError {}

impl From<StandardLicenseError> for StandardFailure {
    fn from(err: StandardLicenseError) -> Self {
        StandardFailure::new(err.message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_license_error_creation() {
        let err = StandardLicenseError::new("License verification failed");
        assert_eq!(err.message(), "License verification failed");
        assert_eq!(err.exception_type(), "Standard_LicenseError");
    }

    #[test]
    fn test_license_error_display() {
        let err = StandardLicenseError::new("Invalid license");
        assert_eq!(err.to_string(), "Standard_LicenseError: Invalid license");
    }

    #[test]
    fn test_failure_creation() {
        let err = StandardFailure::new("Generic failure");
        assert_eq!(err.message(), "Generic failure");
    }

    #[test]
    fn test_license_error_to_failure_conversion() {
        let license_err = StandardLicenseError::new("License error");
        let failure: StandardFailure = license_err.into();
        assert_eq!(failure.message(), "License error");
    }
}
