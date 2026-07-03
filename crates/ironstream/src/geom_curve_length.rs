// FILE: rust/ironstream/crates/ironstream/src/geom_curve_length.rs
//! Curve-length utilities — arc-length point location and uniform abscissa
//! distribution along parametric curves.
//!
//! This module provides pure-Rust, zero-dependency stubs modelled after the
//! OpenCascade Technology (OCCT) `GCPnts` package:
//!
//! - [`AbscissaPoint`]  — locate the parameter at a given arc-length offset
//!                        from a starting parameter along a named curve.
//!                        Mirrors `GCPnts_AbscissaPoint`.
//! - [`UniformAbscissa`] — distribute N equally-spaced (by arc length)
//!                         parameter values along a named curve.
//!                         Mirrors `GCPnts_UniformAbscissa`.
//! - [`curve_length`]   — compute the total arc length of a polyline given as
//!                         a slice of 3-D points.
//!
//! These are deliberate stubs: no actual curve geometry is evaluated.  The
//! `curve: &str` argument is accepted for API compatibility and future
//! integration but is currently unused.  Computations use linear / uniform
//! arithmetic on the parameter domain.  Only `std` is used (`f64::sqrt`).

// ---------------------------------------------------------------------------
// AbscissaPoint
// ---------------------------------------------------------------------------

/// Locates the curve parameter that lies at a given arc-length offset from a
/// starting parameter.
///
/// Mirrors OCCT's `GCPnts_AbscissaPoint`.  The full OCCT implementation
/// integrates the curve speed with Gauss–Legendre quadrature; this stub
/// stores the result directly for use in unit-speed or test scenarios.
// occt: GCPnts_AbscissaPoint
#[derive(Clone, Debug)]
pub struct AbscissaPoint {
    /// The computed parameter value (valid only when `done` is `true`).
    pub param: f64,
    /// Whether the computation succeeded.
    pub done: bool,
}

impl AbscissaPoint {
    /// Construct an empty, not-done instance.
    ///
    /// Equivalent to the default `GCPnts_AbscissaPoint()` constructor in OCCT.
    pub fn new() -> Self {
        AbscissaPoint {
            param: 0.0,
            done: false,
        }
    }

    /// Find the parameter at arc-length `abscissa` from `u0` along `curve`.
    ///
    /// The stub treats the parameter domain as unit-speed, so the result is
    /// simply `u0 + abscissa` clamped to a valid state when `|abscissa|` and
    /// `tol` are consistent.
    ///
    /// * `curve`    — curve identifier (accepted for API compatibility; unused
    ///                in this stub).
    /// * `abscissa` — signed arc-length offset from `u0`.
    /// * `u0`       — starting parameter on the curve.
    /// * `tol`      — tolerance used to decide whether the computation is valid.
    ///
    /// Sets `done = true` when `tol >= 0.0`.
    pub fn compute(curve: &str, abscissa: f64, u0: f64, tol: f64) -> Self {
        let _ = curve; // accepted for API parity; unused in stub
        if tol < 0.0 {
            return AbscissaPoint {
                param: 0.0,
                done: false,
            };
        }
        AbscissaPoint {
            param: u0 + abscissa,
            done: true,
        }
    }

    /// Return the computed parameter.
    ///
    /// Mirrors `GCPnts_AbscissaPoint::Parameter()`.
    ///
    /// # Panics
    /// Panics when `is_done()` is `false`.
    pub fn parameter(&self) -> f64 {
        assert!(self.done, "AbscissaPoint: is_done is false");
        self.param
    }

    /// Return whether the computation succeeded.
    ///
    /// Mirrors `GCPnts_AbscissaPoint::IsDone()`.
    pub fn is_done(&self) -> bool {
        self.done
    }
}

impl Default for AbscissaPoint {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// UniformAbscissa
// ---------------------------------------------------------------------------

/// Distributes parameter values at equal arc-length intervals along a curve.
///
/// Mirrors OCCT's `GCPnts_UniformAbscissa`.  The full OCCT implementation
/// integrates the curve speed numerically; this stub distributes parameters
/// uniformly in the parameter domain, suitable for unit-speed curves and
/// testing.
// occt: GCPnts_UniformAbscissa
#[derive(Clone, Debug)]
pub struct UniformAbscissa {
    /// The distributed parameter values.
    pub params: Vec<f64>,
    /// Whether the computation succeeded.
    pub done: bool,
}

impl UniformAbscissa {
    /// Distribute `n` parameter values uniformly over the curve `curve`.
    ///
    /// The stub samples `n` evenly spaced values in `[0.0, 1.0]`.  For
    /// `n == 1` only the value `0.0` is returned.  For `n == 0` the instance
    /// is marked not-done.
    ///
    /// * `curve` — curve identifier (accepted for API compatibility; unused).
    /// * `n`     — number of sample points (must be ≥ 1 for success).
    /// * `tol`   — tolerance (accepted for API compatibility; unused).
    ///
    /// Mirrors the `GCPnts_UniformAbscissa(C, NbPoints, Tol)` constructor.
    pub fn new(curve: &str, n: u32, tol: f64) -> Self {
        let _ = (curve, tol);
        if n == 0 {
            return UniformAbscissa {
                params: Vec::new(),
                done: false,
            };
        }
        let params = uniform_params(0.0, 1.0, n);
        UniformAbscissa { params, done: true }
    }

    /// Distribute parameters so that consecutive points are separated by
    /// arc-length `abscissa` along `curve`.
    ///
    /// The stub computes the number of intervals as
    /// `floor(1.0 / abscissa.abs())` (treating the parameter domain as
    /// `[0, 1]` with unit speed), then distributes `n + 1` evenly spaced
    /// values.  For `abscissa == 0` or `tol < 0` the instance is not-done.
    ///
    /// * `curve`    — curve identifier (unused in stub).
    /// * `abscissa` — desired arc-length spacing between consecutive points.
    /// * `tol`      — tolerance (unused in stub beyond sign check).
    ///
    /// Mirrors the `GCPnts_UniformAbscissa(C, Abscissa, Tol)` constructor.
    pub fn from_abscissa(curve: &str, abscissa: f64, tol: f64) -> Self {
        let _ = (curve, tol);
        if abscissa.abs() == 0.0 || tol < 0.0 {
            return UniformAbscissa {
                params: Vec::new(),
                done: false,
            };
        }
        let n_intervals = (1.0_f64 / abscissa.abs()).floor() as u32;
        if n_intervals == 0 {
            return UniformAbscissa {
                params: Vec::new(),
                done: false,
            };
        }
        let n_points = n_intervals + 1;
        let params = uniform_params(0.0, 1.0, n_points);
        UniformAbscissa { params, done: true }
    }

    /// Return the number of distributed points.
    ///
    /// Mirrors `GCPnts_UniformAbscissa::NbPoints()`.
    pub fn nb_points(&self) -> usize {
        self.params.len()
    }

    /// Return the parameter at index `i` (0-based).
    ///
    /// Mirrors `GCPnts_UniformAbscissa::Parameter(Index)`.
    ///
    /// # Panics
    /// Panics when `is_done()` is `false` or `i >= nb_points()`.
    pub fn parameter(&self, i: usize) -> f64 {
        assert!(self.done, "UniformAbscissa: is_done is false");
        assert!(
            i < self.params.len(),
            "UniformAbscissa: index {i} out of range (nb_points={})",
            self.params.len()
        );
        self.params[i]
    }

    /// Return whether the computation succeeded.
    ///
    /// Mirrors `GCPnts_UniformAbscissa::IsDone()`.
    pub fn is_done(&self) -> bool {
        self.done
    }
}

// ---------------------------------------------------------------------------
// curve_length
// ---------------------------------------------------------------------------

/// Compute the total arc length of a 3-D polyline.
///
/// Each consecutive pair of points in `pts` contributes a straight-line
/// segment; the total length is the sum of all segment lengths.
///
/// Returns `0.0` when `pts` has fewer than two points.
///
/// Uses only `f64::sqrt` from `std`; no external crates are required.
///
/// # Example
/// ```
/// # use ironstream::geom_curve_length::curve_length;
/// let pts = [[0.0_f64, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0, 0.0]];
/// assert!((curve_length(&pts) - 2.0).abs() < 1e-12);
/// ```
pub fn curve_length(pts: &[[f64; 3]]) -> f64 {
    if pts.len() < 2 {
        return 0.0;
    }
    pts.windows(2).fold(0.0, |acc, w| {
        let [ax, ay, az] = w[0];
        let [bx, by, bz] = w[1];
        let dx = bx - ax;
        let dy = by - ay;
        let dz = bz - az;
        acc + f64::sqrt(dx * dx + dy * dy + dz * dz)
    })
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

/// Return `n` evenly spaced parameter values in `[first, last]` (inclusive).
///
/// For `n == 1` returns `[first]`.
fn uniform_params(first: f64, last: f64, n: u32) -> Vec<f64> {
    debug_assert!(n >= 1);
    if n == 1 {
        return vec![first];
    }
    let denom = (n - 1) as f64;
    (0..n)
        .map(|i| first + (last - first) * (i as f64) / denom)
        .collect()
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn near(a: f64, b: f64, tol: f64, msg: &str) {
        assert!(
            (a - b).abs() <= tol,
            "{msg}: got {a}, expected {b} (tol {tol})"
        );
    }

    // -----------------------------------------------------------------------
    // AbscissaPoint
    // -----------------------------------------------------------------------

    #[test]
    fn abscissa_point_new_not_done() {
        let ap = AbscissaPoint::new();
        assert!(!ap.is_done());
    }

    #[test]
    fn abscissa_point_compute_basic() {
        let ap = AbscissaPoint::compute("line", 0.5, 0.0, 1e-7);
        assert!(ap.is_done());
        near(ap.parameter(), 0.5, 1e-12, "u0 + abscissa");
    }

    #[test]
    fn abscissa_point_compute_offset() {
        let ap = AbscissaPoint::compute("line", 0.3, 0.7, 1e-7);
        assert!(ap.is_done());
        near(ap.parameter(), 1.0, 1e-12, "u0 + abscissa = 1.0");
    }

    #[test]
    fn abscissa_point_negative_abscissa() {
        let ap = AbscissaPoint::compute("line", -0.2, 1.0, 1e-7);
        assert!(ap.is_done());
        near(ap.parameter(), 0.8, 1e-12, "negative abscissa");
    }

    #[test]
    fn abscissa_point_zero_abscissa() {
        let ap = AbscissaPoint::compute("curve", 0.0, 0.5, 1e-7);
        assert!(ap.is_done());
        near(ap.parameter(), 0.5, 1e-12, "zero abscissa stays at u0");
    }

    #[test]
    fn abscissa_point_negative_tol_not_done() {
        let ap = AbscissaPoint::compute("line", 0.5, 0.0, -1e-7);
        assert!(!ap.is_done());
    }

    #[test]
    fn abscissa_point_zero_tol_done() {
        let ap = AbscissaPoint::compute("line", 0.5, 0.0, 0.0);
        assert!(ap.is_done());
    }

    #[test]
    fn abscissa_point_param_field_matches_parameter() {
        let ap = AbscissaPoint::compute("c", 1.0, 2.0, 1e-6);
        assert_eq!(ap.param, ap.parameter());
    }

    // -----------------------------------------------------------------------
    // UniformAbscissa::new
    // -----------------------------------------------------------------------

    #[test]
    fn uniform_abscissa_new_zero_not_done() {
        let ua = UniformAbscissa::new("c", 0, 1e-7);
        assert!(!ua.is_done());
        assert_eq!(ua.nb_points(), 0);
    }

    #[test]
    fn uniform_abscissa_new_one_point() {
        let ua = UniformAbscissa::new("c", 1, 1e-7);
        assert!(ua.is_done());
        assert_eq!(ua.nb_points(), 1);
        near(ua.parameter(0), 0.0, 1e-12, "single point = 0.0");
    }

    #[test]
    fn uniform_abscissa_new_two_points() {
        let ua = UniformAbscissa::new("c", 2, 1e-7);
        assert!(ua.is_done());
        assert_eq!(ua.nb_points(), 2);
        near(ua.parameter(0), 0.0, 1e-12, "first = 0.0");
        near(ua.parameter(1), 1.0, 1e-12, "last = 1.0");
    }

    #[test]
    fn uniform_abscissa_new_five_points() {
        let ua = UniformAbscissa::new("c", 5, 1e-7);
        assert!(ua.is_done());
        assert_eq!(ua.nb_points(), 5);
        near(ua.parameter(0), 0.00, 1e-12, "param[0]");
        near(ua.parameter(1), 0.25, 1e-12, "param[1]");
        near(ua.parameter(2), 0.50, 1e-12, "param[2]");
        near(ua.parameter(3), 0.75, 1e-12, "param[3]");
        near(ua.parameter(4), 1.00, 1e-12, "param[4]");
    }

    #[test]
    fn uniform_abscissa_new_spacing_uniform() {
        let ua = UniformAbscissa::new("c", 6, 1e-7);
        assert!(ua.is_done());
        let step = ua.parameter(1) - ua.parameter(0);
        for i in 1..5 {
            let s = ua.parameter(i + 1) - ua.parameter(i);
            near(s, step, 1e-12, &format!("step {i}"));
        }
    }

    #[test]
    fn uniform_abscissa_new_done_field() {
        let ua = UniformAbscissa::new("c", 3, 1e-7);
        assert_eq!(ua.done, ua.is_done());
    }

    // -----------------------------------------------------------------------
    // UniformAbscissa::from_abscissa
    // -----------------------------------------------------------------------

    #[test]
    fn from_abscissa_half_spacing() {
        // abscissa = 0.5 => 2 intervals => 3 points at 0.0, 0.5, 1.0
        let ua = UniformAbscissa::from_abscissa("c", 0.5, 1e-7);
        assert!(ua.is_done());
        assert_eq!(ua.nb_points(), 3);
        near(ua.parameter(0), 0.0, 1e-12, "param[0]");
        near(ua.parameter(1), 0.5, 1e-12, "param[1]");
        near(ua.parameter(2), 1.0, 1e-12, "param[2]");
    }

    #[test]
    fn from_abscissa_quarter_spacing() {
        // abscissa = 0.25 => 4 intervals => 5 points
        let ua = UniformAbscissa::from_abscissa("c", 0.25, 1e-7);
        assert!(ua.is_done());
        assert_eq!(ua.nb_points(), 5);
        near(ua.parameter(0), 0.0, 1e-12, "first");
        near(ua.parameter(4), 1.0, 1e-12, "last");
    }

    #[test]
    fn from_abscissa_zero_not_done() {
        let ua = UniformAbscissa::from_abscissa("c", 0.0, 1e-7);
        assert!(!ua.is_done());
    }

    #[test]
    fn from_abscissa_negative_tol_not_done() {
        let ua = UniformAbscissa::from_abscissa("c", 0.5, -1e-7);
        assert!(!ua.is_done());
    }

    #[test]
    fn from_abscissa_large_abscissa_not_done() {
        // abscissa > 1.0 => floor(1/2.0) = 0 intervals => not-done
        let ua = UniformAbscissa::from_abscissa("c", 2.0, 1e-7);
        assert!(!ua.is_done());
    }

    #[test]
    fn from_abscissa_negative_abscissa_treated_as_abs() {
        let ua = UniformAbscissa::from_abscissa("c", -0.5, 1e-7);
        assert!(ua.is_done());
        assert_eq!(ua.nb_points(), 3);
    }

    #[test]
    fn from_abscissa_params_field_matches() {
        let ua = UniformAbscissa::from_abscissa("c", 0.5, 1e-7);
        assert_eq!(ua.params.len(), ua.nb_points());
    }

    // -----------------------------------------------------------------------
    // curve_length
    // -----------------------------------------------------------------------

    #[test]
    fn curve_length_empty() {
        assert_eq!(curve_length(&[]), 0.0);
    }

    #[test]
    fn curve_length_single_point() {
        assert_eq!(curve_length(&[[1.0, 2.0, 3.0]]), 0.0);
    }

    #[test]
    fn curve_length_unit_segment_x() {
        let pts = [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]];
        near(curve_length(&pts), 1.0, 1e-12, "unit x segment");
    }

    #[test]
    fn curve_length_unit_segment_y() {
        let pts = [[0.0, 0.0, 0.0], [0.0, 1.0, 0.0]];
        near(curve_length(&pts), 1.0, 1e-12, "unit y segment");
    }

    #[test]
    fn curve_length_unit_segment_z() {
        let pts = [[0.0, 0.0, 0.0], [0.0, 0.0, 1.0]];
        near(curve_length(&pts), 1.0, 1e-12, "unit z segment");
    }

    #[test]
    fn curve_length_diagonal_3d() {
        // (0,0,0) -> (1,1,1): distance = sqrt(3)
        let pts = [[0.0, 0.0, 0.0], [1.0, 1.0, 1.0]];
        near(curve_length(&pts), f64::sqrt(3.0), 1e-12, "3-D diagonal");
    }

    #[test]
    fn curve_length_l_shape() {
        // (0,0,0) -> (1,0,0) -> (1,1,0): total = 1 + 1 = 2
        let pts = [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0, 0.0]];
        near(curve_length(&pts), 2.0, 1e-12, "L-shape");
    }

    #[test]
    fn curve_length_closed_triangle() {
        // equilateral triangle with side 1
        use std::f64::consts::FRAC_PI_3;
        let side = 1.0_f64;
        let pts = [
            [0.0, 0.0, 0.0],
            [side, 0.0, 0.0],
            [side * FRAC_PI_3.cos(), side * FRAC_PI_3.sin(), 0.0],
            [0.0, 0.0, 0.0],
        ];
        near(curve_length(&pts), 3.0 * side, 1e-12, "equilateral triangle");
    }

    #[test]
    fn curve_length_two_unit_segments() {
        let pts = [
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [2.0, 0.0, 0.0],
        ];
        near(curve_length(&pts), 2.0, 1e-12, "two unit segments");
    }

    #[test]
    fn curve_length_coincident_points() {
        let pts = [[1.0, 2.0, 3.0], [1.0, 2.0, 3.0]];
        near(curve_length(&pts), 0.0, 1e-12, "coincident points");
    }

    // -----------------------------------------------------------------------
    // uniform_params helper (private, tested indirectly)
    // -----------------------------------------------------------------------

    #[test]
    fn uniform_params_direct() {
        let p = uniform_params(2.0, 5.0, 4);
        assert_eq!(p.len(), 4);
        near(p[0], 2.0, 1e-12, "first");
        near(p[3], 5.0, 1e-12, "last");
        // spacing should be 1.0
        for i in 0..3 {
            near(p[i + 1] - p[i], 1.0, 1e-12, &format!("step {i}"));
        }
    }

    #[test]
    fn uniform_params_single() {
        let p = uniform_params(3.0, 7.0, 1);
        assert_eq!(p, vec![3.0]);
    }
}
