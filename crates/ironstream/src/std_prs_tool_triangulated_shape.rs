// FILE: std_prs_tool_triangulated_shape.rs
// occt: StdPrs_ToolTriangulatedShape

/// Tool for triangulated shape operations.
#[derive(Clone, Debug)]
pub struct ToolTriangulatedShape {
    pub tool_id: u32,
    pub triangle_count: usize,
    pub vertex_count: usize,
}

impl ToolTriangulatedShape {
    pub fn new(tool_id: u32) -> Self {
        Self {
            tool_id,
            triangle_count: 0,
            vertex_count: 0,
        }
    }

    pub fn add_triangle(&mut self) {
        self.triangle_count += 1;
    }

    pub fn add_vertex(&mut self) {
        self.vertex_count += 1;
    }

    pub fn nb_triangles(&self) -> usize {
        self.triangle_count
    }

    pub fn nb_vertices(&self) -> usize {
        self.vertex_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_tool() {
        let tool = ToolTriangulatedShape::new(1);
        assert_eq!(tool.tool_id, 1);
    }

    #[test]
    fn add_triangles_and_vertices() {
        let mut tool = ToolTriangulatedShape::new(1);
        for _ in 0..10 {
            tool.add_vertex();
        }
        for _ in 0..5 {
            tool.add_triangle();
        }
        assert_eq!(tool.nb_vertices(), 10);
        assert_eq!(tool.nb_triangles(), 5);
    }
}
