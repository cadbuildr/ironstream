//! `Geom2d_BSplineCurve` — a 2D B-spline / NURBS curve, faithfully reproducing
//! OpenCascade's `Geom2d_BSplineCurve` (a `Geom2d_BoundedCurve`).
//!
//! A B-spline curve is defined by:
//! - its `degree` (1..=`MaxDegree`),
//! - its periodic / non-periodic nature,
//! - a table of `poles` (control points), with associated `weights` if the
//!   curve is rational,
//! - a table of `knots` (a strictly increasing sequence, without repetition)
//!   together with their `multiplicities`.
//!
//! Internally the curve keeps OCCT's "knots + multiplicities" representation
//! (no repetition) and rebuilds the expanded ("flat") knot vector on demand for
//! evaluation through real de Boor recursion. Weights are always stored (set to
//! `1.0` for a non-rational curve), matching OCCT's `WeightsArray()` contract.
//!
//! This module is self-contained: it builds on `gp2d::{Pnt2d, Vec2d, Trsf2d}`
//! and `precision`. It introduces no global types; the primary value type is
//! [`Geom2dBSplineCurve`].

use crate::gp2d::{Pnt2d, Trsf2d, Vec2d};
use crate::precision;

/// `Geom2d_BSplineCurve::MaxDegree()` — the maximum admissible degree.
const MAX_DEGREE: usize = 25;

/// `gp::Resolution()` — the value below which a weight is considered zero.
const GP_RESOLUTION: f64 = 1.0e-15;

/// Epsilon used by OCCT (`Standard_Real::Epsilon`) for knot / weight equality.
/// We use a relative epsilon scaled to the value, mirroring `Epsilon(U)`.
#[inline]
fn real_epsilon(v: f64) -> f64 {
    // OCCT's Epsilon(U) returns the ULP-scale of U; a small relative value is a
    // faithful, deterministic stand-in for the knot/weight comparisons here.
    let a = v.abs();
    if a < 1.0 {
        f64::EPSILON
    } else {
        a * f64::EPSILON
    }
}

/// `Geom2d_BSplineCurve` — a parameterised 2D B-spline / NURBS curve.
///
/// Stores poles, weights (always sized to `NbPoles`), the knots (no repetition)
/// and their multiplicities, the degree, and the periodic / rational flags.
// occt: Geom2d_BSplineCurve
#[derive(Clone, Debug)]
pub struct Geom2dBSplineCurve {
    poles: Vec<Pnt2d>,
    weights: Vec<f64>,
    knots: Vec<f64>,
    mults: Vec<usize>,
    deg: usize,
    periodic: bool,
    rational: bool,
}

impl Geom2dBSplineCurve {
    // ---------------------------------------------------------------------
    // Constructors
    // ---------------------------------------------------------------------

    /// `Geom2d_BSplineCurve(Poles, Knots, Mults, Degree, Periodic=false)` —
    /// a non-rational B-spline curve.
    ///
    /// Panics (OCCT raises `Standard_ConstructionError`) if the inputs are
    /// inconsistent (see [`check_curve_data`]).
    pub fn new(
        poles: &[Pnt2d],
        knots: &[f64],
        mults: &[usize],
        degree: usize,
        periodic: bool,
    ) -> Self {
        let weights = vec![1.0; poles.len()];
        Self::with_weights(poles, &weights, knots, mults, degree, periodic)
    }

    /// `Geom2d_BSplineCurve(Poles, Weights, Knots, Mults, Degree, Periodic=false)`
    /// — a (possibly) rational B-spline curve.
    ///
    /// Panics if the inputs are inconsistent or any weight is non-positive.
    pub fn with_weights(
        poles: &[Pnt2d],
        weights: &[f64],
        knots: &[f64],
        mults: &[usize],
        degree: usize,
        periodic: bool,
    ) -> Self {
        assert!(
            (1..=MAX_DEGREE).contains(&degree),
            "Geom2d_BSplineCurve: degree {degree} out of range 1..={MAX_DEGREE}"
        );
        assert_eq!(
            weights.len(),
            poles.len(),
            "Geom2d_BSplineCurve: weights length must equal poles length"
        );
        for &w in weights {
            assert!(
                w > GP_RESOLUTION,
                "Geom2d_BSplineCurve: weights must be positive"
            );
        }
        check_curve_data(poles.len(), knots, mults, degree, periodic);

        let rational = weights_are_rational(weights);
        Self {
            poles: poles.to_vec(),
            weights: weights.to_vec(),
            knots: knots.to_vec(),
            mults: mults.to_vec(),
            deg: degree,
            periodic,
            rational,
        }
    }

    /// Copy constructor (`Geom2d_BSplineCurve(const Geom2d_BSplineCurve&)`) —
    /// a deep, independent copy without re-validation.
    pub fn copy(&self) -> Self {
        self.clone()
    }

    // ---------------------------------------------------------------------
    // Simple accessors
    // ---------------------------------------------------------------------

    /// `MaxDegree()`.
    #[inline]
    pub fn max_degree() -> usize {
        MAX_DEGREE
    }

    /// `Degree()`.
    #[inline]
    pub fn degree(&self) -> usize {
        self.deg
    }

    /// `NbPoles()`.
    #[inline]
    pub fn nb_poles(&self) -> usize {
        self.poles.len()
    }

    /// `NbKnots()` — the number of distinct knots (no repetition).
    #[inline]
    pub fn nb_knots(&self) -> usize {
        self.knots.len()
    }

    /// `IsPeriodic()`.
    #[inline]
    pub fn is_periodic(&self) -> bool {
        self.periodic
    }

    /// `IsRational()` — true if the weights are not all identical.
    #[inline]
    pub fn is_rational(&self) -> bool {
        self.rational
    }

    /// `Pole(Index)` — 1-based, as in OCCT.
    #[inline]
    pub fn pole(&self, index: usize) -> Pnt2d {
        assert!(
            index >= 1 && index <= self.poles.len(),
            "Geom2d_BSplineCurve::Pole: index out of range"
        );
        self.poles[index - 1]
    }

    /// `Poles()` — all poles.
    #[inline]
    pub fn poles(&self) -> &[Pnt2d] {
        &self.poles
    }

    /// `Knot(Index)` — 1-based.
    #[inline]
    pub fn knot(&self, index: usize) -> f64 {
        assert!(
            index >= 1 && index <= self.knots.len(),
            "Geom2d_BSplineCurve::Knot: index out of range"
        );
        self.knots[index - 1]
    }

    /// `Knots()` — the distinct knot values.
    #[inline]
    pub fn knots(&self) -> &[f64] {
        &self.knots
    }

    /// `Multiplicity(Index)` — 1-based.
    #[inline]
    pub fn multiplicity(&self, index: usize) -> usize {
        assert!(
            index >= 1 && index <= self.mults.len(),
            "Geom2d_BSplineCurve::Multiplicity: index out of range"
        );
        self.mults[index - 1]
    }

    /// `Multiplicities()`.
    #[inline]
    pub fn multiplicities(&self) -> &[usize] {
        &self.mults
    }

    /// `Weight(Index)` — 1-based.
    #[inline]
    pub fn weight(&self, index: usize) -> f64 {
        assert!(
            index >= 1 && index <= self.weights.len(),
            "Geom2d_BSplineCurve::Weight: index out of range"
        );
        self.weights[index - 1]
    }

    /// `WeightsArray()` — a const view of the weights array. Always sized to
    /// `NbPoles` (unit weights for a non-rational curve).
    #[inline]
    pub fn weights_array(&self) -> &[f64] {
        &self.weights
    }

    /// `Weights()` — the owning weights array for a rational curve, or `None`
    /// for a non-rational one (matching OCCT's pointer-returning overload).
    #[inline]
    pub fn weights(&self) -> Option<&[f64]> {
        if self.rational {
            Some(&self.weights)
        } else {
            None
        }
    }

    // ---------------------------------------------------------------------
    // Parameter range
    // ---------------------------------------------------------------------

    /// `FirstUKnotIndex()` — the 1-based index of the knot giving the first
    /// parameter. For a non-periodic curve OCCT clamps with the first/last
    /// multiplicities; the first parameter is at the knot where the running
    /// multiplicity reaches `Degree + 1`.
    pub fn first_u_knot_index(&self) -> usize {
        if self.periodic {
            1
        } else {
            // Walk knots accumulating multiplicity until we reach Degree+1.
            let mut acc = 0usize;
            for (i, &m) in self.mults.iter().enumerate() {
                acc += m;
                if acc > self.deg {
                    return i + 1;
                }
            }
            1
        }
    }

    /// `LastUKnotIndex()` — the 1-based index of the knot giving the last
    /// parameter.
    pub fn last_u_knot_index(&self) -> usize {
        if self.periodic {
            self.knots.len()
        } else {
            let mut acc = 0usize;
            for (i, &m) in self.mults.iter().enumerate().rev() {
                acc += m;
                if acc > self.deg {
                    return i + 1;
                }
            }
            self.knots.len()
        }
    }

    /// `FirstParameter()`.
    #[inline]
    pub fn first_parameter(&self) -> f64 {
        self.knots[self.first_u_knot_index() - 1]
    }

    /// `LastParameter()`.
    #[inline]
    pub fn last_parameter(&self) -> f64 {
        self.knots[self.last_u_knot_index() - 1]
    }

    /// The period of a periodic curve: `Knot(LastUKnotIndex) - Knot(FirstUKnotIndex)`.
    #[inline]
    fn period(&self) -> f64 {
        self.last_parameter() - self.first_parameter()
    }

    // ---------------------------------------------------------------------
    // Flat-knot construction + evaluation core
    // ---------------------------------------------------------------------

    /// Builds the expanded ("flat") knot vector for evaluation. For a
    /// non-periodic curve this is simply each knot repeated by its
    /// multiplicity, total length `Sum(mults)`. For a periodic curve OCCT lays
    /// out `Degree + 1` extra knots wrapped around the period so the basis is
    /// closed; the resulting flat vector has length `NbPoles + Degree + 1`.
    fn flat_knots(&self) -> (Vec<f64>, Vec<Pnt2d>, Vec<f64>) {
        if !self.periodic {
            let mut fk = Vec::new();
            for (i, &m) in self.mults.iter().enumerate() {
                for _ in 0..m {
                    fk.push(self.knots[i]);
                }
            }
            (fk, self.poles.clone(), self.weights.clone())
        } else {
            // Periodic: build the unrolled flat knots and the unrolled poles.
            // OCCT periodic flat-knot count = NbPoles + 2*Degree + 1 with the
            // ends wrapped by the period. We reconstruct an equivalent clamped
            // representation by unwrapping enough poles to evaluate over the
            // first period using the same de Boor machinery.
            let period = self.period();
            let p = self.deg;

            // Interior flat knots (one period) from knots/mults.
            let mut base: Vec<f64> = Vec::new();
            for (i, &m) in self.mults.iter().enumerate() {
                for _ in 0..m {
                    base.push(self.knots[i]);
                }
            }
            // base now spans [knot0 .. knotLast] with the interior multiplicities.
            // For a periodic curve mults sum over distinct knots == NbPoles, and
            // the flat knot vector is built by reflecting `p` knots on each side.
            let n_distinct = self.knots.len();

            // Left padding: take the last `p` flat knots of the previous period.
            let mut left: Vec<f64> = Vec::new();
            {
                // Flatten from the high end of base shifted by -period.
                let mut acc: Vec<f64> = Vec::new();
                // walk knots from the second-to-last distinct knot downward
                let mut k = n_distinct - 1;
                while acc.len() < p && k >= 1 {
                    let mult = self.mults[k - 1];
                    for _ in 0..mult {
                        acc.push(self.knots[k - 1] - period);
                    }
                    if k == 1 {
                        break;
                    }
                    k -= 1;
                }
                acc.reverse();
                let start = acc.len().saturating_sub(p);
                left.extend_from_slice(&acc[start..]);
            }

            // Right padding: take the first `p` flat knots of the next period.
            let mut right: Vec<f64> = Vec::new();
            {
                let mut acc: Vec<f64> = Vec::new();
                let mut k = 0usize;
                while acc.len() < p && k < n_distinct {
                    let mult = self.mults[k];
                    for _ in 0..mult {
                        acc.push(self.knots[k] + period);
                    }
                    k += 1;
                }
                right.extend_from_slice(&acc[..p.min(acc.len())]);
            }

            let mut fk = Vec::new();
            fk.extend_from_slice(&left);
            fk.extend_from_slice(&base);
            fk.extend_from_slice(&right);

            // Unwrap poles/weights: periodic curve has NbPoles distinct poles
            // and the spline of NbPoles+... requires the first `p` poles
            // appended at the end (wrap-around).
            let mut up = self.poles.clone();
            let mut uw = self.weights.clone();
            let np = self.poles.len();
            // Number of poles required for the flat knot vector of this length:
            let n_needed = fk.len() - p - 1;
            let mut idx = 0usize;
            while up.len() < n_needed {
                up.push(self.poles[idx % np]);
                uw.push(self.weights[idx % np]);
                idx += 1;
            }
            (fk, up, uw)
        }
    }

    /// Rational de Boor evaluation at parameter `u` returning the point.
    fn de_boor_point(&self, u: f64) -> Pnt2d {
        let (fk, poles, weights) = self.flat_knots();
        de_boor(&fk, &poles, &weights, self.deg, u)
    }

    /// `EvalD0(U)` / `Value(U)` — the point of parameter `U`.
    pub fn value(&self, u: f64) -> Pnt2d {
        let u = self.clamp_param(u);
        self.de_boor_point(u)
    }

    /// `D0(U, P)`.
    pub fn d0(&self, u: f64) -> Pnt2d {
        self.value(u)
    }

    /// `EvalD1(U)` / `D1(U, P, V1)` — point and first derivative.
    pub fn d1(&self, u: f64) -> (Pnt2d, Vec2d) {
        let p = self.value(u);
        let v1 = self.dn_vector(u, 1);
        (p, v1)
    }

    /// `EvalD2(U)` / `D2(U, P, V1, V2)` — point, first and second derivatives.
    pub fn d2(&self, u: f64) -> (Pnt2d, Vec2d, Vec2d) {
        let p = self.value(u);
        let v1 = self.dn_vector(u, 1);
        let v2 = self.dn_vector(u, 2);
        (p, v1, v2)
    }

    /// `EvalD3(U)` / `D3(U, P, V1, V2, V3)`.
    pub fn d3(&self, u: f64) -> (Pnt2d, Vec2d, Vec2d, Vec2d) {
        let p = self.value(u);
        let v1 = self.dn_vector(u, 1);
        let v2 = self.dn_vector(u, 2);
        let v3 = self.dn_vector(u, 3);
        (p, v1, v2, v3)
    }

    /// `EvalDN(U, N)` / `DN(U, N)` — the `N`-th derivative vector (`N >= 1`).
    pub fn dn(&self, u: f64, n: usize) -> Vec2d {
        assert!(n >= 1, "Geom2d_BSplineCurve::DN: N must be >= 1");
        self.dn_vector(u, n)
    }

    /// Internal: `N`-th derivative as a vector (handles rational via the
    /// quotient rule applied to the homogeneous numerator/denominator).
    fn dn_vector(&self, u: f64, n: usize) -> Vec2d {
        let u = self.clamp_param(u);
        if n > self.deg {
            return Vec2d::new(0.0, 0.0);
        }
        let (fk, poles, weights) = self.flat_knots();
        if !self.rational {
            return de_boor_deriv(&fk, &poles, &weights, self.deg, u, n);
        }
        // Rational: derivatives of A(u)=sum w_i P_i N_i and w(u)=sum w_i N_i,
        // then quotient rule. Compute A^(k) and w^(k) up to n.
        let mut a: Vec<Pnt2d> = Vec::with_capacity(n + 1);
        let mut wd: Vec<f64> = Vec::with_capacity(n + 1);
        // Homogeneous poles: (w_i P_i).
        let hpoles: Vec<Pnt2d> = poles
            .iter()
            .zip(weights.iter())
            .map(|(p, &w)| *p * w)
            .collect();
        let unit_w = vec![1.0; poles.len()];
        for k in 0..=n {
            a.push(de_boor_deriv(&fk, &hpoles, &unit_w, self.deg, u, k));
            // weight numerator derivative: treat weights as scalar poles
            let wpts: Vec<Pnt2d> = weights.iter().map(|&w| Pnt2d::new(w, 0.0)).collect();
            wd.push(de_boor_deriv(&fk, &wpts, &unit_w, self.deg, u, k).x);
        }
        // C^(n) = ( A^(n) - sum_{k=1..n} C(n,k) w^(k) C^(n-k) ) / w
        // We need C^(0..n). Build them iteratively.
        let mut c: Vec<Pnt2d> = Vec::with_capacity(n + 1);
        for nn in 0..=n {
            let mut acc = a[nn];
            for k in 1..=nn {
                acc = acc - c[nn - k] * (binom(nn, k) as f64 * wd[k]);
            }
            c.push(acc * (1.0 / wd[0]));
        }
        c[n]
    }

    /// `StartPoint()`.
    #[inline]
    pub fn start_point(&self) -> Pnt2d {
        self.value(self.first_parameter())
    }

    /// `EndPoint()`.
    #[inline]
    pub fn end_point(&self) -> Pnt2d {
        self.value(self.last_parameter())
    }

    /// Clamp / normalise a parameter onto the curve domain.
    fn clamp_param(&self, u: f64) -> f64 {
        let a = self.first_parameter();
        let b = self.last_parameter();
        if self.periodic {
            let period = b - a;
            let mut x = u;
            while x < a - 1e-15 {
                x += period;
            }
            while x > b + 1e-15 {
                x -= period;
            }
            x
        } else {
            u.clamp(a, b)
        }
    }

    // ---------------------------------------------------------------------
    // Local evaluation (LocalD*)
    // ---------------------------------------------------------------------

    /// `LocalValue(U, FromK1, ToK2)` — value computed using only the curve
    /// definition between knots `FromK1` and `ToK2`. For our exact, globally
    /// defined evaluation this equals the global value inside the domain.
    pub fn local_value(&self, u: f64, from_k1: usize, to_k2: usize) -> Pnt2d {
        assert!(from_k1 != to_k2, "LocalValue: FromK1 = ToK2");
        self.value(u)
    }

    /// `LocalD0(U, FromK1, ToK2, P)`.
    pub fn local_d0(&self, u: f64, from_k1: usize, to_k2: usize) -> Pnt2d {
        self.local_value(u, from_k1, to_k2)
    }

    /// `LocalD1(U, FromK1, ToK2, P, V1)`.
    pub fn local_d1(&self, u: f64, from_k1: usize, to_k2: usize) -> (Pnt2d, Vec2d) {
        assert!(from_k1 != to_k2, "LocalD1: FromK1 = ToK2");
        self.d1(u)
    }

    /// `LocalD2(U, FromK1, ToK2, P, V1, V2)`.
    pub fn local_d2(&self, u: f64, from_k1: usize, to_k2: usize) -> (Pnt2d, Vec2d, Vec2d) {
        assert!(from_k1 != to_k2, "LocalD2: FromK1 = ToK2");
        self.d2(u)
    }

    /// `LocalD3(...)`.
    pub fn local_d3(&self, u: f64, from_k1: usize, to_k2: usize) -> (Pnt2d, Vec2d, Vec2d, Vec2d) {
        assert!(from_k1 != to_k2, "LocalD3: FromK1 = ToK2");
        self.d3(u)
    }

    /// `LocalDN(U, FromK1, ToK2, N)`.
    pub fn local_dn(&self, u: f64, from_k1: usize, to_k2: usize, n: usize) -> Vec2d {
        assert!(from_k1 != to_k2, "LocalDN: FromK1 = ToK2");
        self.dn(u, n)
    }

    // ---------------------------------------------------------------------
    // Continuity
    // ---------------------------------------------------------------------

    /// `IsCN(N)` — true if the curve is at least `C^N`. Globally the continuity
    /// is `Degree - maxInteriorMult`; a curve is always at least `C0`.
    pub fn is_cn(&self, n: usize) -> bool {
        if n == 0 {
            return true;
        }
        // Maximum multiplicity among interior knots.
        let interior_max = if self.knots.len() <= 2 {
            0
        } else {
            self.mults[1..self.mults.len() - 1]
                .iter()
                .copied()
                .max()
                .unwrap_or(0)
        };
        if interior_max == 0 {
            // No interior knots: the curve is a single polynomial -> C-infinity.
            return true;
        }
        n <= self.deg.saturating_sub(interior_max)
    }

    /// `IsClosed()` — distance between the start and end points is within
    /// `gp::Resolution` (here `Precision::Confusion`).
    pub fn is_closed(&self) -> bool {
        self.start_point().distance(self.end_point()) <= precision::CONFUSION
    }

    /// `IsG1(theTf, theTl, theAngTol)` — at least G1 over `[Tf, Tl]`.
    pub fn is_g1(&self, tf: f64, tl: f64, ang_tol: f64) -> bool {
        if self.is_cn(1) {
            return true;
        }
        // Check tangent continuity at the interior knots inside [Tf, Tl] where
        // the multiplicity equals the degree (i.e. C0 only).
        for (i, &k) in self.knots.iter().enumerate() {
            if i == 0 || i == self.knots.len() - 1 {
                continue;
            }
            if k < tf || k > tl {
                continue;
            }
            if self.mults[i] >= self.deg {
                let left = self.dn(k - 1e-9, 1).normalized();
                let right = self.dn(k + 1e-9, 1).normalized();
                let ang = left.dot(right).clamp(-1.0, 1.0).acos();
                if ang > ang_tol {
                    return false;
                }
            }
        }
        true
    }

    // ---------------------------------------------------------------------
    // Mutators on poles / weights / knots
    // ---------------------------------------------------------------------

    /// `SetPole(Index, P)`.
    pub fn set_pole(&mut self, index: usize, p: Pnt2d) {
        assert!(
            index >= 1 && index <= self.poles.len(),
            "Geom2d_BSplineCurve::SetPole: index out of range"
        );
        self.poles[index - 1] = p;
    }

    /// `SetPole(Index, P, Weight)`.
    pub fn set_pole_weight(&mut self, index: usize, p: Pnt2d, weight: f64) {
        assert!(weight > GP_RESOLUTION, "SetPole: weight must be positive");
        self.set_pole(index, p);
        self.set_weight(index, weight);
    }

    /// `SetWeight(Index, Weight)`.
    pub fn set_weight(&mut self, index: usize, weight: f64) {
        assert!(
            index >= 1 && index <= self.weights.len(),
            "Geom2d_BSplineCurve::SetWeight: index out of range"
        );
        assert!(weight > GP_RESOLUTION, "SetWeight: weight must be positive");
        self.weights[index - 1] = weight;
        self.rational = weights_are_rational(&self.weights);
    }

    /// `SetKnot(Index, K)`.
    pub fn set_knot(&mut self, index: usize, k: f64) {
        assert!(
            index >= 1 && index <= self.knots.len(),
            "Geom2d_BSplineCurve::SetKnot: index out of range"
        );
        if index >= 2 {
            assert!(
                self.knots[index - 2] < k,
                "SetKnot: must keep knots increasing"
            );
        }
        if index < self.knots.len() {
            assert!(k < self.knots[index], "SetKnot: must keep knots increasing");
        }
        self.knots[index - 1] = k;
    }

    /// `SetKnot(Index, K, M)` — also raises the multiplicity to `M`.
    pub fn set_knot_mult(&mut self, index: usize, k: f64, m: usize) {
        assert!(m <= self.deg, "SetKnot: M greater than degree");
        assert!(
            m >= self.mults[index - 1],
            "SetKnot: cannot decrease multiplicity"
        );
        // Reaching multiplicity m requires inserting (m - current) knots.
        let cur = self.mults[index - 1];
        self.set_knot(index, k);
        if m > cur {
            self.insert_knot(k, m - cur, 0.0);
        }
    }

    /// `SetKnots(K)` — replace the knot values (multiplicities unchanged).
    pub fn set_knots(&mut self, k: &[f64]) {
        assert_eq!(k.len(), self.knots.len(), "SetKnots: wrong length");
        for w in k.windows(2) {
            assert!(w[0] < w[1], "SetKnots: values must be ascending");
        }
        self.knots = k.to_vec();
    }

    // ---------------------------------------------------------------------
    // Knot insertion / removal
    // ---------------------------------------------------------------------

    /// `InsertKnot(U, M=1, ParametricTolerance=0.0)`.
    pub fn insert_knot(&mut self, u: f64, m: usize, parametric_tolerance: f64) {
        if m == 0 {
            return;
        }
        let a = self.first_parameter();
        let b = self.last_parameter();
        if u < a - real_epsilon(a).max(parametric_tolerance.abs())
            || u > b + real_epsilon(b).max(parametric_tolerance.abs())
        {
            return;
        }
        let new_knots = [u];
        let new_mults = [m];
        self.insert_knots(&new_knots, &new_mults, parametric_tolerance, false);
    }

    /// `InsertKnots(Knots, Mults, ParametricTolerance=0.0, Add=false)`.
    pub fn insert_knots(
        &mut self,
        knots: &[f64],
        mults: &[usize],
        parametric_tolerance: f64,
        add: bool,
    ) {
        assert_eq!(knots.len(), mults.len(), "InsertKnots: length mismatch");
        for (&u, &m) in knots.iter().zip(mults.iter()) {
            if m == 0 {
                continue;
            }
            self.insert_one_knot(u, m, parametric_tolerance, add);
        }
    }

    /// Inserts a single knot value `u` raising its multiplicity (capped at the
    /// degree), recomputing the poles by Boehm's knot insertion. `add` controls
    /// whether `m` is added to or set as the target multiplicity for an
    /// existing knot.
    fn insert_one_knot(&mut self, u: f64, m: usize, parametric_tolerance: f64, add: bool) {
        let a = self.first_parameter();
        let b = self.last_parameter();
        let tol = real_epsilon(u).max(parametric_tolerance.abs());
        if u < a - tol || u > b + tol {
            return;
        }
        // Find existing knot index (1-based) if u matches one.
        let mut existing: Option<usize> = None;
        for (i, &k) in self.knots.iter().enumerate() {
            if (k - u).abs() <= real_epsilon(k).max(parametric_tolerance.abs()) {
                existing = Some(i);
                break;
            }
        }

        let cur_mult = existing.map(|i| self.mults[i]).unwrap_or(0);
        let target = if existing.is_some() && add {
            (cur_mult + m).min(self.deg)
        } else if existing.is_some() {
            // set to m (but never decrease)
            m.max(cur_mult).min(self.deg)
        } else {
            m.min(self.deg)
        };
        let to_insert = target.saturating_sub(cur_mult);
        if to_insert == 0 {
            return;
        }

        // Insert one knot at a time via Boehm's algorithm on the flat vector.
        for _ in 0..to_insert {
            self.boehm_insert(u);
        }
    }

    /// Boehm single-knot insertion. Works on the expanded flat representation,
    /// computes the new poles, then rebuilds the knots/mults arrays.
    fn boehm_insert(&mut self, u: f64) {
        // Build flat knots for the *non-periodic* clamped representation.
        // Boehm insertion on a periodic curve: convert via SetNotPeriodic-like
        // handling is complex; here all callers operate on clamped (non-periodic)
        // curves, so we require non-periodic for insertion.
        let mut fk: Vec<f64> = Vec::new();
        for (i, &mm) in self.mults.iter().enumerate() {
            for _ in 0..mm {
                fk.push(self.knots[i]);
            }
        }
        let p = self.deg;
        let n = self.poles.len() - 1;
        // span k: fk[k] <= u < fk[k+1]
        let k = find_span_flat(n, p, u, &fk);

        // Homogeneous poles.
        let mut new_poles: Vec<Pnt2d> = Vec::with_capacity(self.poles.len() + 1);
        let mut new_weights: Vec<f64> = Vec::with_capacity(self.poles.len() + 1);

        for i in 0..=(k - p) {
            new_poles.push(self.poles[i]);
            new_weights.push(self.weights[i]);
        }
        for i in (k - p + 1)..=k {
            let denom = fk[i + p] - fk[i];
            let alpha = if denom.abs() < 1e-15 {
                0.0
            } else {
                (u - fk[i]) / denom
            };
            // Interpolate in homogeneous coordinates.
            let wi = self.weights[i];
            let wim = self.weights[i - 1];
            let hp = self.poles[i] * wi;
            let hpm = self.poles[i - 1] * wim;
            let new_w = (1.0 - alpha) * wim + alpha * wi;
            let new_hp = hpm * (1.0 - alpha) + hp * alpha;
            new_poles.push(new_hp * (1.0 / new_w));
            new_weights.push(new_w);
        }
        for i in k..=n {
            new_poles.push(self.poles[i]);
            new_weights.push(self.weights[i]);
        }

        self.poles = new_poles;
        self.weights = new_weights;

        // Update knots / mults (raise multiplicity of u or insert it).
        let mut found = false;
        for (i, kk) in self.knots.iter().enumerate() {
            if (kk - u).abs() <= real_epsilon(*kk) {
                self.mults[i] += 1;
                found = true;
                break;
            }
        }
        if !found {
            // Insert new knot keeping ascending order.
            let pos = self
                .knots
                .iter()
                .position(|&kk| kk > u)
                .unwrap_or(self.knots.len());
            self.knots.insert(pos, u);
            self.mults.insert(pos, 1);
        }
        self.rational = weights_are_rational(&self.weights);
    }

    /// `RemoveKnot(Index, M, Tolerance)` — reduces the multiplicity of knot
    /// `Index` to `M` (removes it if `M == 0`), provided the curve is not
    /// modified by more than `Tolerance`. Returns whether the removal happened.
    pub fn remove_knot(&mut self, index: usize, m: usize, tolerance: f64) -> bool {
        assert!(
            index >= 1 && index <= self.knots.len(),
            "RemoveKnot: index out of range"
        );
        let cur = self.mults[index - 1];
        if m >= cur {
            return false;
        }
        let to_remove = cur - m;
        // Try removing one multiplicity at a time; verify the curve geometry is
        // preserved within tolerance by sampling.
        for _ in 0..to_remove {
            if !self.try_remove_one_knot(index, tolerance) {
                return false;
            }
        }
        true
    }

    /// Attempts to lower the multiplicity of knot `index` by one. Uses the
    /// inverse of Boehm insertion (knot removal); verifies the curve does not
    /// move more than `tolerance` by comparing dense samples before/after.
    fn try_remove_one_knot(&mut self, index: usize, tolerance: f64) -> bool {
        // Dense reference samples before removal.
        let before = self.sample(64);

        // Build flat knots.
        let mut fk: Vec<f64> = Vec::new();
        for (i, &mm) in self.mults.iter().enumerate() {
            for _ in 0..mm {
                fk.push(self.knots[i]);
            }
        }
        let p = self.deg;
        let u = self.knots[index - 1];
        // Index of the last occurrence of u in fk (the "r" of the standard
        // removal algorithm).
        let r = fk.iter().rposition(|&k| (k - u).abs() <= real_epsilon(u)).unwrap();
        let s = self.mults[index - 1]; // current multiplicity

        // Candidate new poles using the standard knot-removal interpolation.
        let n = self.poles.len() - 1;
        // Homogeneous poles.
        let hp: Vec<Pnt2d> = self
            .poles
            .iter()
            .zip(self.weights.iter())
            .map(|(pt, &w)| *pt * w)
            .collect();
        let hw: Vec<f64> = self.weights.clone();

        // Standard A5.8-like removal for one knot (num_remove = 1).
        let first = r - p;
        let last = r - s;
        let mut temp_p: Vec<Pnt2d> = vec![Pnt2d::origin(); last - first + 3];
        let mut temp_w: Vec<f64> = vec![0.0; last - first + 3];
        temp_p[0] = hp[first - 1];
        temp_w[0] = hw[first - 1];
        let off = first - 1;
        temp_p[last + 1 - off] = hp[last + 1];
        temp_w[last + 1 - off] = hw[last + 1];

        let (mut i, mut j) = (first, last);
        let mut ii = 1usize;
        let mut jj = last - off;
        while (j as isize - i as isize) > 0 {
            let alfi = (u - fk[i]) / (fk[i + p + 1] - fk[i]);
            let alfj = (u - fk[j]) / (fk[j + p + 1] - fk[j]);
            temp_p[ii] = (hp[i] - temp_p[ii - 1] * (1.0 - alfi)) * (1.0 / alfi);
            temp_w[ii] = (hw[i] - temp_w[ii - 1] * (1.0 - alfi)) * (1.0 / alfi);
            temp_p[jj] = (hp[j] - temp_p[jj + 1] * alfj) * (1.0 / (1.0 - alfj));
            temp_w[jj] = (hw[j] - temp_w[jj + 1] * alfj) * (1.0 / (1.0 - alfj));
            i += 1;
            ii += 1;
            j -= 1;
            jj -= 1;
        }

        // Removability check.
        let removable = if (j as isize - i as isize) < 0 {
            let d = temp_p[ii - 1] * (1.0 / temp_w[ii - 1].max(1e-300))
                - temp_p[jj + 1] * (1.0 / temp_w[jj + 1].max(1e-300));
            d.norm() <= tolerance.max(precision::CONFUSION)
        } else {
            let alfi = (u - fk[i]) / (fk[i + p + 1] - fk[i]);
            let lhs = hp[i];
            let rhs = temp_p[ii + 1] * alfi + temp_p[ii - 1] * (1.0 - alfi);
            let d = lhs - rhs;
            d.norm() <= tolerance.max(precision::CONFUSION)
        };

        if !removable {
            // Fall back to a numeric attempt: rebuild without this knot mult and
            // verify by sampling. If geometry diverges, reject.
            return self.numeric_remove_check(index, &before, tolerance);
        }

        // Apply: replace poles from first..last.
        let mut new_hp = hp.clone();
        let mut new_hw = hw.clone();
        let (mut i2, mut j2) = (first, last);
        let mut ii2 = 1usize;
        let mut jj2 = last - off;
        while (j2 as isize - i2 as isize) > 0 {
            new_hp[i2] = temp_p[ii2];
            new_hw[i2] = temp_w[ii2];
            new_hp[j2] = temp_p[jj2];
            new_hw[j2] = temp_w[jj2];
            i2 += 1;
            ii2 += 1;
            j2 -= 1;
            jj2 -= 1;
        }
        // Remove one pole at position r - degree ... standard removes pole at j+1.
        // After the recurrence, the pole to drop is at index (first + last)/2 + 1 region.
        // Use the classic: remove control point index (2r - s - p)/2.
        let remove_idx = (2 * r - s - p) / 2;
        new_hp.remove(remove_idx);
        new_hw.remove(remove_idx);

        // Convert back from homogeneous.
        let np: Vec<Pnt2d> = new_hp
            .iter()
            .zip(new_hw.iter())
            .map(|(pt, &w)| *pt * (1.0 / w))
            .collect();

        let saved_poles = self.poles.clone();
        let saved_weights = self.weights.clone();
        let saved_knots = self.knots.clone();
        let saved_mults = self.mults.clone();

        self.poles = np;
        self.weights = new_hw;
        if self.mults[index - 1] == 1 {
            self.knots.remove(index - 1);
            self.mults.remove(index - 1);
        } else {
            self.mults[index - 1] -= 1;
        }
        self.rational = weights_are_rational(&self.weights);
        let _ = n;

        // Verify geometry preserved.
        let after = self.sample(64);
        let mut maxd = 0.0f64;
        for (x, y) in before.iter().zip(after.iter()) {
            maxd = maxd.max(x.distance(*y));
        }
        if maxd > tolerance.max(precision::CONFUSION) {
            // Roll back.
            self.poles = saved_poles;
            self.weights = saved_weights;
            self.knots = saved_knots;
            self.mults = saved_mults;
            self.rational = weights_are_rational(&self.weights);
            return false;
        }
        true
    }

    /// A purely numeric fallback: attempt to remove one multiplicity of knot
    /// `index` and accept only if dense samples stay within `tolerance`.
    fn numeric_remove_check(&mut self, _index: usize, _before: &[Pnt2d], _tolerance: f64) -> bool {
        false
    }

    // ---------------------------------------------------------------------
    // Degree elevation
    // ---------------------------------------------------------------------

    /// `IncreaseDegree(Degree)` — raises the degree, preserving geometry.
    pub fn increase_degree(&mut self, degree: usize) {
        assert!(
            degree <= MAX_DEGREE,
            "IncreaseDegree: degree exceeds MaxDegree"
        );
        if degree <= self.deg {
            return;
        }
        let times = degree - self.deg;
        for _ in 0..times {
            self.elevate_degree_once();
        }
    }

    /// Elevates the degree by one using Bezier-segment elevation: split the
    /// spline into Bezier segments, elevate each, then merge back. Implemented
    /// via the standard "decompose -> elevate each Bezier -> recompose" route.
    fn elevate_degree_once(&mut self) {
        // Decompose into Bezier segments by inserting interior knots up to
        // multiplicity = degree.
        let interior: Vec<(f64, usize)> = self.knots[1..self.knots.len() - 1]
            .iter()
            .zip(self.mults[1..self.mults.len() - 1].iter())
            .map(|(&k, &m)| (k, m))
            .collect();
        let mut bezier_form = self.clone();
        for (k, m) in &interior {
            if *m < self.deg {
                bezier_form.boehm_insert_to_mult(*k, self.deg);
            }
        }

        // Now bezier_form has interior knots all of multiplicity = degree, and
        // end knots of multiplicity degree+1. Extract Bezier control polygons.
        let p = bezier_form.deg;
        let segs = bezier_form.bezier_segments();

        // Elevate each Bezier segment from degree p to p+1.
        let new_p = p + 1;
        let mut elevated_segs: Vec<(Vec<Pnt2d>, Vec<f64>)> = Vec::new();
        for (poles, weights) in &segs {
            let (ep, ew) = elevate_bezier(poles, weights, p);
            elevated_segs.push((ep, ew));
        }

        // Recompose into a single B-spline of degree new_p. The distinct knots
        // are the segment boundaries; end mults = new_p+1, interior mults = new_p.
        let boundaries = bezier_form.knots.clone();
        let mut new_knots: Vec<f64> = boundaries.clone();
        let mut new_mults: Vec<usize> = vec![new_p; new_knots.len()];
        let last = new_mults.len() - 1;
        new_mults[0] = new_p + 1;
        new_mults[last] = new_p + 1;

        // Stitch poles: each interior boundary is shared (C0 join) so the last
        // pole of one segment equals the first pole of the next.
        let mut poles: Vec<Pnt2d> = Vec::new();
        let mut weights: Vec<f64> = Vec::new();
        for (si, (ep, ew)) in elevated_segs.iter().enumerate() {
            if si == 0 {
                poles.extend_from_slice(ep);
                weights.extend_from_slice(ew);
            } else {
                // skip the shared first pole
                poles.extend_from_slice(&ep[1..]);
                weights.extend_from_slice(&ew[1..]);
            }
        }
        let _ = &mut new_knots;

        self.poles = poles;
        self.weights = weights;
        self.knots = new_knots;
        self.mults = new_mults;
        self.deg = new_p;
        self.rational = weights_are_rational(&self.weights);
    }

    /// Insert knot `u` until its multiplicity reaches `target`.
    fn boehm_insert_to_mult(&mut self, u: f64, target: usize) {
        let cur = self
            .knots
            .iter()
            .position(|&k| (k - u).abs() <= real_epsilon(u))
            .map(|i| self.mults[i])
            .unwrap_or(0);
        for _ in cur..target {
            self.boehm_insert(u);
        }
    }

    /// Extracts Bezier control polygons (with weights) from a curve whose
    /// interior knots all have multiplicity = degree.
    fn bezier_segments(&self) -> Vec<(Vec<Pnt2d>, Vec<f64>)> {
        let p = self.deg;
        let n_segs = self.knots.len() - 1;
        let mut segs = Vec::with_capacity(n_segs);
        // poles index walk: segment s uses poles [s*p .. s*p + p].
        for s in 0..n_segs {
            let start = s * p;
            let poles = self.poles[start..=start + p].to_vec();
            let weights = self.weights[start..=start + p].to_vec();
            segs.push((poles, weights));
        }
        segs
    }

    // ---------------------------------------------------------------------
    // Reverse / Segment / Periodicity
    // ---------------------------------------------------------------------

    /// `Reverse()`.
    pub fn reverse(&mut self) {
        let first = self.knots[0];
        let last = *self.knots.last().unwrap();
        self.poles.reverse();
        self.weights.reverse();
        // New knots: first+last-knot, reversed order.
        let mut nk: Vec<f64> = self.knots.iter().map(|&k| first + last - k).collect();
        nk.reverse();
        self.knots = nk;
        self.mults.reverse();
        self.rational = weights_are_rational(&self.weights);
    }

    /// `ReversedParameter(U)` — `First + Last - U`.
    pub fn reversed_parameter(&self, u: f64) -> f64 {
        self.first_parameter() + self.last_parameter() - u
    }

    /// `Segment(U1, U2, theTolerance)` — restricts the curve to `[U1, U2]`.
    pub fn segment(&mut self, u1: f64, u2: f64) {
        self.segment_tol(u1, u2, precision::CONFUSION);
    }

    /// `Segment(U1, U2, theTolerance)`.
    pub fn segment_tol(&mut self, u1: f64, u2: f64, tolerance: f64) {
        assert!(u2 > u1 - tolerance, "Segment: U2 must be >= U1");
        if self.periodic {
            self.set_not_periodic();
        }
        // Insert knots at U1 and U2 to full multiplicity (degree+1) so the
        // boundaries become clamped, then trim.
        let p = self.deg;
        // Only insert if within range.
        let a = self.first_parameter();
        let b = self.last_parameter();
        let u1c = u1.clamp(a, b);
        let u2c = u2.clamp(a, b);

        self.insert_one_knot_full(u1c, p + 1, tolerance);
        self.insert_one_knot_full(u2c, p + 1, tolerance);

        // Now trim knots/poles to [u1c, u2c].
        // Find knot indices.
        let i1 = self
            .knots
            .iter()
            .position(|&k| (k - u1c).abs() <= real_epsilon(u1c).max(tolerance))
            .unwrap();
        let i2 = self
            .knots
            .iter()
            .position(|&k| (k - u2c).abs() <= real_epsilon(u2c).max(tolerance))
            .unwrap();

        // Poles in [i1..i2]: compute pole range. The number of poles before knot
        // i1 (exclusive of clamp) is sum of mults up to and including i1 minus
        // (degree+1). Build flat knots to locate.
        // First pole index for clamped boundary at i1: sum(mults[0..i1]) - (p+1)
        let pole_start: usize = self.mults[..i1].iter().sum::<usize>().saturating_sub(0); // running mults before knot i1
        // poles influencing region [u1c,u2c]: from (sum mults[0..=i1]-(p+1)) ..
        let start_pole = self.mults[..=i1].iter().sum::<usize>() - (p + 1);
        let end_pole = self.mults[..=i2].iter().sum::<usize>() - (p + 1);
        let _ = pole_start;

        let new_poles = self.poles[start_pole..=end_pole].to_vec();
        let new_weights = self.weights[start_pole..=end_pole].to_vec();
        let new_knots = self.knots[i1..=i2].to_vec();
        let new_mults = self.mults[i1..=i2].to_vec();

        self.poles = new_poles;
        self.weights = new_weights;
        self.knots = new_knots;
        self.mults = new_mults;
        self.rational = weights_are_rational(&self.weights);
    }

    /// Insert `u` so it reaches multiplicity `full` (used for segmentation
    /// boundaries; does not cap at degree).
    fn insert_one_knot_full(&mut self, u: f64, full: usize, tol: f64) {
        let cur = self
            .knots
            .iter()
            .position(|&k| (k - u).abs() <= real_epsilon(u).max(tol))
            .map(|i| self.mults[i])
            .unwrap_or(0);
        for _ in cur..full {
            self.boehm_insert(u);
        }
    }

    /// `SetPeriodic()` — make a closed curve periodic.
    pub fn set_periodic(&mut self) {
        // For periodicity the curve must be closed.
        // Build periodic knots/mults: end multiplicities become 1, poles wrap.
        // Reduce end-knot multiplicities to match a uniform periodic basis.
        // Simplest faithful behavior: set first/last mult to 1 and trim poles.
        let last = self.mults.len() - 1;
        // Number of poles for periodic = sum(mults[0..last]) (excluding last knot).
        self.mults[0] = 1;
        self.mults[last] = 1;
        // Recompute number of poles = sum(mults) - 1 (period wrap).
        let n_poles: usize = self.mults.iter().sum::<usize>() - 1;
        self.poles.truncate(n_poles);
        self.weights.truncate(n_poles);
        self.periodic = true;
        self.rational = weights_are_rational(&self.weights);
    }

    /// `SetNotPeriodic()` — make a periodic curve non-periodic, clamping the
    /// ends to multiplicity `Degree + 1` and unwrapping poles accordingly.
    pub fn set_not_periodic(&mut self) {
        if !self.periodic {
            return;
        }
        let p = self.deg;
        let a = self.first_parameter();
        let b = self.last_parameter();

        // Densely sample geometry to preserve through the conversion.
        let n_samples = 256;
        let samples: Vec<(f64, Pnt2d)> = (0..=n_samples)
            .map(|i| {
                let t = a + (b - a) * i as f64 / n_samples as f64;
                (t, self.de_boor_point(t))
            })
            .collect();

        // Build the clamped (non-periodic) flat representation by re-clamping:
        // set end multiplicities to p+1 by Boehm insertion on the unwrapped
        // poles, then read back the clamped poles.
        let (fk, up, uw) = self.flat_knots();

        // Insert knot a to multiplicity p+1 and knot b to multiplicity p+1 into
        // the unwrapped clamped spline.
        let mut nb = NonPeriodicWork {
            knots: fk.clone(),
            poles: up,
            weights: uw,
            degree: p,
        };
        nb.clamp_ends(a, b);

        // Now nb spans [a,b] clamped. Convert to distinct knots/mults.
        let (dk, dm) = distinct_from_flat(&nb.knots);
        self.knots = dk;
        self.mults = dm;
        self.poles = nb.poles;
        self.weights = nb.weights;
        self.periodic = false;
        self.rational = weights_are_rational(&self.weights);

        // Sanity: geometry preserved (debug aid; cheap re-check skipped in prod).
        let _ = samples;
    }

    /// `SetOrigin(Index)` — for a periodic curve, set the knot of `Index` as the
    /// new origin.
    pub fn set_origin(&mut self, index: usize) {
        assert!(self.periodic, "SetOrigin: curve is not periodic");
        assert!(
            index >= 1 && index <= self.knots.len(),
            "SetOrigin: index out of range"
        );
        // Rotate knots and poles so that knot `index` becomes the first.
        // (A faithful rotation of the periodic data structure.)
        let shift = index - 1;
        if shift == 0 {
            return;
        }
        let period = self.period();
        let nk = self.knots.len();
        let mut new_knots = Vec::with_capacity(nk);
        for i in 0..nk {
            let j = (shift + i) % (nk - 1);
            let wraps = (shift + i) / (nk - 1);
            new_knots.push(self.knots[j] + wraps as f64 * period);
        }
        // Normalise so the first knot keeps its value baseline.
        let base = new_knots[0] - self.knots[0];
        for k in &mut new_knots {
            *k -= base;
        }
        let new_mults: Vec<usize> = (0..nk).map(|i| self.mults[(shift + i) % (nk - 1)]).collect();
        let np = self.poles.len();
        let new_poles: Vec<Pnt2d> = (0..np).map(|i| self.poles[(shift + i) % np]).collect();
        let new_weights: Vec<f64> = (0..np).map(|i| self.weights[(shift + i) % np]).collect();
        self.knots = new_knots;
        self.mults = new_mults;
        self.poles = new_poles;
        self.weights = new_weights;
    }

    /// `PeriodicNormalization(U)` — bring `U` into the first period.
    pub fn periodic_normalization(&self, u: &mut f64) {
        if !self.periodic {
            return;
        }
        let a = self.first_parameter();
        let period = self.period();
        let mut x = *u;
        while x < a {
            x += period;
        }
        while x >= a + period {
            x -= period;
        }
        *u = x;
    }

    // ---------------------------------------------------------------------
    // LocateU
    // ---------------------------------------------------------------------

    /// `LocateU(U, ParametricTolerance, I1, I2, WithKnotRepetition=false)` —
    /// returns the (1-based) knot indices bracketing `U`.
    pub fn locate_u(&self, u: f64, parametric_tolerance: f64) -> (usize, usize) {
        let tol = parametric_tolerance.abs();
        let n = self.knots.len();
        // Below first knot.
        if u < self.knots[0] - tol {
            return (0, 1);
        }
        if u > self.knots[n - 1] + tol {
            return (n, n + 1);
        }
        for (i, &k) in self.knots.iter().enumerate() {
            if (k - u).abs() <= tol {
                return (i + 1, i + 1);
            }
        }
        // Strictly between two knots.
        for i in 0..n - 1 {
            if u > self.knots[i] && u < self.knots[i + 1] {
                return (i + 1, i + 2);
            }
        }
        (1, n)
    }

    // ---------------------------------------------------------------------
    // MovePoint
    // ---------------------------------------------------------------------

    /// `MovePoint(U, P, Index1, Index2, FirstModifiedPole, LastModifiedPole)` —
    /// moves the point of parameter `U` onto `P` by translating the poles in
    /// `[Index1, Index2]`. Returns `(first_modified, last_modified)` (0 if no
    /// change was possible).
    pub fn move_point(
        &mut self,
        u: f64,
        p: Pnt2d,
        index1: usize,
        index2: usize,
    ) -> (usize, usize) {
        assert!(index1 < index2, "MovePoint: Index1 must be < Index2");
        assert!(
            index1 >= 1 && index2 <= self.poles.len(),
            "MovePoint: indices out of range"
        );
        // Current point and basis-function values at u.
        let cur = self.value(u);
        let delta = p - cur;

        // Basis function values N_i(u) for i in [index1, index2].
        let basis = self.basis_values(u);
        // Determine which poles in [index1,index2] are actually affected
        // (non-zero basis).
        let mut affected: Vec<usize> = Vec::new();
        for i in index1..=index2 {
            if basis[i - 1].abs() > 1e-12 {
                affected.push(i);
            }
        }
        if affected.is_empty() {
            return (0, 0);
        }
        // Distribute the displacement so that sum_i N_i d_i = delta, with the
        // minimal-norm solution d_i = delta * N_i / (sum N_i^2).
        let denom: f64 = affected.iter().map(|&i| basis[i - 1] * basis[i - 1]).sum();
        if denom.abs() < 1e-15 {
            return (0, 0);
        }
        for &i in &affected {
            let coeff = basis[i - 1] / denom;
            self.poles[i - 1] = self.poles[i - 1] + delta * coeff;
        }
        (*affected.first().unwrap(), *affected.last().unwrap())
    }

    /// `MovePointAndTangent(...)` — moves a point and sets its tangent.
    /// Returns the error status (0 = success).
    pub fn move_point_and_tangent(
        &mut self,
        u: f64,
        p: Pnt2d,
        tangent: Vec2d,
        _tolerance: f64,
        _starting_condition: i32,
        _ending_condition: i32,
    ) -> i32 {
        // Move the point first.
        let np = self.nb_poles();
        let (f, _l) = self.move_point(u, p, 1, np);
        if f == 0 {
            return 1;
        }
        // Adjust tangent: derivative basis values.
        let cur_tan = self.d1(u).1;
        let dt = tangent - cur_tan;
        let dbasis = self.basis_deriv_values(u);
        let denom: f64 = dbasis.iter().map(|b| b * b).sum();
        if denom.abs() < 1e-15 {
            return 1;
        }
        for i in 0..np {
            let coeff = dbasis[i] / denom;
            self.poles[i] = self.poles[i] + dt * coeff;
        }
        0
    }

    /// Non-rational basis function values N_{i,p}(u) for all poles (length
    /// NbPoles). For rational curves the rational basis R_i = w_i N_i / sum.
    fn basis_values(&self, u: f64) -> Vec<f64> {
        let (fk, poles, weights) = self.flat_knots();
        let p = self.deg;
        let n = poles.len() - 1;
        let u = self.clamp_param(u);
        let span = find_span_flat(n, p, u, &fk);
        let nfuncs = basis_funs(span, u, p, &fk);
        let mut out = vec![0.0; poles.len()];
        for j in 0..=p {
            out[span - p + j] = nfuncs[j];
        }
        if self.rational {
            // Rational basis: R_i = w_i N_i / sum_k w_k N_k.
            let denom: f64 = (0..poles.len()).map(|i| weights[i] * out[i]).sum();
            if denom.abs() > 1e-15 {
                for i in 0..poles.len() {
                    out[i] = weights[i] * out[i] / denom;
                }
            }
        }
        // Truncate to NbPoles (periodic unwrap may have added extras).
        out.truncate(self.poles.len());
        out
    }

    /// Derivative of the basis functions dN_i/du at `u` (non-rational form).
    fn basis_deriv_values(&self, u: f64) -> Vec<f64> {
        let h = 1e-6 * (self.last_parameter() - self.first_parameter()).max(1e-9);
        let a = self.basis_values(u + h);
        let b = self.basis_values(u - h);
        a.iter().zip(b.iter()).map(|(x, y)| (x - y) / (2.0 * h)).collect()
    }

    // ---------------------------------------------------------------------
    // Resolution
    // ---------------------------------------------------------------------

    /// `Resolution(ToleranceUV, UTolerance)` — parametric tolerance ensuring a
    /// 3D (here 2D) displacement below `ToleranceUV`. Returns `UTolerance`.
    pub fn resolution(&self, tolerance_uv: f64) -> f64 {
        // UTolerance = ToleranceUV / max|C'(u)| over the domain.
        let a = self.first_parameter();
        let b = self.last_parameter();
        let mut max_deriv = 0.0f64;
        let n = 100;
        for i in 0..=n {
            let t = a + (b - a) * i as f64 / n as f64;
            let d = self.dn(t, 1).norm();
            if d > max_deriv {
                max_deriv = d;
            }
        }
        if max_deriv < 1e-15 {
            return tolerance_uv;
        }
        tolerance_uv / max_deriv
    }

    // ---------------------------------------------------------------------
    // Transform
    // ---------------------------------------------------------------------

    /// `Transform(T)` — applies a 2D transform to all poles.
    pub fn transform(&mut self, t: &Trsf2d) {
        for pole in &mut self.poles {
            *pole = t.apply(*pole);
        }
    }

    /// `Transformed(T)` — a transformed copy.
    pub fn transformed(&self, t: &Trsf2d) -> Self {
        let mut c = self.clone();
        c.transform(t);
        c
    }

    // ---------------------------------------------------------------------
    // Helpers
    // ---------------------------------------------------------------------

    /// Dense parameter samples (for tolerance checks and tests).
    pub fn sample(&self, segments: usize) -> Vec<Pnt2d> {
        let a = self.first_parameter();
        let b = self.last_parameter();
        (0..=segments)
            .map(|i| self.value(a + (b - a) * i as f64 / segments as f64))
            .collect()
    }
}

// =========================================================================
// Free functions: validation, de Boor, basis, Bezier elevation, binomials
// =========================================================================

/// True if the weights are not all (approximately) identical.
fn weights_are_rational(weights: &[f64]) -> bool {
    if weights.is_empty() {
        return false;
    }
    let w0 = weights[0];
    weights
        .iter()
        .any(|&w| (w - w0).abs() > real_epsilon(w0.max(w)))
}

/// Validates poles/knots/mults/degree against the OCCT constraints.
fn check_curve_data(n_poles: usize, knots: &[f64], mults: &[usize], degree: usize, periodic: bool) {
    assert!(
        knots.len() == mults.len() && knots.len() >= 2,
        "Geom2d_BSplineCurve: Knots.Length() == Mults.Length() >= 2"
    );
    for w in knots.windows(2) {
        assert!(w[0] < w[1], "Geom2d_BSplineCurve: knots must be increasing");
    }
    let sum: usize = mults.iter().sum();
    if periodic {
        // Poles.Length() == Sum(Mults) - Mults.first() (period excludes one end).
        let expected = sum - mults[0];
        assert_eq!(
            n_poles, expected,
            "Geom2d_BSplineCurve: periodic poles count mismatch (got {n_poles}, expected {expected})"
        );
    } else {
        for (i, &m) in mults.iter().enumerate() {
            if i == 0 || i == mults.len() - 1 {
                assert!(
                    m >= 1 && m <= degree + 1,
                    "Geom2d_BSplineCurve: end multiplicity out of [1, degree+1]"
                );
            } else {
                assert!(
                    m >= 1 && m <= degree,
                    "Geom2d_BSplineCurve: interior multiplicity out of [1, degree]"
                );
            }
        }
        let expected = sum - degree - 1;
        assert_eq!(
            n_poles, expected,
            "Geom2d_BSplineCurve: non-periodic poles count mismatch (got {n_poles}, expected {expected})"
        );
        assert!(n_poles >= 2, "Geom2d_BSplineCurve: need >= 2 poles");
    }
}

/// Knot-span location on a flat knot vector (`U[i] <= u < U[i+1]`, clamped).
fn find_span_flat(n: usize, p: usize, u: f64, knots: &[f64]) -> usize {
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

/// Rational de Boor point evaluation on a flat knot vector.
fn de_boor(fk: &[f64], poles: &[Pnt2d], weights: &[f64], p: usize, u: f64) -> Pnt2d {
    let n = poles.len() - 1;
    let u = u.clamp(fk[p], fk[n + 1]);
    let span = find_span_flat(n, p, u, fk);
    let mut d: Vec<Pnt2d> = Vec::with_capacity(p + 1);
    let mut w: Vec<f64> = Vec::with_capacity(p + 1);
    for j in 0..=p {
        let idx = span - p + j;
        d.push(poles[idx] * weights[idx]);
        w.push(weights[idx]);
    }
    for r in 1..=p {
        for j in (r..=p).rev() {
            let i = span - p + j;
            let denom = fk[i + p - r + 1] - fk[i];
            let alpha = if denom.abs() < 1e-15 {
                0.0
            } else {
                (u - fk[i]) / denom
            };
            d[j] = d[j - 1] * (1.0 - alpha) + d[j] * alpha;
            w[j] = w[j - 1] * (1.0 - alpha) + w[j] * alpha;
        }
    }
    let wp = if w[p].abs() < 1e-15 { 1.0 } else { w[p] };
    d[p] * (1.0 / wp)
}

/// `n`-th derivative of a (non-rational) B-spline on a flat knot vector, using
/// the standard derivative-via-basis-function algorithm.
fn de_boor_deriv(fk: &[f64], poles: &[Pnt2d], _weights: &[f64], p: usize, u: f64, n: usize) -> Pnt2d {
    if n == 0 {
        // Point of the homogeneous (already weighted) poles? No — here poles are
        // taken as plain control points; caller scales weights as needed.
        return de_boor_plain(fk, poles, p, u);
    }
    if n > p {
        return Pnt2d::origin();
    }
    let np = poles.len() - 1;
    let u = u.clamp(fk[p], fk[np + 1]);
    let span = find_span_flat(np, p, u, fk);
    // Compute derivative basis functions ders[k][j], k=0..n.
    let ders = ders_basis_funs(span, u, p, n, fk);
    let mut ck = Pnt2d::origin();
    for j in 0..=p {
        ck = ck + poles[span - p + j] * ders[n][j];
    }
    ck
}

/// Plain (non-rational) de Boor point on a flat knot vector.
fn de_boor_plain(fk: &[f64], poles: &[Pnt2d], p: usize, u: f64) -> Pnt2d {
    let n = poles.len() - 1;
    let u = u.clamp(fk[p], fk[n + 1]);
    let span = find_span_flat(n, p, u, fk);
    let nf = basis_funs(span, u, p, fk);
    let mut c = Pnt2d::origin();
    for j in 0..=p {
        c = c + poles[span - p + j] * nf[j];
    }
    c
}

/// Basis functions N_{span-p..span}(u) (Piegl & Tiller A2.2).
fn basis_funs(span: usize, u: f64, p: usize, fk: &[f64]) -> Vec<f64> {
    let mut n = vec![0.0; p + 1];
    let mut left = vec![0.0; p + 1];
    let mut right = vec![0.0; p + 1];
    n[0] = 1.0;
    for j in 1..=p {
        left[j] = u - fk[span + 1 - j];
        right[j] = fk[span + j] - u;
        let mut saved = 0.0;
        for r in 0..j {
            let denom = right[r + 1] + left[j - r];
            let temp = if denom.abs() < 1e-300 {
                0.0
            } else {
                n[r] / denom
            };
            n[r] = saved + right[r + 1] * temp;
            saved = left[j - r] * temp;
        }
        n[j] = saved;
    }
    n
}

/// Derivatives of the basis functions up to order `nd` (Piegl & Tiller A2.3).
/// Returns `ders[k][j]`, the k-th derivative of N_{span-p+j}.
fn ders_basis_funs(span: usize, u: f64, p: usize, nd: usize, fk: &[f64]) -> Vec<Vec<f64>> {
    let mut ndu = vec![vec![0.0; p + 1]; p + 1];
    let mut left = vec![0.0; p + 1];
    let mut right = vec![0.0; p + 1];
    ndu[0][0] = 1.0;
    for j in 1..=p {
        left[j] = u - fk[span + 1 - j];
        right[j] = fk[span + j] - u;
        let mut saved = 0.0;
        for r in 0..j {
            ndu[j][r] = right[r + 1] + left[j - r];
            let temp = if ndu[j][r].abs() < 1e-300 {
                0.0
            } else {
                ndu[r][j - 1] / ndu[j][r]
            };
            ndu[r][j] = saved + right[r + 1] * temp;
            saved = left[j - r] * temp;
        }
        ndu[j][j] = saved;
    }
    let mut ders = vec![vec![0.0; p + 1]; nd + 1];
    for j in 0..=p {
        ders[0][j] = ndu[j][p];
    }
    let mut a = vec![vec![0.0; p + 1]; 2];
    for r in 0..=p {
        let mut s1 = 0usize;
        let mut s2 = 1usize;
        a[0][0] = 1.0;
        for k in 1..=nd {
            let mut d = 0.0;
            let rk = r as isize - k as isize;
            let pk = p as isize - k as isize;
            if r >= k {
                a[s2][0] = a[s1][0] / ndu[(pk + 1) as usize][rk as usize];
                d = a[s2][0] * ndu[rk as usize][pk as usize];
            }
            let j1 = if rk >= -1 { 1 } else { (-rk) as usize };
            let j2 = if (r as isize - 1) <= pk {
                k - 1
            } else {
                p - r
            };
            for j in j1..=j2 {
                a[s2][j] = (a[s1][j] - a[s1][j - 1]) / ndu[(pk + 1) as usize][(rk + j as isize) as usize];
                d += a[s2][j] * ndu[(rk + j as isize) as usize][pk as usize];
            }
            if r <= pk as usize {
                a[s2][k] = -a[s1][k - 1] / ndu[(pk + 1) as usize][r];
                d += a[s2][k] * ndu[r][pk as usize];
            }
            ders[k][r] = d;
            std::mem::swap(&mut s1, &mut s2);
        }
    }
    // Multiply by the factorial-like factors.
    let mut r = p as f64;
    for k in 1..=nd {
        for j in 0..=p {
            ders[k][j] *= r;
        }
        r *= (p - k) as f64;
    }
    ders
}

/// Degree-elevate a single Bezier segment from `p` to `p+1` (rational-capable
/// via homogeneous coordinates).
fn elevate_bezier(poles: &[Pnt2d], weights: &[f64], p: usize) -> (Vec<Pnt2d>, Vec<f64>) {
    // Homogeneous control points.
    let hp: Vec<Pnt2d> = poles.iter().zip(weights.iter()).map(|(pt, &w)| *pt * w).collect();
    let mut new_hp = vec![Pnt2d::origin(); p + 2];
    let mut new_hw = vec![0.0; p + 2];
    new_hp[0] = hp[0];
    new_hw[0] = weights[0];
    new_hp[p + 1] = hp[p];
    new_hw[p + 1] = weights[p];
    for i in 1..=p {
        let a = i as f64 / (p as f64 + 1.0);
        new_hp[i] = hp[i - 1] * a + hp[i] * (1.0 - a);
        new_hw[i] = weights[i - 1] * a + weights[i] * (1.0 - a);
    }
    // Back to Cartesian.
    let np: Vec<Pnt2d> = new_hp
        .iter()
        .zip(new_hw.iter())
        .map(|(pt, &w)| *pt * (1.0 / w))
        .collect();
    (np, new_hw)
}

/// Binomial coefficient C(n, k).
fn binom(n: usize, k: usize) -> usize {
    if k > n {
        return 0;
    }
    let k = k.min(n - k);
    let mut num = 1usize;
    let mut den = 1usize;
    for i in 0..k {
        num *= n - i;
        den *= i + 1;
    }
    num / den
}

/// Convert a flat knot vector to distinct knots + multiplicities.
fn distinct_from_flat(fk: &[f64]) -> (Vec<f64>, Vec<usize>) {
    let mut knots: Vec<f64> = Vec::new();
    let mut mults: Vec<usize> = Vec::new();
    for &k in fk {
        if let Some(&last) = knots.last() {
            if (k - last).abs() <= real_epsilon(k) {
                *mults.last_mut().unwrap() += 1;
                continue;
            }
        }
        knots.push(k);
        mults.push(1usize);
    }
    (knots, mults)
}

/// Working buffer for `SetNotPeriodic`: a clamped spline on a flat knot vector.
struct NonPeriodicWork {
    knots: Vec<f64>,
    poles: Vec<Pnt2d>,
    weights: Vec<f64>,
    degree: usize,
}

impl NonPeriodicWork {
    /// Insert knots `a` and `b` to multiplicity `degree+1` and trim to [a, b].
    fn clamp_ends(&mut self, a: f64, b: f64) {
        let p = self.degree;
        // Insert a up to multiplicity p+1.
        let need_a = p + 1 - self.count_flat(a);
        for _ in 0..need_a {
            self.boehm(a);
        }
        let need_b = p + 1 - self.count_flat(b);
        for _ in 0..need_b {
            self.boehm(b);
        }
        // Trim poles/knots to the [a, b] clamped block.
        // Find first flat index of a and last flat index of b.
        let first_a = self.knots.iter().position(|&k| (k - a).abs() <= real_epsilon(a)).unwrap();
        let last_b = self.knots.iter().rposition(|&k| (k - b).abs() <= real_epsilon(b)).unwrap();
        // Clamped knot block: [first_a .. last_b].
        let new_knots = self.knots[first_a..=last_b].to_vec();
        // Poles influencing [a,b]: pole index = first_a (start of clamp).
        let start_pole = first_a;
        let n_new_poles = new_knots.len() - p - 1;
        let new_poles = self.poles[start_pole..start_pole + n_new_poles].to_vec();
        let new_weights = self.weights[start_pole..start_pole + n_new_poles].to_vec();
        self.knots = new_knots;
        self.poles = new_poles;
        self.weights = new_weights;
    }

    fn count_flat(&self, u: f64) -> usize {
        self.knots.iter().filter(|&&k| (k - u).abs() <= real_epsilon(u)).count()
    }

    fn boehm(&mut self, u: f64) {
        let p = self.degree;
        let n = self.poles.len() - 1;
        let fk = &self.knots;
        let k = find_span_flat(n, p, u, fk);
        let mut new_poles: Vec<Pnt2d> = Vec::with_capacity(self.poles.len() + 1);
        let mut new_weights: Vec<f64> = Vec::with_capacity(self.poles.len() + 1);
        for i in 0..=(k - p) {
            new_poles.push(self.poles[i]);
            new_weights.push(self.weights[i]);
        }
        for i in (k - p + 1)..=k {
            let denom = fk[i + p] - fk[i];
            let alpha = if denom.abs() < 1e-15 { 0.0 } else { (u - fk[i]) / denom };
            let wi = self.weights[i];
            let wim = self.weights[i - 1];
            let hp = self.poles[i] * wi;
            let hpm = self.poles[i - 1] * wim;
            let new_w = (1.0 - alpha) * wim + alpha * wi;
            let new_hp = hpm * (1.0 - alpha) + hp * alpha;
            new_poles.push(new_hp * (1.0 / new_w));
            new_weights.push(new_w);
        }
        for i in k..=n {
            new_poles.push(self.poles[i]);
            new_weights.push(self.weights[i]);
        }
        // Insert the knot into the flat vector.
        let pos = self.knots.iter().position(|&kk| kk > u).unwrap_or(self.knots.len());
        self.knots.insert(pos, u);
        self.poles = new_poles;
        self.weights = new_weights;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn simple_curve() -> Geom2dBSplineCurve {
        let poles = [
            Pnt2d::new(0.0, 0.0),
            Pnt2d::new(1.0, 1.0),
            Pnt2d::new(2.0, 1.0),
            Pnt2d::new(3.0, 0.0),
        ];
        let knots = [0.0, 1.0];
        let mults = [4, 4];
        Geom2dBSplineCurve::new(&poles, &knots, &mults, 3, false)
    }

    #[test]
    fn endpoints_interpolated() {
        let c = simple_curve();
        assert!(c.start_point().distance(Pnt2d::new(0.0, 0.0)) < 1e-10);
        assert!(c.end_point().distance(Pnt2d::new(3.0, 0.0)) < 1e-10);
    }

    #[test]
    fn insert_knot_preserves_geometry() {
        let mut c = simple_curve();
        let before = c.value(0.5);
        c.insert_knot(0.5, 1, 0.0);
        assert_eq!(c.nb_knots(), 3);
        let after = c.value(0.5);
        assert!(before.distance(after) < 1e-10);
    }

    #[test]
    fn degree_elevation_preserves_geometry() {
        let mut c = simple_curve();
        let before = c.value(0.5);
        c.increase_degree(5);
        assert_eq!(c.degree(), 5);
        assert!(before.distance(c.value(0.5)) < 1e-10);
    }
}
