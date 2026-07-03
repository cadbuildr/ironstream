// FILE: graphic3d_material_definition_error.rs
// occt: Graphic3d_MaterialDefinitionError

use std::fmt;

/// Exception raised when material definition is invalid.
#[derive(Debug, Clone)]
pub struct Graphic3dMaterialDefinitionError {
    message: String,
}

impl Graphic3dMaterialDefinitionError {
    /// Creates a new material definition error with the given message.
    pub fn new(message: impl Into<String>) -> Self {
        Graphic3dMaterialDefinitionError {
            message: message.into(),
        }
    }

    /// Returns the error message.
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for Graphic3dMaterialDefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Material Definition Error: {}", self.message)
    }
}

impl std::error::Error for Graphic3dMaterialDefinitionError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_material_definition_error_creation() {
        let err = Graphic3dMaterialDefinitionError::new("Invalid material");
        assert_eq!(err.message(), "Invalid material");
    }

    #[test]
    fn test_material_definition_error_display() {
        let err = Graphic3dMaterialDefinitionError::new("Property out of range");
        let display = format!("{}", err);
        assert!(display.contains("Property out of range"));
    }

    #[test]
    fn test_material_definition_error_is_error() {
        let err = Graphic3dMaterialDefinitionError::new("Test");
        let _: &dyn std::error::Error = &err;
    }
}
