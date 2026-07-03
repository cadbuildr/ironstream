// FILE: aspect_aspect_fill_area_definition_error.rs
// occt: Aspect_AspectFillAreaDefinitionError

/// Exception for invalid fill area definition.
#[derive(Debug, Clone)]
pub struct AspectFillAreaDefinitionError {
    message: String,
}

impl AspectFillAreaDefinitionError {
    /// Create exception with message.
    pub fn new(message: impl Into<String>) -> Self {
        AspectFillAreaDefinitionError {
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
        let err = AspectFillAreaDefinitionError::new("invalid fill area");
        assert_eq!(err.message(), "invalid fill area");
    }
}
