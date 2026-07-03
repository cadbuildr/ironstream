// FILE: b_rep_mesh_vertex_inspector.rs
// occt: BRepMesh_VertexInspector

/// Inspector for vertex inspection in mesh structures.
pub struct VertexInspector {
    vertices: Vec<(f64, f64, f64)>,
    tolerance: f64,
}

impl VertexInspector {
    /// Constructor.
    /// @param tolerance Tolerance for vertex comparison.
    pub fn new(tolerance: f64) -> Self {
        VertexInspector {
            vertices: Vec::new(),
            tolerance,
        }
    }

    /// Adds a vertex to inspect.
    pub fn add_vertex(&mut self, x: f64, y: f64, z: f64) -> usize {
        self.vertices.push((x, y, z));
        self.vertices.len() - 1
    }

    /// Finds vertex within tolerance.
    pub fn find_vertex(&self, x: f64, y: f64, z: f64) -> Option<usize> {
        for (i, (vx, vy, vz)) in self.vertices.iter().enumerate() {
            let dx = vx - x;
            let dy = vy - y;
            let dz = vz - z;
            let dist_sq = dx * dx + dy * dy + dz * dz;
            if dist_sq <= self.tolerance * self.tolerance {
                return Some(i);
            }
        }
        None
    }

    /// Returns number of vertices.
    pub fn vertices_count(&self) -> usize {
        self.vertices.len()
    }
}

impl Default for VertexInspector {
    fn default() -> Self {
        Self::new(0.01)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vertex_inspector_new() {
        let inspector = VertexInspector::new(0.01);
        assert_eq!(inspector.vertices_count(), 0);
    }

    #[test]
    fn test_vertex_inspector_add_vertex() {
        let mut inspector = VertexInspector::new(0.01);
        let idx = inspector.add_vertex(1.0, 2.0, 3.0);
        assert_eq!(idx, 0);
        assert_eq!(inspector.vertices_count(), 1);
    }

    #[test]
    fn test_vertex_inspector_find_vertex() {
        let mut inspector = VertexInspector::new(0.01);
        inspector.add_vertex(1.0, 2.0, 3.0);
        let found = inspector.find_vertex(1.0, 2.0, 3.0);
        assert_eq!(found, Some(0));
    }

    #[test]
    fn test_vertex_inspector_find_vertex_not_found() {
        let inspector = VertexInspector::new(0.01);
        let found = inspector.find_vertex(1.0, 2.0, 3.0);
        assert_eq!(found, None);
    }
}
