// FILE: aspect_aspect_marker_definition_error.rs
// occt: Aspect_AspectMarkerDefinitionError

/// Exception for invalid marker definition.
#[derive(Debug, Clone)]
pub struct AspectMarkerDefinitionError {
    message: String,
}

impl AspectMarkerDefinitionError {
    /// Create exception with message.
    pub fn new(message: impl Into<String>) -> Self {
        AspectMarkerDefinitionError {
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
        let err = AspectMarkerDefinitionError::new("invalid marker");
        assert_eq!(err.message(), "invalid marker");
    }
}
