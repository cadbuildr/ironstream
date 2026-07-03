// FILE: standard_no_such_object.rs
// occt: Standard_NoSuchObject

//! Exception thrown when a searched object is not found.

use std::fmt;

#[derive(Debug, Clone)]
pub struct StandardNoSuchObject {
    message: String,
}

impl StandardNoSuchObject {
    pub fn new(message: impl Into<String>) -> Self {
        StandardNoSuchObject {
            message: message.into(),
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn exception_type(&self) -> &str {
        "Standard_NoSuchObject"
    }
}

impl fmt::Display for StandardNoSuchObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.exception_type(), self.message)
    }
}

impl std::error::Error for StandardNoSuchObject {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_such_object_creation() {
        let err = StandardNoSuchObject::new("Object not found");
        assert_eq!(err.message(), "Object not found");
    }

    #[test]
    fn test_no_such_object_display() {
        let err = StandardNoSuchObject::new("Searched object");
        assert_eq!(err.to_string(), "Standard_NoSuchObject: Searched object");
    }

    #[test]
    fn test_no_such_object_clone() {
        let err = StandardNoSuchObject::new("Test");
        let err2 = err.clone();
        assert_eq!(err.message(), err2.message());
    }
}
