// FILE: step_element_surface_section_field.rs
// occt: StepElement_SurfaceSectionField

/// Base representation of STEP entity SurfaceSectionField.
/// This is an abstract base class with no direct instantiation.
#[derive(Clone)]
pub struct SurfaceSectionField;

impl SurfaceSectionField {
    /// Creates a new SurfaceSectionField.
    pub fn new() -> Self {
        Self
    }
}

impl Default for SurfaceSectionField {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let field = SurfaceSectionField::new();
        assert_eq!(std::mem::size_of_val(&field), 0);
    }

    #[test]
    fn test_default() {
        let field = SurfaceSectionField::default();
        assert_eq!(std::mem::size_of_val(&field), 0);
    }
}
