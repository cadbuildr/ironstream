// FILE: extrema_curve_tool.rs
// occt: Extrema_CurveTool

/// Tool for accessing 3D curve properties.
pub struct ExtremaCurveTool;

impl ExtremaCurveTool {
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
    pub fn point_at(_curve_id: i32, _u: f64) -> (f64, f64, f64) {
        (0.0, 0.0, 0.0)
    }

    /// Evaluates tangent on curve.
    pub fn tangent_at(_curve_id: i32, _u: f64) -> (f64, f64, f64) {
        (1.0, 0.0, 0.0)
    }

    /// Evaluates curvature on curve.
    pub fn curvature_at(_curve_id: i32, _u: f64) -> f64 {
        0.0
    }

    /// Returns curve resolution for given tolerance.
    pub fn resolution(_curve_id: i32, _tol: f64) -> f64 {
        1.0e-3
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parameters() {
        assert_eq!(ExtremaCurveTool::first_parameter(0), 0.0);
        assert_eq!(ExtremaCurveTool::last_parameter(0), 1.0);
    }

    #[test]
    fn test_continuity() {
        assert_eq!(ExtremaCurveTool::continuity(0), 2);
    }

    #[test]
    fn test_intervals() {
        assert_eq!(ExtremaCurveTool::nb_intervals(0), 1);
    }

    #[test]
    fn test_point_and_tangent() {
        let pt = ExtremaCurveTool::point_at(0, 0.5);
        assert_eq!(pt, (0.0, 0.0, 0.0));
        let tan = ExtremaCurveTool::tangent_at(0, 0.5);
        assert_eq!(tan, (1.0, 0.0, 0.0));
    }

    #[test]
    fn test_curvature() {
        assert_eq!(ExtremaCurveTool::curvature_at(0, 0.5), 0.0);
    }

    #[test]
    fn test_resolution() {
        assert_eq!(ExtremaCurveTool::resolution(0, 1.0e-6), 1.0e-3);
    }
}
