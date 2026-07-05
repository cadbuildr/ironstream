// FILE: draw_drawable2_d.rs
// occt: Draw_Drawable2D

//! Base class for 2D drawable objects in the Draw application.
//! A 2D drawable is a drawable that projects onto the XY plane.

/// Represents a 2D drawable object
pub struct DrawDrawable2D {
    /// Base drawable properties
    base: crate::draw_drawable3_d::DrawDrawable3DImpl,
}

impl DrawDrawable2D {
    /// Create a new 2D drawable
    pub fn new() -> Self {
        DrawDrawable2D {
            base: crate::draw_drawable3_d::DrawDrawable3DImpl::new(),
        }
    }

    /// Check if this is a 3D object (always returns false for 2D drawable)
    pub fn is_3d(&self) -> bool {
        false
    }

    /// Get the name
    pub fn name(&self) -> Option<&str> {
        self.base.name()
    }

    /// Set the name
    pub fn set_name(&mut self, name: String) {
        self.base.set_name(name);
    }

    /// Get visibility
    pub fn visible(&self) -> bool {
        self.base.visible()
    }

    /// Set visibility
    pub fn set_visible(&mut self, visible: bool) {
        self.base.set_visible(visible);
    }

    /// Get protected status
    pub fn protected(&self) -> bool {
        self.base.protected()
    }

    /// Set protected status
    pub fn set_protected(&mut self, protected: bool) {
        self.base.set_protected(protected);
    }

    /// Get bounds
    pub fn bounds(&self) -> (f64, f64, f64, f64) {
        self.base.bounds()
    }

    /// Set bounds
    pub fn set_bounds(&mut self, x_min: f64, x_max: f64, y_min: f64, y_max: f64) {
        self.base.set_bounds(x_min, x_max, y_min, y_max);
    }
}

impl Default for DrawDrawable2D {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drawable2d_creation() {
        let drawable = DrawDrawable2D::new();
        assert!(!drawable.is_3d());
        assert!(drawable.visible());
    }

    #[test]
    fn test_drawable2d_is_not_3d() {
        let drawable = DrawDrawable2D::new();
        assert!(!drawable.is_3d());
        // Verify it's 2D by checking is_3d returns false
    }

    #[test]
    fn test_drawable2d_visibility() {
        let mut drawable = DrawDrawable2D::new();
        drawable.set_visible(false);
        assert!(!drawable.visible());
    }

    #[test]
    fn test_drawable2d_bounds() {
        let mut drawable = DrawDrawable2D::new();
        drawable.set_bounds(0.0, 10.0, 0.0, 10.0);
        let (xmin, xmax, ymin, ymax) = drawable.bounds();
        assert_eq!((xmin, xmax, ymin, ymax), (0.0, 10.0, 0.0, 10.0));
    }

    #[test]
    fn test_drawable2d_name() {
        let mut drawable = DrawDrawable2D::new();
        drawable.set_name("my_2d_drawable".to_string());
        assert_eq!(drawable.name(), Some("my_2d_drawable"));
    }
}
