// FILE: step_visual_point_style.rs
// occt: StepVisual_PointStyle

/// A point style in STEP representation.
///
/// This defines the visual style for points.
pub struct PointStyle {
    name: String,
    size: f64,
    colour: Option<String>,
}

impl PointStyle {
    /// Creates a new point style.
    pub fn new(name: String) -> Self {
        PointStyle {
            name,
            size: 1.0,
            colour: None,
        }
    }

    /// Returns the name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Sets the size.
    pub fn set_size(&mut self, size: f64) {
        self.size = size;
    }

    /// Returns the size.
    pub fn size(&self) -> f64 {
        self.size
    }

    /// Sets the colour.
    pub fn set_colour(&mut self, colour: String) {
        self.colour = Some(colour);
    }

    /// Returns the colour.
    pub fn colour(&self) -> Option<&str> {
        self.colour.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_style_new() {
        let style = PointStyle::new("DefaultPointStyle".to_string());
        assert_eq!(style.name(), "DefaultPointStyle");
        assert_eq!(style.size(), 1.0);
        assert_eq!(style.colour(), None);
    }

    #[test]
    fn test_set_size() {
        let mut style = PointStyle::new("PointStyle".to_string());
        style.set_size(2.5);
        assert_eq!(style.size(), 2.5);
    }

    #[test]
    fn test_set_colour() {
        let mut style = PointStyle::new("ColoredPoints".to_string());
        style.set_colour("Blue".to_string());
        assert_eq!(style.colour(), Some("Blue"));
    }
}
