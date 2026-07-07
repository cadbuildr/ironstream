// FILE: geometry_test_drawable_qualified_curve2d.rs
// occt: GeometryTest_DrawableQualifiedCurve2d

//! Drawable qualified curve in 2D for geometry testing.

#[derive(Clone, Debug)]
pub struct DrawableQualifiedCurve2d {
    pub curve_id: usize,
    pub qualifier: String,
}

impl DrawableQualifiedCurve2d {
    pub fn new(curve_id: usize, qualifier: String) -> Self {
        DrawableQualifiedCurve2d { curve_id, qualifier }
    }

    pub fn draw(&self) -> String {
        format!("Drawing curve {} with qualifier: {}", self.curve_id, self.qualifier)
    }

    pub fn set_qualifier(&mut self, qualifier: String) {
        self.qualifier = qualifier;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let drawable = DrawableQualifiedCurve2d::new(1, "tangent".to_string());
        assert_eq!(drawable.curve_id, 1);
        assert_eq!(drawable.qualifier, "tangent");
    }

    #[test]
    fn test_draw() {
        let drawable = DrawableQualifiedCurve2d::new(42, "normal".to_string());
        let output = drawable.draw();
        assert!(output.contains("42"));
        assert!(output.contains("normal"));
    }

    #[test]
    fn test_set_qualifier() {
        let mut drawable = DrawableQualifiedCurve2d::new(5, "old".to_string());
        drawable.set_qualifier("new".to_string());
        assert_eq!(drawable.qualifier, "new");
        assert!(drawable.draw().contains("new"));
    }

    #[test]
    fn test_clone() {
        let original = DrawableQualifiedCurve2d::new(7, "test".to_string());
        let cloned = original.clone();
        assert_eq!(original.curve_id, cloned.curve_id);
        assert_eq!(original.qualifier, cloned.qualifier);
    }
}
