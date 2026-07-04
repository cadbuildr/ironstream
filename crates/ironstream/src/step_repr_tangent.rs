// FILE: step_repr_tangent.rs
// occt: StepRepr_Tangent

/// Represents a tangent shape aspect for dimensional tolerances (STEP).
/// This is derived from DerivedShapeAspect.
pub struct Tangent {
    name: Option<String>,
    description: Option<String>,
}

impl Tangent {
    /// Create a new Tangent
    pub fn new() -> Self {
        Tangent {
            name: None,
            description: None,
        }
    }

    /// Initialize tangent with name and optional description
    pub fn init(&mut self, name: String, description: Option<String>) {
        self.name = Some(name);
        self.description = description;
    }

    /// Get the name
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Set the name
    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    /// Get the description
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Set the description
    pub fn set_description(&mut self, description: String) {
        self.description = Some(description);
    }
}

impl Default for Tangent {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let tang = Tangent::new();
        assert_eq!(tang.name(), None);
        assert_eq!(tang.description(), None);
    }

    #[test]
    fn test_init() {
        let mut tang = Tangent::new();
        tang.init(
            "TangentAspect".to_string(),
            Some("Tangent description".to_string()),
        );
        assert_eq!(tang.name(), Some("TangentAspect"));
        assert_eq!(tang.description(), Some("Tangent description"));
    }

    #[test]
    fn test_set_and_get_name() {
        let mut tang = Tangent::new();
        tang.set_name("TestTangent".to_string());
        assert_eq!(tang.name(), Some("TestTangent"));
    }

    #[test]
    fn test_set_and_get_description() {
        let mut tang = Tangent::new();
        tang.set_description("Test Desc".to_string());
        assert_eq!(tang.description(), Some("Test Desc"));
    }
}
