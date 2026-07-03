// FILE: b_rep_blend_h_curve2d_tool.rs
// occt: BRepBlend_HCurve2dTool

pub struct BRepBlendHCurve2dTool;

impl BRepBlendHCurve2dTool {
    pub fn first_parameter() -> f64 {
        0.0
    }

    pub fn last_parameter() -> f64 {
        1.0
    }

    pub fn is_closed() -> bool {
        false
    }

    pub fn is_periodic() -> bool {
        false
    }

    pub fn period() -> f64 {
        1.0
    }

    pub fn value(u: f64) -> (f64, f64) {
        (u, 0.0)
    }

    pub fn d0(u: f64) -> (f64, f64) {
        (u, 0.0)
    }

    pub fn d1(u: f64) -> ((f64, f64), (f64, f64)) {
        ((u, 0.0), (1.0, 0.0))
    }

    pub fn d2(u: f64) -> ((f64, f64), (f64, f64), (f64, f64)) {
        ((u, 0.0), (1.0, 0.0), (0.0, 0.0))
    }

    pub fn resolution(r3d: f64) -> f64 {
        r3d
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parameters() {
        assert_eq!(BRepBlendHCurve2dTool::first_parameter(), 0.0);
        assert_eq!(BRepBlendHCurve2dTool::last_parameter(), 1.0);
    }

    #[test]
    fn test_is_closed() {
        assert!(!BRepBlendHCurve2dTool::is_closed());
    }

    #[test]
    fn test_is_periodic() {
        assert!(!BRepBlendHCurve2dTool::is_periodic());
    }

    #[test]
    fn test_value() {
        assert_eq!(BRepBlendHCurve2dTool::value(0.5), (0.5, 0.0));
    }
}
