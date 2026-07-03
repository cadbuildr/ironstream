// FILE: geom_nurbs_convert.rs
//! `GeomNurbsConvert` — B-spline / NURBS conversion utilities, mirroring
//! OpenCascade's `GeomConvert` / `Geom2dConvert` packages
//! (`src/ModelingData/TKG3d/GeomConvert/`).
//!
//! Provides:
//! - [`NurbsCurve`]    — a rational B-spline (NURBS) curve in 3D space
//! - [`NurbsSurface`]  — a rational B-spline (NURBS) surface in 3D space
//! - [`curve_to_nurbs`]    — convert a named curve kind to a [`NurbsCurve`]
//! - [`surface_to_nurbs`]  — convert a named surface kind to a [`NurbsSurface`]
//!
//! All code is pure Rust with no external dependencies.

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

const GP_RESOLUTION: f64 = 1.0e-15;

// ---------------------------------------------------------------------------
// Internal helpers — de Boor / knot utilities
// ---------------------------------------------------------------------------

/// Find the knot-span index for parameter `t` in a flat knot vector.
///
/// Returns the index `i` such that `fk[i] <= t < fk[i+1]`, clamped so that
/// index is always in `[degree, n_poles - 1]`.
fn find_span(fk: &[f64], degree: usize, n_poles: usize, t: f64) -> usize {
    let n = n_poles - 1; // last basis-function index
    // Clamp to the last valid span when t equals the end knot.
    if t >= fk[n + 1] {
        return n;
    }
    if t <= fk[degree] {
        return degree;
    }
    // Binary search.
    let mut lo = degree;
    let mut hi = n + 1;
    let mut mid = (lo + hi) / 2;
    while t < fk[mid] || t >= fk[mid + 1] {
        if t < fk[mid] {
            hi = mid;
        } else {
            lo = mid;
        }
        mid = (lo + hi) / 2;
    }
    mid
}

/// Evaluate a 3D NURBS curve using the rational de Boor algorithm.
///
/// `fk`      — flat (repeated) knot vector of length `n_poles + degree + 1`
/// `poles`   — control points as `[x, y, z]`
/// `weights` — one weight per pole (all 1.0 for non-rational)
/// `degree`  — polynomial degree
/// `t`       — parameter value
fn eval_nurbs_curve(
    fk: &[f64],
    poles: &[[f64; 3]],
    weights: &[f64],
    degree: usize,
    t: f64,
) -> [f64; 3] {
    let n = poles.len();
    if n == 0 {
        return [0.0; 3];
    }
    let span = find_span(fk, degree, n, t);
    let p = degree;

    // Build de Boor tableau in homogeneous coordinates: (wx, wy, wz, w).
    let mut d: Vec<(f64, f64, f64, f64)> = (0..=p)
        .map(|j| {
            let idx = (span + j).saturating_sub(p).min(n - 1);
            let w = weights[idx];
            (poles[idx][0] * w, poles[idx][1] * w, poles[idx][2] * w, w)
        })
        .collect();

    for r in 1..=p {
        for j in (r..=p).rev() {
            let i = span + j - p;
            let denom = if i + p + 1 - r < fk.len() && i < fk.len() {
                fk[i + p + 1 - r] - fk[i]
            } else {
                0.0
            };
            let alpha = if denom.abs() < GP_RESOLUTION {
                0.0
            } else {
                (t - fk[i]) / denom
            };
            d[j].0 = (1.0 - alpha) * d[j - 1].0 + alpha * d[j].0;
            d[j].1 = (1.0 - alpha) * d[j - 1].1 + alpha * d[j].1;
            d[j].2 = (1.0 - alpha) * d[j - 1].2 + alpha * d[j].2;
            d[j].3 = (1.0 - alpha) * d[j - 1].3 + alpha * d[j].3;
        }
    }
    let (hx, hy, hz, hw) = d[p];
    if hw.abs() < GP_RESOLUTION {
        [hx, hy, hz]
    } else {
        [hx / hw, hy / hw, hz / hw]
    }
}

/// Evaluate a 3D NURBS surface at `(u, v)` using tensor-product de Boor.
///
/// The surface is computed by first evaluating `v_degree + 1` NURBS curves
/// in the u-direction at parameter `u`, then interpolating those results
/// in the v-direction at parameter `v`.
fn eval_nurbs_surface(
    u_fk: &[f64],
    v_fk: &[f64],
    poles: &[Vec<[f64; 3]>],
    weights: &[Vec<f64>],
    u_degree: usize,
    v_degree: usize,
    u: f64,
    v: f64,
) -> [f64; 3] {
    let n_u = poles.len();
    let n_v = if n_u == 0 { 0 } else { poles[0].len() };
    if n_u == 0 || n_v == 0 {
        return [0.0; 3];
    }

    // For each row of v-control-points, evaluate the u-direction curve at `u`.
    // This gives us n_v "row points" (in homogeneous space).
    let v_span = find_span(v_fk, v_degree, n_v, v);

    // Collect the n_v columns touched by the v-span.
    let mut temp_poles: Vec<[f64; 3]> = Vec::with_capacity(v_degree + 1);
    let mut temp_w: Vec<f64> = Vec::with_capacity(v_degree + 1);

    for jv in 0..=v_degree {
        let col = (v_span + jv).saturating_sub(v_degree).min(n_v - 1);
        // Build single-column u-curve poles/weights.
        let col_poles: Vec<[f64; 3]> = (0..n_u).map(|iu| poles[iu][col]).collect();
        let col_weights: Vec<f64> = (0..n_u).map(|iu| weights[iu][col]).collect();
        let pt = eval_nurbs_curve(u_fk, &col_poles, &col_weights, u_degree, u);
        // Compute the weight at this column by treating the weight grid as a
        // scalar B-spline and evaluating it at u.
        let w_poles_1d: Vec<[f64; 3]> =
            col_weights.iter().map(|&w| [w, 0.0, 0.0]).collect();
        let w_at_u =
            eval_nurbs_curve(u_fk, &w_poles_1d, &vec![1.0f64; n_u], u_degree, u)[0];
        // Store in homogeneous form.
        let wau = w_at_u.max(GP_RESOLUTION);
        temp_poles.push([pt[0] * wau, pt[1] * wau, pt[2] * wau]);
        temp_w.push(wau);
    }

    // Now evaluate the v-direction with the temporary poles using the full v_fk.

    // Evaluate using de Boor in v (the temp_poles are already in homogeneous).
    let p = v_degree;
    let mut d: Vec<(f64, f64, f64, f64)> = temp_poles
        .iter()
        .zip(temp_w.iter())
        .map(|(pt, &w)| (pt[0], pt[1], pt[2], w))
        .collect();

    let lo_col = v_span.saturating_sub(v_degree);
    for r in 1..=p {
        for j in (r..=p).rev() {
            let ki = lo_col + j;
            let denom = if ki + p + 1 - r < v_fk.len() && ki < v_fk.len() {
                v_fk[ki + p + 1 - r] - v_fk[ki]
            } else {
                0.0
            };
            let alpha = if denom.abs() < GP_RESOLUTION {
                0.0
            } else {
                (v - v_fk[ki]) / denom
            };
            d[j].0 = (1.0 - alpha) * d[j - 1].0 + alpha * d[j].0;
            d[j].1 = (1.0 - alpha) * d[j - 1].1 + alpha * d[j].1;
            d[j].2 = (1.0 - alpha) * d[j - 1].2 + alpha * d[j].2;
            d[j].3 = (1.0 - alpha) * d[j - 1].3 + alpha * d[j].3;
        }
    }
    let (hx, hy, hz, hw) = d[p];
    if hw.abs() < GP_RESOLUTION {
        [hx, hy, hz]
    } else {
        [hx / hw, hy / hw, hz / hw]
    }
}

/// Build a uniform open (clamped) flat knot vector for the given `degree` and
/// `n_poles`.  Equivalent to multiplicity `degree+1` at both ends and
/// single-multiplicity interior knots.
fn uniform_open_knots(degree: usize, n_poles: usize) -> Vec<f64> {
    let n_knots = n_poles + degree + 1;
    let n_inner = n_knots - 2 * (degree + 1);
    let mut fk = Vec::with_capacity(n_knots);
    for _ in 0..=degree {
        fk.push(0.0);
    }
    for i in 1..=n_inner {
        fk.push(i as f64 / (n_inner + 1) as f64);
    }
    for _ in 0..=degree {
        fk.push(1.0);
    }
    fk
}

// ---------------------------------------------------------------------------
// NurbsCurve
// ---------------------------------------------------------------------------

/// A rational B-spline (NURBS) curve in 3D space.
///
/// Mirrors the data model of `Geom_BSplineCurve` from OpenCascade, using a
/// flat (repeated) knot vector together with weighted control points.
// occt: GeomConvert
#[derive(Clone, Debug)]
pub struct NurbsCurve {
    /// Polynomial degree of the curve.
    pub degree: u32,
    /// Flat (repeated) knot vector.  Length must equal `poles.len() + degree + 1`.
    pub knots: Vec<f64>,
    /// Control points (poles) in 3D Cartesian space.
    pub poles: Vec<[f64; 3]>,
    /// One weight per pole.  All `1.0` for a non-rational B-spline.
    pub weights: Vec<f64>,
}

impl NurbsCurve {
    /// `new(degree)` — create an empty NURBS curve with the given polynomial degree.
    ///
    /// Poles, knots, and weights are initially empty; use [`add_pole`] to build
    /// up the curve incrementally.
    pub fn new(degree: u32) -> Self {
        Self {
            degree,
            knots: Vec::new(),
            poles: Vec::new(),
            weights: Vec::new(),
        }
    }

    /// `add_pole(p, w)` — append a control point `p` with weight `w`.
    ///
    /// The flat knot vector is regenerated automatically using a uniform open
    /// (clamped) distribution each time a pole is added, so the curve remains
    /// valid after every call.
    pub fn add_pole(&mut self, p: [f64; 3], w: f64) {
        self.poles.push(p);
        self.weights.push(w.max(GP_RESOLUTION));
        // Recompute the knot vector to stay consistent.
        let d = self.degree as usize;
        let n = self.poles.len();
        if n > d {
            self.knots = uniform_open_knots(d, n);
        } else {
            // Not enough poles yet to define a valid curve of this degree;
            // store a trivial knot vector so the struct is always consistent.
            self.knots = vec![0.0; n + d + 1];
        }
    }

    /// `evaluate(t)` — evaluate the curve at parameter `t` ∈ [0, 1].
    ///
    /// Returns the 3D Cartesian point on the curve.  Returns `[0, 0, 0]` if
    /// the curve has no poles or fewer poles than `degree + 1`.
    pub fn evaluate(&self, t: f64) -> [f64; 3] {
        let d = self.degree as usize;
        if self.poles.len() <= d {
            return [0.0; 3];
        }
        eval_nurbs_curve(&self.knots, &self.poles, &self.weights, d, t)
    }

    /// `is_rational()` — returns `true` when any weight differs from `1.0` by
    /// more than a tight tolerance (`1e-10`).
    pub fn is_rational(&self) -> bool {
        self.weights.iter().any(|&w| (w - 1.0).abs() > 1.0e-10)
    }

    /// `nb_poles()` — returns the number of control points.
    pub fn nb_poles(&self) -> usize {
        self.poles.len()
    }
}

// ---------------------------------------------------------------------------
// NurbsSurface
// ---------------------------------------------------------------------------

/// A rational B-spline (NURBS) surface in 3D space.
///
/// Mirrors the data model of `Geom_BSplineSurface` from OpenCascade, using
/// tensor-product B-spline basis functions in u and v.
#[derive(Clone, Debug)]
pub struct NurbsSurface {
    /// Polynomial degree in the u direction.
    pub u_degree: u32,
    /// Polynomial degree in the v direction.
    pub v_degree: u32,
    /// Control point grid: `poles[i][j]` is the pole at (u-index i, v-index j).
    pub poles: Vec<Vec<[f64; 3]>>,
    /// Weight grid parallel to `poles`.  All `1.0` for non-rational surfaces.
    pub weights: Vec<Vec<f64>>,
    /// Flat knot vector in the u direction.
    u_knots: Vec<f64>,
    /// Flat knot vector in the v direction.
    v_knots: Vec<f64>,
}

impl NurbsSurface {
    /// `new(ud, vd)` — create an empty NURBS surface with the given polynomial
    /// degrees in u and v.
    pub fn new(ud: u32, vd: u32) -> Self {
        Self {
            u_degree: ud,
            v_degree: vd,
            poles: Vec::new(),
            weights: Vec::new(),
            u_knots: Vec::new(),
            v_knots: Vec::new(),
        }
    }

    /// `evaluate(u, v)` — evaluate the surface at `(u, v)` ∈ [0, 1] × [0, 1].
    ///
    /// Returns the 3D Cartesian point on the surface.  Returns `[0, 0, 0]` if
    /// the pole grid is empty or under-sized for the declared degrees.
    pub fn evaluate(&self, u: f64, v: f64) -> [f64; 3] {
        let ud = self.u_degree as usize;
        let vd = self.v_degree as usize;
        let nu = self.poles.len();
        let nv = if nu == 0 { 0 } else { self.poles[0].len() };
        if nu <= ud || nv <= vd {
            return [0.0; 3];
        }
        eval_nurbs_surface(
            &self.u_knots,
            &self.v_knots,
            &self.poles,
            &self.weights,
            ud,
            vd,
            u,
            v,
        )
    }

    /// `is_rational()` — returns `true` when any weight in the grid differs
    /// from `1.0` by more than `1e-10`.
    pub fn is_rational(&self) -> bool {
        self.weights
            .iter()
            .flat_map(|row| row.iter())
            .any(|&w| (w - 1.0).abs() > 1.0e-10)
    }

    // -----------------------------------------------------------------------
    // Builder helpers (not part of the public OCCT API surface but needed for
    // `surface_to_nurbs` and tests).
    // -----------------------------------------------------------------------

    /// Set the full pole/weight grid and regenerate the knot vectors.
    ///
    /// `grid[i][j]` is the control point at u-index `i`, v-index `j`.
    /// `wgrid` must have the same dimensions.
    pub fn set_poles(
        &mut self,
        grid: Vec<Vec<[f64; 3]>>,
        wgrid: Vec<Vec<f64>>,
    ) {
        let nu = grid.len();
        let nv = if nu == 0 { 0 } else { grid[0].len() };
        self.poles = grid;
        self.weights = wgrid;
        let ud = self.u_degree as usize;
        let vd = self.v_degree as usize;
        self.u_knots = if nu > ud {
            uniform_open_knots(ud, nu)
        } else {
            vec![0.0; nu + ud + 1]
        };
        self.v_knots = if nv > vd {
            uniform_open_knots(vd, nv)
        } else {
            vec![0.0; nv + vd + 1]
        };
    }
}

// ---------------------------------------------------------------------------
// Conversion stubs
// ---------------------------------------------------------------------------

/// `curve_to_nurbs(curve)` — convert a named curve type to a [`NurbsCurve`].
///
/// This is a stub that recognises a small set of common curve type names and
/// returns a representative NURBS approximation.  Unrecognised names yield a
/// default degree-3 straight-line curve from `[0,0,0]` to `[1,0,0]`.
///
/// Mirrors `GeomConvert::CurveToBSplineCurve` from OpenCascade.
// occt: GeomConvert
pub fn curve_to_nurbs(curve: &str) -> NurbsCurve {
    match curve.to_ascii_lowercase().trim() {
        // Unit circle arc approximated as a degree-2 rational NURBS (exact).
        // Quarter-circle: poles (1,0,0), (1,1,0), (0,1,0); weights 1, 1/√2, 1.
        "circle" | "arc" => {
            let w = std::f64::consts::FRAC_1_SQRT_2; // 1/√2
            let mut c = NurbsCurve::new(2);
            c.poles = vec![[1.0, 0.0, 0.0], [1.0, 1.0, 0.0], [0.0, 1.0, 0.0]];
            c.weights = vec![1.0, w, 1.0];
            c.knots = vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0];
            c
        }
        // Line segment from (0,0,0) to (1,0,0): degree-1 B-spline.
        "line" | "segment" => {
            let mut c = NurbsCurve::new(1);
            c.poles = vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]];
            c.weights = vec![1.0, 1.0];
            c.knots = vec![0.0, 0.0, 1.0, 1.0];
            c
        }
        // Cubic Bezier-form B-spline (S-curve).
        "bezier" | "spline" | "bspline" => {
            let mut c = NurbsCurve::new(3);
            c.poles = vec![
                [0.0, 0.0, 0.0],
                [0.333, 1.0, 0.0],
                [0.667, 1.0, 0.0],
                [1.0, 0.0, 0.0],
            ];
            c.weights = vec![1.0, 1.0, 1.0, 1.0];
            c.knots = vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0];
            c
        }
        // Ellipse approximated as degree-2 rational NURBS.
        // Semi-axes a=1, b=0.5; single-span approximation in [0,1].
        "ellipse" => {
            let w = std::f64::consts::FRAC_1_SQRT_2;
            let mut c = NurbsCurve::new(2);
            c.poles = vec![[1.0, 0.0, 0.0], [1.0, 0.5, 0.0], [0.0, 0.5, 0.0]];
            c.weights = vec![1.0, w, 1.0];
            c.knots = vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0];
            c
        }
        // Default: degree-3 straight line with four equidistant poles.
        _ => {
            let mut c = NurbsCurve::new(3);
            c.poles = vec![
                [0.0, 0.0, 0.0],
                [0.333, 0.0, 0.0],
                [0.667, 0.0, 0.0],
                [1.0, 0.0, 0.0],
            ];
            c.weights = vec![1.0, 1.0, 1.0, 1.0];
            c.knots = vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0];
            c
        }
    }
}

/// `surface_to_nurbs(surface)` — convert a named surface type to a
/// [`NurbsSurface`].
///
/// This is a stub that recognises a small set of common surface type names and
/// returns a representative NURBS approximation.  Unrecognised names yield a
/// default bilinear (degree-1 × degree-1) planar patch in the XY plane.
///
/// Mirrors `GeomConvert::SurfaceToBSplineSurface` from OpenCascade.
// occt: GeomConvert
pub fn surface_to_nurbs(surface: &str) -> NurbsSurface {
    match surface.to_ascii_lowercase().trim() {
        // Bilinear planar patch (unit square in XY plane).
        "plane" | "planar" | "flat" => {
            let mut s = NurbsSurface::new(1, 1);
            let grid = vec![
                vec![[0.0, 0.0, 0.0], [0.0, 1.0, 0.0]],
                vec![[1.0, 0.0, 0.0], [1.0, 1.0, 0.0]],
            ];
            let wgrid = vec![vec![1.0, 1.0], vec![1.0, 1.0]];
            s.set_poles(grid, wgrid);
            s
        }
        // Biquadratic rational NURBS for a spherical patch (one octant).
        // Exact quarter-sphere of radius 1 centred at the origin.
        "sphere" | "spherical" => {
            let w = std::f64::consts::FRAC_1_SQRT_2; // 1/√2
            let w2 = w * w; // 1/2
            let mut s = NurbsSurface::new(2, 2);
            // 3×3 control grid for the spherical patch (u: longitude, v: latitude).
            let grid = vec![
                vec![[1.0, 0.0, 0.0], [1.0, 0.0, 1.0], [0.0, 0.0, 1.0]],
                vec![[1.0, 1.0, 0.0], [1.0, 1.0, 1.0], [0.0, 0.0, 1.0]],
                vec![[0.0, 1.0, 0.0], [0.0, 1.0, 1.0], [0.0, 0.0, 1.0]],
            ];
            let wgrid = vec![
                vec![1.0, w, 1.0],
                vec![w, w2, w],
                vec![1.0, w, 1.0],
            ];
            s.set_poles(grid, wgrid);
            s
        }
        // Bicubic B-spline cylinder (unit radius, axis along Z, height 1).
        // Non-rational: the cross-section uses cosine/sine poles as an approximation.
        "cylinder" | "cylindrical" => {
            let mut s = NurbsSurface::new(3, 1);
            // 4 poles in u (degree-3 approximation of a circular arc).
            // 2 poles in v (linear extrusion along Z).
            let grid = vec![
                vec![[1.0, 0.0, 0.0], [1.0, 0.0, 1.0]],
                vec![[1.0, 1.0, 0.0], [1.0, 1.0, 1.0]],
                vec![[-1.0, 1.0, 0.0], [-1.0, 1.0, 1.0]],
                vec![[-1.0, 0.0, 0.0], [-1.0, 0.0, 1.0]],
            ];
            let wgrid = vec![
                vec![1.0, 1.0],
                vec![1.0, 1.0],
                vec![1.0, 1.0],
                vec![1.0, 1.0],
            ];
            s.set_poles(grid, wgrid);
            s
        }
        // Biquadratic patch for a toroidal surface (approximate).
        "torus" | "toroidal" => {
            let mut s = NurbsSurface::new(2, 2);
            // 3×3 control grid representing one quadrant of a torus
            // with major radius R=2, minor radius r=1.
            let r = 2.0_f64;
            let grid = vec![
                vec![[r + 1.0, 0.0, 0.0], [r + 1.0, 0.0, 1.0], [r, 0.0, 1.0]],
                vec![[r + 1.0, r + 1.0, 0.0], [r + 1.0, r + 1.0, 1.0], [r, r, 1.0]],
                vec![[0.0, r + 1.0, 0.0], [0.0, r + 1.0, 1.0], [0.0, r, 1.0]],
            ];
            let w = std::f64::consts::FRAC_1_SQRT_2;
            let wgrid = vec![
                vec![1.0, w, 1.0],
                vec![w, w * w, w],
                vec![1.0, w, 1.0],
            ];
            s.set_poles(grid, wgrid);
            s
        }
        // Default: bilinear patch.
        _ => {
            let mut s = NurbsSurface::new(1, 1);
            let grid = vec![
                vec![[0.0, 0.0, 0.0], [0.0, 1.0, 0.0]],
                vec![[1.0, 0.0, 0.0], [1.0, 1.0, 0.0]],
            ];
            let wgrid = vec![vec![1.0, 1.0], vec![1.0, 1.0]];
            s.set_poles(grid, wgrid);
            s
        }
    }
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // -----------------------------------------------------------------------
    // NurbsCurve — construction
    // -----------------------------------------------------------------------

    #[test]
    fn curve_new_is_empty() {
        let c = NurbsCurve::new(3);
        assert_eq!(c.degree, 3);
        assert_eq!(c.nb_poles(), 0);
        assert!(c.poles.is_empty());
        assert!(c.knots.is_empty());
    }

    #[test]
    fn curve_add_pole_increments_count() {
        let mut c = NurbsCurve::new(2);
        c.add_pole([0.0, 0.0, 0.0], 1.0);
        assert_eq!(c.nb_poles(), 1);
        c.add_pole([1.0, 0.0, 0.0], 1.0);
        assert_eq!(c.nb_poles(), 2);
        c.add_pole([1.0, 1.0, 0.0], 1.0);
        assert_eq!(c.nb_poles(), 3);
    }

    #[test]
    fn curve_knot_vector_length_after_add_pole() {
        let mut c = NurbsCurve::new(2);
        for i in 0..4 {
            c.add_pole([i as f64, 0.0, 0.0], 1.0);
        }
        // flat knot vector length = n_poles + degree + 1 = 4 + 2 + 1 = 7
        assert_eq!(c.knots.len(), 7);
    }

    #[test]
    fn curve_is_rational_all_unit_weights() {
        let mut c = NurbsCurve::new(1);
        c.add_pole([0.0, 0.0, 0.0], 1.0);
        c.add_pole([1.0, 0.0, 0.0], 1.0);
        assert!(!c.is_rational());
    }

    #[test]
    fn curve_is_rational_nonunit_weight() {
        let mut c = NurbsCurve::new(1);
        c.add_pole([0.0, 0.0, 0.0], 1.0);
        c.add_pole([1.0, 1.0, 0.0], 0.707);
        assert!(c.is_rational());
    }

    // -----------------------------------------------------------------------
    // NurbsCurve — evaluation
    // -----------------------------------------------------------------------

    #[test]
    fn linear_curve_start_end() {
        // Degree-1 line from (0,0,0) to (2,0,0).
        let mut c = NurbsCurve::new(1);
        c.add_pole([0.0, 0.0, 0.0], 1.0);
        c.add_pole([2.0, 0.0, 0.0], 1.0);
        let start = c.evaluate(0.0);
        let end = c.evaluate(1.0);
        let mid = c.evaluate(0.5);
        assert!((start[0]).abs() < 1.0e-10);
        assert!((end[0] - 2.0).abs() < 1.0e-10);
        assert!((mid[0] - 1.0).abs() < 1.0e-10);
    }

    #[test]
    fn evaluate_empty_returns_zero() {
        let c = NurbsCurve::new(3);
        let pt = c.evaluate(0.5);
        assert_eq!(pt, [0.0, 0.0, 0.0]);
    }

    #[test]
    fn quadratic_nurbs_endpoints() {
        // Quarter circle via degree-2 rational NURBS (exact).
        let w = std::f64::consts::FRAC_1_SQRT_2;
        let mut c = NurbsCurve::new(2);
        c.poles = vec![[1.0, 0.0, 0.0], [1.0, 1.0, 0.0], [0.0, 1.0, 0.0]];
        c.weights = vec![1.0, w, 1.0];
        c.knots = vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0];
        let start = c.evaluate(0.0);
        let end = c.evaluate(1.0);
        assert!((start[0] - 1.0).abs() < 1.0e-9, "start.x = {}", start[0]);
        assert!((start[1]).abs() < 1.0e-9, "start.y = {}", start[1]);
        assert!((end[0]).abs() < 1.0e-9, "end.x = {}", end[0]);
        assert!((end[1] - 1.0).abs() < 1.0e-9, "end.y = {}", end[1]);
    }

    #[test]
    fn quadratic_nurbs_midpoint_on_unit_circle() {
        // The midpoint of the degree-2 NURBS quarter circle must lie on the unit circle.
        let w = std::f64::consts::FRAC_1_SQRT_2;
        let mut c = NurbsCurve::new(2);
        c.poles = vec![[1.0, 0.0, 0.0], [1.0, 1.0, 0.0], [0.0, 1.0, 0.0]];
        c.weights = vec![1.0, w, 1.0];
        c.knots = vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0];
        let mid = c.evaluate(0.5);
        let r = (mid[0] * mid[0] + mid[1] * mid[1]).sqrt();
        assert!(
            (r - 1.0).abs() < 1.0e-9,
            "midpoint radius = {r}, expected 1.0"
        );
    }

    // -----------------------------------------------------------------------
    // NurbsSurface — construction
    // -----------------------------------------------------------------------

    #[test]
    fn surface_new_is_empty() {
        let s = NurbsSurface::new(2, 3);
        assert_eq!(s.u_degree, 2);
        assert_eq!(s.v_degree, 3);
        assert!(s.poles.is_empty());
        assert!(s.weights.is_empty());
    }

    #[test]
    fn surface_set_poles_stores_grid() {
        let mut s = NurbsSurface::new(1, 1);
        let grid = vec![
            vec![[0.0, 0.0, 0.0], [0.0, 1.0, 0.0]],
            vec![[1.0, 0.0, 0.0], [1.0, 1.0, 0.0]],
        ];
        let wgrid = vec![vec![1.0, 1.0], vec![1.0, 1.0]];
        s.set_poles(grid, wgrid);
        assert_eq!(s.poles.len(), 2);
        assert_eq!(s.poles[0].len(), 2);
    }

    #[test]
    fn surface_is_rational_unit_weights() {
        let mut s = NurbsSurface::new(1, 1);
        let grid = vec![
            vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]],
            vec![[0.0, 1.0, 0.0], [1.0, 1.0, 0.0]],
        ];
        let wgrid = vec![vec![1.0, 1.0], vec![1.0, 1.0]];
        s.set_poles(grid, wgrid);
        assert!(!s.is_rational());
    }

    #[test]
    fn surface_is_rational_nonunit_weights() {
        let mut s = NurbsSurface::new(1, 1);
        let grid = vec![
            vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]],
            vec![[0.0, 1.0, 0.0], [1.0, 1.0, 0.0]],
        ];
        let wgrid = vec![vec![1.0, 0.5], vec![1.0, 1.0]];
        s.set_poles(grid, wgrid);
        assert!(s.is_rational());
    }

    // -----------------------------------------------------------------------
    // NurbsSurface — evaluation
    // -----------------------------------------------------------------------

    #[test]
    fn bilinear_surface_corners() {
        let mut s = NurbsSurface::new(1, 1);
        let grid = vec![
            vec![[0.0, 0.0, 0.0], [0.0, 2.0, 0.0]],
            vec![[3.0, 0.0, 0.0], [3.0, 2.0, 0.0]],
        ];
        let wgrid = vec![vec![1.0, 1.0], vec![1.0, 1.0]];
        s.set_poles(grid, wgrid);
        let c00 = s.evaluate(0.0, 0.0);
        let c10 = s.evaluate(1.0, 0.0);
        let c01 = s.evaluate(0.0, 1.0);
        let c11 = s.evaluate(1.0, 1.0);
        assert!((c00[0]).abs() < 1.0e-9 && (c00[1]).abs() < 1.0e-9);
        assert!((c10[0] - 3.0).abs() < 1.0e-9);
        assert!((c01[1] - 2.0).abs() < 1.0e-9);
        assert!((c11[0] - 3.0).abs() < 1.0e-9 && (c11[1] - 2.0).abs() < 1.0e-9);
    }

    #[test]
    fn surface_evaluate_empty_returns_zero() {
        let s = NurbsSurface::new(2, 2);
        assert_eq!(s.evaluate(0.5, 0.5), [0.0, 0.0, 0.0]);
    }

    // -----------------------------------------------------------------------
    // curve_to_nurbs
    // -----------------------------------------------------------------------

    #[test]
    fn curve_to_nurbs_circle_is_rational() {
        let c = curve_to_nurbs("circle");
        assert!(c.is_rational(), "circle NURBS should be rational");
    }

    #[test]
    fn curve_to_nurbs_circle_degree_two() {
        let c = curve_to_nurbs("circle");
        assert_eq!(c.degree, 2);
    }

    #[test]
    fn curve_to_nurbs_line_degree_one() {
        let c = curve_to_nurbs("line");
        assert_eq!(c.degree, 1);
        assert!(!c.is_rational());
    }

    #[test]
    fn curve_to_nurbs_bezier_degree_three() {
        let c = curve_to_nurbs("bezier");
        assert_eq!(c.degree, 3);
    }

    #[test]
    fn curve_to_nurbs_unknown_gives_default() {
        let c = curve_to_nurbs("unknown_geom_type");
        assert_eq!(c.degree, 3);
        assert_eq!(c.nb_poles(), 4);
    }

    #[test]
    fn curve_to_nurbs_circle_unit_radius_at_midpoint() {
        let c = curve_to_nurbs("circle");
        let mid = c.evaluate(0.5);
        let r = (mid[0] * mid[0] + mid[1] * mid[1]).sqrt();
        assert!((r - 1.0).abs() < 1.0e-9, "r = {r}");
    }

    #[test]
    fn curve_to_nurbs_knots_length_consistent() {
        for name in &["circle", "line", "bezier", "ellipse", "plane"] {
            let c = curve_to_nurbs(name);
            let expected = c.poles.len() + c.degree as usize + 1;
            assert_eq!(
                c.knots.len(),
                expected,
                "curve '{}': knots.len()={} expected={}",
                name,
                c.knots.len(),
                expected
            );
        }
    }

    // -----------------------------------------------------------------------
    // surface_to_nurbs
    // -----------------------------------------------------------------------

    #[test]
    fn surface_to_nurbs_plane_degree_one() {
        let s = surface_to_nurbs("plane");
        assert_eq!(s.u_degree, 1);
        assert_eq!(s.v_degree, 1);
    }

    #[test]
    fn surface_to_nurbs_sphere_is_rational() {
        let s = surface_to_nurbs("sphere");
        assert!(s.is_rational(), "sphere NURBS should be rational");
    }

    #[test]
    fn surface_to_nurbs_sphere_degree_two() {
        let s = surface_to_nurbs("sphere");
        assert_eq!(s.u_degree, 2);
        assert_eq!(s.v_degree, 2);
    }

    #[test]
    fn surface_to_nurbs_cylinder_degree_u_three() {
        let s = surface_to_nurbs("cylinder");
        assert_eq!(s.u_degree, 3);
        assert_eq!(s.v_degree, 1);
    }

    #[test]
    fn surface_to_nurbs_torus_has_3x3_grid() {
        let s = surface_to_nurbs("torus");
        assert_eq!(s.poles.len(), 3);
        assert_eq!(s.poles[0].len(), 3);
    }

    #[test]
    fn surface_to_nurbs_unknown_gives_default() {
        let s = surface_to_nurbs("unknown_surface");
        assert_eq!(s.u_degree, 1);
        assert_eq!(s.v_degree, 1);
    }

    #[test]
    fn surface_to_nurbs_plane_corner_at_origin() {
        let s = surface_to_nurbs("plane");
        let pt = s.evaluate(0.0, 0.0);
        assert!((pt[0]).abs() < 1.0e-9 && (pt[1]).abs() < 1.0e-9 && (pt[2]).abs() < 1.0e-9);
    }
}
