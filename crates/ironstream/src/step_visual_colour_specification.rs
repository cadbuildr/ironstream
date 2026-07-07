// FILE: step_visual_colour_specification.rs
// occt: StepVisual_ColourSpecification

/// A STEP colour specification.
///
/// This class represents a colour specification in STEP data exchange,
/// inheriting from Colour and holding a name string.
pub struct ColourSpecification {
    name: Option<String>,
}

impl ColourSpecification {
    /// Creates a new colour specification.
    pub fn new() -> Self {
        ColourSpecification { name: None }
    }

    /// Initializes the colour specification with a name.
    pub fn init(&mut self, name: String) {
        self.name = Some(name);
    }

    /// Sets the name.
    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    /// Gets the name.
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
}

impl Default for ColourSpecification {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_colour_specification_new() {
        let spec = ColourSpecification::new();
        assert!(spec.name().is_none());
    }

    #[test]
    fn test_colour_specification_init() {
        let mut spec = ColourSpecification::new();
        spec.init("Red".to_string());
        assert_eq!(spec.name(), Some("Red"));
    }

    #[test]
    fn test_colour_specification_set_name() {
        let mut spec = ColourSpecification::new();
        spec.set_name("Green".to_string());
        assert_eq!(spec.name(), Some("Green"));
    }

    #[test]
    fn test_colour_specification_default() {
        let spec = ColourSpecification::default();
        assert!(spec.name().is_none());
    }
}
