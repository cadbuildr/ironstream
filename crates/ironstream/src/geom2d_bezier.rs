//! `Geom2d_BezierCurve` — a rational or non-rational planar Bezier curve,
//! faithfully reproducing OpenCascade's `Geom2d_BezierCurve` (a
//! `Geom2d_BoundedCurve`).
//!
//! A Bezier curve is defined by a table of *poles* (control points) and,
//! optionally, a parallel table of *weights*:
//!
//! ```text
//! non-rational:  C(u) = sum_i  P_i * B_i^n(u)
//! rational:      C(u) = ( sum_i w_i P_i B_i^n(u) ) / ( sum_i w_i B_i^n(u) )
//! ```
//!
//! where `B_i^n(u) = C(n,i) * u^i * (1-u)^(n-i)` are the Bernstein polynomials
//! of degree `n = NbPoles - 1`. The parameter range is always `[0, 1]`.
//!
//! Like OCCT, the poles and weights are addressed with **1-based** indices in
//! `[1, NbPoles]`, the degree is `NbPoles - 1`, the maximum degree is `25`, and
//! a weight is "zero" / two weights are "equal" with respect to
//! [`gp::Resolution()`](RESOLUTION) `= 1e-15`.
//!
//! This module is self-contained: it builds on `gp2d::{Pnt2d, Vec2d, Trsf2d}`
//! and `precision`, with all Bezier math implemented here from scratch.

use crate::gp2d::{Pnt2d, Trsf2d, Vec2d};

/// `gp::Resolution()` — the absolute threshold below which a weight is treated
/// as zero, and below which two weights are treated as identical.
pub const RESOLUTION: f64 = 1.0e-15;

/// `Geom2d_BezierCurve::MaxDegree()` — the largest admissible degree (25), and
/// hence `MaxDegree + 1 = 26` is the largest admissible number of poles.
pub const MAX_DEGREE: usize = 25;

/// `gp_Pnt2d::IsEqual(Other, Tol)` — coincidence within a linear tolerance.
#[inline]
#[cfg_attr(not(test), allow(dead_code))]
fn pnt2d_is_equal(a: Pnt2d, b: Pnt2d, tol: f64) -> bool {
    a.distance(b) <= tol
}

/// Describes a rational or non-rational planar Bezier curve.
///
/// Internally the poles and weights are stored in `0`-based `Vec`s, but the
/// public accessors take and return OCCT's `1`-based indices. A curve is
/// flagged rational only when its weights are not all equal (within
/// [`RESOLUTION`]); a non-rational curve carries an implicit unit-weight array.
// occt: Geom2d_BezierCurve
#[derive(Clone, Debug)]
pub struct Geom2dBezierCurve {
    poles: Vec<Pnt2d>,
    /// Always sized `NbPoles`; for a non-rational curve every entry is `1.0`.
    weights: Vec<f64>,
    rational: bool,
    closed: bool,
}

impl Geom2dBezierCurve {
    // ----------------------------------------------------------------- ctors

    /// `Geom2d_BezierCurve(const Array1OfPnt2d& CurvePoles)` — a non-rational
    /// curve; all weights default to `1`.
    ///
    /// Panics (OCCT raises `Standard_ConstructionError`) if there are fewer
    /// than 2 poles or more than `MaxDegree + 1`.
    pub fn new(poles: &[Pnt2d]) -> Self {
        Self::check_poles_len(poles.len());
        let n = poles.len();
        let mut c = Self {
            poles: poles.to_vec(),
            weights: vec![1.0; n],
            rational: false,
            closed: false,
        };
        c.update_closed();
        c
    }

    /// `Geom2d_BezierCurve(const Array1OfPnt2d& CurvePoles, const Array1OfReal&
    /// PoleWeights)` — a rational curve. If all the supplied weights are
    /// identical the curve is treated as non-rational.
    ///
    /// Panics if the pole count is out of range, the two arrays differ in
    /// length, or any weight is `<= gp::Resolution()`.
    pub fn with_weights(poles: &[Pnt2d], weights: &[f64]) -> Self {
        Self::check_poles_len(poles.len());
        assert_eq!(
            poles.len(),
            weights.len(),
            "Geom2d_BezierCurve: poles and weights must have the same length"
        );
        for &w in weights {
            assert!(
                w > RESOLUTION,
                "Geom2d_BezierCurve: weight must be > gp::Resolution()"
            );
        }
        let rational = !weights_all_equal(weights);
        let mut c = Self {
            poles: poles.to_vec(),
            weights: weights.to_vec(),
            rational,
            closed: false,
        };
        c.update_closed();
        c
    }

    fn check_poles_len(n: usize) {
        assert!(
            (2..=MAX_DEGREE + 1).contains(&n),
            "Geom2d_BezierCurve: number of poles must be in [2, {}]",
            MAX_DEGREE + 1
        );
    }

    fn update_closed(&mut self) {
        let first = self.poles[0];
        let last = self.poles[self.poles.len() - 1];
        self.closed = first.distance(last) <= RESOLUTION;
    }

    /// Recomputes the rational flag from the current weights.
    fn update_rational(&mut self) {
        self.rational = !weights_all_equal(&self.weights);
    }

    // ----------------------------------------------------------- properties

    /// `MaxDegree()` — the largest admissible degree, `25`.
    #[inline]
    pub fn max_degree() -> usize {
        MAX_DEGREE
    }

    /// `Degree()` — `NbPoles - 1`.
    #[inline]
    pub fn degree(&self) -> usize {
        self.poles.len() - 1
    }

    /// `NbPoles()`.
    #[inline]
    pub fn nb_poles(&self) -> usize {
        self.poles.len()
    }

    /// `IsRational()` — false when all weights are identical (within
    /// [`RESOLUTION`]).
    #[inline]
    pub fn is_rational(&self) -> bool {
        self.rational
    }

    /// `IsClosed()` — true when the first and last poles coincide within
    /// `gp::Resolution()`.
    #[inline]
    pub fn is_closed(&self) -> bool {
        self.closed
    }

    /// `IsPeriodic()` — always false for a Bezier curve.
    #[inline]
    pub fn is_periodic(&self) -> bool {
        false
    }

    /// `IsCN(N)` — a Bezier curve is infinitely differentiable, so this is true
    /// for every `N >= 0`.
    #[inline]
    pub fn is_cn(&self, _n: i32) -> bool {
        true
    }

    /// `FirstParameter()` — `0.0`.
    #[inline]
    pub fn first_parameter(&self) -> f64 {
        0.0
    }

    /// `LastParameter()` — `1.0`.
    #[inline]
    pub fn last_parameter(&self) -> f64 {
        1.0
    }

    /// `Pole(Index)` — the pole at the 1-based `index`.
    ///
    /// Panics if `index` is not in `[1, NbPoles]`.
    #[inline]
    pub fn pole(&self, index: usize) -> Pnt2d {
        assert!(
            (1..=self.poles.len()).contains(&index),
            "Geom2d_BezierCurve::Pole: index out of range"
        );
        self.poles[index - 1]
    }

    /// `Poles()` — all the poles, in order.
    #[inline]
    pub fn poles(&self) -> &[Pnt2d] {
        &self.poles
    }

    /// `Weight(Index)` — the weight at the 1-based `index` (`1.0` for a
    /// non-rational curve).
    ///
    /// Panics if `index` is not in `[1, NbPoles]`.
    #[inline]
    pub fn weight(&self, index: usize) -> f64 {
        assert!(
            (1..=self.weights.len()).contains(&index),
            "Geom2d_BezierCurve::Weight: index out of range"
        );
        self.weights[index - 1]
    }

    /// `Weights()` — `Some(weights)` for a rational curve, `None` otherwise
    /// (matching the OCCT pointer accessor that returns `nullptr` /
    /// `BSplCLib::NoWeights()` for a non-rational curve).
    #[inline]
    pub fn weights(&self) -> Option<&[f64]> {
        if self.rational {
            Some(&self.weights)
        } else {
            None
        }
    }

    /// `WeightsArray()` — the weights array sized to `NbPoles`, always present
    /// (unit weights for a non-rational curve).
    #[inline]
    pub fn weights_array(&self) -> &[f64] {
        &self.weights
    }

    /// `StartPoint()` — the first pole, the value at `u = 0`.
    #[inline]
    pub fn start_point(&self) -> Pnt2d {
        self.poles[0]
    }

    /// `EndPoint()` — the last pole, the value at `u = 1`.
    #[inline]
    pub fn end_point(&self) -> Pnt2d {
        self.poles[self.poles.len() - 1]
    }

    // ------------------------------------------------------------ evaluation

    /// `Value(U)` / `D0(U)` — the point of parameter `U`.
    pub fn value(&self, u: f64) -> Pnt2d {
        self.eval_d0(u)
    }

    /// `EvalD0(U)` — the point of parameter `U` via de Casteljau.
    pub fn eval_d0(&self, u: f64) -> Pnt2d {
        if self.rational {
            let (a, w) = self.homogeneous_value(u);
            Pnt2d::new(a.x / w, a.y / w)
        } else {
            de_casteljau(&self.poles, u)
        }
    }

    /// `EvalD1(U)` — point and first derivative.
    pub fn eval_d1(&self, u: f64) -> (Pnt2d, Vec2d) {
        let d = self.derivatives(u, 1);
        let p = Pnt2d::new(d[0].x, d[0].y);
        (p, d[1])
    }

    /// `EvalD2(U)` — point, first and second derivatives.
    pub fn eval_d2(&self, u: f64) -> (Pnt2d, Vec2d, Vec2d) {
        let d = self.derivatives(u, 2);
        let p = Pnt2d::new(d[0].x, d[0].y);
        (p, d[1], d[2])
    }

    /// `EvalD3(U)` — point, first, second and third derivatives.
    pub fn eval_d3(&self, u: f64) -> (Pnt2d, Vec2d, Vec2d, Vec2d) {
        let d = self.derivatives(u, 3);
        let p = Pnt2d::new(d[0].x, d[0].y);
        (p, d[1], d[2], d[3])
    }

    /// `EvalDN(U, N)` — the `N`-th derivative vector (`N >= 1`).
    ///
    /// Panics if `n < 1` (OCCT raises `Standard_RangeError`).
    pub fn eval_dn(&self, u: f64, n: i32) -> Vec2d {
        assert!(n >= 1, "Geom2d_BezierCurve::EvalDN: N must be >= 1");
        let n = n as usize;
        let d = self.derivatives(u, n);
        d[n]
    }

    /// `D1(U)` alias mirroring the OCCT out-parameter signature.
    pub fn d1(&self, u: f64) -> (Pnt2d, Vec2d) {
        self.eval_d1(u)
    }

    /// Computes derivatives `0..=order` of the curve at `u`.
    ///
    /// For a non-rational curve the result is the Bernstein/de-Casteljau
    /// derivative of the polynomial directly. For a rational curve the
    /// homogeneous numerator `A(u)` and weight `w(u)` are differentiated and the
    /// rational quotient rule (`A = w * C`, solved by Leibniz) yields `C^(k)`.
    fn derivatives(&self, u: f64, order: usize) -> Vec<Vec2d> {
        let n = self.degree();
        // Numerator (poles * weight) and weight derivatives, up to `order`,
        // capped at the polynomial degree (higher derivatives vanish).
        let kmax = order.min(n);

        // Homogeneous control points: (w*P, w).
        let mut hx: Vec<f64> = Vec::with_capacity(self.poles.len());
        let mut hy: Vec<f64> = Vec::with_capacity(self.poles.len());
        let mut hw: Vec<f64> = Vec::with_capacity(self.poles.len());
        for (p, &w) in self.poles.iter().zip(self.weights.iter()) {
            hx.push(w * p.x);
            hy.push(w * p.y);
            hw.push(w);
        }

        // Derivatives of the homogeneous numerator A and weight w.
        let ax = bernstein_derivatives(&hx, u, kmax);
        let ay = bernstein_derivatives(&hy, u, kmax);
        let aw = bernstein_derivatives(&hw, u, kmax);

        // a[k] = (ax[k], ay[k]) is A^(k); wd[k] is w^(k). Pad with zeros above
        // the polynomial degree.
        let a: Vec<Vec2d> = (0..=order)
            .map(|k| {
                if k <= kmax {
                    Vec2d::new(ax[k], ay[k])
                } else {
                    Vec2d::new(0.0, 0.0)
                }
            })
            .collect();
        let wd: Vec<f64> = (0..=order)
            .map(|k| if k <= kmax { aw[k] } else { 0.0 })
            .collect();

        if !self.rational {
            // For a non-rational curve w(u) == 1 and all w^(k>=1) == 0, so the
            // numerator derivatives already are the curve derivatives.
            return a;
        }

        // Rational quotient rule: A = w * C  =>  by Leibniz,
        //   A^(k) = sum_{j=0..k} C(k,j) * w^(j) * C^(k-j)
        // Solving for C^(k) (the j=0 term):
        //   C^(k) = ( A^(k) - sum_{j=1..k} C(k,j) w^(j) C^(k-j) ) / w
        let w0 = wd[0];
        let mut c: Vec<Vec2d> = vec![Vec2d::new(0.0, 0.0); order + 1];
        for k in 0..=order {
            let mut acc = a[k];
            for j in 1..=k {
                let coeff = binomial(k, j) as f64 * wd[j];
                acc = acc - c[k - j] * coeff;
            }
            c[k] = acc * (1.0 / w0);
        }
        c
    }

    /// Homogeneous value `(A(u), w(u))` for a rational curve.
    fn homogeneous_value(&self, u: f64) -> (Vec2d, f64) {
        let n = self.poles.len();
        let mut hx: Vec<f64> = Vec::with_capacity(n);
        let mut hy: Vec<f64> = Vec::with_capacity(n);
        let mut hw: Vec<f64> = Vec::with_capacity(n);
        for (p, &w) in self.poles.iter().zip(self.weights.iter()) {
            hx.push(w * p.x);
            hy.push(w * p.y);
            hw.push(w);
        }
        let x = de_casteljau_scalar(&hx, u);
        let y = de_casteljau_scalar(&hy, u);
        let w = de_casteljau_scalar(&hw, u);
        (Vec2d::new(x, y), w)
    }

    // ------------------------------------------------------------ modifiers

    /// `SetPole(Index, P)` — replaces the pole at the 1-based `index`, leaving
    /// its weight unchanged.
    ///
    /// Panics if `index` is out of `[1, NbPoles]`.
    pub fn set_pole(&mut self, index: usize, p: Pnt2d) {
        assert!(
            (1..=self.poles.len()).contains(&index),
            "Geom2d_BezierCurve::SetPole: index out of range"
        );
        self.poles[index - 1] = p;
        self.update_closed();
    }

    /// `SetPole(Index, P, Weight)` — replaces both the pole and the weight at
    /// the 1-based `index`. May turn the curve rational or non-rational.
    ///
    /// Panics if `index` is out of range or `weight <= gp::Resolution()`.
    pub fn set_pole_with_weight(&mut self, index: usize, p: Pnt2d, weight: f64) {
        assert!(
            (1..=self.poles.len()).contains(&index),
            "Geom2d_BezierCurve::SetPole: index out of range"
        );
        assert!(
            weight > RESOLUTION,
            "Geom2d_BezierCurve::SetPole: weight must be > gp::Resolution()"
        );
        self.poles[index - 1] = p;
        self.weights[index - 1] = weight;
        self.update_rational();
        self.update_closed();
    }

    /// `SetWeight(Index, Weight)` — changes the weight at the 1-based `index`.
    /// May turn the curve rational or non-rational.
    ///
    /// Panics if `index` is out of range or `weight <= gp::Resolution()`.
    pub fn set_weight(&mut self, index: usize, weight: f64) {
        assert!(
            (1..=self.weights.len()).contains(&index),
            "Geom2d_BezierCurve::SetWeight: index out of range"
        );
        assert!(
            weight > RESOLUTION,
            "Geom2d_BezierCurve::SetWeight: weight must be > gp::Resolution()"
        );
        self.weights[index - 1] = weight;
        self.update_rational();
    }

    /// `InsertPoleAfter(Index, P, Weight)` — inserts a pole (with weight) right
    /// after the 1-based `index` (`index = 0` inserts at the front).
    ///
    /// Panics if `index` is not in `[0, NbPoles]`, the result would exceed
    /// `MaxDegree + 1` poles, or `weight <= gp::Resolution()`.
    pub fn insert_pole_after(&mut self, index: usize, p: Pnt2d, weight: f64) {
        assert!(
            index <= self.poles.len(),
            "Geom2d_BezierCurve::InsertPoleAfter: index out of range"
        );
        assert!(
            weight > RESOLUTION,
            "Geom2d_BezierCurve::InsertPoleAfter: weight must be > gp::Resolution()"
        );
        assert!(
            self.poles.len() < MAX_DEGREE + 1,
            "Geom2d_BezierCurve::InsertPoleAfter: too many poles"
        );
        self.poles.insert(index, p);
        self.weights.insert(index, weight);
        self.update_rational();
        self.update_closed();
    }

    /// `InsertPoleBefore(Index, P, Weight)` — inserts a pole (with weight) right
    /// before the 1-based `index`.
    ///
    /// Panics if `index` is not in `[1, NbPoles + 1]`, the result would exceed
    /// `MaxDegree + 1` poles, or `weight <= gp::Resolution()`.
    pub fn insert_pole_before(&mut self, index: usize, p: Pnt2d, weight: f64) {
        assert!(
            (1..=self.poles.len() + 1).contains(&index),
            "Geom2d_BezierCurve::InsertPoleBefore: index out of range"
        );
        self.insert_pole_after(index - 1, p, weight);
    }

    /// `RemovePole(Index)` — removes the pole at the 1-based `index`. May turn
    /// a rational curve non-rational.
    ///
    /// Panics if `index` is out of `[1, NbPoles]` or fewer than 2 poles would
    /// remain.
    pub fn remove_pole(&mut self, index: usize) {
        assert!(
            (1..=self.poles.len()).contains(&index),
            "Geom2d_BezierCurve::RemovePole: index out of range"
        );
        assert!(
            self.poles.len() > 2,
            "Geom2d_BezierCurve::RemovePole: at least 2 poles must remain"
        );
        self.poles.remove(index - 1);
        self.weights.remove(index - 1);
        self.update_rational();
        self.update_closed();
    }

    /// `Increase(Degree)` — raises the curve to `degree`, keeping the geometry
    /// unchanged (degree elevation).
    ///
    /// Panics if `degree` is greater than `MaxDegree` or lower than the current
    /// degree.
    pub fn increase(&mut self, degree: usize) {
        assert!(
            degree <= MAX_DEGREE,
            "Geom2d_BezierCurve::Increase: degree exceeds MaxDegree"
        );
        assert!(
            degree >= self.degree(),
            "Geom2d_BezierCurve::Increase: degree lower than current degree"
        );
        while self.degree() < degree {
            self.elevate_one();
        }
    }

    /// Elevates the degree by exactly one (Bezier degree elevation), working in
    /// homogeneous coordinates so rational curves keep their shape.
    fn elevate_one(&mut self) {
        let n = self.degree(); // old degree
        let np1 = n + 1; // new degree
        // Homogeneous old poles.
        let old_h: Vec<[f64; 3]> = self
            .poles
            .iter()
            .zip(self.weights.iter())
            .map(|(p, &w)| [w * p.x, w * p.y, w])
            .collect();
        // New poles: Q_0 = P_0, Q_{np1} = P_n,
        // Q_i = a*P_{i-1} + (1-a)*P_i with a = i/(n+1).
        let mut new_h: Vec<[f64; 3]> = Vec::with_capacity(np1 + 1);
        new_h.push(old_h[0]);
        for i in 1..=n {
            let a = i as f64 / (np1 as f64);
            let prev = old_h[i - 1];
            let cur = old_h[i];
            new_h.push([
                a * prev[0] + (1.0 - a) * cur[0],
                a * prev[1] + (1.0 - a) * cur[1],
                a * prev[2] + (1.0 - a) * cur[2],
            ]);
        }
        new_h.push(old_h[n]);

        self.poles = new_h
            .iter()
            .map(|h| Pnt2d::new(h[0] / h[2], h[1] / h[2]))
            .collect();
        self.weights = new_h.iter().map(|h| h[2]).collect();
        // Degree elevation never changes the rational/non-rational state of the
        // shape, but recompute defensively.
        self.update_rational();
        self.update_closed();
    }

    /// `Reverse()` — reverses the direction of parametrisation in place:
    /// `Value(NewU) = Value(1 - OldU)`. Poles (and weights) are reversed.
    pub fn reverse(&mut self) {
        self.poles.reverse();
        self.weights.reverse();
        self.update_closed();
    }

    /// `ReversedParameter(U)` — the parameter on the reversed curve, `1 - U`.
    #[inline]
    pub fn reversed_parameter(&self, u: f64) -> f64 {
        1.0 - u
    }

    /// `Segment(U1, U2)` — restricts the curve to the parameter range
    /// `[U1, U2]`, re-parametrised onto `[0, 1]` and oriented from `U1` to `U2`.
    /// `U1`/`U2` may lie outside `[0, 1]`. The poles are recomputed by repeated
    /// de Casteljau subdivision so the geometry is preserved exactly.
    pub fn segment(&mut self, u1: f64, u2: f64) {
        let n = self.poles.len();
        // Homogeneous poles.
        let mut h: Vec<[f64; 3]> = self
            .poles
            .iter()
            .zip(self.weights.iter())
            .map(|(p, &w)| [w * p.x, w * p.y, w])
            .collect();

        // Map the sub-segment [u1, u2] of the unit Bezier onto a new control
        // polygon. Both blossom-based formulas below are standard:
        //   left split at t  -> keep "left" poles of [0, t]
        //   trim start to u1, then end to u2 (re-expressed on the trimmed seg).
        // We use the blossom: the new pole Q_i is the blossom of the original
        // curve evaluated at the multiset {u1 (n-i times), u2 (i times)}.
        h = bezier_subsegment(&h, u1, u2, n - 1);

        self.poles = h
            .iter()
            .map(|p| Pnt2d::new(p[0] / p[2], p[1] / p[2]))
            .collect();
        self.weights = h.iter().map(|p| p[2]).collect();
        self.update_rational();
        self.update_closed();
    }

    /// `Resolution(ToleranceUV, &UTolerance)` — computes the parametric
    /// tolerance ensuring `|t1 - t0| < UTolerance => |C(t1) - C(t0)| <
    /// ToleranceUV`.
    ///
    /// Returns `UTolerance`. It is derived from the largest first-derivative
    /// magnitude: `UTolerance = ToleranceUV / max|C'|`, which bounds the
    /// parametric speed (for a rational curve the weights scale the bound).
    pub fn resolution(&mut self, tolerance_uv: f64) -> f64 {
        // Bound on the derivative magnitude. For a degree-n Bezier the
        // derivative poles are n*(P_{i+1} - P_i); the maximum control-leg length
        // bounds |C'| from above (convex-hull property of the derivative curve).
        let n = self.degree();
        let mut max_deriv = 0.0_f64;
        if self.rational {
            // Use the homogeneous control legs scaled by the minimum weight.
            let wmin = self.weights.iter().cloned().fold(f64::INFINITY, f64::min);
            for i in 0..n {
                let leg = self.poles[i + 1].distance(self.poles[i]);
                let d = (n as f64) * leg;
                if d > max_deriv {
                    max_deriv = d;
                }
            }
            // Rational curves can amplify the parametric speed by the weight
            // spread; divide by wmin to stay conservative.
            if wmin > 0.0 {
                max_deriv /= wmin;
            }
        } else {
            for i in 0..n {
                let leg = self.poles[i + 1].distance(self.poles[i]);
                let d = (n as f64) * leg;
                if d > max_deriv {
                    max_deriv = d;
                }
            }
        }
        if max_deriv <= RESOLUTION {
            // A degenerate (point) curve: any parametric step keeps the point
            // fixed, so the tolerance is unbounded; report the input.
            tolerance_uv.max(RESOLUTION)
        } else {
            tolerance_uv / max_deriv
        }
    }

    /// `Continuity()` — a Bezier curve is `GeomAbs_CN` (infinitely smooth).
    #[inline]
    pub fn continuity(&self) -> Continuity {
        Continuity::CN
    }

    /// `Transform(const gp_Trsf2d& T)` — applies a 2D transform to every pole.
    /// Weights are invariant under an affine transform.
    pub fn transform(&mut self, t: &Trsf2d) {
        for p in self.poles.iter_mut() {
            *p = t.apply(*p);
        }
        self.update_closed();
    }

    /// `Transformed(T)` — a transformed copy.
    pub fn transformed(&self, t: &Trsf2d) -> Geom2dBezierCurve {
        let mut c = self.clone();
        c.transform(t);
        c
    }

    /// `Copy()` — a deep, independent copy (the optimised copy constructor).
    pub fn copy(&self) -> Geom2dBezierCurve {
        self.clone()
    }
}

/// `GeomAbs_Shape` (the subset relevant to a Bezier curve).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Continuity {
    /// `GeomAbs_CN` — infinitely differentiable.
    CN,
}

// ------------------------------------------------------------------- helpers

/// Whether all weights are equal within [`RESOLUTION`].
fn weights_all_equal(weights: &[f64]) -> bool {
    if weights.is_empty() {
        return true;
    }
    let w0 = weights[0];
    weights.iter().all(|&w| (w - w0).abs() <= RESOLUTION)
}

/// de Casteljau evaluation of a Bezier polygon of 2D points at `u in [0,1]`.
fn de_casteljau(poles: &[Pnt2d], u: f64) -> Pnt2d {
    let mut tmp = poles.to_vec();
    let n = tmp.len();
    for r in 1..n {
        for i in 0..(n - r) {
            tmp[i] = tmp[i].lerp(tmp[i + 1], u);
        }
    }
    tmp[0]
}

/// de Casteljau evaluation of a scalar Bezier polygon at `u in [0,1]`.
fn de_casteljau_scalar(coeffs: &[f64], u: f64) -> f64 {
    let mut tmp = coeffs.to_vec();
    let n = tmp.len();
    for r in 1..n {
        for i in 0..(n - r) {
            tmp[i] = tmp[i] * (1.0 - u) + tmp[i + 1] * u;
        }
    }
    tmp[0]
}

/// Derivatives `0..=kmax` of a scalar Bezier polynomial (control coefficients
/// `c`) at `u`. The `k`-th derivative of a degree-`n` Bezier is itself a
/// degree-`(n-k)` Bezier whose control values are
/// `n!/(n-k)! * delta^k c`, where `delta` is forward differencing.
fn bernstein_derivatives(c: &[f64], u: f64, kmax: usize) -> Vec<f64> {
    let n = c.len() - 1; // degree
    let mut out = Vec::with_capacity(kmax + 1);
    // Forward-difference table; diff[k] holds the k-th forward differences.
    let mut diff = c.to_vec();
    // falling = n * (n-1) * ... * (n-k+1)
    let mut falling = 1.0_f64;
    for k in 0..=kmax {
        // The k-th derivative control polygon is `falling * diff` (length n-k+1),
        // a degree-(n-k) Bezier; evaluate it at u.
        let scaled: Vec<f64> = diff[..(n - k + 1)].iter().map(|&v| falling * v).collect();
        out.push(de_casteljau_scalar(&scaled, u));
        // Advance to the next difference order.
        if k < kmax {
            let m = n - k; // current degree before differencing
            let mut next = Vec::with_capacity(m);
            for i in 0..m {
                next.push(diff[i + 1] - diff[i]);
            }
            diff = next;
            falling *= (n - k) as f64;
        }
    }
    out
}

/// Binomial coefficient `C(n, k)` for small `n` (degree <= 25).
fn binomial(n: usize, k: usize) -> u64 {
    if k > n {
        return 0;
    }
    let k = k.min(n - k);
    let mut num = 1u64;
    let mut den = 1u64;
    for i in 0..k {
        num *= (n - i) as u64;
        den *= (i + 1) as u64;
    }
    num / den
}

/// Computes the homogeneous control points of the sub-segment of a degree-`deg`
/// Bezier curve spanning the parameter interval `[u1, u2]`, re-parametrised onto
/// `[0, 1]`. Uses the blossom (polar form): the `i`-th new pole is the blossom
/// of the original curve at the argument multiset with `u1` repeated `deg - i`
/// times and `u2` repeated `i` times.
fn bezier_subsegment(h: &[[f64; 3]], u1: f64, u2: f64, deg: usize) -> Vec<[f64; 3]> {
    let mut out = Vec::with_capacity(deg + 1);
    for i in 0..=deg {
        // Argument list: u1 (deg-i times), u2 (i times).
        let mut args = Vec::with_capacity(deg);
        for _ in 0..(deg - i) {
            args.push(u1);
        }
        for _ in 0..i {
            args.push(u2);
        }
        out.push(blossom(h, &args));
    }
    out
}

/// The blossom (polar form) of a degree-`deg` Bezier curve with homogeneous
/// control points `h`, evaluated at the `deg` arguments in `args`. Each argument
/// performs one de Casteljau reduction step at that parameter.
fn blossom(h: &[[f64; 3]], args: &[f64]) -> [f64; 3] {
    let mut tmp = h.to_vec();
    let n = tmp.len();
    debug_assert_eq!(args.len(), n - 1);
    for (r, &t) in args.iter().enumerate() {
        let count = n - 1 - r;
        for i in 0..count {
            for c in 0..3 {
                tmp[i][c] = tmp[i][c] * (1.0 - t) + tmp[i + 1][c] * t;
            }
        }
    }
    tmp[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_curve() -> Geom2dBezierCurve {
        Geom2dBezierCurve::new(&[
            Pnt2d::new(0.0, 0.0),
            Pnt2d::new(1.0, 1.0),
            Pnt2d::new(2.0, 1.0),
            Pnt2d::new(3.0, 0.0),
        ])
    }

    #[test]
    fn endpoints_interpolate() {
        let c = sample_curve();
        assert!(pnt2d_is_equal(c.value(0.0), Pnt2d::new(0.0, 0.0), 1e-12));
        assert!(pnt2d_is_equal(c.value(1.0), Pnt2d::new(3.0, 0.0), 1e-12));
    }

    #[test]
    fn degree_elevation_preserves_geometry() {
        let mut c = sample_curve();
        let before = c.value(0.5);
        c.increase(6);
        assert_eq!(c.degree(), 6);
        assert!(pnt2d_is_equal(c.value(0.5), before, 1e-12));
    }

    #[test]
    fn segment_matches_subcurve() {
        let mut c = sample_curve();
        let p25 = c.value(0.25);
        let p75 = c.value(0.75);
        c.segment(0.25, 0.75);
        assert!(pnt2d_is_equal(c.start_point(), p25, 1e-9));
        assert!(pnt2d_is_equal(c.end_point(), p75, 1e-9));
    }

    #[test]
    fn rational_circle_arc_radius() {
        let c = Geom2dBezierCurve::with_weights(
            &[
                Pnt2d::new(1.0, 0.0),
                Pnt2d::new(1.0, 1.0),
                Pnt2d::new(0.0, 1.0),
            ],
            &[1.0, 1.0 / 2.0_f64.sqrt(), 1.0],
        );
        assert!(c.is_rational());
        let m = c.value(0.5);
        assert!((m.norm() - 1.0).abs() < 1e-9);
    }
}
