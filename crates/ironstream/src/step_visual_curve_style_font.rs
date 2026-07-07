// FILE: step_visual_curve_style_font.rs
// occt: StepVisual_CurveStyleFont

/// A curve style font in STEP representation.
///
/// This defines font properties for curve styles.
pub struct CurveStyleFont {
    name: String,
    pattern_scale: f64,
}

impl CurveStyleFont {
    /// Creates a new curve style font.
    pub fn new(name: String) -> Self {
        CurveStyleFont {
            name,
            pattern_scale: 1.0,
        }
    }

    /// Returns the name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Sets the pattern scale.
    pub fn set_pattern_scale(&mut self, scale: f64) {
        self.pattern_scale = scale;
    }

    /// Returns the pattern scale.
    pub fn pattern_scale(&self) -> f64 {
        self.pattern_scale
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_curve_style_font_new() {
        let font = CurveStyleFont::new("DefaultFont".to_string());
        assert_eq!(font.name(), "DefaultFont");
        assert_eq!(font.pattern_scale(), 1.0);
    }

    #[test]
    fn test_set_pattern_scale() {
        let mut font = CurveStyleFont::new("MyFont".to_string());
        font.set_pattern_scale(2.0);
        assert_eq!(font.pattern_scale(), 2.0);
    }
}
