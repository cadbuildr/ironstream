// FILE: aspect_ident_definition_error.rs
// occt: Aspect_IdentDefinitionError

/// Exception for invalid identifier definition.
#[derive(Debug, Clone)]
pub struct AspectIdentDefinitionError {
    message: String,
}

impl AspectIdentDefinitionError {
    /// Create exception with message.
    pub fn new(message: impl Into<String>) -> Self {
        AspectIdentDefinitionError {
            message: message.into(),
        }
    }

    /// Get error message.
    pub fn message(&self) -> &str {
        &self.message
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let err = AspectIdentDefinitionError::new("invalid ident");
        assert_eq!(err.message(), "invalid ident");
    }
}
