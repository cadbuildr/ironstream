// FILE: aspect_aspect_line_definition_error.rs
// occt: Aspect_AspectLineDefinitionError

/// Exception for invalid line definition.
#[derive(Debug, Clone)]
pub struct AspectLineDefinitionError {
    message: String,
}

impl AspectLineDefinitionError {
    /// Create exception with message.
    pub fn new(message: impl Into<String>) -> Self {
        AspectLineDefinitionError {
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
        let err = AspectLineDefinitionError::new("invalid line");
        assert_eq!(err.message(), "invalid line");
    }
}
