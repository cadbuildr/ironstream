// FILE: geom_bezier_surface.rs
// occt: Geom_BezierSurface

/// Bernstein basis polynomial B_{i,n}(t) = C(n,i) * t^i * (1-t)^(n-i).
///
/// Uses integer binomial computed without overflow for the degrees expected
/// in a Bezier surface (degree <= ~25 is fine with f64 mantissa).
fn bernstein(n: u32, i: u32, t: f64) -> f64 {
    if i > n {
        return 0.0;
    }
    // binomial coefficient C(n, i)
    let binom = {
        let k = i.min(n - i) as usize;
        let n = n as usize;
        let mut c = 1.0f64;
        for j in 0..k {
            c = c * (n - j) as f64 / (j + 1) as f64;
        }
        c
    };
    binom * t.powi(i as i32) * (1.0 - t).powi((n - i) as i32)
}

/// A Bézier surface of bi-degree (u_degree × v_degree) in 3-D.
///
/// The surface is defined by a (u_degree+1) × (v_degree+1) grid of control
/// poles. Evaluation uses the Bernstein basis factored independently in u and
/// v:
///
/// ```text
/// S(u, v) = Σ_i Σ_j B_{i,u_degree}(u) · B_{j,v_degree}(v) · poles[i][j]
/// ```
///
/// When optional `weights` are provided the rational (NURBS) form is used:
/// ```text
/// S(u, v) = Σ_i Σ_j B_i(u)·B_j(v)·w_ij·P_ij / Σ_i Σ_j B_i(u)·B_j(v)·w_ij
/// ```
///
/// # OCCT analogue
/// Models `Geom_BezierSurface` from Open CASCADE Technology.
// occt: Geom_BezierSurface
#[derive(Clone, Debug)]
pub struct BezierSurface {
    /// Control poles indexed `[i][j]` where `i ∈ [0, u_degree]` and
    /// `j ∈ [0, v_degree]`.  Each pole is `[x, y, z]`.
    pub poles: Vec<Vec<[f64; 3]>>,
    /// Polynomial degree in the u parametric direction.
    pub u_degree: u32,
    /// Polynomial degree in the v parametric direction.
    pub v_degree: u32,
    /// Optional per-pole weights for rational (NURBS) evaluation.
    /// When `None` the surface is non-rational (all weights = 1).
    pub weights: Option<Vec<Vec<f64>>>,
}

impl BezierSurface {
    /// Return the polynomial degree in the u direction as `usize`.
    ///
    /// This method exists alongside the public `u_degree: u32` field so that
    /// callers using the OCCT-style `surface.u_degree()` method syntax continue
    /// to compile, and so the return type is consistent with
    /// `GeomBSplineSurface::u_degree()`.
    pub fn u_degree(&self) -> usize {
        self.u_degree as usize
    }

    /// Return the polynomial degree in the v direction as `usize`.
    ///
    /// This method exists alongside the public `v_degree: u32` field so that
    /// callers using the OCCT-style `surface.v_degree()` method syntax continue
    /// to compile, and so the return type is consistent with
    /// `GeomBSplineSurface::v_degree()`.
    pub fn v_degree(&self) -> usize {
        self.v_degree as usize
    }

    /// Return the number of poles in the u direction.
    /// Mirrors `Geom_BezierSurface::NbUPoles`.
    pub fn nb_u_poles(&self) -> usize {
        self.poles.len()
    }

    /// Return the number of poles in the v direction.
    /// Mirrors `Geom_BezierSurface::NbVPoles`.
    pub fn nb_v_poles(&self) -> usize {
        if self.poles.is_empty() { 0 } else { self.poles[0].len() }
    }

    /// Return `false` — Bézier surfaces are never periodic.
    /// Mirrors `Geom_BezierSurface::IsUPeriodic`.
    pub fn is_u_periodic(&self) -> bool {
        false
    }

    /// Return `false` — Bézier surfaces are never periodic.
    /// Mirrors `Geom_BezierSurface::IsVPeriodic`.
    pub fn is_v_periodic(&self) -> bool {
        false
    }

    /// Return `true` if this surface has non-unit weights.
    /// Mirrors `Geom_BezierSurface::IsRational`.
    pub fn is_rational(&self) -> bool {
        self.weights.is_some()
    }

    /// Return the weight at pole `(i, j)` (1-based indices).
    /// Returns `1.0` for non-rational surfaces.
    /// Mirrors `Geom_BezierSurface::Weight`.
    pub fn weight(&self, i: usize, j: usize) -> f64 {
        match &self.weights {
            Some(w) => w[i - 1][j - 1],
            None => 1.0,
        }
    }

    /// Return the pole at (1-based) indices `(i, j)`.
    /// Mirrors `Geom_BezierSurface::Pole`.
    pub fn pole(&self, i: usize, j: usize) -> [f64; 3] {
        self.poles[i - 1][j - 1]
    }

    /// Evaluate the surface at `(u, v)` — alias for `evaluate`.
    /// Mirrors `Geom_BezierSurface::Value`.
    pub fn value(&self, u: f64, v: f64) -> [f64; 3] {
        self.evaluate(u, v)
    }

    /// Return `(point, du, dv)` — point and partial derivatives at `(u, v)`.
    /// Mirrors `Geom_BezierSurface::D1`.
    pub fn d1(&self, u: f64, v: f64) -> ([f64; 3], [f64; 3], [f64; 3]) {
        (self.evaluate(u, v), self.d1u(u, v), self.d1v(u, v))
    }

    /// Elevate the u-degree to `new_degree`, recomputing the control net.
    /// Mirrors `Geom_BezierSurface::Increase(UDeg, VDeg)`.
    pub fn increase_u_degree(&mut self, new_degree: u32) {
        if new_degree <= self.u_degree {
            return;
        }
        // Degree elevation: elevate one step at a time.
        while self.u_degree < new_degree {
            self.elevate_u_once();
        }
    }

    fn elevate_u_once(&mut self) {
        let n = self.u_degree as usize; // old degree
        let nv = self.nb_v_poles();
        // new_poles[i][j] for i in 0..=(n+1), j in 0..nv
        let mut new_poles: Vec<Vec<[f64; 3]>> = vec![vec![[0.0; 3]; nv]; n + 2];
        for j in 0..nv {
            // First point unchanged
            new_poles[0][j] = self.poles[0][j];
            // Interior points: degree elevation formula
            for i in 1..=(n) {
                let alpha = i as f64 / (n as f64 + 1.0);
                let p_prev = self.poles[i - 1][j];
                let p_curr = self.poles[i][j];
                new_poles[i][j] = [
                    alpha * p_prev[0] + (1.0 - alpha) * p_curr[0],
                    alpha * p_prev[1] + (1.0 - alpha) * p_curr[1],
                    alpha * p_prev[2] + (1.0 - alpha) * p_curr[2],
                ];
            }
            // Last point unchanged
            new_poles[n + 1][j] = self.poles[n][j];
        }
        self.poles = new_poles;
        self.u_degree += 1;
    }

    /// Elevate the v-degree to `new_degree`.
    /// Mirrors `Geom_BezierSurface::Increase(UDeg, VDeg)`.
    pub fn increase_v_degree(&mut self, new_degree: u32) {
        if new_degree <= self.v_degree {
            return;
        }
        while self.v_degree < new_degree {
            self.elevate_v_once();
        }
    }

    fn elevate_v_once(&mut self) {
        let m = self.v_degree as usize; // old degree
        let nu = self.nb_u_poles();
        let mut new_poles: Vec<Vec<[f64; 3]>> = vec![Vec::with_capacity(m + 2); nu];
        for i in 0..nu {
            // First point
            new_poles[i].push(self.poles[i][0]);
            for j in 1..=(m) {
                let alpha = j as f64 / (m as f64 + 1.0);
                let p_prev = self.poles[i][j - 1];
                let p_curr = self.poles[i][j];
                new_poles[i].push([
                    alpha * p_prev[0] + (1.0 - alpha) * p_curr[0],
                    alpha * p_prev[1] + (1.0 - alpha) * p_curr[1],
                    alpha * p_prev[2] + (1.0 - alpha) * p_curr[2],
                ]);
            }
            // Last point
            new_poles[i].push(self.poles[i][m]);
        }
        self.poles = new_poles;
        self.v_degree += 1;
    }

    /// Insert a row of poles at (1-based) u-index `at`.
    /// Mirrors `Geom_BezierSurface::InsertPoleRowAfter`.
    pub fn insert_u_pole_row(&mut self, at: usize, row: &[[f64; 3]], _weights: Option<&[f64]>) {
        self.poles.insert(at - 1, row.to_vec());
        // Degree stays the same when inserting a row explicitly (stub behaviour).
    }

    /// Insert a column of poles at (1-based) v-index `at`.
    /// Mirrors `Geom_BezierSurface::InsertPoleColAfter`.
    pub fn insert_v_pole_col(&mut self, at: usize, col: &[[f64; 3]], _weights: Option<&[f64]>) {
        for (i, row) in self.poles.iter_mut().enumerate() {
            let p = if i < col.len() { col[i] } else { [0.0; 3] };
            row.insert(at - 1, p);
        }
    }

    /// Construct a zero-initialised surface with the given degrees.
    ///
    /// All `(u_degree + 1) × (v_degree + 1)` poles start at the origin
    /// `[0.0, 0.0, 0.0]`.
    pub fn new(u_degree: u32, v_degree: u32) -> Self {
        let nu = (u_degree + 1) as usize;
        let nv = (v_degree + 1) as usize;
        let poles = vec![vec![[0.0f64; 3]; nv]; nu];
        BezierSurface {
            poles,
            u_degree,
            v_degree,
            weights: None,
        }
    }

    /// Set the control pole at index `(i, j)` (0-based).
    ///
    /// # Panics
    /// Panics if `i > u_degree` or `j > v_degree`.
    pub fn set_pole(&mut self, i: usize, j: usize, p: [f64; 3]) {
        self.poles[i][j] = p;
    }

    /// Evaluate the surface at `(u, v) ∈ [0, 1]²`.
    ///
    /// Returns the 3-D point `S(u, v)`.
    pub fn evaluate(&self, u: f64, v: f64) -> [f64; 3] {
        let n = self.u_degree;
        let m = self.v_degree;
        let mut num = [0.0f64; 3];
        let mut den = 0.0f64;
        for i in 0..=(n as usize) {
            let bu = bernstein(n, i as u32, u);
            for j in 0..=(m as usize) {
                let bv = bernstein(m, j as u32, v);
                let wij = match &self.weights {
                    Some(w) => w[i][j],
                    None => 1.0,
                };
                let bw = bu * bv * wij;
                let p = self.poles[i][j];
                num[0] += bw * p[0];
                num[1] += bw * p[1];
                num[2] += bw * p[2];
                den += bw;
            }
        }
        if den == 0.0 {
            [0.0; 3]
        } else {
            [num[0] / den, num[1] / den, num[2] / den]
        }
    }

    /// Partial derivative with respect to `u` at `(u, v)`.
    ///
    /// Computed via the Bernstein derivative identity:
    /// `dB_{i,n}/du = n * (B_{i-1,n-1}(u) - B_{i,n-1}(u))`.
    pub fn d1u(&self, u: f64, v: f64) -> [f64; 3] {
        let n = self.u_degree;
        let m = self.v_degree;
        if n == 0 {
            return [0.0; 3];
        }
        let mut result = [0.0f64; 3];
        for i in 0..=(n as usize) {
            // dB_{i,n}/du = n * (B_{i-1,n-1} - B_{i,n-1})
            let b_prev = if i > 0 {
                bernstein(n - 1, (i - 1) as u32, u)
            } else {
                0.0
            };
            let b_curr = if (i as u32) <= n - 1 {
                bernstein(n - 1, i as u32, u)
            } else {
                0.0
            };
            let dbu = (n as f64) * (b_prev - b_curr);
            for j in 0..=(m as usize) {
                let bv = bernstein(m, j as u32, v);
                let w = dbu * bv;
                let p = self.poles[i][j];
                result[0] += w * p[0];
                result[1] += w * p[1];
                result[2] += w * p[2];
            }
        }
        result
    }

    /// Partial derivative with respect to `v` at `(u, v)`.
    pub fn d1v(&self, u: f64, v: f64) -> [f64; 3] {
        let n = self.u_degree;
        let m = self.v_degree;
        if m == 0 {
            return [0.0; 3];
        }
        let mut result = [0.0f64; 3];
        for i in 0..=(n as usize) {
            let bu = bernstein(n, i as u32, u);
            for j in 0..=(m as usize) {
                let b_prev = if j > 0 {
                    bernstein(m - 1, (j - 1) as u32, v)
                } else {
                    0.0
                };
                let b_curr = if (j as u32) <= m - 1 {
                    bernstein(m - 1, j as u32, v)
                } else {
                    0.0
                };
                let dbv = (m as f64) * (b_prev - b_curr);
                let w = bu * dbv;
                let p = self.poles[i][j];
                result[0] += w * p[0];
                result[1] += w * p[1];
                result[2] += w * p[2];
            }
        }
        result
    }

    /// Unit surface normal at `(u, v)`, computed as `dS/du × dS/dv` normalised
    /// to unit length.
    ///
    /// Returns `[0.0, 0.0, 0.0]` when the cross product is (near) zero, i.e. at
    /// a degenerate point where the partial derivatives are parallel or vanish.
    pub fn normal(&self, u: f64, v: f64) -> [f64; 3] {
        let du = self.d1u(u, v);
        let dv = self.d1v(u, v);
        // Cross product du × dv
        let cx = du[1] * dv[2] - du[2] * dv[1];
        let cy = du[2] * dv[0] - du[0] * dv[2];
        let cz = du[0] * dv[1] - du[1] * dv[0];
        let len = (cx * cx + cy * cy + cz * cz).sqrt();
        if len < 1.0e-14 {
            [0.0, 0.0, 0.0]
        } else {
            [cx / len, cy / len, cz / len]
        }
    }
}

// ---------------------------------------------------------------------------
// Builder
// ---------------------------------------------------------------------------

/// Builder for [`BezierSurface`] assembled one row at a time.
///
/// Each call to [`add_row`](BezierSurfaceBuilder::add_row) appends one
/// u-isoline of control points.  The degrees are inferred from the dimensions
/// of the finished net when [`build`](BezierSurfaceBuilder::build) is called.
///
/// # OCCT analogue
/// Provides a convenience construction path not present in OCCT's C++ API,
/// which takes a full 2-D array in the constructor.
#[derive(Clone, Debug)]
pub struct BezierSurfaceBuilder {
    /// The control net being accumulated, indexed `[u_index][v_index]`.
    pub control_net: Vec<Vec<[f64; 3]>>,
}

impl BezierSurfaceBuilder {
    /// Create an empty builder with no rows yet.
    pub fn new() -> Self {
        BezierSurfaceBuilder {
            control_net: Vec::new(),
        }
    }

    /// Append one row of control poles (one u-isoline).
    ///
    /// All rows must have the same number of columns; this is checked in
    /// [`build`](BezierSurfaceBuilder::build).
    pub fn add_row(&mut self, row: Vec<[f64; 3]>) {
        self.control_net.push(row);
    }

    /// Consume the builder and produce a [`BezierSurface`].
    ///
    /// # Panics
    /// - If no rows have been added.
    /// - If any row has a different number of columns than the first row.
    /// - If any row is empty.
    pub fn build(self) -> BezierSurface {
        assert!(
            !self.control_net.is_empty(),
            "BezierSurfaceBuilder::build: no rows added"
        );
        let nv = self.control_net[0].len();
        assert!(
            nv > 0,
            "BezierSurfaceBuilder::build: rows must not be empty"
        );
        for (idx, row) in self.control_net.iter().enumerate() {
            assert!(
                row.len() == nv,
                "BezierSurfaceBuilder::build: row {} has {} columns, expected {}",
                idx,
                row.len(),
                nv
            );
        }
        let u_degree = (self.control_net.len() - 1) as u32;
        let v_degree = (nv - 1) as u32;
        BezierSurface {
            poles: self.control_net,
            u_degree,
            v_degree,
            weights: None,
        }
    }
}

impl Default for BezierSurfaceBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Backward-compatibility alias and constructor
// ---------------------------------------------------------------------------

/// Type alias for [`BezierSurface`] that matches the OCCT class name
/// `Geom_BezierSurface` and preserves source compatibility with older callers.
pub type GeomBezierSurface = BezierSurface;

impl BezierSurface {
    /// Construct directly from an owned 2-D pole grid.
    ///
    /// `poles[i][j]` is the `(i, j)` control point (0-based).
    /// `weights`, if provided, must have the same dimensions as `poles`.
    ///
    /// The degrees are inferred from the grid dimensions:
    /// `u_degree = poles.len() - 1`, `v_degree = poles[0].len() - 1`.
    ///
    /// # Panics
    /// Panics if `poles` is empty or any row has a different length from the first.
    pub fn from_owned(poles: Vec<Vec<[f64; 3]>>, weights: Option<Vec<Vec<f64>>>) -> Self {
        assert!(!poles.is_empty(), "BezierSurface::from_owned: poles must not be empty");
        let nv = poles[0].len();
        assert!(nv > 0, "BezierSurface::from_owned: pole rows must not be empty");
        for row in &poles {
            assert!(row.len() == nv, "BezierSurface::from_owned: all rows must have the same length");
        }
        let u_degree = (poles.len() - 1) as u32;
        let v_degree = (nv - 1) as u32;
        BezierSurface { poles, u_degree, v_degree, weights }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // Helper: bilinear patch
    //   poles[0][0] = (0,0,0)  poles[0][1] = (1,0,0)
    //   poles[1][0] = (0,1,0)  poles[1][1] = (1,1,0)
    fn bilinear() -> BezierSurface {
        let mut s = BezierSurface::new(1, 1);
        s.set_pole(0, 0, [0.0, 0.0, 0.0]);
        s.set_pole(0, 1, [1.0, 0.0, 0.0]);
        s.set_pole(1, 0, [0.0, 1.0, 0.0]);
        s.set_pole(1, 1, [1.0, 1.0, 0.0]);
        s
    }

    #[test]
    fn test_degrees() {
        let s = bilinear();
        assert_eq!(s.u_degree, 1);
        assert_eq!(s.v_degree, 1);
    }

    #[test]
    fn test_corners() {
        let s = bilinear();
        let p00 = s.evaluate(0.0, 0.0);
        assert!((p00[0] - 0.0).abs() < 1e-12 && (p00[1] - 0.0).abs() < 1e-12);

        let p10 = s.evaluate(1.0, 0.0);
        assert!((p10[0] - 0.0).abs() < 1e-12 && (p10[1] - 1.0).abs() < 1e-12);

        let p01 = s.evaluate(0.0, 1.0);
        assert!((p01[0] - 1.0).abs() < 1e-12 && (p01[1] - 0.0).abs() < 1e-12);

        let p11 = s.evaluate(1.0, 1.0);
        assert!((p11[0] - 1.0).abs() < 1e-12 && (p11[1] - 1.0).abs() < 1e-12);
    }

    #[test]
    fn test_center() {
        let s = bilinear();
        let mid = s.evaluate(0.5, 0.5);
        assert!((mid[0] - 0.5).abs() < 1e-12);
        assert!((mid[1] - 0.5).abs() < 1e-12);
        assert!((mid[2] - 0.0).abs() < 1e-12);
    }

    #[test]
    fn test_d1u_d1v_bilinear() {
        let s = bilinear();
        // S(u,v) = (v, u, 0), so dS/du = (0,1,0) and dS/dv = (1,0,0).
        let du = s.d1u(0.5, 0.5);
        assert!((du[0] - 0.0).abs() < 1e-10, "du.x = {}", du[0]);
        assert!((du[1] - 1.0).abs() < 1e-10, "du.y = {}", du[1]);
        assert!((du[2] - 0.0).abs() < 1e-10);

        let dv = s.d1v(0.5, 0.5);
        assert!((dv[0] - 1.0).abs() < 1e-10, "dv.x = {}", dv[0]);
        assert!((dv[1] - 0.0).abs() < 1e-10, "dv.y = {}", dv[1]);
        assert!((dv[2] - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_normal_bilinear() {
        let s = bilinear();
        // dS/du = (0,1,0), dS/dv = (1,0,0).
        // Normal = dS/du × dS/dv = (0,1,0)×(1,0,0) = (0*0-0*0, 0*1-0*0, 0*0-1*1) = (0,0,-1).
        // This is consistent with OCCT's Geom_Surface normal convention.
        let n = s.normal(0.5, 0.5);
        assert!((n[0] - 0.0).abs() < 1e-10);
        assert!((n[1] - 0.0).abs() < 1e-10);
        assert!((n[2] + 1.0).abs() < 1e-10, "n.z = {}", n[2]);
    }

    #[test]
    fn test_bernstein_partition_of_unity() {
        let n = 3u32;
        for &t in &[0.0, 0.25, 0.5, 0.75, 1.0] {
            let sum: f64 = (0..=n).map(|i| bernstein(n, i, t)).sum();
            assert!((sum - 1.0).abs() < 1e-14, "partition of unity failed at t={}", t);
        }
    }

    #[test]
    fn test_builder() {
        let mut b = BezierSurfaceBuilder::new();
        b.add_row(vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]);
        b.add_row(vec![[0.0, 1.0, 0.0], [1.0, 1.0, 0.0]]);
        let s = b.build();
        assert_eq!(s.u_degree, 1);
        assert_eq!(s.v_degree, 1);
        let mid = s.evaluate(0.5, 0.5);
        assert!((mid[0] - 0.5).abs() < 1e-12);
        assert!((mid[1] - 0.5).abs() < 1e-12);
    }

    #[test]
    fn test_constant_surface() {
        let p = [3.0f64, 4.0, 5.0];
        let mut s = BezierSurface::new(2, 2);
        for i in 0..3 {
            for j in 0..3 {
                s.set_pole(i, j, p);
            }
        }
        for &u in &[0.0, 0.5, 1.0] {
            for &v in &[0.0, 0.5, 1.0] {
                let ev = s.evaluate(u, v);
                assert!((ev[0] - 3.0).abs() < 1e-12);
                assert!((ev[1] - 4.0).abs() < 1e-12);
                assert!((ev[2] - 5.0).abs() < 1e-12);
            }
        }
    }

    #[test]
    fn test_new_zeros() {
        let s = BezierSurface::new(2, 3);
        assert_eq!(s.u_degree, 2);
        assert_eq!(s.v_degree, 3);
        assert_eq!(s.poles.len(), 3);
        assert_eq!(s.poles[0].len(), 4);
        let pt = s.evaluate(0.5, 0.5);
        assert_eq!(pt, [0.0, 0.0, 0.0]);
    }
}
