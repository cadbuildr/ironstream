// FILE: standard_not_implemented.rs
// occt: Standard_NotImplemented

//! Exception thrown when a method or feature is not implemented.

use std::fmt;

#[derive(Debug, Clone)]
pub struct StandardNotImplemented {
    message: String,
}

impl StandardNotImplemented {
    pub fn new(message: impl Into<String>) -> Self {
        StandardNotImplemented {
            message: message.into(),
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn exception_type(&self) -> &str {
        "Standard_NotImplemented"
    }
}

impl fmt::Display for StandardNotImplemented {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.exception_type(), self.message)
    }
}

impl std::error::Error for StandardNotImplemented {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_implemented_creation() {
        let err = StandardNotImplemented::new("Feature not yet implemented");
        assert_eq!(err.message(), "Feature not yet implemented");
    }

    #[test]
    fn test_not_implemented_display() {
        let err = StandardNotImplemented::new("Method X");
        assert_eq!(err.to_string(), "Standard_NotImplemented: Method X");
    }

    #[test]
    fn test_not_implemented_clone() {
        let err = StandardNotImplemented::new("Test");
        let err2 = err.clone();
        assert_eq!(err.message(), err2.message());
    }
}
