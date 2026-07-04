// FILE: step_visual_externally_defined_curve_font.rs
// occt: StepVisual_ExternallyDefinedCurveFont

/// An externally defined curve font in STEP representation.
///
/// This represents a curve font defined by an external source.
pub struct ExternallyDefinedCurveFont {
    name: String,
    source: String,
}

impl ExternallyDefinedCurveFont {
    /// Creates a new externally defined curve font.
    pub fn new(name: String) -> Self {
        ExternallyDefinedCurveFont {
            name,
            source: String::new(),
        }
    }

    /// Returns the name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Sets the source.
    pub fn set_source(&mut self, source: String) {
        self.source = source;
    }

    /// Returns the source.
    pub fn source(&self) -> &str {
        &self.source
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_externally_defined_curve_font_new() {
        let font = ExternallyDefinedCurveFont::new("ExtFont".to_string());
        assert_eq!(font.name(), "ExtFont");
        assert_eq!(font.source(), "");
    }

    #[test]
    fn test_set_source() {
        let mut font = ExternallyDefinedCurveFont::new("Font".to_string());
        font.set_source("http://example.com".to_string());
        assert_eq!(font.source(), "http://example.com");
    }
}
