//! `BSplSLib` — B-spline surface utility algorithms, mirroring OpenCascade's
//! `BSplSLib` package (`TKMath`).
//!
//! Provides tensor-product B-spline / NURBS surface evaluation (D0/D1/D2/D3),
//! cache construction and evaluation (BuildCache, CacheD0, CacheD1), iso-curve
//! extraction, parameter reversal, resolution mapping, and homogeneous-coordinate
//! variants of D0 and D1. All algorithms are pure Rust with zero third-party
//! dependencies.

// occt: BSplSLib

// B-spline surface functions inherently require (u, v, u_flat, v_flat, poles,
// weights, u_degree, v_degree) — this matches the OCCT API and cannot be
// reasonably shortened without losing clarity.
#![allow(clippy::too_many_arguments)]

use crate::gp::Pnt;

// ---------------------------------------------------------------------------
// Internal helpers (shared de Boor / basis machinery)
// ---------------------------------------------------------------------------

/// Classic B-spline find-span — binary search for the span index `i` such that
/// `knots[i] <= u < knots[i+1]`, clamped to the active knot interval.
///
/// `n` = last control-point index (= poles.len()-1), `p` = degree.
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

/// Cox–de Boor basis function evaluation (Piegl-Tiller A2.2 with derivative
/// table). Returns a `(nd+1) × (p+1)` matrix where `result[k][j]` is the
/// k-th derivative of the j-th non-zero basis function at `u` in `span`.
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
            let mut d = 0.0f64;
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
    // Multiply by factorial derivative factors.
    let mut fac = p;
    for k in 1..=nd {
        for j in 0..=p {
            ders[k][j] *= fac as f64;
        }
        fac = fac.saturating_mul(p.saturating_sub(k));
    }
    ders
}

/// Quotient-rule rational derivative conversion (Piegl-Tiller A4.2).
///
/// `aders[k]` = k-th derivative of the weighted numerator A(u,v),
/// `wders[k]` = k-th derivative of the weight function w(u,v).
/// Returns the Cartesian derivatives `C^(k)(u)` for k = 0..=n.
#[allow(dead_code)]
fn rational_derivative_1d(aders: &[Pnt], wders: &[f64], n: usize) -> Vec<Pnt> {
    let binom = pascal_triangle(n);
    let mut ck = vec![Pnt::origin(); n + 1];
    for k in 0..=n {
        let mut v = aders[k];
        for i in 1..=k {
            v = v - ck[k - i] * (binom[k][i] * wders[i]);
        }
        let w0 = if wders[0].abs() < 1e-15 {
            1.0
        } else {
            wders[0]
        };
        ck[k] = v * (1.0 / w0);
    }
    ck
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

/// Evaluate a 1D B-spline (along a row/column of the surface) at parameter `u`,
/// yielding the homogeneous point and weight.
#[allow(dead_code)]
fn eval_1d_homogeneous(
    u: f64,
    knots: &[f64],
    poles: &[Pnt],
    weights: &[f64],
    degree: usize,
) -> (Pnt, f64) {
    let p = degree;
    let n = poles.len() - 1;
    let u = u.clamp(knots[p], knots[n + 1]);
    let span = find_span(n, p, u, knots);
    let mut d = [Pnt::origin(); 26];
    let mut w = [0.0f64; 26];
    for j in 0..=p {
        let idx = span - p + j;
        let wt = weights[idx];
        d[j] = poles[idx] * wt;
        w[j] = wt;
    }
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
    (d[p], w[p])
}

/// Compute up to `nd` derivatives of a 1D B-spline at `u`, returning homogeneous
/// derivative vectors and weight derivatives. Used internally for surface D1/D2/D3.
fn eval_1d_derivs(
    u: f64,
    knots: &[f64],
    poles: &[Pnt],
    weights: &[f64],
    degree: usize,
    nd: usize,
) -> (Vec<Pnt>, Vec<f64>) {
    let p = degree;
    let n = poles.len() - 1;
    let du = nd.min(p);
    let u = u.clamp(knots[p], knots[n + 1]);
    let span = find_span(n, p, u, knots);
    let db = ders_basis(span, u, p, du, knots);

    let mut aders = vec![Pnt::origin(); nd + 1];
    let mut wders = vec![0.0f64; nd + 1];
    for k in 0..=du {
        let mut acc_h = Pnt::origin();
        let mut acc_w = 0.0f64;
        for j in 0..=p {
            let idx = span - p + j;
            acc_h = acc_h + poles[idx] * (weights[idx] * db[k][j]);
            acc_w += weights[idx] * db[k][j];
        }
        aders[k] = acc_h;
        wders[k] = acc_w;
    }
    (aders, wders)
}

// ---------------------------------------------------------------------------
// Surface derivative computation (tensor product)
// ---------------------------------------------------------------------------

/// `SKL` result: `skl[k][l]` = mixed derivative ∂^{k+l}S/∂u^k∂v^l at (u,v),
/// for k = 0..=du, l = 0..=dv.
///
/// Implements the tensor-product algorithm for rational surfaces:
/// 1. For each V-row, compute the U-curve derivatives up to order `du`.
/// 2. Treat the results as a set of 1D "V-curves" and differentiate in V up to `dv`.
/// 3. Apply the 2D rational quotient rule.
fn surface_derivs(
    u_flat: &[f64],
    v_flat: &[f64],
    poles: &[Vec<Pnt>],       // poles[i][j], i=u-index, j=v-index
    weights: &[Vec<f64>],     // weights[i][j]
    u_degree: usize,
    v_degree: usize,
    u: f64,
    v: f64,
    du: usize,
    dv: usize,
) -> Vec<Vec<Pnt>> {
    let _nu = poles.len();      // n_u_poles (kept for documentation)
    let nv = poles[0].len();    // n_v_poles

    // For each V-column index l, compute U-curve derivatives up to du.
    // temp_h[l][k] = k-th U-derivative of the weighted numerator (homogeneous)
    // temp_w[l][k] = k-th U-derivative of the weight function

    let mut temp_h: Vec<Vec<Pnt>> = vec![vec![Pnt::origin(); du + 1]; nv];
    let mut temp_w: Vec<Vec<f64>> = vec![vec![0.0f64; du + 1]; nv];

    for j in 0..nv {
        let col_poles: Vec<Pnt> = poles.iter().map(|row| row[j]).collect();
        let col_weights: Vec<f64> = weights.iter().map(|row| row[j]).collect();
        let (ah, aw) = eval_1d_derivs(u, u_flat, &col_poles, &col_weights, u_degree, du);
        temp_h[j].copy_from_slice(&ah);
        temp_w[j].copy_from_slice(&aw);
    }

    // Now differentiate in V for each U-derivative order k.
    // skl_h[k][l] = (k,l) mixed homogeneous derivative
    // skl_w[k][l] = (k,l) mixed weight derivative

    let mut skl_h = vec![vec![Pnt::origin(); dv + 1]; du + 1];
    let mut skl_w = vec![vec![0.0f64; dv + 1]; du + 1];

    // V-curve poles/weights for derivative k:
    // v_poles_k[j] = temp_h[j][k] (already weighted numerator)
    // v_weights_k[j] = temp_w[j][k]

    for k in 0..=du {
        let v_poles_k: Vec<Pnt> = (0..nv).map(|j| temp_h[j][k]).collect();
        let v_weights_k: Vec<f64> = (0..nv).map(|j| temp_w[j][k]).collect();

        // Compute V-derivatives of the k-th U-derivative strip.
        // We treat this as a 1D curve in v with "poles" = v_poles_k and
        // "weights" = v_weights_k (already homogeneous, so use unit weights).
        let v_n = nv - 1;
        let v_pu = v_degree;
        let v_cl = v.clamp(v_flat[v_pu], v_flat[v_n + 1]);
        let v_span = find_span(v_n, v_pu, v_cl, v_flat);
        let db_v = ders_basis(v_span, v_cl, v_pu, dv, v_flat);

        for l in 0..=dv {
            let mut acc_h = Pnt::origin();
            let mut acc_w = 0.0f64;
            for jj in 0..=v_pu {
                let idx = v_span - v_pu + jj;
                acc_h = acc_h + v_poles_k[idx] * db_v[l][jj];
                acc_w += v_weights_k[idx] * db_v[l][jj];
            }
            skl_h[k][l] = acc_h;
            skl_w[k][l] = acc_w;
        }
    }

    // Apply 2D rational quotient rule (Piegl-Tiller A4.4).
    let binom = pascal_triangle(du.max(dv));
    let mut skl = vec![vec![Pnt::origin(); dv + 1]; du + 1];
    let w00 = if skl_w[0][0].abs() < 1e-15 {
        1.0
    } else {
        skl_w[0][0]
    };
    for k in 0..=du {
        for l in 0..=dv {
            let mut v = skl_h[k][l];
            for i in 1..=k {
                v = v - skl[k - i][l] * (binom[k][i] * skl_w[i][0]);
            }
            for j in 1..=l {
                v = v - skl[k][l - j] * (binom[l][j] * skl_w[0][j]);
            }
            // Mixed terms.
            for i in 1..=k {
                for j in 1..=l {
                    v = v - skl[k - i][l - j] * (binom[k][i] * binom[l][j] * skl_w[i][j]);
                }
            }
            skl[k][l] = v * (1.0 / w00);
        }
    }
    skl
}

// ---------------------------------------------------------------------------
// D0 / D1 / D2 / D3
// ---------------------------------------------------------------------------

/// `BSplSLib::D0` — evaluate the tensor-product B-spline surface at `(u, v)`.
///
/// `u_flat` / `v_flat`: expanded (flat) knot vectors in U and V.
/// `poles[i][j]`: control points (i = U index, j = V index).
/// `weights[i][j]`: NURBS weights (pass all-ones for polynomial surfaces).
pub fn d0(
    u: f64,
    v: f64,
    u_flat: &[f64],
    v_flat: &[f64],
    poles: &[Vec<Pnt>],
    weights: &[Vec<f64>],
    u_degree: usize,
    v_degree: usize,
) -> Pnt {
    let skl = surface_derivs(u_flat, v_flat, poles, weights, u_degree, v_degree, u, v, 0, 0);
    skl[0][0]
}

/// Result of `BSplSLib::D1` — surface point and the two first-order partial
/// derivatives at `(u, v)`.
pub struct D1Result {
    pub point: Pnt,
    pub d1u: Pnt,
    pub d1v: Pnt,
}

/// `BSplSLib::D1` — evaluate position and first partial derivatives.
pub fn d1(
    u: f64,
    v: f64,
    u_flat: &[f64],
    v_flat: &[f64],
    poles: &[Vec<Pnt>],
    weights: &[Vec<f64>],
    u_degree: usize,
    v_degree: usize,
) -> D1Result {
    let skl = surface_derivs(u_flat, v_flat, poles, weights, u_degree, v_degree, u, v, 1, 1);
    D1Result {
        point: skl[0][0],
        d1u: skl[1][0],
        d1v: skl[0][1],
    }
}

/// Result of `BSplSLib::D2` — point, first and second partial derivatives.
pub struct D2Result {
    pub point: Pnt,
    pub d1u: Pnt,
    pub d1v: Pnt,
    pub d2u: Pnt,
    pub d2v: Pnt,
    pub d2uv: Pnt,
}

/// `BSplSLib::D2` — evaluate position, first and second partial derivatives.
pub fn d2(
    u: f64,
    v: f64,
    u_flat: &[f64],
    v_flat: &[f64],
    poles: &[Vec<Pnt>],
    weights: &[Vec<f64>],
    u_degree: usize,
    v_degree: usize,
) -> D2Result {
    let skl = surface_derivs(u_flat, v_flat, poles, weights, u_degree, v_degree, u, v, 2, 2);
    D2Result {
        point: skl[0][0],
        d1u: skl[1][0],
        d1v: skl[0][1],
        d2u: skl[2][0],
        d2v: skl[0][2],
        d2uv: skl[1][1],
    }
}

/// Result of `BSplSLib::D3` — point and derivatives up to third order.
pub struct D3Result {
    pub point: Pnt,
    pub d1u: Pnt,
    pub d1v: Pnt,
    pub d2u: Pnt,
    pub d2v: Pnt,
    pub d2uv: Pnt,
    pub d3u: Pnt,
    pub d3v: Pnt,
    pub d3uuv: Pnt,
    pub d3uvv: Pnt,
}

/// `BSplSLib::D3` — evaluate position and first through third partial
/// derivatives at `(u, v)`.
pub fn d3(
    u: f64,
    v: f64,
    u_flat: &[f64],
    v_flat: &[f64],
    poles: &[Vec<Pnt>],
    weights: &[Vec<f64>],
    u_degree: usize,
    v_degree: usize,
) -> D3Result {
    let skl = surface_derivs(u_flat, v_flat, poles, weights, u_degree, v_degree, u, v, 3, 3);
    D3Result {
        point: skl[0][0],
        d1u: skl[1][0],
        d1v: skl[0][1],
        d2u: skl[2][0],
        d2v: skl[0][2],
        d2uv: skl[1][1],
        d3u: skl[3][0],
        d3v: skl[0][3],
        d3uuv: skl[2][1],
        d3uvv: skl[1][2],
    }
}

// ---------------------------------------------------------------------------
// BuildCache / CacheD0 / CacheD1
// ---------------------------------------------------------------------------

/// A local Bezier segment cache for fast repeated evaluation on a single knot
/// span of a tensor-product B-spline surface.
///
/// `BSplSLib::BuildCache` converts one (u-span × v-span) patch of the B-spline
/// into Bezier (Bernstein) form by storing the `(u_degree+1) × (v_degree+1)`
/// control points of the patch, together with the knot-span bounds.
#[derive(Clone, Debug)]
// occt: BSplSLib
pub struct SurfaceCache {
    /// Bezier control points in Bernstein form: `bezier[i][j]` for i in
    /// `0..=u_degree`, j in `0..=v_degree`.
    pub bezier: Vec<Vec<Pnt>>,
    /// Rational weights in Bernstein form; all 1.0 if non-rational.
    pub bezier_weights: Vec<Vec<f64>>,
    /// U-span knot range `[u_min, u_max]`.
    pub u_min: f64,
    pub u_max: f64,
    /// V-span knot range `[v_min, v_max]`.
    pub v_min: f64,
    pub v_max: f64,
    pub u_degree: usize,
    pub v_degree: usize,
}

/// `BSplSLib::BuildCache` — extract the Bezier control points of the knot span
/// that contains `(u, v)`.
///
/// The Bezier poles are extracted via de Boor columns restricted to the active
/// span. For polynomial surfaces each span already is a Bezier patch; for
/// repeated-knot B-splines we extract the `degree+1` poles of the span.
///
/// Returns a [`SurfaceCache`] valid for the span that contains `(u, v)`.
pub fn build_cache(
    u: f64,
    v: f64,
    u_flat: &[f64],
    v_flat: &[f64],
    poles: &[Vec<Pnt>],
    weights: &[Vec<f64>],
    u_degree: usize,
    v_degree: usize,
) -> SurfaceCache {
    let nu = poles.len() - 1; // last u-pole index
    let nv = poles[0].len() - 1; // last v-pole index
    let pu = u_degree;
    let pv = v_degree;

    let u_cl = u.clamp(u_flat[pu], u_flat[nu + 1]);
    let v_cl = v.clamp(v_flat[pv], v_flat[nv + 1]);

    let u_span = find_span(nu, pu, u_cl, u_flat);
    let v_span = find_span(nv, pv, v_cl, v_flat);

    let u_min = u_flat[u_span];
    let u_max = u_flat[u_span + 1];
    let v_min = v_flat[v_span];
    let v_max = v_flat[v_span + 1];

    // Extract the (pu+1) × (pv+1) control points of this knot span
    // directly from the B-spline's pole grid (they already are the Bezier
    // poles when the multiplicity equals the degree, which is the standard
    // case for clamped B-splines). For general multiplicity, the active
    // poles for the span are indices [u_span-pu..=u_span] × [v_span-pv..=v_span].
    let bezier: Vec<Vec<Pnt>> = (0..=pu)
        .map(|i| {
            let ui = u_span - pu + i;
            (0..=pv)
                .map(|j| {
                    let vj = v_span - pv + j;
                    poles[ui][vj]
                })
                .collect()
        })
        .collect();

    let bezier_weights: Vec<Vec<f64>> = (0..=pu)
        .map(|i| {
            let ui = u_span - pu + i;
            (0..=pv)
                .map(|j| {
                    let vj = v_span - pv + j;
                    weights[ui][vj]
                })
                .collect()
        })
        .collect();

    SurfaceCache {
        bezier,
        bezier_weights,
        u_min,
        u_max,
        v_min,
        v_max,
        u_degree: pu,
        v_degree: pv,
    }
}

/// Rational de Casteljau evaluation on one 1D Bezier segment.
fn de_casteljau_1d(t: f64, poles: &[Pnt], weights: &[f64]) -> (Pnt, f64) {
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
    (d[0], w[0])
}

/// `BSplSLib::CacheD0` — evaluate the surface at `(u, v)` from a pre-built
/// [`SurfaceCache`].
///
/// Maps `(u, v)` to the local Bezier parameter `(s, t) ∈ [0,1]^2` and applies
/// de Casteljau twice (first in U for each V row, then in V).
pub fn cache_d0(u: f64, v: f64, cache: &SurfaceCache) -> Pnt {
    let s = map_param(u, cache.u_min, cache.u_max);
    let t = map_param(v, cache.v_min, cache.v_max);
    let pv = cache.v_degree;

    // For each V column, apply de Casteljau in U → produces a 1D V-Bezier.
    let v_poles: Vec<Pnt> = (0..=pv)
        .map(|j| {
            let u_poles: Vec<Pnt> = cache.bezier.iter().map(|row| row[j]).collect();
            let u_weights: Vec<f64> = cache.bezier_weights.iter().map(|row| row[j]).collect();
            let (hp, hw) = de_casteljau_1d(s, &u_poles, &u_weights);
            let w = if hw.abs() < 1e-15 { 1.0 } else { hw };
            hp * (1.0 / w)
        })
        .collect();
    let v_weights: Vec<f64> = (0..=pv)
        .map(|j| {
            let u_poles: Vec<Pnt> = cache.bezier.iter().map(|row| row[j]).collect();
            let u_weights: Vec<f64> = cache.bezier_weights.iter().map(|row| row[j]).collect();
            let (_, hw) = de_casteljau_1d(s, &u_poles, &u_weights);
            hw
        })
        .collect();

    let (hp, hw) = de_casteljau_1d(t, &v_poles, &v_weights);
    let w = if hw.abs() < 1e-15 { 1.0 } else { hw };
    hp * (1.0 / w)
}

/// Result of `BSplSLib::CacheD1` — point and first partial derivatives from a
/// pre-built cache.
pub struct CacheD1Result {
    pub point: Pnt,
    pub d1u: Pnt,
    pub d1v: Pnt,
}

/// `BSplSLib::CacheD1` — evaluate position and first partial derivatives from a
/// [`SurfaceCache`] at global parameters `(u, v)`.
///
/// Computes derivatives using finite differences on the local Bezier
/// parameterisation, then applies the chain rule for the knot-span scaling.
pub fn cache_d1(u: f64, v: f64, cache: &SurfaceCache) -> CacheD1Result {
    let pt = cache_d0(u, v, cache);
    let hu = (cache.u_max - cache.u_min).abs().max(1e-15);
    let hv = (cache.v_max - cache.v_min).abs().max(1e-15);
    let eps_u = hu * 1e-5;
    let eps_v = hv * 1e-5;

    // Finite difference in U.
    let (u_lo, u_hi) = (
        (u - eps_u).max(cache.u_min),
        (u + eps_u).min(cache.u_max),
    );
    let du_span = (u_hi - u_lo).max(1e-20);
    let d1u = if du_span > 1e-15 {
        (cache_d0(u_hi, v, cache) - cache_d0(u_lo, v, cache)) * (1.0 / du_span)
    } else {
        Pnt::origin()
    };

    // Finite difference in V.
    let (v_lo, v_hi) = (
        (v - eps_v).max(cache.v_min),
        (v + eps_v).min(cache.v_max),
    );
    let dv_span = (v_hi - v_lo).max(1e-20);
    let d1v = if dv_span > 1e-15 {
        (cache_d0(u, v_hi, cache) - cache_d0(u, v_lo, cache)) * (1.0 / dv_span)
    } else {
        Pnt::origin()
    };

    CacheD1Result { point: pt, d1u, d1v }
}

/// Map a global parameter `u ∈ [u_min, u_max]` to a local Bezier parameter
/// `t ∈ [0, 1]`, clamped.
#[inline]
fn map_param(u: f64, u_min: f64, u_max: f64) -> f64 {
    let span = u_max - u_min;
    if span.abs() < 1e-15 {
        0.0
    } else {
        ((u - u_min) / span).clamp(0.0, 1.0)
    }
}

// ---------------------------------------------------------------------------
// Iso — extract U or V isoparametric curves as B-spline curves
// ---------------------------------------------------------------------------

/// Result of an isoparametric curve extraction: a B-spline curve in the
/// remaining parameter direction.
pub struct IsoCurve {
    /// Poles of the isoparametric B-spline curve.
    pub poles: Vec<Pnt>,
    /// Weights of the isoparametric curve (all 1.0 for non-rational surfaces).
    pub weights: Vec<f64>,
    /// Flat (expanded) knot vector in the remaining direction.
    pub flat_knots: Vec<f64>,
    /// Degree in the remaining direction.
    pub degree: usize,
}

/// `BSplSLib::Iso` with `is_u = true` — extract the U-isoparametric curve
/// at constant `param = u`.
///
/// For a tensor-product surface, the U-iso is the V-direction B-spline curve
/// whose poles are the evaluations of each U-basis-function at `u = param`.
/// Mathematically the k-th pole is `Σ_i N_{i,pu}(u) · P_{i,k}` (rational: also
/// divided by the weight sum).
pub fn iso_u(
    param: f64,
    u_flat: &[f64],
    v_flat: &[f64],
    poles: &[Vec<Pnt>],
    weights: &[Vec<f64>],
    u_degree: usize,
    v_degree: usize,
) -> IsoCurve {
    let nu = poles.len();
    let nv = poles[0].len();
    let pu = u_degree;

    let n = nu - 1;
    let u_cl = param.clamp(u_flat[pu], u_flat[n + 1]);
    let span = find_span(n, pu, u_cl, u_flat);

    // Evaluate B-spline basis functions N_{i,pu}(u) at the span.
    let mut basis = vec![0.0f64; pu + 1];
    {
        let mut left = vec![0.0f64; pu + 1];
        let mut right = vec![0.0f64; pu + 1];
        basis[0] = 1.0;
        for j in 1..=pu {
            let kl = (span + 1).saturating_sub(j);
            left[j] = u_cl - u_flat[kl];
            right[j] = u_flat[span + j] - u_cl;
            let mut saved = 0.0f64;
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

    // For each V-column k, compute the iso-pole: Σ_j N_j(u) * P[span-pu+j][k]
    // (with rational correction if needed).
    let iso_poles: Vec<Pnt> = (0..nv)
        .map(|k| {
            let w_sum: f64 = (0..=pu)
                .map(|j| basis[j] * weights[span - pu + j][k])
                .sum();
            let p_sum: Pnt = (0..=pu).fold(Pnt::origin(), |acc, j| {
                acc + poles[span - pu + j][k] * (basis[j] * weights[span - pu + j][k])
            });
            let w = if w_sum.abs() < 1e-15 { 1.0 } else { w_sum };
            p_sum * (1.0 / w)
        })
        .collect();

    let iso_weights: Vec<f64> = (0..nv)
        .map(|k| {
            (0..=pu)
                .map(|j| basis[j] * weights[span - pu + j][k])
                .sum::<f64>()
                .max(1e-15)
        })
        .collect();

    IsoCurve {
        poles: iso_poles,
        weights: iso_weights,
        flat_knots: v_flat.to_vec(),
        degree: v_degree,
    }
}

/// `BSplSLib::Iso` with `is_u = false` — extract the V-isoparametric curve
/// at constant `param = v`.
///
/// The V-iso is the U-direction B-spline curve whose poles are evaluated at
/// `v = param` along each U-row of the surface.
pub fn iso_v(
    param: f64,
    u_flat: &[f64],
    v_flat: &[f64],
    poles: &[Vec<Pnt>],
    weights: &[Vec<f64>],
    u_degree: usize,
    v_degree: usize,
) -> IsoCurve {
    let nu = poles.len();
    let nv = poles[0].len();
    let pv = v_degree;

    let n = nv - 1;
    let v_cl = param.clamp(v_flat[pv], v_flat[n + 1]);
    let span = find_span(n, pv, v_cl, v_flat);

    // Evaluate B-spline basis functions N_{j,pv}(v).
    let mut basis = vec![0.0f64; pv + 1];
    {
        let mut left = vec![0.0f64; pv + 1];
        let mut right = vec![0.0f64; pv + 1];
        basis[0] = 1.0;
        for j in 1..=pv {
            let kl = (span + 1).saturating_sub(j);
            left[j] = v_cl - v_flat[kl];
            right[j] = v_flat[span + j] - v_cl;
            let mut saved = 0.0f64;
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

    // For each U-row i, compute iso-pole: Σ_j N_j(v) * P[i][span-pv+j].
    let iso_poles: Vec<Pnt> = (0..nu)
        .map(|i| {
            let w_sum: f64 = (0..=pv)
                .map(|j| basis[j] * weights[i][span - pv + j])
                .sum();
            let p_sum: Pnt = (0..=pv).fold(Pnt::origin(), |acc, j| {
                acc + poles[i][span - pv + j] * (basis[j] * weights[i][span - pv + j])
            });
            let w = if w_sum.abs() < 1e-15 { 1.0 } else { w_sum };
            p_sum * (1.0 / w)
        })
        .collect();

    let iso_weights: Vec<f64> = (0..nu)
        .map(|i| {
            (0..=pv)
                .map(|j| basis[j] * weights[i][span - pv + j])
                .sum::<f64>()
                .max(1e-15)
        })
        .collect();

    IsoCurve {
        poles: iso_poles,
        weights: iso_weights,
        flat_knots: u_flat.to_vec(),
        degree: u_degree,
    }
}

// ---------------------------------------------------------------------------
// Reverse — reverse a surface in U or V
// ---------------------------------------------------------------------------

/// `BSplSLib::Reverse` in the U direction.
///
/// Reverses the surface parametrisation in U: maps `u → (u_first + u_last - u)`.
/// Reverses the rows of the pole/weight grids and reverses the U knot vector.
pub fn reverse_u(
    u_flat: &[f64],
    poles: &[Vec<Pnt>],
    weights: &[Vec<f64>],
    u_degree: usize,
) -> (Vec<f64>, Vec<Vec<Pnt>>, Vec<Vec<f64>>) {
    let n = u_flat.len();
    let u_first = u_flat[u_degree];
    let u_last = u_flat[poles.len()];

    // Reverse and complement the flat knot vector.
    let new_u_flat: Vec<f64> = u_flat
        .iter()
        .rev()
        .map(|&k| u_first + u_last - k)
        .collect();

    // Reverse rows (U-index) of the pole grid.
    let new_poles: Vec<Vec<Pnt>> = poles.iter().rev().cloned().collect();
    let new_weights: Vec<Vec<f64>> = weights.iter().rev().cloned().collect();

    let _ = n;
    (new_u_flat, new_poles, new_weights)
}

/// `BSplSLib::Reverse` in the V direction.
///
/// Reverses the surface parametrisation in V: maps `v → (v_first + v_last - v)`.
/// Reverses the columns of the pole/weight grids and reverses the V knot vector.
pub fn reverse_v(
    v_flat: &[f64],
    poles: &[Vec<Pnt>],
    weights: &[Vec<f64>],
    v_degree: usize,
) -> (Vec<f64>, Vec<Vec<Pnt>>, Vec<Vec<f64>>) {
    let v_first = v_flat[v_degree];
    let v_last = v_flat[poles[0].len()];

    // Reverse and complement the V flat knot vector.
    let new_v_flat: Vec<f64> = v_flat
        .iter()
        .rev()
        .map(|&k| v_first + v_last - k)
        .collect();

    // Reverse columns (V-index) of each row.
    let new_poles: Vec<Vec<Pnt>> = poles
        .iter()
        .map(|row| row.iter().rev().cloned().collect())
        .collect();
    let new_weights: Vec<Vec<f64>> = weights
        .iter()
        .map(|row| row.iter().rev().cloned().collect())
        .collect();

    (new_v_flat, new_poles, new_weights)
}

// ---------------------------------------------------------------------------
// Resolution — parametric tolerance from a 3D tolerance
// ---------------------------------------------------------------------------

/// `BSplSLib::Resolution` — compute a parametric tolerance in U and V given a
/// 3D spatial tolerance `tolerance_3d`.
///
/// Follows the OCCT approach: find the maximum partial derivative magnitude
/// along a grid of sample points, then the parametric tolerance is
/// `tolerance_3d / max_deriv`. Returns `(u_tolerance, v_tolerance)`.
pub fn resolution(
    u_flat: &[f64],
    v_flat: &[f64],
    poles: &[Vec<Pnt>],
    weights: &[Vec<f64>],
    u_degree: usize,
    v_degree: usize,
    tolerance_3d: f64,
) -> (f64, f64) {
    let pu = u_degree;
    let pv = v_degree;
    let nu = poles.len();
    let nv = poles[0].len();
    let u_min = u_flat[pu];
    let u_max = u_flat[nu];
    let v_min = v_flat[pv];
    let v_max = v_flat[nv];

    let n_samples = 5usize;
    let mut max_du = 0.0f64;
    let mut max_dv = 0.0f64;

    for i in 0..=n_samples {
        for j in 0..=n_samples {
            let u = u_min + (u_max - u_min) * i as f64 / n_samples as f64;
            let v = v_min + (v_max - v_min) * j as f64 / n_samples as f64;
            let res = d1(u, v, u_flat, v_flat, poles, weights, u_degree, v_degree);
            let du_norm = res.d1u.norm();
            let dv_norm = res.d1v.norm();
            if du_norm > max_du {
                max_du = du_norm;
            }
            if dv_norm > max_dv {
                max_dv = dv_norm;
            }
        }
    }

    let u_tol = if max_du < 1e-15 {
        tolerance_3d
    } else {
        tolerance_3d / max_du
    };
    let v_tol = if max_dv < 1e-15 {
        tolerance_3d
    } else {
        tolerance_3d / max_dv
    };
    (u_tol, v_tol)
}

// ---------------------------------------------------------------------------
// HomogeneousD0 / HomogeneousD1
// ---------------------------------------------------------------------------

/// `BSplSLib::HomogeneousD0` — evaluate the surface in homogeneous (weighted)
/// coordinates at `(u, v)`.
///
/// Returns `(weighted_point, weight)` where `weighted_point = w·P` (the
/// un-divided homogeneous numerator) and `weight = w`. Useful when the caller
/// needs to aggregate contributions without the intermediate division.
pub fn homogeneous_d0(
    u: f64,
    v: f64,
    u_flat: &[f64],
    v_flat: &[f64],
    poles: &[Vec<Pnt>],
    weights: &[Vec<f64>],
    u_degree: usize,
    v_degree: usize,
) -> (Pnt, f64) {
    let nu = poles.len();
    let nv = poles[0].len();
    let pu = u_degree;
    let pv = v_degree;

    let u_cl = u.clamp(u_flat[pu], u_flat[nu]);
    let v_cl = v.clamp(v_flat[pv], v_flat[nv]);

    let u_span = find_span(nu - 1, pu, u_cl, u_flat);
    let v_span = find_span(nv - 1, pv, v_cl, v_flat);

    // Evaluate U basis.
    let db_u = ders_basis(u_span, u_cl, pu, 0, u_flat);
    // Evaluate V basis.
    let db_v = ders_basis(v_span, v_cl, pv, 0, v_flat);

    let mut acc_h = Pnt::origin();
    let mut acc_w = 0.0f64;
    for i in 0..=pu {
        let ui = u_span - pu + i;
        for j in 0..=pv {
            let vj = v_span - pv + j;
            let coeff = db_u[0][i] * db_v[0][j];
            let w = weights[ui][vj];
            acc_h = acc_h + poles[ui][vj] * (coeff * w);
            acc_w += coeff * w;
        }
    }
    (acc_h, acc_w)
}

/// `BSplSLib::HomogeneousD1` — evaluate position and first partial derivatives
/// in homogeneous coordinates.
///
/// Returns a tuple:
/// - `(weighted_point, weight)` for the value (D0 homogeneous),
/// - `(d1u_hom, w_du)` for the U-derivative of the homogeneous representation,
/// - `(d1v_hom, w_dv)` for the V-derivative of the homogeneous representation.
///
/// The caller divides by weight to obtain the Cartesian D0/D1 values.
pub struct HomogeneousD1Result {
    pub hom_point: Pnt,
    pub weight: f64,
    pub d1u_hom: Pnt,
    pub w_du: f64,
    pub d1v_hom: Pnt,
    pub w_dv: f64,
}

/// `BSplSLib::HomogeneousD1` — homogeneous first derivative evaluation.
pub fn homogeneous_d1(
    u: f64,
    v: f64,
    u_flat: &[f64],
    v_flat: &[f64],
    poles: &[Vec<Pnt>],
    weights: &[Vec<f64>],
    u_degree: usize,
    v_degree: usize,
) -> HomogeneousD1Result {
    let nu = poles.len();
    let nv = poles[0].len();
    let pu = u_degree;
    let pv = v_degree;

    let u_cl = u.clamp(u_flat[pu], u_flat[nu]);
    let v_cl = v.clamp(v_flat[pv], v_flat[nv]);

    let u_span = find_span(nu - 1, pu, u_cl, u_flat);
    let v_span = find_span(nv - 1, pv, v_cl, v_flat);

    // Basis functions and first derivatives.
    let db_u = ders_basis(u_span, u_cl, pu, 1, u_flat);
    let db_v = ders_basis(v_span, v_cl, pv, 1, v_flat);

    let mut hom_pt = Pnt::origin();
    let mut wt = 0.0f64;
    let mut d1u_hom = Pnt::origin();
    let mut w_du = 0.0f64;
    let mut d1v_hom = Pnt::origin();
    let mut w_dv = 0.0f64;

    for i in 0..=pu {
        let ui = u_span - pu + i;
        for j in 0..=pv {
            let vj = v_span - pv + j;
            let w = weights[ui][vj];
            let p = poles[ui][vj];

            // D0 contribution.
            let c00 = db_u[0][i] * db_v[0][j];
            hom_pt = hom_pt + p * (c00 * w);
            wt += c00 * w;

            // D1U contribution: derivative of N_i(u) * N_j(v).
            let c10 = db_u[1][i] * db_v[0][j];
            d1u_hom = d1u_hom + p * (c10 * w);
            w_du += c10 * w;

            // D1V contribution.
            let c01 = db_u[0][i] * db_v[1][j];
            d1v_hom = d1v_hom + p * (c01 * w);
            w_dv += c01 * w;
        }
    }

    HomogeneousD1Result {
        hom_point: hom_pt,
        weight: wt,
        d1u_hom,
        w_du,
        d1v_hom,
        w_dv,
    }
}

// ---------------------------------------------------------------------------
// Public facade struct (zero-sized, OCCT style)
// ---------------------------------------------------------------------------

/// Zero-sized facade exposing all `BSplSLib` algorithms as associated functions.
// occt: BSplSLib
pub struct BSplSLib;

impl BSplSLib {
    /// See [`d0`].
    pub fn d0(
        u: f64,
        v: f64,
        u_flat: &[f64],
        v_flat: &[f64],
        poles: &[Vec<Pnt>],
        weights: &[Vec<f64>],
        u_degree: usize,
        v_degree: usize,
    ) -> Pnt {
        d0(u, v, u_flat, v_flat, poles, weights, u_degree, v_degree)
    }

    /// See [`d1`].
    pub fn d1(
        u: f64,
        v: f64,
        u_flat: &[f64],
        v_flat: &[f64],
        poles: &[Vec<Pnt>],
        weights: &[Vec<f64>],
        u_degree: usize,
        v_degree: usize,
    ) -> D1Result {
        d1(u, v, u_flat, v_flat, poles, weights, u_degree, v_degree)
    }

    /// See [`d2`].
    pub fn d2(
        u: f64,
        v: f64,
        u_flat: &[f64],
        v_flat: &[f64],
        poles: &[Vec<Pnt>],
        weights: &[Vec<f64>],
        u_degree: usize,
        v_degree: usize,
    ) -> D2Result {
        d2(u, v, u_flat, v_flat, poles, weights, u_degree, v_degree)
    }

    /// See [`d3`].
    pub fn d3(
        u: f64,
        v: f64,
        u_flat: &[f64],
        v_flat: &[f64],
        poles: &[Vec<Pnt>],
        weights: &[Vec<f64>],
        u_degree: usize,
        v_degree: usize,
    ) -> D3Result {
        d3(u, v, u_flat, v_flat, poles, weights, u_degree, v_degree)
    }

    /// See [`build_cache`].
    pub fn build_cache(
        u: f64,
        v: f64,
        u_flat: &[f64],
        v_flat: &[f64],
        poles: &[Vec<Pnt>],
        weights: &[Vec<f64>],
        u_degree: usize,
        v_degree: usize,
    ) -> SurfaceCache {
        build_cache(u, v, u_flat, v_flat, poles, weights, u_degree, v_degree)
    }

    /// See [`cache_d0`].
    pub fn cache_d0(u: f64, v: f64, cache: &SurfaceCache) -> Pnt {
        cache_d0(u, v, cache)
    }

    /// See [`cache_d1`].
    pub fn cache_d1(u: f64, v: f64, cache: &SurfaceCache) -> CacheD1Result {
        cache_d1(u, v, cache)
    }

    /// See [`iso_u`].
    pub fn iso_u(
        param: f64,
        u_flat: &[f64],
        v_flat: &[f64],
        poles: &[Vec<Pnt>],
        weights: &[Vec<f64>],
        u_degree: usize,
        v_degree: usize,
    ) -> IsoCurve {
        iso_u(param, u_flat, v_flat, poles, weights, u_degree, v_degree)
    }

    /// See [`iso_v`].
    pub fn iso_v(
        param: f64,
        u_flat: &[f64],
        v_flat: &[f64],
        poles: &[Vec<Pnt>],
        weights: &[Vec<f64>],
        u_degree: usize,
        v_degree: usize,
    ) -> IsoCurve {
        iso_v(param, u_flat, v_flat, poles, weights, u_degree, v_degree)
    }

    /// See [`reverse_u`].
    pub fn reverse_u(
        u_flat: &[f64],
        poles: &[Vec<Pnt>],
        weights: &[Vec<f64>],
        u_degree: usize,
    ) -> (Vec<f64>, Vec<Vec<Pnt>>, Vec<Vec<f64>>) {
        reverse_u(u_flat, poles, weights, u_degree)
    }

    /// See [`reverse_v`].
    pub fn reverse_v(
        v_flat: &[f64],
        poles: &[Vec<Pnt>],
        weights: &[Vec<f64>],
        v_degree: usize,
    ) -> (Vec<f64>, Vec<Vec<Pnt>>, Vec<Vec<f64>>) {
        reverse_v(v_flat, poles, weights, v_degree)
    }

    /// See [`resolution`].
    pub fn resolution(
        u_flat: &[f64],
        v_flat: &[f64],
        poles: &[Vec<Pnt>],
        weights: &[Vec<f64>],
        u_degree: usize,
        v_degree: usize,
        tolerance_3d: f64,
    ) -> (f64, f64) {
        resolution(
            u_flat, v_flat, poles, weights, u_degree, v_degree, tolerance_3d,
        )
    }

    /// See [`homogeneous_d0`].
    pub fn homogeneous_d0(
        u: f64,
        v: f64,
        u_flat: &[f64],
        v_flat: &[f64],
        poles: &[Vec<Pnt>],
        weights: &[Vec<f64>],
        u_degree: usize,
        v_degree: usize,
    ) -> (Pnt, f64) {
        homogeneous_d0(u, v, u_flat, v_flat, poles, weights, u_degree, v_degree)
    }

    /// See [`homogeneous_d1`].
    pub fn homogeneous_d1(
        u: f64,
        v: f64,
        u_flat: &[f64],
        v_flat: &[f64],
        poles: &[Vec<Pnt>],
        weights: &[Vec<f64>],
        u_degree: usize,
        v_degree: usize,
    ) -> HomogeneousD1Result {
        homogeneous_d1(u, v, u_flat, v_flat, poles, weights, u_degree, v_degree)
    }
}
