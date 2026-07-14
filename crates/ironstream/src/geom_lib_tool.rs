// FILE: geom_lib_tool.rs
// occt: GeomLib_Tool
// occt-ref: GeomLib_CheckBSplineCurve, GeomLib_AdjustExtremity
//       GeomLib_DenominatorMultiplier

/// Result of a geometry library check operation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum GeomLibCheckResult {
    #[default]
    NotChecked,
    Valid,
    InvalidTangent,
    InvalidWeight,
    InvalidKnot,
    NullLength,
}

impl GeomLibCheckResult {
    pub fn is_valid(&self) -> bool { *self == Self::Valid }
}

/// Checks a B-spline curve for degenerate tangents and invalid weights.
// occt-ref: GeomLib_CheckBSplineCurve
#[derive(Clone, Debug)]
pub struct GeomLibCheckBSplineCurve {
    pub curve_id: u32,
    pub angle_tolerance: f64,
    pub sin_tolerance: f64,
    pub result: GeomLibCheckResult,
    pub invalid_tangent: bool,
    pub invalid_weight: bool,
}

impl GeomLibCheckBSplineCurve {
    pub fn new(curve_id: u32) -> Self {
        Self {
            curve_id,
            angle_tolerance: 0.001,
            sin_tolerance: 1e-10,
            result: GeomLibCheckResult::NotChecked,
            invalid_tangent: false,
            invalid_weight: false,
        }
    }

    pub fn set_angle_tolerance(&mut self, a: f64) { self.angle_tolerance = a.max(0.0); }

    /// Stub: marks valid for any non-zero curve_id.
    pub fn perform(&mut self) {
        if self.curve_id > 0 {
            self.result = GeomLibCheckResult::Valid;
        } else {
            self.result = GeomLibCheckResult::NullLength;
        }
    }

    pub fn is_done(&self) -> bool { self.result != GeomLibCheckResult::NotChecked }
    pub fn result(&self) -> GeomLibCheckResult { self.result }
    pub fn nb_invalid_tangents(&self) -> usize { if self.invalid_tangent { 1 } else { 0 } }
    pub fn nb_invalid_weights(&self) -> usize { if self.invalid_weight { 1 } else { 0 } }
}

/// Adjusts tangent vectors at curve extremities to avoid oscillations.
// occt-ref: GeomLib_AdjustExtremity
#[derive(Clone, Debug)]
pub struct GeomLibAdjustExtremity {
    pub first_tangent: [f64; 3],
    pub last_tangent: [f64; 3],
    pub first_curvature: [f64; 3],
    pub last_curvature: [f64; 3],
}

impl Default for GeomLibAdjustExtremity {
    fn default() -> Self {
        Self {
            first_tangent: [1.0, 0.0, 0.0],
            last_tangent: [1.0, 0.0, 0.0],
            first_curvature: [0.0; 3],
            last_curvature: [0.0; 3],
        }
    }
}

impl GeomLibAdjustExtremity {
    pub fn new() -> Self { Self::default() }

    pub fn set_first(&mut self, tangent: [f64; 3], curvature: [f64; 3]) {
        self.first_tangent = tangent;
        self.first_curvature = curvature;
    }

    pub fn set_last(&mut self, tangent: [f64; 3], curvature: [f64; 3]) {
        self.last_tangent = tangent;
        self.last_curvature = curvature;
    }

    /// Length of first tangent vector.
    pub fn first_tangent_length(&self) -> f64 {
        let [x,y,z] = self.first_tangent;
        (x*x + y*y + z*z).sqrt()
    }

    /// Length of last tangent vector.
    pub fn last_tangent_length(&self) -> f64 {
        let [x,y,z] = self.last_tangent;
        (x*x + y*y + z*z).sqrt()
    }
}

/// Evaluates a point and its derivatives on a surface at (u, v).
/// occt: GeomLib_Tool
#[derive(Clone, Debug)]
pub struct GeomLibTool {
    pub surface_id: u32,
    pub tolerance: f64,
}

impl GeomLibTool {
    pub fn new(surface_id: u32) -> Self {
        Self { surface_id, tolerance: 1e-7 }
    }

    pub fn set_tolerance(&mut self, t: f64) { self.tolerance = t.max(1e-14); }

    /// Stub: point evaluation returns (u, v, 0.0) for any surface.
    pub fn point_on_surface(&self, u: f64, v: f64) -> [f64; 3] {
        [u, v, 0.0]
    }

    /// Stub: derivatives as unit tangent vectors along u and v.
    pub fn d1(&self, u: f64, v: f64) -> ([f64; 3], [f64; 3], [f64; 3]) {
        let _ = (u, v);
        ([1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 0.0])
    }

    /// Check if two parameters are within tolerance.
    pub fn same_parameter(&self, p1: f64, p2: f64) -> bool {
        (p1 - p2).abs() <= self.tolerance
    }
}

/// Computes a denominator multiplier to reparametrize a rational curve.
/// occt: GeomLib_DenominatorMultiplier
#[derive(Clone, Debug)]
pub struct GeomLibDenominatorMultiplier {
    pub degree: usize,
    pub knots: Vec<f64>,
    pub mults: Vec<usize>,
}

impl GeomLibDenominatorMultiplier {
    pub fn new(degree: usize, knots: Vec<f64>, mults: Vec<usize>) -> Self {
        Self { degree, knots, mults }
    }

    /// Stub: value at parameter t returns 1.0 always.
    pub fn value(&self, _t: f64) -> f64 { 1.0 }

    pub fn nb_knots(&self) -> usize { self.knots.len() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_bspline_perform() {
        let mut c = GeomLibCheckBSplineCurve::new(5);
        assert!(!c.is_done());
        c.perform();
        assert!(c.is_done());
        assert!(c.result().is_valid());
    }

    #[test]
    fn check_bspline_zero_curve() {
        let mut c = GeomLibCheckBSplineCurve::new(0);
        c.perform();
        assert_eq!(c.result(), GeomLibCheckResult::NullLength);
    }

    #[test]
    fn adjust_extremity_tangent_length() {
        let mut a = GeomLibAdjustExtremity::new();
        a.set_first([3.0, 4.0, 0.0], [0.0; 3]);
        assert!((a.first_tangent_length() - 5.0).abs() < 1e-10);
    }

    #[test]
    fn tool_point_on_surface() {
        let t = GeomLibTool::new(1);
        let p = t.point_on_surface(2.0, 3.0);
        assert_eq!(p, [2.0, 3.0, 0.0]);
    }

    #[test]
    fn tool_same_parameter() {
        let t = GeomLibTool::new(1);
        assert!(t.same_parameter(1.0, 1.0 + 1e-10));
        assert!(!t.same_parameter(1.0, 1.0 + 1e-6));
    }
}
