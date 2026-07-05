// FILE: i_vtk_tools_shape_object.rs
// occt: IVtkTools_ShapeObject

/// Shape object wrapper for VTK tools.
#[derive(Clone, Debug)]
pub struct IVtkTools_ShapeObject {
    shape_id: u32,
    visible: bool,
}

impl IVtkTools_ShapeObject {
    /// Create a new shape object.
    pub fn new(shape_id: u32) -> Self {
        IVtkTools_ShapeObject {
            shape_id,
            visible: true,
        }
    }

    /// Get the shape ID.
    pub fn shape_id(&self) -> u32 {
        self.shape_id
    }

    /// Set visibility.
    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    /// Check if shape is visible.
    pub fn is_visible(&self) -> bool {
        self.visible
    }

    /// Display the shape.
    pub fn display(&self) {
        // Display logic
    }

    /// Hide the shape.
    pub fn hide(&mut self) {
        self.visible = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_shape_object() {
        let obj = IVtkTools_ShapeObject::new(10);
        assert_eq!(obj.shape_id(), 10);
        assert!(obj.is_visible());
    }

    #[test]
    fn test_set_visible() {
        let mut obj = IVtkTools_ShapeObject::new(10);
        obj.set_visible(false);
        assert!(!obj.is_visible());
    }

    #[test]
    fn test_hide() {
        let mut obj = IVtkTools_ShapeObject::new(10);
        obj.hide();
        assert!(!obj.is_visible());
    }
}
