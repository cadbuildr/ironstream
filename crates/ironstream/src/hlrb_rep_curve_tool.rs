// FILE: hlrb_rep_curve_tool.rs
// occt: HLRBRep_CurveTool

pub struct HlrbrépCurveTool;

impl HlrbrépCurveTool {
    pub fn first_parameter() -> f64 { 0.0 }
    pub fn last_parameter() -> f64 { 1.0 }
    pub fn is_closed() -> bool { false }
    pub fn is_periodic() -> bool { false }
    pub fn period() -> f64 { 0.0 }
    pub fn resolution(_r3d: f64) -> f64 { 0.01 }
    pub fn nb_intervals() -> usize { 1 }
    pub fn continuity() -> usize { 2 }
    pub fn degree() -> usize { 3 }
    pub fn is_rational() -> bool { false }
    pub fn nb_poles() -> usize { 4 }
    pub fn nb_knots() -> usize { 0 }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parameters() {
        assert_eq!(HlrbrépCurveTool::first_parameter(), 0.0);
        assert_eq!(HlrbrépCurveTool::last_parameter(), 1.0);
    }
}
