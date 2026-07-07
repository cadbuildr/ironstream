// FILE: step_visual_fill_area_style.rs
// occt: StepVisual_FillAreaStyle

/// A fill area style in STEP representation.
///
/// This defines how areas are filled/shaded.
pub struct FillAreaStyle {
    name: String,
    fill_type: String,
}

impl FillAreaStyle {
    /// Creates a new fill area style.
    pub fn new(name: String) -> Self {
        FillAreaStyle {
            name,
            fill_type: String::new(),
        }
    }

    /// Returns the name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Sets the fill type.
    pub fn set_fill_type(&mut self, fill_type: String) {
        self.fill_type = fill_type;
    }

    /// Returns the fill type.
    pub fn fill_type(&self) -> &str {
        &self.fill_type
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fill_area_style_new() {
        let style = FillAreaStyle::new("DefaultFill".to_string());
        assert_eq!(style.name(), "DefaultFill");
        assert_eq!(style.fill_type(), "");
    }

    #[test]
    fn test_set_fill_type() {
        let mut style = FillAreaStyle::new("Style".to_string());
        style.set_fill_type("Solid".to_string());
        assert_eq!(style.fill_type(), "Solid");
    }
}
