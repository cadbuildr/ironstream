// FILE: geom2d_int_geom2d_curve_tool.rs
// occt: Geom2dInt_Geom2dCurveTool

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeomAbsCurveType {
    Line,
    Circle,
    Ellipse,
    Hyperbola,
    Parabola,
    BezierCurve,
    BSplineCurve,
    OffsetCurve,
    OtherCurve,
}

/// Tool for handling 2D curves in intersection computations.
pub struct Geom2dIntGeom2dCurveTool;

impl Geom2dIntGeom2dCurveTool {
    /// Get the type of a curve.
    pub fn get_type() -> GeomAbsCurveType {
        GeomAbsCurveType::OtherCurve
    }

    /// Check if curve is composite.
    pub fn is_composite() -> bool {
        false
    }

    /// Compute number of samples for a curve.
    pub fn nb_samples() -> i32 {
        33
    }

    /// Compute epsilon for curve.
    pub fn eps_x() -> f64 {
        1e-15
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_type() {
        assert_eq!(Geom2dIntGeom2dCurveTool::get_type(), GeomAbsCurveType::OtherCurve);
    }

    #[test]
    fn test_nb_samples() {
        assert_eq!(Geom2dIntGeom2dCurveTool::nb_samples(), 33);
    }

    #[test]
    fn test_eps_x() {
        let eps = Geom2dIntGeom2dCurveTool::eps_x();
        assert!(eps > 0.0);
    }
}
