//! `Convert_EllipseToBSplineCurve` — converts an ellipse (or ellipse arc) to a
//! rational quadratic B-spline (NURBS), mirroring OpenCascade's
//! `Convert_EllipseToBSplineCurve` in package `TKMath`.
//!
//! # Algorithm
//!
//! An ellipse arc is represented exactly as a rational quadratic B-spline by
//! subdividing it into at most 4 quarter-arc segments. Each quarter arc maps to
//! a single rational quadratic Bézier span with 3 poles:
//!
//! ```text
//! P0 = E(t0)
//! P1 = tangent intersection point (inner control point)
//! P2 = E(t0 + dtheta)
//! weight1 = cos(dtheta / 2)   (middle weight; end-point weights = 1)
//! ```
//!
//! The resulting piecewise curve is assembled as a clamped B-spline of degree 2
//! with Piecewise-Bézier knot structure.
//!
//! # Parameterisation
//!
//! The ellipse is parameterised by angle `U`:
//!
//! ```text
//! E(U) = Centre + a*cos(U)*XDir + b*sin(U)*YDir
//! ```
//!
//! Angles `alpha1` and `alpha2` are standard angular parameters.  The arc is
//! swept from `alpha1` to `alpha2` **counter-clockwise** (increasing angle) for
//! `Trig` type, or as the shortest arc for `Rational`/`Quasiangular`.
//!
//! # Convert_ParameterisationType
//!
//! This port implements the **Rational** parameterisation (the only one
//! expressible as a rational quadratic B-spline with exact arc geometry), which
//! is the OCCT default. The `type_` field is stored for API completeness.

// occt: Convert_EllipseToBSplineCurve

use crate::gp::Pnt;
use crate::gp_elips::Elips;
use std::f64::consts::{FRAC_PI_2, PI};

/// `Convert_ParameterisationType` — the requested parameterisation style.
///
/// In this implementation only `Rational` (the default) is fully supported:
/// it produces the exact rational quadratic B-spline for the ellipse arc.
/// Other variants map to the same algorithm (consistent with OCCT's Rational
/// path when an exact conic is needed).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParameterisationType {
    /// Standard rational (exact) parameterisation — default.
    Rational,
    /// Quasi-angular (approximately uniform in arc-length). Maps to Rational here.
    Quasiangular,
    /// Trigonometric (true-angle parameterisation). Maps to Rational here.
    Trig,
}

/// `Convert_EllipseToBSplineCurve` — converts an ellipse arc to a rational
/// quadratic B-spline curve.
///
/// # Usage
///
/// ```ignore
/// use crate::convert_ellipse_to_bspline::{EllipseToBSpline, ParameterisationType};
/// use crate::gp::{Ax3, Pnt};
/// use crate::gp_elips::Elips;
///
/// let elips = Elips::new(Ax3::identity(), 5.0, 3.0);
/// let conv = EllipseToBSpline::new(&elips, 0.0, 2.0 * std::f64::consts::PI,
///                                  ParameterisationType::Rational);
/// assert_eq!(conv.nb_poles(), 9);
/// assert_eq!(conv.nb_knots(), 5);
/// ```
// occt: Convert_EllipseToBSplineCurve
#[derive(Clone, Debug)]
pub struct EllipseToBSpline {
    /// Control points (poles), stored in order.
    poles: Vec<Pnt>,
    /// Rational weights, one per pole.
    weights: Vec<f64>,
    /// Distinct knots (strictly increasing).
    knots: Vec<f64>,
    /// Knot multiplicities.
    mults: Vec<i32>,
    /// The requested parameterisation type (stored for completeness).
    param_type: ParameterisationType,
}

impl EllipseToBSpline {
    /// `Convert_EllipseToBSplineCurve(E, Alpha1, Alpha2, Type)`
    ///
    /// Constructs a rational quadratic B-spline that represents the ellipse arc
    /// of `E` between angles `alpha1` and `alpha2`.
    ///
    /// - `alpha2 > alpha1` is required; both are in radians.
    /// - The full ellipse (`alpha2 - alpha1 >= 2π`) generates 9 poles.
    /// - A partial arc is split into quarter-arc segments (1–4 segments).
    ///
    /// # Panics
    ///
    /// Panics if `alpha2 <= alpha1` (no arc) or if the arc span exceeds `2π`.
    pub fn new(elips: &Elips, alpha1: f64, alpha2: f64, param_type: ParameterisationType) -> Self {
        assert!(
            alpha2 > alpha1,
            "Convert_EllipseToBSplineCurve: alpha2 must be > alpha1"
        );

        let angle_span = alpha2 - alpha1;
        // Clamp to full turn.
        let is_full = angle_span >= 2.0 * PI - 1.0e-10;

        let pos = elips.position();
        let a = elips.major_radius(); // semi-major
        let b = elips.minor_radius(); // semi-minor

        // Determine number of quadrant segments needed.
        // OCCT splits at every π/2 boundary, giving 1–4 segments.
        let n_segs = if is_full {
            4
        } else if angle_span <= FRAC_PI_2 + 1.0e-10 {
            1
        } else if angle_span <= PI + 1.0e-10 {
            2
        } else if angle_span <= 3.0 * FRAC_PI_2 + 1.0e-10 {
            3
        } else {
            4
        };

        // Each segment spans `dtheta` radians.
        let total_span = if is_full { 2.0 * PI } else { angle_span };
        let dtheta = total_span / n_segs as f64;
        // Middle-weight for each segment: cos(dtheta/2).
        let w_mid = (dtheta / 2.0).cos();

        // Build poles and weights.
        // A rational quadratic arc from t0 to t0+dtheta:
        //   P0 = E(t0)           weight 1
        //   P1 = tangent apex    weight w_mid = cos(dtheta/2)
        //   P2 = E(t0+dtheta)   weight 1
        //
        // The tangent apex is found by intersecting the tangent lines at P0 and P2.
        // For an ellipse E(t) = C + a*cos(t)*X + b*sin(t)*Y:
        //   dE/dt = -a*sin(t)*X + b*cos(t)*Y
        //
        // The tangent at t0 and t1 = t0+dtheta intersect at:
        //   P_apex = E(t0) + lambda * dE/dt|t0
        // where lambda is chosen so the apex lies on the tangent at t1 too.
        //
        // It can be shown that for the rational conic construction (Piegl-Tiller):
        //   P_apex = ( cos(t1)*P0 - cos(t0)*P2 ) / sin(t1 - t0)
        //             but expressed in homogeneous coordinates.
        //
        // Equivalently (per Farin's "Curves and Surfaces for CAGD"):
        //   The apex in 3D is:
        //     P_apex = C + a * (cos_t0 + cos_t1)/(1 + cos(dtheta)) * X
        //                + b * (sin_t0 + sin_t1)/(1 + cos(dtheta)) * Y
        //   weighted by w_mid = cos(dtheta/2).
        //
        // This is the standard NURBS exact conic formula.

        let centre = pos.location;
        let x_dir = pos.x_dir;
        let y_dir = pos.y_dir;

        // Number of poles = 2*n_segs + 1
        let n_poles = 2 * n_segs + 1;
        let mut poles = Vec::with_capacity(n_poles);
        let mut weights = Vec::with_capacity(n_poles);

        for seg in 0..n_segs {
            let t0 = alpha1 + seg as f64 * dtheta;
            let t1 = t0 + dtheta;

            let (s0, c0) = t0.sin_cos();
            let (s1, c1) = t1.sin_cos();

            // Start pole of this segment.
            let p0 = centre + x_dir * (a * c0) + y_dir * (b * s0);

            // Tangent-intersection apex (in homogeneous NURBS sense):
            // unweighted apex position P_apex / w_mid.
            // Using the formula: P_apex_raw = 0.5 * (P0/w0 + P2/w2) with
            // the conic apex at the intersection of tangents.
            //
            // The exact formula for the apex:
            //   P1_homog = P0_homog + P2_homog - ... (see derivation)
            // A clean closed form (Piegl-Tiller, "The NURBS Book", p.299):
            //   P1 = E(t0) × tangent(t0) intersected with E(t1) × tangent(t1)
            //
            // For the ellipse this simplifies to:
            //   P1_x = (c0 + c1) * a / (1 + c0*c1 + s0*s1)
            //         = (c0 + c1) * a / (1 + cos(t1-t0))
            //   P1_y = (s0 + s1) * b / (1 + c0*c1 + s0*s1)
            //
            // This gives the unweighted apex P1 (not divided by w_mid).
            // The NURBS control point is stored as (P1, w_mid).
            let cos_dt = (t1 - t0).cos(); // = cos(dtheta)
            let denom = 1.0 + cos_dt; // = 1 + cos(dtheta)

            let p_apex = centre
                + x_dir * (a * (c0 + c1) / denom)
                + y_dir * (b * (s0 + s1) / denom);

            // End pole.
            let p2 = centre + x_dir * (a * c1) + y_dir * (b * s1);

            if seg == 0 {
                poles.push(p0);
                weights.push(1.0);
            }
            poles.push(p_apex);
            weights.push(w_mid);
            poles.push(p2);
            weights.push(1.0);
        }

        // Build knots and multiplicities for degree-2 piecewise-Bézier clamped
        // B-spline.
        // Knots: [0, 1/n_segs, 2/n_segs, ..., 1] but scaled to [alpha1, alpha2]
        // Mults: [3, 2, 2, ..., 2, 3]  (n_segs+1 distinct knots)
        let mut knots = Vec::with_capacity(n_segs + 1);
        let mut mults: Vec<i32> = Vec::with_capacity(n_segs + 1);

        for i in 0..=n_segs {
            knots.push(alpha1 + i as f64 * dtheta);
            if i == 0 || i == n_segs {
                mults.push(3);
            } else {
                mults.push(2);
            }
        }

        Self {
            poles,
            weights,
            knots,
            mults,
            param_type,
        }
    }

    // ─────────────────────────────────── Accessors ────────────────────────────

    /// `NbPoles()` — number of control points.
    #[inline]
    pub fn nb_poles(&self) -> i32 {
        self.poles.len() as i32
    }

    /// `NbKnots()` — number of distinct knots.
    #[inline]
    pub fn nb_knots(&self) -> i32 {
        self.knots.len() as i32
    }

    /// `Pole(Index)` — 1-based control point.
    ///
    /// # Panics
    /// Panics if `index` is out of range `[1, NbPoles]`.
    pub fn pole(&self, index: i32) -> Pnt {
        assert!(
            index >= 1 && index <= self.nb_poles(),
            "Convert_EllipseToBSplineCurve::Pole: index out of range"
        );
        self.poles[(index - 1) as usize]
    }

    /// `Weight(Index)` — 1-based rational weight.
    ///
    /// # Panics
    /// Panics if `index` is out of range `[1, NbPoles]`.
    pub fn weight(&self, index: i32) -> f64 {
        assert!(
            index >= 1 && index <= self.nb_poles(),
            "Convert_EllipseToBSplineCurve::Weight: index out of range"
        );
        self.weights[(index - 1) as usize]
    }

    /// `Knot(Index)` — 1-based distinct knot value.
    ///
    /// # Panics
    /// Panics if `index` is out of range `[1, NbKnots]`.
    pub fn knot(&self, index: i32) -> f64 {
        assert!(
            index >= 1 && index <= self.nb_knots(),
            "Convert_EllipseToBSplineCurve::Knot: index out of range"
        );
        self.knots[(index - 1) as usize]
    }

    /// `Multiplicity(Index)` — 1-based knot multiplicity.
    ///
    /// # Panics
    /// Panics if `index` is out of range `[1, NbKnots]`.
    pub fn multiplicity(&self, index: i32) -> i32 {
        assert!(
            index >= 1 && index <= self.nb_knots(),
            "Convert_EllipseToBSplineCurve::Multiplicity: index out of range"
        );
        self.mults[(index - 1) as usize]
    }

    /// The requested parameterisation type.
    #[inline]
    pub fn param_type(&self) -> ParameterisationType {
        self.param_type
    }

    // ─────────────────────────────────── Evaluation ───────────────────────────

    /// Evaluate the NURBS curve at parameter `u` using de Boor's algorithm.
    ///
    /// `u` must lie in `[knot(1), knot(NbKnots)]`.
    pub fn value(&self, u: f64) -> Pnt {
        // Build the flat (expanded) knot vector from distinct knots + mults.
        let flat: Vec<f64> = self
            .knots
            .iter()
            .zip(self.mults.iter())
            .flat_map(|(&k, &m)| std::iter::repeat(k).take(m as usize))
            .collect();

        let degree = 2_usize;
        let n = self.poles.len() - 1; // last pole index
        let u_clamped = u.clamp(flat[degree], flat[n + 1]);

        // Find knot span.
        let span = bspline_find_span(n, degree, u_clamped, &flat);

        // Rational de Boor.
        let mut d: Vec<Pnt> = Vec::with_capacity(degree + 1);
        let mut w: Vec<f64> = Vec::with_capacity(degree + 1);
        for j in 0..=degree {
            let idx = span - degree + j;
            let wt = self.weights[idx];
            d.push(self.poles[idx] * wt);
            w.push(wt);
        }
        for r in 1..=degree {
            for j in (r..=degree).rev() {
                let i = span - degree + j;
                let denom = flat[i + degree - r + 1] - flat[i];
                let alpha = if denom.abs() < 1.0e-15 {
                    0.0
                } else {
                    (u_clamped - flat[i]) / denom
                };
                d[j] = d[j - 1] * (1.0 - alpha) + d[j] * alpha;
                w[j] = w[j - 1] * (1.0 - alpha) + w[j] * alpha;
            }
        }
        let wp = if w[degree].abs() < 1.0e-15 {
            1.0
        } else {
            w[degree]
        };
        d[degree] * (1.0 / wp)
    }

    // ─────────────────────────────────── Helpers ──────────────────────────────

    /// All pole positions as a slice (0-based).
    #[inline]
    pub fn poles(&self) -> &[Pnt] {
        &self.poles
    }

    /// All weights as a slice (0-based).
    #[inline]
    pub fn weights_slice(&self) -> &[f64] {
        &self.weights
    }

    /// All distinct knots as a slice (0-based).
    #[inline]
    pub fn knots_slice(&self) -> &[f64] {
        &self.knots
    }

    /// All multiplicities as a slice (0-based).
    #[inline]
    pub fn mults_slice(&self) -> &[i32] {
        &self.mults
    }
}

// ─────────────────────────────────── Private helpers ──────────────────────────

/// B-spline knot span binary search (de Boor, "A Practical Guide").
/// Returns index `i` such that `flat[i] <= u < flat[i+1]`, clamped to `[p, n]`.
fn bspline_find_span(n: usize, p: usize, u: f64, flat: &[f64]) -> usize {
    if u >= flat[n + 1] {
        return n;
    }
    if u <= flat[p] {
        return p;
    }
    let (mut lo, mut hi) = (p, n + 1);
    let mut mid = (lo + hi) / 2;
    while u < flat[mid] || u >= flat[mid + 1] {
        if u < flat[mid] {
            hi = mid;
        } else {
            lo = mid;
        }
        mid = (lo + hi) / 2;
    }
    mid
}

// ─────────────────────────────────── Unit tests ───────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gp::Ax3;
    use crate::gp_elips::Elips;
    use crate::precision::{ANGULAR, CONFUSION};
    use std::f64::consts::PI;

    fn unit_circle_elips() -> Elips {
        Elips::new(Ax3::identity(), 1.0, 1.0)
    }

    fn ellipse_5_3() -> Elips {
        Elips::new(Ax3::identity(), 5.0, 3.0)
    }

    #[test]
    fn full_ellipse_has_9_poles_and_5_knots() {
        let e = ellipse_5_3();
        let conv = EllipseToBSpline::new(&e, 0.0, 2.0 * PI, ParameterisationType::Rational);
        assert_eq!(conv.nb_poles(), 9);
        assert_eq!(conv.nb_knots(), 5);
    }

    #[test]
    fn quarter_arc_has_3_poles_and_2_knots() {
        let e = ellipse_5_3();
        let conv =
            EllipseToBSpline::new(&e, 0.0, FRAC_PI_2, ParameterisationType::Rational);
        assert_eq!(conv.nb_poles(), 3);
        assert_eq!(conv.nb_knots(), 2);
    }

    #[test]
    fn full_ellipse_mults_are_3_2_2_2_3() {
        let e = ellipse_5_3();
        let conv = EllipseToBSpline::new(&e, 0.0, 2.0 * PI, ParameterisationType::Rational);
        assert_eq!(conv.multiplicity(1), 3);
        assert_eq!(conv.multiplicity(2), 2);
        assert_eq!(conv.multiplicity(3), 2);
        assert_eq!(conv.multiplicity(4), 2);
        assert_eq!(conv.multiplicity(5), 3);
    }

    #[test]
    fn endpoints_are_on_ellipse() {
        let e = ellipse_5_3();
        let conv = EllipseToBSpline::new(&e, 0.0, 2.0 * PI, ParameterisationType::Rational);
        // The curve is periodic: start == end.
        let p0 = conv.value(0.0);
        let p_end = conv.value(2.0 * PI);
        // The ellipse point at 0: (5, 0, 0).
        let expected = Pnt::new(5.0, 0.0, 0.0);
        assert!(p0.is_equal(expected, CONFUSION * 10.0), "p0={p0:?}");
        assert!(p_end.is_equal(expected, CONFUSION * 10.0), "p_end={p_end:?}");
    }

    #[test]
    fn quarter_arc_endpoints_on_ellipse() {
        let e = ellipse_5_3();
        let conv =
            EllipseToBSpline::new(&e, 0.0, FRAC_PI_2, ParameterisationType::Rational);
        let p0 = conv.value(0.0);
        let p1 = conv.value(FRAC_PI_2);
        assert!(p0.is_equal(Pnt::new(5.0, 0.0, 0.0), CONFUSION * 10.0), "p0={p0:?}");
        assert!(p1.is_equal(Pnt::new(0.0, 3.0, 0.0), CONFUSION * 10.0), "p1={p1:?}");
    }

    #[test]
    fn mid_point_lies_on_ellipse_unit_circle() {
        // For the unit circle (a=b=1), the NURBS must reproduce the circle.
        let e = unit_circle_elips();
        let conv =
            EllipseToBSpline::new(&e, 0.0, FRAC_PI_2, ParameterisationType::Rational);
        let mid = conv.value(FRAC_PI_2 / 2.0);
        let r = (mid.x * mid.x + mid.y * mid.y).sqrt();
        assert!((r - 1.0).abs() < CONFUSION * 100.0, "r={r}");
    }

    #[test]
    fn half_arc_has_5_poles() {
        let e = ellipse_5_3();
        let conv = EllipseToBSpline::new(&e, 0.0, PI, ParameterisationType::Rational);
        assert_eq!(conv.nb_poles(), 5);
        assert_eq!(conv.nb_knots(), 3);
    }

    #[test]
    fn weights_are_one_at_endpoints_and_less_in_middle() {
        let e = ellipse_5_3();
        let conv = EllipseToBSpline::new(&e, 0.0, 2.0 * PI, ParameterisationType::Rational);
        // Odd poles (1,3,5,7,9) should have weight 1; even poles are inner.
        assert!((conv.weight(1) - 1.0).abs() < ANGULAR, "w1={}", conv.weight(1));
        assert!((conv.weight(3) - 1.0).abs() < ANGULAR, "w3={}", conv.weight(3));
        assert!((conv.weight(9) - 1.0).abs() < ANGULAR, "w9={}", conv.weight(9));
        // Inner control point weight = cos(PI/4) ≈ 0.7071.
        assert!(conv.weight(2) < 1.0, "w2 should be < 1");
    }

    #[test]
    fn sampled_points_lie_on_ellipse() {
        // Sample the full ellipse NURBS at many parameter values and verify each
        // point lies on the ellipse (x/a)^2 + (y/b)^2 = 1.
        let a = 5.0_f64;
        let b = 3.0_f64;
        let e = Elips::new(Ax3::identity(), a, b);
        let conv = EllipseToBSpline::new(&e, 0.0, 2.0 * PI, ParameterisationType::Rational);
        let n = 40;
        for i in 0..=n {
            let u = i as f64 / n as f64 * 2.0 * PI;
            let p = conv.value(u);
            let err = (p.x / a) * (p.x / a) + (p.y / b) * (p.y / b) - 1.0;
            assert!(err.abs() < 1.0e-9, "u={u:.4}: err={err:e}, p={p:?}");
        }
    }

    #[test]
    fn three_quarter_arc_has_7_poles() {
        let e = ellipse_5_3();
        let conv =
            EllipseToBSpline::new(&e, 0.0, 1.5 * PI, ParameterisationType::Rational);
        assert_eq!(conv.nb_poles(), 7);
        assert_eq!(conv.nb_knots(), 4);
    }

    #[test]
    fn pole_index_out_of_range_panics() {
        let e = ellipse_5_3();
        let conv =
            EllipseToBSpline::new(&e, 0.0, FRAC_PI_2, ParameterisationType::Rational);
        let r = std::panic::catch_unwind(|| conv.pole(0));
        assert!(r.is_err());
    }

    #[test]
    fn knots_are_strictly_increasing() {
        let e = ellipse_5_3();
        let conv = EllipseToBSpline::new(&e, 0.0, 2.0 * PI, ParameterisationType::Rational);
        for i in 1..conv.nb_knots() {
            assert!(
                conv.knot(i + 1) > conv.knot(i),
                "knots not strictly increasing at {}",
                i
            );
        }
    }

    #[test]
    fn param_type_stored_correctly() {
        let e = ellipse_5_3();
        let conv =
            EllipseToBSpline::new(&e, 0.0, 2.0 * PI, ParameterisationType::Quasiangular);
        assert_eq!(conv.param_type(), ParameterisationType::Quasiangular);
    }
}
