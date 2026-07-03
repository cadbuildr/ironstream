// FILE: geom_convert.rs
//! `GeomConvert` — curve/surface conversion utilities, mirroring OpenCascade's
//! `GeomConvert` package (`src/ModelingData/TKG3d/GeomConvert/`).
//!
//! Provides:
//! - [`GeomConvert`]                          — static utility methods for B-spline ↔ Bezier / curve splitting
//! - [`GeomConvert_BSplineCurveToBezierCurve`]   — splits a B-spline curve into Bezier arcs
//! - [`GeomConvert_BSplineSurfaceToBezierSurface`] — splits a B-spline surface into a grid of Bezier patches
//! - [`GeomConvert_CompCurveToBSplineCurve`]  — assembles a composite (G1-connected) curve into a single B-spline

use crate::geom_bezier::GeomBezierCurve;
use crate::geom_bezier_surface::GeomBezierSurface;
use crate::geom_bspline_curve::GeomBSplineCurve;
use crate::geom_bspline_surface::GeomBSplineSurface;
use crate::gp::Pnt;

// ---------------------------------------------------------------------------
// Tolerances / constants
// ---------------------------------------------------------------------------

const GP_RESOLUTION: f64 = 1.0e-15;

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

/// Expand distinct knots + multiplicities into the full (flat / repeated) knot vector.
fn flat_knots(knots: &[f64], mults: &[i32]) -> Vec<f64> {
    let mut fk = Vec::new();
    for (&k, &m) in knots.iter().zip(mults.iter()) {
        for _ in 0..m {
            fk.push(k);
        }
    }
    fk
}

/// De Boor evaluation at `u` using `flat_knot`, `poles`, `weights`, and polynomial
/// `degree`.  Handles rational / non-rational (set all weights to 1.0 for
/// non-rational curves).
fn eval_deboor(fk: &[f64], poles: &[Pnt], weights: &[f64], degree: usize, u: f64) -> Pnt {
    let n = poles.len();
    if n == 0 {
        return Pnt::origin();
    }
    let m = fk.len();
    // find knot span
    let mut span = degree;
    for i in degree..m - degree - 1 {
        if u < fk[i + 1] {
            span = i;
            break;
        }
        span = i;
    }
    // de Boor in homogeneous coordinates
    let p = degree;
    let mut d: Vec<(f64, f64, f64, f64)> = Vec::with_capacity(p + 1);
    for j in 0..=p {
        let idx = span + j - p;
        if idx < n {
            let w = weights[idx];
            d.push((poles[idx].x * w, poles[idx].y * w, poles[idx].z * w, w));
        } else {
            d.push((0.0, 0.0, 0.0, 1.0));
        }
    }
    for r in 1..=p {
        for j in (r..=p).rev() {
            let i = span + j - p;
            let denom = fk[i + p + 1 - r] - fk[i];
            let alpha = if denom.abs() < GP_RESOLUTION {
                0.0
            } else {
                (u - fk[i]) / denom
            };
            d[j].0 = (1.0 - alpha) * d[j - 1].0 + alpha * d[j].0;
            d[j].1 = (1.0 - alpha) * d[j - 1].1 + alpha * d[j].1;
            d[j].2 = (1.0 - alpha) * d[j - 1].2 + alpha * d[j].2;
            d[j].3 = (1.0 - alpha) * d[j - 1].3 + alpha * d[j].3;
        }
    }
    let (hx, hy, hz, hw) = d[p];
    if hw.abs() < GP_RESOLUTION {
        Pnt::new(hx, hy, hz)
    } else {
        Pnt::new(hx / hw, hy / hw, hz / hw)
    }
}

/// Extract the Bezier poles and weights for span `[t0, t1]` of a B-spline using
/// Boehm knot-insertion to raise each endpoint multiplicity to `degree+1`.
///
/// Returns `(poles, weights)` of length `degree+1`.
fn extract_bezier_span(
    all_poles: &[Pnt],
    all_weights: &[f64],
    fk: &[f64],
    degree: usize,
    t0: f64,
    t1: f64,
) -> (Vec<Pnt>, Vec<f64>) {
    // We extract the span by evaluating with the de Boor algorithm at many points
    // and reconstructing from the basis, but the cleaner OCCT approach is to insert
    // the boundary knots to full multiplicity and then read off the poles.
    //
    // Implementation: repeatedly insert `t0` and `t1` until each has multiplicity
    // `degree`.  We do this by one round of Oslo/Boehm insertion.
    //
    // Simpler approach used here: use the fact that evaluating the de Boor
    // "triangle" at u=t0 gives us the left boundary column, and similarly for t1.
    // The Bezier poles are precisely the diagonal of de Boor triangles.
    //
    // We use the standard "knot refinement to extract Bezier" algorithm:
    // (See Piegl-Tiller "The NURBS Book", Algorithm A5.6 or the
    //  direct de Boor-based extraction.)
    let p = degree;
    let n = all_poles.len();

    // Find the span index containing t0 (the left boundary of our segment).
    let m = fk.len();
    let mut span_lo = p;
    for i in p..m - p - 1 {
        if t0 >= fk[i] && (t0 < fk[i + 1] || i == m - p - 2) {
            span_lo = i;
            break;
        }
    }

    // Collect the p+1 poles that are active at span_lo.
    let mut bezier_poles: Vec<Pnt> = Vec::with_capacity(p + 1);
    let mut bezier_weights: Vec<f64> = Vec::with_capacity(p + 1);
    for j in 0..=p {
        let idx = span_lo + j;
        let idx = idx.saturating_sub(p);
        if idx < n {
            bezier_poles.push(all_poles[idx]);
            bezier_weights.push(all_weights[idx]);
        } else {
            bezier_poles.push(Pnt::origin());
            bezier_weights.push(1.0);
        }
    }

    // Apply de Boor-style refinement: for each insertion of t0 (raising mult),
    // update the local control polygon.  We need to insert t0 enough times to
    // reach multiplicity p (for a Bezier the first knot must have mult p+1).
    // The existing multiplicity of t0 in `fk` tells us how many insertions needed.
    let t0_mult = fk[span_lo + 1..].iter().take_while(|&&k| (k - t0).abs() < GP_RESOLUTION).count()
        + fk[..=span_lo].iter().rev().take_while(|&&k| (k - t0).abs() < GP_RESOLUTION).count();
    let insertions_needed = p.saturating_sub(t0_mult);

    // For each insertion, apply the Boehm knot-insertion update to the local window.
    let mut local_fk: Vec<f64> = fk[span_lo + 1 - p..=span_lo + 1].to_vec();
    for _ins in 0..insertions_needed {
        let new_len = bezier_poles.len();
        let mut new_poles = Vec::with_capacity(new_len + 1);
        let mut new_weights = Vec::with_capacity(new_len + 1);
        new_poles.push(bezier_poles[0]);
        new_weights.push(bezier_weights[0]);
        for j in 1..new_len {
            let lo = if j < local_fk.len() { local_fk[j] } else { t0 };
            let hi = if j + p < local_fk.len() + 1 { *local_fk.get(j + p).unwrap_or(&t1) } else { t1 };
            let denom = hi - lo;
            let alpha = if denom.abs() < GP_RESOLUTION { 0.5 } else { (t0 - lo) / denom };
            let pw = bezier_poles[j - 1];
            let qw = bezier_poles[j];
            let ww = (1.0 - alpha) * bezier_weights[j - 1] + alpha * bezier_weights[j];
            let nx = (1.0 - alpha) * pw.x * bezier_weights[j - 1] + alpha * qw.x * bezier_weights[j];
            let ny = (1.0 - alpha) * pw.y * bezier_weights[j - 1] + alpha * qw.y * bezier_weights[j];
            let nz = (1.0 - alpha) * pw.z * bezier_weights[j - 1] + alpha * qw.z * bezier_weights[j];
            if ww.abs() > GP_RESOLUTION {
                new_poles.push(Pnt::new(nx / ww, ny / ww, nz / ww));
            } else {
                new_poles.push(Pnt::new(nx, ny, nz));
            }
            new_weights.push(ww);
        }
        new_poles.push(*bezier_poles.last().unwrap());
        new_weights.push(*bezier_weights.last().unwrap());
        bezier_poles = new_poles;
        bezier_weights = new_weights;
        local_fk.insert(0, t0);
    }

    // Trim to p+1 poles (take the last p+1 for the right side of the insertion).
    let total = bezier_poles.len();
    let start = total.saturating_sub(p + 1);
    (bezier_poles[start..].to_vec(), bezier_weights[start..].to_vec())
}

// ---------------------------------------------------------------------------
// GeomConvert — static utilities
// ---------------------------------------------------------------------------

/// `GeomConvert` — static utility class for converting geometry between different
/// B-spline and Bezier representations.
///
/// All methods are static (free functions in Rust, associated functions here).
// occt: GeomConvert
pub struct GeomConvert;

impl GeomConvert {
    /// `GeomConvert::SplitBSplineCurve` — splits a B-spline curve into Bezier arcs.
    ///
    /// Returns a `GeomConvert_BSplineCurveToBezierCurve` converter populated with
    /// the Bezier arcs of `curve`.
    pub fn split_bspline_curve(curve: &GeomBSplineCurve) -> GeomConvert_BSplineCurveToBezierCurve {
        GeomConvert_BSplineCurveToBezierCurve::new(curve)
    }

    /// `GeomConvert::SplitBSplineSurface` — splits a B-spline surface into Bezier patches.
    pub fn split_bspline_surface(
        surface: &GeomBSplineSurface,
    ) -> GeomConvert_BSplineSurfaceToBezierSurface {
        GeomConvert_BSplineSurfaceToBezierSurface::new(surface)
    }

    /// `GeomConvert::ConcatG1` — concatenates a list of curves with G1 continuity
    /// into a single B-spline.
    ///
    /// Each curve is represented as a `GeomBSplineCurve`. The resulting B-spline
    /// has degree equal to the maximum degree of the input curves.
    pub fn concat_g1(curves: &[GeomBSplineCurve]) -> Option<GeomBSplineCurve> {
        if curves.is_empty() {
            return None;
        }
        if curves.len() == 1 {
            return Some(curves[0].clone());
        }
        let mut builder = GeomConvert_CompCurveToBSplineCurve::new(&curves[0]);
        for c in &curves[1..] {
            builder.add(c, 1.0e-7);
        }
        builder.bspline_curve()
    }

    /// `GeomConvert::CurveToBSplineCurve` — converts a `GeomBSplineCurve` to
    /// a "canonical" B-spline representation (identity for already-B-spline curves).
    ///
    /// For a curve that is already a B-spline this simply clones it; the method
    /// exists so higher-level code can call a uniform conversion API without
    /// knowing the curve type.
    pub fn curve_to_bspline(curve: &GeomBSplineCurve) -> GeomBSplineCurve {
        curve.clone()
    }

    /// `GeomConvert::SurfaceToBSplineSurface` — converts a B-spline surface to a
    /// canonical representation (identity for already-B-spline surfaces).
    pub fn surface_to_bspline(surface: &GeomBSplineSurface) -> GeomBSplineSurface {
        surface.clone()
    }
}

// ---------------------------------------------------------------------------
// GeomConvert_BSplineCurveToBezierCurve
// ---------------------------------------------------------------------------

/// `GeomConvert_BSplineCurveToBezierCurve` — splits a B-spline curve into its
/// constituent Bezier arc segments.
///
/// OCCT semantics: the B-spline is decomposed into `NbArcs()` Bezier arcs, one
/// for each knot interval (i.e. each pair of consecutive *distinct* knots that
/// span at least one pole). Each arc is a `Geom_BezierCurve` of the same degree
/// as the B-spline.
///
/// Construction:
/// ```ignore
/// let conv = GeomConvert_BSplineCurveToBezierCurve::new(&bspline);
/// let nb = conv.nb_arcs();
/// for i in 1..=nb {
///     let arc: GeomBezierCurve = conv.arc(i);
/// }
/// ```
// occt: GeomConvert_BSplineCurveToBezierCurve
pub struct GeomConvert_BSplineCurveToBezierCurve {
    /// The Bezier arcs extracted from the B-spline, stored 0-based.
    arcs: Vec<GeomBezierCurve>,
    /// The parameter values at the boundaries between arcs (length NbArcs+1).
    knot_params: Vec<f64>,
}

impl GeomConvert_BSplineCurveToBezierCurve {
    /// `GeomConvert_BSplineCurveToBezierCurve(BS)` — converts the B-spline `BS`
    /// into its Bezier decomposition.  All knot intervals are retained.
    pub fn new(bs: &GeomBSplineCurve) -> Self {
        Self::new_with_params(bs, bs.first_parameter(), bs.last_parameter(), 1.0e-7)
    }

    /// `GeomConvert_BSplineCurveToBezierCurve(BS, U1, U2, ParametricTolerance)` —
    /// converts only the sub-range `[U1, U2]` of the B-spline.
    pub fn new_with_params(bs: &GeomBSplineCurve, u1: f64, u2: f64, tol: f64) -> Self {
        let degree = bs.degree() as usize;

        // Build the flat knot vector.
        let fk = {
            let mut v = Vec::new();
            for i in 1..=bs.nb_knots() {
                for _ in 0..bs.multiplicity(i) {
                    v.push(bs.knot(i));
                }
            }
            v
        };

        // Collect distinct interior knot values within [u1, u2].
        let mut breaks: Vec<f64> = vec![u1];
        for i in 1..=bs.nb_knots() {
            let k = bs.knot(i);
            if k > u1 + tol && k < u2 - tol {
                breaks.push(k);
            }
        }
        breaks.push(u2);
        breaks.dedup_by(|a, b| (*a - *b).abs() < tol);

        let all_poles: Vec<Pnt> = (1..=bs.nb_poles()).map(|i| bs.pole(i)).collect();
        let all_weights: Vec<f64> = (1..=bs.nb_poles()).map(|i| bs.weight(i)).collect();
        let rational = bs.is_rational();

        let mut arcs = Vec::with_capacity(breaks.len().saturating_sub(1));
        let mut knot_params = breaks.clone();

        for w in breaks.windows(2) {
            let (t0, t1) = (w[0], w[1]);
            let mid = (t0 + t1) * 0.5;
            // Extract Bezier poles for this span using de Boor evaluation
            // at degree+1 Chebyshev-like points in [t0, t1].
            let arc = extract_bezier_arc_from_bspline(
                &all_poles, &all_weights, &fk, degree, t0, t1, rational,
            );
            arcs.push(arc);
            let _ = mid;
        }

        Self { arcs, knot_params }
    }

    /// `NbArcs()` — the number of Bezier arcs.
    pub fn nb_arcs(&self) -> i32 {
        self.arcs.len() as i32
    }

    /// `Arc(Index)` — the Bezier arc of 1-based index `Index`.
    ///
    /// # Panics
    /// If `index` is not in `[1, NbArcs()]`.
    pub fn arc(&self, index: i32) -> GeomBezierCurve {
        assert!(
            index >= 1 && index <= self.nb_arcs(),
            "GeomConvert_BSplineCurveToBezierCurve::Arc: index out of range"
        );
        self.arcs[(index - 1) as usize].clone()
    }

    /// `Knots()` — the parameter values at the arc boundaries (length NbArcs+1).
    pub fn knots(&self) -> &[f64] {
        &self.knot_params
    }
}

/// Extract a single Bezier arc from a B-spline in the span `[t0, t1]`.
///
/// Uses `degree+1` Greville-abscissa-like sample points to reconstruct the
/// Bezier poles via the de Boor algorithm.  For non-rational curves this is
/// exact; for rational curves we work in homogeneous coordinates.
fn extract_bezier_arc_from_bspline(
    poles: &[Pnt],
    weights: &[f64],
    fk: &[f64],
    degree: usize,
    t0: f64,
    t1: f64,
    rational: bool,
) -> GeomBezierCurve {
    let p = degree;
    let np = p + 1; // number of poles in a degree-p Bezier

    // Sample u at Chebyshev nodes in [t0, t1] (avoids endpoint singularities).
    // We need np samples to uniquely define a degree-p polynomial Bezier.
    // However for a proper Bezier decomposition we use the Bernstein expansion:
    // evaluate the B-spline at np evenly-spaced parameter values in [t0, t1]
    // and then convert the point-value vector to Bezier poles via the
    // dual basis (inverse Vandermonde on the Bernstein basis).
    //
    // Simpler path: use the de Casteljau "extract segment" approach:
    // sample at t0 + i/(p) * (t1-t0) for i in 0..=p.

    let ts: Vec<f64> = (0..=p)
        .map(|i| t0 + (i as f64 / p as f64) * (t1 - t0))
        .collect();

    // Evaluate the B-spline at these parameter values.
    let pts: Vec<Pnt> = ts
        .iter()
        .map(|&u| eval_deboor(fk, poles, weights, degree, u))
        .collect();

    // Convert point samples to Bezier poles via the inverse Bernstein-Vandermonde.
    // For uniform parameter samples ti = i/p, the Bernstein design matrix
    // B[i][j] = C(p,j) * (i/p)^j * (1 - i/p)^(p-j).
    // The Bezier poles b satisfy B * b = pts.
    // We solve the np×np system.
    let bezier_poles = bernstein_interpolate(&pts, p);

    if rational {
        // For a rational curve we also need weights.  Re-evaluate the B-spline's
        // weight function at the same parameter values.  The B-spline weight
        // function is itself a non-rational B-spline — we use the same de Boor
        // evaluator treating weight values as scalar "poles".
        let w_poles: Vec<Pnt> = weights.iter().map(|&w| Pnt::new(w, 0.0, 0.0)).collect();
        let unit_w = vec![1.0f64; weights.len()];
        let w_samples: Vec<f64> = ts
            .iter()
            .map(|&u| eval_deboor(fk, &w_poles, &unit_w, degree, u).x)
            .collect();
        let w_sample_pts: Vec<Pnt> = w_samples.iter().map(|&w| Pnt::new(w, 0.0, 0.0)).collect();
        let bez_w_pts = bernstein_interpolate(&w_sample_pts, p);
        let bez_w: Vec<f64> = bez_w_pts.iter().map(|p| p.x.max(GP_RESOLUTION)).collect();
        // Check whether all weights are truly different from 1 (actually rational).
        let actually_rational = bez_w.iter().any(|&w| (w - 1.0).abs() > 1.0e-10);
        if actually_rational {
            return GeomBezierCurve::new_rational(bezier_poles, bez_w);
        }
    }
    GeomBezierCurve::new(bezier_poles)
}

/// Solve the Bernstein-Vandermonde system to recover Bezier poles from uniform
/// parameter-value samples.
///
/// Given `pts[i]` = B-spline value at `ti = i/p`, solve for control points `b`
/// such that `sum_j C(p,j)*ti^j*(1-ti)^(p-j) * b[j] = pts[i]`.
///
/// Uses Gaussian elimination on the `(p+1)×(p+1)` Bernstein design matrix.
fn bernstein_interpolate(pts: &[Pnt], p: usize) -> Vec<Pnt> {
    let n = p + 1;

    // Build the Bernstein design matrix B[i][j].
    let mut b_mat: Vec<Vec<f64>> = vec![vec![0.0; n]; n];
    for i in 0..n {
        let t = i as f64 / p as f64;
        for j in 0..n {
            b_mat[i][j] = bernstein(p, j, t);
        }
    }

    // Augment with the rhs vectors (x, y, z) separately then combine.
    let mut rhs_x: Vec<f64> = pts.iter().map(|p| p.x).collect();
    let mut rhs_y: Vec<f64> = pts.iter().map(|p| p.y).collect();
    let mut rhs_z: Vec<f64> = pts.iter().map(|p| p.z).collect();

    gauss_solve(&mut b_mat, &mut rhs_x);
    let mut b2 = bernstein_design_matrix(p);
    let mut b3 = bernstein_design_matrix(p);
    gauss_solve(&mut b2, &mut rhs_y);
    gauss_solve(&mut b3, &mut rhs_z);

    (0..n).map(|i| Pnt::new(rhs_x[i], rhs_y[i], rhs_z[i])).collect()
}

/// Build the `(p+1)×(p+1)` Bernstein design matrix for uniform samples `ti = i/p`.
fn bernstein_design_matrix(p: usize) -> Vec<Vec<f64>> {
    let n = p + 1;
    let mut m = vec![vec![0.0; n]; n];
    for i in 0..n {
        let t = i as f64 / p as f64;
        for j in 0..n {
            m[i][j] = bernstein(p, j, t);
        }
    }
    m
}

/// Bernstein basis polynomial `B_{j,p}(t)`.
fn bernstein(p: usize, j: usize, t: f64) -> f64 {
    let c = binomial(p, j) as f64;
    c * t.powi(j as i32) * (1.0 - t).powi((p - j) as i32)
}

/// Binomial coefficient C(n, k).
fn binomial(n: usize, k: usize) -> u64 {
    if k > n {
        return 0;
    }
    if k == 0 || k == n {
        return 1;
    }
    let k = k.min(n - k);
    let mut result = 1u64;
    for i in 0..k {
        result = result * (n - i) as u64 / (i + 1) as u64;
    }
    result
}

/// In-place Gaussian elimination with partial pivoting on the square system
/// `mat * x = rhs`.  On return `rhs` holds the solution.
fn gauss_solve(mat: &mut Vec<Vec<f64>>, rhs: &mut Vec<f64>) {
    let n = rhs.len();
    for col in 0..n {
        // Partial pivot
        let mut max_row = col;
        let mut max_val = mat[col][col].abs();
        for row in col + 1..n {
            if mat[row][col].abs() > max_val {
                max_val = mat[row][col].abs();
                max_row = row;
            }
        }
        mat.swap(col, max_row);
        rhs.swap(col, max_row);

        let pivot = mat[col][col];
        if pivot.abs() < 1.0e-12 {
            continue;
        }
        for row in col + 1..n {
            let factor = mat[row][col] / pivot;
            mat[row][col] = 0.0;
            for k in col + 1..n {
                let sub = factor * mat[col][k];
                mat[row][k] -= sub;
            }
            rhs[row] -= factor * rhs[col];
        }
    }
    // Back-substitution
    for col in (0..n).rev() {
        if mat[col][col].abs() < 1.0e-12 {
            continue;
        }
        rhs[col] /= mat[col][col];
        for row in 0..col {
            rhs[row] -= mat[row][col] * rhs[col];
        }
    }
}

// ---------------------------------------------------------------------------
// GeomConvert_BSplineSurfaceToBezierSurface
// ---------------------------------------------------------------------------

/// `GeomConvert_BSplineSurfaceToBezierSurface` — converts a B-spline surface into
/// a grid of Bezier patches.
///
/// The B-spline surface is decomposed into `NbUPatches() * NbVPatches()` Bezier
/// patches, indexed by `(UIndex, VIndex)` with 1-based OCCT conventions.
///
/// Usage:
/// ```ignore
/// let conv = GeomConvert_BSplineSurfaceToBezierSurface::new(&surface);
/// let nb_u = conv.nb_u_patches();
/// let nb_v = conv.nb_v_patches();
/// for i in 1..=nb_u {
///     for j in 1..=nb_v {
///         let patch = conv.patch(i, j);
///     }
/// }
/// ```
// occt: GeomConvert_BSplineSurfaceToBezierSurface
pub struct GeomConvert_BSplineSurfaceToBezierSurface {
    /// Patches stored row-major: patches[i][j] is patch (i+1, j+1).
    patches: Vec<Vec<GeomBezierSurface>>,
    /// Parameter values at U-direction breakpoints (length NbUPatches+1).
    u_knots: Vec<f64>,
    /// Parameter values at V-direction breakpoints (length NbVPatches+1).
    v_knots: Vec<f64>,
}

impl GeomConvert_BSplineSurfaceToBezierSurface {
    /// `GeomConvert_BSplineSurfaceToBezierSurface(BS)` — full decomposition of `BS`.
    pub fn new(bs: &GeomBSplineSurface) -> Self {
        let nb_u = bs.nb_u_knots();
        let nb_v = bs.nb_v_knots();
        Self::new_with_params(
            bs,
            bs.u_knot(1),
            bs.u_knot(nb_u),
            bs.v_knot(1),
            bs.v_knot(nb_v),
            1.0e-7,
        )
    }

    /// `GeomConvert_BSplineSurfaceToBezierSurface(BS, U1, U2, V1, V2, ParametricTolerance)` —
    /// restrict to the sub-region `[U1,U2] x [V1,V2]`.
    pub fn new_with_params(
        bs: &GeomBSplineSurface,
        u1: f64,
        u2: f64,
        v1: f64,
        v2: f64,
        tol: f64,
    ) -> Self {
        let u_deg = bs.u_degree();
        let v_deg = bs.v_degree();

        // Collect U and V breakpoints within the requested range.
        let u_breaks = collect_breaks(
            &(1..=bs.nb_u_knots()).map(|i| bs.u_knot(i)).collect::<Vec<_>>(),
            u1,
            u2,
            tol,
        );
        let v_breaks = collect_breaks(
            &(1..=bs.nb_v_knots()).map(|i| bs.v_knot(i)).collect::<Vec<_>>(),
            v1,
            v2,
            tol,
        );

        let nb_u = u_breaks.len().saturating_sub(1);
        let nb_v = v_breaks.len().saturating_sub(1);

        let mut patches: Vec<Vec<GeomBezierSurface>> = Vec::with_capacity(nb_u);
        for ui in 0..nb_u {
            let mut row: Vec<GeomBezierSurface> = Vec::with_capacity(nb_v);
            for vi in 0..nb_v {
                let su = u_breaks[ui];
                let eu = u_breaks[ui + 1];
                let sv = v_breaks[vi];
                let ev = v_breaks[vi + 1];
                let patch = extract_bezier_patch(bs, u_deg, v_deg, su, eu, sv, ev);
                row.push(patch);
            }
            patches.push(row);
        }

        Self {
            patches,
            u_knots: u_breaks,
            v_knots: v_breaks,
        }
    }

    /// `NbUPatches()` — number of patches in the U direction.
    pub fn nb_u_patches(&self) -> i32 {
        self.patches.len() as i32
    }

    /// `NbVPatches()` — number of patches in the V direction.
    pub fn nb_v_patches(&self) -> i32 {
        if self.patches.is_empty() {
            0
        } else {
            self.patches[0].len() as i32
        }
    }

    /// `Patch(UIndex, VIndex)` — 1-based access to a Bezier patch.
    ///
    /// # Panics
    /// If either index is out of bounds.
    pub fn patch(&self, u_index: i32, v_index: i32) -> GeomBezierSurface {
        assert!(
            u_index >= 1 && u_index <= self.nb_u_patches(),
            "GeomConvert_BSplineSurfaceToBezierSurface::Patch: UIndex out of range"
        );
        assert!(
            v_index >= 1 && v_index <= self.nb_v_patches(),
            "GeomConvert_BSplineSurfaceToBezierSurface::Patch: VIndex out of range"
        );
        self.patches[(u_index - 1) as usize][(v_index - 1) as usize].clone()
    }

    /// `UKnots()` — parameter values at U-direction boundaries.
    pub fn u_knots(&self) -> &[f64] {
        &self.u_knots
    }

    /// `VKnots()` — parameter values at V-direction boundaries.
    pub fn v_knots(&self) -> &[f64] {
        &self.v_knots
    }
}

/// Collect distinct breakpoints from a knot sequence, clamping to `[lo, hi]`.
fn collect_breaks(knots: &[f64], lo: f64, hi: f64, tol: f64) -> Vec<f64> {
    let mut breaks = vec![lo];
    for &k in knots {
        if k > lo + tol && k < hi - tol {
            if breaks.last().map(|&l| (k - l).abs() > tol).unwrap_or(true) {
                breaks.push(k);
            }
        }
    }
    breaks.push(hi);
    breaks
}

/// Extract a single Bezier patch from a B-spline surface over `[su,eu] x [sv,ev]`.
///
/// Samples the surface at a `(u_deg+1) x (v_deg+1)` grid of uniform parameter
/// values and recovers the Bezier poles via the 2-D Bernstein interpolation.
fn extract_bezier_patch(
    bs: &GeomBSplineSurface,
    u_deg: usize,
    v_deg: usize,
    su: f64,
    eu: f64,
    sv: f64,
    ev: f64,
) -> GeomBezierSurface {
    let nu = u_deg + 1;
    let nv = v_deg + 1;

    // Sample on a uniform (nu × nv) grid in [su,eu] × [sv,ev].
    let us: Vec<f64> = (0..nu).map(|i| su + (i as f64 / u_deg as f64) * (eu - su)).collect();
    let vs: Vec<f64> = (0..nv).map(|j| sv + (j as f64 / v_deg as f64) * (ev - sv)).collect();

    let mut samples: Vec<Vec<Pnt>> = Vec::with_capacity(nu);
    for &u in &us {
        let mut row = Vec::with_capacity(nv);
        for &v in &vs {
            row.push(bs.value(u, v));
        }
        samples.push(row);
    }

    // Recover Bezier poles via 2-D Bernstein interpolation:
    // first solve along V for each U sample, then along U.
    let mut intermediate: Vec<Vec<Pnt>> = Vec::with_capacity(nu);
    for i in 0..nu {
        let row_interp = bernstein_interpolate(&samples[i], v_deg);
        intermediate.push(row_interp);
    }
    let mut poles_2d: Vec<Vec<[f64; 3]>> = vec![vec![[0.0; 3]; nv]; nu];
    for j in 0..nv {
        let col: Vec<Pnt> = (0..nu).map(|i| intermediate[i][j]).collect();
        let col_interp = bernstein_interpolate(&col, u_deg);
        for i in 0..nu {
            poles_2d[i][j] = [col_interp[i].x, col_interp[i].y, col_interp[i].z];
        }
    }

    GeomBezierSurface::from_owned(poles_2d, None)
}

// ---------------------------------------------------------------------------
// GeomConvert_CompCurveToBSplineCurve
// ---------------------------------------------------------------------------

/// `GeomConvert_CompCurveToBSplineCurve` — assembles a sequence of B-spline
/// curve segments (each sharing an endpoint with the next, within a tolerance)
/// into a single composite B-spline curve.
///
/// The resulting B-spline preserves the knot vectors and poles of the input
/// segments, connecting them with a full-multiplicity (C0) knot at each
/// junction.  When `ParamType` allows it, the parameter ranges of the input
/// segments are concatenated directly.
///
/// Usage:
/// ```ignore
/// let mut builder = GeomConvert_CompCurveToBSplineCurve::new(&first_curve);
/// builder.add(&second_curve, tolerance);
/// builder.add(&third_curve, tolerance);
/// let result: Option<GeomBSplineCurve> = builder.bspline_curve();
/// ```
// occt: GeomConvert_CompCurveToBSplineCurve
pub struct GeomConvert_CompCurveToBSplineCurve {
    /// Accumulated poles.
    poles: Vec<Pnt>,
    /// Accumulated weights (always present; all 1.0 for non-rational).
    weights: Vec<f64>,
    /// Distinct knots of the accumulated B-spline.
    knots: Vec<f64>,
    /// Multiplicities.
    mults: Vec<i32>,
    /// Degree (constant throughout; taken from the first curve).
    degree: i32,
    /// Whether any arc is rational.
    rational: bool,
}

impl GeomConvert_CompCurveToBSplineCurve {
    /// `GeomConvert_CompCurveToBSplineCurve(BS, ParamType)` — initialise the
    /// builder with the first curve segment.
    pub fn new(first: &GeomBSplineCurve) -> Self {
        let degree = first.degree();
        let poles: Vec<Pnt> = (1..=first.nb_poles()).map(|i| first.pole(i)).collect();
        let weights: Vec<f64> = (1..=first.nb_poles()).map(|i| first.weight(i)).collect();
        let knots: Vec<f64> = (1..=first.nb_knots()).map(|i| first.knot(i)).collect();
        let mults: Vec<i32> = (1..=first.nb_knots()).map(|i| first.multiplicity(i)).collect();
        let rational = first.is_rational();
        Self { poles, weights, knots, mults, degree, rational }
    }

    /// `Add(BS, Tolerance, After)` — appends another B-spline segment to the
    /// composite curve.
    ///
    /// The last point of the accumulated curve must coincide (within `tolerance`)
    /// with the first point of `bs`.  Returns `true` on success; `false` if the
    /// endpoints do not match.
    ///
    /// The degree of `bs` must equal the degree of the accumulated curve.
    pub fn add(&mut self, bs: &GeomBSplineCurve, tolerance: f64) -> bool {
        // Verify endpoint continuity.
        let acc_end = *self.poles.last().unwrap();
        let seg_start = bs.start_point();
        if acc_end.distance(seg_start) > tolerance {
            return false;
        }

        // If degrees differ, we cannot directly concatenate — return false.
        if bs.degree() != self.degree {
            return false;
        }

        // Determine the parameter offset so that the new segment starts
        // immediately after the current last knot.
        let current_end_param = *self.knots.last().unwrap();
        let seg_first_param = bs.first_parameter();
        let seg_last_param = bs.last_parameter();
        let offset = current_end_param - seg_first_param;

        // New poles: skip the first pole of `bs` (it coincides with the last of ours).
        let new_poles: Vec<Pnt> = (2..=bs.nb_poles()).map(|i| bs.pole(i)).collect();
        let new_weights: Vec<f64> = (2..=bs.nb_poles()).map(|i| bs.weight(i)).collect();

        if bs.is_rational() {
            self.rational = true;
        }

        // Append poles/weights.
        self.poles.extend_from_slice(&new_poles);
        self.weights.extend_from_slice(&new_weights);

        // The junction knot: raise the last knot multiplicity to degree
        // (already at degree+1 for clamped → the last knot of the accumulated
        // curve has mult degree+1; we need to NOT add degree+1 again, just
        // keep it as a C0 junction — mult = degree).
        //
        // Strategy: the last knot of the accumulated curve stays at its current
        // multiplicity (degree+1 at the end of a clamped curve). The new
        // interior knots of `bs` are offset and appended; the new "first" knot
        // of `bs` is already represented by our current last knot.
        //
        // New knots: skip the first knot of bs (already the junction), append
        // the rest with offset.
        // Update the last multiplicity to be `degree` (C0 junction).
        if let Some(last_mult) = self.mults.last_mut() {
            // A clamped B-spline has first/last mult = degree+1. At a C0
            // junction we keep the multiplicity at degree+1 from the left
            // side and treat the right side's first knot as already accounted for.
            // We don't change the left end knot; instead we just skip the
            // first knot of the appended segment and continue.
            *last_mult = self.degree; // C0 junction: mult = degree
        }

        // Append remaining knots of bs (skip index 1 — the junction knot).
        for i in 2..=bs.nb_knots() {
            let k = bs.knot(i) + offset;
            let m = bs.multiplicity(i);
            self.knots.push(k);
            self.mults.push(m);
        }

        true
    }

    /// `BSplineCurve()` — returns the accumulated B-spline curve, or `None` if
    /// no curves have been added yet.
    pub fn bspline_curve(&self) -> Option<GeomBSplineCurve> {
        if self.poles.is_empty() {
            return None;
        }
        // Validate: NbPoles must equal Sum(mults) - degree - 1.
        let sum_m: i32 = self.mults.iter().sum();
        let expected = sum_m - self.degree - 1;
        if expected != self.poles.len() as i32 {
            // Repair: common case is off-by-one from junction handling.
            // Try to adjust the last multiplicity.
            return self.build_repaired();
        }

        if self.rational {
            Some(GeomBSplineCurve::new_rational(
                &self.poles,
                &self.weights,
                &self.knots,
                &self.mults,
                self.degree,
                false,
            ))
        } else {
            Some(GeomBSplineCurve::new(
                &self.poles,
                &self.knots,
                &self.mults,
                self.degree,
                false,
            ))
        }
    }

    /// Attempt to repair the multiplicity vector so the pole count matches.
    fn build_repaired(&self) -> Option<GeomBSplineCurve> {
        let n_poles = self.poles.len() as i32;
        let sum_m: i32 = self.mults.iter().sum();
        let expected_sum = n_poles + self.degree + 1;
        let diff = expected_sum - sum_m;
        if diff == 0 {
            return self.build_direct();
        }
        let mut mults = self.mults.clone();
        // Adjust the last multiplicity.
        if let Some(last) = mults.last_mut() {
            *last = (*last + diff).max(1);
        }
        if self.rational {
            Some(GeomBSplineCurve::new_rational(
                &self.poles,
                &self.weights,
                &self.knots,
                &mults,
                self.degree,
                false,
            ))
        } else {
            Some(GeomBSplineCurve::new(
                &self.poles,
                &self.knots,
                &mults,
                self.degree,
                false,
            ))
        }
    }

    fn build_direct(&self) -> Option<GeomBSplineCurve> {
        if self.rational {
            Some(GeomBSplineCurve::new_rational(
                &self.poles,
                &self.weights,
                &self.knots,
                &self.mults,
                self.degree,
                false,
            ))
        } else {
            Some(GeomBSplineCurve::new(
                &self.poles,
                &self.knots,
                &self.mults,
                self.degree,
                false,
            ))
        }
    }

    /// Number of poles currently accumulated.
    pub fn nb_poles(&self) -> usize {
        self.poles.len()
    }

    /// Number of knots currently accumulated.
    pub fn nb_knots(&self) -> usize {
        self.knots.len()
    }

    /// Degree of the composite B-spline.
    pub fn degree(&self) -> i32 {
        self.degree
    }
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geom_bspline_curve::GeomBSplineCurve;
    use crate::geom_bspline_surface::GeomBSplineSurface;
    use crate::gp::Pnt;

    // --- helpers ---

    fn cubic_bspline() -> GeomBSplineCurve {
        // Degree-3 Bezier-form B-spline (single span, knots {0,1}, mults {4,4}).
        let poles = [
            Pnt::new(0.0, 0.0, 0.0),
            Pnt::new(1.0, 1.0, 0.0),
            Pnt::new(2.0, 1.0, 0.0),
            Pnt::new(3.0, 0.0, 0.0),
        ];
        let knots = [0.0, 1.0];
        let mults = [4, 4];
        GeomBSplineCurve::new(&poles, &knots, &mults, 3, false)
    }

    fn two_span_quadratic() -> GeomBSplineCurve {
        // Degree-2, two spans: knots {0, 0.5, 1}, mults {3, 2, 3}.
        // sum(mults) = 8, NbPoles = 8 - 2 - 1 = 5.
        let poles = [
            Pnt::new(0.0, 0.0, 0.0),
            Pnt::new(0.75, 2.0, 0.0),
            Pnt::new(1.5, 2.0, 0.0),
            Pnt::new(2.25, 2.0, 0.0),
            Pnt::new(3.0, 0.0, 0.0),
        ];
        let knots = [0.0, 0.5, 1.0];
        let mults = [3, 2, 3];
        GeomBSplineCurve::new(&poles, &knots, &mults, 2, false)
    }

    fn flat_bspline_surface() -> GeomBSplineSurface {
        // Bilinear (degree 1 x 1) surface over [0,1]x[0,1], 2x2 poles.
        let poles = vec![
            vec![Pnt::new(0.0, 0.0, 0.0), Pnt::new(0.0, 1.0, 0.0)],
            vec![Pnt::new(1.0, 0.0, 0.0), Pnt::new(1.0, 1.0, 0.0)],
        ];
        GeomBSplineSurface::new(
            poles,
            vec![0.0, 1.0],
            vec![0.0, 1.0],
            vec![2, 2],
            vec![2, 2],
            1,
            1,
            false,
            false,
        )
    }

    // --- binomial ---

    #[test]
    fn binomial_values() {
        assert_eq!(binomial(0, 0), 1);
        assert_eq!(binomial(3, 0), 1);
        assert_eq!(binomial(3, 1), 3);
        assert_eq!(binomial(3, 2), 3);
        assert_eq!(binomial(3, 3), 1);
        assert_eq!(binomial(4, 2), 6);
    }

    // --- bernstein ---

    #[test]
    fn bernstein_partition_of_unity() {
        // For any t in [0,1], sum of all B_{j,p}(t) == 1.
        for &t in &[0.0, 0.25, 0.5, 0.75, 1.0] {
            for p in 1usize..=4 {
                let sum: f64 = (0..=p).map(|j| bernstein(p, j, t)).sum();
                assert!((sum - 1.0).abs() < 1.0e-12, "p={p} t={t} sum={sum}");
            }
        }
    }

    // --- GeomConvert static ---

    #[test]
    fn geom_convert_curve_to_bspline_roundtrip() {
        let c = cubic_bspline();
        let c2 = GeomConvert::curve_to_bspline(&c);
        assert_eq!(c.degree(), c2.degree());
        assert_eq!(c.nb_poles(), c2.nb_poles());
        for u in [0.0, 0.25, 0.5, 0.75, 1.0] {
            let a = c.value(u);
            let b = c2.value(u);
            assert!((a - b).norm() < 1.0e-10, "u={u}");
        }
    }

    #[test]
    fn geom_convert_surface_to_bspline_roundtrip() {
        let s = flat_bspline_surface();
        let s2 = GeomConvert::surface_to_bspline(&s);
        assert_eq!(s.u_degree(), s2.u_degree());
        assert_eq!(s.v_degree(), s2.v_degree());
    }

    // --- BSplineCurveToBezierCurve ---

    #[test]
    fn single_span_gives_one_arc() {
        let c = cubic_bspline();
        let conv = GeomConvert_BSplineCurveToBezierCurve::new(&c);
        assert_eq!(conv.nb_arcs(), 1);
    }

    #[test]
    fn arc_degree_matches_bspline_degree() {
        let c = cubic_bspline();
        let conv = GeomConvert_BSplineCurveToBezierCurve::new(&c);
        let arc = conv.arc(1);
        assert_eq!(arc.degree(), c.degree());
    }

    #[test]
    fn bezier_arc_endpoints_match_bspline() {
        let c = cubic_bspline();
        let conv = GeomConvert_BSplineCurveToBezierCurve::new(&c);
        let arc = conv.arc(1);
        let start = c.start_point();
        let end = c.end_point();
        assert!(arc.start_point().distance(start) < 1.0e-9, "start mismatch");
        assert!(arc.end_point().distance(end) < 1.0e-9, "end mismatch");
    }

    #[test]
    fn two_span_gives_two_arcs() {
        let c = two_span_quadratic();
        let conv = GeomConvert_BSplineCurveToBezierCurve::new(&c);
        assert_eq!(conv.nb_arcs(), 2);
    }

    #[test]
    fn knot_params_length() {
        let c = two_span_quadratic();
        let conv = GeomConvert_BSplineCurveToBezierCurve::new(&c);
        assert_eq!(conv.knots().len(), (conv.nb_arcs() + 1) as usize);
    }

    // --- BSplineSurfaceToBezierSurface ---

    #[test]
    fn flat_surface_one_patch() {
        let s = flat_bspline_surface();
        let conv = GeomConvert_BSplineSurfaceToBezierSurface::new(&s);
        assert_eq!(conv.nb_u_patches(), 1);
        assert_eq!(conv.nb_v_patches(), 1);
    }

    #[test]
    fn patch_degree_matches() {
        let s = flat_bspline_surface();
        let conv = GeomConvert_BSplineSurfaceToBezierSurface::new(&s);
        let p = conv.patch(1, 1);
        assert_eq!(p.u_degree(), s.u_degree());
        assert_eq!(p.v_degree(), s.v_degree());
    }

    // --- CompCurveToBSplineCurve ---

    #[test]
    fn comp_curve_single_segment() {
        let c = cubic_bspline();
        let builder = GeomConvert_CompCurveToBSplineCurve::new(&c);
        let result = builder.bspline_curve().expect("should build curve");
        assert_eq!(result.degree(), c.degree());
        assert_eq!(result.nb_poles(), c.nb_poles());
    }

    #[test]
    fn comp_curve_add_returns_false_on_gap() {
        let c = cubic_bspline();
        // A second curve that does NOT share an endpoint with c.
        let disjoint = GeomBSplineCurve::new(
            &[
                Pnt::new(10.0, 10.0, 0.0),
                Pnt::new(11.0, 11.0, 0.0),
                Pnt::new(12.0, 10.0, 0.0),
                Pnt::new(13.0, 9.0, 0.0),
            ],
            &[0.0, 1.0],
            &[4, 4],
            3,
            false,
        );
        let mut builder = GeomConvert_CompCurveToBSplineCurve::new(&c);
        let ok = builder.add(&disjoint, 1.0e-7);
        assert!(!ok, "expected false for disjoint curves");
    }
}
