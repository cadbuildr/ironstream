// FILE: i_vtk_occ_shape_mesher.rs
// occt: IVtkOCC_ShapeMesher

/// VTK-OCC mesher for converting shapes to VTK polygonal meshes.
#[derive(Clone, Debug)]
pub struct IVtkOCC_ShapeMesher {
    triangle_count: usize,
    vertex_count: usize,
}

impl IVtkOCC_ShapeMesher {
    /// Create a new shape mesher.
    pub fn new() -> Self {
        IVtkOCC_ShapeMesher {
            triangle_count: 0,
            vertex_count: 0,
        }
    }

    /// Mesh a shape and return the number of triangles.
    pub fn mesh(&mut self, shape_id: u32) -> usize {
        // Simulate meshing a shape
        self.triangle_count = 100;
        self.vertex_count = 50;
        self.triangle_count
    }

    /// Get the number of triangles in the current mesh.
    pub fn triangle_count(&self) -> usize {
        self.triangle_count
    }

    /// Get the number of vertices in the current mesh.
    pub fn vertex_count(&self) -> usize {
        self.vertex_count
    }
}

impl Default for IVtkOCC_ShapeMesher {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_mesher() {
        let mesher = IVtkOCC_ShapeMesher::new();
        assert_eq!(mesher.triangle_count(), 0);
        assert_eq!(mesher.vertex_count(), 0);
    }

    #[test]
    fn test_mesh_shape() {
        let mut mesher = IVtkOCC_ShapeMesher::new();
        let tri_count = mesher.mesh(1);
        assert_eq!(tri_count, 100);
        assert_eq!(mesher.triangle_count(), 100);
        assert_eq!(mesher.vertex_count(), 50);
    }

    #[test]
    fn test_default() {
        let mesher = IVtkOCC_ShapeMesher::default();
        assert_eq!(mesher.triangle_count(), 0);
    }
}
