//! Lightweight B-spline surface stub modelled after `Geom_BSplineSurface`
//! evaluation utilities from OpenCASCADE Technology (OCCT).
//!
//! This module is intentionally zero-dependency: it uses only `std` and raw
//! `[f64; 3]` arrays so it can be embedded without pulling in any external crate
//! or the heavier `gp` layer.
//!
//! The tensor-product de Boor evaluation follows the algorithm in
//! Piegl & Tiller, *The NURBS Book* (2nd ed.), §4.3.

// ---------------------------------------------------------------------------
// Core type
// ---------------------------------------------------------------------------

/// A tensor-product B-spline surface stored as raw pole coordinates.
///
/// `poles[i][j]` is the control point at U-index `i`, V-index `j`
/// (both 0-based).  The grid dimensions are determined at construction time
/// from `u_degree` / `v_degree` and the knot vectors supplied via
/// [`BspSurface::set_u_knots`] / [`BspSurface::set_v_knots`].
// occt: Geom_BSplineSurface
#[derive(Clone, Debug)]
pub struct BspSurface {
    pub u_degree: u32,
    pub v_degree: u32,
    pub u_knots: Vec<f64>,
    pub v_knots: Vec<f64>,
    /// Row-major control-point grid: `poles[i]` is the i-th U-row.
    pub poles: Vec<Vec<[f64; 3]>>,
}

// ---------------------------------------------------------------------------
// Construction helpers
// ---------------------------------------------------------------------------

impl BspSurface {
    /// Create a new surface with the given degrees.
    ///
    /// The knot vectors default to empty and must be supplied via
    /// [`set_u_knots`][Self::set_u_knots] / [`set_v_knots`][Self::set_v_knots].
    /// The pole grid is not pre-allocated; poles are added via
    /// [`set_pole`][Self::set_pole].
    pub fn new(ud: u32, vd: u32) -> Self {
        assert!(ud >= 1, "BspSurface: u_degree must be >= 1");
        assert!(vd >= 1, "BspSurface: v_degree must be >= 1");
        BspSurface {
            u_degree: ud,
            v_degree: vd,
            u_knots: Vec::new(),
            v_knots: Vec::new(),
            poles: Vec::new(),
        }
    }

    /// Replace the U knot vector.
    ///
    /// Must be non-decreasing and have length `≥ u_degree + 2` for a
    /// non-trivial surface.  No validation is performed beyond a debug assertion.
    pub fn set_u_knots(&mut self, knots: Vec<f64>) {
        debug_assert!(
            knots.len() >= (self.u_degree as usize) + 2,
            "BspSurface::set_u_knots: knot vector is too short"
        );
        self.u_knots = knots;
    }

    /// Replace the V knot vector.
    pub fn set_v_knots(&mut self, knots: Vec<f64>) {
        debug_assert!(
            knots.len() >= (self.v_degree as usize) + 2,
            "BspSurface::set_v_knots: knot vector is too short"
        );
        self.v_knots = knots;
    }

    /// Set the control point at grid position `(i, j)` (0-based).
    ///
    /// The `poles` grid is grown automatically to accommodate `(i, j)`.
    pub fn set_pole(&mut self, i: usize, j: usize, p: [f64; 3]) {
        // Grow the U dimension.
        if i >= self.poles.len() {
            self.poles.resize(i + 1, Vec::new());
        }
        // Grow the V dimension of that row.
        if j >= self.poles[i].len() {
            self.poles[i].resize(j + 1, [0.0; 3]);
        }
        self.poles[i][j] = p;
    }
}

// ---------------------------------------------------------------------------
// Accessors
// ---------------------------------------------------------------------------

impl BspSurface {
    /// Number of poles in the U direction (number of rows).
    pub fn nb_u_poles(&self) -> usize {
        self.poles.len()
    }

    /// Number of poles in the V direction (number of columns in the first row).
    ///
    /// Returns `0` if the grid is empty.
    pub fn nb_v_poles(&self) -> usize {
        self.poles.first().map(|r| r.len()).unwrap_or(0)
    }

    /// Return `true` if the surface is closed in the U direction, i.e. the
    /// first and last U-column of poles coincide within the default tolerance
    /// (`1e-7`).
    pub fn is_u_closed(&self) -> bool {
        if self.poles.len() < 2 {
            return false;
        }
        let n = self.nb_v_poles();
        let first = &self.poles[0];
        let last = &self.poles[self.poles.len() - 1];
        if first.len() != n || last.len() != n {
            return false;
        }
        let tol2 = 1e-7_f64 * 1e-7;
        first.iter().zip(last.iter()).all(|(a, b)| {
            let dx = a[0] - b[0];
            let dy = a[1] - b[1];
            let dz = a[2] - b[2];
            dx * dx + dy * dy + dz * dz <= tol2
        })
    }

    /// Return `true` if the surface is closed in the V direction, i.e. the
    /// first and last V-row of poles coincide within the default tolerance.
    pub fn is_v_closed(&self) -> bool {
        if self.nb_v_poles() < 2 {
            return false;
        }
        let tol2 = 1e-7_f64 * 1e-7;
        let nv = self.nb_v_poles();
        self.poles.iter().all(|row| {
            if row.len() < 2 {
                return false;
            }
            let a = row[0];
            let b = row[nv - 1];
            let dx = a[0] - b[0];
            let dy = a[1] - b[1];
            let dz = a[2] - b[2];
            dx * dx + dy * dy + dz * dz <= tol2
        })
    }
}

// ---------------------------------------------------------------------------
// De Boor evaluation kernel (pure-Rust, no alloc beyond small local Vecs)
// ---------------------------------------------------------------------------

/// Find the knot-span index `k` such that `flat[k] <= u < flat[k+1]`, clamped
/// to the valid range `[p, n]`.  `n` is the index of the last pole (0-based).
fn find_span(n: usize, p: usize, u: f64, flat: &[f64]) -> usize {
    // Clamp to the domain.
    if u >= flat[n + 1] {
        return n;
    }
    if u <= flat[p] {
        return p;
    }
    let (mut lo, mut hi) = (p, n + 1);
    let mut mid = (lo + hi) / 2;
    while u < flat[mid] || u >= flat[mid + 1] {
        if u < flat[mid] {
            hi = mid;
        } else {
            lo = mid;
        }
        mid = (lo + hi) / 2;
    }
    mid
}

/// Compute the `p + 1` non-zero B-spline basis functions at knot-span `span`.
/// Implements Algorithm A2.2 from *The NURBS Book*.
fn basis_funs(span: usize, u: f64, p: usize, flat: &[f64]) -> Vec<f64> {
    let mut n = vec![0.0_f64; p + 1];
    let mut left = vec![0.0_f64; p + 1];
    let mut right = vec![0.0_f64; p + 1];
    n[0] = 1.0;
    for j in 1..=p {
        left[j] = u - flat[span + 1 - j];
        right[j] = flat[span + j] - u;
        let mut saved = 0.0;
        for r in 0..j {
            let denom = right[r + 1] + left[j - r];
            let temp = if denom.abs() < f64::EPSILON { 0.0 } else { n[r] / denom };
            n[r] = saved + right[r + 1] * temp;
            saved = left[j - r] * temp;
        }
        n[j] = saved;
    }
    n
}

/// Compute the basis functions *and* their first derivatives at `span`.
/// Returns `(N, dN)` each of length `p + 1`.
///
/// Implements Algorithm A2.3 from *The NURBS Book* specialised to `nd = 1`.
fn basis_funs_d1(span: usize, u: f64, p: usize, flat: &[f64]) -> (Vec<f64>, Vec<f64>) {
    // Build the triangular table `ndu`.
    // ndu[j][r] for r < j stores the knot differences (denominator terms).
    // ndu[j][p] for j = 0..=p stores the basis-function values N_{span-p+j, p}(u).
    let mut ndu = vec![vec![0.0_f64; p + 1]; p + 1];
    let mut left = vec![0.0_f64; p + 1];
    let mut right = vec![0.0_f64; p + 1];
    ndu[0][0] = 1.0;
    for j in 1..=p {
        left[j] = u - flat[span + 1 - j];
        right[j] = flat[span + j] - u;
        let mut saved = 0.0;
        for r in 0..j {
            // Store the denominator in the lower triangle.
            ndu[j][r] = right[r + 1] + left[j - r];
            let temp = if ndu[j][r].abs() < f64::EPSILON {
                0.0
            } else {
                ndu[r][j - 1] / ndu[j][r]
            };
            ndu[r][j] = saved + right[r + 1] * temp;
            saved = left[j - r] * temp;
        }
        ndu[j][j] = saved;
    }

    // Basis values: ders[0][j] = N_{span-p+j, p}(u).
    let n_vals: Vec<f64> = (0..=p).map(|j| ndu[j][p]).collect();

    // Compute first derivatives using the full A2.3 differentiation pass.
    // `a[s][j]` stores the two most recently computed rows of the alpha table.
    let nd = 1_usize;
    let mut ders = vec![vec![0.0_f64; p + 1]; nd + 1];
    for j in 0..=p {
        ders[0][j] = ndu[j][p];
    }

    let mut a = vec![vec![0.0_f64; p + 1]; 2];
    for r in 0..=p {
        // Reset the two-row buffer for each basis function r.
        a[0].iter_mut().for_each(|x| *x = 0.0);
        a[1].iter_mut().for_each(|x| *x = 0.0);
        let mut s1 = 0_usize;
        let mut s2 = 1_usize;
        a[s1][0] = 1.0;

        for k in 1..=nd {
            let mut d = 0.0_f64;
            let rk = r as i64 - k as i64;
            let pk = p as i64 - k as i64;
            if r >= k {
                a[s2][0] = a[s1][0] / ndu[(pk + 1) as usize][rk as usize];
                d = a[s2][0] * ndu[rk as usize][pk as usize];
            }
            let j1: usize = if rk >= -1 { 1 } else { (-rk) as usize };
            let j2: usize = if (r as i64 - 1) <= pk {
                k - 1
            } else {
                p - r
            };
            if j1 <= j2 {
                for j in j1..=j2 {
                    let idx = (rk + j as i64) as usize;
                    a[s2][j] =
                        (a[s1][j] - a[s1][j - 1]) / ndu[(pk + 1) as usize][idx];
                    d += a[s2][j] * ndu[idx][pk as usize];
                }
            }
            if r <= pk as usize {
                a[s2][k] = -a[s1][k - 1] / ndu[(pk + 1) as usize][r];
                d += a[s2][k] * ndu[r][pk as usize];
            }
            ders[k][r] = d;
            std::mem::swap(&mut s1, &mut s2);
        }
    }

    // Multiply derivative entries by the correct factorial factor: p! / (p-k)! = p for k=1.
    let mut fac = p;
    for k in 1..=nd {
        for x in ders[k].iter_mut() {
            *x *= fac as f64;
        }
        fac *= if p > k { p - k } else { 1 };
    }

    (n_vals, ders[1].clone())
}

// ---------------------------------------------------------------------------
// Tensor-product evaluation
// ---------------------------------------------------------------------------

impl BspSurface {
    /// Clamp a parameter to the valid domain implied by the knot vector and
    /// the surface degree.
    fn clamp_u(&self, u: f64) -> f64 {
        let p = self.u_degree as usize;
        let n = self.nb_u_poles();
        if n == 0 || self.u_knots.len() < p + 2 {
            return u;
        }
        u.clamp(self.u_knots[p], self.u_knots[n])
    }

    fn clamp_v(&self, v: f64) -> f64 {
        let p = self.v_degree as usize;
        let n = self.nb_v_poles();
        if n == 0 || self.v_knots.len() < p + 2 {
            return v;
        }
        v.clamp(self.v_knots[p], self.v_knots[n])
    }

    /// Evaluate the surface at `(u, v)` using tensor-product de Boor.
    ///
    /// Returns the 3-D point `S(u, v)`.
    pub fn evaluate(&self, u: f64, v: f64) -> [f64; 3] {
        let pu = self.u_degree as usize;
        let pv = self.v_degree as usize;
        let nu = self.nb_u_poles();
        let nv = self.nb_v_poles();
        if nu == 0 || nv == 0 {
            return [0.0; 3];
        }

        let uc = self.clamp_u(u);
        let vc = self.clamp_v(v);

        let span_u = find_span(nu - 1, pu, uc, &self.u_knots);
        let span_v = find_span(nv - 1, pv, vc, &self.v_knots);
        let nu_b = basis_funs(span_u, uc, pu, &self.u_knots);
        let nv_b = basis_funs(span_v, vc, pv, &self.v_knots);

        let mut result = [0.0_f64; 3];
        for r in 0..=pu {
            let ui = span_u - pu + r;
            let row = &self.poles[ui];
            for s in 0..=pv {
                let vi = span_v - pv + s;
                let coeff = nu_b[r] * nv_b[s];
                let p = row[vi];
                result[0] += coeff * p[0];
                result[1] += coeff * p[1];
                result[2] += coeff * p[2];
            }
        }
        result
    }

    /// First partial derivative with respect to U at `(u, v)`.
    pub fn d1u(&self, u: f64, v: f64) -> [f64; 3] {
        let pu = self.u_degree as usize;
        let pv = self.v_degree as usize;
        let nu = self.nb_u_poles();
        let nv = self.nb_v_poles();
        if nu == 0 || nv == 0 {
            return [0.0; 3];
        }

        let uc = self.clamp_u(u);
        let vc = self.clamp_v(v);

        let span_u = find_span(nu - 1, pu, uc, &self.u_knots);
        let span_v = find_span(nv - 1, pv, vc, &self.v_knots);
        let (_, dnu_b) = basis_funs_d1(span_u, uc, pu, &self.u_knots);
        let nv_b = basis_funs(span_v, vc, pv, &self.v_knots);

        let mut result = [0.0_f64; 3];
        for r in 0..=pu {
            let ui = span_u - pu + r;
            let row = &self.poles[ui];
            for s in 0..=pv {
                let vi = span_v - pv + s;
                let coeff = dnu_b[r] * nv_b[s];
                let p = row[vi];
                result[0] += coeff * p[0];
                result[1] += coeff * p[1];
                result[2] += coeff * p[2];
            }
        }
        result
    }

    /// First partial derivative with respect to V at `(u, v)`.
    pub fn d1v(&self, u: f64, v: f64) -> [f64; 3] {
        let pu = self.u_degree as usize;
        let pv = self.v_degree as usize;
        let nu = self.nb_u_poles();
        let nv = self.nb_v_poles();
        if nu == 0 || nv == 0 {
            return [0.0; 3];
        }

        let uc = self.clamp_u(u);
        let vc = self.clamp_v(v);

        let span_u = find_span(nu - 1, pu, uc, &self.u_knots);
        let span_v = find_span(nv - 1, pv, vc, &self.v_knots);
        let nu_b = basis_funs(span_u, uc, pu, &self.u_knots);
        let (_, dnv_b) = basis_funs_d1(span_v, vc, pv, &self.v_knots);

        let mut result = [0.0_f64; 3];
        for r in 0..=pu {
            let ui = span_u - pu + r;
            let row = &self.poles[ui];
            for s in 0..=pv {
                let vi = span_v - pv + s;
                let coeff = nu_b[r] * dnv_b[s];
                let p = row[vi];
                result[0] += coeff * p[0];
                result[1] += coeff * p[1];
                result[2] += coeff * p[2];
            }
        }
        result
    }
}

// ---------------------------------------------------------------------------
// Free function: parametric bounds
// ---------------------------------------------------------------------------

/// Return the parametric bounds of `s` as `([u_min, u_max], [v_min, v_max])`.
///
/// The bounds are derived from the knot vectors using the standard clamped
/// B-spline convention: the first valid parameter is `u_knots[u_degree]` and
/// the last is `u_knots[nb_u_poles]` (and symmetrically for V).
///
/// Returns `([0.0, 1.0], [0.0, 1.0])` when the surface is insufficiently
/// initialised (empty knot vectors or pole grid).
pub fn bsp_surface_bounds(s: &BspSurface) -> ([f64; 2], [f64; 2]) {
    let pu = s.u_degree as usize;
    let pv = s.v_degree as usize;
    let nu = s.nb_u_poles();
    let nv = s.nb_v_poles();

    let u_range = if nu > 0 && s.u_knots.len() > nu {
        [s.u_knots[pu], s.u_knots[nu]]
    } else {
        [0.0, 1.0]
    };

    let v_range = if nv > 0 && s.v_knots.len() > nv {
        [s.v_knots[pv], s.v_knots[nv]]
    } else {
        [0.0, 1.0]
    };

    (u_range, v_range)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    /// Build a flat (z = 0) bilinear (degree-1) patch over [0, 1] x [0, 1].
    fn flat_bilinear() -> BspSurface {
        let mut s = BspSurface::new(1, 1);
        // Clamped knot vectors for degree-1: [0, 0, 1, 1].
        s.set_u_knots(vec![0.0, 0.0, 1.0, 1.0]);
        s.set_v_knots(vec![0.0, 0.0, 1.0, 1.0]);
        // 2 x 2 poles forming the unit square on z = 0.
        s.set_pole(0, 0, [0.0, 0.0, 0.0]);
        s.set_pole(0, 1, [0.0, 1.0, 0.0]);
        s.set_pole(1, 0, [1.0, 0.0, 0.0]);
        s.set_pole(1, 1, [1.0, 1.0, 0.0]);
        s
    }

    #[test]
    fn test_bounds_bilinear() {
        let s = flat_bilinear();
        let (u_rng, v_rng) = bsp_surface_bounds(&s);
        assert!((u_rng[0] - 0.0).abs() < 1e-12);
        assert!((u_rng[1] - 1.0).abs() < 1e-12);
        assert!((v_rng[0] - 0.0).abs() < 1e-12);
        assert!((v_rng[1] - 1.0).abs() < 1e-12);
    }

    #[test]
    fn test_evaluate_corners() {
        let s = flat_bilinear();
        let tol = 1e-12;
        let p00 = s.evaluate(0.0, 0.0);
        let p11 = s.evaluate(1.0, 1.0);
        let p01 = s.evaluate(0.0, 1.0);
        let p10 = s.evaluate(1.0, 0.0);
        assert!((p00[0] - 0.0).abs() < tol && (p00[1] - 0.0).abs() < tol);
        assert!((p11[0] - 1.0).abs() < tol && (p11[1] - 1.0).abs() < tol);
        assert!((p01[0] - 0.0).abs() < tol && (p01[1] - 1.0).abs() < tol);
        assert!((p10[0] - 1.0).abs() < tol && (p10[1] - 0.0).abs() < tol);
    }

    #[test]
    fn test_evaluate_midpoint() {
        let s = flat_bilinear();
        let pm = s.evaluate(0.5, 0.5);
        let tol = 1e-12;
        assert!((pm[0] - 0.5).abs() < tol);
        assert!((pm[1] - 0.5).abs() < tol);
        assert!(pm[2].abs() < tol);
    }

    #[test]
    fn test_d1u_constant_for_linear_patch() {
        // For a linear patch over [0,1]^2, dS/du = [1, 0, 0] everywhere.
        let s = flat_bilinear();
        let du = s.d1u(0.5, 0.5);
        let tol = 1e-12;
        assert!((du[0] - 1.0).abs() < tol);
        assert!(du[1].abs() < tol);
        assert!(du[2].abs() < tol);
    }

    #[test]
    fn test_d1v_constant_for_linear_patch() {
        // For a linear patch over [0,1]^2, dS/dv = [0, 1, 0] everywhere.
        let s = flat_bilinear();
        let dv = s.d1v(0.5, 0.5);
        let tol = 1e-12;
        assert!(dv[0].abs() < tol);
        assert!((dv[1] - 1.0).abs() < tol);
        assert!(dv[2].abs() < tol);
    }

    #[test]
    fn test_nb_poles() {
        let s = flat_bilinear();
        assert_eq!(s.nb_u_poles(), 2);
        assert_eq!(s.nb_v_poles(), 2);
    }

    #[test]
    fn test_is_closed_open_patch() {
        let s = flat_bilinear();
        // A flat square patch is open, not closed.
        assert!(!s.is_u_closed());
        assert!(!s.is_v_closed());
    }

    #[test]
    fn test_empty_surface_bounds() {
        let s = BspSurface::new(3, 3);
        let (u_rng, v_rng) = bsp_surface_bounds(&s);
        // Falls back to default [0, 1].
        assert!((u_rng[0] - 0.0).abs() < 1e-12 && (u_rng[1] - 1.0).abs() < 1e-12);
        assert!((v_rng[0] - 0.0).abs() < 1e-12 && (v_rng[1] - 1.0).abs() < 1e-12);
    }
}
