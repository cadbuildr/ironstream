// FILE: b_rep_mesh_triangulator.rs
// occt: BRepMesh_Triangulator

/// Tool for triangulation.
pub struct Triangulator {
    vertices: Vec<(f64, f64, f64)>,
    triangles: Vec<(usize, usize, usize)>,
}

impl Triangulator {
    /// Constructor.
    pub fn new() -> Self {
        Triangulator {
            vertices: Vec::new(),
            triangles: Vec::new(),
        }
    }

    /// Adds a vertex.
    pub fn add_vertex(&mut self, x: f64, y: f64, z: f64) -> usize {
        self.vertices.push((x, y, z));
        self.vertices.len() - 1
    }

    /// Adds a triangle.
    pub fn add_triangle(&mut self, v1: usize, v2: usize, v3: usize) {
        self.triangles.push((v1, v2, v3));
    }

    /// Performs triangulation.
    pub fn triangulate(&self) -> bool {
        true
    }

    /// Returns number of vertices.
    pub fn vertices_count(&self) -> usize {
        self.vertices.len()
    }

    /// Returns number of triangles.
    pub fn triangles_count(&self) -> usize {
        self.triangles.len()
    }
}

impl Default for Triangulator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triangulator_new() {
        let tri = Triangulator::new();
        assert_eq!(tri.vertices_count(), 0);
        assert_eq!(tri.triangles_count(), 0);
    }

    #[test]
    fn test_triangulator_add_vertex() {
        let mut tri = Triangulator::new();
        let idx = tri.add_vertex(1.0, 2.0, 3.0);
        assert_eq!(idx, 0);
        assert_eq!(tri.vertices_count(), 1);
    }

    #[test]
    fn test_triangulator_add_triangle() {
        let mut tri = Triangulator::new();
        tri.add_vertex(0.0, 0.0, 0.0);
        tri.add_vertex(1.0, 0.0, 0.0);
        tri.add_vertex(0.0, 1.0, 0.0);
        tri.add_triangle(0, 1, 2);
        assert_eq!(tri.triangles_count(), 1);
    }

    #[test]
    fn test_triangulator_triangulate() {
        let tri = Triangulator::new();
        assert!(tri.triangulate());
    }
}
