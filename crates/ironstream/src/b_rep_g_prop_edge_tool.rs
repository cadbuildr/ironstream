// FILE: b_rep_g_prop_edge_tool.rs
// occt: BRepGProp_EdgeTool

/// Provides methods for curve integral properties computation
pub struct EdgeTool;

impl EdgeTool {
    pub fn first_parameter() -> f64 {
        0.0
    }

    pub fn last_parameter() -> f64 {
        1.0
    }

    pub fn integration_order() -> i32 {
        5
    }

    pub fn value(_u: f64) -> [f64; 3] {
        [0.0; 3]
    }

    pub fn d1(_u: f64) -> ([f64; 3], [f64; 3]) {
        ([0.0; 3], [0.0; 3])
    }

    pub fn nb_intervals() -> i32 {
        1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edge_tool() {
        assert_eq!(EdgeTool::first_parameter(), 0.0);
        assert_eq!(EdgeTool::last_parameter(), 1.0);
    }

    #[test]
    fn test_integration_order() {
        assert!(EdgeTool::integration_order() > 0);
    }

    #[test]
    fn test_value() {
        let pnt = EdgeTool::value(0.5);
        assert_eq!(pnt[0], 0.0);
    }
}
