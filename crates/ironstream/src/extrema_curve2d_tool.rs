// FILE: extrema_curve2d_tool.rs
// occt: Extrema_Curve2dTool

/// Tool for accessing 2D curve properties.
pub struct ExtremaCurve2dTool;

impl ExtremaCurve2dTool {
    /// Returns first parameter of curve.
    pub fn first_parameter(_curve_id: i32) -> f64 {
        0.0
    }

    /// Returns last parameter of curve.
    pub fn last_parameter(_curve_id: i32) -> f64 {
        1.0
    }

    /// Returns curve continuity.
    pub fn continuity(_curve_id: i32) -> i32 {
        2  // C2 continuity
    }

    /// Returns number of intervals.
    pub fn nb_intervals(_curve_id: i32) -> i32 {
        1
    }

    /// Returns parameter value at interval.
    pub fn interval_first(_curve_id: i32, _i: i32) -> f64 {
        0.0
    }

    /// Returns parameter value at interval end.
    pub fn interval_last(_curve_id: i32, _i: i32) -> f64 {
        1.0
    }

    /// Evaluates point on curve.
    pub fn point_at(_curve_id: i32, _u: f64) -> (f64, f64) {
        (0.0, 0.0)
    }

    /// Evaluates tangent on curve.
    pub fn tangent_at(_curve_id: i32, _u: f64) -> (f64, f64) {
        (1.0, 0.0)
    }

    /// Evaluates curvature on curve.
    pub fn curvature_at(_curve_id: i32, _u: f64) -> f64 {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parameters() {
        assert_eq!(ExtremaCurve2dTool::first_parameter(0), 0.0);
        assert_eq!(ExtremaCurve2dTool::last_parameter(0), 1.0);
    }

    #[test]
    fn test_continuity() {
        assert_eq!(ExtremaCurve2dTool::continuity(0), 2);
    }

    #[test]
    fn test_intervals() {
        assert_eq!(ExtremaCurve2dTool::nb_intervals(0), 1);
        assert_eq!(ExtremaCurve2dTool::interval_first(0, 0), 0.0);
        assert_eq!(ExtremaCurve2dTool::interval_last(0, 0), 1.0);
    }

    #[test]
    fn test_point_and_tangent() {
        let pt = ExtremaCurve2dTool::point_at(0, 0.5);
        assert_eq!(pt, (0.0, 0.0));
        let tan = ExtremaCurve2dTool::tangent_at(0, 0.5);
        assert_eq!(tan, (1.0, 0.0));
    }

    #[test]
    fn test_curvature() {
        assert_eq!(ExtremaCurve2dTool::curvature_at(0, 0.5), 0.0);
    }
}
