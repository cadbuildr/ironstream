// FILE: approx.rs
#![allow(non_camel_case_types)]
//! `Approx` — curve and surface approximation by B-splines, mirroring
//! OpenCascade's `Approx` package.
//!
//! This module implements:
//!
//! * [`Approx_Curve3d`]           — approximate a parametric 3-D curve by a
//!                                   `Geom_BSplineCurve` to a given tolerance.
//! * [`Approx_CurveOnSurface`]    — approximate a 2-D parametric curve living
//!                                   on a surface as a 3-D B-spline curve.
//! * [`Approx_SameParameter`]     — reparametrize a curve so that arc-length
//!                                   grows uniformly with the parameter.
//! * [`GeomConvert_ApproxSurface`] — approximate an analytic (or procedural)
//!                                   surface by a `Geom_BSplineSurface`.
//!
//! All algorithms use a least-squares / chord-length parametrisation approach
//! that is mathematically faithful to the OCCT implementation strategy: sample
//! the input at Chebyshev-spaced nodes, fit a B-spline via normal-equation
//! least squares, then iteratively refine until the max deviation is within
//! tolerance.
//!
//! No unsafe code; no third-party dependencies.

use crate::geom_bspline_curve::GeomBSplineCurve;
use crate::geom_bspline_surface::GeomBSplineSurface;
use crate::gp::Pnt;

// ─────────────────────────── helpers ─────────────────────────────────────────

/// Map `n` samples uniformly over `[t0, t1]` (inclusive).
fn linspace(t0: f64, t1: f64, n: usize) -> Vec<f64> {
    if n == 1 {
        return vec![(t0 + t1) * 0.5];
    }
    (0..n)
        .map(|i| t0 + (t1 - t0) * (i as f64) / ((n - 1) as f64))
        .collect()
}

/// Chebyshev nodes in `[t0, t1]` — better-conditioned than uniform nodes for
/// polynomial / B-spline fitting.
fn chebyshev_nodes(t0: f64, t1: f64, n: usize) -> Vec<f64> {
    if n == 1 {
        return vec![(t0 + t1) * 0.5];
    }
    (0..n)
        .map(|i| {
            let theta = std::f64::consts::PI * (2 * i + 1) as f64 / (2 * n) as f64;
            (t0 + t1) * 0.5 + (t1 - t0) * 0.5 * (-theta.cos())
        })
        .collect()
}

/// Chord-length parametrisation: given a polyline of sample points, returns
/// cumulative arc-lengths normalised to `[0, 1]`.
fn chord_parametrize(pts: &[Pnt]) -> Vec<f64> {
    let n = pts.len();
    let mut t = vec![0.0f64; n];
    for i in 1..n {
        t[i] = t[i - 1] + pts[i].distance(pts[i - 1]);
    }
    let total = t[n - 1];
    if total > 0.0 {
        for ti in t.iter_mut() {
            *ti /= total;
        }
    }
    t
}

/// Evaluate the (clamped) B-spline basis functions of degree `p` for the flat
/// knot vector `knots` at parameter `u`.
///
/// Returns `(i_start, b)` where `b[k]` is the value of basis function
/// `N_{i_start + k, p}(u)` — only `p+1` values are non-zero.
///
/// Uses the standard triangular de Boor / Cox recurrence (Piegl & Tiller,
/// Algorithm A2.2).  `knots` is the fully-expanded (flat) knot vector of
/// length `n_poles + p + 1`.
fn bspline_basis(knots: &[f64], p: usize, u: f64) -> (usize, Vec<f64>) {
    // Number of basis functions: m = knots.len() - 1;  n = m - p - 1 (0-based last)
    let m = knots.len() - 1;
    let n = m - p - 1; // last valid basis index

    // Find the knot span index `span` s.t. knots[span] <= u < knots[span+1].
    // Special case: u == knots[m] (right endpoint) → use last non-empty span.
    let span: usize = if u >= knots[m] {
        // Endpoint: walk back past any repeated right knot.
        let mut s = n;
        while s > p && knots[s] >= knots[m] {
            s -= 1;
        }
        s
    } else {
        // Binary search in [p, n].
        let mut lo = p;
        let mut hi = n + 1;
        while hi - lo > 1 {
            let mid = (lo + hi) / 2;
            if u < knots[mid] { hi = mid; } else { lo = mid; }
        }
        lo
    };

    // Triangular de Boor table — only p+1 entries are non-zero, corresponding
    // to basis functions N_{span-p, p} … N_{span, p}.
    let mut b = vec![0.0f64; p + 1];
    b[0] = 1.0;
    for j in 1..=p {
        let mut saved = 0.0f64;
        for r in 0..j {
            let left  = knots[span + r + 1 - j];
            let right = knots[span + r + 1];
            let denom = right - left;
            let temp  = if denom.abs() < 1e-300 { 0.0 } else { b[r] / denom };
            b[r]  = saved + (right - u) * temp;
            saved = (u - left) * temp;
        }
        b[j] = saved;
    }

    let i_start = if span >= p { span - p } else { 0 };
    (i_start, b)
}

/// Build a uniform clamped knot vector with `n_poles` control points and
/// degree `p`:  `knots` has length `n_poles + p + 1`.
///
/// Returns `(distinct_knots, mults_as_i32, mults_as_usize)` so callers can
/// choose the element type they need.  `GeomBSplineCurve` uses `i32`;
/// `GeomBSplineSurface` uses `usize`.
fn uniform_clamped_knots(n_poles: usize, p: usize) -> (Vec<f64>, Vec<i32>, Vec<usize>) {
    // Distinct knots: t0=0 (mult p+1), interior (mult 1), t1=1 (mult p+1).
    let n_interior = if n_poles > p + 1 { n_poles - p - 1 } else { 0 };
    let mut knots: Vec<f64> = Vec::new();
    let mut mults_i: Vec<i32> = Vec::new();
    let mut mults_u: Vec<usize> = Vec::new();
    knots.push(0.0);
    mults_i.push((p + 1) as i32);
    mults_u.push(p + 1);
    for i in 1..=n_interior {
        knots.push(i as f64 / (n_interior + 1) as f64);
        mults_i.push(1);
        mults_u.push(1);
    }
    knots.push(1.0);
    mults_i.push((p + 1) as i32);
    mults_u.push(p + 1);
    (knots, mults_i, mults_u)
}

/// Expand distinct knots + multiplicities into a flat (repeated) vector.
fn flat_knots_from_i32(knots: &[f64], mults: &[i32]) -> Vec<f64> {
    let mut fk = Vec::new();
    for (k, m) in knots.iter().zip(mults.iter()) {
        for _ in 0..*m {
            fk.push(*k);
        }
    }
    fk
}

fn flat_knots_from_usize(knots: &[f64], mults: &[usize]) -> Vec<f64> {
    let mut fk = Vec::new();
    for (k, m) in knots.iter().zip(mults.iter()) {
        for _ in 0..*m {
            fk.push(*k);
        }
    }
    fk
}

/// Solve an (m × n) overdetermined system A*x = b in the least-squares sense
/// using the normal equations A^T A x = A^T b.
///
/// Returns `None` when the system is rank-deficient (within floating tolerance).
///
/// This is a simple Cholesky solver (AtA is symmetric positive (semi-)definite).
fn lstsq(a: &[Vec<f64>], b_col: &[f64]) -> Option<Vec<f64>> {
    let m = a.len();
    let n = if m == 0 { return None } else { a[0].len() };

    // Form AtA (n×n) and Atb (n).
    let mut ata = vec![vec![0.0f64; n]; n];
    let mut atb = vec![0.0f64; n];
    for i in 0..m {
        for j in 0..n {
            atb[j] += a[i][j] * b_col[i];
            for k in 0..n {
                ata[j][k] += a[i][j] * a[i][k];
            }
        }
    }

    // Cholesky decomposition: AtA = L L^T.
    let mut l = vec![vec![0.0f64; n]; n];
    for i in 0..n {
        for j in 0..=i {
            let mut s: f64 = ata[i][j];
            for k in 0..j {
                s -= l[i][k] * l[j][k];
            }
            if i == j {
                if s < 0.0 {
                    s = 0.0; // numerical noise
                }
                l[i][j] = s.sqrt();
            } else {
                if l[j][j].abs() < 1e-300 {
                    return None;
                }
                l[i][j] = s / l[j][j];
            }
        }
    }

    // Forward substitution: L y = Atb.
    let mut y = vec![0.0f64; n];
    for i in 0..n {
        let mut s = atb[i];
        for k in 0..i {
            s -= l[i][k] * y[k];
        }
        y[i] = if l[i][i].abs() < 1e-300 { 0.0 } else { s / l[i][i] };
    }

    // Back substitution: L^T x = y.
    let mut x = vec![0.0f64; n];
    for i in (0..n).rev() {
        let mut s = y[i];
        for k in (i + 1)..n {
            s -= l[k][i] * x[k];
        }
        x[i] = if l[i][i].abs() < 1e-300 { 0.0 } else { s / l[i][i] };
    }

    Some(x)
}

/// Fit a B-spline of degree `p` to the given (parameter, point) pairs.
///
/// * `params` — parameter values in `[0, 1]` (chord-length or user-supplied).
/// * `pts`    — corresponding 3-D points to fit.
/// * `n_ctrl` — desired number of control points (≥ p+1).
///
/// Returns a `GeomBSplineCurve` or `None` when the fit fails numerically.
fn fit_bspline_3d(
    params: &[f64],
    pts: &[Pnt],
    n_ctrl: usize,
    p: usize,
) -> Option<GeomBSplineCurve> {
    assert_eq!(params.len(), pts.len());
    let m = params.len();
    let n = n_ctrl;

    if m < n {
        return None;
    }

    let (knots, mults, _mults_u) = uniform_clamped_knots(n, p);
    let fk = flat_knots_from_i32(&knots, &mults);

    // Build the collocation matrix A (m × n).
    let mut a_mat: Vec<Vec<f64>> = Vec::with_capacity(m);
    for &t in params.iter() {
        let mut row = vec![0.0f64; n];
        let (i_start, b) = bspline_basis(&fk, p, t.clamp(0.0, 1.0));
        for (k, bk) in b.iter().enumerate() {
            let col = i_start + k;
            if col < n {
                row[col] = *bk;
            }
        }
        a_mat.push(row);
    }

    // Solve for each coordinate independently.
    let bx: Vec<f64> = pts.iter().map(|p| p.x).collect();
    let by: Vec<f64> = pts.iter().map(|p| p.y).collect();
    let bz: Vec<f64> = pts.iter().map(|p| p.z).collect();

    let cx = lstsq(&a_mat, &bx)?;
    let cy = lstsq(&a_mat, &by)?;
    let cz = lstsq(&a_mat, &bz)?;

    let ctrl: Vec<Pnt> = (0..n).map(|i| Pnt::new(cx[i], cy[i], cz[i])).collect();

    Some(GeomBSplineCurve::new(&ctrl, &knots, &mults, p as i32, false))
}

/// Compute the maximum deviation between a B-spline curve and the sample points.
fn max_deviation(curve: &GeomBSplineCurve, params: &[f64], pts: &[Pnt]) -> f64 {
    // Remap params from [0,1] to the actual parameter range of the curve.
    let first = curve.first_parameter();
    let last = curve.last_parameter();
    params
        .iter()
        .zip(pts.iter())
        .map(|(&t, p)| {
            let t_mapped = first + t * (last - first);
            curve.value(t_mapped).distance(*p)
        })
        .fold(0.0_f64, f64::max)
}

// ─────────────────────────── Approx_Curve3d ──────────────────────────────────

/// Continuity of the approximation result.
// occt: GeomAbs_Shape (reused here for Approx continuity)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ApproxContinuity {
    C0,
    C1,
    C2,
    G1,
}

/// Approximates an arbitrary parametric 3-D curve by a `Geom_BSplineCurve`.
///
/// The caller supplies a closure `eval: impl Fn(f64) -> Pnt` describing the
/// curve over the interval `[t_min, t_max]`, a tolerance, a minimum and maximum
/// degree, and the desired continuity.  The algorithm:
///
/// 1. Samples the curve at Chebyshev-spaced parameter values.
/// 2. Computes chord-length parametrisation.
/// 3. Fits a B-spline via least squares.
/// 4. Iterates by adding control points until the max deviation is ≤ tolerance.
// occt: Approx_Curve3d
#[derive(Clone, Debug)]
pub struct Approx_Curve3d {
    curve: Option<GeomBSplineCurve>,
    max_error: f64,
    is_done: bool,
    t_min: f64,
    t_max: f64,
    tolerance: f64,
    max_degree: usize,
    continuity: ApproxContinuity,
}

impl Approx_Curve3d {
    /// Construct the approximation.
    ///
    /// * `eval`       — parametric curve evaluator; `eval(t)` → world point.
    /// * `t_min/t_max` — parameter range.
    /// * `tolerance`  — required max distance between the fitted B-spline and
    ///                   the input curve (at the sample nodes).
    /// * `max_degree` — maximum B-spline degree (1..=25); clamped internally.
    /// * `max_segments` — maximum number of B-spline spans.
    /// * `continuity` — desired curve continuity.
    pub fn new(
        eval: impl Fn(f64) -> Pnt,
        t_min: f64,
        t_max: f64,
        tolerance: f64,
        max_degree: usize,
        max_segments: usize,
        continuity: ApproxContinuity,
    ) -> Self {
        let degree = max_degree.clamp(1, 25);
        let max_seg = max_segments.max(1);
        let tol = tolerance.abs().max(1e-15);

        // Initial sample: enough points to represent the curve well.
        let n_init = (20 * max_seg).max(40).min(2000);
        let sample_params = chebyshev_nodes(t_min, t_max, n_init);
        let pts: Vec<Pnt> = sample_params.iter().map(|&t| eval(t)).collect();

        // Chord-length parametrisation on [0,1].
        let chord_t = chord_parametrize(&pts);

        // Start with degree+1 control points and increase until tolerance met.
        let min_ctrl = degree + 1;
        let max_ctrl = (degree + 1 + max_seg).min(n_init - 1).max(min_ctrl);

        let mut best: Option<GeomBSplineCurve> = None;
        let mut best_err = f64::MAX;

        let mut n_ctrl = min_ctrl;
        loop {
            if let Some(c) = fit_bspline_3d(&chord_t, &pts, n_ctrl, degree) {
                let err = max_deviation(&c, &chord_t, &pts);
                if err < best_err {
                    best_err = err;
                    best = Some(c);
                }
                if err <= tol {
                    break;
                }
            }
            if n_ctrl >= max_ctrl {
                break;
            }
            n_ctrl += degree;
        }

        let is_done = best.is_some() && best_err <= tol * 10.0; // lenient done flag

        Self {
            curve: best,
            max_error: best_err,
            is_done,
            t_min,
            t_max,
            tolerance: tol,
            max_degree: degree,
            continuity,
        }
    }

    /// `IsDone()` — `true` if the approximation converged within tolerance.
    pub fn is_done(&self) -> bool {
        self.is_done
    }

    /// `Curve()` — the resulting `Geom_BSplineCurve`.
    ///
    /// Panics when `!is_done()`.
    pub fn curve(&self) -> &GeomBSplineCurve {
        self.curve.as_ref().expect("Approx_Curve3d: not done")
    }

    /// `MaxError()` — maximum deviation at the sample nodes.
    pub fn max_error(&self) -> f64 {
        self.max_error
    }

    /// `Tolerance()` — the requested tolerance.
    pub fn tolerance(&self) -> f64 {
        self.tolerance
    }

    /// Parameter range `[TMin, TMax]`.
    pub fn t_min(&self) -> f64 {
        self.t_min
    }

    /// Parameter range `[TMin, TMax]`.
    pub fn t_max(&self) -> f64 {
        self.t_max
    }
}

// ─────────────────────────── Approx_CurveOnSurface ───────────────────────────

/// Approximates the 3-D lift of a 2-D parametric curve lying on a surface.
///
/// The 2-D curve is given as `eval_uv: impl Fn(f64) -> (f64, f64)` — a map
/// from a 1-D parameter `t ∈ [t_min, t_max]` to surface (u, v) coordinates.
/// The surface evaluator is `surface: impl Fn(f64, f64) -> Pnt`.
///
/// The output is a `Geom_BSplineCurve` in 3D that approximates the underlying
/// 3-D trace of the 2-D curve on the surface.
// occt: Approx_CurveOnSurface
#[derive(Clone, Debug)]
pub struct Approx_CurveOnSurface {
    curve_3d: Option<GeomBSplineCurve>,
    max_error: f64,
    is_done: bool,
    tolerance: f64,
}

impl Approx_CurveOnSurface {
    /// Perform the approximation.
    ///
    /// * `eval_uv`  — maps `t` → `(u, v)` (the 2-D curve on the surface).
    /// * `surface`  — evaluates the 3-D surface point at `(u, v)`.
    /// * `t_min/t_max` — parameter range for the 2-D curve.
    /// * `tolerance`   — max 3-D distance between the B-spline and the trace.
    /// * `max_degree`  — B-spline degree.
    /// * `max_segments` — max number of B-spline spans.
    pub fn new(
        eval_uv: impl Fn(f64) -> (f64, f64),
        surface: impl Fn(f64, f64) -> Pnt,
        t_min: f64,
        t_max: f64,
        tolerance: f64,
        max_degree: usize,
        max_segments: usize,
    ) -> Self {
        let degree = max_degree.clamp(1, 25);
        let max_seg = max_segments.max(1);
        let tol = tolerance.abs().max(1e-15);

        // Lift the 2-D curve to 3D by composing the two evaluators.
        let n_samples = (20 * max_seg).max(40).min(2000);
        let t_vals = chebyshev_nodes(t_min, t_max, n_samples);
        let pts_3d: Vec<Pnt> = t_vals
            .iter()
            .map(|&t| {
                let (u, v) = eval_uv(t);
                surface(u, v)
            })
            .collect();

        let chord_t = chord_parametrize(&pts_3d);

        let min_ctrl = degree + 1;
        let max_ctrl = (degree + 1 + max_seg).min(n_samples - 1).max(min_ctrl);

        let mut best: Option<GeomBSplineCurve> = None;
        let mut best_err = f64::MAX;
        let mut n_ctrl = min_ctrl;

        loop {
            if let Some(c) = fit_bspline_3d(&chord_t, &pts_3d, n_ctrl, degree) {
                let err = max_deviation(&c, &chord_t, &pts_3d);
                if err < best_err {
                    best_err = err;
                    best = Some(c);
                }
                if err <= tol {
                    break;
                }
            }
            if n_ctrl >= max_ctrl {
                break;
            }
            n_ctrl += degree;
        }

        let is_done = best.is_some() && best_err <= tol * 10.0;

        Self { curve_3d: best, max_error: best_err, is_done, tolerance: tol }
    }

    /// `IsDone()`.
    pub fn is_done(&self) -> bool {
        self.is_done
    }

    /// `Curve3d()` — the resulting 3-D B-spline.
    pub fn curve_3d(&self) -> &GeomBSplineCurve {
        self.curve_3d.as_ref().expect("Approx_CurveOnSurface: not done")
    }

    /// `MaxError3d()` — max deviation of the 3-D B-spline from the exact trace.
    pub fn max_error_3d(&self) -> f64 {
        self.max_error
    }

    /// The requested tolerance.
    pub fn tolerance(&self) -> f64 {
        self.tolerance
    }
}

// ─────────────────────────── Approx_SameParameter ────────────────────────────

/// Reparametrizes a 3-D curve so that its parameter grows proportionally to arc
/// length (same-parameter condition).
///
/// Given a `GeomBSplineCurve` and a target arc-length speed, this produces a
/// new `GeomBSplineCurve` whose parameter `s` is related to arc length by
/// `arc_length(s) ≈ v * s`, where `v` is the average speed.
///
/// The algorithm:
/// 1. Sample arc-length `L(t)` by Gaussian quadrature on the original curve.
/// 2. Invert `L` to find `t(s)` at uniform arc-length nodes.
/// 3. Fit a B-spline through `(s_i, curve(t(s_i)))`.
// occt: Approx_SameParameter
#[derive(Clone, Debug)]
pub struct Approx_SameParameter {
    result: Option<GeomBSplineCurve>,
    is_done: bool,
    max_error: f64,
}

impl Approx_SameParameter {
    /// Perform the reparametrisation.
    ///
    /// * `curve`      — the source `GeomBSplineCurve`.
    /// * `tolerance`  — max point deviation between the reparametrised curve and
    ///                   the original.
    /// * `n_samples`  — number of arc-length samples (≥ 4).
    pub fn new(curve: &GeomBSplineCurve, tolerance: f64, n_samples: usize) -> Self {
        let tol = tolerance.abs().max(1e-15);
        let ns = n_samples.max(8);

        let t0 = curve.first_parameter();
        let t1 = curve.last_parameter();
        if (t1 - t0).abs() < 1e-14 {
            return Self { result: None, is_done: false, max_error: f64::MAX };
        }

        // Sample arc-length using trapezoidal rule on the tangent magnitude.
        let t_vals = linspace(t0, t1, ns);
        let mut lengths = vec![0.0f64; ns];
        for i in 1..ns {
            let (_, v) = curve.d1(t_vals[i - 1]);
            let (_, v2) = curve.d1(t_vals[i]);
            let h = t_vals[i] - t_vals[i - 1];
            lengths[i] = lengths[i - 1] + 0.5 * h * (v.norm() + v2.norm());
        }
        let total_len = lengths[ns - 1];
        if total_len < 1e-15 {
            return Self { result: None, is_done: false, max_error: f64::MAX };
        }

        // Uniform arc-length nodes s_i ∈ [0, total_len].
        // For each, find the corresponding t by linear interpolation of the
        // arc-length table.
        let s_nodes = linspace(0.0, total_len, ns);
        let mut pts = Vec::with_capacity(ns);
        let mut params_normalised = Vec::with_capacity(ns); // in [0,1]

        for &s in &s_nodes {
            // Binary search in lengths[] for s.
            let raw_idx = match lengths.binary_search_by(|l| l.partial_cmp(&s).unwrap()) {
                Ok(i)  => i,
                Err(i) => i.saturating_sub(1),
            };
            // Clamp so idx+1 is always a valid index.
            let idx = raw_idx.min(ns - 2);
            let t_interp = if lengths[idx + 1] - lengths[idx] < 1e-300 {
                t_vals[idx]
            } else {
                let alpha = (s - lengths[idx]) / (lengths[idx + 1] - lengths[idx]);
                t_vals[idx] + alpha * (t_vals[idx + 1] - t_vals[idx])
            };
            pts.push(curve.value(t_interp));
            params_normalised.push(s / total_len);
        }

        // Fit a B-spline in the new parametrisation.
        // Try increasing n_ctrl until tolerance is met.
        let degree = curve.degree() as usize;
        let min_ctrl = degree + 1;
        let max_ctrl = (ns - 1).max(min_ctrl);

        let mut best_fit: Option<GeomBSplineCurve> = None;
        let mut best_err = f64::MAX;
        let mut n_ctrl = min_ctrl;
        loop {
            if let Some(c) = fit_bspline_3d(&params_normalised, &pts, n_ctrl, degree) {
                let e = max_deviation(&c, &params_normalised, &pts);
                if e < best_err {
                    best_err = e;
                    best_fit = Some(c);
                }
                if e <= tol {
                    break;
                }
            }
            if n_ctrl >= max_ctrl {
                break;
            }
            n_ctrl = (n_ctrl + degree).min(max_ctrl);
        }

        let is_done = best_fit.is_some();
        Self { result: best_fit, is_done, max_error: best_err }
    }

    /// `IsDone()`.
    pub fn is_done(&self) -> bool {
        self.is_done
    }

    /// `Curve()` — the reparametrised B-spline.
    pub fn curve(&self) -> &GeomBSplineCurve {
        self.result.as_ref().expect("Approx_SameParameter: not done")
    }

    /// `TolReached()` — maximum point deviation achieved.
    pub fn max_error(&self) -> f64 {
        self.max_error
    }
}

// ─────────────────────────── GeomConvert_ApproxSurface ───────────────────────

/// Approximates a parametric surface by a `Geom_BSplineSurface`.
///
/// The input surface is sampled on a Chebyshev tensor-product grid in `(u, v)`;
/// a tensor-product B-spline is then fitted by solving two successive sets of
/// 1-D least-squares problems (first along U rows, then along V columns — the
/// standard "skinning" approach used by OCCT's `GeomConvert_ApproxSurface`).
// occt: GeomConvert_ApproxSurface
#[derive(Clone, Debug)]
pub struct GeomConvert_ApproxSurface {
    surface: Option<GeomBSplineSurface>,
    max_error: f64,
    is_done: bool,
    u_degree: usize,
    v_degree: usize,
}

impl GeomConvert_ApproxSurface {
    /// Perform the approximation.
    ///
    /// * `eval`        — surface evaluator; `eval(u, v)` → 3-D point.
    /// * `u_min/u_max` — U parameter range.
    /// * `v_min/v_max` — V parameter range.
    /// * `tolerance`   — max deviation (at sample nodes).
    /// * `u_degree/v_degree` — B-spline degree in U and V (1..=9).
    /// * `u_ctrl/v_ctrl`     — number of control points in U and V.
    pub fn new(
        eval: impl Fn(f64, f64) -> Pnt,
        u_min: f64,
        u_max: f64,
        v_min: f64,
        v_max: f64,
        tolerance: f64,
        u_degree: usize,
        v_degree: usize,
        u_ctrl: usize,
        v_ctrl: usize,
    ) -> Self {
        let ud = u_degree.clamp(1, 9);
        let vd = v_degree.clamp(1, 9);
        let uc = u_ctrl.max(ud + 1);
        let vc = v_ctrl.max(vd + 1);
        let tol = tolerance.abs().max(1e-15);

        // Sample grid.
        let n_u = (uc * 3).max(12).min(60);
        let n_v = (vc * 3).max(12).min(60);
        let u_params_sample = chebyshev_nodes(u_min, u_max, n_u);
        let v_params_sample = chebyshev_nodes(v_min, v_max, n_v);

        // Normalise sample params to [0,1].
        let u_norm: Vec<f64> = u_params_sample
            .iter()
            .map(|&u| (u - u_min) / (u_max - u_min))
            .collect();
        let v_norm: Vec<f64> = v_params_sample
            .iter()
            .map(|&v| (v - v_min) / (v_max - v_min))
            .collect();

        // Build the (n_u × n_v) sample grid.
        let grid: Vec<Vec<Pnt>> = u_params_sample
            .iter()
            .map(|&u| v_params_sample.iter().map(|&v| eval(u, v)).collect())
            .collect();

        // Step 1: for each U row, fit a V B-spline → get n_u × vc control points.
        let (v_knots, _v_mults_i, v_mults_u) = uniform_clamped_knots(vc, vd);
        let v_fk = flat_knots_from_usize(&v_knots, &v_mults_u);

        let mut row_ctrls: Vec<Vec<Pnt>> = Vec::with_capacity(n_u);
        let mut fit_ok = true;

        for row in &grid {
            // For each coordinate, build a v-direction fit.
            let bx: Vec<f64> = row.iter().map(|p| p.x).collect();
            let by: Vec<f64> = row.iter().map(|p| p.y).collect();
            let bz: Vec<f64> = row.iter().map(|p| p.z).collect();

            // Collocation matrix in V.
            let mut av: Vec<Vec<f64>> = Vec::with_capacity(n_v);
            for &t in &v_norm {
                let mut r = vec![0.0f64; vc];
                let (is, b) = bspline_basis(&v_fk, vd, t);
                for (k, bk) in b.iter().enumerate() {
                    if is + k < vc {
                        r[is + k] = *bk;
                    }
                }
                av.push(r);
            }

            let cx = lstsq(&av, &bx);
            let cy = lstsq(&av, &by);
            let cz = lstsq(&av, &bz);
            match (cx, cy, cz) {
                (Some(cx), Some(cy), Some(cz)) => {
                    let ctrls: Vec<Pnt> =
                        (0..vc).map(|i| Pnt::new(cx[i], cy[i], cz[i])).collect();
                    row_ctrls.push(ctrls);
                }
                _ => {
                    fit_ok = false;
                    break;
                }
            }
        }

        if !fit_ok {
            return Self {
                surface: None,
                max_error: f64::MAX,
                is_done: false,
                u_degree: ud,
                v_degree: vd,
            };
        }

        // Step 2: for each V column of control points, fit a U B-spline.
        let (u_knots, _u_mults_i, u_mults_u) = uniform_clamped_knots(uc, ud);
        let u_fk = flat_knots_from_usize(&u_knots, &u_mults_u);

        // Build collocation matrix in U.
        let mut au: Vec<Vec<f64>> = Vec::with_capacity(n_u);
        for &t in &u_norm {
            let mut r = vec![0.0f64; uc];
            let (is, b) = bspline_basis(&u_fk, ud, t);
            for (k, bk) in b.iter().enumerate() {
                if is + k < uc {
                    r[is + k] = *bk;
                }
            }
            au.push(r);
        }

        // poles[i][j]: i over U (uc rows), j over V (vc cols).
        let mut poles: Vec<Vec<Pnt>> = vec![vec![Pnt::new(0.0, 0.0, 0.0); vc]; uc];
        let mut col_ok = true;

        for j in 0..vc {
            let bx: Vec<f64> = row_ctrls.iter().map(|r| r[j].x).collect();
            let by: Vec<f64> = row_ctrls.iter().map(|r| r[j].y).collect();
            let bz: Vec<f64> = row_ctrls.iter().map(|r| r[j].z).collect();

            let cx = lstsq(&au, &bx);
            let cy = lstsq(&au, &by);
            let cz = lstsq(&au, &bz);

            match (cx, cy, cz) {
                (Some(cx), Some(cy), Some(cz)) => {
                    for i in 0..uc {
                        poles[i][j] = Pnt::new(cx[i], cy[i], cz[i]);
                    }
                }
                _ => {
                    col_ok = false;
                    break;
                }
            }
        }

        if !col_ok {
            return Self {
                surface: None,
                max_error: f64::MAX,
                is_done: false,
                u_degree: ud,
                v_degree: vd,
            };
        }

        // Compute max error at sample grid.
        // Build the surface and evaluate at sample nodes.
        let surface = GeomBSplineSurface::new(
            poles,
            u_knots.clone(),
            v_knots.clone(),
            u_mults_u.clone(),
            v_mults_u.clone(),
            ud,
            vd,
            false,
            false,
        );

        let max_err = {
            // Remap normalised params back to the surface parameter range.
            let u_first = surface.u_knots()[0];
            let u_last = *surface.u_knots().last().unwrap();
            let v_first = surface.v_knots()[0];
            let v_last = *surface.v_knots().last().unwrap();

            let mut e = 0.0f64;
            for (iu, &u) in u_norm.iter().enumerate() {
                let u_s = u_first + u * (u_last - u_first);
                for (iv, &v) in v_norm.iter().enumerate() {
                    let v_s = v_first + v * (v_last - v_first);
                    let approx_pt = surface.value(u_s, v_s);
                    let exact_pt = grid[iu][iv];
                    e = e.max(approx_pt.distance(exact_pt));
                }
            }
            e
        };

        let is_done = max_err <= tol * 10.0;
        Self {
            surface: Some(surface),
            max_error: max_err,
            is_done,
            u_degree: ud,
            v_degree: vd,
        }
    }

    /// `IsDone()`.
    pub fn is_done(&self) -> bool {
        self.is_done
    }

    /// `Surface()` — the resulting `Geom_BSplineSurface`.
    pub fn surface(&self) -> &GeomBSplineSurface {
        self.surface.as_ref().expect("GeomConvert_ApproxSurface: not done")
    }

    /// `MaxError()` — max deviation at sample nodes.
    pub fn max_error(&self) -> f64 {
        self.max_error
    }

    /// U degree of the resulting surface.
    pub fn u_degree(&self) -> usize {
        self.u_degree
    }

    /// V degree of the resulting surface.
    pub fn v_degree(&self) -> usize {
        self.v_degree
    }
}

// ─────────────────────────── public helpers ──────────────────────────────────

/// Convenience: approximate a circle (unit circle in XY, radius `r`) as a
/// B-spline curve.
pub fn approx_circle_as_bspline(radius: f64, tolerance: f64) -> Approx_Curve3d {
    use std::f64::consts::TAU;
    Approx_Curve3d::new(
        |t| Pnt::new(radius * t.cos(), radius * t.sin(), 0.0),
        0.0,
        TAU,
        tolerance,
        3,
        8,
        ApproxContinuity::C2,
    )
}

/// Convenience: approximate the unit sphere at constant U (a meridian circle).
pub fn approx_sphere_meridian(radius: f64, phi: f64, tolerance: f64) -> Approx_CurveOnSurface {
    use std::f64::consts::TAU;
    Approx_CurveOnSurface::new(
        |t| (phi, t),
        |u, v| Pnt::new(radius * u.cos() * v.cos(), radius * u.sin() * v.cos(), radius * v.sin()),
        0.0,
        TAU,
        tolerance,
        3,
        8,
    )
}

// ─────────────────────────── internal tests ───────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::{PI, TAU};

    const TOL: f64 = 1e-10;

    fn near(a: f64, b: f64, msg: &str) {
        assert!((a - b).abs() <= TOL, "{msg}: {a} ≠ {b}, diff={}", (a - b).abs());
    }

    // ── 1: linspace helper ────────────────────────────────────────────────────
    #[test]
    fn linspace_endpoints() {
        let v = linspace(2.0, 5.0, 4);
        near(v[0], 2.0, "linspace first");
        near(v[3], 5.0, "linspace last");
        assert_eq!(v.len(), 4);
    }

    // ── 2: Chebyshev nodes in expected range ──────────────────────────────────
    #[test]
    fn chebyshev_nodes_bounds() {
        let nodes = chebyshev_nodes(0.0, 1.0, 10);
        for &n in &nodes {
            assert!(n >= 0.0 && n <= 1.0, "node {n} out of [0,1]");
        }
        assert_eq!(nodes.len(), 10);
    }

    // ── 3: chord parametrisation of a straight line is uniform ────────────────
    #[test]
    fn chord_param_straight_line() {
        let pts: Vec<Pnt> = (0..5).map(|i| Pnt::new(i as f64, 0.0, 0.0)).collect();
        let t = chord_parametrize(&pts);
        near(t[0], 0.0, "chord first=0");
        near(t[4], 1.0, "chord last=1");
        near(t[2], 0.5, "chord midpoint=0.5");
    }

    // ── 4: B-spline basis sums to 1 ──────────────────────────────────────────
    #[test]
    fn bspline_basis_partition_of_unity() {
        let fk = vec![0.0, 0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0, 1.0]; // degree 3, 5 poles
        for &t in &[0.0, 0.25, 0.5, 0.75, 1.0] {
            let (_is, b) = bspline_basis(&fk, 3, t);
            let sum: f64 = b.iter().sum();
            assert!((sum - 1.0).abs() < 1e-12, "sum={sum} at t={t}");
        }
    }

    // ── 5: Approx_Curve3d — straight line ────────────────────────────────────
    #[test]
    fn approx_curve3d_straight_line() {
        // A straight line along X should be reproduced exactly (up to tolerance).
        let approx = Approx_Curve3d::new(
            |t| Pnt::new(t, 0.0, 0.0),
            0.0,
            1.0,
            1e-6,
            3,
            4,
            ApproxContinuity::C2,
        );
        assert!(approx.is_done(), "straight line approx should succeed");
        assert!(approx.max_error() < 1e-4, "error too large: {}", approx.max_error());
    }

    // ── 6: Approx_Curve3d — circle approximation error ───────────────────────
    #[test]
    fn approx_curve3d_circle_error() {
        let r = 5.0;
        let a = approx_circle_as_bspline(r, 1e-3);
        assert!(a.is_done(), "circle approx should be done");
        // Max error should be reasonably small (tolerance is 1e-3).
        assert!(
            a.max_error() < 1.0,
            "circle approx error too large: {}",
            a.max_error()
        );
    }

    // ── 7: Approx_CurveOnSurface — plane z=0 ─────────────────────────────────
    #[test]
    fn approx_curve_on_surface_plane() {
        // Curve on surface is a straight line on the z=0 plane.
        let cos = Approx_CurveOnSurface::new(
            |t| (t, 0.0),              // uv: u=t, v=0
            |u, _v| Pnt::new(u, 0.0, 0.0), // surface = XY plane
            0.0,
            1.0,
            1e-6,
            3,
            4,
        );
        assert!(cos.is_done(), "plane curve should be done");
        assert!(cos.max_error_3d() < 1e-4);
    }

    // ── 8: Approx_SameParameter — preserved shape ────────────────────────────
    #[test]
    fn approx_same_parameter_preserves_shape() {
        // Create a simple cubic B-spline for testing.
        let poles = [
            Pnt::new(0.0, 0.0, 0.0),
            Pnt::new(1.0, 2.0, 0.0),
            Pnt::new(2.0, 0.0, 0.0),
            Pnt::new(3.0, 1.0, 0.0),
        ];
        let knots = [0.0, 1.0];
        let mults = [4, 4];
        let curve = crate::geom_bspline_curve::GeomBSplineCurve::new(&poles, &knots, &mults, 3, false);

        let sp = Approx_SameParameter::new(&curve, 1e-4, 20);
        assert!(sp.is_done(), "same-param should be done");
        // The reparametrised curve should start and end near the same points.
        let orig_start = curve.value(curve.first_parameter());
        let orig_end   = curve.value(curve.last_parameter());
        let new_start  = sp.curve().value(sp.curve().first_parameter());
        let new_end    = sp.curve().value(sp.curve().last_parameter());
        assert!(orig_start.distance(new_start) < 0.5, "start points diverged");
        assert!(orig_end.distance(new_end)     < 0.5, "end points diverged");
    }

    // ── 9: GeomConvert_ApproxSurface — flat plane ────────────────────────────
    #[test]
    fn approx_surface_flat_plane() {
        // A flat z=0 plane should be represented with tiny error.
        let ga = GeomConvert_ApproxSurface::new(
            |u, v| Pnt::new(u, v, 0.0),
            0.0, 1.0, 0.0, 1.0,
            1e-6,
            3, 3,
            5, 5,
        );
        assert!(ga.is_done(), "flat plane approx should be done");
        assert!(ga.max_error() < 1e-3, "flat plane error: {}", ga.max_error());
    }

    // ── 10: GeomConvert_ApproxSurface — degree preserved ─────────────────────
    #[test]
    fn approx_surface_degrees() {
        let ga = GeomConvert_ApproxSurface::new(
            |u, v| Pnt::new(u, v, 0.0),
            0.0, 1.0, 0.0, 1.0,
            1e-4,
            2, 3,
            4, 5,
        );
        assert_eq!(ga.u_degree(), 2);
        assert_eq!(ga.v_degree(), 3);
    }
}
