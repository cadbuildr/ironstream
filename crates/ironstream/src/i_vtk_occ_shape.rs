// FILE: i_vtk_occ_shape.rs
// occt: IVtkOCC_Shape

/// VTK-OCC representation of a topological shape for visualization.
#[derive(Clone, Debug)]
pub struct IVtkOCC_Shape {
    shape_id: u32,
    is_visible: bool,
}

impl IVtkOCC_Shape {
    /// Create a new VTK shape representation.
    pub fn new(shape_id: u32) -> Self {
        IVtkOCC_Shape {
            shape_id,
            is_visible: true,
        }
    }

    /// Get shape ID.
    pub fn shape_id(&self) -> u32 {
        self.shape_id
    }

    /// Set visibility.
    pub fn set_visible(&mut self, visible: bool) {
        self.is_visible = visible;
    }

    /// Check if shape is visible.
    pub fn is_visible(&self) -> bool {
        self.is_visible
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_shape() {
        let shape = IVtkOCC_Shape::new(100);
        assert_eq!(shape.shape_id(), 100);
        assert!(shape.is_visible());
    }

    #[test]
    fn test_set_visible() {
        let mut shape = IVtkOCC_Shape::new(200);
        shape.set_visible(false);
        assert!(!shape.is_visible());
    }

    #[test]
    fn test_visibility_toggle() {
        let mut shape = IVtkOCC_Shape::new(300);
        assert!(shape.is_visible());
        shape.set_visible(false);
        assert!(!shape.is_visible());
        shape.set_visible(true);
        assert!(shape.is_visible());
    }
}
