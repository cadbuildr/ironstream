// FILE: i_vtk_i_shape.rs
// occt: IVtk_IShape

/// Interface for VTK shape representation.
pub trait IVtk_IShape {
    /// Get shape ID.
    fn shape_id(&self) -> u32;

    /// Check if shape is valid.
    fn is_valid(&self) -> bool;

    /// Get number of vertices.
    fn vertex_count(&self) -> usize;

    /// Get number of cells.
    fn cell_count(&self) -> usize;
}

/// Default implementation of IVtk_IShape.
#[derive(Clone, Debug)]
pub struct DefaultShape {
    id: u32,
    valid: bool,
}

impl DefaultShape {
    /// Create a new default shape.
    pub fn new(id: u32) -> Self {
        DefaultShape { id, valid: true }
    }
}

impl IVtk_IShape for DefaultShape {
    fn shape_id(&self) -> u32 {
        self.id
    }

    fn is_valid(&self) -> bool {
        self.valid
    }

    fn vertex_count(&self) -> usize {
        if self.valid { 10 } else { 0 }
    }

    fn cell_count(&self) -> usize {
        if self.valid { 5 } else { 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_shape() {
        let shape = DefaultShape::new(42);
        assert_eq!(shape.shape_id(), 42);
        assert!(shape.is_valid());
        assert_eq!(shape.vertex_count(), 10);
        assert_eq!(shape.cell_count(), 5);
    }

    #[test]
    fn test_shape_trait() {
        let shape: Box<dyn IVtk_IShape> = Box::new(DefaultShape::new(99));
        assert_eq!(shape.shape_id(), 99);
        assert!(shape.is_valid());
    }
}
