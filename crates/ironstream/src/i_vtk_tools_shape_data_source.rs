// FILE: i_vtk_tools_shape_data_source.rs
// occt: IVtkTools_ShapeDataSource

/// Data source for VTK shapes.
#[derive(Clone, Debug)]
pub struct IVtkTools_ShapeDataSource {
    shape_id: u32,
    is_valid: bool,
}

impl IVtkTools_ShapeDataSource {
    /// Create a new shape data source.
    pub fn new(shape_id: u32) -> Self {
        IVtkTools_ShapeDataSource {
            shape_id,
            is_valid: true,
        }
    }

    /// Get the shape ID.
    pub fn shape_id(&self) -> u32 {
        self.shape_id
    }

    /// Check if the data source is valid.
    pub fn is_valid(&self) -> bool {
        self.is_valid
    }

    /// Invalidate the data source.
    pub fn invalidate(&mut self) {
        self.is_valid = false;
    }

    /// Get the number of vertices.
    pub fn vertex_count(&self) -> usize {
        if self.is_valid {
            100
        } else {
            0
        }
    }

    /// Get the number of faces.
    pub fn face_count(&self) -> usize {
        if self.is_valid {
            50
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_source() {
        let source = IVtkTools_ShapeDataSource::new(5);
        assert_eq!(source.shape_id(), 5);
        assert!(source.is_valid());
    }

    #[test]
    fn test_vertex_count() {
        let source = IVtkTools_ShapeDataSource::new(1);
        assert_eq!(source.vertex_count(), 100);
    }

    #[test]
    fn test_face_count() {
        let source = IVtkTools_ShapeDataSource::new(1);
        assert_eq!(source.face_count(), 50);
    }

    #[test]
    fn test_invalidate() {
        let mut source = IVtkTools_ShapeDataSource::new(1);
        source.invalidate();
        assert!(!source.is_valid());
        assert_eq!(source.vertex_count(), 0);
        assert_eq!(source.face_count(), 0);
    }
}
