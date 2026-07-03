// FILE: aspect_display_connection_definition_error.rs
// occt: Aspect_DisplayConnectionDefinitionError

/// Exception for invalid display connection definition.
#[derive(Debug, Clone)]
pub struct AspectDisplayConnectionDefinitionError {
    message: String,
}

impl AspectDisplayConnectionDefinitionError {
    /// Create exception with message.
    pub fn new(message: impl Into<String>) -> Self {
        AspectDisplayConnectionDefinitionError {
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
        let err = AspectDisplayConnectionDefinitionError::new("no display");
        assert_eq!(err.message(), "no display");
    }
}
