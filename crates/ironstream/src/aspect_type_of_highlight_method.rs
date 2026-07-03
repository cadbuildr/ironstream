// FILE: aspect_type_of_highlight_method.rs
// occt: Aspect_TypeOfHighlightMethod

/// Method for highlighting selected objects.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AspectTypeOfHighlightMethod {
    /// Color replacement.
    Color = 0,
    /// Bounding box.
    BoundingBox = 1,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_values() {
        assert_eq!(AspectTypeOfHighlightMethod::Color as i32, 0);
        assert_eq!(AspectTypeOfHighlightMethod::BoundingBox as i32, 1);
    }
}
