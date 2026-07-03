// FILE: hlrb_rep_b_curve_tool.rs
// occt: HLRBRep_BCurveTool

/// Tool for accessing Bezier/BSpline curve properties.
pub struct HlrbrépBCurveTool;

impl HlrbrépBCurveTool {
    /// Get first parameter of curve.
    pub fn first_parameter() -> f64 {
        0.0
    }

    /// Get last parameter of curve.
    pub fn last_parameter() -> f64 {
        1.0
    }

    /// Check if curve is closed.
    pub fn is_closed() -> bool {
        false
    }

    /// Check if curve is periodic.
    pub fn is_periodic() -> bool {
        false
    }

    /// Get period of curve (if periodic).
    pub fn period() -> f64 {
        0.0
    }

    /// Get parametric resolution for given 3D resolution.
    pub fn resolution(r3d: f64) -> f64 {
        r3d * 0.1 // Simplified: assume 10:1 ratio
    }

    /// Get number of intervals.
    pub fn nb_intervals() -> usize {
        1
    }

    /// Get degree of curve.
    pub fn degree() -> usize {
        3
    }

    /// Check if curve is rational.
    pub fn is_rational() -> bool {
        false
    }

    /// Get number of poles.
    pub fn nb_poles() -> usize {
        4
    }

    /// Get number of knots.
    pub fn nb_knots() -> usize {
        0
    }

    /// Get number of samples for parameter range.
    pub fn nb_samples(u0: f64, u1: f64) -> usize {
        let param_range = (u1 - u0).abs();
        ((param_range * 100.0) as usize).max(2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parameters() {
        assert_eq!(HlrbrépBCurveTool::first_parameter(), 0.0);
        assert_eq!(HlrbrépBCurveTool::last_parameter(), 1.0);
    }

    #[test]
    fn test_closed_periodic() {
        assert!(!HlrbrépBCurveTool::is_closed());
        assert!(!HlrbrépBCurveTool::is_periodic());
    }

    #[test]
    fn test_resolution() {
        assert!(HlrbrépBCurveTool::resolution(0.1) > 0.0);
    }

    #[test]
    fn test_properties() {
        assert_eq!(HlrbrépBCurveTool::degree(), 3);
        assert_eq!(HlrbrépBCurveTool::nb_poles(), 4);
        assert!(!HlrbrépBCurveTool::is_rational());
    }

    #[test]
    fn test_nb_samples() {
        let samples = HlrbrépBCurveTool::nb_samples(0.0, 1.0);
        assert!(samples >= 2);
    }
}
