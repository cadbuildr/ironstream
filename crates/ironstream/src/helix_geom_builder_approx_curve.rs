// FILE: helix_geom_builder_approx_curve.rs
// occt: HelixGeom_BuilderApproxCurve

/// Represents continuity requirements for curve approximation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GeomAbsShape {
    C0 = 0,
    C1 = 1,
    C2 = 2,
    C3 = 3,
    CN = 4,
}

/// Represents a BSpline curve (placeholder for complex curve type).
#[derive(Clone, Debug)]
pub struct GeomCurve {
    pub id: usize,
}

/// Base class for helix curve approximation algorithms.
///
/// This abstract-like class provides common functionality for approximating
/// parametric helix curves using B-spline curves. It manages:
/// - Approximation tolerance and parameters
/// - Continuity requirements (C0, C1, C2)
/// - Maximum degree and number of segments
/// - Error and warning status reporting
/// - Result curve storage
pub struct HelixGeomBuilderApproxCurve {
    error_status: i32,
    warning_status: i32,
    tolerance: f64,
    continuity: GeomAbsShape,
    max_degree: i32,
    max_seg: i32,
    tolerance_reached: f64,
    curves: Vec<GeomCurve>,
}

impl HelixGeomBuilderApproxCurve {
    /// Creates a new builder with default parameters.
    pub fn new() -> Self {
        HelixGeomBuilderApproxCurve {
            error_status: 0,
            warning_status: 0,
            tolerance: 1e-6,
            continuity: GeomAbsShape::C2,
            max_degree: 8,
            max_seg: 100,
            tolerance_reached: 0.0,
            curves: Vec::new(),
        }
    }

    /// Sets approximation parameters.
    pub fn set_approx_parameters(&mut self, continuity: GeomAbsShape, max_degree: i32, max_seg: i32) {
        self.continuity = continuity;
        self.max_degree = max_degree;
        self.max_seg = max_seg;
    }

    /// Gets approximation parameters.
    pub fn approx_parameters(&self) -> (GeomAbsShape, i32, i32) {
        (self.continuity, self.max_degree, self.max_seg)
    }

    /// Sets approximation tolerance.
    pub fn set_tolerance(&mut self, tolerance: f64) {
        self.tolerance = tolerance;
    }

    /// Gets approximation tolerance.
    pub fn tolerance(&self) -> f64 {
        self.tolerance
    }

    /// Gets actual tolerance reached by approximation algorithm.
    pub fn tolerance_reached(&self) -> f64 {
        self.tolerance_reached
    }

    /// Gets sequence of BSpline curves representing helix coils.
    pub fn curves(&self) -> &[GeomCurve] {
        &self.curves
    }

    /// Returns error status of algorithm.
    pub fn error_status(&self) -> i32 {
        self.error_status
    }

    /// Returns warning status of algorithm.
    pub fn warning_status(&self) -> i32 {
        self.warning_status
    }

    /// Internal method to set error status.
    fn set_error_status(&mut self, status: i32) {
        self.error_status = status;
    }

    /// Internal method to set warning status.
    fn set_warning_status(&mut self, status: i32) {
        self.warning_status = status;
    }

    /// Internal method to set tolerance reached.
    fn set_tolerance_reached(&mut self, tol: f64) {
        self.tolerance_reached = tol;
    }

    /// Internal method to add a curve.
    fn add_curve(&mut self, curve: GeomCurve) {
        self.curves.push(curve);
    }
}

impl Default for HelixGeomBuilderApproxCurve {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_builder() {
        let builder = HelixGeomBuilderApproxCurve::new();
        assert_eq!(builder.error_status(), 0);
        assert_eq!(builder.warning_status(), 0);
        assert_eq!(builder.tolerance(), 1e-6);
        assert!(builder.curves().is_empty());
    }

    #[test]
    fn test_set_approx_parameters() {
        let mut builder = HelixGeomBuilderApproxCurve::new();
        builder.set_approx_parameters(GeomAbsShape::C1, 6, 50);

        let (cont, degree, segs) = builder.approx_parameters();
        assert_eq!(cont, GeomAbsShape::C1);
        assert_eq!(degree, 6);
        assert_eq!(segs, 50);
    }

    #[test]
    fn test_get_approx_parameters() {
        let builder = HelixGeomBuilderApproxCurve::new();
        let (cont, degree, segs) = builder.approx_parameters();
        assert_eq!(cont, GeomAbsShape::C2);
        assert_eq!(degree, 8);
        assert_eq!(segs, 100);
    }

    #[test]
    fn test_set_tolerance() {
        let mut builder = HelixGeomBuilderApproxCurve::new();
        builder.set_tolerance(0.001);
        assert_eq!(builder.tolerance(), 0.001);
    }

    #[test]
    fn test_tolerance_reached() {
        let mut builder = HelixGeomBuilderApproxCurve::new();
        // Internal method simulation
        builder.set_tolerance_reached(0.0005);
        assert_eq!(builder.tolerance_reached(), 0.0005);
    }

    #[test]
    fn test_error_and_warning_status() {
        let mut builder = HelixGeomBuilderApproxCurve::new();
        assert_eq!(builder.error_status(), 0);
        assert_eq!(builder.warning_status(), 0);

        builder.set_error_status(1);
        builder.set_warning_status(2);

        assert_eq!(builder.error_status(), 1);
        assert_eq!(builder.warning_status(), 2);
    }

    #[test]
    fn test_add_curves() {
        let mut builder = HelixGeomBuilderApproxCurve::new();
        let curve1 = GeomCurve { id: 1 };
        let curve2 = GeomCurve { id: 2 };

        builder.add_curve(curve1);
        builder.add_curve(curve2);

        assert_eq!(builder.curves().len(), 2);
        assert_eq!(builder.curves()[0].id, 1);
        assert_eq!(builder.curves()[1].id, 2);
    }

    #[test]
    fn test_default() {
        let builder = HelixGeomBuilderApproxCurve::default();
        assert_eq!(builder.error_status(), 0);
        assert!(builder.curves().is_empty());
    }
}
