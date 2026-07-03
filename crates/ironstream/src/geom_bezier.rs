//! `Geom_BezierCurve` — a rational or non-rational Bezier curve in 3D space, a
//! faithful reproduction of OpenCascade's `Geom_BezierCurve`
//! (`src/ModelingData/TKG3d/Geom/Geom_BezierCurve.hxx`).
//!
//! A non-rational Bezier curve is defined by a table of poles (control points);
//! a rational Bezier curve additionally carries a parallel table of weights.
//! The first pole is the start point of the curve and the last pole is the end
//! point. The degree of a Bezier curve is `NbPoles - 1`, must be `>= 1`, and is
//! limited to [`GeomBezierCurve::MAX_DEGREE`] (25). The parameter range is
//! `[0, 1]`. The continuity of a Bezier curve is infinite (`C-infinity`).
//!
//! The evaluation follows the standard Bernstein / de Casteljau formulation. For
//! rational curves the curve is evaluated through homogeneous coordinates
//! `(P_i * w_i, w_i)` and the rational derivatives are recovered with the
//! classic quotient recurrence.
//!
//! Builds on the existing `gp` API (zero third-party deps, pure std).

use crate::gp::{Pnt, Trsf, Vec3};

/// `gp::Resolution()` — the smallest weight value distinguishable from zero.
/// OCCT uses `1e-15`; two weights `W1`, `W2` are considered equal when
/// `|W2 - W1| <= gp::Resolution()`.
const GP_RESOLUTION: f64 = 1.0e-15;

/// `Geom_BezierCurve` — describes a rational or non-rational Bezier curve.
///
/// Mirrors OCCT: stores the poles, the (optional) weights, and the rational /
/// closed flags. Derived (conceptually) from `Geom_BoundedCurve` / `Geom_Curve`.
// occt: Geom_BezierCurve
#[derive(Clone, Debug)]
pub struct GeomBezierCurve {
    /// Control points (`myPoles`). Length is `NbPoles >= 2`.
    poles: Vec<Pnt>,
    /// Weights (`myWeights`). For a non-rational curve these are all `1.0`; the
    /// array is always sized to match the poles.
    weights: Vec<f64>,
    /// `myRational`: false when all weights are identical.
    rational: bool,
    /// `myClosed`: first and last poles coincide within `gp::Resolution()`.
    closed: bool,
}

impl GeomBezierCurve {
    /// `static Standard_Integer MaxDegree()` — the maximum polynomial degree of
    /// any `Geom_BezierCurve`. This value is 25.
    pub const MAX_DEGREE: i32 = 25;

    /// `static Standard_Integer MaxDegree()`.
    pub fn max_degree() -> i32 {
        Self::MAX_DEGREE
    }

    /// `Geom_BezierCurve(const NCollection_Array1<gp_Pnt>& CurvePoles)` — a
    /// non-rational Bezier curve. The weights default to all being `1`.
    ///
    /// # Panics
    /// Raises (Standard_ConstructionError) if the number of poles is greater
    /// than `MaxDegree + 1` or lower than `2`.
    pub fn new(poles: Vec<Pnt>) -> Self {
        let n = poles.len();
        assert!(
            n >= 2 && n as i32 <= Self::MAX_DEGREE + 1,
            "Geom_BezierCurve: number of poles must be in [2, MaxDegree+1]"
        );
        let weights = vec![1.0; n];
        let closed = poles[0].is_equal(poles[n - 1], GP_RESOLUTION);
        Self {
            poles,
            weights,
            rational: false,
            closed,
        }
    }

    /// `Geom_BezierCurve(const NCollection_Array1<gp_Pnt>& CurvePoles,
    /// const NCollection_Array1<double>& PoleWeights)` — a rational Bezier
    /// curve. If all the weights are identical the curve is considered as
    /// non-rational.
    ///
    /// # Panics
    /// Raises (Standard_ConstructionError) if the number of poles is greater
    /// than `MaxDegree + 1` or lower than `2`, or if `poles` and `weights` do
    /// not have the same length, or if one weight value is lower than or equal
    /// to `gp::Resolution()`.
    pub fn new_rational(poles: Vec<Pnt>, weights: Vec<f64>) -> Self {
        let n = poles.len();
        assert!(
            n >= 2 && n as i32 <= Self::MAX_DEGREE + 1,
            "Geom_BezierCurve: number of poles must be in [2, MaxDegree+1]"
        );
        assert!(
            poles.len() == weights.len(),
            "Geom_BezierCurve: poles and weights must have the same length"
        );
        for &w in &weights {
            assert!(
                w > GP_RESOLUTION,
                "Geom_BezierCurve: weight must be > gp::Resolution()"
            );
        }
        let rational = weights_are_rational(&weights);
        let closed = poles[0].is_equal(poles[n - 1], GP_RESOLUTION);
        Self {
            poles,
            weights,
            rational,
            closed,
        }
    }

    /// Copy constructor `Geom_BezierCurve(const Geom_BezierCurve&)` — optimized
    /// copy without validation.
    pub fn from_other(other: &GeomBezierCurve) -> Self {
        other.clone()
    }

    /// `Handle(Geom_Geometry) Copy() const` — a new curve which is a copy of
    /// this one. Uses the optimized copy constructor.
    pub fn copy(&self) -> GeomBezierCurve {
        Self::from_other(self)
    }

    /// `Standard_Integer NbPoles() const` — the number of poles.
    pub fn nb_poles(&self) -> i32 {
        self.poles.len() as i32
    }

    /// `Standard_Integer Degree() const` — the polynomial degree, `NbPoles - 1`.
    pub fn degree(&self) -> i32 {
        self.poles.len() as i32 - 1
    }

    /// `const gp_Pnt& Pole(const Standard_Integer Index) const` — the pole of
    /// range `Index` (1-based, OCCT convention).
    ///
    /// # Panics
    /// Raised if `index` is not in the range `[1, NbPoles]`.
    pub fn pole(&self, index: i32) -> Pnt {
        assert!(
            index >= 1 && index <= self.nb_poles(),
            "Geom_BezierCurve::Pole: index out of range [1, NbPoles]"
        );
        self.poles[(index - 1) as usize]
    }

    /// `const NCollection_Array1<gp_Pnt>& Poles() const` — all the poles.
    pub fn poles(&self) -> &[Pnt] {
        &self.poles
    }

    /// `double Weight(const Standard_Integer Index) const` — the weight of range
    /// `Index` (1-based).
    ///
    /// # Panics
    /// Raised if `index` is not in the range `[1, NbPoles]`.
    pub fn weight(&self, index: i32) -> f64 {
        assert!(
            index >= 1 && index <= self.nb_poles(),
            "Geom_BezierCurve::Weight: index out of range [1, NbPoles]"
        );
        self.weights[(index - 1) as usize]
    }

    /// `const NCollection_Array1<double>* Weights() const` — all the weights, or
    /// `None` for a non-rational curve (OCCT returns a null pointer).
    pub fn weights(&self) -> Option<&[f64]> {
        if self.rational {
            Some(&self.weights)
        } else {
            None
        }
    }

    /// `const NCollection_Array1<double>& WeightsArray() const` — a const view of
    /// the weights array, always sized to match `NbPoles`. For a non-rational
    /// curve this is a view of unit weights.
    pub fn weights_array(&self) -> &[f64] {
        &self.weights
    }

    /// `Standard_Boolean IsRational() const` — false when all weights are
    /// identical (criterion: `gp::Resolution()`).
    pub fn is_rational(&self) -> bool {
        self.rational
    }

    /// `Standard_Boolean IsClosed() const` — true if the distance between the
    /// first and last poles is `<= gp::Resolution()`.
    pub fn is_closed(&self) -> bool {
        self.closed
    }

    /// `Standard_Boolean IsPeriodic() const` — always false for a Bezier curve.
    pub fn is_periodic(&self) -> bool {
        false
    }

    /// `Standard_Boolean IsCN(const Standard_Integer N) const` — a Bezier curve
    /// is `C-infinity`, so this returns true for every `N >= 0`.
    pub fn is_cn(&self, _n: i32) -> bool {
        true
    }

    /// `GeomAbs_Shape Continuity() const` — a Bezier curve is `GeomAbs_CN`.
    pub fn continuity(&self) -> GeomAbsShape {
        GeomAbsShape::CN
    }

    /// `Standard_Real FirstParameter() const` — `0.0`.
    pub fn first_parameter(&self) -> f64 {
        0.0
    }

    /// `Standard_Real LastParameter() const` — `1.0`.
    pub fn last_parameter(&self) -> f64 {
        1.0
    }

    /// `gp_Pnt StartPoint() const` — `Value(0.0)`, the first control point.
    pub fn start_point(&self) -> Pnt {
        self.poles[0]
    }

    /// `gp_Pnt EndPoint() const` — `Value(1.0)`, the last control point.
    pub fn end_point(&self) -> Pnt {
        self.poles[self.poles.len() - 1]
    }

    /// `Standard_Real ReversedParameter(const Standard_Real U) const` — `1 - U`.
    pub fn reversed_parameter(&self, u: f64) -> f64 {
        1.0 - u
    }

    // ---------------------------------------------------------------------
    // Evaluation
    // ---------------------------------------------------------------------

    /// `gp_Pnt Value(const Standard_Real U) const` / `D0` — the point of
    /// parameter `U`. The Bezier curve has a polynomial representation, so `U`
    /// can be outside `[0, 1]`.
    pub fn value(&self, u: f64) -> Pnt {
        self.d0(u)
    }

    /// `void D0(const Standard_Real U, gp_Pnt& P) const`.
    pub fn d0(&self, u: f64) -> Pnt {
        // Homogeneous point A = sum B_i(u) * w_i * P_i ; w = sum B_i(u) * w_i.
        let (a, w) = self.homogeneous_derivatives(u, 0);
        let h = &a[0];
        let wv = w[0];
        Pnt::new(h.x / wv, h.y / wv, h.z / wv)
    }

    /// `void D1(const Standard_Real U, gp_Pnt& P, gp_Vec& V1) const`.
    pub fn d1(&self, u: f64) -> (Pnt, Vec3) {
        let d = self.rational_derivatives(u, 1);
        (d[0], d[1])
    }

    /// `void D2(const Standard_Real U, gp_Pnt& P, gp_Vec& V1, gp_Vec& V2) const`.
    pub fn d2(&self, u: f64) -> (Pnt, Vec3, Vec3) {
        let d = self.rational_derivatives(u, 2);
        (d[0], d[1], d[2])
    }

    /// `void D3(const Standard_Real U, gp_Pnt& P, gp_Vec& V1, gp_Vec& V2,
    /// gp_Vec& V3) const`.
    pub fn d3(&self, u: f64) -> (Pnt, Vec3, Vec3, Vec3) {
        let d = self.rational_derivatives(u, 3);
        (d[0], d[1], d[2], d[3])
    }

    /// `gp_Vec DN(const Standard_Real U, const Standard_Integer N) const` — the
    /// vector corresponding to the `N`-th derivative.
    ///
    /// # Panics
    /// Raised (Standard_RangeError) if `n < 1`.
    pub fn dn(&self, u: f64, n: i32) -> Vec3 {
        assert!(n >= 1, "Geom_BezierCurve::DN: N must be >= 1");
        let d = self.rational_derivatives(u, n);
        d[n as usize]
    }

    /// Homogeneous derivatives up to order `n`: returns `(A, W)` where
    /// `A[k]` is the `k`-th derivative of `sum B_i(u) * w_i * P_i` and `W[k]` is
    /// the `k`-th derivative of `sum B_i(u) * w_i`. Evaluated via repeated
    /// differencing of the de Casteljau / Bernstein control polygon, which is
    /// exact for polynomials.
    fn homogeneous_derivatives(&self, u: f64, n: i32) -> (Vec<Pnt>, Vec<f64>) {
        let p = self.degree(); // polynomial degree
        let nb = self.poles.len();

        // Homogeneous control points: ( w_i * P_i , w_i ).
        let mut hp: Vec<Pnt> = Vec::with_capacity(nb);
        let mut hw: Vec<f64> = Vec::with_capacity(nb);
        for i in 0..nb {
            let w = self.weights[i];
            hp.push(self.poles[i] * w);
            hw.push(w);
        }

        let order = n.min(p);
        let mut a_out: Vec<Pnt> = Vec::with_capacity((n + 1) as usize);
        let mut w_out: Vec<f64> = Vec::with_capacity((n + 1) as usize);

        // For each derivative order k, the k-th derivative of a degree-p Bezier
        // at parameter u equals  p!/(p-k)! * deCasteljau( forward_diff^k(poles) ).
        // We build the k-th forward differences and run de Casteljau on them with
        // degree p-k.
        for k in 0..=order {
            // k-th forward difference of the control polygon.
            let mut dp = hp.clone();
            let mut dw = hw.clone();
            for _ in 0..k {
                let m = dp.len();
                let mut np: Vec<Pnt> = Vec::with_capacity(m - 1);
                let mut nw: Vec<f64> = Vec::with_capacity(m - 1);
                for j in 0..m - 1 {
                    np.push(dp[j + 1] - dp[j]);
                    nw.push(dw[j + 1] - dw[j]);
                }
                dp = np;
                dw = nw;
            }
            // de Casteljau of degree (p - k) on the differenced polygon.
            let (ap, aw) = de_casteljau(&dp, &dw, u);
            // Scale factor p!/(p-k)! = p*(p-1)*...*(p-k+1).
            let mut scale = 1.0f64;
            for i in 0..k {
                scale *= (p - i) as f64;
            }
            a_out.push(ap * scale);
            w_out.push(aw * scale);
        }
        // Derivatives of order > p are zero.
        for _ in (order + 1)..=n {
            a_out.push(Pnt::origin());
            w_out.push(0.0);
        }
        (a_out, w_out)
    }

    /// Rational derivatives `[C, C', C'', ...]` up to order `n`, recovered from
    /// the homogeneous derivatives with the quotient recurrence
    /// `C^(k) = (A^(k) - sum_{i=1..k} binom(k,i) w^(i) C^(k-i)) / w`.
    fn rational_derivatives(&self, u: f64, n: i32) -> Vec<Pnt> {
        let (a, w) = self.homogeneous_derivatives(u, n);
        let w0 = w[0];
        let mut c: Vec<Pnt> = Vec::with_capacity((n + 1) as usize);
        for k in 0..=(n as usize) {
            let mut v = a[k];
            for i in 1..=k {
                let bin = binomial(k as i32, i as i32);
                v = v - c[k - i] * (bin * w[i]);
            }
            c.push(Pnt::new(v.x / w0, v.y / w0, v.z / w0));
        }
        c
    }

    // ---------------------------------------------------------------------
    // Modification
    // ---------------------------------------------------------------------

    /// `void SetPole(const Standard_Integer Index, const gp_Pnt& P)`.
    /// The weight of range `Index` is not modified.
    ///
    /// # Panics
    /// Raised if `index` is not in the range `[1, NbPoles]`.
    pub fn set_pole(&mut self, index: i32, p: Pnt) {
        assert!(
            index >= 1 && index <= self.nb_poles(),
            "Geom_BezierCurve::SetPole: index out of range [1, NbPoles]"
        );
        self.poles[(index - 1) as usize] = p;
        self.update_closed();
    }

    /// `void SetPole(const Standard_Integer Index, const gp_Pnt& P,
    /// const double Weight)` — substitutes the pole and weight of range `Index`.
    ///
    /// # Panics
    /// Raised if `index` is not in the range `[1, NbPoles]`, or if
    /// `weight <= gp::Resolution()`.
    pub fn set_pole_with_weight(&mut self, index: i32, p: Pnt, weight: f64) {
        assert!(
            index >= 1 && index <= self.nb_poles(),
            "Geom_BezierCurve::SetPole: index out of range [1, NbPoles]"
        );
        assert!(
            weight > GP_RESOLUTION,
            "Geom_BezierCurve::SetPole: weight must be > gp::Resolution()"
        );
        self.poles[(index - 1) as usize] = p;
        self.weights[(index - 1) as usize] = weight;
        self.rational = weights_are_rational(&self.weights);
        self.update_closed();
    }

    /// `void SetWeight(const Standard_Integer Index, const double Weight)`.
    ///
    /// # Panics
    /// Raised if `index` is not in the range `[1, NbPoles]`, or if
    /// `weight <= gp::Resolution()`.
    pub fn set_weight(&mut self, index: i32, weight: f64) {
        assert!(
            index >= 1 && index <= self.nb_poles(),
            "Geom_BezierCurve::SetWeight: index out of range [1, NbPoles]"
        );
        assert!(
            weight > GP_RESOLUTION,
            "Geom_BezierCurve::SetWeight: weight must be > gp::Resolution()"
        );
        self.weights[(index - 1) as usize] = weight;
        self.rational = weights_are_rational(&self.weights);
    }

    /// `void InsertPoleAfter(const Standard_Integer Index, const gp_Pnt& P)` —
    /// inserts a pole after the pole of range `Index`. If the curve is rational
    /// the new pole's weight is `1.0`.
    pub fn insert_pole_after(&mut self, index: i32, p: Pnt) {
        self.insert_pole_after_with_weight(index, p, 1.0);
    }

    /// `void InsertPoleAfter(const Standard_Integer Index, const gp_Pnt& P,
    /// const double Weight)`.
    ///
    /// # Panics
    /// Raised if `index` is not in the range `[1, NbPoles]`, if the resulting
    /// number of poles exceeds `MaxDegree + 1`, or if `weight <= gp::Resolution()`.
    pub fn insert_pole_after_with_weight(&mut self, index: i32, p: Pnt, weight: f64) {
        assert!(
            index >= 1 && index <= self.nb_poles(),
            "Geom_BezierCurve::InsertPoleAfter: index out of range [1, NbPoles]"
        );
        assert!(
            weight > GP_RESOLUTION,
            "Geom_BezierCurve::InsertPoleAfter: weight must be > gp::Resolution()"
        );
        assert!(
            self.nb_poles() < Self::MAX_DEGREE + 1,
            "Geom_BezierCurve::InsertPoleAfter: resulting poles exceed MaxDegree+1"
        );
        let at = index as usize; // insert after (1-based) Index => before 0-based `index`.
        self.poles.insert(at, p);
        self.weights.insert(at, weight);
        self.rational = weights_are_rational(&self.weights);
        self.update_closed();
    }

    /// `void InsertPoleBefore(const Standard_Integer Index, const gp_Pnt& P)`.
    pub fn insert_pole_before(&mut self, index: i32, p: Pnt) {
        self.insert_pole_before_with_weight(index, p, 1.0);
    }

    /// `void InsertPoleBefore(const Standard_Integer Index, const gp_Pnt& P,
    /// const double Weight)`.
    ///
    /// # Panics
    /// Raised if `index` is not in the range `[1, NbPoles]`, if the resulting
    /// number of poles exceeds `MaxDegree + 1`, or if `weight <= gp::Resolution()`.
    pub fn insert_pole_before_with_weight(&mut self, index: i32, p: Pnt, weight: f64) {
        assert!(
            index >= 1 && index <= self.nb_poles(),
            "Geom_BezierCurve::InsertPoleBefore: index out of range [1, NbPoles]"
        );
        assert!(
            weight > GP_RESOLUTION,
            "Geom_BezierCurve::InsertPoleBefore: weight must be > gp::Resolution()"
        );
        assert!(
            self.nb_poles() < Self::MAX_DEGREE + 1,
            "Geom_BezierCurve::InsertPoleBefore: resulting poles exceed MaxDegree+1"
        );
        let at = (index - 1) as usize; // insert before (1-based) Index.
        self.poles.insert(at, p);
        self.weights.insert(at, weight);
        self.rational = weights_are_rational(&self.weights);
        self.update_closed();
    }

    /// `void RemovePole(const Standard_Integer Index)`. If the curve was
    /// rational it can become non-rational.
    ///
    /// # Panics
    /// Raised if `index` is not in the range `[1, NbPoles]`, or if the resulting
    /// degree would be lower than `1` (poles below `2`).
    pub fn remove_pole(&mut self, index: i32) {
        assert!(
            index >= 1 && index <= self.nb_poles(),
            "Geom_BezierCurve::RemovePole: index out of range [1, NbPoles]"
        );
        assert!(
            self.nb_poles() > 2,
            "Geom_BezierCurve::RemovePole: degree would be lower than 1"
        );
        let at = (index - 1) as usize;
        self.poles.remove(at);
        self.weights.remove(at);
        self.rational = weights_are_rational(&self.weights);
        self.update_closed();
    }

    /// `void Reverse()` — reverses the direction of parametrization so that
    /// `Value(NewU) = Value(1 - OldU)`. Poles and weights are reversed.
    pub fn reverse(&mut self) {
        self.poles.reverse();
        self.weights.reverse();
        // Closed flag is symmetric, but recompute for consistency.
        self.update_closed();
    }

    /// `void Increase(const Standard_Integer Degree)` — degree elevation to the
    /// new `Degree` while keeping the curve geometrically identical.
    ///
    /// # Panics
    /// Raised if `degree` is greater than `MaxDegree`, lower than `2`, or lower
    /// than the initial degree of the curve.
    pub fn increase(&mut self, degree: i32) {
        assert!(
            degree <= Self::MAX_DEGREE,
            "Geom_BezierCurve::Increase: degree greater than MaxDegree"
        );
        let cur = self.degree();
        assert!(
            degree >= 2 && degree >= cur,
            "Geom_BezierCurve::Increase: degree lower than 2 or lower than current degree"
        );
        while self.degree() < degree {
            self.elevate_once();
        }
    }

    /// Elevates the degree by one using the standard Bezier degree-elevation
    /// formula on homogeneous control points.
    fn elevate_once(&mut self) {
        let p = self.degree(); // current degree
        let n = self.poles.len(); // current pole count = p + 1
        // Homogeneous poles.
        let hp: Vec<Pnt> = (0..n).map(|i| self.poles[i] * self.weights[i]).collect();
        let hw: Vec<f64> = self.weights.clone();

        let new_n = n + 1; // new pole count = p + 2
        let mut nhp: Vec<Pnt> = Vec::with_capacity(new_n);
        let mut nhw: Vec<f64> = Vec::with_capacity(new_n);
        let pf = (p + 1) as f64;
        for i in 0..new_n {
            let alpha = i as f64 / pf; // i/(p+1)
            if i == 0 {
                nhp.push(hp[0]);
                nhw.push(hw[0]);
            } else if i == new_n - 1 {
                nhp.push(hp[n - 1]);
                nhw.push(hw[n - 1]);
            } else {
                let q = hp[i - 1] * alpha + hp[i] * (1.0 - alpha);
                let qw = hw[i - 1] * alpha + hw[i] * (1.0 - alpha);
                nhp.push(q);
                nhw.push(qw);
            }
        }
        // Recover Cartesian poles and weights.
        let mut poles: Vec<Pnt> = Vec::with_capacity(new_n);
        let mut weights: Vec<f64> = Vec::with_capacity(new_n);
        for i in 0..new_n {
            let w = nhw[i];
            poles.push(Pnt::new(nhp[i].x / w, nhp[i].y / w, nhp[i].z / w));
            weights.push(w);
        }
        self.poles = poles;
        self.weights = weights;
        self.rational = weights_are_rational(&self.weights);
        self.update_closed();
    }

    /// `void Segment(const Standard_Real U1, const Standard_Real U2)` — restricts
    /// the curve to the parameter sub-range `[U1, U2]` (which may be outside the
    /// bounds), re-parametrizing the result onto `[0, 1]`. The poles are modified
    /// and the curve is oriented from `U1` to `U2`.
    pub fn segment(&mut self, u1: f64, u2: f64) {
        let n = self.poles.len();
        // Homogeneous poles.
        let mut hp: Vec<Pnt> = (0..n).map(|i| self.poles[i] * self.weights[i]).collect();
        let mut hw: Vec<f64> = self.weights.clone();

        // Re-parametrize the Bezier control polygon onto [u1, u2] by applying the
        // de Casteljau "blossom" reparametrization. We build the new poles as the
        // blossoms b(t_0,...) where each argument is interpolated between u1, u2.
        // Concretely: first trim left at u1, then right.
        // We implement the general (possibly out-of-range) reparametrization via
        // repeated linear interpolation of poles along the two endpoints.
        reparametrize(&mut hp, &mut hw, u1, u2);

        // Back to Cartesian.
        let mut poles: Vec<Pnt> = Vec::with_capacity(n);
        let mut weights: Vec<f64> = Vec::with_capacity(n);
        for i in 0..n {
            let w = hw[i];
            poles.push(Pnt::new(hp[i].x / w, hp[i].y / w, hp[i].z / w));
            weights.push(w);
        }
        self.poles = poles;
        self.weights = weights;
        self.rational = weights_are_rational(&self.weights);
        self.update_closed();
    }

    /// `void Transform(const gp_Trsf& T)` — applies the transformation `T` to
    /// each pole of this Bezier curve. Weights are unchanged.
    pub fn transform(&mut self, t: &Trsf) {
        for p in &mut self.poles {
            *p = t.apply_point(*p);
        }
        self.update_closed();
    }

    /// `void Resolution(const Standard_Real Tolerance3D, Standard_Real&
    /// UTolerance)` — the parametric tolerance such that
    /// `|t1 - t0| < UTolerance ==> |f(t1) - f(t0)| < Tolerance3D`.
    ///
    /// `UTolerance = Tolerance3D / max|f'(t)|`. The maximum of `|f'|` is bounded
    /// by `degree * max_i |P_{i+1} - P_i|` for the polynomial case, which is the
    /// classic Bezier derivative bound used by OCCT (`BSplCLib::Resolution`).
    pub fn resolution(&self, tolerance_3d: f64) -> f64 {
        let p = self.degree() as f64;
        let n = self.poles.len();
        // max distance between consecutive (homogeneous-normalized) poles.
        let mut max_step = 0.0f64;
        for i in 0..n - 1 {
            let d = self.poles[i + 1].distance(self.poles[i]);
            if d > max_step {
                max_step = d;
            }
        }
        let max_deriv = p * max_step;
        if max_deriv <= GP_RESOLUTION {
            // Degenerate (all poles coincide): any parametric step is fine.
            f64::INFINITY
        } else {
            tolerance_3d / max_deriv
        }
    }

    /// Recompute the `myClosed` flag.
    fn update_closed(&mut self) {
        let n = self.poles.len();
        self.closed = self.poles[0].is_equal(self.poles[n - 1], GP_RESOLUTION);
    }
}

/// `GeomAbs_Shape` — the continuity classification (only `CN` is produced by a
/// Bezier curve, but the full set mirrors OCCT for faithful comparisons).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GeomAbsShape {
    /// `GeomAbs_C0`.
    C0,
    /// `GeomAbs_G1`.
    G1,
    /// `GeomAbs_C1`.
    C1,
    /// `GeomAbs_G2`.
    G2,
    /// `GeomAbs_C2`.
    C2,
    /// `GeomAbs_C3`.
    C3,
    /// `GeomAbs_CN`.
    CN,
}

/// Returns true when the weights are *not* all identical (rational), using the
/// `gp::Resolution()` criterion on `|w_i - w_0|`.
fn weights_are_rational(weights: &[f64]) -> bool {
    if weights.is_empty() {
        return false;
    }
    let w0 = weights[0];
    weights.iter().any(|&w| (w - w0).abs() > GP_RESOLUTION)
}

/// de Casteljau evaluation of a (homogeneous) Bezier control polygon at `u`.
/// Returns the interpolated `(point, weight)`.
fn de_casteljau(poles: &[Pnt], weights: &[f64], u: f64) -> (Pnt, f64) {
    let mut p = poles.to_vec();
    let mut w = weights.to_vec();
    let n = p.len();
    if n == 0 {
        return (Pnt::origin(), 0.0);
    }
    let t = u;
    let mt = 1.0 - u;
    for r in 1..n {
        for i in 0..n - r {
            p[i] = p[i] * mt + p[i + 1] * t;
            w[i] = w[i] * mt + w[i + 1] * t;
        }
    }
    (p[0], w[0])
}

/// Binomial coefficient `C(n, k)` as an `f64` (exact for the small `n` used
/// here, `n <= MaxDegree`).
fn binomial(n: i32, k: i32) -> f64 {
    if k < 0 || k > n {
        return 0.0;
    }
    let k = k.min(n - k);
    let mut result = 1.0f64;
    for i in 0..k {
        result = result * (n - i) as f64 / (i + 1) as f64;
    }
    result
}

/// In-place reparametrization of a Bezier control polygon (homogeneous poles
/// `hp`, weights `hw`) so that the new curve over `[0,1]` traces the original
/// over `[u1, u2]`.
///
/// This is the de Casteljau "trim + extend" reparametrization: a Bezier of
/// degree `p` over `[u1, u2]` has poles equal to the blossom values
/// `b(u1^{p-i}, u2^{i})`. We compute those blossoms by, for each new pole index
/// `i`, running de Casteljau with `i` steps at `u2` followed by `p - i` steps at
/// `u1` on the leading edge of the triangle — but to handle out-of-range `u1`,
/// `u2`, we use the symmetric blossom evaluation below.
fn reparametrize(hp: &mut Vec<Pnt>, hw: &mut Vec<f64>, u1: f64, u2: f64) {
    let p = hp.len() - 1; // degree
                          // The blossom b of a Bezier with control points c_0..c_p satisfies
                          // c_i = b(0^{p-i}, 1^{i}). The new control points are
                          // d_i = b(u1^{p-i}, u2^{i}).
                          // We evaluate the multi-affine blossom by repeated linear interpolation.
    let cp = hp.clone();
    let cw = hw.clone();
    for i in 0..=p {
        // arguments: (p - i) copies of u1, then i copies of u2.
        let mut args: Vec<f64> = Vec::with_capacity(p);
        for _ in 0..(p - i) {
            args.push(u1);
        }
        for _ in 0..i {
            args.push(u2);
        }
        let (bp, bw) = blossom(&cp, &cw, &args);
        hp[i] = bp;
        hw[i] = bw;
    }
}

/// Evaluates the blossom (polar form) of a Bezier control polygon at the given
/// `args` (which must number `degree`). Each argument advances one de Casteljau
/// level using its own interpolation parameter.
fn blossom(poles: &[Pnt], weights: &[f64], args: &[f64]) -> (Pnt, f64) {
    let mut p = poles.to_vec();
    let mut w = weights.to_vec();
    let n = p.len();
    debug_assert_eq!(args.len(), n - 1);
    for (level, &t) in args.iter().enumerate() {
        let mt = 1.0 - t;
        let len = n - level; // current number of points
        for i in 0..len - 1 {
            p[i] = p[i] * mt + p[i + 1] * t;
            w[i] = w[i] * mt + w[i + 1] * t;
        }
    }
    (p[0], w[0])
}
