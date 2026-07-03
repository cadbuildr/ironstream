// FILE: aspect_graphic_device_definition_error.rs
// occt: Aspect_GraphicDeviceDefinitionError

/// Exception for invalid graphics device definition.
#[derive(Debug, Clone)]
pub struct AspectGraphicDeviceDefinitionError {
    message: String,
}

impl AspectGraphicDeviceDefinitionError {
    /// Create exception with message.
    pub fn new(message: impl Into<String>) -> Self {
        AspectGraphicDeviceDefinitionError {
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
        let err = AspectGraphicDeviceDefinitionError::new("invalid device");
        assert_eq!(err.message(), "invalid device");
    }
}
