// FILE: step_visual_composite_text_with_extent.rs
// occt: StepVisual_CompositeTextWithExtent

/// A composite text with extent in STEP representation.
///
/// This extends composite text with a planar extent defining the text area.
pub struct CompositeTextWithExtent {
    name: String,
    extent_width: f64,
    extent_height: f64,
}

impl CompositeTextWithExtent {
    /// Creates a new composite text with extent.
    pub fn new(name: String) -> Self {
        CompositeTextWithExtent {
            name,
            extent_width: 0.0,
            extent_height: 0.0,
        }
    }

    /// Returns the name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Sets the extent width.
    pub fn set_extent_width(&mut self, width: f64) {
        self.extent_width = width;
    }

    /// Returns the extent width.
    pub fn extent_width(&self) -> f64 {
        self.extent_width
    }

    /// Sets the extent height.
    pub fn set_extent_height(&mut self, height: f64) {
        self.extent_height = height;
    }

    /// Returns the extent height.
    pub fn extent_height(&self) -> f64 {
        self.extent_height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_composite_text_with_extent_new() {
        let text = CompositeTextWithExtent::new("MyTextWithExtent".to_string());
        assert_eq!(text.name(), "MyTextWithExtent");
        assert_eq!(text.extent_width(), 0.0);
        assert_eq!(text.extent_height(), 0.0);
    }

    #[test]
    fn test_extent_operations() {
        let mut text = CompositeTextWithExtent::new("Label".to_string());
        text.set_extent_width(100.0);
        text.set_extent_height(50.0);
        assert_eq!(text.extent_width(), 100.0);
        assert_eq!(text.extent_height(), 50.0);
    }
}
