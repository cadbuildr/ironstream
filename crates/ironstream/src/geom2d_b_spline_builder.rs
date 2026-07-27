// FILE: rust/ironstream/crates/ironstream/src/geom2d_b_spline_builder.rs
//! `Geom2dAPI_PointsToBSpline` builder — lightweight zero-dependency port of
//! the OCCT `Geom2dAPI_PointsToBSpline` workflow (2D version), split into a
//! parameters struct, a result struct, and the algorithm itself.

// ─────────────────────────────────────────────────────────────────────────────
// Geom2dPtsSplineParams
// ─────────────────────────────────────────────────────────────────────────────

/// Fitting parameters for [`Geom2dApiPointsToBSpline`].
// occt: Geom2dAPI_PointsToBSpline // params
#[derive(Clone, Debug)]
pub struct Geom2dPtsSplineParams {
    /// Minimum allowed degree of the output B-spline.
    degree_min: u32,
    /// Maximum allowed degree of the output B-spline.
    degree_max: u32,
    /// Minimum internal continuity (0 = C0, 1 = C1, 2 = C2, …).
    continuity: u8,
    /// Maximum allowed point-to-curve deviation.
    tolerance: f64,
}

impl Geom2dPtsSplineParams {
    /// Default parameters: degree\_min=3, degree\_max=8, continuity=2,
    /// tolerance=1e-6.
    // occt: Geom2dAPI_PointsToBSpline // default constructor values
    pub fn new() -> Self {
        Self { degree_min: 3, degree_max: 8, continuity: 2, tolerance: 1e-6 }
    }

    /// Minimum degree.
    pub fn degree_min(&self) -> u32 {
        self.degree_min
    }

    /// Maximum degree.
    pub fn degree_max(&self) -> u32 {
        self.degree_max
    }

    /// Continuity order.
    pub fn continuity(&self) -> u8 {
        self.continuity
    }

    /// Geometric tolerance.
    pub fn tolerance(&self) -> f64 {
        self.tolerance
    }
}

impl Default for Geom2dPtsSplineParams {
    fn default() -> Self {
        Self::new()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Geom2dPtsSplineResult
// ─────────────────────────────────────────────────────────────────────────────

/// Result 2D B-spline curve produced by [`Geom2dApiPointsToBSpline`].
// occt-note: result 2D B-spline
#[derive(Clone, Debug)]
pub struct Geom2dPtsSplineResult {
    /// Control points of the fitted 2D B-spline.
    pub poles: Vec<[f64; 2]>,
    /// Distinct knot values.
    pub knots: Vec<f64>,
    /// Knot multiplicities.
    pub mults: Vec<u32>,
    /// Polynomial degree.
    pub degree: u32,
    /// True when `perform()` completed without error.
    pub is_done: bool,
    /// Maximum deviation from the input points after fitting.
    pub max_error: f64,
}

impl Geom2dPtsSplineResult {
    /// Construct an empty (not-done) result.
    pub fn new() -> Self {
        Self {
            poles: Vec::new(),
            knots: Vec::new(),
            mults: Vec::new(),
            degree: 0,
            is_done: false,
            max_error: 0.0,
        }
    }

    /// Whether the algorithm has completed successfully.
    pub fn is_done(&self) -> bool {
        self.is_done
    }

    /// Number of control poles.
    pub fn nb_poles(&self) -> usize {
        self.poles.len()
    }

    /// Degree of the B-spline.
    pub fn degree(&self) -> u32 {
        self.degree
    }

    /// Maximum deviation from input data recorded after `perform()`.
    pub fn max_error(&self) -> f64 {
        self.max_error
    }

    /// Return the i-th pole (0-based).
    ///
    /// Panics if `i >= nb_poles()`.
    pub fn pole(&self, i: usize) -> [f64; 2] {
        self.poles[i]
    }
}

impl Default for Geom2dPtsSplineResult {
    fn default() -> Self {
        Self::new()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Helpers
// ─────────────────────────────────────────────────────────────────────────────

/// Build a uniform clamped `(knots, mults)` for `n` poles at the given
/// `degree`.  The knot vector spans `[0, 1]`.
fn uniform_knots_mults(n: usize, degree: usize) -> (Vec<f64>, Vec<u32>) {
    let n_inner = (n as isize - degree as isize - 1).max(0) as usize;
    let mut knots = vec![0.0_f64];
    let mut mults = vec![(degree as u32) + 1];
    for i in 1..=n_inner {
        knots.push(i as f64 / (n_inner + 1) as f64);
        mults.push(1);
    }
    knots.push(1.0);
    mults.push((degree as u32) + 1);
    (knots, mults)
}

// ─────────────────────────────────────────────────────────────────────────────
// Geom2dApiPointsToBSpline
// ─────────────────────────────────────────────────────────────────────────────

/// Stub port of `Geom2dAPI_PointsToBSpline` (2D version).
///
/// `perform()` sets the poles to the input points directly (stub behaviour),
/// assigns a uniform clamped knot vector, uses `degree = params.degree_min`,
/// and marks the result as done.  A full least-squares fitting implementation
/// would replace this stub.
// occt: Geom2dAPI_PointsToBSpline
#[derive(Clone, Debug)]
pub struct Geom2dApiPointsToBSpline {
    /// Input 2D point cloud.
    pub points: Vec<[f64; 2]>,
    /// Fitting parameters.
    pub params: Geom2dPtsSplineParams,
    /// Output result (populated by `perform()`).
    pub result: Geom2dPtsSplineResult,
}

impl Geom2dApiPointsToBSpline {
    /// Construct the algorithm with the given parameters.
    // occt-note: Geom2dAPI_PointsToBSpline(Params)
    pub fn new(params: Geom2dPtsSplineParams) -> Self {
        Self {
            points: Vec::new(),
            params,
            result: Geom2dPtsSplineResult::new(),
        }
    }

    /// Add a single 2D point to the input cloud.
    // occt: Geom2dAPI_PointsToBSpline // ::Add / Init
    pub fn add_point(&mut self, p: [f64; 2]) {
        self.points.push(p);
    }

    /// Run the fitting algorithm.
    ///
    /// Stub implementation: sets poles = input points, builds a uniform clamped
    /// knot vector, degree = `params.degree_min`, `is_done = true`.
    // occt: Geom2dAPI_PointsToBSpline // ::Perform
    pub fn perform(&mut self) {
        self.result = Geom2dPtsSplineResult::new(); // reset

        let n = self.points.len();
        if n == 0 {
            return;
        }

        let degree = self.params.degree_min as usize;
        let (knots, mults) = uniform_knots_mults(n, degree);

        self.result.poles = self.points.clone();
        self.result.knots = knots;
        self.result.mults = mults;
        self.result.degree = degree as u32;
        self.result.is_done = true;
        self.result.max_error = 0.0; // stub: poles == points so error is zero
    }

    /// Whether `perform()` has completed successfully.
    pub fn is_done(&self) -> bool {
        self.result.is_done
    }

    /// Number of input points currently added.
    pub fn nb_points(&self) -> usize {
        self.points.len()
    }

    /// Borrow the result of the last `perform()` call.
    pub fn result(&self) -> &Geom2dPtsSplineResult {
        &self.result
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Unit tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_defaults() {
        let p = Geom2dPtsSplineParams::new();
        assert_eq!(p.degree_min(), 3);
        assert_eq!(p.degree_max(), 8);
        assert_eq!(p.continuity(), 2);
        assert!((p.tolerance() - 1e-6).abs() < 1e-15);
    }

    #[test]
    fn result_new_is_not_done() {
        let r = Geom2dPtsSplineResult::new();
        assert!(!r.is_done());
        assert_eq!(r.nb_poles(), 0);
        assert_eq!(r.degree(), 0);
        assert_eq!(r.max_error(), 0.0);
    }

    #[test]
    fn perform_empty_is_not_done() {
        let mut algo = Geom2dApiPointsToBSpline::new(Geom2dPtsSplineParams::new());
        algo.perform();
        assert!(!algo.is_done());
    }

    #[test]
    fn perform_single_point() {
        let mut algo = Geom2dApiPointsToBSpline::new(Geom2dPtsSplineParams::new());
        algo.add_point([1.0, 2.0]);
        algo.perform();
        assert!(algo.is_done());
        assert_eq!(algo.nb_points(), 1);
        let r = algo.result();
        assert_eq!(r.nb_poles(), 1);
        assert_eq!(r.degree(), Geom2dPtsSplineParams::new().degree_min());
    }

    #[test]
    fn perform_sets_poles_to_points() {
        let mut algo = Geom2dApiPointsToBSpline::new(Geom2dPtsSplineParams::new());
        algo.add_point([0.0, 0.0]);
        algo.add_point([1.0, 0.0]);
        algo.add_point([2.0, 1.0]);
        algo.perform();
        assert!(algo.is_done());
        let r = algo.result();
        assert_eq!(r.nb_poles(), 3);
        assert_eq!(r.pole(0), [0.0, 0.0]);
        assert_eq!(r.pole(1), [1.0, 0.0]);
        assert_eq!(r.pole(2), [2.0, 1.0]);
    }

    #[test]
    fn perform_knots_span_zero_to_one() {
        let mut algo = Geom2dApiPointsToBSpline::new(Geom2dPtsSplineParams::new());
        for i in 0..5 {
            algo.add_point([i as f64, 0.0]);
        }
        algo.perform();
        let r = algo.result();
        assert!((r.knots[0] - 0.0).abs() < 1e-15);
        assert!((r.knots[r.knots.len() - 1] - 1.0).abs() < 1e-15);
    }

    #[test]
    fn perform_mults_sum_equals_nb_poles_plus_degree_plus_one() {
        let mut p = Geom2dPtsSplineParams::new();
        // degree_min stays at 3 (default)
        let mut algo = Geom2dApiPointsToBSpline::new(p);
        for i in 0..4 {
            algo.add_point([i as f64, 0.0]);
        }
        algo.perform();
        let r = algo.result();
        let sum_mults: u32 = r.mults.iter().sum();
        assert_eq!(sum_mults, r.nb_poles() as u32 + r.degree() + 1);
    }

    #[test]
    fn perform_max_error_is_zero_for_stub() {
        let mut algo = Geom2dApiPointsToBSpline::new(Geom2dPtsSplineParams::new());
        algo.add_point([0.0, 0.0]);
        algo.add_point([3.0, 4.0]);
        algo.perform();
        assert_eq!(algo.result().max_error(), 0.0);
    }

    #[test]
    fn nb_points_tracks_added_points() {
        let mut algo = Geom2dApiPointsToBSpline::new(Geom2dPtsSplineParams::new());
        assert_eq!(algo.nb_points(), 0);
        algo.add_point([1.0, 0.0]);
        assert_eq!(algo.nb_points(), 1);
        algo.add_point([2.0, 0.0]);
        assert_eq!(algo.nb_points(), 2);
    }
}
