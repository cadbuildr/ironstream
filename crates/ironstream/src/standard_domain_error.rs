// FILE: standard_domain_error.rs
// occt: Standard_DomainError

/// Exception class for domain errors
#[derive(Debug, Clone)]
pub struct StandardDomainError {
    message: String,
}

impl StandardDomainError {
    /// Creates a new StandardDomainError exception with a message
    pub fn new(message: impl Into<String>) -> Self {
        StandardDomainError {
            message: message.into(),
        }
    }

    /// Gets the error message
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl std::fmt::Display for StandardDomainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Standard_DomainError: {}", self.message)
    }
}

impl std::error::Error for StandardDomainError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standard_domain_error() {
        let err = StandardDomainError::new("invalid domain");
        assert_eq!(err.message(), "invalid domain");
    }

    #[test]
    fn test_standard_domain_error_display() {
        let err = StandardDomainError::new("out of range");
        assert_eq!(err.to_string(), "Standard_DomainError: out of range");
    }
}
