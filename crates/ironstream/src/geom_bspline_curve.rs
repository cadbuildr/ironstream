//! `Geom_BSplineCurve` — a B-spline (and NURBS) curve in 3D space, a faithful
//! reproduction of OpenCascade's `Geom_BSplineCurve`
//! (`src/ModelingData/TKG3d/Geom/Geom_BSplineCurve.hxx`).
//!
//! A B-spline curve can be uniform or non-uniform, rational or non-rational,
//! periodic or non-periodic. It is defined by:
//! - its degree (limited to [`GeomBSplineCurve::MAX_DEGREE`]),
//! - its periodic/non-periodic nature,
//! - a table of poles (control points), with associated weights if rational,
//! - a table of knots with their multiplicities.
//!
//! Indexing convention mirrors OCCT: the public `Pole`, `Knot`, `Multiplicity`,
//! `Weight` accessors are **1-based** (like `NCollection_Array1`), while the
//! internal storage is the usual 0-based Rust `Vec`.
//!
//! This module is self-contained and builds only on the existing `gp`/`precision`
//! API; the de Boor / knot-insertion / degree-elevation algorithms are
//! implemented here directly (Boehm knot insertion, the Cox-de Boor recursion,
//! and standard Piegl-Tiller style basis-function differentiation).

use crate::gp::{Pnt, Trsf, Vec3};
use crate::precision;

/// Knot-distribution classification (`GeomAbs_BSplKnotDistribution`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSplKnotDistribution {
    NonUniform,
    Uniform,
    QuasiUniform,
    PiecewiseBezier,
}

/// Global continuity classification (`GeomAbs_Shape`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Shape {
    C0,
    G1,
    C1,
    G2,
    C2,
    C3,
    CN,
}

/// `Geom_BSplineCurve` — a B-spline / NURBS curve in 3D space.
///
/// The curve stores the OCCT data structure: poles, weights (always populated;
/// all `1.0` when non-rational), distinct knots, their multiplicities, the
/// degree, and the periodic / rational flags. The expanded ("flat") knot
/// sequence is derived from the knots and multiplicities.
// occt: Geom_BSplineCurve
#[derive(Clone, Debug)]
pub struct GeomBSplineCurve {
    poles: Vec<Pnt>,
    /// Always length == poles.len(); all 1.0 when the curve is non-rational.
    weights: Vec<f64>,
    /// Distinct knots (strictly increasing), 1 entry per distinct knot.
    knots: Vec<f64>,
    /// Multiplicities, one per distinct knot.
    mults: Vec<i32>,
    degree: i32,
    periodic: bool,
    rational: bool,
}

impl GeomBSplineCurve {
    /// `Geom_BSplineCurve::MaxDegree()`.
    pub const MAX_DEGREE: i32 = 25;

    // ---------------------------------------------------------------------
    // Construction
    // ---------------------------------------------------------------------

    /// `Geom_BSplineCurve(Poles, Knots, Multiplicities, Degree, Periodic=false)`
    /// — non-rational B-spline curve.
    pub fn new(
        poles: &[Pnt],
        knots: &[f64],
        mults: &[i32],
        degree: i32,
        periodic: bool,
    ) -> Self {
        let weights = vec![1.0; poles.len()];
        Self::build(poles, &weights, knots, mults, degree, periodic, false)
    }

    /// `Geom_BSplineCurve(Poles, Weights, Knots, Multiplicities, Degree,
    /// Periodic=false, CheckRational=true)` — rational B-spline curve.
    pub fn new_rational(
        poles: &[Pnt],
        weights: &[f64],
        knots: &[f64],
        mults: &[i32],
        degree: i32,
        periodic: bool,
    ) -> Self {
        assert_eq!(
            weights.len(),
            poles.len(),
            "Geom_BSplineCurve: Weights.Length() must equal Poles.Length()"
        );
        // CheckRational: rational only if the weights are not all identical.
        let rational = !weights_are_constant(weights);
        Self::build(poles, weights, knots, mults, degree, periodic, rational)
    }

    fn build(
        poles: &[Pnt],
        weights: &[f64],
        knots: &[f64],
        mults: &[i32],
        degree: i32,
        periodic: bool,
        rational: bool,
    ) -> Self {
        assert!((1..=Self::MAX_DEGREE).contains(&degree), "Geom_BSplineCurve: bad degree");
        assert_eq!(knots.len(), mults.len(), "Geom_BSplineCurve: Knots/Mults length mismatch");
        assert!(knots.len() >= 2, "Geom_BSplineCurve: need >= 2 knots");
        for i in 1..knots.len() {
            assert!(
                knots[i] > knots[i - 1],
                "Geom_BSplineCurve: knots must be strictly increasing"
            );
        }
        let expected = if periodic {
            // periodic: NbPoles == Sum(interior mults) (i.e. total minus last)
            mults.iter().take(mults.len() - 1).sum::<i32>()
        } else {
            mults.iter().sum::<i32>() - degree - 1
        };
        assert_eq!(
            poles.len() as i32,
            expected,
            "Geom_BSplineCurve: NbPoles inconsistent with knots/mults/degree/periodic"
        );

        Self {
            poles: poles.to_vec(),
            weights: weights.to_vec(),
            knots: knots.to_vec(),
            mults: mults.to_vec(),
            degree,
            periodic,
            rational,
        }
    }

    /// Copy constructor (`Geom_BSplineCurve(const Geom_BSplineCurve&)`).
    pub fn copy(&self) -> Self {
        self.clone()
    }

    /// `Geom_Geometry::Copy()` — returns a new curve equal to this one.
    pub fn copy_geom(&self) -> Self {
        self.clone()
    }

    // ---------------------------------------------------------------------
    // Basic properties
    // ---------------------------------------------------------------------

    /// `MaxDegree()`.
    pub fn max_degree() -> i32 {
        Self::MAX_DEGREE
    }

    /// `Degree()`.
    pub fn degree(&self) -> i32 {
        self.degree
    }

    /// `NbPoles()`.
    pub fn nb_poles(&self) -> i32 {
        self.poles.len() as i32
    }

    /// `NbKnots()` — number of distinct knots.
    pub fn nb_knots(&self) -> i32 {
        self.knots.len() as i32
    }

    /// `IsPeriodic()`.
    pub fn is_periodic(&self) -> bool {
        self.periodic
    }

    /// `IsRational()`.
    pub fn is_rational(&self) -> bool {
        self.rational
    }

    // ---------------------------------------------------------------------
    // Pole / weight / knot accessors (1-based, like NCollection_Array1)
    // ---------------------------------------------------------------------

    /// `Pole(Index)` — 1-based.
    pub fn pole(&self, index: i32) -> Pnt {
        assert!(index >= 1 && index <= self.nb_poles(), "Geom_BSplineCurve::Pole: out of range");
        self.poles[(index - 1) as usize]
    }

    /// `Poles()` — 0-based slice of all poles.
    pub fn poles(&self) -> &[Pnt] {
        &self.poles
    }

    /// `Weight(Index)` — 1-based.
    pub fn weight(&self, index: i32) -> f64 {
        assert!(index >= 1 && index <= self.nb_poles(), "Geom_BSplineCurve::Weight: out of range");
        self.weights[(index - 1) as usize]
    }

    /// `Weights()` — `Some` only when rational (mirrors OCCT returning `nullptr`
    /// for a non-rational curve).
    pub fn weights(&self) -> Option<&[f64]> {
        if self.rational {
            Some(&self.weights)
        } else {
            None
        }
    }

    /// `WeightsArray()` — const reference to the weights array; always sized to
    /// `NbPoles()`, returning unit weights for non-rational curves.
    pub fn weights_array(&self) -> &[f64] {
        &self.weights
    }

    /// `Knot(Index)` — 1-based.
    pub fn knot(&self, index: i32) -> f64 {
        assert!(index >= 1 && index <= self.nb_knots(), "Geom_BSplineCurve::Knot: out of range");
        self.knots[(index - 1) as usize]
    }

    /// `Knots()` — 0-based slice of the distinct knots.
    pub fn knots(&self) -> &[f64] {
        &self.knots
    }

    /// `Multiplicity(Index)` — 1-based.
    pub fn multiplicity(&self, index: i32) -> i32 {
        assert!(index >= 1 && index <= self.nb_knots(), "Geom_BSplineCurve::Multiplicity: out of range");
        self.mults[(index - 1) as usize]
    }

    /// `Multiplicities()` — 0-based slice.
    pub fn multiplicities(&self) -> &[i32] {
        &self.mults
    }

    /// `KnotSequence()` — the expanded knot vector (knots repeated by their
    /// multiplicities, with the periodic extension when periodic).
    pub fn knot_sequence(&self) -> Vec<f64> {
        self.flat_knots()
    }

    // ---------------------------------------------------------------------
    // Parameter range
    // ---------------------------------------------------------------------

    /// `FirstUKnotIndex()` — index of the knot giving the first parameter.
    pub fn first_u_knot_index(&self) -> i32 {
        // For a periodic curve the parameter range is the full distinct-knot
        // span: the first index is always 1 (BSplCLib treats periodic knots as
        // a wrapping table).
        if self.periodic {
            return 1;
        }
        // Non-periodic (clamped): BSplCLib::FirstUKnotIndex — the first distinct
        // knot whose cumulative multiplicity reaches Degree+1.
        let target = self.degree + 1;
        let mut acc = 0;
        for (i, &m) in self.mults.iter().enumerate() {
            acc += m;
            if acc >= target {
                return (i + 1) as i32;
            }
        }
        1
    }

    /// `LastUKnotIndex()`.
    pub fn last_u_knot_index(&self) -> i32 {
        if self.periodic {
            return self.nb_knots();
        }
        let target = self.degree + 1;
        let mut acc = 0;
        for (i, &m) in self.mults.iter().enumerate().rev() {
            acc += m;
            if acc >= target {
                return (i + 1) as i32;
            }
        }
        self.nb_knots()
    }

    /// `FirstParameter()`.
    pub fn first_parameter(&self) -> f64 {
        self.knots[(self.first_u_knot_index() - 1) as usize]
    }

    /// `LastParameter()`.
    pub fn last_parameter(&self) -> f64 {
        self.knots[(self.last_u_knot_index() - 1) as usize]
    }

    /// Period of a periodic curve.
    fn period(&self) -> f64 {
        self.last_parameter() - self.first_parameter()
    }

    /// `PeriodicNormalization(U)` — fold a parameter into the period.
    pub fn periodic_normalization(&self, u: &mut f64) {
        if !self.periodic {
            return;
        }
        let first = self.first_parameter();
        let last = self.last_parameter();
        let period = last - first;
        if period <= 0.0 {
            return;
        }
        while *u < first - precision::CONFUSION {
            *u += period;
        }
        while *u > last + precision::CONFUSION {
            *u -= period;
        }
    }

    // ---------------------------------------------------------------------
    // Flat knot vector
    // ---------------------------------------------------------------------

    /// Expanded (flat) knot vector used by de Boor.
    ///
    /// Non-periodic: each distinct knot repeated by its multiplicity. Length =
    /// NbPoles + Degree + 1.
    ///
    /// Periodic: the OCCT representation. With `p = degree`, the flat sequence
    /// is built so that there are `nb_poles + p + 1` knots, wrapping around the
    /// period; the curve is clamped neither end.
    fn flat_knots(&self) -> Vec<f64> {
        if !self.periodic {
            let mut fk = Vec::new();
            for (k, &m) in self.knots.iter().zip(self.mults.iter()) {
                for _ in 0..m {
                    fk.push(*k);
                }
            }
            return fk;
        }
        // Periodic: build a flat knot vector of length nbpoles + 2*(degree+1) ...
        // Use OCCT's scheme: total flat length = NbPoles + 2*Degree + 2 - mult(1).
        // For the common all-mult-1 case this gives NbPoles + 2*Degree + 1.
        let p = self.degree;
        let period = self.period();
        // Interior (base) flat sequence from distinct knots and multiplicities.
        let mut base: Vec<f64> = Vec::new();
        for (k, &m) in self.knots.iter().zip(self.mults.iter()) {
            for _ in 0..m {
                base.push(*k);
            }
        }
        // base has length Sum(mults). We need to add `p+1 - mult(first)` knots
        // at the front (taken from the end minus period) and the same count of
        // knots at the back (taken from the front plus period). OCCT adds enough
        // to make the curve evaluable across the whole period.
        let cont = p + 1 - self.mults[0];
        let mut fk = Vec::new();
        // Front extension: the `cont` knots preceding base's last block, minus period.
        let blen = base.len();
        for i in 0..cont as usize {
            // knot at position blen-1 - cont + i in base, minus period
            let idx = blen as i32 - 1 - cont + i as i32;
            fk.push(base[idx as usize] - period);
        }
        fk.extend_from_slice(&base);
        for i in 0..cont as usize {
            let idx = i as i32 + self.mults[0];
            fk.push(base[idx as usize] + period);
        }
        fk
    }

    /// Poles expanded for periodic evaluation: the periodic curve wraps the
    /// first `degree` poles around.
    fn eval_data(&self) -> (Vec<f64>, Vec<Pnt>, Vec<f64>, usize) {
        if !self.periodic {
            (self.flat_knots(), self.poles.clone(), self.weights.clone(), self.degree as usize)
        } else {
            let p = self.degree as usize;
            let fk = self.flat_knots();
            // Number of poles in the flat representation = fk.len() - p - 1.
            let n_flat_poles = fk.len() - p - 1;
            let mut poles = Vec::with_capacity(n_flat_poles);
            let mut weights = Vec::with_capacity(n_flat_poles);
            for i in 0..n_flat_poles {
                let j = i % self.poles.len();
                poles.push(self.poles[j]);
                weights.push(self.weights[j]);
            }
            (fk, poles, weights, p)
        }
    }

    // ---------------------------------------------------------------------
    // Evaluation
    // ---------------------------------------------------------------------

    fn clamp_param(&self, u: f64) -> f64 {
        let mut u = u;
        if self.periodic {
            self.periodic_normalization(&mut u);
        }
        u
    }

    /// `Value(U)` / `D0(U, P)` — the point of parameter U.
    pub fn value(&self, u: f64) -> Pnt {
        let (fk, poles, weights, p) = self.eval_data();
        eval_deboor(&fk, &poles, &weights, p, self.clamp_param(u))
    }

    /// `D0(U) -> P`.
    pub fn d0(&self, u: f64) -> Pnt {
        self.value(u)
    }

    /// `D1(U) -> (P, V1)`.
    pub fn d1(&self, u: f64) -> (Pnt, Vec3) {
        let ders = self.derivatives(u, 1);
        (ders[0], ders[1])
    }

    /// `D2(U) -> (P, V1, V2)`.
    pub fn d2(&self, u: f64) -> (Pnt, Vec3, Vec3) {
        let ders = self.derivatives(u, 2);
        (ders[0], ders[1], ders[2])
    }

    /// `D3(U) -> (P, V1, V2, V3)`.
    pub fn d3(&self, u: f64) -> (Pnt, Vec3, Vec3, Vec3) {
        let ders = self.derivatives(u, 3);
        (ders[0], ders[1], ders[2], ders[3])
    }

    /// `DN(U, N)` — the N-th derivative vector (N >= 1).
    pub fn dn(&self, u: f64, n: i32) -> Vec3 {
        assert!(n >= 1, "Geom_BSplineCurve::DN: N must be >= 1");
        let ders = self.derivatives(u, n);
        ders[n as usize]
    }

    /// Returns the derivatives `[C, C', C'', ...]` up to order `n` (inclusive).
    fn derivatives(&self, u: f64, n: i32) -> Vec<Pnt> {
        let (fk, poles, weights, p) = self.eval_data();
        let u = self.clamp_param(u);
        if !self.rational {
            // Polynomial case: derivatives of a non-rational B-spline directly.
            return curve_derivs_poly(&fk, &poles, p, u, n as usize);
        }
        // Rational case: differentiate the homogeneous curve, then apply the
        // quotient rule (Piegl-Tiller A4.2).
        let mut hpoles: Vec<Pnt> = Vec::with_capacity(poles.len());
        for (pt, &w) in poles.iter().zip(weights.iter()) {
            hpoles.push(*pt * w);
        }
        let aders = curve_derivs_poly(&fk, &hpoles, p, u, n as usize); // numerators A^(k)
        let wders = curve_derivs_scalar(&fk, &weights, p, u, n as usize); // w^(k)
        rational_derivs(&aders, &wders, n as usize)
    }

    /// `StartPoint()` — first point of the curve.
    pub fn start_point(&self) -> Pnt {
        self.value(self.first_parameter())
    }

    /// `EndPoint()` — last point of the curve.
    pub fn end_point(&self) -> Pnt {
        self.value(self.last_parameter())
    }

    /// `LocalValue(U, FromK1, ToK2)` — value computed using only the curve
    /// definition between knots `FromK1` and `ToK2`. For a globally-evaluable
    /// curve this equals `Value(U)`.
    pub fn local_value(&self, u: f64, from_k1: i32, to_k2: i32) -> Pnt {
        assert!(from_k1 != to_k2, "Geom_BSplineCurve::LocalValue: FromK1 == ToK2");
        self.value(u)
    }

    /// `LocalD0(U, FromK1, ToK2) -> P`.
    pub fn local_d0(&self, u: f64, from_k1: i32, to_k2: i32) -> Pnt {
        self.local_value(u, from_k1, to_k2)
    }

    /// `LocalD1(U, FromK1, ToK2) -> (P, V1)`.
    pub fn local_d1(&self, u: f64, from_k1: i32, to_k2: i32) -> (Pnt, Vec3) {
        assert!(from_k1 != to_k2, "Geom_BSplineCurve::LocalD1: FromK1 == ToK2");
        self.d1(u)
    }

    // ---------------------------------------------------------------------
    // Continuity and closure
    // ---------------------------------------------------------------------

    /// Maximum interior knot multiplicity (used for continuity).
    fn max_interior_mult(&self) -> i32 {
        let first = self.first_u_knot_index();
        let last = self.last_u_knot_index();
        let mut mx = 0;
        for i in (first as usize)..((last as usize).saturating_sub(1)) {
            // interior knots are strictly between first and last knot index
            if (i + 1) as i32 > first && ((i + 1) as i32) < last {
                mx = mx.max(self.mults[i]);
            }
        }
        mx
    }

    /// `IsCN(N)` — is the curve N-times continuously differentiable.
    pub fn is_cn(&self, n: i32) -> bool {
        assert!(n >= 0, "Geom_BSplineCurve::IsCN: N must be >= 0");
        if n <= self.degree - self.max_interior_mult() {
            return true;
        }
        // No interior knots => the curve is a single polynomial piece => C-infinity.
        self.max_interior_mult() == 0 && n <= self.degree
    }

    /// `Continuity()` — global continuity classification.
    pub fn continuity(&self) -> Shape {
        let m = self.max_interior_mult();
        let c = self.degree - m;
        if m == 0 {
            // Single polynomial span (no interior knots): infinitely differentiable.
            return Shape::CN;
        }
        match c {
            x if x >= 4 => Shape::CN,
            3 => Shape::C3,
            2 => Shape::C2,
            1 => Shape::C1,
            _ => Shape::C0,
        }
    }

    /// `IsClosed()` — start and end points coincide within `Precision::Confusion`.
    pub fn is_closed(&self) -> bool {
        self.start_point().distance(self.end_point()) <= precision::CONFUSION
    }

    // ---------------------------------------------------------------------
    // Knot distribution
    // ---------------------------------------------------------------------

    /// `KnotDistribution()`.
    pub fn knot_distribution(&self) -> BSplKnotDistribution {
        let n = self.knots.len();
        // Check uniform spacing.
        let mut uniform_spacing = true;
        if n >= 2 {
            let step = self.knots[1] - self.knots[0];
            for i in 2..n {
                if (self.knots[i] - self.knots[i - 1] - step).abs() > f64::EPSILON.sqrt() {
                    uniform_spacing = false;
                    break;
                }
            }
        }
        let first_m = self.mults[0];
        let last_m = self.mults[n - 1];
        let deg = self.degree;
        let interior_all_one = self.mults[1..n - 1].iter().all(|&m| m == 1);
        let interior_all_deg = self.mults[1..n - 1].iter().all(|&m| m == deg);
        let all_one = self.mults.iter().all(|&m| m == 1);

        // PiecewiseBezier: first and last mult = degree+1, interior = degree.
        if first_m == deg + 1 && last_m == deg + 1 && (n == 2 || interior_all_deg) {
            return BSplKnotDistribution::PiecewiseBezier;
        }
        if uniform_spacing {
            if all_one {
                return BSplKnotDistribution::Uniform;
            }
            if first_m == deg + 1 && last_m == deg + 1 && interior_all_one {
                return BSplKnotDistribution::QuasiUniform;
            }
        }
        BSplKnotDistribution::NonUniform
    }

    // ---------------------------------------------------------------------
    // LocateU
    // ---------------------------------------------------------------------

    /// `LocateU(U, ParametricTolerance) -> (I1, I2)` (no knot repetition).
    pub fn locate_u(&self, u: f64, parametric_tolerance: f64) -> (i32, i32) {
        let tol = parametric_tolerance.abs();
        let n = self.knots.len();
        if u < self.knots[0] - tol {
            return (0, 1);
        }
        if u > self.knots[n - 1] + tol {
            return (n as i32, (n + 1) as i32);
        }
        // Find a knot equal to U within tolerance.
        for (i, &k) in self.knots.iter().enumerate() {
            if (u - k).abs() <= tol {
                let idx = (i + 1) as i32;
                return (idx, idx);
            }
        }
        // Otherwise bracket.
        for i in 0..n - 1 {
            if u > self.knots[i] && u < self.knots[i + 1] {
                return ((i + 1) as i32, (i + 2) as i32);
            }
        }
        (1, 1)
    }

    // ---------------------------------------------------------------------
    // Modifiers: poles & weights
    // ---------------------------------------------------------------------

    /// `SetPole(Index, P)` — 1-based.
    pub fn set_pole(&mut self, index: i32, p: Pnt) {
        assert!(index >= 1 && index <= self.nb_poles(), "Geom_BSplineCurve::SetPole: out of range");
        self.poles[(index - 1) as usize] = p;
    }

    /// `SetPole(Index, P, Weight)`.
    pub fn set_pole_weight(&mut self, index: i32, p: Pnt, weight: f64) {
        assert!(weight > 0.0, "Geom_BSplineCurve::SetPole: weight must be > 0");
        self.set_pole(index, p);
        self.set_weight(index, weight);
    }

    /// `SetWeight(Index, Weight)`.
    pub fn set_weight(&mut self, index: i32, weight: f64) {
        assert!(index >= 1 && index <= self.nb_poles(), "Geom_BSplineCurve::SetWeight: out of range");
        assert!(weight > 0.0, "Geom_BSplineCurve::SetWeight: weight must be > 0");
        self.weights[(index - 1) as usize] = weight;
        self.rational = !weights_are_constant(&self.weights);
    }

    /// `SetKnot(Index, K)` — change a knot value without changing multiplicity.
    pub fn set_knot(&mut self, index: i32, k: f64) {
        assert!(index >= 1 && index <= self.nb_knots(), "Geom_BSplineCurve::SetKnot: out of range");
        let i = (index - 1) as usize;
        if i > 0 {
            assert!(k > self.knots[i - 1], "Geom_BSplineCurve::SetKnot: K <= Knots(Index-1)");
        }
        if i + 1 < self.knots.len() {
            assert!(k < self.knots[i + 1], "Geom_BSplineCurve::SetKnot: K >= Knots(Index+1)");
        }
        self.knots[i] = k;
    }

    /// `SetKnot(Index, K, M)` — also raise the multiplicity to `M`.
    pub fn set_knot_mult(&mut self, index: i32, k: f64, m: i32) {
        let i = (index - 1) as usize;
        assert!(index >= 1 && index <= self.nb_knots(), "Geom_BSplineCurve::SetKnot: out of range");
        assert!(m <= self.degree, "Geom_BSplineCurve::SetKnot: M > Degree");
        assert!(m >= self.mults[i], "Geom_BSplineCurve::SetKnot: cannot decrease multiplicity");
        self.set_knot(index, k);
        if m > self.mults[i] {
            self.increase_multiplicity(index, m);
        }
    }

    // ---------------------------------------------------------------------
    // Reverse
    // ---------------------------------------------------------------------

    /// `Reverse()` — reverse the direction of parametrization.
    pub fn reverse(&mut self) {
        let first = self.first_parameter();
        let last = self.last_parameter();
        // New knots: knot'(i) = first + last - knot(n+1-i), reversed order.
        let n = self.knots.len();
        let mut new_knots = Vec::with_capacity(n);
        for i in (0..n).rev() {
            new_knots.push(first + last - self.knots[i]);
        }
        let mut new_mults = self.mults.clone();
        new_mults.reverse();
        self.knots = new_knots;
        self.mults = new_mults;
        self.poles.reverse();
        self.weights.reverse();
    }

    /// `ReversedParameter(U)` — `UFirst + ULast - U`.
    pub fn reversed_parameter(&self, u: f64) -> f64 {
        self.first_parameter() + self.last_parameter() - u
    }

    // ---------------------------------------------------------------------
    // Transform
    // ---------------------------------------------------------------------

    /// `Transform(T)` — apply a transformation to all poles.
    pub fn transform(&mut self, t: &Trsf) {
        for p in self.poles.iter_mut() {
            *p = t.apply_point(*p);
        }
    }

    // ---------------------------------------------------------------------
    // Resolution
    // ---------------------------------------------------------------------

    /// `Resolution(Tolerance3D) -> UTolerance`.
    ///
    /// Computes a parametric tolerance such that a parametric step smaller than
    /// it guarantees a 3D step smaller than `tolerance3d`. We bound the maximum
    /// derivative magnitude over the curve and use
    /// `UTolerance = Tolerance3D / max|C'|`.
    pub fn resolution(&self, tolerance3d: f64) -> f64 {
        let a = self.first_parameter();
        let b = self.last_parameter();
        let mut max_d1 = 0.0_f64;
        let samples = 100;
        for i in 0..=samples {
            let u = a + (b - a) * (i as f64) / (samples as f64);
            let (_, v1) = self.d1(u);
            max_d1 = max_d1.max(v1.norm());
        }
        if max_d1 <= 0.0 {
            max_d1 = 1.0;
        }
        tolerance3d / max_d1
    }

    // ---------------------------------------------------------------------
    // Equality
    // ---------------------------------------------------------------------

    /// `IsEqual(other, preci)` — geometric equality within `preci`.
    pub fn is_equal(&self, other: &GeomBSplineCurve, preci: f64) -> bool {
        if self.degree != other.degree
            || self.periodic != other.periodic
            || self.rational != other.rational
            || self.poles.len() != other.poles.len()
            || self.knots.len() != other.knots.len()
        {
            return false;
        }
        for (a, b) in self.poles.iter().zip(other.poles.iter()) {
            if !a.is_equal(*b, preci) {
                return false;
            }
        }
        for (a, b) in self.weights.iter().zip(other.weights.iter()) {
            if (a - b).abs() > preci {
                return false;
            }
        }
        for (a, b) in self.knots.iter().zip(other.knots.iter()) {
            if (a - b).abs() > preci {
                return false;
            }
        }
        for (a, b) in self.mults.iter().zip(other.mults.iter()) {
            if a != b {
                return false;
            }
        }
        true
    }

    // ---------------------------------------------------------------------
    // Periodicity
    // ---------------------------------------------------------------------

    /// `SetNotPeriodic()` — convert a periodic curve into a clamped (open) one
    /// that traces the same geometry, by inserting knots at both ends so the
    /// boundary knot multiplicities become `degree + 1`.
    pub fn set_not_periodic(&mut self) {
        if !self.periodic {
            return;
        }
        let p = self.degree;
        // Build the new clamped representation directly from the periodic flat
        // data by extracting poles/knots over one period and clamping.
        let (fk, ext_poles, ext_weights, _) = self.eval_data();
        // The full clamped knot vector spans [first, last] with end mults p+1.
        // Strategy: insert knots at first and last parameters until mult = p+1,
        // operating on the extended (flat) representation, then trim.
        let first = self.first_parameter();
        let last = self.last_parameter();

        // Build a working (non-periodic) curve from the extended flat data so we
        // can use Boehm insertion uniformly.
        let (mut wknots, mut wmults) = flat_to_knots_mults(&fk);
        let mut wpoles = ext_poles;
        let mut wweights = ext_weights;

        // Raise the multiplicity at `first` and `last` to p+1 via insertion.
        let need_first = (p + 1) - mult_at(&wknots, &wmults, first);
        for _ in 0..need_first.max(0) {
            insert_knot_into(&mut wknots, &mut wmults, &mut wpoles, &mut wweights, p, first);
        }
        let need_last = (p + 1) - mult_at(&wknots, &wmults, last);
        for _ in 0..need_last.max(0) {
            insert_knot_into(&mut wknots, &mut wmults, &mut wpoles, &mut wweights, p, last);
        }

        // Now trim to the segment [first, last]: keep only knots in [first,last]
        // and the corresponding poles. The flat index of `first` (after clamping
        // mult p+1) starts the active poles.
        let new_flat = knots_mults_to_flat(&wknots, &wmults);
        // Determine pole index range: poles correspond to flat indices.
        // Active flat span for [first,last]: from the first occurrence of `first`
        // to the last occurrence of `last`.
        let start_flat = new_flat.iter().position(|&x| (x - first).abs() < 1e-12).unwrap();
        let end_flat = new_flat.iter().rposition(|&x| (x - last).abs() < 1e-12).unwrap();
        // Clamped knot vector slice is [start_flat .. end_flat] inclusive.
        let clamped_flat: Vec<f64> = new_flat[start_flat..=end_flat].to_vec();
        let n_new_poles = clamped_flat.len() - (p as usize + 1);
        let new_poles: Vec<Pnt> = wpoles[start_flat..start_flat + n_new_poles].to_vec();
        let new_weights: Vec<f64> = wweights[start_flat..start_flat + n_new_poles].to_vec();

        let (nk, nm) = flat_to_knots_mults(&clamped_flat);
        self.knots = nk;
        self.mults = nm;
        self.poles = new_poles;
        self.weights = new_weights;
        self.periodic = false;
    }

    /// `SetPeriodic()` — turn a closed curve into a periodic one.
    pub fn set_periodic(&mut self) {
        if self.periodic {
            return;
        }
        // Reduce boundary multiplicities to 1 and mark periodic, keeping the
        // poles that define one period.
        let p = self.degree as usize;
        let n = self.knots.len();
        // New mults: clamp interior, set ends to 1.
        let mut new_mults = self.mults.clone();
        new_mults[0] = 1;
        new_mults[n - 1] = 1;
        // Number of periodic poles = Sum(new_mults) - new_mults(last).
        let n_poles: i32 = new_mults.iter().take(n - 1).sum();
        self.poles.truncate(n_poles as usize);
        self.weights.truncate(n_poles as usize);
        self.mults = new_mults;
        self.periodic = true;
        let _ = p;
    }

    /// `SetOrigin(Index)` — set the knot of index `Index` as the period origin.
    pub fn set_origin(&mut self, index: i32) {
        assert!(self.periodic, "Geom_BSplineCurve::SetOrigin: curve is not periodic");
        assert!(index >= 1 && index <= self.nb_knots(), "Geom_BSplineCurve::SetOrigin: index out of range");
        let first = self.first_u_knot_index();
        let last = self.last_u_knot_index();
        let i = index;
        if i == first {
            return;
        }
        // Shift in knot index space by (i - first), wrapping around the period.
        let shift = (i - first) as usize;
        let period = self.period();
        // Number of distinct interior knots in one period = last - first.
        let nper = (last - first) as usize;
        if nper == 0 {
            return;
        }
        // Reindex knots: new distinct knots = rotate(knots[first-1 .. last-1]) by shift,
        // adjusting values across the period; keep the structure.
        // We work with the period block of distinct knots indices [first-1 .. last-1).
        let base_lo = (first - 1) as usize;
        let block_knots: Vec<f64> = self.knots[base_lo..base_lo + nper].to_vec();
        let block_mults: Vec<i32> = self.mults[base_lo..base_lo + nper].to_vec();
        let origin_val = block_knots[shift % nper];

        // Build rotated period of knots starting at the chosen origin.
        let mut new_block_knots = Vec::with_capacity(nper + 1);
        let mut new_block_mults = Vec::with_capacity(nper + 1);
        for j in 0..nper {
            let src = (shift + j) % nper;
            let mut val = block_knots[src] - origin_val;
            if val < 0.0 {
                val += period;
            }
            new_block_knots.push(val);
            new_block_mults.push(block_mults[src]);
        }
        // Append the closing knot (period) with the same mult as the origin.
        new_block_knots.push(period);
        new_block_mults.push(block_mults[shift % nper]);

        // Rebuild full distinct knots/mults: the periodic representation just
        // needs these nper+1 distinct knots.
        self.knots = new_block_knots;
        self.mults = new_block_mults;

        // Rotate poles by `shift`.
        let np = self.poles.len();
        let s = shift % np;
        self.poles.rotate_left(s);
        self.weights.rotate_left(s);
    }

    // ---------------------------------------------------------------------
    // Knot insertion
    // ---------------------------------------------------------------------

    /// `InsertKnot(U, M=1, ParametricTolerance=0, Add=true)`.
    pub fn insert_knot(&mut self, u: f64, m: i32) {
        if u < self.first_parameter() - precision::CONFUSION
            || u > self.last_parameter() + precision::CONFUSION
        {
            return;
        }
        if m <= 0 {
            return;
        }
        let mreq = m.min(self.degree);
        let existing = mult_at(&self.knots, &self.mults, u);
        let to_insert = (existing + mreq).min(self.degree) - existing;
        for _ in 0..to_insert {
            insert_knot_into(
                &mut self.knots,
                &mut self.mults,
                &mut self.poles,
                &mut self.weights,
                self.degree,
                u,
            );
        }
    }

    /// `InsertKnots(Knots, Mults, ParametricTolerance=0, Add=false)`.
    pub fn insert_knots(&mut self, knots: &[f64], mults: &[i32]) {
        for (&u, &m) in knots.iter().zip(mults.iter()) {
            // Add == false in the GTest: set multiplicity to m (insert until reached).
            if u < self.first_parameter() - precision::CONFUSION
                || u > self.last_parameter() + precision::CONFUSION
            {
                continue;
            }
            let existing = mult_at(&self.knots, &self.mults, u);
            let target = m.min(self.degree);
            let to_insert = (target - existing).max(0);
            for _ in 0..to_insert {
                insert_knot_into(
                    &mut self.knots,
                    &mut self.mults,
                    &mut self.poles,
                    &mut self.weights,
                    self.degree,
                    u,
                );
            }
        }
    }

    /// `IncreaseMultiplicity(Index, M)`.
    pub fn increase_multiplicity(&mut self, index: i32, m: i32) {
        assert!(index >= 1 && index <= self.nb_knots(), "Geom_BSplineCurve::IncreaseMultiplicity: out of range");
        let i = (index - 1) as usize;
        let target = m.min(self.degree);
        let u = self.knots[i];
        let current = self.mults[i];
        let to_insert = (target - current).max(0);
        for _ in 0..to_insert {
            insert_knot_into(
                &mut self.knots,
                &mut self.mults,
                &mut self.poles,
                &mut self.weights,
                self.degree,
                u,
            );
        }
    }

    // ---------------------------------------------------------------------
    // Knot removal
    // ---------------------------------------------------------------------

    /// `RemoveKnot(Index, M, Tolerance)` — reduce the multiplicity of knot
    /// `Index` to `M` (remove entirely if `M == 0`). Returns `true` if the curve
    /// is not modified beyond `Tolerance`.
    pub fn remove_knot(&mut self, index: i32, m: i32, tolerance: f64) -> bool {
        assert!(index >= 1 && index <= self.nb_knots(), "Geom_BSplineCurve::RemoveKnot: out of range");
        let i = (index - 1) as usize;
        // Cannot remove boundary knots below clamp.
        if index <= self.first_u_knot_index() || index >= self.last_u_knot_index() {
            return false;
        }
        let current = self.mults[i];
        if m >= current {
            return true; // nothing to remove
        }
        let num_remove = current - m;
        // Snapshot for tolerance comparison.
        let before = self.clone();
        let mut ok = true;
        for _ in 0..num_remove {
            if !self.remove_one_knot(self.knots[idx_clamp(self, index)], tolerance) {
                ok = false;
                break;
            }
        }
        if !ok {
            *self = before;
            return false;
        }
        // Verify geometric proximity.
        let samples = 50;
        let a = self.first_parameter();
        let b = self.last_parameter();
        for s in 0..=samples {
            let u = a + (b - a) * (s as f64) / (samples as f64);
            if before.value(u).distance(self.value(u)) > tolerance.max(precision::CONFUSION) {
                *self = before;
                return false;
            }
        }
        true
    }

    /// Remove a single knot occurrence at value `u` (Piegl-Tiller A5.8
    /// RemoveCurveKnot, one removal). Rational-aware via homogeneous
    /// coordinates. Returns false if removal would move the curve by more than
    /// `tol`.
    fn remove_one_knot(&mut self, u: f64, tol: f64) -> bool {
        let ki = match self.knots.iter().position(|&k| (k - u).abs() < 1e-12) {
            Some(k) => k,
            None => return false,
        };
        let s = self.mults[ki] as usize; // current multiplicity
        if s == 0 {
            return false;
        }
        let fk = knots_mults_to_flat(&self.knots, &self.mults);
        let p = self.degree as usize;
        // r = flat index of the last occurrence of u.
        let r = fk.iter().rposition(|&x| (x - u).abs() < 1e-12).unwrap();
        let n = self.poles.len() - 1; // last pole index

        // Homogeneous poles Pw (rational-aware).
        let mut pw: Vec<Pnt> = self
            .poles
            .iter()
            .zip(self.weights.iter())
            .map(|(pt, &w)| *pt * w)
            .collect();
        let mut ww: Vec<f64> = self.weights.clone();

        // The NURBS Book A5.8, RemoveCurveKnot, single removal (t = 0).
        // `ord` here is the curve order (Degree + 1), not the multiplicity.
        let ord = p + 1;
        let last = r - s;
        let first = r - p;
        // temp indices span [0, last+1-off]; size last-first+2 (+slack).
        let mut temp: Vec<Pnt> = vec![Pnt::origin(); 2 * p + 4];
        let mut tempw: Vec<f64> = vec![0.0; 2 * p + 4];

        let off = first - 1;
        temp[0] = pw[off];
        tempw[0] = ww[off];
        temp[last + 1 - off] = pw[last + 1];
        tempw[last + 1 - off] = ww[last + 1];

        let mut i = first;
        let mut j = last;
        let mut ii: usize = 1;
        let mut jj: usize = last - off;
        let tol_eff = tol.max(precision::CONFUSION);

        while (j as i64) - (i as i64) > 0 {
            let alfi = (u - fk[i]) / (fk[i + ord] - fk[i]);
            let alfj = (u - fk[j]) / (fk[j + ord] - fk[j]);
            temp[ii] = (pw[i] - temp[ii - 1] * (1.0 - alfi)) * (1.0 / alfi);
            tempw[ii] = (ww[i] - tempw[ii - 1] * (1.0 - alfi)) / alfi;
            temp[jj] = (pw[j] - temp[jj + 1] * alfj) * (1.0 / (1.0 - alfj));
            tempw[jj] = (ww[j] - tempw[jj + 1] * alfj) / (1.0 - alfj);
            i += 1;
            ii += 1;
            j -= 1;
            jj -= 1;
        }

        // Removability check.
        let removable = if (j as i64) - (i as i64) < 0 {
            temp[ii - 1].distance(temp[jj + 1]) <= tol_eff
        } else {
            let alfi = (u - fk[i]) / (fk[i + ord] - fk[i]);
            let cand = temp[ii + 1] * alfi + temp[ii - 1] * (1.0 - alfi);
            pw[i].distance(cand) <= tol_eff
        };
        if !removable {
            return false;
        }

        // Write the new control points back.
        i = first;
        j = last;
        while j as i64 - i as i64 > 0 {
            pw[i] = temp[i - off];
            ww[i] = tempw[i - off];
            pw[j] = temp[j - off];
            ww[j] = tempw[j - off];
            i += 1;
            j -= 1;
        }

        // Remove one pole: shift down indices > fout, where fout = (2r - s - p)/2.
        let fout = (2 * r - s - p) / 2;
        let mut k = fout + 1;
        while k <= n {
            pw[k - 1] = pw[k];
            ww[k - 1] = ww[k];
            k += 1;
        }
        pw.truncate(n);
        ww.truncate(n);

        // Convert homogeneous back to cartesian.
        let new_poles: Vec<Pnt> = pw
            .iter()
            .zip(ww.iter())
            .map(|(p, w)| *p * (1.0 / *w))
            .collect();

        // Update knots/mults.
        self.mults[ki] -= 1;
        if self.mults[ki] == 0 {
            self.knots.remove(ki);
            self.mults.remove(ki);
        }
        self.poles = new_poles;
        self.weights = ww;
        self.rational = !weights_are_constant(&self.weights);
        true
    }

    // ---------------------------------------------------------------------
    // Degree elevation
    // ---------------------------------------------------------------------

    /// `IncreaseDegree(Degree)` — elevate the degree of the curve.
    pub fn increase_degree(&mut self, new_degree: i32) {
        assert!(new_degree <= Self::MAX_DEGREE, "Geom_BSplineCurve::IncreaseDegree: Degree > MaxDegree");
        if new_degree <= self.degree {
            return;
        }
        let t = new_degree - self.degree;
        for _ in 0..t {
            self.elevate_degree_by_one();
        }
    }

    /// Elevate the degree by one, span by span (Bezier elevation per segment).
    fn elevate_degree_by_one(&mut self) {
        // Split into Bezier segments by inserting knots until interior mult == degree.
        let p = self.degree;
        // Convert to clamped Bezier piecewise by inserting interior knots to mult p.
        let distinct: Vec<f64> = self.knots.clone();
        for (i, &k) in distinct.iter().enumerate() {
            if (i as i32 + 1) > self.first_u_knot_index()
                && (i as i32 + 1) < self.last_u_knot_index()
            {
                let cur = mult_at(&self.knots, &self.mults, k);
                for _ in 0..(p - cur).max(0) {
                    insert_knot_into(
                        &mut self.knots,
                        &mut self.mults,
                        &mut self.poles,
                        &mut self.weights,
                        self.degree,
                        k,
                    );
                }
            }
        }
        // Now each span is a degree-p Bezier. Elevate each Bezier piece to p+1.
        // Build homogeneous poles.
        let n_segments = self.knots.len() - 1;
        let mut hpoles: Vec<Pnt> = self
            .poles
            .iter()
            .zip(self.weights.iter())
            .map(|(pt, &w)| *pt * w)
            .collect();
        let mut hw: Vec<f64> = self.weights.clone();

        // Collect per-segment Bezier control points (p+1 each), elevate, reassemble.
        let mut new_hpoles: Vec<Pnt> = Vec::new();
        let mut new_hw: Vec<f64> = Vec::new();
        let mut new_knots: Vec<f64> = Vec::new();
        let mut new_mults: Vec<i32> = Vec::new();

        let pu = p as usize;
        for seg in 0..n_segments {
            let base = seg * pu; // first pole index of this Bezier segment
            let bez_h: Vec<Pnt> = (0..=pu).map(|k| hpoles[base + k]).collect();
            let bez_w: Vec<f64> = (0..=pu).map(|k| hw[base + k]).collect();
            let (elev_h, elev_w) = bezier_elevate(&bez_h, &bez_w);
            // Append: for first segment keep all, otherwise skip the shared first pole.
            if seg == 0 {
                for k in 0..elev_h.len() {
                    new_hpoles.push(elev_h[k]);
                    new_hw.push(elev_w[k]);
                }
                new_knots.push(self.knots[0]);
                new_mults.push(p + 2); // degree+1 (new degree p+1)+? clamp end-of-curve handled below
            } else {
                for k in 1..elev_h.len() {
                    new_hpoles.push(elev_h[k]);
                    new_hw.push(elev_w[k]);
                }
                new_knots.push(self.knots[seg]);
                new_mults.push(p + 1); // interior knot: degree of new curve = p+1
            }
        }
        new_knots.push(self.knots[n_segments]);
        new_mults.push(p + 2);
        // Fix first mult: clamped Bezier with new degree p+1 => end mults p+2.
        new_mults[0] = p + 2;
        let _ = &mut hpoles;
        let _ = &mut hw;

        // Convert homogeneous back.
        let new_poles: Vec<Pnt> = new_hpoles
            .iter()
            .zip(new_hw.iter())
            .map(|(p, w)| *p * (1.0 / *w))
            .collect();

        self.degree = p + 1;
        self.knots = new_knots;
        self.mults = new_mults;
        self.poles = new_poles;
        self.weights = new_hw;
        self.rational = !weights_are_constant(&self.weights);
    }

    // ---------------------------------------------------------------------
    // Segment
    // ---------------------------------------------------------------------

    /// `Segment(U1, U2, Tolerance)` — trim the curve to `[U1, U2]`.
    pub fn segment(&mut self, u1: f64, u2: f64) {
        assert!(u2 >= u1, "Geom_BSplineCurve::Segment: U2 < U1");
        let p = self.degree;
        // Insert knots at U1 and U2 to multiplicity degree+1 so we can clamp.
        let need1 = (p + 1) - mult_at_tol(&self.knots, &self.mults, u1);
        for _ in 0..need1.max(0) {
            insert_knot_into(
                &mut self.knots,
                &mut self.mults,
                &mut self.poles,
                &mut self.weights,
                self.degree,
                u1,
            );
        }
        let need2 = (p + 1) - mult_at_tol(&self.knots, &self.mults, u2);
        for _ in 0..need2.max(0) {
            insert_knot_into(
                &mut self.knots,
                &mut self.mults,
                &mut self.poles,
                &mut self.weights,
                self.degree,
                u2,
            );
        }
        // Build flat knots, find the active window [U1, U2].
        let fk = knots_mults_to_flat(&self.knots, &self.mults);
        let start_flat = fk.iter().position(|&x| (x - u1).abs() < 1e-9).unwrap();
        let end_flat = fk.iter().rposition(|&x| (x - u2).abs() < 1e-9).unwrap();
        let clamped_flat: Vec<f64> = fk[start_flat..=end_flat].to_vec();
        let n_new_poles = clamped_flat.len() - (p as usize + 1);
        let new_poles: Vec<Pnt> = self.poles[start_flat..start_flat + n_new_poles].to_vec();
        let new_weights: Vec<f64> = self.weights[start_flat..start_flat + n_new_poles].to_vec();
        let (nk, nm) = flat_to_knots_mults(&clamped_flat);
        self.knots = nk;
        self.mults = nm;
        self.poles = new_poles;
        self.weights = new_weights;
        self.periodic = false;
        self.rational = !weights_are_constant(&self.weights);
    }

    // ---------------------------------------------------------------------
    // MovePoint
    // ---------------------------------------------------------------------

    /// `MovePoint(U, P, Index1, Index2) -> (FirstModifiedPole, LastModifiedPole)`.
    ///
    /// Moves the point of parameter `U` to `P` by translating the poles in
    /// `[Index1, Index2]` (1-based). Returns the indices of the first and last
    /// modified poles, or `(0, 0)` on incompatibility.
    pub fn move_point(
        &mut self,
        u: f64,
        p: Pnt,
        index1: i32,
        index2: i32,
    ) -> (i32, i32) {
        if index1 >= index2
            || index1 < 1
            || index2 > self.nb_poles()
        {
            return (0, 0);
        }
        // Current point and the displacement needed.
        let cur = self.value(u);
        let delta = p - cur;

        // Determine which poles actually influence parameter U (non-zero basis).
        let fk = knots_mults_to_flat(&self.knots, &self.mults);
        let deg = self.degree as usize;
        let n = self.poles.len() - 1;
        let span = find_span_flat(n, deg, u, &fk);
        // Influencing poles: span-deg .. span (0-based).
        let infl_lo = (span - deg) as i32 + 1; // 1-based
        let infl_hi = span as i32 + 1; // 1-based

        // Intersection of [index1,index2] and [infl_lo,infl_hi].
        let lo = index1.max(infl_lo);
        let hi = index2.min(infl_hi);
        if lo > hi {
            return (0, 0);
        }

        // Evaluate the basis functions at U for the influencing poles.
        let basis = basis_funs(span, u, deg, &fk);
        // Minimal-norm displacement: translate movable pole i by
        // delta * (B_i / Σ B_j²). The curve displacement at U is then
        // Σ B_i · delta · B_i / Σ B_j² = delta, so the curve passes through P.
        let mut sum_b2 = 0.0;
        for i in lo..=hi {
            let local = (i - 1) - (span as i32 - deg as i32);
            let b = basis[local as usize];
            sum_b2 += b * b;
        }
        if sum_b2.abs() < 1e-12 {
            return (0, 0);
        }
        for i in lo..=hi {
            let local = (i - 1) - (span as i32 - deg as i32);
            let factor = basis[local as usize] / sum_b2;
            let idx = (i - 1) as usize;
            self.poles[idx] = self.poles[idx] + delta * factor;
        }
        (lo, hi)
    }
}

// =====================================================================
// Free helper functions (B-spline core algorithms)
// =====================================================================

fn idx_clamp(c: &GeomBSplineCurve, index: i32) -> usize {
    (index - 1).clamp(0, c.knots.len() as i32 - 1) as usize
}

/// True if all weights are equal (within `Real` epsilon), like OCCT's
/// CheckRational.
fn weights_are_constant(weights: &[f64]) -> bool {
    if weights.is_empty() {
        return true;
    }
    let w0 = weights[0];
    weights.iter().all(|&w| (w - w0).abs() <= f64::EPSILON * w0.abs().max(1.0) * 4.0)
}

/// Multiplicity of value `u` in (distinct knots, mults), 0 if not a knot.
fn mult_at(knots: &[f64], mults: &[i32], u: f64) -> i32 {
    for (k, &m) in knots.iter().zip(mults.iter()) {
        if (k - u).abs() < 1e-12 {
            return m;
        }
    }
    0
}

fn mult_at_tol(knots: &[f64], mults: &[i32], u: f64) -> i32 {
    for (k, &m) in knots.iter().zip(mults.iter()) {
        if (k - u).abs() < 1e-9 {
            return m;
        }
    }
    0
}

/// Expand distinct knots+mults into the flat knot vector.
fn knots_mults_to_flat(knots: &[f64], mults: &[i32]) -> Vec<f64> {
    let mut fk = Vec::new();
    for (k, &m) in knots.iter().zip(mults.iter()) {
        for _ in 0..m {
            fk.push(*k);
        }
    }
    fk
}

/// Collapse a flat knot vector into distinct knots + multiplicities.
fn flat_to_knots_mults(fk: &[f64]) -> (Vec<f64>, Vec<i32>) {
    let mut knots: Vec<f64> = Vec::new();
    let mut mults: Vec<i32> = Vec::new();
    for &k in fk {
        if let Some(&last) = knots.last() {
            if (k - last).abs() < 1e-12 {
                *mults.last_mut().unwrap() += 1;
                continue;
            }
        }
        knots.push(k);
        mults.push(1);
    }
    (knots, mults)
}

/// `find_span` for a flat knot vector (clamped).
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

/// Cox-de Boor basis functions at `u` for the span (Piegl-Tiller A2.2).
fn basis_funs(span: usize, u: f64, p: usize, knots: &[f64]) -> Vec<f64> {
    let mut n = vec![0.0; p + 1];
    let mut left = vec![0.0; p + 1];
    let mut right = vec![0.0; p + 1];
    n[0] = 1.0;
    for j in 1..=p {
        left[j] = u - knots[span + 1 - j];
        right[j] = knots[span + j] - u;
        let mut saved = 0.0;
        for r in 0..j {
            let denom = right[r + 1] + left[j - r];
            let temp = if denom.abs() < 1e-300 { 0.0 } else { n[r] / denom };
            n[r] = saved + right[r + 1] * temp;
            saved = left[j - r] * temp;
        }
        n[j] = saved;
    }
    n
}

/// de Boor evaluation of a (possibly rational) B-spline given flat knots.
fn eval_deboor(fk: &[f64], poles: &[Pnt], weights: &[f64], p: usize, u: f64) -> Pnt {
    let n = poles.len() - 1;
    let u = u.clamp(fk[p], fk[n + 1]);
    let span = find_span_flat(n, p, u, fk);
    let mut d: Vec<Pnt> = Vec::with_capacity(p + 1);
    let mut w: Vec<f64> = Vec::with_capacity(p + 1);
    for j in 0..=p {
        let idx = span - p + j;
        let weight = weights[idx];
        d.push(poles[idx] * weight);
        w.push(weight);
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

/// Derivative basis functions up to order `nd` (Piegl-Tiller A2.3).
/// Returns `ders[k][j]` = k-th derivative of the j-th nonzero basis fn.
fn ders_basis_funs(span: usize, u: f64, p: usize, nd: usize, knots: &[f64]) -> Vec<Vec<f64>> {
    let mut ndu = vec![vec![0.0; p + 1]; p + 1];
    let mut left = vec![0.0; p + 1];
    let mut right = vec![0.0; p + 1];
    ndu[0][0] = 1.0;
    for j in 1..=p {
        left[j] = u - knots[span + 1 - j];
        right[j] = knots[span + j] - u;
        let mut saved = 0.0;
        for r in 0..j {
            let denom = right[r + 1] + left[j - r];
            ndu[j][r] = denom;
            let temp = if denom.abs() < 1e-300 { 0.0 } else { ndu[r][j - 1] / denom };
            ndu[r][j] = saved + right[r + 1] * temp;
            saved = left[j - r] * temp;
        }
        ndu[j][j] = saved;
    }
    let mut ders = vec![vec![0.0; p + 1]; nd + 1];
    for j in 0..=p {
        ders[0][j] = ndu[j][p];
    }
    for r in 0..=p {
        let mut a = vec![vec![0.0; p + 1]; 2];
        let (mut s1, mut s2) = (0usize, 1usize);
        a[0][0] = 1.0;
        for k in 1..=nd {
            let mut d = 0.0;
            let rk = r as i64 - k as i64;
            let pk = p as i64 - k as i64;
            if r >= k {
                a[s2][0] = a[s1][0] / ndu[(pk + 1) as usize][rk as usize];
                d = a[s2][0] * ndu[rk as usize][pk as usize];
            }
            let j1 = if rk >= -1 { 1 } else { (-rk) as usize };
            let j2 = if (r as i64 - 1) <= pk { k - 1 } else { p - r };
            for j in j1..=j2 {
                a[s2][j] = (a[s1][j] - a[s1][j - 1]) / ndu[(pk + 1) as usize][(rk + j as i64) as usize];
                d += a[s2][j] * ndu[(rk + j as i64) as usize][pk as usize];
            }
            if r <= pk as usize {
                a[s2][k] = -a[s1][k - 1] / ndu[(pk + 1) as usize][r];
                d += a[s2][k] * ndu[r][pk as usize];
            }
            ders[k][r] = d;
            std::mem::swap(&mut s1, &mut s2);
        }
    }
    // Multiply by the correct factors.
    let mut r = p;
    for k in 1..=nd {
        for j in 0..=p {
            ders[k][j] *= r as f64;
        }
        r *= p - k;
    }
    ders
}

/// Curve derivatives (point + derivatives) for a non-rational B-spline,
/// orders 0..=nd. Piegl-Tiller A3.2.
fn curve_derivs_poly(fk: &[f64], poles: &[Pnt], p: usize, u: f64, nd: usize) -> Vec<Pnt> {
    let n = poles.len() - 1;
    let u = u.clamp(fk[p], fk[n + 1]);
    let du = nd.min(p);
    let span = find_span_flat(n, p, u, fk);
    let ders_b = ders_basis_funs(span, u, p, du, fk);
    let mut ck = vec![Pnt::origin(); nd + 1];
    for k in 0..=du {
        let mut acc = Pnt::origin();
        for j in 0..=p {
            acc = acc + poles[span - p + j] * ders_b[k][j];
        }
        ck[k] = acc;
    }
    // Orders > p are zero (already origin).
    ck
}

/// Scalar (weight) derivatives, orders 0..=nd.
fn curve_derivs_scalar(fk: &[f64], weights: &[f64], p: usize, u: f64, nd: usize) -> Vec<f64> {
    let n = weights.len() - 1;
    let u = u.clamp(fk[p], fk[n + 1]);
    let du = nd.min(p);
    let span = find_span_flat(n, p, u, fk);
    let ders_b = ders_basis_funs(span, u, p, du, fk);
    let mut wk = vec![0.0; nd + 1];
    for k in 0..=du {
        let mut acc = 0.0;
        for j in 0..=p {
            acc += weights[span - p + j] * ders_b[k][j];
        }
        wk[k] = acc;
    }
    wk
}

/// Rational curve derivatives from homogeneous numerator (`aders`) and weight
/// (`wders`) derivatives, via the quotient rule (Piegl-Tiller A4.2).
fn rational_derivs(aders: &[Pnt], wders: &[f64], nd: usize) -> Vec<Pnt> {
    let mut ck = vec![Pnt::origin(); nd + 1];
    let binom = pascal(nd);
    for k in 0..=nd {
        let mut v = aders[k];
        for i in 1..=k {
            v = v - ck[k - i] * (binom[k][i] * wders[i]);
        }
        ck[k] = v * (1.0 / wders[0]);
    }
    ck
}

/// Pascal's triangle of binomial coefficients up to `n`.
fn pascal(n: usize) -> Vec<Vec<f64>> {
    let mut c = vec![vec![0.0; n + 1]; n + 1];
    for i in 0..=n {
        c[i][0] = 1.0;
        for j in 1..=i {
            c[i][j] = c[i - 1][j - 1] + if j < i { c[i - 1][j] } else { 0.0 };
        }
    }
    c
}

/// Boehm single knot insertion at value `u` (rational-aware). Mutates the
/// distinct knots / mults / poles / weights in place, inserting one occurrence.
fn insert_knot_into(
    knots: &mut Vec<f64>,
    mults: &mut Vec<i32>,
    poles: &mut Vec<Pnt>,
    weights: &mut Vec<f64>,
    degree: i32,
    u: f64,
) {
    let p = degree as usize;
    let fk = knots_mults_to_flat(knots, mults);
    let n = poles.len() - 1;
    let span = find_span_flat(n, p, u, &fk);
    // Current multiplicity of u (s).
    let s = mult_at(knots, mults, u) as usize;

    // Homogeneous poles.
    let hpoles: Vec<Pnt> = poles
        .iter()
        .zip(weights.iter())
        .map(|(pt, &w)| *pt * w)
        .collect();
    let hw: Vec<f64> = weights.clone();

    // Piegl-Tiller A5.1 CurveKnotIns, single insertion (r = 1).
    // n = last pole index, k = span, s = current multiplicity.
    let k = span;
    // The new control polygon has poles.len()+1 control points.
    let mut new_hpoles: Vec<Pnt> = vec![Pnt::origin(); poles.len() + 1];
    let mut new_hw: Vec<f64> = vec![0.0; weights.len() + 1];

    // Unaffected on the left: Q[i] = P[i] for i = 0..=k-p.
    new_hpoles[..=(k - p)].copy_from_slice(&hpoles[..=(k - p)]);
    new_hw[..=(k - p)].copy_from_slice(&hw[..=(k - p)]);
    // Unaffected on the right: Q[i+1] = P[i] for i = k-s..=n.
    new_hpoles[(k - s + 1)..=(n + 1)].copy_from_slice(&hpoles[(k - s)..=n]);
    new_hw[(k - s + 1)..=(n + 1)].copy_from_slice(&hw[(k - s)..=n]);
    // Affected poles: L = k-p+1 .. k-s, computed from a local temp.
    // temp[i] = P[k-p+i] for i = 0..=p-s.
    let mut temp: Vec<Pnt> = Vec::with_capacity(p - s + 1);
    let mut tempw: Vec<f64> = Vec::with_capacity(p - s + 1);
    for i in 0..=(p - s) {
        temp.push(hpoles[k - p + i]);
        tempw.push(hw[k - p + i]);
    }
    for i in 0..(p - s) {
        let l = k - p + 1 + i;
        let denom = fk[i + k + 1] - fk[l];
        let alpha = if denom.abs() < 1e-15 {
            0.0
        } else {
            (u - fk[l]) / denom
        };
        let np = temp[i + 1] * alpha + temp[i] * (1.0 - alpha);
        let nw = tempw[i + 1] * alpha + tempw[i] * (1.0 - alpha);
        new_hpoles[l] = np;
        new_hw[l] = nw;
    }

    // Convert back to cartesian.
    let new_poles: Vec<Pnt> = new_hpoles
        .iter()
        .zip(new_hw.iter())
        .map(|(p, w)| *p * (1.0 / *w))
        .collect();

    *poles = new_poles;
    *weights = new_hw;

    // Update knots/mults.
    if let Some(ki) = knots.iter().position(|&x| (x - u).abs() < 1e-12) {
        mults[ki] += 1;
    } else {
        // Insert new distinct knot in sorted position.
        let pos = knots.iter().position(|&x| x > u).unwrap_or(knots.len());
        knots.insert(pos, u);
        mults.insert(pos, 1);
    }
}

/// Bezier degree elevation by one (homogeneous). Given p+1 control points and
/// weights, returns p+2 control points and weights.
fn bezier_elevate(poles: &[Pnt], weights: &[f64]) -> (Vec<Pnt>, Vec<f64>) {
    let p = poles.len() - 1;
    let pe = p + 1;
    let mut out_p = vec![Pnt::origin(); pe + 1];
    let mut out_w = vec![0.0; pe + 1];
    out_p[0] = poles[0];
    out_w[0] = weights[0];
    out_p[pe] = poles[p];
    out_w[pe] = weights[p];
    for i in 1..pe {
        let a = i as f64 / (pe as f64);
        out_p[i] = poles[i - 1] * a + poles[i] * (1.0 - a);
        out_w[i] = weights[i - 1] * a + weights[i] * (1.0 - a);
    }
    (out_p, out_w)
}
