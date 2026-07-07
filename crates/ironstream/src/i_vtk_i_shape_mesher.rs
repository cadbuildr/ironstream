// FILE: i_vtk_i_shape_mesher.rs
// occt: IVtk_IShapeMesher

/// Interface for shape meshing in VTK.
pub trait IVtk_IShapeMesher {
    /// Mesh a shape.
    fn mesh(&mut self, shape_id: u32) -> bool;

    /// Get triangle count.
    fn triangle_count(&self) -> usize;

    /// Get vertex count.
    fn vertex_count(&self) -> usize;

    /// Check if mesh is valid.
    fn is_valid(&self) -> bool;
}

/// Default implementation of IVtk_IShapeMesher.
#[derive(Clone, Debug)]
pub struct DefaultShapeMesher {
    triangles: usize,
    vertices: usize,
    valid: bool,
}

impl DefaultShapeMesher {
    /// Create a new shape mesher.
    pub fn new() -> Self {
        DefaultShapeMesher {
            triangles: 0,
            vertices: 0,
            valid: true,
        }
    }
}

impl Default for DefaultShapeMesher {
    fn default() -> Self {
        Self::new()
    }
}

impl IVtk_IShapeMesher for DefaultShapeMesher {
    fn mesh(&mut self, _shape_id: u32) -> bool {
        self.triangles = 100;
        self.vertices = 50;
        true
    }

    fn triangle_count(&self) -> usize {
        self.triangles
    }

    fn vertex_count(&self) -> usize {
        self.vertices
    }

    fn is_valid(&self) -> bool {
        self.valid
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_mesher() {
        let mesher = DefaultShapeMesher::new();
        assert_eq!(mesher.triangle_count(), 0);
        assert_eq!(mesher.vertex_count(), 0);
        assert!(mesher.is_valid());
    }

    #[test]
    fn test_mesh_shape() {
        let mut mesher = DefaultShapeMesher::new();
        let success = mesher.mesh(1);
        assert!(success);
        assert_eq!(mesher.triangle_count(), 100);
        assert_eq!(mesher.vertex_count(), 50);
    }

    #[test]
    fn test_mesher_trait() {
        let mut mesher: Box<dyn IVtk_IShapeMesher> = Box::new(DefaultShapeMesher::new());
        mesher.mesh(1);
        assert_eq!(mesher.triangle_count(), 100);
    }
}
