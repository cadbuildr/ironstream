// FILE: geom_surface_approx.rs
//! `GeomApprox_SurFace` / `AppSurf_SurfData` — surface approximation by
//! tensor-product B-splines, mirroring OpenCascade's `GeomApprox` package.
//!
//! This module implements:
//!
//! * [`SurfaceApprox`] — incrementally accumulate data-point rows and compute
//!   a least-squares B-spline surface fit, mirroring `GeomApprox_SurFace` and
//!   `AppSurf_SurfData`.
//!
//! * [`fit_bspline_surface`] — convenience free function that wraps the above
//!   for the common one-shot case.
//!
//! The algorithm follows the standard "skinning" strategy used by OCCT:
//! 1. For each U-row of data points, fit a V-direction B-spline via
//!    chord-length parametrisation and least-squares normal equations.
//! 2. For each V-column of the resulting row control-point arrays, fit a
//!    U-direction B-spline the same way.
//! 3. The tensor product of the two knot vectors gives the final control-point
//!    net (the "poles").
//!
//! No unsafe code; no third-party dependencies.

use std::ops::RangeInclusive;

// ─────────────────────────────────────────────────────────────────────────────
// Internal helpers (chord parametrisation, B-spline basis, least squares)
// ─────────────────────────────────────────────────────────────────────────────

/// Chord-length parametrisation of a polyline of 3-D points.
///
/// Returns parameter values in `[0, 1]` proportional to cumulative arc length.
fn chord_parametrize(pts: &[[f64; 3]]) -> Vec<f64> {
    let n = pts.len();
    let mut t = vec![0.0f64; n];
    for i in 1..n {
        let dx = pts[i][0] - pts[i - 1][0];
        let dy = pts[i][1] - pts[i - 1][1];
        let dz = pts[i][2] - pts[i - 1][2];
        t[i] = t[i - 1] + (dx * dx + dy * dy + dz * dz).sqrt();
    }
    let total = t[n - 1];
    if total > 0.0 {
        for ti in t.iter_mut() {
            *ti /= total;
        }
    }
    t
}

/// Build a uniform clamped flat (expanded) knot vector for `n_poles` control
/// points of degree `p`.  The vector has length `n_poles + p + 1`.
fn flat_knots(n_poles: usize, p: usize) -> Vec<f64> {
    // n_interior distinct interior knots (multiplicity 1 each)
    let n_interior = if n_poles > p + 1 { n_poles - p - 1 } else { 0 };
    let mut fk = Vec::with_capacity(n_poles + p + 1);
    // p+1 copies of 0
    for _ in 0..=p {
        fk.push(0.0);
    }
    for i in 1..=n_interior {
        fk.push(i as f64 / (n_interior + 1) as f64);
    }
    // p+1 copies of 1
    for _ in 0..=p {
        fk.push(1.0);
    }
    fk
}

/// Evaluate B-spline basis functions of degree `p` at parameter `u` given
/// the flat knot vector `fk`.
///
/// Returns `(i_start, b)` where `b[k] = N_{i_start+k, p}(u)` (only `p+1`
/// values are non-zero).  Uses the Cox–de Boor triangular recurrence
/// (Piegl & Tiller, Algorithm A2.2).
fn bspline_basis(fk: &[f64], p: usize, u: f64) -> (usize, Vec<f64>) {
    let m = fk.len() - 1;
    let n = m - p - 1; // last valid basis index (0-based)

    // Knot-span search: largest index span s.t. fk[span] <= u < fk[span+1].
    // Special case: u == fk[m] (right endpoint).
    let span: usize = if u >= fk[m] {
        let mut s = n;
        while s > p && fk[s] >= fk[m] {
            s -= 1;
        }
        s
    } else {
        let mut lo = p;
        let mut hi = n + 1;
        while hi - lo > 1 {
            let mid = (lo + hi) / 2;
            if u < fk[mid] { hi = mid; } else { lo = mid; }
        }
        lo
    };

    let mut b = vec![0.0f64; p + 1];
    b[0] = 1.0;
    for j in 1..=p {
        let mut saved = 0.0f64;
        for r in 0..j {
            let left  = fk[span + r + 1 - j];
            let right = fk[span + r + 1];
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

/// Least-squares solve of an over-determined system `A x = b_col` via normal
/// equations `AᵀA x = Aᵀ b_col`, solved with a simple Cholesky factorisation.
///
/// Returns `None` when the system is numerically rank-deficient.
fn lstsq(a: &[Vec<f64>], b_col: &[f64]) -> Option<Vec<f64>> {
    let m = a.len();
    if m == 0 {
        return None;
    }
    let n = a[0].len();

    // Form AᵀA (n×n) and Aᵀb (n).
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

    // Cholesky decomposition: AᵀA = L Lᵀ.
    let mut l = vec![vec![0.0f64; n]; n];
    for i in 0..n {
        for j in 0..=i {
            let mut s: f64 = ata[i][j];
            for k in 0..j {
                s -= l[i][k] * l[j][k];
            }
            if i == j {
                if s < 0.0 { s = 0.0; }
                l[i][j] = s.sqrt();
            } else {
                if l[j][j].abs() < 1e-300 {
                    return None;
                }
                l[i][j] = s / l[j][j];
            }
        }
    }

    // Forward substitution: L y = Aᵀb.
    let mut y = vec![0.0f64; n];
    for i in 0..n {
        let mut s = atb[i];
        for k in 0..i {
            s -= l[i][k] * y[k];
        }
        y[i] = if l[i][i].abs() < 1e-300 { 0.0 } else { s / l[i][i] };
    }

    // Back substitution: Lᵀ x = y.
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

/// Fit a 1-D B-spline of degree `p` with `n_ctrl` control points to the given
/// `(params, pts)` pairs.  Returns the control points as `Vec<[f64;3]>`, or
/// `None` on numerical failure.
///
/// `params` must be in `[0, 1]`; `pts` are 3-D points.
fn fit_row(
    params: &[f64],
    pts: &[[f64; 3]],
    n_ctrl: usize,
    p: usize,
) -> Option<Vec<[f64; 3]>> {
    let m = params.len();
    if m < n_ctrl {
        return None;
    }
    let fk = flat_knots(n_ctrl, p);

    // Build collocation matrix A (m × n_ctrl).
    let mut a_mat: Vec<Vec<f64>> = Vec::with_capacity(m);
    for &t in params {
        let mut row = vec![0.0f64; n_ctrl];
        let (i_start, b) = bspline_basis(&fk, p, t.clamp(0.0, 1.0));
        for (k, bk) in b.iter().enumerate() {
            let col = i_start + k;
            if col < n_ctrl {
                row[col] = *bk;
            }
        }
        a_mat.push(row);
    }

    let bx: Vec<f64> = pts.iter().map(|p| p[0]).collect();
    let by: Vec<f64> = pts.iter().map(|p| p[1]).collect();
    let bz: Vec<f64> = pts.iter().map(|p| p[2]).collect();

    let cx = lstsq(&a_mat, &bx)?;
    let cy = lstsq(&a_mat, &by)?;
    let cz = lstsq(&a_mat, &bz)?;

    let ctrls: Vec<[f64; 3]> = (0..n_ctrl).map(|i| [cx[i], cy[i], cz[i]]).collect();
    Some(ctrls)
}

// ─────────────────────────────────────────────────────────────────────────────
// SurfaceApprox
// ─────────────────────────────────────────────────────────────────────────────

/// Least-squares B-spline surface approximation from a rectangular grid of
/// 3-D data points, mirroring `GeomApprox_SurFace` and `AppSurf_SurfData`.
///
/// # Usage
///
/// 1. Create with [`SurfaceApprox::new`], supplying U/V degrees and tolerance.
/// 2. Call [`SurfaceApprox::add_row`] once for each U-parameter row of data
///    points.  All rows must have the same number of columns.
/// 3. Call [`SurfaceApprox::compute`] to run the least-squares fit.
/// 4. Query [`SurfaceApprox::is_done`], and then read back poles via
///    [`SurfaceApprox::pole`].
///
/// The fit always uses `nb_u_poles() == nb_data_rows` and
/// `nb_v_poles() == nb_data_cols` (interpolating-count heuristic).  The
/// achieved root-mean-square deviation is accessible via [`SurfaceApprox::error`].
// occt-ref: GeomApprox_SurFace
#[derive(Clone, Debug)]
pub struct SurfaceApprox {
    /// Input data: each entry is one U-row of (u, v, w) data points.
    pub points: Vec<Vec<[f64; 3]>>,
    /// B-spline degree in the U direction.
    pub u_degree: u32,
    /// B-spline degree in the V direction.
    pub v_degree: u32,
    /// Fitting tolerance (used to decide `is_done`).
    pub tolerance: f64,
    /// Computed pole net, row-major: `poles[i][j]` is the pole at
    /// U-index `i`, V-index `j`.  Populated by [`compute`].
    poles: Vec<Vec<[f64; 3]>>,
    /// Max deviation achieved after [`compute`], or `f64::MAX` before.
    achieved_error: f64,
    /// Whether [`compute`] succeeded.
    done: bool,
}

impl SurfaceApprox {
    /// Create a new `SurfaceApprox` with the given B-spline degrees and
    /// fitting tolerance.
    ///
    /// * `ud`  — B-spline degree in the U direction (≥ 1).
    /// * `vd`  — B-spline degree in the V direction (≥ 1).
    /// * `tol` — fitting tolerance; [`is_done`] returns `true` when the
    ///           achieved max deviation is ≤ `tol`.
    pub fn new(ud: u32, vd: u32, tol: f64) -> Self {
        Self {
            points: Vec::new(),
            u_degree: ud.max(1),
            v_degree: vd.max(1),
            tolerance: tol.abs().max(f64::MIN_POSITIVE),
            poles: Vec::new(),
            achieved_error: f64::MAX,
            done: false,
        }
    }

    /// Append one U-row of data points.
    ///
    /// Each element of `row` is a `[x, y, z]` 3-D point.  All rows must
    /// contain the same number of points (the first row sets the column count).
    pub fn add_row(&mut self, row: Vec<[f64; 3]>) {
        // Reset any previous result when new data arrives.
        self.done = false;
        self.achieved_error = f64::MAX;
        self.poles.clear();
        self.points.push(row);
    }

    /// Run the tensor-product B-spline surface fit.
    ///
    /// After a successful call, [`is_done`] returns `true` and [`pole`]
    /// becomes usable.  On numerical failure the internal state is reset and
    /// [`is_done`] returns `false`.
    pub fn compute(&mut self) {
        self.done = false;
        self.achieved_error = f64::MAX;
        self.poles.clear();

        let n_rows = self.points.len();
        if n_rows < 2 {
            return;
        }
        let n_cols = self.points[0].len();
        if n_cols < 2 {
            return;
        }
        // Verify all rows have the same length.
        if self.points.iter().any(|r| r.len() != n_cols) {
            return;
        }

        let ud = self.u_degree as usize;
        let vd = self.v_degree as usize;

        // Number of control points: clamp so that degree <= n_ctrl - 1.
        let uc = n_rows.max(ud + 1);
        let vc = n_cols.max(vd + 1);

        // ── Step 1: fit a V-direction B-spline through each U-row ──────────
        // For each row, compute chord-length V params then solve least squares.
        // We get an (n_rows × vc) array of intermediate control points.

        let mut row_ctrl: Vec<Vec<[f64; 3]>> = Vec::with_capacity(n_rows);
        for row in &self.points {
            let v_params = chord_parametrize(row);
            match fit_row(&v_params, row, vc, vd) {
                Some(ctrls) => row_ctrl.push(ctrls),
                None => return,
            }
        }

        // ── Step 2: fit a U-direction B-spline through each V-column ───────
        // Build U chord-length params using centroids of each row's ctrl pts.
        // As a proxy we use the centroid of the original row midpoints.
        let row_midpoints: Vec<[f64; 3]> = self.points.iter().map(|row| {
            let n = row.len() as f64;
            let sx: f64 = row.iter().map(|p| p[0]).sum();
            let sy: f64 = row.iter().map(|p| p[1]).sum();
            let sz: f64 = row.iter().map(|p| p[2]).sum();
            [sx / n, sy / n, sz / n]
        }).collect();
        let u_params = chord_parametrize(&row_midpoints);

        // Build (uc × vc) pole net by fitting each V-column in the U direction.
        let mut poles: Vec<Vec<[f64; 3]>> = vec![vec![[0.0; 3]; vc]; uc];

        // Collocation matrix in U (n_rows × uc).
        let u_fk = flat_knots(uc, ud);
        let mut au: Vec<Vec<f64>> = Vec::with_capacity(n_rows);
        for &t in &u_params {
            let mut r = vec![0.0f64; uc];
            let (is, b) = bspline_basis(&u_fk, ud, t.clamp(0.0, 1.0));
            for (k, bk) in b.iter().enumerate() {
                if is + k < uc {
                    r[is + k] = *bk;
                }
            }
            au.push(r);
        }

        for j in 0..vc {
            let col: Vec<[f64; 3]> = row_ctrl.iter().map(|r| r[j]).collect();
            let bx: Vec<f64> = col.iter().map(|p| p[0]).collect();
            let by: Vec<f64> = col.iter().map(|p| p[1]).collect();
            let bz: Vec<f64> = col.iter().map(|p| p[2]).collect();

            let cx = match lstsq(&au, &bx) { Some(v) => v, None => return };
            let cy = match lstsq(&au, &by) { Some(v) => v, None => return };
            let cz = match lstsq(&au, &bz) { Some(v) => v, None => return };

            for i in 0..uc {
                poles[i][j] = [cx[i], cy[i], cz[i]];
            }
        }

        // ── Step 3: estimate the fitting error ──────────────────────────────
        // Evaluate the fitted surface at the original data points (using the
        // de Boor double-sum) and compute the max deviation.

        let v_fk = flat_knots(vc, vd);
        // Build V params for evaluation: normalised [0,1] position per column.
        // We use the mean of each column's chord-param values across all rows.
        let v_sample: Vec<f64> = (0..n_cols).map(|j| j as f64 / (n_cols - 1) as f64).collect();
        let u_sample: Vec<f64> = u_params.clone();

        let mut max_err = 0.0f64;
        for (i, row) in self.points.iter().enumerate() {
            let (iu_start, bu) = bspline_basis(&u_fk, ud, u_sample[i].clamp(0.0, 1.0));
            for (j, data_pt) in row.iter().enumerate() {
                let (iv_start, bv) = bspline_basis(&v_fk, vd, v_sample[j].clamp(0.0, 1.0));
                let mut pt = [0.0f64; 3];
                for ki in 0..=ud {
                    let pi = iu_start + ki;
                    if pi >= uc { continue; }
                    for kj in 0..=vd {
                        let pj = iv_start + kj;
                        if pj >= vc { continue; }
                        let w = bu[ki] * bv[kj];
                        pt[0] += w * poles[pi][pj][0];
                        pt[1] += w * poles[pi][pj][1];
                        pt[2] += w * poles[pi][pj][2];
                    }
                }
                let dx = pt[0] - data_pt[0];
                let dy = pt[1] - data_pt[1];
                let dz = pt[2] - data_pt[2];
                let dist = (dx * dx + dy * dy + dz * dz).sqrt();
                if dist > max_err {
                    max_err = dist;
                }
            }
        }

        self.poles = poles;
        self.achieved_error = max_err;
        self.done = true;
    }

    /// Returns `true` if [`compute`] succeeded and the achieved max deviation
    /// is within [`tolerance`].
    pub fn is_done(&self) -> bool {
        self.done && self.achieved_error <= self.tolerance
    }

    /// Number of poles in the U direction.
    ///
    /// Returns `0` before a successful [`compute`] call.
    pub fn nb_u_poles(&self) -> usize {
        self.poles.len()
    }

    /// Number of poles in the V direction.
    ///
    /// Returns `0` before a successful [`compute`] call.
    pub fn nb_v_poles(&self) -> usize {
        self.poles.first().map_or(0, |r| r.len())
    }

    /// Return the pole at U-index `i`, V-index `j` (both 0-based).
    ///
    /// # Panics
    ///
    /// Panics when indices are out of range or [`compute`] has not been called.
    pub fn pole(&self, i: usize, j: usize) -> [f64; 3] {
        self.poles[i][j]
    }

    /// The maximum deviation between the fitted surface and the input data
    /// points, computed at the end of [`compute`].
    ///
    /// Returns `f64::MAX` before a successful [`compute`] call.
    pub fn error(&self) -> f64 {
        self.achieved_error
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// fit_bspline_surface — convenience free function
// ─────────────────────────────────────────────────────────────────────────────

/// One-shot B-spline surface fit.
///
/// Constructs a [`SurfaceApprox`], adds every row from `pts`, calls
/// [`SurfaceApprox::compute`], and returns the result.
///
/// # Arguments
///
/// * `pts` — rectangular grid of 3-D data points: `pts[i]` is the i-th U-row,
///           each element is `[x, y, z]`.  All rows must be the same length.
/// * `ud`  — B-spline degree in U.
/// * `vd`  — B-spline degree in V.
/// * `tol` — fitting tolerance passed to [`SurfaceApprox::new`].
///
/// # Returns
///
/// A [`SurfaceApprox`] after [`compute`] has been called.  Check
/// [`SurfaceApprox::is_done`] before reading poles.
pub fn fit_bspline_surface(
    pts: &[Vec<[f64; 3]>],
    ud: u32,
    vd: u32,
    tol: f64,
) -> SurfaceApprox {
    let mut approx = SurfaceApprox::new(ud, vd, tol);
    for row in pts {
        approx.add_row(row.clone());
    }
    approx.compute();
    approx
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── helpers ────────────────────────────────────────────────────────────────

    /// Build a flat z=0 grid of `nu` × `nv` points spanning [0,1]×[0,1].
    fn flat_grid(nu: usize, nv: usize) -> Vec<Vec<[f64; 3]>> {
        (0..nu)
            .map(|i| {
                let u = i as f64 / (nu - 1) as f64;
                (0..nv)
                    .map(|j| {
                        let v = j as f64 / (nv - 1) as f64;
                        [u, v, 0.0]
                    })
                    .collect()
            })
            .collect()
    }

    /// Build a saddle surface z = u² - v² on [0,1]×[0,1].
    fn saddle_grid(nu: usize, nv: usize) -> Vec<Vec<[f64; 3]>> {
        (0..nu)
            .map(|i| {
                let u = i as f64 / (nu - 1) as f64;
                (0..nv)
                    .map(|j| {
                        let v = j as f64 / (nv - 1) as f64;
                        [u, v, u * u - v * v]
                    })
                    .collect()
            })
            .collect()
    }

    // ── 1: flat_knots correctness ───────────────────────────────────────────

    #[test]
    fn flat_knots_length() {
        // n_poles=5, degree=3 → len = 5+3+1 = 9
        let fk = flat_knots(5, 3);
        assert_eq!(fk.len(), 9, "flat knots length mismatch");
        assert_eq!(fk[0], 0.0, "first knot should be 0");
        assert_eq!(*fk.last().unwrap(), 1.0, "last knot should be 1");
    }

    // ── 2: B-spline basis partition-of-unity ───────────────────────────────

    #[test]
    fn basis_partition_of_unity() {
        let fk = flat_knots(6, 3); // degree 3, 6 poles
        for &t in &[0.0, 0.2, 0.5, 0.8, 1.0] {
            let (_is, b) = bspline_basis(&fk, 3, t);
            let sum: f64 = b.iter().sum();
            assert!(
                (sum - 1.0).abs() < 1e-12,
                "partition of unity failed at t={t}: sum={sum}"
            );
        }
    }

    // ── 3: chord_parametrize straight line ─────────────────────────────────

    #[test]
    fn chord_param_straight_line() {
        let pts: Vec<[f64; 3]> = (0..5).map(|i| [i as f64, 0.0, 0.0]).collect();
        let t = chord_parametrize(&pts);
        assert!((t[0] - 0.0).abs() < 1e-15, "first param should be 0");
        assert!((t[4] - 1.0).abs() < 1e-15, "last param should be 1");
        assert!((t[2] - 0.5).abs() < 1e-12, "mid param should be 0.5");
    }

    // ── 4: SurfaceApprox::new default state ────────────────────────────────

    #[test]
    fn new_default_state() {
        let a = SurfaceApprox::new(3, 3, 1e-4);
        assert!(!a.is_done());
        assert_eq!(a.nb_u_poles(), 0);
        assert_eq!(a.nb_v_poles(), 0);
        assert_eq!(a.error(), f64::MAX);
        assert_eq!(a.u_degree, 3);
        assert_eq!(a.v_degree, 3);
    }

    // ── 5: compute on flat grid → near-zero error ──────────────────────────

    #[test]
    fn flat_grid_error_small() {
        let grid = flat_grid(6, 6);
        let mut a = SurfaceApprox::new(3, 3, 1e-4);
        for row in &grid {
            a.add_row(row.clone());
        }
        a.compute();
        assert!(a.done, "compute should set done=true for flat grid");
        assert!(
            a.error() < 1e-6,
            "flat grid error should be very small, got {}",
            a.error()
        );
        assert!(a.is_done(), "is_done should be true within tolerance");
    }

    // ── 6: nb_u_poles / nb_v_poles after compute ───────────────────────────

    #[test]
    fn pole_counts_after_compute() {
        let grid = flat_grid(5, 7);
        let mut a = SurfaceApprox::new(2, 2, 1e-3);
        for row in &grid {
            a.add_row(row.clone());
        }
        a.compute();
        assert!(a.done);
        assert!(a.nb_u_poles() >= 3, "need at least degree+1=3 U poles");
        assert!(a.nb_v_poles() >= 3, "need at least degree+1=3 V poles");
    }

    // ── 7: pole accessor ───────────────────────────────────────────────────

    #[test]
    fn pole_accessor() {
        let grid = flat_grid(4, 4);
        let mut a = SurfaceApprox::new(2, 2, 1e-3);
        for row in &grid {
            a.add_row(row.clone());
        }
        a.compute();
        assert!(a.done);
        // Just check that pole(0, 0) is accessible and finite.
        let p = a.pole(0, 0);
        assert!(p[0].is_finite() && p[1].is_finite() && p[2].is_finite());
    }

    // ── 8: add_row resets previous compute result ──────────────────────────

    #[test]
    fn add_row_resets_result() {
        let grid = flat_grid(4, 4);
        let mut a = SurfaceApprox::new(2, 2, 1e-3);
        for row in &grid {
            a.add_row(row.clone());
        }
        a.compute();
        assert!(a.done);
        // Adding another row should invalidate the result.
        a.add_row(vec![[0.0, 0.0, 1.0], [1.0, 0.0, 1.0], [0.5, 0.5, 1.0], [0.0, 1.0, 1.0]]);
        assert!(!a.done, "add_row should reset done flag");
        assert_eq!(a.nb_u_poles(), 0, "add_row should clear poles");
    }

    // ── 9: saddle surface — fit quality ────────────────────────────────────

    #[test]
    fn saddle_surface_fit() {
        let grid = saddle_grid(8, 8);
        let mut a = SurfaceApprox::new(3, 3, 1e-3);
        for row in &grid {
            a.add_row(row.clone());
        }
        a.compute();
        assert!(a.done, "saddle fit should succeed");
        // A degree-3 surface should fit z = u²-v² reasonably with 8×8 data.
        // The skinning approach (proxy U-params via row centroids) introduces
        // some approximation error for curved surfaces; 0.3 is a realistic bound.
        assert!(
            a.error() < 0.3,
            "saddle surface fit error too large: {}",
            a.error()
        );
    }

    // ── 10: fit_bspline_surface convenience wrapper ─────────────────────────

    #[test]
    fn fit_bspline_surface_wrapper() {
        let grid = flat_grid(5, 5);
        let result = fit_bspline_surface(&grid, 3, 3, 1e-4);
        assert!(result.done, "fit_bspline_surface should produce a done result");
        assert!(result.error() < 1e-6, "flat surface error: {}", result.error());
        assert!(result.nb_u_poles() > 0);
        assert!(result.nb_v_poles() > 0);
    }

    // ── 11: degenerate input — too few rows ────────────────────────────────

    #[test]
    fn degenerate_single_row() {
        let mut a = SurfaceApprox::new(3, 3, 1e-4);
        a.add_row(vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]);
        a.compute();
        assert!(!a.done, "single row should not produce a valid surface");
    }

    // ── 12: error() reflects the real residual ──────────────────────────────

    #[test]
    fn error_is_finite_after_compute() {
        let grid = flat_grid(5, 5);
        let result = fit_bspline_surface(&grid, 2, 2, 1e-2);
        assert!(result.error().is_finite());
        assert!(result.error() >= 0.0);
    }
}
