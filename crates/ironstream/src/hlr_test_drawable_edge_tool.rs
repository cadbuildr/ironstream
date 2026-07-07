// FILE: hlr_test_drawable_edge_tool.rs
// occt: HLRTest_DrawableEdgeTool

//! Tool for drawing HLR edges in test visualizations.

#[derive(Clone, Debug)]
pub struct DrawableEdgeTool {
    pub edge_id: usize,
    pub visible: bool,
    pub color: u32,
}

impl DrawableEdgeTool {
    pub fn new(edge_id: usize, visible: bool) -> Self {
        DrawableEdgeTool {
            edge_id,
            visible,
            color: 0xFFFFFF,
        }
    }

    pub fn set_color(&mut self, color: u32) {
        self.color = color;
    }

    pub fn draw(&self) -> String {
        let status = if self.visible { "visible" } else { "hidden" };
        format!("Drawing edge {} ({}) with color {:#x}", self.edge_id, status, self.color)
    }

    pub fn toggle_visibility(&mut self) {
        self.visible = !self.visible;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let tool = DrawableEdgeTool::new(1, true);
        assert_eq!(tool.edge_id, 1);
        assert!(tool.visible);
    }

    #[test]
    fn test_set_color() {
        let mut tool = DrawableEdgeTool::new(1, true);
        tool.set_color(0xFF0000);
        assert_eq!(tool.color, 0xFF0000);
    }

    #[test]
    fn test_draw() {
        let tool = DrawableEdgeTool::new(42, true);
        let output = tool.draw();
        assert!(output.contains("42"));
        assert!(output.contains("visible"));
    }

    #[test]
    fn test_toggle_visibility() {
        let mut tool = DrawableEdgeTool::new(1, true);
        assert!(tool.visible);
        tool.toggle_visibility();
        assert!(!tool.visible);
        tool.toggle_visibility();
        assert!(tool.visible);
    }

    #[test]
    fn test_default_color() {
        let tool = DrawableEdgeTool::new(1, false);
        assert_eq!(tool.color, 0xFFFFFF);
        let output = tool.draw();
        assert!(output.contains("hidden"));
    }
}
