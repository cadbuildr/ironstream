// FILE: draw_drawable3_d.rs
// occt: Draw_Drawable3D

//! Base class for drawable objects in the Draw application.
//! Represents objects that can be displayed in 2D/3D views.

/// Base trait for drawable objects
pub trait DrawDrawable3D {
    /// Draw the object on a display
    fn draw_on(&self, _display: &mut crate::draw_display::DrawDisplay);

    /// Check if pick is outside the bounding box
    fn pick_reject(&self, _x: f64, _y: f64, _prec: f64) -> bool {
        true
    }

    /// Create a copy of the drawable
    fn copy(&self) -> Box<dyn DrawDrawable3D>;

    /// Dump the drawable to a string
    fn dump(&self) -> String {
        String::from("Draw_Drawable3D")
    }

    /// Get the type name of the drawable
    fn whatis(&self) -> &str {
        "Draw_Drawable3D"
    }

    /// Check if this is a 3D object (default true)
    fn is_3d(&self) -> bool {
        true
    }

    /// Check if object can be displayed (default true)
    fn is_displayable(&self) -> bool {
        true
    }

    /// Get the name of the drawable
    fn name(&self) -> Option<&str> {
        None
    }

    /// Set the name of the drawable
    fn set_name(&mut self, _name: &str) {}

    /// Get visibility status
    fn visible(&self) -> bool {
        true
    }

    /// Set visibility status
    fn set_visible(&mut self, _visible: bool) {}

    /// Get protected status
    fn protected(&self) -> bool {
        false
    }

    /// Set protected status
    fn set_protected(&mut self, _protected: bool) {}

    /// Get bounding box
    fn bounds(&self) -> (f64, f64, f64, f64) {
        (0.0, 0.0, 0.0, 0.0)
    }

    /// Set bounding box
    fn set_bounds(&mut self, _x_min: f64, _x_max: f64, _y_min: f64, _y_max: f64) {}
}

/// Concrete drawable base implementation
pub struct DrawDrawable3DImpl {
    name: Option<String>,
    visible: bool,
    protected: bool,
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
}

impl DrawDrawable3DImpl {
    /// Create a new drawable
    pub fn new() -> Self {
        DrawDrawable3DImpl {
            name: None,
            visible: true,
            protected: false,
            x_min: 0.0,
            x_max: 0.0,
            y_min: 0.0,
            y_max: 0.0,
        }
    }

    /// Get the name
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Set the name
    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    /// Get visibility
    pub fn visible(&self) -> bool {
        self.visible
    }

    /// Set visibility
    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    /// Get protected status
    pub fn protected(&self) -> bool {
        self.protected
    }

    /// Set protected status
    pub fn set_protected(&mut self, protected: bool) {
        self.protected = protected;
    }

    /// Get bounds
    pub fn bounds(&self) -> (f64, f64, f64, f64) {
        (self.x_min, self.x_max, self.y_min, self.y_max)
    }

    /// Set bounds
    pub fn set_bounds(&mut self, x_min: f64, x_max: f64, y_min: f64, y_max: f64) {
        self.x_min = x_min;
        self.x_max = x_max;
        self.y_min = y_min;
        self.y_max = y_max;
    }
}

impl Default for DrawDrawable3DImpl {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drawable_creation() {
        let drawable = DrawDrawable3DImpl::new();
        assert!(drawable.visible());
        assert!(!drawable.protected());
        assert_eq!(drawable.bounds(), (0.0, 0.0, 0.0, 0.0));
    }

    #[test]
    fn test_drawable_visibility() {
        let mut drawable = DrawDrawable3DImpl::new();
        drawable.set_visible(false);
        assert!(!drawable.visible());
    }

    #[test]
    fn test_drawable_protected() {
        let mut drawable = DrawDrawable3DImpl::new();
        drawable.set_protected(true);
        assert!(drawable.protected());
    }

    #[test]
    fn test_drawable_bounds() {
        let mut drawable = DrawDrawable3DImpl::new();
        drawable.set_bounds(1.0, 2.0, 3.0, 4.0);
        assert_eq!(drawable.bounds(), (1.0, 2.0, 3.0, 4.0));
    }

    #[test]
    fn test_drawable_name() {
        let mut drawable = DrawDrawable3DImpl::new();
        assert!(drawable.name().is_none());
        drawable.set_name("test".to_string());
        assert_eq!(drawable.name(), Some("test"));
    }
}
