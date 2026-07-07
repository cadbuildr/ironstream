// FILE: step_visual_curve_style_font_pattern.rs
// occt: StepVisual_CurveStyleFontPattern

/// A curve style font pattern in STEP representation.
///
/// This defines a pattern for curve style fonts.
pub struct CurveStyleFontPattern {
    visible_segment_length: f64,
    invisible_segment_length: f64,
}

impl CurveStyleFontPattern {
    /// Creates a new curve style font pattern.
    pub fn new() -> Self {
        CurveStyleFontPattern {
            visible_segment_length: 1.0,
            invisible_segment_length: 1.0,
        }
    }

    /// Sets the visible segment length.
    pub fn set_visible_segment_length(&mut self, length: f64) {
        self.visible_segment_length = length;
    }

    /// Returns the visible segment length.
    pub fn visible_segment_length(&self) -> f64 {
        self.visible_segment_length
    }

    /// Sets the invisible segment length.
    pub fn set_invisible_segment_length(&mut self, length: f64) {
        self.invisible_segment_length = length;
    }

    /// Returns the invisible segment length.
    pub fn invisible_segment_length(&self) -> f64 {
        self.invisible_segment_length
    }
}

impl Default for CurveStyleFontPattern {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_curve_style_font_pattern_new() {
        let pattern = CurveStyleFontPattern::new();
        assert_eq!(pattern.visible_segment_length(), 1.0);
        assert_eq!(pattern.invisible_segment_length(), 1.0);
    }

    #[test]
    fn test_set_segments() {
        let mut pattern = CurveStyleFontPattern::new();
        pattern.set_visible_segment_length(2.0);
        pattern.set_invisible_segment_length(1.0);
        assert_eq!(pattern.visible_segment_length(), 2.0);
        assert_eq!(pattern.invisible_segment_length(), 1.0);
    }
}
