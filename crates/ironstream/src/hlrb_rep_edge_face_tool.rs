// FILE: hlrb_rep_edge_face_tool.rs
// occt: HLRBRep_EdgeFaceTool

pub struct HlrbrépEdgeFaceTool;

impl HlrbrépEdgeFaceTool {
    pub fn is_edge_on_face(_edge_idx: i32, _face_idx: i32) -> bool { false }
    pub fn parameter_on_face(_edge_idx: i32, _face_idx: i32, _t: f64) -> f64 { 0.0 }
    pub fn state_on_face(_edge_idx: i32, _face_idx: i32) -> i32 { 0 }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_edge_on_face() {
        assert!(!HlrbrépEdgeFaceTool::is_edge_on_face(0, 0));
    }
}
