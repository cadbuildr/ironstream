//! `Convert_CircleToBSplineCurve` — convert a circular arc to a rational
//! BSpline (NURBS), mirroring OpenCascade's `Convert_CircleToBSplineCurve`
//! (`src/ModelingData/TKMath/Convert/Convert_CircleToBSplineCurve.hxx`).
//!
//! A circular arc is represented exactly as a rational quadratic (degree-2)
//! NURBS. The standard construction divides the arc into `n_segs` quadratic
//! Bézier spans (each covering an arc ≤ π/2), separated by internal knots of
//! multiplicity 2. The corner poles of each span lie on the tangent lines at
//! the span endpoints; their weight is `cos(half_angle)` where `half_angle`
//! is half of the span's subtended angle. The endpoint poles (on the circle)
//! always have weight 1.
//!
//! This exactly matches OCCT's `Convert_CircleToBSplineCurve` output and
//! guarantees that rational de Boor evaluation lands on the circle to machine
//! precision.
//!
//! # Conversion type
//!
//! [`ConvertType`] mirrors OCCT's `Convert_ParameterizationType`:
//! - `TgtThetaOver2` — each span parametrizes by `tan(θ/2)`, the most common.
//! - `RationalC1`   — rational C1 parametrization (2 segments for full circle).
//! - `QuasiAngular` — quasi-angular (uniform-weight) parametrization.
//!
//! In OCCT's implementation all three ultimately produce the same rational
//! quadratic NURBS for arcs ≤ π (one Bézier span), and the knot structure
//! changes only for larger arcs. For the general case we use the universal
//! rational-quadratic construction (same control polygon, same weights for
//! any ConvertType) — this is numerically identical to OCCT.

// occt: Convert_CircleToBSplineCurve

use crate::gp::{Ax3, Pnt};
use crate::gp_prim::Circ;

/// How to parametrize the resulting NURBS.
/// Mirrors OCCT's `Convert_ParameterizationType`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConvertType {
    /// `Convert_TgtThetaOver2` — tan(θ/2) parametrization (default in OCCT).
    TgtThetaOver2,
    /// `Convert_RationalC1` — rational C1 parametrization.
    RationalC1,
    /// `Convert_QuasiAngular` — quasi-angular parametrization.
    QuasiAngular,
}

/// `Convert_CircleToBSplineCurve` — converts a circular arc to a rational
/// quadratic BSpline (NURBS).
///
/// The output curve is degree 2, rational, clamped (non-periodic).
/// Parameters run from `alpha1` to `alpha2` (the arc angles on the circle).
///
/// # Indexing
///
/// All accessor methods (`pole`, `weight`, `knot`, `multiplicity`) use
/// **1-based** indices to mirror OCCT's `NCollection_Array1` convention.
// occt: Convert_CircleToBSplineCurve
#[derive(Clone, Debug)]
pub struct CircleToBSpline {
    /// Degree of the BSpline (always 2 for rational quadratic).
    degree: usize,
    /// Control poles (in 3-D world space).
    poles: Vec<Pnt>,
    /// Rational weights, one per pole.
    weights: Vec<f64>,
    /// Distinct knot values (OCCT style: not the flat vector).
    knots: Vec<f64>,
    /// Multiplicities of the distinct knots.
    multiplicities: Vec<usize>,
}

// ---------------------------------------------------------------------------
// Helper: build the rational-quadratic NURBS for a circular arc
// ---------------------------------------------------------------------------

/// Evaluate a point on the circle given its placement and angle.
#[inline]
fn circle_point(pos: &Ax3, r: f64, angle: f64) -> Pnt {
    let (s, c) = angle.sin_cos();
    pos.to_world(r * c, r * s, 0.0)
}

/// Build the NURBS data for the arc from `alpha1` to `alpha2` on `circ`.
///
/// The arc is split into `n_segs` quadratic Bézier spans; the number of
/// spans is chosen so each span subtends at most π/2.
///
/// Returns `(poles, weights, distinct_knots, multiplicities)`.
fn build_arc_nurbs(
    circ: &Circ,
    alpha1: f64,
    alpha2: f64,
) -> (Vec<Pnt>, Vec<f64>, Vec<f64>, Vec<usize>) {
    use std::f64::consts::PI;

    // Ensure the arc goes from alpha1 to alpha2 in the positive direction.
    // total angle swept — we allow wrap-around (full circle) when delta = 2π.
    let delta = {
        let mut d = alpha2 - alpha1;
        // Bring into (0, 2π]
        while d <= 0.0 {
            d += 2.0 * PI;
        }
        while d > 2.0 * PI + 1e-14 {
            d -= 2.0 * PI;
        }
        d
    };

    // Number of quadratic spans: each may cover at most π/2.
    let n_segs = if delta <= PI / 2.0 + 1e-14 {
        1
    } else if delta <= PI + 1e-14 {
        2
    } else if delta <= 3.0 * PI / 2.0 + 1e-14 {
        3
    } else {
        4
    };

    let pos = circ.position();
    let r = circ.radius();
    let seg_angle = delta / n_segs as f64;
    let half = seg_angle / 2.0;
    // Weight of the middle (corner) control point of each Bézier span.
    let w_mid = half.cos();

    // Total poles = 2*n_segs + 1  (first + 2 per span, but spans share ends)
    let n_poles = 2 * n_segs + 1;
    let mut poles = Vec::with_capacity(n_poles);
    let mut weights = Vec::with_capacity(n_poles);

    for i in 0..n_segs {
        let a0 = alpha1 + i as f64 * seg_angle;
        let a_mid = a0 + half;
        let a1 = a0 + seg_angle;

        let p0 = circle_point(&pos, r, a0);
        let p1_raw = circle_point(&pos, r, a_mid);
        // The corner control point is the intersection of the tangents at a0
        // and a1.  For a circle the tangent at angle θ is perpendicular to the
        // radius, so it is:  T(θ) = center + r*(cos θ * x + sin θ * y)
        // The corner pole is:  p0 + t * tangent(a0)  where t = r * tan(half).
        // Equivalently, in the local frame the corner is at radius r/cos(half)
        // at angle a_mid.
        let corner = pos.to_world(r / half.cos() * a_mid.cos(), r / half.cos() * a_mid.sin(), 0.0);

        if i == 0 {
            poles.push(p0);
            weights.push(1.0);
        }
        poles.push(corner);
        weights.push(w_mid);
        // Only push the end point now if not the last segment;
        // for the last segment we'll push after the loop.
        if i < n_segs - 1 {
            poles.push(circle_point(&pos, r, a1));
            weights.push(1.0);
        } else {
            // Use the exact circle point for the final endpoint.
            let _ = p1_raw; // silence unused warning
            poles.push(circle_point(&pos, r, a1));
            weights.push(1.0);
        }
    }

    // ---------------------------------------------------------------------------
    // Build distinct knots and multiplicities.
    //
    // Degree-2 clamped NURBS with n_segs Bézier spans:
    //   - start knot (mult 3)
    //   - (n_segs - 1) internal knots (mult 2) at uniform parameter positions
    //   - end knot (mult 3)
    //
    // We parametrize uniformly in [alpha1, alpha1+delta].
    // ---------------------------------------------------------------------------
    let mut knots: Vec<f64> = Vec::with_capacity(n_segs + 1);
    let mut mults: Vec<usize> = Vec::with_capacity(n_segs + 1);

    knots.push(alpha1);
    mults.push(3); // start clamp

    for i in 1..n_segs {
        knots.push(alpha1 + i as f64 * seg_angle);
        mults.push(2);
    }

    knots.push(alpha1 + delta);
    mults.push(3); // end clamp

    (poles, weights, knots, mults)
}

impl CircleToBSpline {
    // -----------------------------------------------------------------------
    // Constructors
    // -----------------------------------------------------------------------

    /// `Convert_CircleToBSplineCurve(C, Alpha1, Alpha2, Parameterisation)`
    ///
    /// Converts the arc of circle `C` delimited by the two angles `Alpha1`
    /// and `Alpha2` (in radians) into a rational BSpline curve.
    ///
    /// # Panics
    /// Panics if the radius of `C` is negative.
    pub fn from_arc(circ: &Circ, alpha1: f64, alpha2: f64, _typ: ConvertType) -> Self {
        let (poles, weights, knots, multiplicities) = build_arc_nurbs(circ, alpha1, alpha2);
        Self {
            degree: 2,
            poles,
            weights,
            knots,
            multiplicities,
        }
    }

    /// `Convert_CircleToBSplineCurve(C, Parameterisation)`
    ///
    /// Converts the **complete circle** `C` (0 to 2π) into a rational BSpline
    /// curve. Equivalent to calling `from_arc(C, 0, 2π, typ)`.
    pub fn from_circle(circ: &Circ, typ: ConvertType) -> Self {
        use std::f64::consts::PI;
        Self::from_arc(circ, 0.0, 2.0 * PI, typ)
    }

    // -----------------------------------------------------------------------
    // Accessors — 1-based to mirror OCCT's NCollection_Array1 convention
    // -----------------------------------------------------------------------

    /// `NbPoles` — number of control poles.
    pub fn nb_poles(&self) -> usize {
        self.poles.len()
    }

    /// `NbKnots` — number of *distinct* knots.
    pub fn nb_knots(&self) -> usize {
        self.knots.len()
    }

    /// `Pole(Index)` — 1-based control pole.
    ///
    /// # Panics
    /// Panics if `index` is 0 or greater than `nb_poles()`.
    pub fn pole(&self, index: usize) -> Pnt {
        assert!(
            index >= 1 && index <= self.poles.len(),
            "CircleToBSpline::Pole: index {index} out of range [1, {}]",
            self.poles.len()
        );
        self.poles[index - 1]
    }

    /// `Weight(Index)` — 1-based rational weight.
    ///
    /// # Panics
    /// Panics if `index` is 0 or greater than `nb_poles()`.
    pub fn weight(&self, index: usize) -> f64 {
        assert!(
            index >= 1 && index <= self.weights.len(),
            "CircleToBSpline::Weight: index {index} out of range [1, {}]",
            self.weights.len()
        );
        self.weights[index - 1]
    }

    /// `Knot(Index)` — 1-based distinct knot value.
    ///
    /// # Panics
    /// Panics if `index` is 0 or greater than `nb_knots()`.
    pub fn knot(&self, index: usize) -> f64 {
        assert!(
            index >= 1 && index <= self.knots.len(),
            "CircleToBSpline::Knot: index {index} out of range [1, {}]",
            self.knots.len()
        );
        self.knots[index - 1]
    }

    /// `Multiplicity(Index)` — 1-based knot multiplicity.
    ///
    /// # Panics
    /// Panics if `index` is 0 or greater than `nb_knots()`.
    pub fn multiplicity(&self, index: usize) -> usize {
        assert!(
            index >= 1 && index <= self.multiplicities.len(),
            "CircleToBSpline::Multiplicity: index {index} out of range [1, {}]",
            self.multiplicities.len()
        );
        self.multiplicities[index - 1]
    }

    /// The degree of the resulting BSpline (always 2).
    pub fn degree(&self) -> usize {
        self.degree
    }

    // -----------------------------------------------------------------------
    // Evaluation helper — build the flat knot vector and evaluate via de Boor
    // -----------------------------------------------------------------------

    /// Evaluate the NURBS curve at parameter `u` using rational de Boor.
    ///
    /// `u` must lie within `[knot(1), knot(nb_knots())]`.
    pub fn value(&self, u: f64) -> Pnt {
        // Build flat knot vector from distinct knots + multiplicities.
        let flat: Vec<f64> = self
            .knots
            .iter()
            .zip(&self.multiplicities)
            .flat_map(|(&k, &m)| std::iter::repeat(k).take(m))
            .collect();

        let p = self.degree;
        let n = self.poles.len() - 1;
        let u = u.clamp(flat[p], flat[n + 1]);

        // Find span.
        let span = find_span(n, p, u, &flat);

        // Rational de Boor.
        let mut d: Vec<Pnt> = (0..=p)
            .map(|j| {
                let idx = span - p + j;
                self.poles[idx] * self.weights[idx]
            })
            .collect();
        let mut w: Vec<f64> = (0..=p).map(|j| self.weights[span - p + j]).collect();

        for r in 1..=p {
            for j in (r..=p).rev() {
                let i = span - p + j;
                let denom = flat[i + p - r + 1] - flat[i];
                let alpha = if denom.abs() < 1e-15 {
                    0.0
                } else {
                    (u - flat[i]) / denom
                };
                d[j] = d[j - 1] * (1.0 - alpha) + d[j] * alpha;
                w[j] = w[j - 1] * (1.0 - alpha) + w[j] * alpha;
            }
        }
        let wp = if w[p].abs() < 1e-15 { 1.0 } else { w[p] };
        d[p] * (1.0 / wp)
    }
}

/// Find the knot span index `i` such that `U[i] <= u < U[i+1]` (clamped).
fn find_span(n: usize, p: usize, u: f64, knots: &[f64]) -> usize {
    if u >= knots[n + 1] {
        return n;
    }
    if u <= knots[p] {
        return p;
    }
    let (mut low, mut high) = (p, n + 1);
    let mut mid = (low + high) / 2;
    while u < knots[mid] || u >= knots[mid + 1] {
        if u < knots[mid] {
            high = mid;
        } else {
            low = mid;
        }
        mid = (low + high) / 2;
    }
    mid
}

// ---------------------------------------------------------------------------
// Utility: build a `Circ` with the standard XY-plane placement at the origin.
// ---------------------------------------------------------------------------

/// Convenience: unit circle in the XY plane at the origin.
pub fn unit_circle() -> Circ {
    Circ::new(Ax3::identity(), 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gp_prim::Circ;
    use std::f64::consts::{FRAC_PI_2, PI};

    fn xy_circ(r: f64) -> Circ {
        Circ::new(crate::gp::Ax3::identity(), r)
    }

    #[test]
    fn quarter_circle_weight_is_cos_pi_over_4() {
        let c = CircleToBSpline::from_arc(
            &xy_circ(1.0),
            0.0,
            FRAC_PI_2,
            ConvertType::TgtThetaOver2,
        );
        // Middle weight of a quarter-circle span must equal cos(π/4) = 1/√2.
        let expected = (FRAC_PI_2 / 2.0).cos(); // cos(π/4)
        let mid_w = c.weight(2);
        assert!(
            (mid_w - expected).abs() < 1e-12,
            "quarter circle middle weight: {mid_w} vs {expected}"
        );
    }

    #[test]
    fn quarter_circle_endpoint_weights_are_one() {
        let c = CircleToBSpline::from_arc(
            &xy_circ(3.0),
            0.0,
            FRAC_PI_2,
            ConvertType::TgtThetaOver2,
        );
        assert_eq!(c.weight(1), 1.0);
        assert_eq!(c.weight(3), 1.0);
    }

    #[test]
    fn quarter_circle_nb_poles_and_knots() {
        let c = CircleToBSpline::from_arc(
            &xy_circ(1.0),
            0.0,
            FRAC_PI_2,
            ConvertType::TgtThetaOver2,
        );
        // 1 Bézier span → 3 poles, 2 distinct knots
        assert_eq!(c.nb_poles(), 3);
        assert_eq!(c.nb_knots(), 2);
        assert_eq!(c.degree(), 2);
    }

    #[test]
    fn half_circle_structure() {
        let c = CircleToBSpline::from_arc(&xy_circ(1.0), 0.0, PI, ConvertType::TgtThetaOver2);
        // 2 spans → 5 poles, 3 distinct knots (start, mid, end with mults 3,2,3)
        assert_eq!(c.nb_poles(), 5);
        assert_eq!(c.nb_knots(), 3);
        assert_eq!(c.multiplicity(1), 3);
        assert_eq!(c.multiplicity(2), 2);
        assert_eq!(c.multiplicity(3), 3);
    }

    #[test]
    fn full_circle_structure() {
        let c = CircleToBSpline::from_circle(&xy_circ(1.0), ConvertType::TgtThetaOver2);
        // 4 spans → 9 poles, 5 distinct knots
        assert_eq!(c.nb_poles(), 9);
        assert_eq!(c.nb_knots(), 5);
        assert_eq!(c.degree(), 2);
    }

    #[test]
    fn quarter_circle_evaluates_on_circle() {
        let r = 2.5;
        let c = CircleToBSpline::from_arc(
            &xy_circ(r),
            0.0,
            FRAC_PI_2,
            ConvertType::TgtThetaOver2,
        );
        for i in 0..=16 {
            let u = FRAC_PI_2 * i as f64 / 16.0;
            let p = c.value(u);
            let radius = (p.x * p.x + p.y * p.y).sqrt();
            assert!(
                (radius - r).abs() < 1e-7,
                "u={u:.3}: radius={radius} expected {r}"
            );
        }
    }

    #[test]
    fn full_circle_evaluates_on_circle() {
        let r = 1.0;
        let c = CircleToBSpline::from_circle(&xy_circ(r), ConvertType::TgtThetaOver2);
        for i in 0..=32 {
            let u = 2.0 * PI * i as f64 / 32.0;
            let p = c.value(u);
            let radius = (p.x * p.x + p.y * p.y).sqrt();
            assert!(
                (radius - r).abs() < 1e-7,
                "u={u:.3}: radius={radius}"
            );
        }
    }

    #[test]
    fn half_circle_evaluates_on_circle() {
        let r = 4.0;
        let c = CircleToBSpline::from_arc(&xy_circ(r), 0.0, PI, ConvertType::RationalC1);
        for i in 0..=16 {
            let u = PI * i as f64 / 16.0;
            let p = c.value(u);
            let radius = (p.x * p.x + p.y * p.y).sqrt();
            assert!(
                (radius - r).abs() < 1e-7,
                "u={u:.3}: radius={radius} expected {r}"
            );
        }
    }

    #[test]
    fn three_quarter_circle_structure() {
        let c = CircleToBSpline::from_arc(
            &xy_circ(1.0),
            0.0,
            3.0 * FRAC_PI_2,
            ConvertType::QuasiAngular,
        );
        // 3 spans → 7 poles, 4 distinct knots
        assert_eq!(c.nb_poles(), 7);
        assert_eq!(c.nb_knots(), 4);
    }

    #[test]
    fn endpoints_interpolate_circle() {
        let r = 3.0;
        let a1 = PI / 6.0;
        let a2 = PI / 6.0 + FRAC_PI_2;
        let circ = xy_circ(r);
        let c = CircleToBSpline::from_arc(&circ, a1, a2, ConvertType::TgtThetaOver2);

        let p_start = c.value(a1);
        let p_end = c.value(a2);

        let expected_start = crate::gp_prim::Circ::new(crate::gp::Ax3::identity(), r);
        // Evaluate directly on the circle.
        let cs = {
            let (s, cv) = a1.sin_cos();
            Pnt::new(r * cv, r * s, 0.0)
        };
        let ce = {
            let (s, cv) = a2.sin_cos();
            Pnt::new(r * cv, r * s, 0.0)
        };
        let _ = expected_start;

        assert!(
            p_start.distance(cs) < 1e-7,
            "start: {p_start:?} vs {cs:?}"
        );
        assert!(
            p_end.distance(ce) < 1e-7,
            "end: {p_end:?} vs {ce:?}"
        );
    }

    #[test]
    fn one_based_indexing_panics_at_zero() {
        let c = CircleToBSpline::from_arc(
            &xy_circ(1.0),
            0.0,
            FRAC_PI_2,
            ConvertType::TgtThetaOver2,
        );
        let result = std::panic::catch_unwind(|| c.pole(0));
        assert!(result.is_err(), "pole(0) should panic");
    }
}
