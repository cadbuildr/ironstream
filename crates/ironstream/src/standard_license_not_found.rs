// FILE: standard_license_not_found.rs
// occt: Standard_LicenseNotFound

//! Exception thrown when a license file is not found.
//! Inherits from Standard_LicenseError.

use std::fmt;

/// Exception thrown when a license file is not found.
/// Inherits from Standard_LicenseError.
#[derive(Debug, Clone)]
pub struct StandardLicenseNotFound {
    message: String,
}

impl StandardLicenseNotFound {
    pub fn new(message: impl Into<String>) -> Self {
        StandardLicenseNotFound {
            message: message.into(),
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn exception_type(&self) -> &str {
        "Standard_LicenseNotFound"
    }
}

impl fmt::Display for StandardLicenseNotFound {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.exception_type(), self.message)
    }
}

impl std::error::Error for StandardLicenseNotFound {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_license_not_found_creation() {
        let err = StandardLicenseNotFound::new("License file not found");
        assert_eq!(err.message(), "License file not found");
        assert_eq!(err.exception_type(), "Standard_LicenseNotFound");
    }

    #[test]
    fn test_license_not_found_display() {
        let err = StandardLicenseNotFound::new("/path/to/license.lic");
        assert_eq!(err.to_string(), "Standard_LicenseNotFound: /path/to/license.lic");
    }

    #[test]
    fn test_license_not_found_error_trait() {
        let err = StandardLicenseNotFound::new("Not found");
        let _e: &dyn std::error::Error = &err;
    }

    #[test]
    fn test_license_not_found_clone() {
        let err1 = StandardLicenseNotFound::new("Original");
        let err2 = err1.clone();
        assert_eq!(err1.message(), err2.message());
    }
}
