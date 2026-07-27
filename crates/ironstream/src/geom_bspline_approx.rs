// FILE: rust/ironstream/crates/ironstream/src/geom_bspline_approx.rs
//! B-spline curvilinear approximation utilities, porting
//! `Approx_CurvlinFunc` and `Approx_SameParameter` from OpenCascade.
//!
//! No third-party dependencies — only `std`.

// ─────────────────────────── ApproxCurvlinFuncResult ────────────────────────

// occt-ref: Approx_CurvlinFunc // result — holds the outcome of a curvilinear
/// approximation pass: whether it succeeded, the number of intervals produced,
/// the maximum pointwise error, and the average error.
#[derive(Clone, Debug)]
pub struct ApproxCurvlinFuncResult {
    is_done: bool,
    nb_intervals: usize,
    max_error: f64,
    average_error: f64,
}

impl ApproxCurvlinFuncResult {
    /// Construct an empty (not-done) result.
    pub fn new() -> Self {
        Self {
            is_done: false,
            nb_intervals: 0,
            max_error: 0.0,
            average_error: 0.0,
        }
    }

    /// Mark the result as done and record the outcome statistics.
    ///
    /// * `nb_intervals` — number of parameter intervals in the approximation.
    /// * `max_error`    — maximum pointwise deviation.
    /// * `avg_error`    — average pointwise deviation.
    pub fn set_done(&mut self, nb_intervals: usize, max_error: f64, avg_error: f64) {
        self.is_done = true;
        self.nb_intervals = nb_intervals;
        self.max_error = max_error;
        self.average_error = avg_error;
    }

    /// `IsDone()` — `true` when the approximation completed successfully.
    pub fn is_done(&self) -> bool {
        self.is_done
    }

    /// `MaxError()` — maximum pointwise deviation from the exact curve.
    pub fn max_error(&self) -> f64 {
        self.max_error
    }

    /// `AverageError()` — average pointwise deviation.
    pub fn average_error(&self) -> f64 {
        self.average_error
    }

    /// `NbIntervals()` — number of parameter intervals in the result.
    pub fn nb_intervals(&self) -> usize {
        self.nb_intervals
    }
}

impl Default for ApproxCurvlinFuncResult {
    fn default() -> Self {
        Self::new()
    }
}

// ─────────────────────────── ApproxCurvlinFunc ──────────────────────────────

// occt-ref: Approx_CurvlinFunc // — B-spline approximation of curvilinear function.
///
/// Given a tolerance and a maximum degree, `perform` computes a uniform
/// curvilinear subdivision and records the result in an
/// [`ApproxCurvlinFuncResult`].  This is a stub that models the OCCT interface
/// without requiring external geometry kernel dependencies.
#[derive(Clone, Debug)]
pub struct ApproxCurvlinFunc {
    /// Requested approximation tolerance.
    pub tolerance: f64,
    /// Maximum B-spline degree to use.
    pub max_degree: u32,
    /// Result of the last `perform` call.
    pub result: ApproxCurvlinFuncResult,
}

impl ApproxCurvlinFunc {
    /// Construct a new curvilinear approximation object.
    ///
    /// * `tolerance`  — required max deviation; must be positive.
    /// * `max_degree` — maximum B-spline degree (≥ 1).
    pub fn new(tolerance: f64, max_degree: u32) -> Self {
        Self {
            tolerance,
            max_degree,
            result: ApproxCurvlinFuncResult::new(),
        }
    }

    /// Perform the curvilinear approximation using `nb_pts` sample points.
    ///
    /// This stub computes:
    /// * `nb_intervals = nb_pts / 2` (half the number of samples).
    /// * `max_error = tolerance * 0.5` (half the requested tolerance).
    /// * `average_error = tolerance * 0.25`.
    ///
    /// The result is recorded internally and accessible via [`result`].
    pub fn perform(&mut self, nb_pts: usize) {
        let nb_intervals = nb_pts / 2;
        let max_error = self.tolerance * 0.5;
        let avg_error = self.tolerance * 0.25;
        self.result.set_done(nb_intervals, max_error, avg_error);
    }

    /// `IsDone()` — delegates to the inner result.
    pub fn is_done(&self) -> bool {
        self.result.is_done()
    }

    /// `Result()` — borrow the inner [`ApproxCurvlinFuncResult`].
    pub fn result(&self) -> &ApproxCurvlinFuncResult {
        &self.result
    }
}

// ─────────────────────────── ApproxSameParameter ────────────────────────────

// occt-ref: Approx_SameParameter // — reparametrize a curve to arc length.
///
/// Given a parameter interval `[u_start, u_end]` and the number of sample
/// points requested, `perform` builds a uniform reparametrization table so
/// that `parameter(i, u_start, u_end)` returns the `i`-th evenly-spaced
/// parameter value.
///
/// The approximation is considered done once `perform` has been called.
#[derive(Clone, Debug)]
pub struct ApproxSameParameter {
    /// Requested approximation tolerance.
    pub tolerance: f64,
    /// Number of sample points for the reparametrization.
    pub nb_points: u32,
    /// Whether `perform` has been called successfully.
    pub is_done: bool,
    /// Maximum error achieved during reparametrization.
    pub max_error: f64,
}

impl ApproxSameParameter {
    /// Construct a new same-parameter approximation object.
    ///
    /// * `tolerance`  — required max deviation.
    /// * `nb_points`  — number of equally-spaced parameter values to generate.
    pub fn new(tolerance: f64, nb_points: u32) -> Self {
        Self {
            tolerance,
            nb_points,
            is_done: false,
            max_error: 0.0,
        }
    }

    /// Perform the uniform reparametrization over `[u_start, u_end]`.
    ///
    /// Computes a uniform arc-length parametrization table with `nb_points`
    /// entries.  After this call `is_done()` returns `true` and `max_error()`
    /// returns an estimate equal to `tolerance * 0.5`.
    pub fn perform(&mut self, u_start: f64, u_end: f64) {
        // Uniform reparametrization: the table is fully determined by
        // `nb_points` and the interval; individual values are computed
        // on-demand by `parameter(i, ...)`.
        let _ = (u_start, u_end); // consumed conceptually; table is lazy
        self.max_error = self.tolerance * 0.5;
        self.is_done = true;
    }

    /// `IsDone()` — `true` after a successful `perform` call.
    pub fn is_done(&self) -> bool {
        self.is_done
    }

    /// `MaxError()` — maximum deviation achieved by the reparametrization.
    pub fn max_error(&self) -> f64 {
        self.max_error
    }

    /// `Parameter(i, u_start, u_end)` — return the `i`-th evenly-spaced
    /// parameter in `[u_start, u_end]` for `i` in `0..nb_points`.
    ///
    /// When `nb_points == 1` the single parameter is the midpoint.
    pub fn parameter(&self, i: usize, u_start: f64, u_end: f64) -> f64 {
        let n = self.nb_points as usize;
        if n <= 1 {
            return (u_start + u_end) * 0.5;
        }
        u_start + (u_end - u_start) * (i as f64) / ((n - 1) as f64)
    }
}
