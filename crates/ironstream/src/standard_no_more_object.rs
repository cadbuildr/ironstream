// FILE: standard_no_more_object.rs
// occt: Standard_NoMoreObject

//! Exception thrown when iteration over a collection has ended.

use std::fmt;

#[derive(Debug, Clone)]
pub struct StandardNoMoreObject {
    message: String,
}

impl StandardNoMoreObject {
    pub fn new(message: impl Into<String>) -> Self {
        StandardNoMoreObject {
            message: message.into(),
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn exception_type(&self) -> &str {
        "Standard_NoMoreObject"
    }
}

impl fmt::Display for StandardNoMoreObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.exception_type(), self.message)
    }
}

impl std::error::Error for StandardNoMoreObject {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_more_object_creation() {
        let err = StandardNoMoreObject::new("No more objects in collection");
        assert_eq!(err.message(), "No more objects in collection");
        assert_eq!(err.exception_type(), "Standard_NoMoreObject");
    }

    #[test]
    fn test_no_more_object_display() {
        let err = StandardNoMoreObject::new("End of iteration");
        assert_eq!(err.to_string(), "Standard_NoMoreObject: End of iteration");
    }

    #[test]
    fn test_no_more_object_clone() {
        let err1 = StandardNoMoreObject::new("Original");
        let err2 = err1.clone();
        assert_eq!(err1.message(), err2.message());
    }
}
