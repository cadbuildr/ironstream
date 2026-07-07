// FILE: i_vtk_i_shape_data.rs
// occt: IVtk_IShapeData

/// Interface for VTK shape data.
pub trait IVtk_IShapeData {
    /// Get the number of vertices.
    fn vertex_count(&self) -> usize;

    /// Get the number of faces.
    fn face_count(&self) -> usize;

    /// Get the number of edges.
    fn edge_count(&self) -> usize;

    /// Check if data is valid.
    fn is_valid(&self) -> bool;
}

/// Default implementation of IVtk_IShapeData.
#[derive(Clone, Debug)]
pub struct DefaultShapeData {
    vertices: usize,
    faces: usize,
    edges: usize,
    valid: bool,
}

impl DefaultShapeData {
    /// Create new shape data.
    pub fn new(vertices: usize, faces: usize, edges: usize) -> Self {
        DefaultShapeData {
            vertices,
            faces,
            edges,
            valid: true,
        }
    }
}

impl IVtk_IShapeData for DefaultShapeData {
    fn vertex_count(&self) -> usize {
        if self.valid { self.vertices } else { 0 }
    }

    fn face_count(&self) -> usize {
        if self.valid { self.faces } else { 0 }
    }

    fn edge_count(&self) -> usize {
        if self.valid { self.edges } else { 0 }
    }

    fn is_valid(&self) -> bool {
        self.valid
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_shape_data() {
        let data = DefaultShapeData::new(10, 5, 15);
        assert_eq!(data.vertex_count(), 10);
        assert_eq!(data.face_count(), 5);
        assert_eq!(data.edge_count(), 15);
        assert!(data.is_valid());
    }

    #[test]
    fn test_shape_data_trait() {
        let data: Box<dyn IVtk_IShapeData> = Box::new(DefaultShapeData::new(20, 10, 30));
        assert_eq!(data.vertex_count(), 20);
        assert_eq!(data.face_count(), 10);
        assert_eq!(data.edge_count(), 30);
    }
}
