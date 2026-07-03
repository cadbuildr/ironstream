// FILE: rust/ironstream/crates/ironstream/src/geom_approx_curve.rs
//! Pure-Rust zero-dependency stubs for OpenCascade's `Approx_Curve3d` and
//! `Approx_Curve2dOn3dSurface` approximation types.
//!
//! No unsafe code; no third-party dependencies.

// ─────────────────────────── ApproxCurve3dResult ─────────────────────────────

/// Result of a 3-D curve approximation (`Approx_Curve3d::Curve()`).
///
// occt: Approx_Curve3d result container
#[derive(Clone, Debug)]
pub struct ApproxCurve3dResult {
    /// 3-D control poles of the approximated B-spline curve.
    pub poles: Vec<[f64; 3]>,
    /// Knot vector.
    pub knots: Vec<f64>,
    /// Multiplicity vector (same length as `knots`).
    pub mults: Vec<u32>,
    /// B-spline degree.
    pub degree: u32,
    /// Maximum pointwise approximation error.
    pub max_error: f64,
    /// Average pointwise approximation error.
    pub average_error: f64,
    /// Whether the approximation has been performed.
    pub is_done: bool,
}

impl ApproxCurve3dResult {
    /// Construct an empty, not-done result.
    pub fn new() -> Self {
        Self {
            poles: Vec::new(),
            knots: Vec::new(),
            mults: Vec::new(),
            degree: 0,
            max_error: 0.0,
            average_error: 0.0,
            is_done: false,
        }
    }

    /// Return `true` once the approximation is done.
    pub fn is_done(&self) -> bool {
        self.is_done
    }

    /// Return the number of control poles.
    pub fn nb_poles(&self) -> usize {
        self.poles.len()
    }

    /// Return the B-spline degree.
    pub fn degree(&self) -> u32 {
        self.degree
    }

    /// Return the maximum approximation error.
    pub fn max_error(&self) -> f64 {
        self.max_error
    }

    /// Return the average approximation error.
    pub fn average_error(&self) -> f64 {
        self.average_error
    }

    /// Return the pole at index `i`.
    ///
    /// # Panics
    /// Panics if `i >= nb_poles()`.
    pub fn pole(&self, i: usize) -> [f64; 3] {
        self.poles[i]
    }
}

impl Default for ApproxCurve3dResult {
    fn default() -> Self {
        Self::new()
    }
}

// ─────────────────────────── ApproxCurve3d ───────────────────────────────────

/// Approximates a parametric 3-D curve as a B-spline.
///
/// Mirrors the API of `Approx_Curve3d` from OpenCASCADE Technology.
///
// occt: Approx_Curve3d
#[derive(Clone, Debug)]
pub struct ApproxCurve3d {
    /// Maximum allowed approximation tolerance.
    pub tolerance: f64,
    /// Minimum B-spline degree.
    pub degree_min: u32,
    /// Maximum B-spline degree.
    pub degree_max: u32,
    /// Continuity order (default 2 = C2).
    pub continuity: u32,
    result: ApproxCurve3dResult,
}

impl ApproxCurve3d {
    /// Construct a new approximator.
    ///
    /// * `tolerance`  — maximum allowed pointwise deviation.
    /// * `degree_min` — minimum allowed B-spline degree.
    /// * `degree_max` — maximum allowed B-spline degree.
    ///
    /// `continuity` defaults to 2 (C2).
    pub fn new(tolerance: f64, degree_min: u32, degree_max: u32) -> Self {
        Self {
            tolerance,
            degree_min,
            degree_max,
            continuity: 2,
            result: ApproxCurve3dResult::new(),
        }
    }

    /// Run the approximation with `nb_pts` sample points.
    ///
    /// Stub: generates `nb_pts` evenly spaced poles along the X axis on
    /// `[0, 1]` with a uniform clamped knot vector.
    pub fn perform(&mut self, nb_pts: usize) {
        let n = nb_pts.max(2);
        // Generate poles evenly on [0,1] along X axis.
        self.result.poles = (0..n)
            .map(|i| {
                let t = if n <= 1 { 0.0 } else { i as f64 / (n - 1) as f64 };
                [t, 0.0, 0.0]
            })
            .collect();

        // Simple clamped uniform knot vector: [0, 1] with multiplicity degree+1.
        let deg = self.degree_min;
        self.result.knots = vec![0.0, 1.0];
        self.result.mults = vec![deg + 1, deg + 1];
        self.result.degree = deg;
        self.result.max_error = self.tolerance * 0.01;
        self.result.average_error = self.tolerance * 0.005;
        self.result.is_done = true;
    }

    /// Return `true` once `perform` has been called.
    pub fn is_done(&self) -> bool {
        self.result.is_done
    }

    /// Return a reference to the result.
    pub fn result(&self) -> &ApproxCurve3dResult {
        &self.result
    }
}

// ─────────────────────────── ApproxCurve2dResult ─────────────────────────────

/// Result of a 2-D curve approximation.
///
// occt: Approx_Curve2dOn3dSurface result container
#[derive(Clone, Debug)]
pub struct ApproxCurve2dResult {
    /// 2-D control poles of the approximated B-spline curve.
    pub poles: Vec<[f64; 2]>,
    /// Knot vector.
    pub knots: Vec<f64>,
    /// Multiplicity vector.
    pub mults: Vec<u32>,
    /// B-spline degree.
    pub degree: u32,
    /// Maximum pointwise approximation error.
    pub max_error: f64,
    /// Whether the approximation has been performed.
    pub is_done: bool,
}

impl ApproxCurve2dResult {
    /// Construct an empty, not-done result.
    pub fn new() -> Self {
        Self {
            poles: Vec::new(),
            knots: Vec::new(),
            mults: Vec::new(),
            degree: 0,
            max_error: 0.0,
            is_done: false,
        }
    }

    /// Return `true` once the approximation is done.
    pub fn is_done(&self) -> bool {
        self.is_done
    }

    /// Return the number of control poles.
    pub fn nb_poles(&self) -> usize {
        self.poles.len()
    }

    /// Return the B-spline degree.
    pub fn degree(&self) -> u32 {
        self.degree
    }

    /// Return the maximum approximation error.
    pub fn max_error(&self) -> f64 {
        self.max_error
    }

    /// Return the pole at index `i`.
    ///
    /// # Panics
    /// Panics if `i >= nb_poles()`.
    pub fn pole(&self, i: usize) -> [f64; 2] {
        self.poles[i]
    }
}

impl Default for ApproxCurve2dResult {
    fn default() -> Self {
        Self::new()
    }
}

// ─────────────────────────── internal tests ───────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn approx3d_new_stores_params() {
        let a = ApproxCurve3d::new(1e-4, 2, 7);
        assert!((a.tolerance - 1e-4).abs() < 1e-15);
        assert_eq!(a.degree_min, 2);
        assert_eq!(a.degree_max, 7);
        assert_eq!(a.continuity, 2);
    }

    #[test]
    fn approx3d_not_done_before_perform() {
        let a = ApproxCurve3d::new(1e-3, 3, 8);
        assert!(!a.is_done());
        assert_eq!(a.result().nb_poles(), 0);
    }

    #[test]
    fn approx3d_perform_and_result() {
        let mut a = ApproxCurve3d::new(1e-4, 3, 8);
        a.perform(5);
        assert!(a.is_done());
        assert_eq!(a.result().nb_poles(), 5);
        assert_eq!(a.result().degree(), 3);
        assert!((a.result().max_error() - 1e-6).abs() < 1e-10);
    }

    #[test]
    fn result2d_basics() {
        let mut r = ApproxCurve2dResult::new();
        assert!(!r.is_done());
        r.poles = vec![[0.0, 1.0], [0.5, 0.5]];
        r.degree = 1;
        r.is_done = true;
        assert!(r.is_done());
        assert_eq!(r.nb_poles(), 2);
        assert_eq!(r.pole(0), [0.0, 1.0]);
    }
}
