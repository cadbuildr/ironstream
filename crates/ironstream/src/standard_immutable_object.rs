// FILE: standard_immutable_object.rs
// occt: Standard_ImmutableObject

//! Exception thrown when attempting to modify an immutable object.
//! This exception inherits from Standard_DomainError.

use std::fmt;

/// Base exception class for domain-related errors
#[derive(Debug, Clone)]
pub struct StandardDomainError {
    message: String,
}

impl StandardDomainError {
    pub fn new(message: impl Into<String>) -> Self {
        StandardDomainError {
            message: message.into(),
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for StandardDomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Standard_DomainError: {}", self.message)
    }
}

impl std::error::Error for StandardDomainError {}

/// Exception thrown when attempting to modify an immutable object.
/// Inherits from Standard_DomainError.
#[derive(Debug, Clone)]
pub struct StandardImmutableObject {
    message: String,
}

impl StandardImmutableObject {
    pub fn new(message: impl Into<String>) -> Self {
        StandardImmutableObject {
            message: message.into(),
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn exception_type(&self) -> &str {
        "Standard_ImmutableObject"
    }
}

impl fmt::Display for StandardImmutableObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.exception_type(), self.message)
    }
}

impl std::error::Error for StandardImmutableObject {}

impl From<StandardImmutableObject> for StandardDomainError {
    fn from(err: StandardImmutableObject) -> Self {
        StandardDomainError::new(err.message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_immutable_object_creation() {
        let err = StandardImmutableObject::new("Object is immutable");
        assert_eq!(err.message(), "Object is immutable");
        assert_eq!(err.exception_type(), "Standard_ImmutableObject");
    }

    #[test]
    fn test_immutable_object_display() {
        let err = StandardImmutableObject::new("Cannot modify");
        assert_eq!(err.to_string(), "Standard_ImmutableObject: Cannot modify");
    }

    #[test]
    fn test_domain_error_creation() {
        let err = StandardDomainError::new("Domain error occurred");
        assert_eq!(err.message(), "Domain error occurred");
    }

    #[test]
    fn test_immutable_to_domain_error_conversion() {
        let immutable_err = StandardImmutableObject::new("Immutable");
        let domain_err: StandardDomainError = immutable_err.into();
        assert_eq!(domain_err.message(), "Immutable");
    }
}
