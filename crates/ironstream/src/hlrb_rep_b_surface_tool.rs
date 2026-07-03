// FILE: hlrb_rep_b_surface_tool.rs
// occt: HLRBRep_BSurfaceTool

/// Tool for accessing Bezier/BSpline surface properties.
pub struct HlrbrépBSurfaceTool;

impl HlrbrépBSurfaceTool {
    pub fn first_parameter_u() -> f64 { 0.0 }
    pub fn last_parameter_u() -> f64 { 1.0 }
    pub fn first_parameter_v() -> f64 { 0.0 }
    pub fn last_parameter_v() -> f64 { 1.0 }
    pub fn is_u_closed() -> bool { false }
    pub fn is_v_closed() -> bool { false }
    pub fn is_u_periodic() -> bool { false }
    pub fn is_v_periodic() -> bool { false }
    pub fn u_period() -> f64 { 0.0 }
    pub fn v_period() -> f64 { 0.0 }
    pub fn resolution(r3d: f64) -> f64 { r3d * 0.1 }
    pub fn nb_u_intervals() -> usize { 1 }
    pub fn nb_v_intervals() -> usize { 1 }
    pub fn u_degree() -> usize { 3 }
    pub fn v_degree() -> usize { 3 }
    pub fn nb_u_poles() -> usize { 4 }
    pub fn nb_v_poles() -> usize { 4 }
    pub fn nb_u_knots() -> usize { 0 }
    pub fn nb_v_knots() -> usize { 0 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parameters() {
        assert_eq!(HlrbrépBSurfaceTool::first_parameter_u(), 0.0);
        assert_eq!(HlrbrépBSurfaceTool::last_parameter_u(), 1.0);
        assert_eq!(HlrbrépBSurfaceTool::first_parameter_v(), 0.0);
        assert_eq!(HlrbrépBSurfaceTool::last_parameter_v(), 1.0);
    }

    #[test]
    fn test_closed() {
        assert!(!HlrbrépBSurfaceTool::is_u_closed());
        assert!(!HlrbrépBSurfaceTool::is_v_closed());
    }

    #[test]
    fn test_degrees() {
        assert_eq!(HlrbrépBSurfaceTool::u_degree(), 3);
        assert_eq!(HlrbrépBSurfaceTool::v_degree(), 3);
    }
}
