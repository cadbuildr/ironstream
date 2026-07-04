// FILE: step_visual_curve_style.rs
// occt: StepVisual_CurveStyle

/// A curve style in STEP representation.
///
/// This defines the visual styling of curves.
pub struct CurveStyle {
    name: String,
    width: Option<f64>,
    colour: Option<String>,
}

impl CurveStyle {
    /// Creates a new curve style.
    pub fn new(name: String) -> Self {
        CurveStyle {
            name,
            width: None,
            colour: None,
        }
    }

    /// Returns the name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Sets the width.
    pub fn set_width(&mut self, width: f64) {
        self.width = Some(width);
    }

    /// Returns the width.
    pub fn width(&self) -> Option<f64> {
        self.width
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
    fn test_curve_style_new() {
        let style = CurveStyle::new("DefaultStyle".to_string());
        assert_eq!(style.name(), "DefaultStyle");
        assert_eq!(style.width(), None);
        assert_eq!(style.colour(), None);
    }

    #[test]
    fn test_set_width() {
        let mut style = CurveStyle::new("MyStyle".to_string());
        style.set_width(2.5);
        assert_eq!(style.width(), Some(2.5));
    }

    #[test]
    fn test_set_colour() {
        let mut style = CurveStyle::new("ColoredStyle".to_string());
        style.set_colour("Red".to_string());
        assert_eq!(style.colour(), Some("Red"));
    }
}
