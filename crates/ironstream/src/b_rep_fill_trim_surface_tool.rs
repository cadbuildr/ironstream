// FILE: b_rep_fill_trim_surface_tool.rs
// occt: BRepFill_TrimSurfaceTool

#[derive(Debug, Clone)]
pub struct BRepFillTrimSurfaceTool {
    face1_id: i32,
    face2_id: i32,
    edge1_id: i32,
    edge2_id: i32,
    inv1: bool,
    inv2: bool,
}

impl BRepFillTrimSurfaceTool {
    pub fn new(face1_id: i32, face2_id: i32, edge1_id: i32, edge2_id: i32, inv1: bool, inv2: bool) -> Self {
        BRepFillTrimSurfaceTool {
            face1_id,
            face2_id,
            edge1_id,
            edge2_id,
            inv1,
            inv2,
        }
    }

    pub fn face1_id(&self) -> i32 {
        self.face1_id
    }

    pub fn face2_id(&self) -> i32 {
        self.face2_id
    }

    pub fn edge1_id(&self) -> i32 {
        self.edge1_id
    }

    pub fn edge2_id(&self) -> i32 {
        self.edge2_id
    }

    pub fn is_inv1(&self) -> bool {
        self.inv1
    }

    pub fn is_inv2(&self) -> bool {
        self.inv2
    }

    pub fn is_on_face(&self, p_x: f64, p_y: f64) -> bool {
        p_x >= 0.0 && p_x <= 1.0 && p_y >= 0.0 && p_y <= 1.0
    }

    pub fn proj_on(&self, p_x: f64, _p_y: f64) -> f64 {
        p_x
    }
}

impl Default for BRepFillTrimSurfaceTool {
    fn default() -> Self {
        Self::new(0, 0, 0, 0, false, false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trim_surface_tool_new() {
        let tst = BRepFillTrimSurfaceTool::new(1, 2, 3, 4, true, false);
        assert_eq!(tst.face1_id(), 1);
        assert_eq!(tst.face2_id(), 2);
        assert_eq!(tst.edge1_id(), 3);
        assert_eq!(tst.edge2_id(), 4);
        assert!(tst.is_inv1());
        assert!(!tst.is_inv2());
    }

    #[test]
    fn test_trim_surface_tool_is_on_face() {
        let tst = BRepFillTrimSurfaceTool::default();
        assert!(tst.is_on_face(0.5, 0.5));
        assert!(!tst.is_on_face(-0.1, 0.5));
        assert!(!tst.is_on_face(1.1, 0.5));
    }

    #[test]
    fn test_trim_surface_tool_proj_on() {
        let tst = BRepFillTrimSurfaceTool::default();
        assert_eq!(tst.proj_on(0.3, 0.7), 0.3);
    }

    #[test]
    fn test_trim_surface_tool_default() {
        let tst = BRepFillTrimSurfaceTool::default();
        assert_eq!(tst.face1_id(), 0);
        assert!(!tst.is_inv1());
    }
}
