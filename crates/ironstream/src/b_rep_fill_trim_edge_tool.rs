// FILE: b_rep_fill_trim_edge_tool.rs
// occt: BRepFill_TrimEdgeTool

#[derive(Debug, Clone)]
pub struct BRepFillTrimEdgeTool {
    is_point1: bool,
    is_point2: bool,
    point1_x: f64,
    point1_y: f64,
    point2_x: f64,
    point2_y: f64,
    offset: f64,
}

impl BRepFillTrimEdgeTool {
    pub fn new() -> Self {
        BRepFillTrimEdgeTool {
            is_point1: false,
            is_point2: false,
            point1_x: 0.0,
            point1_y: 0.0,
            point2_x: 0.0,
            point2_y: 0.0,
            offset: 0.0,
        }
    }

    pub fn with_offset(offset: f64) -> Self {
        BRepFillTrimEdgeTool {
            is_point1: false,
            is_point2: false,
            point1_x: 0.0,
            point1_y: 0.0,
            point2_x: 0.0,
            point2_y: 0.0,
            offset,
        }
    }

    pub fn is_inside(&self, p_x: f64, p_y: f64) -> bool {
        let dx1 = p_x - self.point1_x;
        let dy1 = p_y - self.point1_y;
        let dx2 = p_x - self.point2_x;
        let dy2 = p_y - self.point2_y;
        let dist1_sq = dx1 * dx1 + dy1 * dy1;
        let dist2_sq = dx2 * dx2 + dy2 * dy2;
        dist1_sq > self.offset * self.offset && dist2_sq > self.offset * self.offset
    }

    pub fn offset(&self) -> f64 {
        self.offset
    }

    pub fn set_offset(&mut self, offset: f64) {
        self.offset = offset;
    }

    pub fn point1(&self) -> (f64, f64) {
        (self.point1_x, self.point1_y)
    }

    pub fn set_point1(&mut self, x: f64, y: f64) {
        self.point1_x = x;
        self.point1_y = y;
    }

    pub fn point2(&self) -> (f64, f64) {
        (self.point2_x, self.point2_y)
    }

    pub fn set_point2(&mut self, x: f64, y: f64) {
        self.point2_x = x;
        self.point2_y = y;
    }
}

impl Default for BRepFillTrimEdgeTool {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trim_edge_tool_new() {
        let tet = BRepFillTrimEdgeTool::new();
        assert_eq!(tet.offset(), 0.0);
    }

    #[test]
    fn test_trim_edge_tool_with_offset() {
        let tet = BRepFillTrimEdgeTool::with_offset(2.5);
        assert_eq!(tet.offset(), 2.5);
    }

    #[test]
    fn test_trim_edge_tool_points() {
        let mut tet = BRepFillTrimEdgeTool::new();
        tet.set_point1(1.0, 2.0);
        assert_eq!(tet.point1(), (1.0, 2.0));

        tet.set_point2(3.0, 4.0);
        assert_eq!(tet.point2(), (3.0, 4.0));
    }

    #[test]
    fn test_trim_edge_tool_is_inside() {
        let mut tet = BRepFillTrimEdgeTool::new();
        tet.set_offset(1.0);
        tet.set_point1(0.0, 0.0);
        tet.set_point2(5.0, 5.0);
        assert!(tet.is_inside(10.0, 10.0));
        assert!(!tet.is_inside(0.5, 0.5));
    }
}
