//! `Convert_HyperbolicArcToBSplineCurve` — converts a hyperbolic arc to a
//! rational quadratic B-spline (NURBS), mirroring OpenCascade's
//! `Convert_HyperbolicArcToBSplineCurve` in package `TKMath`.
//!
//! # Algorithm
//!
//! A hyperbola arc is represented exactly as a rational quadratic B-spline by
//! subdividing it into segments of at most `π/2` in parameter span.  Each
//! segment maps to a single rational quadratic Bézier span with 3 poles.
//!
//! The hyperbola is parameterised by the real variable `t`:
//!
//! ```text
//! H(t) = Centre + a·cosh(t)·XDir + b·sinh(t)·YDir
//! ```
//!
//! where `a` is the semi-major (transverse) radius, `b` the semi-minor
//! (conjugate) radius, and `XDir`/`YDir` are the local frame axes.
//!
//! For each segment from `t0` to `t1`:
//!
//! ```text
//! P0      = H(t0)                                  (weight 1)
//! P_apex  = tangent-line intersection of H(t0) and H(t1)  (weight w_mid)
//! P2      = H(t1)                                  (weight 1)
//!
//! w_mid = cosh((t1 - t0) / 2)
//! ```
//!
//! The tangent-intersection apex is computed using the conic NURBS formula
//! (Piegl & Tiller, "The NURBS Book", §7.4):
//!
//! For `H(t) = C + a·cosh(t)·X + b·sinh(t)·Y`, the tangent at `t` has
//! direction `a·sinh(t)·X + b·cosh(t)·Y`.  Intersecting the two tangent lines:
//!
//! ```text
//! P_apex = C + a · (cosh(t0)*cosh(t1) - 1) / cosh((t0+t1)/2) / cosh((t1-t0)/2) · X'
//! ```
//!
//! A simpler closed-form (derived for the exact NURBS conic):
//! ```text
//! w_mid   = 1 / cosh(dt/2)       where dt = (t1-t0)/2
//! P_apex  = C + a·(ch0+ch1)/(ch0·ch1+1) · X + b·(sh0+sh1)/(ch0·ch1+1) · Y
//! ```
//! but using the identity `ch0*ch1 + sh0*sh1 = cosh(t1-t0)/... ` (hyperbolic
//! addition) requires more care.  The robust closed form used here is:
//!
//! ```text
//! tm   = (t0 + t1) / 2
//! dt   = (t1 - t0) / 2
//! w    = cosh(dt)
//! P1   = C + a·cosh(tm) / cosh(dt) · X + b·sinh(tm) / cosh(dt) · Y
//! ```
//!
//! This is the standard NURBS conic formula applied to the hyperbolic case.
//!
//! The resulting piecewise curve is assembled as a clamped B-spline of degree 2
//! with a Piecewise-Bézier knot structure.

// occt: Convert_ConicToBSplineCurve

use crate::gp::Pnt;
use crate::gp_hypr::Hypr;
use std::f64::consts::FRAC_PI_2;

/// Maximum parameter span per segment (matches OCCT's split strategy).
/// OCCT splits at every π/2 boundary in the parameter domain.
const MAX_SEG_SPAN: f64 = FRAC_PI_2;

/// `Convert_HyperbolicArcToBSplineCurve` — converts a hyperbola arc to a
/// rational quadratic B-spline curve.
///
/// # Usage
///
/// ```ignore
/// use ironstream::convert_hypr_to_bspline::HyprToBSpline;
/// use ironstream::gp::Ax3;
/// use ironstream::gp_hypr::Hypr;
///
/// let h = Hypr::new(Ax3::identity(), 3.0, 4.0);
/// let conv = HyprToBSpline::new(&h, -1.0, 1.0);
/// assert_eq!(conv.nb_poles(), 3);
/// ```
// occt: Convert_ConicToBSplineCurve
#[derive(Clone, Debug)]
pub struct HyprToBSpline {
    /// Control points (poles), stored in order.
    poles: Vec<Pnt>,
    /// Rational weights, one per pole.
    weights: Vec<f64>,
    /// Distinct knots (strictly increasing).
    knots: Vec<f64>,
    /// Knot multiplicities.
    mults: Vec<i32>,
}

impl HyprToBSpline {
    /// `Convert_HyperbolicArcToBSplineCurve(H, Alpha1, Alpha2)`
    ///
    /// Constructs a rational quadratic B-spline that exactly represents the
    /// hyperbola arc of `h` between parameters `alpha1` and `alpha2`.
    ///
    /// Parameters are the hyperbolic angle (`t` in `cosh(t)`, `sinh(t)`).
    /// Any finite range `alpha1 < alpha2` is accepted.
    ///
    /// # Panics
    ///
    /// Panics if `alpha2 <= alpha1`.
    pub fn new(h: &Hypr, alpha1: f64, alpha2: f64) -> Self {
        assert!(
            alpha2 > alpha1,
            "Convert_HyperbolicArcToBSplineCurve: alpha2 must be > alpha1"
        );

        let pos = h.position();
        let a = h.major_radius();
        let b = h.minor_radius();
        let centre = pos.location;
        let x_dir = pos.x_dir;
        let y_dir = pos.y_dir;

        let span = alpha2 - alpha1;

        // Determine the number of segments: split at every π/2 boundary.
        // n_segs = ceil(span / (π/2)), minimum 1.
        let n_segs = {
            let n = (span / MAX_SEG_SPAN).ceil() as usize;
            n.max(1)
        };

        let dtheta = span / n_segs as f64;

        // Pre-allocate.
        let n_poles = 2 * n_segs + 1;
        let mut poles = Vec::with_capacity(n_poles);
        let mut weights = Vec::with_capacity(n_poles);

        // Evaluate H(t) = centre + a·cosh(t)·x + b·sinh(t)·y
        let eval = |t: f64| -> Pnt {
            centre + x_dir * (a * t.cosh()) + y_dir * (b * t.sinh())
        };

        for seg in 0..n_segs {
            let t0 = alpha1 + seg as f64 * dtheta;
            let t1 = t0 + dtheta;

            // Start and end poles on the hyperbola.
            let p0 = eval(t0);
            let p2 = eval(t1);

            // The mid-parameter and half-span for the weight formula.
            let tm = 0.5 * (t0 + t1); // (t0+t1)/2
            let dt = 0.5 * (t1 - t0); // half-span

            // Weight for the inner (apex) control point:
            //   w_mid = cosh(dt)
            //
            // Derivation: for the rational Bézier to equal H(tm) at the midpoint
            // (Bézier parameter t=0.5 corresponding to NURBS parameter u=tm),
            // we require that B(0.5) = H(tm).  Substituting the apex formula
            // and solving gives w_mid = cosh(dt) > 1.  This differs from the
            // ellipse case (where w_mid = cos(dt) < 1) because the hyperbolic
            // cosine exceeds 1 for all dt > 0.
            let w_mid = dt.cosh();

            // Unweighted tangent-intersection apex:
            //   P1 = centre + a·cosh(tm)/cosh(dt)·X + b·sinh(tm)/cosh(dt)·Y
            // i.e. P1 = eval(tm) / cosh(dt)  (in the sense that the formula for
            // the tangent intersection gives exactly these coordinates)
            //
            // Derivation (standard NURBS conic, see Piegl & Tiller §7.4):
            //   The tangent at t0: direction  (a·sh0·X + b·ch0·Y)
            //   The tangent at t1: direction  (a·sh1·X + b·ch1·Y)
            //
            // Intersection of the two lines gives:
            //   Apex_x = a · (ch0·sh1 - sh0·ch1) / (sh1 - sh0)   ... but using
            //   hyperbolic addition formulae this collapses to a·cosh(tm)/cosh(dt).
            //   Apex_y = b · sinh(tm) / cosh(dt)
            let ch_dt = dt.cosh(); // = cosh((t1-t0)/2)
            let p1 = centre + x_dir * (a * tm.cosh() / ch_dt) + y_dir * (b * tm.sinh() / ch_dt);

            if seg == 0 {
                poles.push(p0);
                weights.push(1.0);
            }
            poles.push(p1);
            weights.push(w_mid);
            poles.push(p2);
            weights.push(1.0);
        }

        // Build knot vector for degree-2 piecewise-Bézier clamped B-spline.
        // Knot sequence: alpha1, alpha1+dtheta, ..., alpha2
        // Mults:         [3, 2, 2, ..., 2, 3]
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
            "Convert_HyperbolicArcToBSplineCurve::Pole: index out of range"
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
            "Convert_HyperbolicArcToBSplineCurve::Weight: index out of range"
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
            "Convert_HyperbolicArcToBSplineCurve::Knot: index out of range"
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
            "Convert_HyperbolicArcToBSplineCurve::Multiplicity: index out of range"
        );
        self.mults[(index - 1) as usize]
    }

    // ─────────────────────────────────── Evaluation ───────────────────────────

    /// Evaluate the NURBS curve at parameter `u` using rational de Boor's algorithm.
    ///
    /// `u` must lie in `[knot(1), knot(NbKnots)]`; values outside are clamped.
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

    // ─────────────────────────────────── Slice accessors ──────────────────────

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
    use crate::gp_hypr::Hypr;
    use crate::precision::{ANGULAR, CONFUSION};

    fn hypr_3_4() -> Hypr {
        Hypr::new(Ax3::identity(), 3.0, 4.0)
    }

    #[test]
    fn single_segment_short_arc() {
        // An arc spanning less than π/2 → 1 segment, 3 poles, 2 knots.
        let h = hypr_3_4();
        let conv = HyprToBSpline::new(&h, -0.5, 0.5);
        assert_eq!(conv.nb_poles(), 3);
        assert_eq!(conv.nb_knots(), 2);
    }

    #[test]
    fn two_segment_arc_has_5_poles() {
        // An arc spanning slightly over π/2 → 2 segments, 5 poles, 3 knots.
        let h = hypr_3_4();
        let conv = HyprToBSpline::new(&h, 0.0, std::f64::consts::FRAC_PI_2 + 0.1);
        assert_eq!(conv.nb_poles(), 5);
        assert_eq!(conv.nb_knots(), 3);
    }

    #[test]
    fn endpoints_lie_on_hyperbola() {
        let h = hypr_3_4();
        let t0 = -1.0_f64;
        let t1 = 1.0_f64;
        let conv = HyprToBSpline::new(&h, t0, t1);

        // Expected endpoints from parametric formula H(t) = a*cosh(t)*X + b*sinh(t)*Y
        let p_start = conv.value(t0);
        let p_end = conv.value(t1);

        let a = 3.0_f64;
        let b = 4.0_f64;
        let expected_start = Pnt::new(a * t0.cosh(), b * t0.sinh(), 0.0);
        let expected_end = Pnt::new(a * t1.cosh(), b * t1.sinh(), 0.0);

        assert!(
            p_start.is_equal(expected_start, CONFUSION * 100.0),
            "start: got {p_start:?}, expected {expected_start:?}"
        );
        assert!(
            p_end.is_equal(expected_end, CONFUSION * 100.0),
            "end: got {p_end:?}, expected {expected_end:?}"
        );
    }

    #[test]
    fn mid_parameter_lies_on_hyperbola() {
        // Evaluate the NURBS at the midpoint t_m and check it lies on H.
        let h = hypr_3_4();
        let t0 = -0.5_f64;
        let t1 = 0.5_f64;
        let tm = 0.0_f64; // midpoint
        let conv = HyprToBSpline::new(&h, t0, t1);

        let p = conv.value(tm);
        let a = 3.0_f64;
        let _b = 4.0_f64;
        // H(0) = (3*cosh(0), 4*sinh(0), 0) = (3, 0, 0)
        let expected = Pnt::new(a, 0.0, 0.0);
        assert!(
            p.is_equal(expected, CONFUSION * 100.0),
            "mid: got {p:?}, expected {expected:?}"
        );
    }

    #[test]
    fn weights_are_correct() {
        // For a single segment from -dt to dt (half-span = dt):
        // w_mid = cosh(dt)
        let h = hypr_3_4();
        let dt = 0.4_f64;
        let conv = HyprToBSpline::new(&h, -dt, dt);

        assert_eq!(conv.nb_poles(), 3);
        // Endpoint weights = 1.
        let tol = ANGULAR * 10.0;
        assert!((conv.weight(1) - 1.0).abs() < tol, "w1={}", conv.weight(1));
        assert!((conv.weight(3) - 1.0).abs() < tol, "w3={}", conv.weight(3));
        // Inner weight = cosh(half-span) = cosh(dt).
        let expected_w = dt.cosh();
        assert!(
            (conv.weight(2) - expected_w).abs() < tol,
            "w2={}, expected {}",
            conv.weight(2),
            expected_w
        );
    }

    #[test]
    fn sampled_points_lie_on_hyperbola() {
        // Sample at many parameters; check that each point satisfies the
        // hyperbola equation (x/a)² - (y/b)² = 1.
        let a = 3.0_f64;
        let b = 4.0_f64;
        let h = Hypr::new(Ax3::identity(), a, b);
        let t0 = -1.2_f64;
        let t1 = 1.2_f64;
        let conv = HyprToBSpline::new(&h, t0, t1);

        let n = 40;
        for i in 0..=n {
            let u = t0 + (t1 - t0) * i as f64 / n as f64;
            let p = conv.value(u);
            let err = (p.x / a) * (p.x / a) - (p.y / b) * (p.y / b) - 1.0;
            assert!(
                err.abs() < CONFUSION * 1000.0,
                "u={u:.4}: err={err:e}, p={p:?}"
            );
        }
    }

    #[test]
    fn knots_are_strictly_increasing() {
        let h = hypr_3_4();
        let conv = HyprToBSpline::new(&h, -2.0, 2.0);
        for i in 1..conv.nb_knots() {
            assert!(
                conv.knot(i + 1) > conv.knot(i),
                "knots not strictly increasing at i={}",
                i
            );
        }
    }

    #[test]
    fn mults_are_3_at_ends_and_2_in_middle() {
        // A two-segment arc (5 poles, 3 knots) → mults = [3, 2, 3].
        let h = hypr_3_4();
        let conv = HyprToBSpline::new(&h, 0.0, std::f64::consts::PI);
        assert_eq!(conv.nb_knots(), 3);
        assert_eq!(conv.multiplicity(1), 3, "first mult");
        assert_eq!(conv.multiplicity(2), 2, "middle mult");
        assert_eq!(conv.multiplicity(3), 3, "last mult");
    }

    #[test]
    fn pole_index_out_of_range_panics() {
        let h = hypr_3_4();
        let conv = HyprToBSpline::new(&h, -0.5, 0.5);
        let r = std::panic::catch_unwind(|| {
            let c = HyprToBSpline::new(&Hypr::new(Ax3::identity(), 3.0, 4.0), -0.5, 0.5);
            c.pole(0)
        });
        assert!(r.is_err(), "pole(0) should panic");
        let _ = conv; // suppress unused warning
    }

    #[test]
    fn weight_index_out_of_range_panics() {
        let h = hypr_3_4();
        let conv = HyprToBSpline::new(&h, -0.5, 0.5);
        let r = std::panic::catch_unwind(move || conv.weight(99));
        assert!(r.is_err(), "weight(99) should panic");
    }

    #[test]
    fn first_pole_matches_eval_at_alpha1() {
        let h = hypr_3_4();
        let t0 = 0.3_f64;
        let t1 = 0.9_f64;
        let conv = HyprToBSpline::new(&h, t0, t1);

        let a = 3.0_f64;
        let b = 4.0_f64;
        let expected = Pnt::new(a * t0.cosh(), b * t0.sinh(), 0.0);
        let got = conv.pole(1);
        assert!(
            got.is_equal(expected, CONFUSION * 10.0),
            "first pole {got:?} vs expected {expected:?}"
        );
    }

    #[test]
    fn nb_poles_grows_with_span() {
        let h = hypr_3_4();
        let pi = std::f64::consts::PI;

        // < π/2 → 1 segment → 3 poles
        let c1 = HyprToBSpline::new(&h, 0.0, pi / 4.0);
        assert_eq!(c1.nb_poles(), 3);

        // exactly π → 2 segments → 5 poles
        let c2 = HyprToBSpline::new(&h, 0.0, pi);
        assert_eq!(c2.nb_poles(), 5);

        // 2π → 4 segments → 9 poles
        let c4 = HyprToBSpline::new(&h, 0.0, 2.0 * pi);
        assert_eq!(c4.nb_poles(), 9);
    }
}
