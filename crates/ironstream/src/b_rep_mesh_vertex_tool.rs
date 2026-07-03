// FILE: b_rep_mesh_vertex_tool.rs
// occt: BRepMesh_VertexTool

/// Tool for vertex operations.
pub struct VertexTool {
    vertices: Vec<(f64, f64, f64)>,
}

impl VertexTool {
    /// Constructor.
    pub fn new() -> Self {
        VertexTool {
            vertices: Vec::new(),
        }
    }

    /// Adds a vertex.
    pub fn add_vertex(&mut self, x: f64, y: f64, z: f64) -> usize {
        self.vertices.push((x, y, z));
        self.vertices.len() - 1
    }

    /// Gets a vertex.
    pub fn vertex(&self, index: usize) -> Option<(f64, f64, f64)> {
        self.vertices.get(index).copied()
    }

    /// Removes duplicates.
    pub fn remove_duplicates(&mut self, tolerance: f64) {
        let mut i = 0;
        while i < self.vertices.len() {
            let mut j = i + 1;
            while j < self.vertices.len() {
                let (x1, y1, z1) = self.vertices[i];
                let (x2, y2, z2) = self.vertices[j];
                let dx = x2 - x1;
                let dy = y2 - y1;
                let dz = z2 - z1;
                if dx * dx + dy * dy + dz * dz <= tolerance * tolerance {
                    self.vertices.remove(j);
                } else {
                    j += 1;
                }
            }
            i += 1;
        }
    }

    /// Returns number of vertices.
    pub fn vertices_count(&self) -> usize {
        self.vertices.len()
    }
}

impl Default for VertexTool {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vertex_tool_new() {
        let tool = VertexTool::new();
        assert_eq!(tool.vertices_count(), 0);
    }

    #[test]
    fn test_vertex_tool_add_vertex() {
        let mut tool = VertexTool::new();
        let idx = tool.add_vertex(1.0, 2.0, 3.0);
        assert_eq!(idx, 0);
        assert_eq!(tool.vertices_count(), 1);
    }

    #[test]
    fn test_vertex_tool_get_vertex() {
        let mut tool = VertexTool::new();
        tool.add_vertex(1.0, 2.0, 3.0);
        let v = tool.vertex(0);
        assert_eq!(v, Some((1.0, 2.0, 3.0)));
    }

    #[test]
    fn test_vertex_tool_remove_duplicates() {
        let mut tool = VertexTool::new();
        tool.add_vertex(1.0, 2.0, 3.0);
        tool.add_vertex(1.001, 2.001, 3.001);
        tool.add_vertex(5.0, 5.0, 5.0);
        tool.remove_duplicates(0.01);
        assert_eq!(tool.vertices_count(), 2);
    }
}
