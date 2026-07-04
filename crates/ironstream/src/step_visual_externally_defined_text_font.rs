// FILE: step_visual_externally_defined_text_font.rs
// occt: StepVisual_ExternallyDefinedTextFont

/// An externally defined text font in STEP representation.
///
/// This represents a text font defined by an external source.
pub struct ExternallyDefinedTextFont {
    name: String,
    source: String,
}

impl ExternallyDefinedTextFont {
    /// Creates a new externally defined text font.
    pub fn new(name: String) -> Self {
        ExternallyDefinedTextFont {
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
    fn test_externally_defined_text_font_new() {
        let font = ExternallyDefinedTextFont::new("TextFont".to_string());
        assert_eq!(font.name(), "TextFont");
        assert_eq!(font.source(), "");
    }

    #[test]
    fn test_set_source() {
        let mut font = ExternallyDefinedTextFont::new("Font".to_string());
        font.set_source("Arial".to_string());
        assert_eq!(font.source(), "Arial");
    }
}
