// FILE: hlr_test_drawable_poly_edge_tool.rs
// occt: HLRTest_DrawablePolyEdgeTool

//! Tool for drawing HLR polyline edges in test visualizations.

#[derive(Clone, Debug)]
pub struct DrawablePolyEdgeTool {
    pub edge_id: usize,
    pub vertices: Vec<(f64, f64, f64)>,
    pub color: u32,
}

impl DrawablePolyEdgeTool {
    pub fn new(edge_id: usize) -> Self {
        DrawablePolyEdgeTool {
            edge_id,
            vertices: Vec::new(),
            color: 0xFFFFFF,
        }
    }

    pub fn add_vertex(&mut self, x: f64, y: f64, z: f64) {
        self.vertices.push((x, y, z));
    }

    pub fn set_color(&mut self, color: u32) {
        self.color = color;
    }

    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    pub fn draw(&self) -> String {
        format!(
            "Drawing polyedge {} with {} vertices, color {:#x}",
            self.edge_id,
            self.vertex_count(),
            self.color
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let tool = DrawablePolyEdgeTool::new(1);
        assert_eq!(tool.edge_id, 1);
        assert_eq!(tool.vertex_count(), 0);
    }

    #[test]
    fn test_add_vertex() {
        let mut tool = DrawablePolyEdgeTool::new(1);
        tool.add_vertex(0.0, 0.0, 0.0);
        tool.add_vertex(1.0, 1.0, 1.0);

        assert_eq!(tool.vertex_count(), 2);
        assert_eq!(tool.vertices[0], (0.0, 0.0, 0.0));
        assert_eq!(tool.vertices[1], (1.0, 1.0, 1.0));
    }

    #[test]
    fn test_set_color() {
        let mut tool = DrawablePolyEdgeTool::new(1);
        tool.set_color(0x00FF00);
        assert_eq!(tool.color, 0x00FF00);
    }

    #[test]
    fn test_draw() {
        let mut tool = DrawablePolyEdgeTool::new(42);
        tool.add_vertex(0.0, 0.0, 0.0);
        tool.add_vertex(1.0, 0.0, 0.0);
        tool.add_vertex(1.0, 1.0, 0.0);

        let output = tool.draw();
        assert!(output.contains("42"));
        assert!(output.contains("3 vertices"));
    }

    #[test]
    fn test_polyedge_with_color() {
        let mut tool = DrawablePolyEdgeTool::new(5);
        tool.set_color(0xFF0000);
        tool.add_vertex(0.0, 0.0, 0.0);

        assert_eq!(tool.vertex_count(), 1);
        assert_eq!(tool.color, 0xFF0000);
    }
}
