// FILE: b_rep_blend_h_curve_tool.rs
// occt: BRepBlend_HCurveTool

pub struct BRepBlendHCurveTool;

impl BRepBlendHCurveTool {
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

    pub fn value(u: f64) -> (f64, f64, f64) {
        (u, 0.0, 0.0)
    }

    pub fn d0(u: f64) -> (f64, f64, f64) {
        (u, 0.0, 0.0)
    }

    pub fn d1(u: f64) -> ((f64, f64, f64), (f64, f64, f64)) {
        ((u, 0.0, 0.0), (1.0, 0.0, 0.0))
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
        assert_eq!(BRepBlendHCurveTool::first_parameter(), 0.0);
        assert_eq!(BRepBlendHCurveTool::last_parameter(), 1.0);
    }

    #[test]
    fn test_value() {
        assert_eq!(BRepBlendHCurveTool::value(0.5), (0.5, 0.0, 0.0));
    }

    #[test]
    fn test_d1() {
        let (p, v) = BRepBlendHCurveTool::d1(0.5);
        assert_eq!(p, (0.5, 0.0, 0.0));
        assert_eq!(v, (1.0, 0.0, 0.0));
    }
}
