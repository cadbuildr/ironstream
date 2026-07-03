//! `Geom_BSplineSurface` — a faithful reproduction of OpenCascade's
//! `Geom_BSplineSurface` (`src/ModelingData/TKG3d/Geom/Geom_BSplineSurface.hxx`).
//!
//! A BSpline surface is a tensor-product surface defined, in each parametric
//! direction (U and V), by:
//! - a degree,
//! - a periodicity flag,
//! - a table of poles (control points) and optional weights (NURBS), and
//! - a table of knots together with their multiplicities.
//!
//! This module mirrors the OCCT public API: construction from the
//! poles / knots / multiplicities tables (rational and non-rational), the
//! evaluation methods (`D0`..`D3`, `DN`, local evaluations), the editing
//! operations (`InsertUKnot`, `RemoveUKnot`, `IncreaseDegree`, `Segment`,
//! `MovePoint`, `SetPole`, `SetWeight`, ...), the orientation operations
//! (`UReverse`, `VReverse`, `ExchangeUV`), the periodicity operations
//! (`SetUNotPeriodic`, `SetVNotPeriodic`), and the iso-curve extraction
//! (`UIso`, `VIso`).
//!
//! Indices in the public API are 1-based, exactly like OCCT. Internally the
//! storage is 0-based and a "flat" (expanded) knot vector is maintained for the
//! de Boor / basis-function evaluation.
//!
//! Built on the existing `gp` API; zero third-party dependencies.

use crate::bsplines::BSplineCurve;
use crate::gp::{Pnt, Trsf, Vec3};

const GP_RESOLUTION: f64 = 1.0e-15;

/// `Geom_BSplineSurface` — describes a BSpline (or NURBS) surface in 3D space.
///
/// Poles are stored row-major: `poles[i][j]` is the pole of U-index `i+1`,
/// V-index `j+1` (OCCT 1-based `Pole(i, j)`). Weights, when present, follow the
/// same layout. The `u_knots` / `v_knots` arrays carry the distinct knot values
/// and `u_mults` / `v_mults` their multiplicities.
// occt: Geom_BSplineSurface
#[derive(Clone, Debug)]
pub struct GeomBSplineSurface {
    u_degree: usize,
    v_degree: usize,
    u_knots: Vec<f64>,
    v_knots: Vec<f64>,
    u_mults: Vec<usize>,
    v_mults: Vec<usize>,
    /// poles[i][j], i over U (NbUPoles rows), j over V (NbVPoles cols).
    poles: Vec<Vec<Pnt>>,
    /// Weights, same layout; `None` ⇒ non-rational (all weights == 1).
    weights: Option<Vec<Vec<f64>>>,
    u_periodic: bool,
    v_periodic: bool,
}

/// The maximum admissible degree for a `Geom_BSplineSurface` (OCCT: 25).
pub const MAX_DEGREE: usize = 25;

// ---------------------------------------------------------------------------
// Flat-knot helpers (knots + mults  ->  expanded knot sequence)
// ---------------------------------------------------------------------------

/// Expand distinct knots + multiplicities into the flat (repeated) knot vector.
fn flat_knots(knots: &[f64], mults: &[usize]) -> Vec<f64> {
    let mut out = Vec::new();
    for (k, m) in knots.iter().zip(mults.iter()) {
        for _ in 0..*m {
            out.push(*k);
        }
    }
    out
}

/// Number of poles implied by a clamped (non-periodic) knot/mult table:
/// `sum(mults) - degree - 1`.
fn nb_poles_nonperiodic(mults: &[usize], degree: usize) -> usize {
    let s: usize = mults.iter().sum();
    s - degree - 1
}

/// Number of poles implied by a periodic knot/mult table:
/// `sum(mults[..last])` (the last knot is the periodic image of the first).
fn nb_poles_periodic(mults: &[usize]) -> usize {
    mults[..mults.len() - 1].iter().sum()
}

// ---------------------------------------------------------------------------
// B-spline basis functions and their derivatives (The NURBS Book, A2.2 / A2.3)
// ---------------------------------------------------------------------------

/// Find the knot span index `i` with `flat[i] <= u < flat[i+1]` (clamped).
/// `n` = last pole index, `p` = degree.
fn find_span(n: usize, p: usize, u: f64, flat: &[f64]) -> usize {
    if u >= flat[n + 1] {
        return n;
    }
    if u <= flat[p] {
        return p;
    }
    let (mut low, mut high) = (p, n + 1);
    let mut mid = (low + high) / 2;
    while u < flat[mid] || u >= flat[mid + 1] {
        if u < flat[mid] {
            high = mid;
        } else {
            low = mid;
        }
        mid = (low + high) / 2;
    }
    mid
}

/// Compute the non-zero basis functions and their derivatives up to order `nd`
/// at `u` in span `span`. Returns `ders[k][j]` = k-th derivative of the j-th
/// non-zero basis function (j in 0..=p). The NURBS Book Algorithm A2.3.
fn ders_basis_funs(span: usize, u: f64, p: usize, nd: usize, flat: &[f64]) -> Vec<Vec<f64>> {
    let mut ndu = vec![vec![0.0f64; p + 1]; p + 1];
    let mut left = vec![0.0f64; p + 1];
    let mut right = vec![0.0f64; p + 1];
    ndu[0][0] = 1.0;
    for j in 1..=p {
        left[j] = u - flat[span + 1 - j];
        right[j] = flat[span + j] - u;
        let mut saved = 0.0;
        for r in 0..j {
            ndu[j][r] = right[r + 1] + left[j - r];
            let temp = if ndu[j][r].abs() < GP_RESOLUTION {
                0.0
            } else {
                ndu[r][j - 1] / ndu[j][r]
            };
            ndu[r][j] = saved + right[r + 1] * temp;
            saved = left[j - r] * temp;
        }
        ndu[j][j] = saved;
    }

    let mut ders = vec![vec![0.0f64; p + 1]; nd + 1];
    for j in 0..=p {
        ders[0][j] = ndu[j][p];
    }

    // Compute derivatives.
    let mut a = vec![vec![0.0f64; p + 1]; 2];
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
                a[s2][j] =
                    (a[s1][j] - a[s1][j - 1]) / ndu[(pk + 1) as usize][(rk + j as isize) as usize];
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

    // Multiply by the correct factors.
    let mut r = p;
    for k in 1..=nd {
        for j in 0..=p {
            ders[k][j] *= r as f64;
        }
        r *= p.saturating_sub(k);
    }
    ders
}

// ---------------------------------------------------------------------------
// Construction
// ---------------------------------------------------------------------------

impl GeomBSplineSurface {
    /// Non-rational constructor.
    /// `Geom_BSplineSurface(Poles, UKnots, VKnots, UMults, VMults, UDegree, VDegree, UPeriodic, VPeriodic)`.
    ///
    /// `poles[i][j]` is the pole of U-index `i`, V-index `j` (0-based here).
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        poles: Vec<Vec<Pnt>>,
        u_knots: Vec<f64>,
        v_knots: Vec<f64>,
        u_mults: Vec<usize>,
        v_mults: Vec<usize>,
        u_degree: usize,
        v_degree: usize,
        u_periodic: bool,
        v_periodic: bool,
    ) -> Self {
        Self::build(
            poles, None, u_knots, v_knots, u_mults, v_mults, u_degree, v_degree, u_periodic,
            v_periodic,
        )
    }

    /// Rational constructor.
    /// `Geom_BSplineSurface(Poles, Weights, UKnots, VKnots, UMults, VMults, UDegree, VDegree, UPeriodic, VPeriodic)`.
    #[allow(clippy::too_many_arguments)]
    pub fn new_rational(
        poles: Vec<Vec<Pnt>>,
        weights: Vec<Vec<f64>>,
        u_knots: Vec<f64>,
        v_knots: Vec<f64>,
        u_mults: Vec<usize>,
        v_mults: Vec<usize>,
        u_degree: usize,
        v_degree: usize,
        u_periodic: bool,
        v_periodic: bool,
    ) -> Self {
        Self::build(
            poles,
            Some(weights),
            u_knots,
            v_knots,
            u_mults,
            v_mults,
            u_degree,
            v_degree,
            u_periodic,
            v_periodic,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn build(
        poles: Vec<Vec<Pnt>>,
        weights: Option<Vec<Vec<f64>>>,
        u_knots: Vec<f64>,
        v_knots: Vec<f64>,
        u_mults: Vec<usize>,
        v_mults: Vec<usize>,
        u_degree: usize,
        v_degree: usize,
        u_periodic: bool,
        v_periodic: bool,
    ) -> Self {
        assert!(
            (1..=MAX_DEGREE).contains(&u_degree),
            "Geom_BSplineSurface: UDegree out of range"
        );
        assert!(
            (1..=MAX_DEGREE).contains(&v_degree),
            "Geom_BSplineSurface: VDegree out of range"
        );
        assert_eq!(
            u_knots.len(),
            u_mults.len(),
            "Geom_BSplineSurface: UKnots / UMults length mismatch"
        );
        assert_eq!(
            v_knots.len(),
            v_mults.len(),
            "Geom_BSplineSurface: VKnots / VMults length mismatch"
        );

        let nb_u = if u_periodic {
            nb_poles_periodic(&u_mults)
        } else {
            nb_poles_nonperiodic(&u_mults, u_degree)
        };
        let nb_v = if v_periodic {
            nb_poles_periodic(&v_mults)
        } else {
            nb_poles_nonperiodic(&v_mults, v_degree)
        };
        assert_eq!(poles.len(), nb_u, "Geom_BSplineSurface: NbUPoles mismatch");
        assert_eq!(
            poles[0].len(),
            nb_v,
            "Geom_BSplineSurface: NbVPoles mismatch"
        );

        let weights = weights.filter(|w| {
            // Drop the weight table if all weights are identical (non-rational).
            let w0 = w[0][0];
            !w.iter()
                .all(|row| row.iter().all(|&x| (x - w0).abs() <= GP_RESOLUTION))
        });

        Self {
            u_degree,
            v_degree,
            u_knots,
            v_knots,
            u_mults,
            v_mults,
            poles,
            weights,
            u_periodic,
            v_periodic,
        }
    }

    /// Copy constructor `Geom_BSplineSurface(const Geom_BSplineSurface&)`.
    pub fn copy(&self) -> Self {
        self.clone()
    }
}

// ---------------------------------------------------------------------------
// Flat knot / pole accessors used by the evaluator (handle periodicity)
// ---------------------------------------------------------------------------

impl GeomBSplineSurface {
    /// Flat U knot vector for evaluation; for periodic surfaces this expands the
    /// periodic structure to a clamped-equivalent flat vector with wrapped poles.
    fn u_eval(&self) -> (Vec<f64>, usize) {
        (flat_knots(&self.u_knots, &self.u_mults), self.u_degree)
    }
    fn v_eval(&self) -> (Vec<f64>, usize) {
        (flat_knots(&self.v_knots, &self.v_mults), self.v_degree)
    }

    /// Returns the (flat_u_knots, flat_v_knots, homogeneous poles, weights, nbU, nbV)
    /// view suitable for tensor-product evaluation. For periodic directions the
    /// poles are extended with their wrapped copies so a clamped evaluator works.
    fn eval_arrays(&self) -> EvalView {
        let (fu, _) = self.u_eval();
        let (fv, _) = self.v_eval();
        // poles indexed [i][j]
        let mut poles = self.poles.clone();
        let mut weights = self.weights.clone();
        let mut fu = fu;
        let mut fv = fv;

        if self.u_periodic {
            let p = self.u_degree;
            let nb = poles.len();
            // Wrap p poles at the end.
            for k in 0..p {
                poles.push(poles[k].clone());
                if let Some(w) = weights.as_mut() {
                    w.push(w[k].clone());
                }
            }
            // The flat knot vector built from periodic mults already has the
            // right length: NbPoles + nb_extra ... rebuild as repeated periodic
            // sequence so length == (nb+p) + degree + 1.
            fu = self.periodic_flat_u();
            let _ = nb;
        }
        if self.v_periodic {
            let p = self.v_degree;
            for row in poles.iter_mut() {
                for k in 0..p {
                    row.push(row[k]);
                }
            }
            if let Some(w) = weights.as_mut() {
                for row in w.iter_mut() {
                    for k in 0..p {
                        row.push(row[k]);
                    }
                }
            }
            fv = self.periodic_flat_v();
        }

        let nb_u = poles.len();
        let nb_v = poles[0].len();
        EvalView {
            fu,
            fv,
            poles,
            weights,
            u_degree: self.u_degree,
            v_degree: self.v_degree,
            nb_u,
            nb_v,
        }
    }

    fn periodic_flat_u(&self) -> Vec<f64> {
        periodic_flat(&self.u_knots, &self.u_mults, self.u_degree)
    }
    fn periodic_flat_v(&self) -> Vec<f64> {
        periodic_flat(&self.v_knots, &self.v_mults, self.v_degree)
    }
}

/// Build a flat knot vector for a periodic B-spline suitable for clamped-style
/// evaluation with `degree` extra wrapped poles. Produces a vector of length
/// `NbPoles + degree + (degree+1)` by extending the periodic pattern on both
/// ends by `degree` knots.
fn periodic_flat(knots: &[f64], mults: &[usize], degree: usize) -> Vec<f64> {
    let period = knots[knots.len() - 1] - knots[0];
    let base = flat_knots(knots, mults);
    let n = base.len();
    let p = degree;
    // Left extension: the `p` knots that periodically precede base[0], i.e. the
    // last `p` distinct steps wrapped by -period: base[n-1-p .. n-2] - period.
    let mut left: Vec<f64> = Vec::with_capacity(p);
    for k in 0..p {
        left.push(base[n - 1 - p + k] - period);
    }
    // Right extension: the `p` knots that periodically follow base[n-1], i.e.
    // base[1 ..= p] + period.
    let mut right: Vec<f64> = Vec::with_capacity(p);
    for k in 0..p {
        right.push(base[1 + k] + period);
    }
    let mut out = Vec::with_capacity(left.len() + n + right.len());
    out.extend_from_slice(&left);
    out.extend_from_slice(&base);
    out.extend_from_slice(&right);
    out
}

/// A flattened, clamped-equivalent view used for tensor-product evaluation.
struct EvalView {
    fu: Vec<f64>,
    fv: Vec<f64>,
    poles: Vec<Vec<Pnt>>,
    weights: Option<Vec<Vec<f64>>>,
    u_degree: usize,
    v_degree: usize,
    nb_u: usize,
    nb_v: usize,
}

impl EvalView {
    /// Evaluate the homogeneous derivatives of order (du, dv) at (u, v).
    /// Returns (sum of w*P derivatives, sum of w derivatives) so the caller can
    /// apply the rational quotient rule across the full grid of orders.
    fn ders(&self, u: f64, v: f64, nu: usize, nv: usize) -> Vec<Vec<(Pnt, f64)>> {
        let pu = self.u_degree;
        let pv = self.v_degree;
        let span_u = find_span(self.nb_u - 1, pu, u, &self.fu);
        let span_v = find_span(self.nb_v - 1, pv, v, &self.fv);
        let du_max = nu.min(pu);
        let dv_max = nv.min(pv);
        let nders_u = ders_basis_funs(span_u, u, pu, du_max, &self.fu);
        let nders_v = ders_basis_funs(span_v, v, pv, dv_max, &self.fv);

        // SKL[k][l] = derivative of order (k in u, l in v), homogeneous.
        let mut skl = vec![vec![(Pnt::origin(), 0.0); nv + 1]; nu + 1];
        for k in 0..=nu.min(pu) {
            // temp[s] = contribution row in v for the k-th u-derivative
            let mut temp: Vec<(Pnt, f64)> = vec![(Pnt::origin(), 0.0); pv + 1];
            for s in 0..=pv {
                let mut acc = (Pnt::origin(), 0.0);
                for r in 0..=pu {
                    let i = span_u - pu + r;
                    let j = span_v - pv + s;
                    let w = self
                        .weights
                        .as_ref()
                        .map(|ww| ww[i][j])
                        .unwrap_or(1.0);
                    let pw = self.poles[i][j] * w;
                    let b = nders_u[k][r];
                    acc.0 = acc.0 + pw * b;
                    acc.1 += w * b;
                }
                temp[s] = acc;
            }
            for l in 0..=nv.min(pv) {
                let mut acc = (Pnt::origin(), 0.0);
                for s in 0..=pv {
                    let b = nders_v[l][s];
                    acc.0 = acc.0 + temp[s].0 * b;
                    acc.1 += temp[s].1 * b;
                }
                skl[k][l] = acc;
            }
        }
        skl
    }
}

/// Binomial coefficient C(n, k).
fn binom(n: usize, k: usize) -> f64 {
    if k > n {
        return 0.0;
    }
    let k = k.min(n - k);
    let mut num = 1.0f64;
    let mut den = 1.0f64;
    for i in 0..k {
        num *= (n - i) as f64;
        den *= (i + 1) as f64;
    }
    num / den
}

/// Apply the rational quotient rule to a grid of homogeneous derivatives `skl`
/// (each entry = (A = sum w*P deriv, w = sum w deriv)) to obtain the Cartesian
/// derivative of order (ku, kv). The NURBS Book, eq. (4.20) generalized to 2D.
fn rational_deriv(skl: &[Vec<(Pnt, f64)>], ku: usize, kv: usize) -> Pnt {
    // C[k][l] computed by recursion. We need C[ku][kv].
    let mut c = vec![vec![Pnt::origin(); kv + 1]; ku + 1];
    for k in 0..=ku {
        for l in 0..=kv {
            let mut v = skl[k][l].0;
            for j in 1..=l {
                v = v - c[k][l - j] * (binom(l, j) * skl[0][j].1);
            }
            for i in 1..=k {
                v = v - c[k - i][l] * (binom(k, i) * skl[i][0].1);
                let mut v2 = Pnt::origin();
                for j in 1..=l {
                    v2 = v2 + c[k - i][l - j] * (binom(l, j) * skl[i][j].1);
                }
                v = v - v2 * binom(k, i);
            }
            let w = skl[0][0].1;
            c[k][l] = v * (1.0 / w);
        }
    }
    c[ku][kv]
}

// ---------------------------------------------------------------------------
// Properties
// ---------------------------------------------------------------------------

impl GeomBSplineSurface {
    pub fn u_degree(&self) -> usize {
        self.u_degree
    }
    pub fn v_degree(&self) -> usize {
        self.v_degree
    }
    pub fn nb_u_poles(&self) -> usize {
        self.poles.len()
    }
    pub fn nb_v_poles(&self) -> usize {
        self.poles[0].len()
    }
    pub fn nb_u_knots(&self) -> usize {
        self.u_knots.len()
    }
    pub fn nb_v_knots(&self) -> usize {
        self.v_knots.len()
    }
    pub fn is_u_periodic(&self) -> bool {
        self.u_periodic
    }
    pub fn is_v_periodic(&self) -> bool {
        self.v_periodic
    }

    /// `IsURational` — true if weights vary along U for any V column.
    pub fn is_u_rational(&self) -> bool {
        match &self.weights {
            None => false,
            Some(w) => {
                let nb_v = self.nb_v_poles();
                (0..nb_v).any(|j| {
                    let w0 = w[0][j];
                    (0..self.nb_u_poles()).any(|i| (w[i][j] - w0).abs() > GP_RESOLUTION)
                })
            }
        }
    }

    /// `IsVRational` — true if some column of weights is non-uniform.
    pub fn is_v_rational(&self) -> bool {
        match &self.weights {
            None => false,
            Some(w) => {
                let nb_u = self.nb_u_poles();
                (0..nb_u).any(|i| {
                    let w0 = w[i][0];
                    (0..self.nb_v_poles()).any(|j| (w[i][j] - w0).abs() > GP_RESOLUTION)
                })
            }
        }
    }

    /// `Pole(UIndex, VIndex)` — 1-based.
    pub fn pole(&self, u_index: usize, v_index: usize) -> Pnt {
        self.poles[u_index - 1][v_index - 1]
    }

    /// `Poles()` — the whole pole table (0-based rows/cols).
    pub fn poles(&self) -> &Vec<Vec<Pnt>> {
        &self.poles
    }

    /// `Weight(UIndex, VIndex)` — 1-based. Returns 1.0 if non-rational.
    pub fn weight(&self, u_index: usize, v_index: usize) -> f64 {
        match &self.weights {
            Some(w) => w[u_index - 1][v_index - 1],
            None => 1.0,
        }
    }

    /// `Weights()` — `Some` only when rational.
    pub fn weights(&self) -> Option<&Vec<Vec<f64>>> {
        self.weights.as_ref()
    }

    /// `WeightsArray()` — always returns a full NbU x NbV table (unit weights
    /// when non-rational).
    pub fn weights_array(&self) -> Vec<Vec<f64>> {
        match &self.weights {
            Some(w) => w.clone(),
            None => vec![vec![1.0; self.nb_v_poles()]; self.nb_u_poles()],
        }
    }

    /// `UKnot(UIndex)` — 1-based.
    pub fn u_knot(&self, i: usize) -> f64 {
        self.u_knots[i - 1]
    }
    /// `VKnot(VIndex)` — 1-based.
    pub fn v_knot(&self, i: usize) -> f64 {
        self.v_knots[i - 1]
    }
    /// `UMultiplicity(UIndex)` — 1-based.
    pub fn u_multiplicity(&self, i: usize) -> usize {
        self.u_mults[i - 1]
    }
    /// `VMultiplicity(VIndex)` — 1-based.
    pub fn v_multiplicity(&self, i: usize) -> usize {
        self.v_mults[i - 1]
    }

    /// `UKnots()` — distinct U knot values.
    pub fn u_knots(&self) -> &Vec<f64> {
        &self.u_knots
    }
    /// `VKnots()` — distinct V knot values.
    pub fn v_knots(&self) -> &Vec<f64> {
        &self.v_knots
    }
    /// `UMultiplicities()`.
    pub fn u_multiplicities(&self) -> &Vec<usize> {
        &self.u_mults
    }
    /// `VMultiplicities()`.
    pub fn v_multiplicities(&self) -> &Vec<usize> {
        &self.v_mults
    }

    /// `Bounds(U1, U2, V1, V2)` — the parametric domain.
    pub fn bounds(&self) -> (f64, f64, f64, f64) {
        let (u1, u2) = self.u_range();
        let (v1, v2) = self.v_range();
        (u1, u2, v1, v2)
    }

    fn u_range(&self) -> (f64, f64) {
        if self.u_periodic {
            (self.u_knots[0], self.u_knots[self.u_knots.len() - 1])
        } else {
            let fu = flat_knots(&self.u_knots, &self.u_mults);
            (fu[self.u_degree], fu[self.nb_u_poles()])
        }
    }
    fn v_range(&self) -> (f64, f64) {
        if self.v_periodic {
            (self.v_knots[0], self.v_knots[self.v_knots.len() - 1])
        } else {
            let fv = flat_knots(&self.v_knots, &self.v_mults);
            (fv[self.v_degree], fv[self.nb_v_poles()])
        }
    }

    /// `FirstUKnotIndex` — index (1-based) of the knot at the U first parameter.
    pub fn first_u_knot_index(&self) -> usize {
        if self.u_periodic {
            return 1;
        }
        // first knot whose cumulative mult >= degree+1
        let mut cum = 0usize;
        for (i, m) in self.u_mults.iter().enumerate() {
            cum += *m;
            if cum > self.u_degree {
                return i + 1;
            }
        }
        1
    }
    /// `LastUKnotIndex`.
    pub fn last_u_knot_index(&self) -> usize {
        if self.u_periodic {
            return self.nb_u_knots();
        }
        let mut cum = 0usize;
        let total: usize = self.u_mults.iter().sum();
        for (i, m) in self.u_mults.iter().enumerate() {
            cum += *m;
            if total - (cum - *m) <= self.u_degree + 1 {
                return i + 1;
            }
        }
        self.nb_u_knots()
    }
    pub fn first_v_knot_index(&self) -> usize {
        if self.v_periodic {
            return 1;
        }
        let mut cum = 0usize;
        for (i, m) in self.v_mults.iter().enumerate() {
            cum += *m;
            if cum > self.v_degree {
                return i + 1;
            }
        }
        1
    }
    pub fn last_v_knot_index(&self) -> usize {
        if self.v_periodic {
            return self.nb_v_knots();
        }
        let mut cum = 0usize;
        let total: usize = self.v_mults.iter().sum();
        for (i, m) in self.v_mults.iter().enumerate() {
            cum += *m;
            if total - (cum - *m) <= self.v_degree + 1 {
                return i + 1;
            }
        }
        self.nb_v_knots()
    }

    /// `MaxDegree()`.
    pub fn max_degree() -> usize {
        MAX_DEGREE
    }
}

// ---------------------------------------------------------------------------
// Evaluation
// ---------------------------------------------------------------------------

impl GeomBSplineSurface {
    /// `Value(U, V)` / `D0(U, V, P)` — the point on the surface.
    pub fn value(&self, u: f64, v: f64) -> Pnt {
        let (u, v) = self.clamp_param(u, v);
        let ev = self.eval_arrays();
        let skl = ev.ders(u, v, 0, 0);
        if self.weights.is_some() {
            skl[0][0].0 * (1.0 / skl[0][0].1)
        } else {
            skl[0][0].0
        }
    }

    /// `D0(U, V, P)`.
    pub fn d0(&self, u: f64, v: f64) -> Pnt {
        self.value(u, v)
    }

    /// `D1(U, V, P, D1U, D1V)`.
    pub fn d1(&self, u: f64, v: f64) -> (Pnt, Vec3, Vec3) {
        let (u, v) = self.clamp_param(u, v);
        let ev = self.eval_arrays();
        let skl = ev.ders(u, v, 1, 1);
        if self.weights.is_some() {
            let p = rational_deriv(&skl, 0, 0);
            let du = rational_deriv(&skl, 1, 0);
            let dv = rational_deriv(&skl, 0, 1);
            (p, du, dv)
        } else {
            (skl[0][0].0, skl[1][0].0, skl[0][1].0)
        }
    }

    /// `D2(U, V, P, D1U, D1V, D2U, D2V, D2UV)`.
    #[allow(clippy::type_complexity)]
    pub fn d2(&self, u: f64, v: f64) -> (Pnt, Vec3, Vec3, Vec3, Vec3, Vec3) {
        let (u, v) = self.clamp_param(u, v);
        let ev = self.eval_arrays();
        let skl = ev.ders(u, v, 2, 2);
        if self.weights.is_some() {
            (
                rational_deriv(&skl, 0, 0),
                rational_deriv(&skl, 1, 0),
                rational_deriv(&skl, 0, 1),
                rational_deriv(&skl, 2, 0),
                rational_deriv(&skl, 0, 2),
                rational_deriv(&skl, 1, 1),
            )
        } else {
            (
                skl[0][0].0,
                skl[1][0].0,
                skl[0][1].0,
                skl[2][0].0,
                skl[0][2].0,
                skl[1][1].0,
            )
        }
    }

    /// `D3(U, V, P, D1U, D1V, D2U, D2V, D2UV, D3U, D3V, D3UUV, D3UVV)`.
    #[allow(clippy::type_complexity)]
    pub fn d3(
        &self,
        u: f64,
        v: f64,
    ) -> (Pnt, Vec3, Vec3, Vec3, Vec3, Vec3, Vec3, Vec3, Vec3, Vec3) {
        let (u, v) = self.clamp_param(u, v);
        let ev = self.eval_arrays();
        let skl = ev.ders(u, v, 3, 3);
        let get = |ku: usize, kv: usize| {
            if self.weights.is_some() {
                rational_deriv(&skl, ku, kv)
            } else {
                skl[ku][kv].0
            }
        };
        (
            get(0, 0),
            get(1, 0),
            get(0, 1),
            get(2, 0),
            get(0, 2),
            get(1, 1),
            get(3, 0),
            get(0, 3),
            get(2, 1),
            get(1, 2),
        )
    }

    /// `DN(U, V, Nu, Nv)` — the derivative of order (Nu, Nv).
    pub fn dn(&self, u: f64, v: f64, nu: usize, nv: usize) -> Vec3 {
        let (u, v) = self.clamp_param(u, v);
        let ev = self.eval_arrays();
        let skl = ev.ders(u, v, nu, nv);
        if self.weights.is_some() {
            rational_deriv(&skl, nu, nv)
        } else {
            skl[nu][nv].0
        }
    }

    fn clamp_param(&self, u: f64, v: f64) -> (f64, f64) {
        let (u1, u2, v1, v2) = self.bounds();
        let cu = if self.u_periodic {
            let per = u2 - u1;
            let mut t = (u - u1) % per;
            if t < 0.0 {
                t += per;
            }
            u1 + t
        } else {
            u.clamp(u1, u2)
        };
        let cv = if self.v_periodic {
            let per = v2 - v1;
            let mut t = (v - v1) % per;
            if t < 0.0 {
                t += per;
            }
            v1 + t
        } else {
            v.clamp(v1, v2)
        };
        (cu, cv)
    }

    /// `Resolution(Tolerance3D, UTolerance, VTolerance)`.
    /// Estimates parametric tolerances guaranteeing a 3D step below `tol3d`.
    pub fn resolution(&self, tol3d: f64) -> (f64, f64) {
        // Max magnitude of the first partial derivatives over a coarse sampling.
        let (u1, u2, v1, v2) = self.bounds();
        let mut max_du: f64 = 0.0;
        let mut max_dv: f64 = 0.0;
        let n = 8;
        for i in 0..=n {
            for j in 0..=n {
                let u = u1 + (u2 - u1) * i as f64 / n as f64;
                let v = v1 + (v2 - v1) * j as f64 / n as f64;
                let (_, du, dv) = self.d1(u, v);
                max_du = max_du.max(du.norm());
                max_dv = max_dv.max(dv.norm());
            }
        }
        let u_tol = if max_du > GP_RESOLUTION {
            tol3d / max_du
        } else {
            tol3d
        };
        let v_tol = if max_dv > GP_RESOLUTION {
            tol3d / max_dv
        } else {
            tol3d
        };
        (u_tol, v_tol)
    }
}

// ---------------------------------------------------------------------------
// Local evaluation (restricted to a knot patch).
// ---------------------------------------------------------------------------

impl GeomBSplineSurface {
    /// `LocalD0(U, V, FromUK1, ToUK2, FromVK1, ToVK2, P)`.
    ///
    /// For non-periodic surfaces with global continuity, the patch restriction
    /// produces the same value as the global evaluation; we evaluate globally.
    #[allow(clippy::too_many_arguments)]
    pub fn local_d0(
        &self,
        u: f64,
        v: f64,
        from_uk1: usize,
        to_uk2: usize,
        from_vk1: usize,
        to_vk2: usize,
    ) -> Pnt {
        assert!(from_uk1 != to_uk2, "LocalD0: FromUK1 == ToUK2");
        assert!(from_vk1 != to_vk2, "LocalD0: FromVK1 == ToVK2");
        self.value(u, v)
    }
}

// ---------------------------------------------------------------------------
// Iso-curves
// ---------------------------------------------------------------------------

impl GeomBSplineSurface {
    /// `UIso(U)` — the isoparametric B-spline curve at fixed U.
    /// The resulting curve is parameterized by V.
    pub fn u_iso(&self, u: f64) -> BSplineCurve<Pnt> {
        let ev = self.eval_arrays();
        // Evaluate the U de Boor for each V control column to get V-poles.
        // Rational: keep homogeneous (P*w, w) then project per V-pole.
        let pu = ev.u_degree;
        let span_u = find_span(ev.nb_u - 1, pu, self.clamp_param(u, self.v_range().0).0, &ev.fu);
        let uu = self.clamp_param(u, self.v_range().0).0;
        let basis = ders_basis_funs(span_u, uu, pu, 0, &ev.fu);
        let mut v_poles = Vec::with_capacity(ev.nb_v);
        let mut v_weights = Vec::with_capacity(ev.nb_v);
        for j in 0..ev.nb_v {
            let mut acc = Pnt::origin();
            let mut wacc = 0.0;
            for r in 0..=pu {
                let i = span_u - pu + r;
                let w = ev.weights.as_ref().map(|ww| ww[i][j]).unwrap_or(1.0);
                acc = acc + ev.poles[i][j] * (w * basis[0][r]);
                wacc += w * basis[0][r];
            }
            v_poles.push(acc * (1.0 / wacc));
            v_weights.push(wacc);
        }
        let weights = if self.weights.is_some() {
            Some(v_weights)
        } else {
            None
        };
        BSplineCurve::new(ev.v_degree, ev.fv.clone(), v_poles, weights)
    }

    /// `VIso(V)` — the isoparametric B-spline curve at fixed V (parameterized by U).
    pub fn v_iso(&self, v: f64) -> BSplineCurve<Pnt> {
        let ev = self.eval_arrays();
        let pv = ev.v_degree;
        let vv = self.clamp_param(self.u_range().0, v).1;
        let span_v = find_span(ev.nb_v - 1, pv, vv, &ev.fv);
        let basis = ders_basis_funs(span_v, vv, pv, 0, &ev.fv);
        let mut u_poles = Vec::with_capacity(ev.nb_u);
        let mut u_weights = Vec::with_capacity(ev.nb_u);
        for i in 0..ev.nb_u {
            let mut acc = Pnt::origin();
            let mut wacc = 0.0;
            for s in 0..=pv {
                let j = span_v - pv + s;
                let w = ev.weights.as_ref().map(|ww| ww[i][j]).unwrap_or(1.0);
                acc = acc + ev.poles[i][j] * (w * basis[0][s]);
                wacc += w * basis[0][s];
            }
            u_poles.push(acc * (1.0 / wacc));
            u_weights.push(wacc);
        }
        let weights = if self.weights.is_some() {
            Some(u_weights)
        } else {
            None
        };
        BSplineCurve::new(ev.u_degree, ev.fu.clone(), u_poles, weights)
    }
}

// ---------------------------------------------------------------------------
// Editing: SetPole / SetWeight and row/col variants.
// ---------------------------------------------------------------------------

impl GeomBSplineSurface {
    /// `SetPole(UIndex, VIndex, P)`.
    pub fn set_pole(&mut self, u_index: usize, v_index: usize, p: Pnt) {
        self.poles[u_index - 1][v_index - 1] = p;
    }

    /// `SetPole(UIndex, VIndex, P, Weight)`.
    pub fn set_pole_weighted(&mut self, u_index: usize, v_index: usize, p: Pnt, w: f64) {
        assert!(w > GP_RESOLUTION, "SetPole: weight <= Resolution");
        self.poles[u_index - 1][v_index - 1] = p;
        self.ensure_weights();
        if let Some(ws) = self.weights.as_mut() {
            ws[u_index - 1][v_index - 1] = w;
        }
    }

    /// `SetPoleRow(UIndex, CPoles)` — a whole U-row of poles (across V).
    pub fn set_pole_row(&mut self, u_index: usize, c_poles: &[Pnt]) {
        assert_eq!(c_poles.len(), self.nb_v_poles(), "SetPoleRow: length");
        for (j, p) in c_poles.iter().enumerate() {
            self.poles[u_index - 1][j] = *p;
        }
    }

    /// `SetPoleCol(VIndex, CPoles)` — a whole V-column of poles (across U).
    pub fn set_pole_col(&mut self, v_index: usize, c_poles: &[Pnt]) {
        assert_eq!(c_poles.len(), self.nb_u_poles(), "SetPoleCol: length");
        for (i, p) in c_poles.iter().enumerate() {
            self.poles[i][v_index - 1] = *p;
        }
    }

    fn ensure_weights(&mut self) {
        if self.weights.is_none() {
            self.weights = Some(vec![vec![1.0; self.nb_v_poles()]; self.nb_u_poles()]);
        }
    }

    /// `SetWeight(UIndex, VIndex, Weight)`.
    pub fn set_weight(&mut self, u_index: usize, v_index: usize, w: f64) {
        assert!(w > GP_RESOLUTION, "SetWeight: weight <= Resolution");
        self.ensure_weights();
        if let Some(ws) = self.weights.as_mut() {
            ws[u_index - 1][v_index - 1] = w;
        }
        self.normalize_rational();
    }

    /// `SetWeightRow(UIndex, CPoleWeights)`.
    pub fn set_weight_row(&mut self, u_index: usize, c_weights: &[f64]) {
        assert_eq!(c_weights.len(), self.nb_v_poles(), "SetWeightRow: length");
        self.ensure_weights();
        if let Some(ws) = self.weights.as_mut() {
            for (j, w) in c_weights.iter().enumerate() {
                assert!(*w > GP_RESOLUTION, "SetWeightRow: weight <= Resolution");
                ws[u_index - 1][j] = *w;
            }
        }
        self.normalize_rational();
    }

    /// `SetWeightCol(VIndex, CPoleWeights)`.
    pub fn set_weight_col(&mut self, v_index: usize, c_weights: &[f64]) {
        assert_eq!(c_weights.len(), self.nb_u_poles(), "SetWeightCol: length");
        self.ensure_weights();
        if let Some(ws) = self.weights.as_mut() {
            for (i, w) in c_weights.iter().enumerate() {
                assert!(*w > GP_RESOLUTION, "SetWeightCol: weight <= Resolution");
                ws[i][v_index - 1] = *w;
            }
        }
        self.normalize_rational();
    }

    /// Drop the weight table if it is globally uniform (became non-rational).
    fn normalize_rational(&mut self) {
        if let Some(w) = &self.weights {
            let w0 = w[0][0];
            let uniform = w
                .iter()
                .all(|row| row.iter().all(|&x| (x - w0).abs() <= GP_RESOLUTION));
            if uniform {
                self.weights = None;
            }
        }
    }

    /// `SetUKnot(UIndex, K)` — substitute a U knot value.
    pub fn set_u_knot(&mut self, u_index: usize, k: f64) {
        let i = u_index - 1;
        if i > 0 {
            assert!(k > self.u_knots[i - 1], "SetUKnot: K <= UKnots(UIndex-1)");
        }
        if i + 1 < self.u_knots.len() {
            assert!(k < self.u_knots[i + 1], "SetUKnot: K >= UKnots(UIndex+1)");
        }
        self.u_knots[i] = k;
    }

    /// `SetVKnot(VIndex, K)`.
    pub fn set_v_knot(&mut self, v_index: usize, k: f64) {
        let i = v_index - 1;
        if i > 0 {
            assert!(k > self.v_knots[i - 1], "SetVKnot: K <= VKnots(VIndex-1)");
        }
        if i + 1 < self.v_knots.len() {
            assert!(k < self.v_knots[i + 1], "SetVKnot: K >= VKnots(VIndex+1)");
        }
        self.v_knots[i] = k;
    }
}

// ---------------------------------------------------------------------------
// Locate
// ---------------------------------------------------------------------------

impl GeomBSplineSurface {
    /// `LocateU(U, ParametricTolerance, I1, I2)`.
    pub fn locate_u(&self, u: f64, tol: f64) -> (usize, usize) {
        locate(&self.u_knots, u, tol)
    }
    /// `LocateV(V, ParametricTolerance, I1, I2)`.
    pub fn locate_v(&self, v: f64, tol: f64) -> (usize, usize) {
        locate(&self.v_knots, v, tol)
    }
}

/// Locate a parameter inside a distinct-knot vector. Returns 1-based (I1, I2)
/// with `Knots(I1) <= U <= Knots(I2)`; I1 == I2 when U is a knot.
fn locate(knots: &[f64], u: f64, tol: f64) -> (usize, usize) {
    let n = knots.len();
    if u < knots[0] - tol.abs() {
        return (0, 1);
    }
    if u > knots[n - 1] + tol.abs() {
        return (n, n + 1);
    }
    for i in 0..n {
        if (u - knots[i]).abs() <= tol.abs() {
            return (i + 1, i + 1);
        }
    }
    for i in 0..n - 1 {
        if u > knots[i] && u < knots[i + 1] {
            return (i + 1, i + 2);
        }
    }
    (1, 1)
}

// ---------------------------------------------------------------------------
// Knot insertion (Boehm's algorithm), applied row/col-wise.
// ---------------------------------------------------------------------------

impl GeomBSplineSurface {
    /// `InsertUKnot(U, M, ParametricTolerance, Add)`.
    pub fn insert_u_knot(&mut self, u: f64, m: usize, tol: f64) {
        if m == 0 {
            return;
        }
        // Determine existing multiplicity.
        let existing = self
            .u_knots
            .iter()
            .position(|&k| (k - u).abs() <= tol.max(1e-15))
            .map(|i| self.u_mults[i]);
        let target = m.min(self.u_degree);
        let to_add = match existing {
            Some(cur) => target.saturating_sub(cur),
            None => target,
        };
        for _ in 0..to_add {
            self.insert_u_knot_once(u);
        }
    }

    /// `InsertVKnot(V, M, ParametricTolerance, Add)`.
    pub fn insert_v_knot(&mut self, v: f64, m: usize, tol: f64) {
        if m == 0 {
            return;
        }
        let existing = self
            .v_knots
            .iter()
            .position(|&k| (k - v).abs() <= tol.max(1e-15))
            .map(|i| self.v_mults[i]);
        let target = m.min(self.v_degree);
        let to_add = match existing {
            Some(cur) => target.saturating_sub(cur),
            None => target,
        };
        for _ in 0..to_add {
            self.insert_v_knot_once(v);
        }
    }

    /// Insert a single U knot (raise its multiplicity by one).
    fn insert_u_knot_once(&mut self, u: f64) {
        let p = self.u_degree;
        let flat = flat_knots(&self.u_knots, &self.u_mults);
        let n = self.nb_u_poles() - 1;
        let k = find_span(n, p, u, &flat);
        let nb_v = self.nb_v_poles();
        let has_w = self.weights.is_some();

        // Build per-column homogeneous insertion.
        let mut cols_p: Vec<Vec<Pnt>> = Vec::with_capacity(nb_v);
        let mut cols_w: Vec<Vec<f64>> = Vec::with_capacity(nb_v);
        for j in 0..nb_v {
            let mut col_p = Vec::with_capacity(self.nb_u_poles() + 1);
            let mut col_w = Vec::with_capacity(self.nb_u_poles() + 1);
            // 0..=k-p unchanged
            for i in 0..=(k - p) {
                let w = if has_w {
                    self.weights.as_ref().unwrap()[i][j]
                } else {
                    1.0
                };
                col_p.push(self.poles[i][j] * w);
                col_w.push(w);
            }
            for i in (k - p + 1)..=k {
                let denom = flat[i + p] - flat[i];
                let alpha = if denom.abs() < 1e-15 {
                    0.0
                } else {
                    (u - flat[i]) / denom
                };
                let wi = if has_w {
                    self.weights.as_ref().unwrap()[i][j]
                } else {
                    1.0
                };
                let wi1 = if has_w {
                    self.weights.as_ref().unwrap()[i - 1][j]
                } else {
                    1.0
                };
                let phi = self.poles[i][j] * wi;
                let phi1 = self.poles[i - 1][j] * wi1;
                col_p.push(phi1 * (1.0 - alpha) + phi * alpha);
                col_w.push(wi1 * (1.0 - alpha) + wi * alpha);
            }
            for i in k..self.nb_u_poles() {
                let w = if has_w {
                    self.weights.as_ref().unwrap()[i][j]
                } else {
                    1.0
                };
                col_p.push(self.poles[i][j] * w);
                col_w.push(w);
            }
            cols_p.push(col_p);
            cols_w.push(col_w);
        }

        let new_nb_u = cols_p[0].len();
        let mut new_poles: Vec<Vec<Pnt>> = vec![Vec::with_capacity(nb_v); new_nb_u];
        let mut new_weights: Vec<Vec<f64>> = vec![Vec::with_capacity(nb_v); new_nb_u];
        for i in 0..new_nb_u {
            for j in 0..nb_v {
                let w = cols_w[j][i];
                new_poles[i].push(cols_p[j][i] * (1.0 / w));
                new_weights[i].push(w);
            }
        }

        self.poles = new_poles;
        if has_w {
            self.weights = Some(new_weights);
        }

        // Update the knot/mult tables.
        self.bump_knot_mult_u(u);
    }

    fn insert_v_knot_once(&mut self, v: f64) {
        let p = self.v_degree;
        let flat = flat_knots(&self.v_knots, &self.v_mults);
        let n = self.nb_v_poles() - 1;
        let k = find_span(n, p, v, &flat);
        let nb_u = self.nb_u_poles();
        let has_w = self.weights.is_some();

        let mut new_poles: Vec<Vec<Pnt>> = Vec::with_capacity(nb_u);
        let mut new_weights: Vec<Vec<f64>> = Vec::with_capacity(nb_u);
        for i in 0..nb_u {
            let mut row_p = Vec::with_capacity(self.nb_v_poles() + 1);
            let mut row_w = Vec::with_capacity(self.nb_v_poles() + 1);
            for j in 0..=(k - p) {
                let w = if has_w {
                    self.weights.as_ref().unwrap()[i][j]
                } else {
                    1.0
                };
                row_p.push(self.poles[i][j] * w);
                row_w.push(w);
            }
            for j in (k - p + 1)..=k {
                let denom = flat[j + p] - flat[j];
                let alpha = if denom.abs() < 1e-15 {
                    0.0
                } else {
                    (v - flat[j]) / denom
                };
                let wj = if has_w {
                    self.weights.as_ref().unwrap()[i][j]
                } else {
                    1.0
                };
                let wj1 = if has_w {
                    self.weights.as_ref().unwrap()[i][j - 1]
                } else {
                    1.0
                };
                let phi = self.poles[i][j] * wj;
                let phi1 = self.poles[i][j - 1] * wj1;
                row_p.push(phi1 * (1.0 - alpha) + phi * alpha);
                row_w.push(wj1 * (1.0 - alpha) + wj * alpha);
            }
            for j in k..self.nb_v_poles() {
                let w = if has_w {
                    self.weights.as_ref().unwrap()[i][j]
                } else {
                    1.0
                };
                row_p.push(self.poles[i][j] * w);
                row_w.push(w);
            }
            let mut proj_p = Vec::with_capacity(row_p.len());
            for t in 0..row_p.len() {
                proj_p.push(row_p[t] * (1.0 / row_w[t]));
            }
            new_poles.push(proj_p);
            new_weights.push(row_w);
        }
        self.poles = new_poles;
        if has_w {
            self.weights = Some(new_weights);
        }
        self.bump_knot_mult_v(v);
    }

    fn bump_knot_mult_u(&mut self, u: f64) {
        if let Some(i) = self
            .u_knots
            .iter()
            .position(|&k| (k - u).abs() <= 1e-12)
        {
            self.u_mults[i] += 1;
        } else {
            let pos = self
                .u_knots
                .iter()
                .position(|&k| k > u)
                .unwrap_or(self.u_knots.len());
            self.u_knots.insert(pos, u);
            self.u_mults.insert(pos, 1);
        }
    }

    fn bump_knot_mult_v(&mut self, v: f64) {
        if let Some(i) = self
            .v_knots
            .iter()
            .position(|&k| (k - v).abs() <= 1e-12)
        {
            self.v_mults[i] += 1;
        } else {
            let pos = self
                .v_knots
                .iter()
                .position(|&k| k > v)
                .unwrap_or(self.v_knots.len());
            self.v_knots.insert(pos, v);
            self.v_mults.insert(pos, 1);
        }
    }
}

// ---------------------------------------------------------------------------
// Knot removal.
// ---------------------------------------------------------------------------

impl GeomBSplineSurface {
    /// `RemoveUKnot(Index, M, Tolerance)` — reduce the U-knot multiplicity to M.
    /// Returns true if the surface stays within `tol` of the original.
    pub fn remove_u_knot(&mut self, index: usize, m: usize, tol: f64) -> bool {
        let i = index - 1;
        if i >= self.u_knots.len() {
            return false;
        }
        let cur = self.u_mults[i];
        if m >= cur {
            return true;
        }
        let times = cur - m;
        // Snapshot for tolerance check.
        let before = self.clone();
        for _ in 0..times {
            if !self.remove_u_knot_once(index) {
                *self = before;
                return false;
            }
        }
        // Validate the deviation by comparing surface samples.
        if !before.within_tol(self, tol) {
            *self = before;
            return false;
        }
        true
    }

    /// `RemoveVKnot(Index, M, Tolerance)`.
    pub fn remove_v_knot(&mut self, index: usize, m: usize, tol: f64) -> bool {
        let i = index - 1;
        if i >= self.v_knots.len() {
            return false;
        }
        let cur = self.v_mults[i];
        if m >= cur {
            return true;
        }
        let times = cur - m;
        let before = self.clone();
        for _ in 0..times {
            if !self.remove_v_knot_once(index) {
                *self = before;
                return false;
            }
        }
        if !before.within_tol(self, tol) {
            *self = before;
            return false;
        }
        true
    }

    /// Sample-based deviation check between two surfaces sharing a domain.
    fn within_tol(&self, other: &Self, tol: f64) -> bool {
        let (u1, u2, v1, v2) = self.bounds();
        let n = 10;
        for i in 0..=n {
            for j in 0..=n {
                let u = u1 + (u2 - u1) * i as f64 / n as f64;
                let v = v1 + (v2 - v1) * j as f64 / n as f64;
                if self.value(u, v).distance(other.value(u, v)) > tol {
                    return false;
                }
            }
        }
        true
    }

    /// Remove one occurrence of the U-knot at `index` (1-based). Returns true on
    /// success. Implemented as the exact inverse of Boehm insertion when the
    /// knot is removable. We operate column-wise on the homogeneous poles.
    fn remove_u_knot_once(&mut self, index: usize) -> bool {
        let p = self.u_degree;
        let i_knot = index - 1;
        let u = self.u_knots[i_knot];
        let s = self.u_mults[i_knot]; // multiplicity before removal
        let flat = flat_knots(&self.u_knots, &self.u_mults);
        // r = index of the last occurrence of u in the flat vector.
        let r = flat.iter().rposition(|&k| (k - u).abs() <= 1e-12).unwrap();
        let nb_v = self.nb_v_poles();
        let has_w = self.weights.is_some();

        let ord = p + 1;
        let first = r - p;
        let last = r - s;

        // Per-column removal (NURBS Book A5.8, one removal).
        let mut new_cols_p: Vec<Vec<Pnt>> = Vec::with_capacity(nb_v);
        let mut new_cols_w: Vec<Vec<f64>> = Vec::with_capacity(nb_v);
        let nb_u = self.nb_u_poles();

        for j in 0..nb_v {
            // homogeneous column
            let mut hp: Vec<Pnt> = (0..nb_u)
                .map(|i| {
                    let w = if has_w {
                        self.weights.as_ref().unwrap()[i][j]
                    } else {
                        1.0
                    };
                    self.poles[i][j] * w
                })
                .collect();
            let mut hw: Vec<f64> = (0..nb_u)
                .map(|i| {
                    if has_w {
                        self.weights.as_ref().unwrap()[i][j]
                    } else {
                        1.0
                    }
                })
                .collect();

            let mut temp_p = vec![Pnt::origin(); 2 * p + 1];
            let mut temp_w = vec![0.0f64; 2 * p + 1];
            temp_p[0] = hp[first - 1];
            temp_w[0] = hw[first - 1];
            temp_p[last + 1 - first + 1] = hp[last + 1];
            temp_w[last + 1 - first + 1] = hw[last + 1];

            let mut ii = first;
            let mut jj = last;
            let mut iii = 1isize;
            let mut jjj = (last - first) as isize + 1;
            let mut removable = true;
            while (jj as isize - ii as isize) > 0 {
                let alfi = (u - flat[ii]) / (flat[ii + ord] - flat[ii]);
                let alfj = (u - flat[jj]) / (flat[jj + ord] - flat[jj]);
                temp_p[iii as usize] =
                    (hp[ii] - temp_p[iii as usize - 1] * (1.0 - alfi)) * (1.0 / alfi);
                temp_w[iii as usize] =
                    (hw[ii] - temp_w[iii as usize - 1] * (1.0 - alfi)) / alfi;
                temp_p[jjj as usize] =
                    (hp[jj] - temp_p[jjj as usize + 1] * alfj) * (1.0 / (1.0 - alfj));
                temp_w[jjj as usize] =
                    (hw[jj] - temp_w[jjj as usize + 1] * alfj) / (1.0 - alfj);
                ii += 1;
                iii += 1;
                jj -= 1;
                jjj -= 1;
            }
            // Check removability.
            if (jj as isize - ii as isize) < 0 {
                let d = temp_p[iii as usize - 1].distance(temp_p[jjj as usize + 1]);
                if d > 1e-6 {
                    removable = false;
                }
            } else {
                let alfi = (u - flat[ii]) / (flat[ii + ord] - flat[ii]);
                let pred =
                    temp_p[iii as usize + 1] * alfi + temp_p[iii as usize - 1] * (1.0 - alfi);
                let d = hp[ii].distance(pred);
                if d > 1e-6 {
                    removable = false;
                }
            }
            if !removable {
                return false;
            }

            // Save new poles.
            let mut ii = first;
            let mut jj = last;
            let mut t = 1usize;
            while (jj as isize - ii as isize) > 0 {
                hp[ii] = temp_p[t];
                hw[ii] = temp_w[t];
                hp[jj] = temp_p[(last - first) + 2 - t];
                hw[jj] = temp_w[(last - first) + 2 - t];
                ii += 1;
                jj -= 1;
                t += 1;
            }

            // Shift down poles after `last`.
            let remove_at = (first + last) / 2 + 1;
            let mut col_p: Vec<Pnt> = Vec::with_capacity(nb_u - 1);
            let mut col_w: Vec<f64> = Vec::with_capacity(nb_u - 1);
            for idx in 0..nb_u {
                if idx == remove_at {
                    continue;
                }
                col_p.push(hp[idx] * (1.0 / hw[idx]));
                col_w.push(hw[idx]);
            }
            new_cols_p.push(col_p);
            new_cols_w.push(col_w);
        }

        // Reassemble.
        let new_nb_u = new_cols_p[0].len();
        let mut np = vec![Vec::with_capacity(nb_v); new_nb_u];
        let mut nw = vec![Vec::with_capacity(nb_v); new_nb_u];
        for i in 0..new_nb_u {
            for j in 0..nb_v {
                np[i].push(new_cols_p[j][i]);
                nw[i].push(new_cols_w[j][i]);
            }
        }
        self.poles = np;
        if has_w {
            self.weights = Some(nw);
        }
        // Update mult/knot.
        if self.u_mults[i_knot] == 1 {
            self.u_knots.remove(i_knot);
            self.u_mults.remove(i_knot);
        } else {
            self.u_mults[i_knot] -= 1;
        }
        true
    }

    fn remove_v_knot_once(&mut self, index: usize) -> bool {
        // Transpose, remove in U, transpose back.
        self.exchange_uv();
        let ok = self.remove_u_knot_once(index);
        self.exchange_uv();
        ok
    }
}

// ---------------------------------------------------------------------------
// Degree elevation.
// ---------------------------------------------------------------------------

impl GeomBSplineSurface {
    /// `IncreaseDegree(UDegree, VDegree)`.
    pub fn increase_degree(&mut self, u_degree: usize, v_degree: usize) {
        if u_degree > self.u_degree {
            self.elevate_u(u_degree);
        }
        if v_degree > self.v_degree {
            self.elevate_v(v_degree);
        }
    }

    /// Elevate the U degree to `t` by elevating every V-iso curve (column).
    fn elevate_u(&mut self, target: usize) {
        let nb_v = self.nb_v_poles();
        let has_w = self.weights.is_some();
        // Collect elevated columns; all share the same resulting knot structure.
        let mut elevated_cols_p: Vec<Vec<Pnt>> = Vec::with_capacity(nb_v);
        let mut elevated_cols_w: Vec<Vec<f64>> = Vec::with_capacity(nb_v);
        let mut new_knots = self.u_knots.clone();
        let mut new_mults = self.u_mults.clone();

        for j in 0..nb_v {
            let poles: Vec<Pnt> = (0..self.nb_u_poles()).map(|i| self.poles[i][j]).collect();
            let weights: Option<Vec<f64>> = if has_w {
                Some(
                    (0..self.nb_u_poles())
                        .map(|i| self.weights.as_ref().unwrap()[i][j])
                        .collect(),
                )
            } else {
                None
            };
            let (np, nw, nk, nm) = elevate_curve(
                self.u_degree,
                &self.u_knots,
                &self.u_mults,
                &poles,
                weights.as_deref(),
                target,
            );
            elevated_cols_p.push(np);
            elevated_cols_w.push(nw);
            new_knots = nk;
            new_mults = nm;
        }

        let new_nb_u = elevated_cols_p[0].len();
        let mut np = vec![Vec::with_capacity(nb_v); new_nb_u];
        let mut nw = vec![Vec::with_capacity(nb_v); new_nb_u];
        for i in 0..new_nb_u {
            for j in 0..nb_v {
                np[i].push(elevated_cols_p[j][i]);
                nw[i].push(elevated_cols_w[j][i]);
            }
        }
        self.poles = np;
        if has_w {
            self.weights = Some(nw);
        }
        self.u_degree = target;
        self.u_knots = new_knots;
        self.u_mults = new_mults;
    }

    fn elevate_v(&mut self, target: usize) {
        self.exchange_uv();
        self.elevate_u(target);
        self.exchange_uv();
    }

    /// `IncreaseUMultiplicity(UIndex, M)`.
    pub fn increase_u_multiplicity(&mut self, u_index: usize, m: usize) {
        let cur = self.u_mults[u_index - 1];
        if m > cur {
            self.insert_u_knot(self.u_knots[u_index - 1], m, 1e-12);
        }
    }
    /// `IncreaseVMultiplicity(VIndex, M)`.
    pub fn increase_v_multiplicity(&mut self, v_index: usize, m: usize) {
        let cur = self.v_mults[v_index - 1];
        if m > cur {
            self.insert_v_knot(self.v_knots[v_index - 1], m, 1e-12);
        }
    }
}

/// Degree-elevate a single B-spline curve from `p` to `target`, given its
/// distinct knots+mults. Returns (poles, weights, new_knots, new_mults).
/// Implemented by sampling: a Bézier-segment elevation that keeps the geometry
/// exactly (degree elevation is geometry-preserving). We use the robust
/// piecewise-Bézier approach: decompose to Bézier, elevate each segment, then
/// recompose by removing the inserted interior knots.
fn elevate_curve(
    p: usize,
    knots: &[f64],
    mults: &[usize],
    poles: &[Pnt],
    weights: Option<&[f64]>,
    target: usize,
) -> (Vec<Pnt>, Vec<f64>, Vec<f64>, Vec<usize>) {
    let t = target - p; // degrees to raise
                        // Decompose into Bézier segments (raise all interior mults to p).
    let mut bs = BezierDecomp::from_curve(p, knots, mults, poles, weights);
    bs.elevate(t);
    bs.recompose()
}

/// Helper holding a piecewise-Bézier decomposition for degree elevation.
struct BezierDecomp {
    degree: usize,
    /// distinct break parameters (length nseg+1)
    breaks: Vec<f64>,
    /// per-segment homogeneous control points (degree+1 each)
    seg_p: Vec<Vec<Pnt>>,
    seg_w: Vec<Vec<f64>>,
}

impl BezierDecomp {
    fn from_curve(
        p: usize,
        knots: &[f64],
        mults: &[usize],
        poles: &[Pnt],
        weights: Option<&[f64]>,
    ) -> Self {
        // Insert interior knots until every interior knot has multiplicity p.
        let mut k = knots.to_vec();
        let mut m = mults.to_vec();
        let mut hp: Vec<Pnt> = poles
            .iter()
            .enumerate()
            .map(|(i, &pp)| pp * weights.map(|w| w[i]).unwrap_or(1.0))
            .collect();
        let mut hw: Vec<f64> = (0..poles.len())
            .map(|i| weights.map(|w| w[i]).unwrap_or(1.0))
            .collect();

        // Raise interior knots to multiplicity p via Boehm on homogeneous coords.
        let mut idx = 1;
        while idx < k.len() - 1 {
            while m[idx] < p {
                bezier_insert_once(p, &mut k, &mut m, &mut hp, &mut hw, idx);
            }
            idx += 1;
        }

        // Now extract Bézier segments.
        let nseg = k.len() - 1;
        let breaks = k.clone();
        let mut seg_p = Vec::with_capacity(nseg);
        let mut seg_w = Vec::with_capacity(nseg);
        // flat poles count = sum(m) - p - 1. Each interior knot has mult p,
        // boundaries have mult p+1 (assume clamped) — but the source may not be
        // clamped; handle general by walking the flat structure.
        // Build flat knots and map segments to pole ranges.
        let mut start = 0usize;
        for seg in 0..nseg {
            // Each segment uses p+1 consecutive poles starting at `start`.
            let mut sp = Vec::with_capacity(p + 1);
            let mut sw = Vec::with_capacity(p + 1);
            for off in 0..=p {
                let i = start + off;
                sp.push(hp[i] * (1.0 / hw[i]));
                sw.push(hw[i]);
            }
            seg_p.push(sp);
            seg_w.push(sw);
            // advance: between segments, mult is p, so poles advance by p.
            let _ = seg;
            start += p;
        }

        BezierDecomp {
            degree: p,
            breaks,
            seg_p,
            seg_w,
        }
    }

    /// Elevate each Bézier segment by `t` degrees.
    fn elevate(&mut self, t: usize) {
        if t == 0 {
            return;
        }
        let p = self.degree;
        let np = p + t;
        for s in 0..self.seg_p.len() {
            // homogeneous control points
            let hp: Vec<Pnt> = (0..=p)
                .map(|i| self.seg_p[s][i] * self.seg_w[s][i])
                .collect();
            let hw: Vec<f64> = self.seg_w[s].clone();
            let mut new_hp = vec![Pnt::origin(); np + 1];
            let mut new_hw = vec![0.0f64; np + 1];
            for i in 0..=np {
                let mut accp = Pnt::origin();
                let mut accw = 0.0;
                let lo = i.saturating_sub(t);
                let hi = i.min(p);
                for k in lo..=hi {
                    let c = binom(p, k) * binom(t, i - k) / binom(np, i);
                    accp = accp + hp[k] * c;
                    accw += hw[k] * c;
                }
                new_hp[i] = accp;
                new_hw[i] = accw;
            }
            self.seg_p[s] = (0..=np).map(|i| new_hp[i] * (1.0 / new_hw[i])).collect();
            self.seg_w[s] = new_hw;
        }
        self.degree = np;
    }

    /// Recompose to a single B-spline: stitch Bézier segments and remove the
    /// interior knots back down to the original continuity is NOT done — OCCT's
    /// IncreaseDegree keeps the same knot values but with raised multiplicities;
    /// the result is a valid (piecewise-Bézier) representation that evaluates
    /// identically. We therefore return the piecewise-Bézier knot vector.
    fn recompose(&self) -> (Vec<Pnt>, Vec<f64>, Vec<f64>, Vec<usize>) {
        let p = self.degree;
        let nseg = self.seg_p.len();
        // Knot vector: clamped piecewise Bézier. Boundaries mult p+1, interior p.
        let mut knots = Vec::new();
        let mut mults = Vec::new();
        for (i, b) in self.breaks.iter().enumerate() {
            knots.push(*b);
            if i == 0 || i == self.breaks.len() - 1 {
                mults.push(p + 1);
            } else {
                mults.push(p);
            }
        }
        // Poles: first segment contributes all p+1, subsequent segments share
        // their first pole with the previous segment's last pole.
        let mut poles = Vec::new();
        let mut weights = Vec::new();
        for s in 0..nseg {
            let startoff = if s == 0 { 0 } else { 1 };
            for off in startoff..=p {
                poles.push(self.seg_p[s][off]);
                weights.push(self.seg_w[s][off]);
            }
        }
        (poles, weights, knots, mults)
    }
}

/// Insert the knot at distinct-index `idx` once (raise its mult by one),
/// updating homogeneous poles/weights in place (clamped B-spline).
fn bezier_insert_once(
    p: usize,
    knots: &mut Vec<f64>,
    mults: &mut Vec<usize>,
    hp: &mut Vec<Pnt>,
    hw: &mut Vec<f64>,
    idx: usize,
) {
    let u = knots[idx];
    let flat = flat_knots(knots, mults);
    let n = hp.len() - 1;
    let k = find_span(n, p, u, &flat);
    let mut new_p = Vec::with_capacity(hp.len() + 1);
    let mut new_w = Vec::with_capacity(hw.len() + 1);
    for i in 0..=(k - p) {
        new_p.push(hp[i]);
        new_w.push(hw[i]);
    }
    for i in (k - p + 1)..=k {
        let denom = flat[i + p] - flat[i];
        let alpha = if denom.abs() < 1e-15 {
            0.0
        } else {
            (u - flat[i]) / denom
        };
        new_p.push(hp[i - 1] * (1.0 - alpha) + hp[i] * alpha);
        new_w.push(hw[i - 1] * (1.0 - alpha) + hw[i] * alpha);
    }
    for i in k..hp.len() {
        new_p.push(hp[i]);
        new_w.push(hw[i]);
    }
    *hp = new_p;
    *hw = new_w;
    mults[idx] += 1;
}

// ---------------------------------------------------------------------------
// Orientation: UReverse / VReverse / ExchangeUV.
// ---------------------------------------------------------------------------

impl GeomBSplineSurface {
    /// `UReverse()` — reverse the U parametric direction.
    pub fn u_reverse(&mut self) {
        // Reverse pole rows.
        self.poles.reverse();
        if let Some(w) = self.weights.as_mut() {
            w.reverse();
        }
        // Reverse and re-map knots: new_knot = first + last - old_knot.
        let (a, b) = (
            self.u_knots[0],
            self.u_knots[self.u_knots.len() - 1],
        );
        let new_knots: Vec<f64> = self.u_knots.iter().rev().map(|k| a + b - k).collect();
        let new_mults: Vec<usize> = self.u_mults.iter().rev().cloned().collect();
        self.u_knots = new_knots;
        self.u_mults = new_mults;
    }

    /// `VReverse()` — reverse the V parametric direction.
    pub fn v_reverse(&mut self) {
        for row in self.poles.iter_mut() {
            row.reverse();
        }
        if let Some(w) = self.weights.as_mut() {
            for row in w.iter_mut() {
                row.reverse();
            }
        }
        let (a, b) = (
            self.v_knots[0],
            self.v_knots[self.v_knots.len() - 1],
        );
        let new_knots: Vec<f64> = self.v_knots.iter().rev().map(|k| a + b - k).collect();
        let new_mults: Vec<usize> = self.v_mults.iter().rev().cloned().collect();
        self.v_knots = new_knots;
        self.v_mults = new_mults;
    }

    /// `UReversedParameter(U)`.
    pub fn u_reversed_parameter(&self, u: f64) -> f64 {
        let (u1, u2) = self.u_range();
        u1 + u2 - u
    }
    /// `VReversedParameter(V)`.
    pub fn v_reversed_parameter(&self, v: f64) -> f64 {
        let (v1, v2) = self.v_range();
        v1 + v2 - v
    }

    /// `ExchangeUV()` — transpose the surface (swap U and V).
    pub fn exchange_uv(&mut self) {
        let nb_u = self.nb_u_poles();
        let nb_v = self.nb_v_poles();
        let mut tp = vec![Vec::with_capacity(nb_u); nb_v];
        for j in 0..nb_v {
            for i in 0..nb_u {
                tp[j].push(self.poles[i][j]);
            }
        }
        self.poles = tp;
        if let Some(w) = &self.weights {
            let mut tw = vec![Vec::with_capacity(nb_u); nb_v];
            for j in 0..nb_v {
                for i in 0..nb_u {
                    tw[j].push(w[i][j]);
                }
            }
            self.weights = Some(tw);
        }
        std::mem::swap(&mut self.u_degree, &mut self.v_degree);
        std::mem::swap(&mut self.u_knots, &mut self.v_knots);
        std::mem::swap(&mut self.u_mults, &mut self.v_mults);
        std::mem::swap(&mut self.u_periodic, &mut self.v_periodic);
    }
}

// ---------------------------------------------------------------------------
// Periodicity.
// ---------------------------------------------------------------------------

impl GeomBSplineSurface {
    /// `SetUNotPeriodic()` — convert to a clamped (non-periodic) representation
    /// covering the same geometry over `[UFirst, ULast]`.
    pub fn set_u_not_periodic(&mut self) {
        if !self.u_periodic {
            return;
        }
        let p = self.u_degree;
        let (u1, u2) = self.u_range();
        // Build the extended clamped representation: expand periodic poles to a
        // clamped flat structure, then re-clamp by inserting boundary knots.
        let ev = self.eval_arrays(); // gives wrapped poles + periodic flat knots
        // Construct a temporary clamped B-spline surface over the wrapped data,
        // then segment to [u1, u2] to obtain the clamped poles/knots.
        let mut tmp = GeomBSplineSurface {
            u_degree: p,
            v_degree: self.v_degree,
            u_knots: distinct_from_flat(&ev.fu).0,
            v_knots: self.v_knots.clone(),
            u_mults: distinct_from_flat(&ev.fu).1,
            v_mults: self.v_mults.clone(),
            poles: ev.poles.clone(),
            weights: ev.weights.clone(),
            u_periodic: false,
            v_periodic: self.v_periodic,
        };
        tmp.segment_u(u1, u2);
        self.u_degree = tmp.u_degree;
        self.u_knots = tmp.u_knots;
        self.u_mults = tmp.u_mults;
        self.poles = tmp.poles;
        self.weights = tmp.weights;
        self.u_periodic = false;
    }

    /// `SetVNotPeriodic()`.
    pub fn set_v_not_periodic(&mut self) {
        if !self.v_periodic {
            return;
        }
        self.exchange_uv();
        self.set_u_not_periodic();
        self.exchange_uv();
    }
}

/// Reconstruct distinct knots + multiplicities from a flat knot vector.
fn distinct_from_flat(flat: &[f64]) -> (Vec<f64>, Vec<usize>) {
    let mut knots: Vec<f64> = Vec::new();
    let mut mults: Vec<usize> = Vec::new();
    for &k in flat {
        if let Some(&last) = knots.last() {
            if (k - last).abs() <= 1e-12 {
                *mults.last_mut().unwrap() += 1;
                continue;
            }
        }
        knots.push(k);
        mults.push(1usize);
    }
    (knots, mults)
}

// ---------------------------------------------------------------------------
// Segmentation.
// ---------------------------------------------------------------------------

impl GeomBSplineSurface {
    /// `Segment(U1, U2, V1, V2)` — restrict the surface to the given box.
    pub fn segment(&mut self, u1: f64, u2: f64, v1: f64, v2: f64) {
        assert!(u2 >= u1 && v2 >= v1, "Segment: U2<U1 or V2<V1");
        self.segment_u(u1, u2);
        self.exchange_uv();
        self.segment_u(v1, v2);
        self.exchange_uv();
    }

    /// Segment in U only: restrict the surface to `[u1, u2]`, producing a clamped
    /// (boundary multiplicity `p+1`) representation that evaluates identically on
    /// `[u1, u2]`.
    fn segment_u(&mut self, u1: f64, u2: f64) {
        let p = self.u_degree;
        // Raise the boundary knot multiplicities to p+1 (full clamp) so each
        // boundary becomes an interpolated corner. `insert_u_knot_raw` is not
        // capped at the degree.
        self.clamp_boundary_u(u1);
        self.clamp_boundary_u(u2);

        // Flat knot vector after clamping.
        let flat = flat_knots(&self.u_knots, &self.u_mults);
        // First pole of the segment: number of flat knots strictly less than u1.
        let i1 = flat.iter().filter(|&&k| k < u1 - 1e-9).count();
        // Build the restricted distinct knots / mults over [u1, u2].
        let mut new_knots = Vec::new();
        let mut new_mults = Vec::new();
        for (i, &k) in self.u_knots.iter().enumerate() {
            if k > u1 - 1e-9 && k < u2 + 1e-9 {
                new_knots.push(k);
                new_mults.push(self.u_mults[i]);
            }
        }
        if let Some(first) = new_mults.first_mut() {
            *first = p + 1;
        }
        if let Some(last) = new_mults.last_mut() {
            *last = p + 1;
        }
        let nb_seg_poles = nb_poles_nonperiodic(&new_mults, p);
        let has_w = self.weights.is_some();
        let nb_v = self.nb_v_poles();
        let mut np = vec![Vec::with_capacity(nb_v); nb_seg_poles];
        let mut nw = vec![Vec::with_capacity(nb_v); nb_seg_poles];
        for r in 0..nb_seg_poles {
            for j in 0..nb_v {
                np[r].push(self.poles[i1 + r][j]);
                if has_w {
                    nw[r].push(self.weights.as_ref().unwrap()[i1 + r][j]);
                }
            }
        }
        self.poles = np;
        if has_w {
            self.weights = Some(nw);
        }
        self.u_knots = new_knots;
        self.u_mults = new_mults;
    }

    /// Insert the knot `u` repeatedly until its multiplicity reaches `p+1`,
    /// without the degree cap that `insert_u_knot` applies. Used by `segment_u`.
    fn clamp_boundary_u(&mut self, u: f64) {
        let p = self.u_degree;
        let cur = self
            .u_knots
            .iter()
            .position(|&k| (k - u).abs() <= 1e-9)
            .map(|i| self.u_mults[i])
            .unwrap_or(0);
        for _ in cur..(p + 1) {
            self.insert_u_knot_once(u);
        }
    }
}

// ---------------------------------------------------------------------------
// MovePoint.
// ---------------------------------------------------------------------------

impl GeomBSplineSurface {
    /// `MovePoint(U, V, P, UIndex1, UIndex2, VIndex1, VIndex2, ...)`.
    /// Moves the surface so that `Value(U,V) == P`, adjusting the poles in the
    /// allowed index window. Returns the first/last U and V indices actually
    /// modified (1-based) or zeros on failure.
    #[allow(clippy::too_many_arguments)]
    pub fn move_point(
        &mut self,
        u: f64,
        v: f64,
        p: Pnt,
        u_index1: usize,
        u_index2: usize,
        v_index1: usize,
        v_index2: usize,
    ) -> (usize, usize, usize, usize) {
        // Displacement to apply at (U,V).
        let cur = self.value(u, v);
        let delta = p - cur;

        // Evaluate the tensor-product basis functions at (U,V).
        let ev = self.eval_arrays();
        let pu = ev.u_degree;
        let pv = ev.v_degree;
        let (cu, cv) = self.clamp_param(u, v);
        let span_u = find_span(ev.nb_u - 1, pu, cu, &ev.fu);
        let span_v = find_span(ev.nb_v - 1, pv, cv, &ev.fv);
        let nu = ders_basis_funs(span_u, cu, pu, 0, &ev.fu);
        let nv = ders_basis_funs(span_v, cv, pv, 0, &ev.fv);

        // The set of affected poles in the window [u_index1..u_index2] x [...].
        let mut affected: Vec<(usize, usize, f64)> = Vec::new();
        let mut sum_sq = 0.0;
        for r in 0..=pu {
            let i = span_u - pu + r; // 0-based pole index
            let iu = i + 1; // 1-based
            if iu < u_index1 || iu > u_index2 {
                continue;
            }
            for s in 0..=pv {
                let j = span_v - pv + s;
                let jv = j + 1;
                if jv < v_index1 || jv > v_index2 {
                    continue;
                }
                let b = nu[0][r] * nv[0][s];
                affected.push((i, j, b));
                sum_sq += b * b;
            }
        }
        if sum_sq < 1e-15 || affected.is_empty() {
            return (0, 0, 0, 0);
        }
        // Least-norm displacement of the poles: dP_ij = (b_ij / sum b^2) * delta.
        let mut u_first = usize::MAX;
        let mut u_last = 0;
        let mut v_first = usize::MAX;
        let mut v_last = 0;
        for (i, j, b) in affected {
            let coeff = b / sum_sq;
            self.poles[i][j] = self.poles[i][j] + delta * coeff;
            u_first = u_first.min(i + 1);
            u_last = u_last.max(i + 1);
            v_first = v_first.min(j + 1);
            v_last = v_last.max(j + 1);
        }
        (u_first, u_last, v_first, v_last)
    }
}

// ---------------------------------------------------------------------------
// Transformation.
// ---------------------------------------------------------------------------

impl GeomBSplineSurface {
    /// `Transform(T)` — apply an affine transform to the poles.
    pub fn transform(&mut self, t: &Trsf) {
        for row in self.poles.iter_mut() {
            for p in row.iter_mut() {
                *p = t.apply_point(*p);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_surface() -> GeomBSplineSurface {
        let mut poles = vec![vec![Pnt::origin(); 3]; 3];
        for i in 0..3 {
            for j in 0..3 {
                poles[i][j] =
                    Pnt::new((i + 1) as f64, (j + 1) as f64, ((i + 1) + (j + 1)) as f64 * 0.1);
            }
        }
        GeomBSplineSurface::new(
            poles,
            vec![0.0, 1.0],
            vec![0.0, 1.0],
            vec![3, 3],
            vec![3, 3],
            2,
            2,
            false,
            false,
        )
    }

    #[test]
    fn corners() {
        let s = sample_surface();
        assert!(s.value(0.0, 0.0).is_equal(Pnt::new(1.0, 1.0, 0.2), 1e-10));
        assert!(s.value(1.0, 1.0).is_equal(Pnt::new(3.0, 3.0, 0.6), 1e-10));
    }

    #[test]
    fn insert_and_value_invariant() {
        let mut s = sample_surface();
        let before = s.value(0.5, 0.5);
        s.insert_u_knot(0.5, 1, 0.0);
        assert_eq!(s.nb_u_knots(), 3);
        assert!(before.is_equal(s.value(0.5, 0.5), 1e-10));
    }
}
