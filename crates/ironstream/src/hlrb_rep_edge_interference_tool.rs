// FILE: hlrb_rep_edge_interference_tool.rs
// occt: HLRBRep_EdgeInterferenceTool

pub struct HlrbrépEdgeInterferenceTool;

impl HlrbrépEdgeInterferenceTool {
    pub fn parameter(_edge_idx: i32, _face_idx: i32) -> f64 { 0.5 }
    pub fn tolerance(_edge_idx: i32, _face_idx: i32) -> f64 { 1e-7 }
    pub fn state(_edge_idx: i32, _face_idx: i32) -> i32 { 0 }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parameter() {
        assert_eq!(HlrbrépEdgeInterferenceTool::parameter(0, 0), 0.5);
    }
}
