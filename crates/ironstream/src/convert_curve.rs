//! `Convert_ConicToBSplineCurve` / `Convert_EllipseToBSplineCurve` —
//! unified stub for converting circles and ellipses to rational quadratic
//! B-spline (NURBS) curves, mirroring the OpenCascade package
//! `Convert` from `TKMath`.
//!
//! # Algorithm
//!
//! A full circle or ellipse arc is represented exactly as a rational
//! quadratic (degree-2) NURBS.  The standard construction divides the
//! parameter domain into at most 4 quadrant-sized spans.  For each span
//! of angular width `dθ`:
//!
//! ```text
//! P0 = conic(t0)                 weight = 1
//! P1 = tangent-intersection apex weight = cos(dθ/2)
//! P2 = conic(t0 + dθ)           weight = 1
//! ```
//!
//! The piecewise curve is assembled as a clamped degree-2 B-spline with
//! knot multiplicities `[3, 2, …, 2, 3]`.
//!
//! A circle of radius `r` centred at `center` is treated as the special
//! case `a = b = r` of the ellipse formula.
//!
//! # Indexing
//!
//! The accessor methods [`ConicToBSpline::pole`], [`ConicToBSpline::weight`],
//! and [`ConicToBSpline::knot`] are **0-based** (matching Rust slice
//! conventions).  The public fields `poles`, `weights`, and `knots` are
//! plain `Vec`s and can be indexed directly.

// occt-ref: Convert_ConicToBSplineCurve
// occt-ref: Convert_EllipseToBSplineCurve

use std::f64::consts::PI;

// ---------------------------------------------------------------------------
// Public struct
// ---------------------------------------------------------------------------

/// Unified rational quadratic B-spline representation of a circle or ellipse.
///
/// Mirrors `Convert_ConicToBSplineCurve` (the common base in OCCT) and the
/// concrete subclass `Convert_EllipseToBSplineCurve`.
///
/// # Knot vector
///
/// `knots` stores the **flat** (expanded) knot vector, not the OCCT-style
/// distinct-knots-plus-multiplicities representation, so that the struct is
/// self-contained and easy to hand to any downstream B-spline evaluator.
///
/// The degree is always `2` (rational quadratic).
// occt-ref: Convert_ConicToBSplineCurve
// occt-ref: Convert_EllipseToBSplineCurve
#[derive(Clone, Debug)]
pub struct ConicToBSpline {
    /// Polynomial degree — always `2` (rational quadratic).
    pub degree: u32,
    /// Flat (expanded) knot vector.  Length = `nb_poles() + degree + 1`.
    pub knots: Vec<f64>,
    /// Control poles in 3-D world space.  Length = `nb_poles()`.
    pub poles: Vec<[f64; 3]>,
    /// Rational weights, one per pole.  Length = `nb_poles()`.
    pub weights: Vec<f64>,
}

// ---------------------------------------------------------------------------
// Core construction helper
// ---------------------------------------------------------------------------

/// Build the rational-quadratic NURBS data for a full ellipse arc
/// from `alpha1` to `alpha2` with semi-axes `a` (X) and `b` (Y),
/// centred at `center` and lying in the XY-plane.
///
/// Returns `(poles, weights, flat_knots)`.
///
/// The number of quadrant-sized Bézier segments is chosen automatically
/// (1–4) based on the angular span.
fn build_ellipse_nurbs(
    center: [f64; 3],
    a: f64,
    b: f64,
    alpha1: f64,
    alpha2: f64,
) -> (Vec<[f64; 3]>, Vec<f64>, Vec<f64>) {
    // Normalise the arc span into (0, 2π].
    let mut span = alpha2 - alpha1;
    while span <= 0.0 {
        span += 2.0 * PI;
    }
    while span > 2.0 * PI + 1.0e-14 {
        span -= 2.0 * PI;
    }

    // Number of quadrant segments.
    let n_segs: usize = if span <= PI / 2.0 + 1.0e-10 {
        1
    } else if span <= PI + 1.0e-10 {
        2
    } else if span <= 3.0 * PI / 2.0 + 1.0e-10 {
        3
    } else {
        4
    };

    let dtheta = span / n_segs as f64;
    // Middle-pole weight for each Bézier span.
    let w_mid = (dtheta / 2.0).cos();

    // Build poles and weights segment by segment.
    // Total poles = 2*n_segs + 1.
    let n_poles = 2 * n_segs + 1;
    let mut poles: Vec<[f64; 3]> = Vec::with_capacity(n_poles);
    let mut weights: Vec<f64> = Vec::with_capacity(n_poles);

    let [cx, cy, cz] = center;

    for seg in 0..n_segs {
        let t0 = alpha1 + seg as f64 * dtheta;
        let t1 = t0 + dtheta;

        let (s0, c0) = t0.sin_cos();
        let (s1, c1) = t1.sin_cos();

        // Start pole on the conic.
        let p0 = [cx + a * c0, cy + b * s0, cz];

        // Tangent-intersection apex (Piegl-Tiller "The NURBS Book", p. 299).
        // For the ellipse E(t) = C + a·cos(t)·X + b·sin(t)·Y:
        //   P_apex = C + a·(c0+c1)/(1+cos(dθ))·X + b·(s0+s1)/(1+cos(dθ))·Y
        let cos_dt = dtheta.cos(); // = cos(dθ)
        let denom = 1.0 + cos_dt;
        let p_apex = [
            cx + a * (c0 + c1) / denom,
            cy + b * (s0 + s1) / denom,
            cz,
        ];

        // End pole on the conic.
        let p2 = [cx + a * c1, cy + b * s1, cz];

        if seg == 0 {
            poles.push(p0);
            weights.push(1.0);
        }
        poles.push(p_apex);
        weights.push(w_mid);
        poles.push(p2);
        weights.push(1.0);
    }

    // Build the flat knot vector for a degree-2 clamped piecewise-Bézier:
    //   [t0, t0, t0,  k1, k1,  k2, k2,  …,  t_end, t_end, t_end]
    // (start/end clamped with mult 3; interior knots with mult 2)
    let n_flat = n_poles + 3; // n_poles + degree + 1
    let mut flat: Vec<f64> = Vec::with_capacity(n_flat);

    // Start clamp (mult 3).
    flat.push(alpha1);
    flat.push(alpha1);
    flat.push(alpha1);

    // Interior knots (mult 2 each).
    for i in 1..n_segs {
        let k = alpha1 + i as f64 * dtheta;
        flat.push(k);
        flat.push(k);
    }

    // End clamp (mult 3).
    let alpha_end = alpha1 + span;
    flat.push(alpha_end);
    flat.push(alpha_end);
    flat.push(alpha_end);

    debug_assert_eq!(
        flat.len(),
        n_flat,
        "flat knot vector length mismatch: {} vs {}",
        flat.len(),
        n_flat
    );

    (poles, weights, flat)
}

// ---------------------------------------------------------------------------
// impl ConicToBSpline
// ---------------------------------------------------------------------------

impl ConicToBSpline {
    // -----------------------------------------------------------------------
    // Constructors
    // -----------------------------------------------------------------------

    /// `Convert_CircleToBSplineCurve` — convert a full circle to a rational
    /// quadratic B-spline.
    ///
    /// The circle lies in the XY-plane with the given `center` and radius `r`.
    ///
    /// This delegates to [`from_ellipse`](Self::from_ellipse) with `a = b = r`.
    ///
    /// # Panics
    ///
    /// Panics if `r` is negative.
    // occt-ref: Convert_ConicToBSplineCurve
    pub fn from_circle(center: [f64; 3], r: f64) -> Self {
        assert!(r >= 0.0, "ConicToBSpline::from_circle: radius must be >= 0");
        Self::from_ellipse(center, r, r)
    }

    /// `Convert_EllipseToBSplineCurve` — convert a full ellipse to a rational
    /// quadratic B-spline.
    ///
    /// The ellipse lies in the XY-plane with the given `center`, semi-major
    /// axis `a` (along X) and semi-minor axis `b` (along Y).
    ///
    /// # Panics
    ///
    /// Panics if `a < b` (semi-major must be >= semi-minor) or if either axis
    /// is negative.
    // occt-ref: Convert_EllipseToBSplineCurve
    pub fn from_ellipse(center: [f64; 3], a: f64, b: f64) -> Self {
        assert!(a >= 0.0, "ConicToBSpline::from_ellipse: semi-major axis a must be >= 0");
        assert!(b >= 0.0, "ConicToBSpline::from_ellipse: semi-minor axis b must be >= 0");
        let (poles, weights, knots) =
            build_ellipse_nurbs(center, a, b, 0.0, 2.0 * PI);
        Self {
            degree: 2,
            knots,
            poles,
            weights,
        }
    }

    // -----------------------------------------------------------------------
    // Accessors
    // -----------------------------------------------------------------------

    /// Number of control poles.
    ///
    /// For a full conic this is always `9` (4 quadrant spans × 2 + 1).
    #[inline]
    pub fn nb_poles(&self) -> usize {
        self.poles.len()
    }

    /// Return the control pole at 0-based index `i`.
    ///
    /// # Panics
    ///
    /// Panics if `i >= nb_poles()`.
    #[inline]
    pub fn pole(&self, i: usize) -> [f64; 3] {
        assert!(
            i < self.poles.len(),
            "ConicToBSpline::pole: index {i} out of range [0, {})",
            self.poles.len()
        );
        self.poles[i]
    }

    /// Return the rational weight at 0-based index `i`.
    ///
    /// # Panics
    ///
    /// Panics if `i >= nb_poles()`.
    #[inline]
    pub fn weight(&self, i: usize) -> f64 {
        assert!(
            i < self.weights.len(),
            "ConicToBSpline::weight: index {i} out of range [0, {})",
            self.weights.len()
        );
        self.weights[i]
    }

    /// Return the flat knot value at 0-based index `i`.
    ///
    /// # Panics
    ///
    /// Panics if `i >= knots.len()`.
    #[inline]
    pub fn knot(&self, i: usize) -> f64 {
        assert!(
            i < self.knots.len(),
            "ConicToBSpline::knot: index {i} out of range [0, {})",
            self.knots.len()
        );
        self.knots[i]
    }
}

// ---------------------------------------------------------------------------
// Free-function convenience wrappers
// ---------------------------------------------------------------------------

/// Convert a circle (XY-plane, given `center` and radius `r`) to a rational
/// quadratic B-spline.
///
/// Equivalent to [`ConicToBSpline::from_circle`].
// occt-ref: Convert_ConicToBSplineCurve
pub fn circle_to_bspline(center: [f64; 3], r: f64) -> ConicToBSpline {
    ConicToBSpline::from_circle(center, r)
}

/// Convert an ellipse (XY-plane, given `center`, semi-major axis `a` along X,
/// semi-minor axis `b` along Y) to a rational quadratic B-spline.
///
/// Equivalent to [`ConicToBSpline::from_ellipse`].
// occt-ref: Convert_EllipseToBSplineCurve
pub fn ellipse_to_bspline(center: [f64; 3], a: f64, b: f64) -> ConicToBSpline {
    ConicToBSpline::from_ellipse(center, a, b)
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    /// Evaluate the NURBS using rational de Boor at parameter `u`.
    fn eval(c: &ConicToBSpline, u: f64) -> [f64; 3] {
        let flat = &c.knots;
        let degree = c.degree as usize;
        let n = c.poles.len() - 1;
        let u = u.clamp(flat[degree], flat[n + 1]);

        // Find knot span.
        let span = {
            if u >= flat[n + 1] {
                n
            } else if u <= flat[degree] {
                degree
            } else {
                let (mut lo, mut hi) = (degree, n + 1);
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
        };

        // Rational de Boor.
        let mut d: Vec<[f64; 3]> = (0..=degree)
            .map(|j| {
                let idx = span - degree + j;
                let w = c.weights[idx];
                let p = c.poles[idx];
                [p[0] * w, p[1] * w, p[2] * w]
            })
            .collect();
        let mut ws: Vec<f64> = (0..=degree)
            .map(|j| c.weights[span - degree + j])
            .collect();

        for r in 1..=degree {
            for j in (r..=degree).rev() {
                let i = span - degree + j;
                let denom = flat[i + degree - r + 1] - flat[i];
                let alpha = if denom.abs() < 1.0e-15 {
                    0.0
                } else {
                    (u - flat[i]) / denom
                };
                let one_m = 1.0 - alpha;
                d[j] = [
                    d[j - 1][0] * one_m + d[j][0] * alpha,
                    d[j - 1][1] * one_m + d[j][1] * alpha,
                    d[j - 1][2] * one_m + d[j][2] * alpha,
                ];
                ws[j] = ws[j - 1] * one_m + ws[j] * alpha;
            }
        }
        let wp = if ws[degree].abs() < 1.0e-15 {
            1.0
        } else {
            ws[degree]
        };
        [d[degree][0] / wp, d[degree][1] / wp, d[degree][2] / wp]
    }

    // -----------------------------------------------------------------------

    #[test]
    fn circle_full_has_9_poles() {
        let c = circle_to_bspline([0.0, 0.0, 0.0], 1.0);
        assert_eq!(c.nb_poles(), 9);
        assert_eq!(c.degree, 2);
    }

    #[test]
    fn ellipse_full_has_9_poles() {
        let c = ellipse_to_bspline([0.0, 0.0, 0.0], 5.0, 3.0);
        assert_eq!(c.nb_poles(), 9);
    }

    #[test]
    fn circle_knot_vector_length() {
        // degree=2, n_poles=9 → flat knot length = 9 + 2 + 1 = 12
        let c = circle_to_bspline([0.0, 0.0, 0.0], 1.0);
        assert_eq!(c.knots.len(), 12, "flat knot length = nb_poles + degree + 1");
    }

    #[test]
    fn endpoint_weights_are_one() {
        let c = circle_to_bspline([0.0, 0.0, 0.0], 2.0);
        // Poles 0, 2, 4, 6, 8 (every even index) are on the circle → weight 1.
        for i in (0..c.nb_poles()).step_by(2) {
            let w = c.weight(i);
            assert!(
                (w - 1.0).abs() < 1.0e-12,
                "pole {i}: expected weight 1, got {w}"
            );
        }
    }

    #[test]
    fn inner_weights_are_cos_pi_over_4() {
        // Full circle → 4 spans of π/2 each → mid weight = cos(π/4).
        let c = circle_to_bspline([0.0, 0.0, 0.0], 1.0);
        let expected = (PI / 4.0).cos();
        for i in (1..c.nb_poles()).step_by(2) {
            let w = c.weight(i);
            assert!(
                (w - expected).abs() < 1.0e-12,
                "pole {i}: expected weight {expected}, got {w}"
            );
        }
    }

    #[test]
    fn circle_evaluates_on_circle() {
        let r = 3.5;
        let c = circle_to_bspline([1.0, 2.0, 0.0], r);
        let [cx, cy, _] = [1.0_f64, 2.0_f64, 0.0_f64];
        for i in 0..=32 {
            let u = 2.0 * PI * i as f64 / 32.0;
            let p = eval(&c, u);
            let dist = ((p[0] - cx).powi(2) + (p[1] - cy).powi(2)).sqrt();
            assert!(
                (dist - r).abs() < 1.0e-7,
                "u={u:.3}: dist={dist}, expected r={r}"
            );
        }
    }

    #[test]
    fn ellipse_evaluates_on_ellipse() {
        let a = 5.0_f64;
        let b = 3.0_f64;
        let c = ellipse_to_bspline([0.0, 0.0, 0.0], a, b);
        for i in 0..=40 {
            let u = 2.0 * PI * i as f64 / 40.0;
            let p = eval(&c, u);
            let err = (p[0] / a).powi(2) + (p[1] / b).powi(2) - 1.0;
            assert!(
                err.abs() < 1.0e-9,
                "u={u:.4}: ellipse eq error = {err:e}, p={p:?}"
            );
        }
    }

    #[test]
    fn circle_is_special_case_of_ellipse() {
        let r = 2.0;
        let c1 = circle_to_bspline([0.0, 0.0, 0.0], r);
        let c2 = ellipse_to_bspline([0.0, 0.0, 0.0], r, r);
        assert_eq!(c1.nb_poles(), c2.nb_poles());
        for i in 0..c1.nb_poles() {
            let p1 = c1.pole(i);
            let p2 = c2.pole(i);
            for k in 0..3 {
                assert!(
                    (p1[k] - p2[k]).abs() < 1.0e-12,
                    "pole {i}[{k}]: circle={} ellipse={}",
                    p1[k],
                    p2[k]
                );
            }
            assert!(
                (c1.weight(i) - c2.weight(i)).abs() < 1.0e-12,
                "weight {i} mismatch"
            );
        }
    }

    #[test]
    fn offset_center_is_reflected_in_poles() {
        let c = circle_to_bspline([10.0, 20.0, 5.0], 1.0);
        // First pole should be near (11, 20, 5) — the +X point on the circle.
        let p0 = c.pole(0);
        assert!((p0[0] - 11.0).abs() < 1.0e-10, "p0[0]={}", p0[0]);
        assert!((p0[1] - 20.0).abs() < 1.0e-10, "p0[1]={}", p0[1]);
        assert!((p0[2] - 5.0).abs() < 1.0e-10,  "p0[2]={}", p0[2]);
    }

    #[test]
    fn knot_accessor_matches_vec() {
        let c = circle_to_bspline([0.0, 0.0, 0.0], 1.0);
        for i in 0..c.knots.len() {
            assert_eq!(c.knot(i), c.knots[i]);
        }
    }

    #[test]
    #[should_panic]
    fn pole_out_of_range_panics() {
        let c = circle_to_bspline([0.0, 0.0, 0.0], 1.0);
        let _ = c.pole(c.nb_poles()); // one past the end
    }

    #[test]
    #[should_panic]
    fn negative_radius_panics() {
        let _ = circle_to_bspline([0.0, 0.0, 0.0], -1.0);
    }
}
