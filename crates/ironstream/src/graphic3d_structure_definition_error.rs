// FILE: graphic3d_structure_definition_error.rs
// occt: Graphic3d_StructureDefinitionError

/// An exception type for structure definition errors in the 3D graphics pipeline.
///
/// This error indicates that a structure was not properly defined according to
/// the constraints of the graphics system.
#[derive(Debug, Clone)]
pub struct Graphic3dStructureDefinitionError {
    message: String,
}

impl Graphic3dStructureDefinitionError {
    /// Creates a new structure definition error with the given message.
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }

    /// Returns the error message.
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl std::fmt::Display for Graphic3dStructureDefinitionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Graphic3d Structure Definition Error: {}", self.message)
    }
}

impl std::error::Error for Graphic3dStructureDefinitionError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_error() {
        let error = Graphic3dStructureDefinitionError::new("Test error message");
        assert_eq!(error.message(), "Test error message");
    }

    #[test]
    fn test_error_display() {
        let error = Graphic3dStructureDefinitionError::new("Invalid structure");
        let display_str = format!("{}", error);
        assert!(display_str.contains("Invalid structure"));
    }

    #[test]
    fn test_error_cloning() {
        let error = Graphic3dStructureDefinitionError::new("Clone test");
        let cloned = error.clone();
        assert_eq!(error.message(), cloned.message());
    }
}
