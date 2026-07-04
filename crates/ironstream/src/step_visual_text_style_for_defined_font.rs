// FILE: step_visual_text_style_for_defined_font.rs
// occt: StepVisual_TextStyleForDefinedFont

/// Represents a STEP TextStyleForDefinedFont entity.
pub struct TextStyleForDefinedFont {
    text_colour: Option<Colour>,
}

/// Placeholder for Colour
pub struct Colour;

impl TextStyleForDefinedFont {
    /// Creates a new text style for defined font.
    pub fn new() -> Self {
        TextStyleForDefinedFont {
            text_colour: None,
        }
    }

    /// Initializes all fields.
    pub fn init(&mut self, text_colour: Option<Colour>) {
        self.text_colour = text_colour;
    }

    /// Returns the text colour.
    pub fn text_colour(&self) -> Option<&Colour> {
        self.text_colour.as_ref()
    }

    /// Sets the text colour.
    pub fn set_text_colour(&mut self, colour: Colour) {
        self.text_colour = Some(colour);
    }
}

impl Default for TextStyleForDefinedFont {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let tsdf = TextStyleForDefinedFont::new();
        assert!(tsdf.text_colour().is_none());
    }

    #[test]
    fn test_text_colour() {
        let mut tsdf = TextStyleForDefinedFont::new();
        let colour = Colour;
        tsdf.set_text_colour(colour);
        assert!(tsdf.text_colour().is_some());
    }
}
