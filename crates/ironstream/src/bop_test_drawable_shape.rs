// FILE: bop_test_drawable_shape.rs
// occt: BOPTest_DrawableShape

//! Drawable shape for BOP testing.
//! BOPTest_DrawableShape extends DBRep_DrawableShape to provide
//! visualization of shapes with text annotations in the BOP test framework.

/// Represents a drawable shape for BOP testing with text annotation capability.
pub struct BopTestDrawableShape {
    text: Option<String>,
    text_color: u32,
}

impl BopTestDrawableShape {
    /// Creates a new drawable shape with text annotation.
    pub fn new(text: Option<String>, text_color: u32) -> Self {
        BopTestDrawableShape { text, text_color }
    }

    /// Gets the text annotation.
    pub fn text(&self) -> Option<&str> {
        self.text.as_deref()
    }

    /// Gets the text color.
    pub fn text_color(&self) -> u32 {
        self.text_color
    }

    /// Sets the text annotation.
    pub fn set_text(&mut self, text: Option<String>) {
        self.text = text;
    }

    /// Sets the text color.
    pub fn set_text_color(&mut self, color: u32) {
        self.text_color = color;
    }

    /// Draws the shape (placeholder for rendering).
    pub fn draw_on(&self) {
        // Placeholder for draw operation
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let shape = BopTestDrawableShape::new(Some("test".to_string()), 0xFF0000);
        assert_eq!(shape.text(), Some("test"));
        assert_eq!(shape.text_color(), 0xFF0000);
    }

    #[test]
    fn test_set_text() {
        let mut shape = BopTestDrawableShape::new(None, 0);
        shape.set_text(Some("annotation".to_string()));
        assert_eq!(shape.text(), Some("annotation"));
    }

    #[test]
    fn test_set_text_color() {
        let mut shape = BopTestDrawableShape::new(None, 0);
        shape.set_text_color(0x00FF00);
        assert_eq!(shape.text_color(), 0x00FF00);
    }

    #[test]
    fn test_draw_on() {
        let shape = BopTestDrawableShape::new(Some("test".to_string()), 0);
        shape.draw_on();
    }
}
