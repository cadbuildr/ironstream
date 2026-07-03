// FILE: rust/ironstream/crates/ironstream/src/geom_gcpnts.rs
//! `GCPnts` — point distribution utilities along curves.
//!
//! Faithful port of OpenCascade's `GCPnts` package:
//! - `GcpntsAbscissaPoint`    — find the curve parameter at a given arc length
//!                              from a starting parameter.
//! - `GcpntsUniformAbscissa`  — distribute N points at equal arc-length
//!                              intervals along a parameter range.
//! - `GcpntsUniformDeflection`— distribute points by chordal deflection
//!                              (stub: evenly spaced in parameter).

// ---------------------------------------------------------------------------
// GcpntsAbscissaPoint
// ---------------------------------------------------------------------------

/// Finds the parameter value on a curve at a given arc length from a start
/// parameter.
///
/// The full OCCT implementation integrates the curve speed numerically
/// (Gauss–Legendre quadrature); this port uses linear interpolation as a
/// stub suitable for unit-speed parameterizations or testing purposes.
// occt: GCPnts_AbscissaPoint
#[derive(Clone, Debug)]
pub struct GcpntsAbscissaPoint {
    /// The found parameter, valid only when `is_done` is true.
    parameter: f64,
    /// Whether the computation succeeded.
    is_done: bool,
}

impl GcpntsAbscissaPoint {
    /// `GCPnts_AbscissaPoint()` — construct an empty, not-done instance.
    pub fn new() -> Self {
        GcpntsAbscissaPoint {
            parameter: 0.0,
            is_done: false,
        }
    }

    /// `GCPnts_AbscissaPoint::Perform(Abscissa, UFromStart, UEnd, Tol)` —
    /// find the parameter corresponding to arc length `length` measured from
    /// `u_start` along the curve whose total arc length over `[u_start, u_end]`
    /// is `curve_length`.
    ///
    /// This stub uses linear interpolation:
    /// `t = u_start + (length / curve_length) * (u_end - u_start)`.
    ///
    /// Sets `is_done = true` when the inputs are valid (curve_length > tolerance
    /// and the requested length is within `[0, curve_length]`).
    pub fn perform(
        &mut self,
        length: f64,
        curve_length: f64,
        u_start: f64,
        u_end: f64,
        tolerance: f64,
    ) {
        self.is_done = false;
        if curve_length.abs() <= tolerance {
            return;
        }
        if length < -tolerance || length > curve_length + tolerance {
            return;
        }
        let t = length / curve_length;
        self.parameter = u_start + t * (u_end - u_start);
        self.is_done = true;
    }

    /// `Standard_Boolean IsDone()` — true when `perform` succeeded.
    pub fn is_done(&self) -> bool {
        self.is_done
    }

    /// `Standard_Real Parameter()` — the found parameter.
    ///
    /// # Panics
    /// Panics if `is_done()` is false.
    pub fn parameter(&self) -> f64 {
        assert!(self.is_done, "GcpntsAbscissaPoint: IsDone is false");
        self.parameter
    }
}

impl Default for GcpntsAbscissaPoint {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// GcpntsUniformAbscissa
// ---------------------------------------------------------------------------

/// Distributes N points at equal arc-length intervals along a parameter range.
///
/// Mirrors OCCT's `GCPnts_UniformAbscissa`. The OCCT implementation integrates
/// the speed of the curve; this port computes evenly spaced parameters as a
/// stub appropriate for unit-speed or arc-length-parameterized curves and for
/// testing purposes.
// occt: GCPnts_UniformAbscissa
#[derive(Clone, Debug)]
pub struct GcpntsUniformAbscissa {
    /// The distributed parameter values (length `nb_points`).
    parameters: Vec<f64>,
    /// Requested and stored number of points.
    nb_points: u32,
    /// Whether the computation succeeded.
    is_done: bool,
}

impl GcpntsUniformAbscissa {
    /// `GCPnts_UniformAbscissa()` — construct an empty, not-done instance.
    pub fn new() -> Self {
        GcpntsUniformAbscissa {
            parameters: Vec::new(),
            nb_points: 0,
            is_done: false,
        }
    }

    /// `GCPnts_UniformAbscissa::Perform(NbPoints, U1, U2)` — distribute
    /// `nb_points` parameter values uniformly between `u_start` and `u_end`
    /// (inclusive at both ends).
    ///
    /// For `nb_points == 1` a single value at `u_start` is returned.
    /// Sets `is_done = true` when `nb_points >= 1`.
    pub fn perform(&mut self, nb_points: u32, u_start: f64, u_end: f64) {
        self.is_done = false;
        self.parameters.clear();
        self.nb_points = 0;

        if nb_points == 0 {
            return;
        }

        self.nb_points = nb_points;
        if nb_points == 1 {
            self.parameters.push(u_start);
        } else {
            let n = nb_points as f64 - 1.0;
            for i in 0..nb_points {
                let t = i as f64 / n;
                self.parameters.push(u_start + t * (u_end - u_start));
            }
        }
        self.is_done = true;
    }

    /// `Standard_Boolean IsDone()` — true when `perform` succeeded.
    pub fn is_done(&self) -> bool {
        self.is_done
    }

    /// `Standard_Integer NbPoints()` — number of distributed points.
    pub fn nb_points(&self) -> u32 {
        self.nb_points
    }

    /// `Standard_Real Parameter(Index)` — the parameter at index `i`
    /// (0-based).
    ///
    /// # Panics
    /// Panics if `is_done()` is false or `i >= nb_points()`.
    pub fn parameter(&self, i: usize) -> f64 {
        assert!(self.is_done, "GcpntsUniformAbscissa: IsDone is false");
        assert!(
            i < self.parameters.len(),
            "GcpntsUniformAbscissa: index {i} out of range (nb_points={})",
            self.nb_points
        );
        self.parameters[i]
    }
}

impl Default for GcpntsUniformAbscissa {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// GcpntsUniformDeflection
// ---------------------------------------------------------------------------

/// Distributes points along a curve by chordal deflection.
///
/// Mirrors OCCT's `GCPnts_UniformDeflection`. The full OCCT implementation
/// adaptively inserts points so that the maximum chordal error between the
/// curve and the polyline does not exceed `deflection`. This port is a stub:
/// it distributes `nb_approx` evenly spaced parameter values in `[u_start,
/// u_end]` regardless of the deflection value, suitable for testing and
/// integration.
// occt: GCPnts_UniformDeflection
#[derive(Clone, Debug)]
pub struct GcpntsUniformDeflection {
    /// The distributed parameter values.
    parameters: Vec<f64>,
    /// Whether the computation succeeded.
    is_done: bool,
}

impl GcpntsUniformDeflection {
    /// `GCPnts_UniformDeflection()` — construct an empty, not-done instance.
    pub fn new() -> Self {
        GcpntsUniformDeflection {
            parameters: Vec::new(),
            is_done: false,
        }
    }

    /// `GCPnts_UniformDeflection::Perform(Deflection, U1, U2, NbApprox)` —
    /// stub implementation: distribute `nb_approx` evenly spaced parameters
    /// in `[u_start, u_end]` (inclusive at both ends).
    ///
    /// The `deflection` argument is accepted for API compatibility but is not
    /// used by this stub.
    ///
    /// Sets `is_done = true` when `nb_approx >= 1`.
    pub fn perform(
        &mut self,
        _deflection: f64,
        u_start: f64,
        u_end: f64,
        nb_approx: u32,
    ) {
        self.is_done = false;
        self.parameters.clear();

        if nb_approx == 0 {
            return;
        }

        if nb_approx == 1 {
            self.parameters.push(u_start);
        } else {
            let n = nb_approx as f64 - 1.0;
            for i in 0..nb_approx {
                let t = i as f64 / n;
                self.parameters.push(u_start + t * (u_end - u_start));
            }
        }
        self.is_done = true;
    }

    /// `Standard_Boolean IsDone()` — true when `perform` succeeded.
    pub fn is_done(&self) -> bool {
        self.is_done
    }

    /// `Standard_Integer NbPoints()` — number of distributed points.
    pub fn nb_points(&self) -> usize {
        self.parameters.len()
    }

    /// `Standard_Real Parameter(Index)` — the parameter at index `i`
    /// (0-based).
    ///
    /// # Panics
    /// Panics if `is_done()` is false or `i >= nb_points()`.
    pub fn parameter(&self, i: usize) -> f64 {
        assert!(self.is_done, "GcpntsUniformDeflection: IsDone is false");
        assert!(
            i < self.parameters.len(),
            "GcpntsUniformDeflection: index {i} out of range (nb_points={})",
            self.parameters.len()
        );
        self.parameters[i]
    }
}

impl Default for GcpntsUniformDeflection {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Internal tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn near(a: f64, b: f64, tol: f64, msg: &str) {
        assert!(
            (a - b).abs() <= tol,
            "{msg}: expected {a} ≈ {b} (tol {tol})"
        );
    }

    // -----------------------------------------------------------------------
    // GcpntsAbscissaPoint
    // -----------------------------------------------------------------------

    #[test]
    fn abscissa_point_midpoint() {
        let mut ap = GcpntsAbscissaPoint::new();
        // curve_length = 10, want parameter at arc length 5 from u=0 to u=1
        ap.perform(5.0, 10.0, 0.0, 1.0, 1e-10);
        assert!(ap.is_done());
        near(ap.parameter(), 0.5, 1e-12, "midpoint parameter");
    }

    #[test]
    fn abscissa_point_start() {
        let mut ap = GcpntsAbscissaPoint::new();
        ap.perform(0.0, 10.0, 2.0, 5.0, 1e-10);
        assert!(ap.is_done());
        near(ap.parameter(), 2.0, 1e-12, "start parameter");
    }

    #[test]
    fn abscissa_point_end() {
        let mut ap = GcpntsAbscissaPoint::new();
        ap.perform(10.0, 10.0, 2.0, 5.0, 1e-10);
        assert!(ap.is_done());
        near(ap.parameter(), 5.0, 1e-12, "end parameter");
    }

    #[test]
    fn abscissa_point_not_done_zero_length() {
        let mut ap = GcpntsAbscissaPoint::new();
        ap.perform(1.0, 0.0, 0.0, 1.0, 1e-10);
        assert!(!ap.is_done(), "zero-length curve should fail");
    }

    #[test]
    fn abscissa_point_not_done_length_exceeds_curve() {
        let mut ap = GcpntsAbscissaPoint::new();
        ap.perform(11.0, 10.0, 0.0, 1.0, 1e-10);
        assert!(!ap.is_done(), "length > curve_length should fail");
    }

    #[test]
    fn abscissa_point_quarter() {
        let mut ap = GcpntsAbscissaPoint::new();
        ap.perform(2.5, 10.0, 0.0, 1.0, 1e-10);
        assert!(ap.is_done());
        near(ap.parameter(), 0.25, 1e-12, "quarter parameter");
    }

    // -----------------------------------------------------------------------
    // GcpntsUniformAbscissa
    // -----------------------------------------------------------------------

    #[test]
    fn uniform_abscissa_two_points() {
        let mut ua = GcpntsUniformAbscissa::new();
        ua.perform(2, 0.0, 1.0);
        assert!(ua.is_done());
        assert_eq!(ua.nb_points(), 2);
        near(ua.parameter(0), 0.0, 1e-12, "param[0]");
        near(ua.parameter(1), 1.0, 1e-12, "param[1]");
    }

    #[test]
    fn uniform_abscissa_five_points() {
        let mut ua = GcpntsUniformAbscissa::new();
        ua.perform(5, 0.0, 4.0);
        assert!(ua.is_done());
        assert_eq!(ua.nb_points(), 5);
        near(ua.parameter(0), 0.0, 1e-12, "param[0]");
        near(ua.parameter(1), 1.0, 1e-12, "param[1]");
        near(ua.parameter(2), 2.0, 1e-12, "param[2]");
        near(ua.parameter(3), 3.0, 1e-12, "param[3]");
        near(ua.parameter(4), 4.0, 1e-12, "param[4]");
    }

    #[test]
    fn uniform_abscissa_single_point() {
        let mut ua = GcpntsUniformAbscissa::new();
        ua.perform(1, 3.0, 7.0);
        assert!(ua.is_done());
        assert_eq!(ua.nb_points(), 1);
        near(ua.parameter(0), 3.0, 1e-12, "single point at u_start");
    }

    #[test]
    fn uniform_abscissa_zero_points_not_done() {
        let mut ua = GcpntsUniformAbscissa::new();
        ua.perform(0, 0.0, 1.0);
        assert!(!ua.is_done());
    }

    #[test]
    fn uniform_abscissa_endpoints_match() {
        let mut ua = GcpntsUniformAbscissa::new();
        ua.perform(10, 2.0, 8.0);
        assert!(ua.is_done());
        assert_eq!(ua.nb_points(), 10);
        near(ua.parameter(0), 2.0, 1e-12, "first == u_start");
        near(ua.parameter(9), 8.0, 1e-12, "last == u_end");
    }

    #[test]
    fn uniform_abscissa_spacing_is_uniform() {
        let mut ua = GcpntsUniformAbscissa::new();
        ua.perform(4, 0.0, 3.0);
        assert!(ua.is_done());
        let step = ua.parameter(1) - ua.parameter(0);
        for i in 1..3 {
            let s = ua.parameter(i + 1) - ua.parameter(i);
            near(s, step, 1e-12, &format!("step {i} uniform"));
        }
    }

    // -----------------------------------------------------------------------
    // GcpntsUniformDeflection
    // -----------------------------------------------------------------------

    #[test]
    fn uniform_deflection_three_points() {
        let mut ud = GcpntsUniformDeflection::new();
        ud.perform(0.1, 0.0, 1.0, 3);
        assert!(ud.is_done());
        assert_eq!(ud.nb_points(), 3);
        near(ud.parameter(0), 0.0, 1e-12, "param[0]");
        near(ud.parameter(1), 0.5, 1e-12, "param[1]");
        near(ud.parameter(2), 1.0, 1e-12, "param[2]");
    }

    #[test]
    fn uniform_deflection_single_point() {
        let mut ud = GcpntsUniformDeflection::new();
        ud.perform(0.01, 5.0, 10.0, 1);
        assert!(ud.is_done());
        assert_eq!(ud.nb_points(), 1);
        near(ud.parameter(0), 5.0, 1e-12, "single point at u_start");
    }

    #[test]
    fn uniform_deflection_zero_approx_not_done() {
        let mut ud = GcpntsUniformDeflection::new();
        ud.perform(0.1, 0.0, 1.0, 0);
        assert!(!ud.is_done());
    }

    #[test]
    fn uniform_deflection_endpoints_match() {
        let mut ud = GcpntsUniformDeflection::new();
        ud.perform(0.05, 1.0, 5.0, 7);
        assert!(ud.is_done());
        assert_eq!(ud.nb_points(), 7);
        near(ud.parameter(0), 1.0, 1e-12, "first == u_start");
        near(ud.parameter(6), 5.0, 1e-12, "last == u_end");
    }

    #[test]
    fn uniform_deflection_spacing_is_uniform() {
        let mut ud = GcpntsUniformDeflection::new();
        ud.perform(0.2, 0.0, 6.0, 4);
        assert!(ud.is_done());
        let step = ud.parameter(1) - ud.parameter(0);
        for i in 1..3 {
            let s = ud.parameter(i + 1) - ud.parameter(i);
            near(s, step, 1e-12, &format!("step {i} uniform"));
        }
    }

    #[test]
    fn uniform_deflection_deflection_arg_ignored() {
        // Two runs with different deflection values should produce identical
        // parameters (deflection is ignored in this stub).
        let mut ud1 = GcpntsUniformDeflection::new();
        ud1.perform(0.001, 0.0, 1.0, 5);
        let mut ud2 = GcpntsUniformDeflection::new();
        ud2.perform(999.0, 0.0, 1.0, 5);
        for i in 0..5 {
            near(
                ud1.parameter(i),
                ud2.parameter(i),
                1e-12,
                &format!("param[{i}] independent of deflection"),
            );
        }
    }
}
