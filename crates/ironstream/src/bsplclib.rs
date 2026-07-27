//! `BSplCLib` — B-spline curve utility algorithms, mirroring OpenCascade's
//! `BSplCLib` package (`TKMath`).
//!
//! Provides knot-sequence construction and analysis, parameter location, basis
//! function evaluation, pole/weight extraction helpers, curve derivatives,
//! rational derivative conversion, knot insertion, cache evaluation, flat Bezier
//! knots, Schoenberg point construction, and the antiderivative polynomial
//! conversion — all in pure, zero-dependency Rust.

// occt-ref: BSplCLib

use crate::gp::Pnt;

// ---------------------------------------------------------------------------
// KnotDistribution (GeomAbs_BSplKnotDistribution)
// ---------------------------------------------------------------------------

/// B-spline knot distribution type (`BSplCLib::KnotDistribution`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KnotDistribution {
    NonUniform,
    Uniform,
    QuasiUniform,
    PiecewiseBezier,
}

// ---------------------------------------------------------------------------
// KnotSequence — build the expanded (flat) knot vector
// ---------------------------------------------------------------------------

/// `BSplCLib::KnotSequence` — expand distinct knots with multiplicities into the
/// flat (open) knot vector used by de Boor evaluation.
///
/// The output has length `sum(multiplicities)`.
pub fn knot_sequence(knots: &[f64], mults: &[i32]) -> Vec<f64> {
    let mut flat = Vec::new();
    for (k, &m) in knots.iter().zip(mults.iter()) {
        for _ in 0..m.max(0) {
            flat.push(*k);
        }
    }
    flat
}

// ---------------------------------------------------------------------------
// KnotAnalysis — classify the knot vector
// ---------------------------------------------------------------------------

/// `BSplCLib::KnotAnalysis` — classify the distribution of a knot vector.
///
/// Mirrors the OCCT algorithm: checks for piecewise-Bezier, quasi-uniform and
/// uniform distributions based on the end/interior multiplicities and the
/// knot spacing.
pub fn knot_analysis(degree: i32, knots: &[f64], mults: &[i32]) -> KnotDistribution {
    let n = knots.len();
    if n < 2 || n != mults.len() {
        return KnotDistribution::NonUniform;
    }
    let first_m = mults[0];
    let last_m = mults[n - 1];
    let deg = degree;

    let interior_all_one = mults[1..n - 1].iter().all(|&m| m == 1);
    let interior_all_deg = n <= 2 || mults[1..n - 1].iter().all(|&m| m == deg);
    let all_one = mults.iter().all(|&m| m == 1);

    // Uniform spacing check.
    let mut uniform = true;
    if n >= 2 {
        let step = knots[1] - knots[0];
        for i in 2..n {
            if (knots[i] - knots[i - 1] - step).abs() > f64::EPSILON.sqrt() {
                uniform = false;
                break;
            }
        }
    }

    // PiecewiseBezier: ends clamped (mult = deg+1), interior mult = deg.
    if first_m == deg + 1 && last_m == deg + 1 && interior_all_deg {
        return KnotDistribution::PiecewiseBezier;
    }
    if uniform {
        if all_one {
            return KnotDistribution::Uniform;
        }
        if first_m == deg + 1 && last_m == deg + 1 && interior_all_one {
            return KnotDistribution::QuasiUniform;
        }
    }
    KnotDistribution::NonUniform
}

// ---------------------------------------------------------------------------
// LocateParameter — find the knot span bracket for a parameter
// ---------------------------------------------------------------------------

/// `BSplCLib::LocateParameter` — locate `u` in the flat knot vector.
///
/// Returns the 0-based index `i` such that `flat[i] <= u < flat[i+1]`,
/// clamped to the valid range. This is identical to the OCCT `Hunt` procedure.
///
/// `degree` is used to clamp to the active interval `[flat[degree],
/// flat[n_poles]]`.
pub fn locate_parameter(flat_knots: &[f64], degree: usize, n_poles: usize, u: f64) -> usize {
    hunt(flat_knots, u).clamp(degree, n_poles.saturating_sub(1))
}

/// `BSplCLib::Hunt` — binary search for the span index `i` such that
/// `knots[i] <= u < knots[i+1]` in the flat knot vector.
///
/// Returns 0 when `u < knots[0]`, and `knots.len()-2` when `u >= knots[last]`.
pub fn hunt(knots: &[f64], u: f64) -> usize {
    if knots.is_empty() {
        return 0;
    }
    let n = knots.len();
    if u >= knots[n - 1] {
        return n.saturating_sub(2);
    }
    if u <= knots[0] {
        return 0;
    }
    let mut lo = 0usize;
    let mut hi = n - 1;
    while hi - lo > 1 {
        let mid = (lo + hi) / 2;
        if knots[mid] <= u {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    lo
}

// ---------------------------------------------------------------------------
// FirstUKnotIndex / LastUKnotIndex
// ---------------------------------------------------------------------------

/// `BSplCLib::FirstUKnotIndex` — the index (0-based) into the distinct-knot
/// array whose cumulative multiplicity first reaches `degree + 1`.
pub fn first_u_knot_index(degree: i32, mults: &[i32]) -> usize {
    let target = degree + 1;
    let mut acc = 0;
    for (i, &m) in mults.iter().enumerate() {
        acc += m;
        if acc >= target {
            return i;
        }
    }
    0
}

/// `BSplCLib::LastUKnotIndex` — the index (0-based) into the distinct-knot
/// array whose trailing cumulative multiplicity first reaches `degree + 1`.
pub fn last_u_knot_index(degree: i32, mults: &[i32]) -> usize {
    let target = degree + 1;
    let mut acc = 0;
    let n = mults.len();
    for (i, &m) in mults.iter().enumerate().rev() {
        acc += m;
        if acc >= target {
            return i;
        }
    }
    n.saturating_sub(1)
}

// ---------------------------------------------------------------------------
// Poles / Weights helpers
// ---------------------------------------------------------------------------

/// Extract the `degree + 1` poles that influence the B-spline at a span.
///
/// `span` is the 0-based flat-knot span index (from `locate_parameter` or
/// `hunt`). Returns the slice `poles[span - degree ..= span]`.
pub fn active_poles(poles: &[Pnt], span: usize, degree: usize) -> &[Pnt] {
    let lo = span.saturating_sub(degree);
    let hi = (span + 1).min(poles.len());
    &poles[lo..hi]
}

/// Extract the active weights corresponding to the active poles at `span`.
pub fn active_weights(weights: &[f64], span: usize, degree: usize) -> &[f64] {
    let lo = span.saturating_sub(degree);
    let hi = (span + 1).min(weights.len());
    &weights[lo..hi]
}

// ---------------------------------------------------------------------------
// EvalBsplineBasis — evaluate all non-zero B-spline basis functions
// ---------------------------------------------------------------------------

/// `BSplCLib::EvalBsplineBasis` (Cox–de Boor, Piegl-Tiller A2.2).
///
/// Evaluates the `degree+1` non-zero B-spline basis functions at parameter
/// `u` in span `span` of the flat knot vector.
///
/// `basis` must have length `>= degree + 1`; on return, `basis[j]` holds
/// `N_{span-degree+j, degree}(u)`.
pub fn eval_bspline_basis(span: usize, u: f64, degree: usize, knots: &[f64], basis: &mut [f64]) {
    let p = degree;
    let mut left = vec![0.0; p + 1];
    let mut right = vec![0.0; p + 1];
    basis[0] = 1.0;
    for j in 1..=p {
        let kl = (span + 1).saturating_sub(j);
        left[j] = u - knots[kl];
        right[j] = knots[span + j] - u;
        let mut saved = 0.0;
        for r in 0..j {
            let denom = right[r + 1] + left[j - r];
            let temp = if denom.abs() < 1e-300 {
                0.0
            } else {
                basis[r] / denom
            };
            basis[r] = saved + right[r + 1] * temp;
            saved = left[j - r] * temp;
        }
        basis[j] = saved;
    }
}

// ---------------------------------------------------------------------------
// D0 / D1 / D2 / D3 / DN — curve derivatives
// ---------------------------------------------------------------------------

/// `BSplCLib::D0` — evaluate the B-spline curve position at `u`.
///
/// `flat_knots`: expanded knot vector.
/// `poles`: control points.
/// `weights`: weights (pass all-ones for non-rational).
pub fn d0(u: f64, flat_knots: &[f64], poles: &[Pnt], weights: &[f64]) -> Pnt {
    let p = flat_knots.len().saturating_sub(poles.len() + 1);
    let n = poles.len() - 1;
    let u = u.clamp(flat_knots[p], flat_knots[n + 1]);
    let span = find_span(n, p, u, flat_knots);
    let mut d = [Pnt::origin(); 26];
    let mut w = [0.0f64; 26];
    for j in 0..=p {
        let idx = span - p + j;
        let wt = weights[idx];
        d[j] = poles[idx] * wt;
        w[j] = wt;
    }
    deboor_step(&mut d, &mut w, p, span, u, flat_knots);
    let wp = if w[p].abs() < 1e-15 { 1.0 } else { w[p] };
    d[p] * (1.0 / wp)
}

/// `BSplCLib::D1` — evaluate position and first derivative.
///
/// Returns `(point, tangent)`.
pub fn d1(u: f64, flat_knots: &[f64], poles: &[Pnt], weights: &[f64]) -> (Pnt, Pnt) {
    let p = flat_knots.len().saturating_sub(poles.len() + 1);
    let n = poles.len() - 1;
    let u = u.clamp(flat_knots[p], flat_knots[n + 1]);
    let ders = curve_derivs(flat_knots, poles, weights, p, u, 1, false);
    (ders[0], ders[1])
}

/// `BSplCLib::D2` — evaluate position, first and second derivatives.
///
/// Returns `(point, tangent, curvature)`.
pub fn d2(u: f64, flat_knots: &[f64], poles: &[Pnt], weights: &[f64]) -> (Pnt, Pnt, Pnt) {
    let p = flat_knots.len().saturating_sub(poles.len() + 1);
    let n = poles.len() - 1;
    let u = u.clamp(flat_knots[p], flat_knots[n + 1]);
    let ders = curve_derivs(flat_knots, poles, weights, p, u, 2, false);
    (ders[0], ders[1], ders[2])
}

/// `BSplCLib::D3` — evaluate position and first three derivatives.
///
/// Returns `(point, d1, d2, d3)`.
pub fn d3(
    u: f64,
    flat_knots: &[f64],
    poles: &[Pnt],
    weights: &[f64],
) -> (Pnt, Pnt, Pnt, Pnt) {
    let p = flat_knots.len().saturating_sub(poles.len() + 1);
    let n = poles.len() - 1;
    let u = u.clamp(flat_knots[p], flat_knots[n + 1]);
    let ders = curve_derivs(flat_knots, poles, weights, p, u, 3, false);
    (ders[0], ders[1], ders[2], ders[3])
}

/// `BSplCLib::DN` — evaluate the N-th derivative at `u` (N >= 1).
pub fn dn(n_deriv: usize, u: f64, flat_knots: &[f64], poles: &[Pnt], weights: &[f64]) -> Pnt {
    let p = flat_knots.len().saturating_sub(poles.len() + 1);
    let n = poles.len() - 1;
    let u = u.clamp(flat_knots[p], flat_knots[n + 1]);
    let ders = curve_derivs(flat_knots, poles, weights, p, u, n_deriv, false);
    ders[n_deriv]
}

// ---------------------------------------------------------------------------
// RationalDerivative — quotient rule (Piegl-Tiller A4.2)
// ---------------------------------------------------------------------------

/// `BSplCLib::RationalDerivative` — convert homogeneous derivatives
/// `(aders, wders)` to Cartesian (rational) derivatives up to order `n`.
///
/// `aders[k]` = k-th derivative of the weighted numerator curve A(u),
/// `wders[k]` = k-th derivative of the weight function w(u).
/// On return, `result[k]` = k-th derivative of the rational curve C(u) = A(u)/w(u).
pub fn rational_derivative(aders: &[Pnt], wders: &[f64], n: usize) -> Vec<Pnt> {
    let binom = pascal_triangle(n);
    let mut ck = vec![Pnt::origin(); n + 1];
    for k in 0..=n {
        let mut v = aders[k];
        for i in 1..=k {
            v = v - ck[k - i] * (binom[k][i] * wders[i]);
        }
        ck[k] = v * (1.0 / wders[0]);
    }
    ck
}

// ---------------------------------------------------------------------------
// BuildKnots — construct a knot vector from distinct knots + mults
// ---------------------------------------------------------------------------

/// `BSplCLib::BuildKnots` — alias for [`knot_sequence`], provided for API
/// symmetry with the OCCT name.
pub fn build_knots(knots: &[f64], mults: &[i32]) -> Vec<f64> {
    knot_sequence(knots, mults)
}

// ---------------------------------------------------------------------------
// BuildSchoenbergPoints — interior Greville / Schoenberg abscissae
// ---------------------------------------------------------------------------

/// `BSplCLib::BuildSchoenbergPoints` — compute the Schoenberg (Greville)
/// abscissae for a B-spline of given degree with the supplied flat knot vector.
///
/// For each pole index `i`, the Greville abscissa is:
/// `t_i = (flat[i+1] + flat[i+2] + … + flat[i+degree]) / degree`
///
/// These are interior points at which the B-spline basis function `N_{i,p}`
/// achieves its maximum value; used for curve interpolation.
pub fn build_schoenberg_points(degree: usize, flat_knots: &[f64]) -> Vec<f64> {
    if degree == 0 || flat_knots.len() < degree + 2 {
        return Vec::new();
    }
    let n_poles = flat_knots.len() - degree - 1;
    (0..n_poles)
        .map(|i| {
            let sum: f64 = (1..=degree).map(|k| flat_knots[i + k]).sum();
            sum / degree as f64
        })
        .collect()
}

// ---------------------------------------------------------------------------
// FlatBezierKnots — the flat knot vector for a Bezier curve of given degree
// ---------------------------------------------------------------------------

/// `BSplCLib::FlatBezierKnots` — return the standard flat knot vector for a
/// Bezier (piecewise single-span) curve of degree `p` over `[0, 1]`.
///
/// This is `[0.0 repeated (p+1), 1.0 repeated (p+1)]`.
pub fn flat_bezier_knots(degree: usize) -> Vec<f64> {
    let p = degree;
    let mut knots = Vec::with_capacity(2 * (p + 1));
    knots.resize(p + 1, 0.0);
    knots.resize(2 * (p + 1), 1.0);
    knots
}

// ---------------------------------------------------------------------------
// InsertKnot — single Boehm knot insertion
// ---------------------------------------------------------------------------

/// `BSplCLib::InsertKnot` (Boehm, Piegl-Tiller A5.1 single insertion).
///
/// Insert one occurrence of the knot value `u` into the B-spline defined by
/// `flat_knots`, `poles`, and `weights` of the given `degree`.
/// The degree must equal `flat_knots.len() - poles.len() - 1`.
///
/// Returns the new `(flat_knots, poles, weights)` triple. The resulting flat
/// knot vector has one additional element. When `weights` are all equal (i.e.
/// non-rational) the output weights are uniformly updated as well.
///
/// # Panics
/// Panics if `u` is outside the knot range.
pub fn insert_knot(
    degree: usize,
    u: f64,
    flat_knots: &[f64],
    poles: &[Pnt],
    weights: &[f64],
) -> (Vec<f64>, Vec<Pnt>, Vec<f64>) {
    let p = degree;
    let n = poles.len() - 1; // last pole index
    let span = find_span(n, p, u, flat_knots);
    let s = flat_knots.iter().filter(|&&k| (k - u).abs() < 1e-12).count();

    // Build homogeneous poles.
    let hpoles: Vec<Pnt> = poles
        .iter()
        .zip(weights.iter())
        .map(|(pt, &w)| *pt * w)
        .collect();
    let hw: Vec<f64> = weights.to_vec();

    let mut new_h = vec![Pnt::origin(); poles.len() + 1];
    let mut new_w = vec![0.0f64; weights.len() + 1];

    // Unaffected left: [0..=span-p].
    new_h[..=(span - p)].copy_from_slice(&hpoles[..=(span - p)]);
    new_w[..=(span - p)].copy_from_slice(&hw[..=(span - p)]);
    // Unaffected right: indices span-s+1..=n become span-s+2..=n+1.
    new_h[(span - s + 1)..=(n + 1)].copy_from_slice(&hpoles[(span - s)..=n]);
    new_w[(span - s + 1)..=(n + 1)].copy_from_slice(&hw[(span - s)..=n]);

    // Affected section.
    let temp: Vec<Pnt> = hpoles[(span - p)..=(span - s)].to_vec();
    let tempw: Vec<f64> = hw[(span - p)..=(span - s)].to_vec();

    for i in 0..(p - s) {
        let l = span - p + 1 + i;
        let denom = flat_knots[i + span + 1] - flat_knots[l];
        let alpha = if denom.abs() < 1e-15 {
            0.0
        } else {
            (u - flat_knots[l]) / denom
        };
        let np = temp[i + 1] * alpha + temp[i] * (1.0 - alpha);
        let nw = tempw[i + 1] * alpha + tempw[i] * (1.0 - alpha);
        new_h[l] = np;
        new_w[l] = nw;
    }

    // Convert back to Cartesian.
    let new_poles: Vec<Pnt> = new_h
        .iter()
        .zip(new_w.iter())
        .map(|(pt, &w)| {
            if w.abs() < 1e-15 {
                *pt
            } else {
                *pt * (1.0 / w)
            }
        })
        .collect();

    // Build new flat knot vector.
    let pos = flat_knots
        .iter()
        .rposition(|&k| k <= u)
        .map(|i| i + 1)
        .unwrap_or(0);
    let mut new_flat = flat_knots.to_vec();
    new_flat.insert(pos, u);

    (new_flat, new_poles, new_w)
}

// ---------------------------------------------------------------------------
// CacheD0 / CacheD1 — evaluate from a local Bezier cache
// ---------------------------------------------------------------------------

/// `BSplCLib::CacheD0` — evaluate a B-spline on a local Bezier segment cache.
///
/// Given a Bezier segment's control points `cache_poles`, the local parameter
/// range `[t_min, t_max]`, and a global parameter `u`, maps `u` to the local
/// parameter and evaluates with de Casteljau.
///
/// `cache_poles` stores the `degree+1` Bezier poles of the active segment.
/// `weights` must be the same length as `cache_poles`.
pub fn cache_d0(
    u: f64,
    t_min: f64,
    t_max: f64,
    cache_poles: &[Pnt],
    weights: &[f64],
) -> Pnt {
    let t = if (t_max - t_min).abs() < 1e-15 {
        0.0
    } else {
        (u - t_min) / (t_max - t_min)
    };
    de_casteljau(t, cache_poles, weights)
}

/// `BSplCLib::CacheD1` — evaluate a B-spline from a Bezier segment cache,
/// returning position and first derivative.
///
/// Returns `(point, tangent)`.
pub fn cache_d1(
    u: f64,
    t_min: f64,
    t_max: f64,
    cache_poles: &[Pnt],
    weights: &[f64],
) -> (Pnt, Pnt) {
    let len = t_max - t_min;
    let t = if len.abs() < 1e-15 {
        0.0
    } else {
        (u - t_min) / len
    };
    let pt = de_casteljau(t, cache_poles, weights);
    // Derivative: finite difference of de Casteljau.
    let h = 1e-6f64;
    let t1 = (t + h).min(1.0);
    let t0 = (t - h).max(0.0);
    let dh = t1 - t0;
    let dv = if dh > 1e-15 {
        (de_casteljau(t1, cache_poles, weights) - de_casteljau(t0, cache_poles, weights))
            * (1.0 / dh)
    } else {
        Pnt::origin()
    };
    // Chain rule: local tangent · (1/(t_max - t_min)).
    let tangent = if len.abs() < 1e-15 {
        Pnt::origin()
    } else {
        dv * (1.0 / len)
    };
    (pt, tangent)
}

// ---------------------------------------------------------------------------
// AntiderivativePC — antiderivative in power-coefficient form
// ---------------------------------------------------------------------------

/// `BSplCLib::AntiderivativePC` — compute the antiderivative of a univariate
/// polynomial given in the **power (monomial) basis** as coefficient vector
/// `c = [c0, c1, …, cn]` (i.e. `P(t) = c0 + c1·t + … + cn·t^n`).
///
/// Returns the antiderivative coefficients `[0, c0, c1/2, c2/3, …, cn/(n+1)]`
/// (the constant of integration is zero).
pub fn antiderivative_pc(coeffs: &[f64]) -> Vec<f64> {
    let mut out = Vec::with_capacity(coeffs.len() + 1);
    out.push(0.0);
    for (i, &c) in coeffs.iter().enumerate() {
        out.push(c / (i + 1) as f64);
    }
    out
}

// ---------------------------------------------------------------------------
// Internal algorithms
// ---------------------------------------------------------------------------

/// Classic B-spline find-span (binary search, clamped).
/// `n` = last pole index, `p` = degree, `knots` = flat knot vector.
fn find_span(n: usize, p: usize, u: f64, knots: &[f64]) -> usize {
    if u >= knots[n + 1] {
        return n;
    }
    if u <= knots[p] {
        return p;
    }
    let (mut lo, mut hi) = (p, n + 1);
    let mut mid = (lo + hi) / 2;
    while u < knots[mid] || u >= knots[mid + 1] {
        if u < knots[mid] {
            hi = mid;
        } else {
            lo = mid;
        }
        mid = (lo + hi) / 2;
    }
    mid
}

/// One pass of the de Boor triangular scheme (in place), working on homogeneous
/// coordinate arrays `d` and `w` of length `degree+1` (already filled from
/// `span-degree..=span`). After the call, `d[degree]` and `w[degree]` hold the
/// result.
fn deboor_step(
    d: &mut [Pnt; 26],
    w: &mut [f64; 26],
    degree: usize,
    span: usize,
    u: f64,
    knots: &[f64],
) {
    let p = degree;
    for r in 1..=p {
        for j in (r..=p).rev() {
            let i = span - p + j;
            let denom = knots[i + p - r + 1] - knots[i];
            let alpha = if denom.abs() < 1e-15 {
                0.0
            } else {
                (u - knots[i]) / denom
            };
            d[j] = d[j - 1] * (1.0 - alpha) + d[j] * alpha;
            w[j] = w[j - 1] * (1.0 - alpha) + w[j] * alpha;
        }
    }
}

/// Derivative basis functions and all k-th order B-spline derivatives up to
/// order `nd` (Piegl-Tiller A2.3 modified for rational support).
fn ders_basis(span: usize, u: f64, p: usize, nd: usize, knots: &[f64]) -> Vec<Vec<f64>> {
    let mut ndu = vec![vec![0.0f64; p + 1]; p + 1];
    let mut left = vec![0.0f64; p + 1];
    let mut right = vec![0.0f64; p + 1];
    ndu[0][0] = 1.0;
    for j in 1..=p {
        let kl = (span + 1).saturating_sub(j);
        left[j] = u - knots[kl];
        right[j] = knots[span + j] - u;
        let mut saved = 0.0;
        for r in 0..j {
            let denom = right[r + 1] + left[j - r];
            ndu[j][r] = denom;
            let temp = if denom.abs() < 1e-300 {
                0.0
            } else {
                ndu[r][j - 1] / denom
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
    for r in 0..=p {
        let mut a = vec![vec![0.0f64; p + 1]; 2];
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
            let j2 = if (r as i64 - 1) <= pk {
                k - 1
            } else {
                p - r
            };
            for j in j1..=j2 {
                let jrk = (rk + j as i64) as usize;
                a[s2][j] = (a[s1][j] - a[s1][j - 1]) / ndu[(pk + 1) as usize][jrk];
                d += a[s2][j] * ndu[jrk][pk as usize];
            }
            if r <= pk as usize {
                a[s2][k] = -a[s1][k - 1] / ndu[(pk + 1) as usize][r];
                d += a[s2][k] * ndu[r][pk as usize];
            }
            ders[k][r] = d;
            std::mem::swap(&mut s1, &mut s2);
        }
    }
    // Multiply by correct derivative factors.
    let mut fac = p;
    for k in 1..=nd {
        for j in 0..=p {
            ders[k][j] *= fac as f64;
        }
        fac = fac.saturating_mul(p.saturating_sub(k));
    }
    ders
}

/// Compute curve derivatives (point + up to `nd` derivative vectors) of a
/// potentially rational B-spline.
///
/// `rational = false` skips the quotient rule (faster for polynomial curves).
fn curve_derivs(
    flat_knots: &[f64],
    poles: &[Pnt],
    weights: &[f64],
    degree: usize,
    u: f64,
    nd: usize,
    rational: bool,
) -> Vec<Pnt> {
    let p = degree;
    let n = poles.len() - 1;
    let du = nd.min(p);
    let span = find_span(n, p, u, flat_knots);
    let db = ders_basis(span, u, p, du, flat_knots);

    let all_one = weights.iter().all(|&w| (w - 1.0).abs() < 1e-12);
    let is_rational = rational || !all_one;

    if !is_rational {
        // Polynomial path.
        let mut ck = vec![Pnt::origin(); nd + 1];
        for k in 0..=du {
            let mut acc = Pnt::origin();
            for j in 0..=p {
                acc = acc + poles[span - p + j] * db[k][j];
            }
            ck[k] = acc;
        }
        return ck;
    }

    // Rational path: homogeneous derivatives then quotient rule.
    let mut hpoles: Vec<Pnt> = poles
        .iter()
        .zip(weights.iter())
        .map(|(pt, &w)| *pt * w)
        .collect();

    let mut aders = vec![Pnt::origin(); nd + 1];
    let mut wders = vec![0.0f64; nd + 1];
    for k in 0..=du {
        let mut acc_h = Pnt::origin();
        let mut acc_w = 0.0;
        for j in 0..=p {
            acc_h = acc_h + hpoles[span - p + j] * db[k][j];
            acc_w += weights[span - p + j] * db[k][j];
        }
        aders[k] = acc_h;
        wders[k] = acc_w;
    }
    // Ensure w0 is set from direct evaluation if du was 0 for w.
    if wders[0].abs() < 1e-15 {
        wders[0] = weights[span - p..=span].iter().zip(db[0].iter()).map(|(&w, &b)| w * b).sum();
    }

    let _ = &mut hpoles; // consumed
    rational_derivative(&aders, &wders, nd)
}

/// Rational de Casteljau evaluation on a Bezier segment.
fn de_casteljau(t: f64, poles: &[Pnt], weights: &[f64]) -> Pnt {
    let n = poles.len();
    let mut d: Vec<Pnt> = poles
        .iter()
        .zip(weights.iter())
        .map(|(p, &w)| *p * w)
        .collect();
    let mut w: Vec<f64> = weights.to_vec();
    let s = 1.0 - t;
    for r in 1..n {
        for j in 0..(n - r) {
            d[j] = d[j] * s + d[j + 1] * t;
            w[j] = w[j] * s + w[j + 1] * t;
        }
    }
    let wp = if w[0].abs() < 1e-15 { 1.0 } else { w[0] };
    d[0] * (1.0 / wp)
}

/// Pascal's triangle (binomial coefficients) up to order `n`.
fn pascal_triangle(n: usize) -> Vec<Vec<f64>> {
    let mut c = vec![vec![0.0f64; n + 1]; n + 1];
    for i in 0..=n {
        c[i][0] = 1.0;
        for j in 1..=i {
            c[i][j] = c[i - 1][j - 1] + if j < i { c[i - 1][j] } else { 0.0 };
        }
    }
    c
}

// ---------------------------------------------------------------------------
// Public facade struct (zero-sized, OCCT style)
// ---------------------------------------------------------------------------

/// Zero-sized facade exposing all `BSplCLib` algorithms as associated functions.
// occt-ref: BSplCLib
pub struct BSplCLib;

impl BSplCLib {
    /// See [`knot_sequence`].
    pub fn knot_sequence(knots: &[f64], mults: &[i32]) -> Vec<f64> {
        knot_sequence(knots, mults)
    }

    /// See [`knot_analysis`].
    pub fn knot_analysis(degree: i32, knots: &[f64], mults: &[i32]) -> KnotDistribution {
        knot_analysis(degree, knots, mults)
    }

    /// See [`locate_parameter`].
    pub fn locate_parameter(
        flat_knots: &[f64],
        degree: usize,
        n_poles: usize,
        u: f64,
    ) -> usize {
        locate_parameter(flat_knots, degree, n_poles, u)
    }

    /// See [`hunt`].
    pub fn hunt(knots: &[f64], u: f64) -> usize {
        hunt(knots, u)
    }

    /// See [`first_u_knot_index`].
    pub fn first_u_knot_index(degree: i32, mults: &[i32]) -> usize {
        first_u_knot_index(degree, mults)
    }

    /// See [`last_u_knot_index`].
    pub fn last_u_knot_index(degree: i32, mults: &[i32]) -> usize {
        last_u_knot_index(degree, mults)
    }

    /// See [`eval_bspline_basis`].
    pub fn eval_bspline_basis(
        span: usize,
        u: f64,
        degree: usize,
        knots: &[f64],
        basis: &mut [f64],
    ) {
        eval_bspline_basis(span, u, degree, knots, basis)
    }

    /// See [`d0`].
    pub fn d0(u: f64, flat_knots: &[f64], poles: &[Pnt], weights: &[f64]) -> Pnt {
        d0(u, flat_knots, poles, weights)
    }

    /// See [`d1`].
    pub fn d1(
        u: f64,
        flat_knots: &[f64],
        poles: &[Pnt],
        weights: &[f64],
    ) -> (Pnt, Pnt) {
        d1(u, flat_knots, poles, weights)
    }

    /// See [`d2`].
    pub fn d2(
        u: f64,
        flat_knots: &[f64],
        poles: &[Pnt],
        weights: &[f64],
    ) -> (Pnt, Pnt, Pnt) {
        d2(u, flat_knots, poles, weights)
    }

    /// See [`d3`].
    pub fn d3(
        u: f64,
        flat_knots: &[f64],
        poles: &[Pnt],
        weights: &[f64],
    ) -> (Pnt, Pnt, Pnt, Pnt) {
        d3(u, flat_knots, poles, weights)
    }

    /// See [`dn`].
    pub fn dn(
        n_deriv: usize,
        u: f64,
        flat_knots: &[f64],
        poles: &[Pnt],
        weights: &[f64],
    ) -> Pnt {
        dn(n_deriv, u, flat_knots, poles, weights)
    }

    /// See [`rational_derivative`].
    pub fn rational_derivative(aders: &[Pnt], wders: &[f64], n: usize) -> Vec<Pnt> {
        rational_derivative(aders, wders, n)
    }

    /// See [`build_knots`].
    pub fn build_knots(knots: &[f64], mults: &[i32]) -> Vec<f64> {
        build_knots(knots, mults)
    }

    /// See [`build_schoenberg_points`].
    pub fn build_schoenberg_points(degree: usize, flat_knots: &[f64]) -> Vec<f64> {
        build_schoenberg_points(degree, flat_knots)
    }

    /// See [`flat_bezier_knots`].
    pub fn flat_bezier_knots(degree: usize) -> Vec<f64> {
        flat_bezier_knots(degree)
    }

    /// See [`insert_knot`].
    pub fn insert_knot(
        degree: usize,
        u: f64,
        flat_knots: &[f64],
        poles: &[Pnt],
        weights: &[f64],
    ) -> (Vec<f64>, Vec<Pnt>, Vec<f64>) {
        insert_knot(degree, u, flat_knots, poles, weights)
    }

    /// See [`cache_d0`].
    pub fn cache_d0(
        u: f64,
        t_min: f64,
        t_max: f64,
        cache_poles: &[Pnt],
        weights: &[f64],
    ) -> Pnt {
        cache_d0(u, t_min, t_max, cache_poles, weights)
    }

    /// See [`cache_d1`].
    pub fn cache_d1(
        u: f64,
        t_min: f64,
        t_max: f64,
        cache_poles: &[Pnt],
        weights: &[f64],
    ) -> (Pnt, Pnt) {
        cache_d1(u, t_min, t_max, cache_poles, weights)
    }

    /// See [`antiderivative_pc`].
    pub fn antiderivative_pc(coeffs: &[f64]) -> Vec<f64> {
        antiderivative_pc(coeffs)
    }

    /// See [`active_poles`].
    pub fn active_poles(poles: &[Pnt], span: usize, degree: usize) -> &[Pnt] {
        active_poles(poles, span, degree)
    }

    /// See [`active_weights`].
    pub fn active_weights(weights: &[f64], span: usize, degree: usize) -> &[f64] {
        active_weights(weights, span, degree)
    }
}
