// FILE: rust/ironstream/crates/ironstream/src/geom_approx.rs
//! `geom_approx` — lightweight B-spline approximation stubs mirroring
//! OpenCascade's `GeomConvert_ApproxCurve` and `GeomConvert_ApproxSurface`.
//!
//! Both types follow the OCCT two-phase pattern:
//! 1. Construct with `new()` (no work yet).
//! 2. Call `perform(…)` to run the approximation stub.
//!
//! No unsafe code; no third-party dependencies.

// ─────────────────────────── GeomApproxCurve ─────────────────────────────────

/// Approximates a curve as a B-spline curve.
///
/// This is a lightweight stub mirroring the interface of
/// `GeomConvert_ApproxCurve`.  The `perform` method generates `nb_poles`
/// equidistant poles on the unit segment [0, 1] × {0} × {0} and fills in the
/// corresponding uniform clamped knot vector and multiplicities.
// occt: GeomConvert_ApproxCurve
#[derive(Clone, Debug)]
pub struct GeomApproxCurve {
    /// Control poles of the resulting B-spline.
    pub poles: Vec<[f64; 3]>,
    /// Distinct knot values.
    pub knots: Vec<f64>,
    /// Knot multiplicities (same length as `knots`).
    pub mults: Vec<u32>,
    /// B-spline degree.
    pub degree: u32,
    /// Whether `perform` completed successfully.
    pub is_done: bool,
    /// Maximum approximation error (distance units).
    pub max_error: f64,
}

impl GeomApproxCurve {
    /// Construct an empty approximator.  Call `perform` to fill in the result.
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

    /// Run the stub approximation.
    ///
    /// Generates `nb_poles` equidistant poles on the unit segment
    /// `[0,1] × {0} × {0}` and builds the corresponding uniform clamped knot
    /// vector of degree `degree`.  Sets `max_error = tolerance * 0.1`.
    ///
    /// # Panics
    /// Panics if `nb_poles < degree as usize + 1` or `nb_poles == 0`.
    pub fn perform(&mut self, nb_poles: usize, degree: u32, tolerance: f64) {
        assert!(nb_poles > 0, "nb_poles must be > 0");
        let p = degree as usize;
        assert!(
            nb_poles >= p + 1,
            "nb_poles ({nb_poles}) must be >= degree + 1 ({})",
            p + 1
        );

        // Equidistant poles on [0,1] along X.
        self.poles = (0..nb_poles)
            .map(|i| {
                let t = if nb_poles == 1 {
                    0.0
                } else {
                    i as f64 / (nb_poles - 1) as f64
                };
                [t, 0.0, 0.0]
            })
            .collect();

        // Uniform clamped knot vector: multiplicity p+1 at ends, 1 interior.
        let n_interior = nb_poles.saturating_sub(p + 1);
        self.knots = Vec::new();
        self.mults = Vec::new();

        self.knots.push(0.0);
        self.mults.push((p + 1) as u32);

        for k in 1..=n_interior {
            self.knots.push(k as f64 / (n_interior + 1) as f64);
            self.mults.push(1);
        }

        self.knots.push(1.0);
        self.mults.push((p + 1) as u32);

        self.degree = degree;
        self.max_error = tolerance * 0.1;
        self.is_done = true;
    }

    /// Returns `true` if `perform` completed successfully.
    pub fn is_done(&self) -> bool {
        self.is_done
    }

    /// Number of control poles.
    pub fn nb_poles(&self) -> usize {
        self.poles.len()
    }

    /// B-spline degree.
    pub fn degree(&self) -> u32 {
        self.degree
    }

    /// Maximum approximation error.
    pub fn max_error(&self) -> f64 {
        self.max_error
    }

    /// Returns the `i`-th pole (0-indexed).
    ///
    /// # Panics
    /// Panics if `i >= nb_poles()`.
    pub fn pole(&self, i: usize) -> [f64; 3] {
        self.poles[i]
    }

    /// Returns the `i`-th distinct knot value (0-indexed).
    ///
    /// # Panics
    /// Panics if `i >= knots.len()`.
    pub fn knot(&self, i: usize) -> f64 {
        self.knots[i]
    }
}

impl Default for GeomApproxCurve {
    fn default() -> Self {
        Self::new()
    }
}

// ─────────────────────────── GeomApproxSurface ───────────────────────────────

/// Approximates a surface as a B-spline surface (stub).
///
/// Mirrors the interface of `GeomConvert_ApproxSurface`.  The `perform` method
/// records the requested grid dimensions and degrees without performing any
/// geometry work.
// occt: GeomConvert_ApproxSurface
#[derive(Clone, Debug)]
pub struct GeomApproxSurface {
    /// Number of control poles in the U direction.
    pub nb_u_poles: usize,
    /// Number of control poles in the V direction.
    pub nb_v_poles: usize,
    /// B-spline degree in the U direction.
    pub u_degree: u32,
    /// B-spline degree in the V direction.
    pub v_degree: u32,
    /// Whether `perform` completed successfully.
    pub is_done: bool,
    /// Maximum approximation error (distance units).
    pub max_error: f64,
}

impl GeomApproxSurface {
    /// Construct an empty approximator.  Call `perform` to fill in the result.
    pub fn new() -> Self {
        Self {
            nb_u_poles: 0,
            nb_v_poles: 0,
            u_degree: 0,
            v_degree: 0,
            is_done: false,
            max_error: 0.0,
        }
    }

    /// Run the stub approximation.
    ///
    /// Records `nb_u`, `nb_v`, `u_deg`, `v_deg` and sets
    /// `max_error = tolerance * 0.1`, `is_done = true`.
    pub fn perform(
        &mut self,
        nb_u: usize,
        nb_v: usize,
        u_deg: u32,
        v_deg: u32,
        tolerance: f64,
    ) {
        self.nb_u_poles = nb_u;
        self.nb_v_poles = nb_v;
        self.u_degree = u_deg;
        self.v_degree = v_deg;
        self.max_error = tolerance * 0.1;
        self.is_done = true;
    }

    /// Returns `true` if `perform` completed successfully.
    pub fn is_done(&self) -> bool {
        self.is_done
    }

    /// Number of control poles in the U direction.
    pub fn nb_u_poles(&self) -> usize {
        self.nb_u_poles
    }

    /// Number of control poles in the V direction.
    pub fn nb_v_poles(&self) -> usize {
        self.nb_v_poles
    }

    /// B-spline degree in the U direction.
    pub fn u_degree(&self) -> u32 {
        self.u_degree
    }

    /// B-spline degree in the V direction.
    pub fn v_degree(&self) -> u32 {
        self.v_degree
    }

    /// Maximum approximation error.
    pub fn max_error(&self) -> f64 {
        self.max_error
    }
}

impl Default for GeomApproxSurface {
    fn default() -> Self {
        Self::new()
    }
}

// ─────────────────────────── unit tests ──────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn curve_new_is_not_done() {
        let c = GeomApproxCurve::new();
        assert!(!c.is_done());
        assert_eq!(c.nb_poles(), 0);
    }

    #[test]
    fn curve_perform_basic() {
        let mut c = GeomApproxCurve::new();
        c.perform(5, 3, 1e-4);
        assert!(c.is_done());
        assert_eq!(c.nb_poles(), 5);
        assert_eq!(c.degree(), 3);
        assert!((c.max_error() - 1e-4 * 0.1).abs() < 1e-15);
    }

    #[test]
    fn curve_poles_equidistant() {
        let mut c = GeomApproxCurve::new();
        c.perform(4, 1, 0.01);
        // poles at x = 0, 1/3, 2/3, 1
        let expected = [0.0, 1.0 / 3.0, 2.0 / 3.0, 1.0];
        for (i, &ex) in expected.iter().enumerate() {
            assert!((c.pole(i)[0] - ex).abs() < 1e-12, "pole {i}: {}", c.pole(i)[0]);
            assert_eq!(c.pole(i)[1], 0.0);
            assert_eq!(c.pole(i)[2], 0.0);
        }
    }

    #[test]
    fn curve_knot_vector_clamped() {
        let mut c = GeomApproxCurve::new();
        c.perform(4, 3, 1e-6); // degree 3, 4 poles → no interior knots
        assert_eq!(c.knot(0), 0.0);
        assert_eq!(c.knot(c.knots.len() - 1), 1.0);
        assert_eq!(c.mults[0], 4);
        assert_eq!(*c.mults.last().unwrap(), 4);
    }

    #[test]
    fn curve_single_pole() {
        let mut c = GeomApproxCurve::new();
        c.perform(1, 0, 0.5);
        assert!(c.is_done());
        assert_eq!(c.nb_poles(), 1);
        assert_eq!(c.pole(0), [0.0, 0.0, 0.0]);
    }

    #[test]
    fn surface_new_is_not_done() {
        let s = GeomApproxSurface::new();
        assert!(!s.is_done());
        assert_eq!(s.nb_u_poles(), 0);
        assert_eq!(s.nb_v_poles(), 0);
    }

    #[test]
    fn surface_perform_basic() {
        let mut s = GeomApproxSurface::new();
        s.perform(6, 8, 3, 2, 1e-3);
        assert!(s.is_done());
        assert_eq!(s.nb_u_poles(), 6);
        assert_eq!(s.nb_v_poles(), 8);
        assert_eq!(s.u_degree(), 3);
        assert_eq!(s.v_degree(), 2);
        assert!((s.max_error() - 1e-3 * 0.1).abs() < 1e-15);
    }

    #[test]
    fn surface_default_equals_new() {
        let s: GeomApproxSurface = Default::default();
        assert!(!s.is_done());
    }
}
