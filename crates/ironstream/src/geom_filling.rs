// FILE: geom_filling.rs
//! `GeomFilling` — B-spline surface filling and Coons-patch algorithms,
//! mirroring OpenCascade's `GeomFill_BSplineCurves` and
//! `GeomFill_CoonsAlgPatch` from `src/ModelingData/TKG3d/GeomFill/`.
//!
//! Provides:
//!
//! - [`FillingStyle`]    — style selector (Curved / Strech / Coons).
//! - [`BSplineFilling`]  — surface fill bounded by polyline boundary curves,
//!   modelled after `GeomFill_BSplineCurves`.
//! - [`CoonsPatch`]      — transfinite Coons-patch blending from four boundary
//!   polylines, modelled after `GeomFill_CoonsAlgPatch`.
//!
//! All implementations use **only** the Rust standard library; no external
//! crates are required.

// ---------------------------------------------------------------------------
// FillingStyle
// ---------------------------------------------------------------------------

/// Selects the blending algorithm used when filling a surface from boundary
/// curves.
///
/// Mirrors `GeomFill_FillingStyle`.
// occt-ref: GeomFill_BSplineCurves
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FillingStyle {
    /// Curved blending — minimises curvature variation across the interior.
    Curved,
    /// Stretch blending — the boundary curves are used as-is; the interior is
    /// filled by a simple bilinear formula.  (Note: named "Strech" in OCCT.)
    Strech,
    /// Coons-patch blending — transfinite interpolation from the four sides.
    Coons,
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

/// Sample a polyline at normalised arc-length parameter `t` ∈ [0, 1].
///
/// Returns the interpolated point.  When the polyline has fewer than 2 points
/// the first point (or the origin if empty) is returned unchanged.
fn sample_polyline(pts: &[[f64; 3]], t: f64) -> [f64; 3] {
    let n = pts.len();
    if n == 0 {
        return [0.0, 0.0, 0.0];
    }
    if n == 1 {
        return pts[0];
    }

    // Compute cumulative chord lengths.
    let mut lengths = Vec::with_capacity(n);
    lengths.push(0.0_f64);
    for i in 1..n {
        let dx = pts[i][0] - pts[i - 1][0];
        let dy = pts[i][1] - pts[i - 1][1];
        let dz = pts[i][2] - pts[i - 1][2];
        let seg = (dx * dx + dy * dy + dz * dz).sqrt();
        lengths.push(lengths[i - 1] + seg);
    }

    let total = lengths[n - 1];
    if total == 0.0 {
        return pts[0];
    }

    let target = t.clamp(0.0, 1.0) * total;

    // Binary search for the spanning segment.
    let idx = lengths
        .partition_point(|&l| l <= target)
        .saturating_sub(1)
        .min(n - 2);

    let seg_len = lengths[idx + 1] - lengths[idx];
    let local_t = if seg_len > 0.0 {
        (target - lengths[idx]) / seg_len
    } else {
        0.0
    };

    let a = pts[idx];
    let b = pts[idx + 1];
    [
        a[0] + local_t * (b[0] - a[0]),
        a[1] + local_t * (b[1] - a[1]),
        a[2] + local_t * (b[2] - a[2]),
    ]
}

/// Add two 3-D points component-wise.
#[inline]
fn add3(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

/// Subtract two 3-D points component-wise.
#[inline]
fn sub3(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

/// Scale a 3-D point by a scalar.
#[inline]
fn scale3(a: [f64; 3], s: f64) -> [f64; 3] {
    [a[0] * s, a[1] * s, a[2] * s]
}

// ---------------------------------------------------------------------------
// BSplineFilling
// ---------------------------------------------------------------------------

/// Surface filling from an ordered list of boundary polylines.
///
/// Mirrors `GeomFill_BSplineCurves`.  Each boundary is stored as a polyline
/// of 3-D points; [`build`] samples the surface on a regular grid and returns
/// the flattened point cloud in row-major order (u varies fastest).
///
/// ## Usage
///
/// ```rust
/// # use ironstream::geom_filling::{BSplineFilling, FillingStyle};
/// let mut fill = BSplineFilling::new(FillingStyle::Coons);
/// fill.add_boundary(vec![[0.0,0.0,0.0],[1.0,0.0,0.0]]);
/// fill.add_boundary(vec![[1.0,0.0,0.0],[1.0,1.0,0.0]]);
/// fill.add_boundary(vec![[1.0,1.0,0.0],[0.0,1.0,0.0]]);
/// fill.add_boundary(vec![[0.0,1.0,0.0],[0.0,0.0,0.0]]);
/// assert!(fill.is_done());
/// let pts = fill.build();
/// assert!(!pts.is_empty());
/// ```
// occt-ref: GeomFill_BSplineCurves
#[derive(Clone, Debug)]
pub struct BSplineFilling {
    /// Ordered boundary polylines (up to 4).
    pub boundary_curves: Vec<Vec<[f64; 3]>>,
    /// Blending style.
    pub style: FillingStyle,
}

impl BSplineFilling {
    /// Construct an empty `BSplineFilling` with the given blending `style`.
    pub fn new(style: FillingStyle) -> Self {
        Self {
            boundary_curves: Vec::new(),
            style,
        }
    }

    /// Append a boundary polyline.  At most four boundaries are used by
    /// [`build`]; additional boundaries beyond the fourth are stored but
    /// ignored during surface evaluation.
    pub fn add_boundary(&mut self, pts: Vec<[f64; 3]>) {
        self.boundary_curves.push(pts);
    }

    /// Return `true` when enough boundaries have been provided for [`build`]
    /// to produce a surface (2, 3, or 4 boundaries with at least 2 points
    /// each).
    pub fn is_done(&self) -> bool {
        let n = self.boundary_curves.len();
        if n < 2 || n > 4 {
            return false;
        }
        self.boundary_curves.iter().all(|b| b.len() >= 2)
    }

    /// Build and return a sample of the filled surface as a flat list of 3-D
    /// points.
    ///
    /// The surface is sampled on a `resolution × resolution` grid where
    /// `resolution` is derived from the maximum number of control points among
    /// all boundaries (clamped to [8, 64]).  Points are stored in row-major
    /// order: `u` (0 → 1) varies in the inner loop, `v` (0 → 1) in the outer
    /// loop.
    ///
    /// Returns an empty `Vec` when [`is_done`] is `false`.
    pub fn build(&self) -> Vec<[f64; 3]> {
        if !self.is_done() {
            return Vec::new();
        }

        // Choose a resolution that respects the boundary density.
        let max_pts = self
            .boundary_curves
            .iter()
            .map(|b| b.len())
            .max()
            .unwrap_or(2);
        let res = max_pts.clamp(8, 64);

        let n = self.boundary_curves.len();
        let mut result = Vec::with_capacity(res * res);

        for iv in 0..res {
            let v = iv as f64 / (res - 1) as f64;
            for iu in 0..res {
                let u = iu as f64 / (res - 1) as f64;
                let pt = match n {
                    2 => self.eval_two(u, v),
                    3 => self.eval_three(u, v),
                    _ => self.eval_four(u, v),
                };
                result.push(pt);
            }
        }

        result
    }

    // ---- internal evaluation methods ----------------------------------------

    /// Evaluate the filling surface for 2 boundary curves (ruled surface).
    fn eval_two(&self, u: f64, _v: f64) -> [f64; 3] {
        // Treat the two curves as the u=0 and u=1 iso-lines.
        let p0 = sample_polyline(&self.boundary_curves[0], u);
        let p1 = sample_polyline(&self.boundary_curves[1], u);
        // Blend along v.
        add3(scale3(p0, 1.0 - _v), scale3(p1, _v))
    }

    /// Evaluate the filling surface for 3 boundary curves (triangular patch).
    fn eval_three(&self, u: f64, v: f64) -> [f64; 3] {
        // Barycentric blending: the three boundaries form a triangular domain.
        // Map (u, v) into barycentric coordinates (λ0, λ1, λ2).
        let l0 = 1.0 - u - v;
        let l1 = u;
        let l2 = v;
        let clamped_l0 = l0.max(0.0);

        let p0 = sample_polyline(&self.boundary_curves[0], if clamped_l0 > 0.0 { l1 / (l1 + clamped_l0).max(1e-15) } else { 1.0 });
        let p1 = sample_polyline(&self.boundary_curves[1], if l1 + l2 > 0.0 { l2 / (l1 + l2).max(1e-15) } else { 0.0 });
        let p2 = sample_polyline(&self.boundary_curves[2], if clamped_l0 + l2 > 0.0 { clamped_l0 / (clamped_l0 + l2).max(1e-15) } else { 0.0 });

        let sum = clamped_l0 + l1 + l2;
        let (w0, w1, w2) = if sum > 0.0 {
            (clamped_l0 / sum, l1 / sum, l2 / sum)
        } else {
            (1.0 / 3.0, 1.0 / 3.0, 1.0 / 3.0)
        };

        add3(add3(scale3(p0, w0), scale3(p1, w1)), scale3(p2, w2))
    }

    /// Evaluate the filling surface for 4 boundary curves.
    ///
    /// Delegates to the Coons, Stretch, or Curved formula based on `self.style`.
    fn eval_four(&self, u: f64, v: f64) -> [f64; 3] {
        // Boundaries: 0 = bottom (v=0), 1 = right (u=1),
        //             2 = top (v=1),    3 = left  (u=0).
        let c0 = &self.boundary_curves[0]; // bottom
        let c1 = &self.boundary_curves[1]; // right
        let c2 = &self.boundary_curves[2]; // top
        let c3 = &self.boundary_curves[3]; // left

        match self.style {
            FillingStyle::Coons => {
                // Classic Coons patch (transfinite interpolation).
                let lc = sample_polyline(c3, v); // left  side at v
                let rc = sample_polyline(c1, v); // right side at v
                let bc = sample_polyline(c0, u); // bottom at u
                let tc = sample_polyline(c2, u); // top    at u

                // Corners.
                let p00 = c0.first().copied().unwrap_or([0.0; 3]);
                let p10 = c0.last().copied().unwrap_or([0.0; 3]);
                let p01 = c2.first().copied().unwrap_or([0.0; 3]);
                let p11 = c2.last().copied().unwrap_or([0.0; 3]);

                // Lc(u,v) + Lv(u,v) - Lcv(u,v)
                let lc_uv = add3(scale3(lc, 1.0 - u), scale3(rc, u));
                let lv_uv = add3(scale3(bc, 1.0 - v), scale3(tc, v));
                let lcv_uv = add3(
                    add3(
                        scale3(p00, (1.0 - u) * (1.0 - v)),
                        scale3(p10, u * (1.0 - v)),
                    ),
                    add3(
                        scale3(p01, (1.0 - u) * v),
                        scale3(p11, u * v),
                    ),
                );
                sub3(add3(lc_uv, lv_uv), lcv_uv)
            }

            FillingStyle::Strech => {
                // Bilinear blend between opposite boundary pairs.
                let bc = sample_polyline(c0, u);
                let tc = sample_polyline(c2, 1.0 - u);
                let lc = sample_polyline(c3, v);
                let rc = sample_polyline(c1, 1.0 - v);
                add3(
                    add3(scale3(bc, 1.0 - v), scale3(tc, v)),
                    scale3(sub3(add3(scale3(lc, 1.0 - u), scale3(rc, u)),
                                add3(scale3(bc, 1.0 - v), scale3(tc, v))),
                           0.5),
                )
            }

            FillingStyle::Curved => {
                // Curved style: average Coons result with midpoint elevation.
                // Uses the Coons formula and adds a curvature-minimising bias
                // via a biquadratic bump.
                let lc = sample_polyline(c3, v);
                let rc = sample_polyline(c1, v);
                let bc = sample_polyline(c0, u);
                let tc = sample_polyline(c2, u);

                let p00 = c0.first().copied().unwrap_or([0.0; 3]);
                let p10 = c0.last().copied().unwrap_or([0.0; 3]);
                let p01 = c2.first().copied().unwrap_or([0.0; 3]);
                let p11 = c2.last().copied().unwrap_or([0.0; 3]);

                let lc_uv = add3(scale3(lc, 1.0 - u), scale3(rc, u));
                let lv_uv = add3(scale3(bc, 1.0 - v), scale3(tc, v));
                let lcv_uv = add3(
                    add3(
                        scale3(p00, (1.0 - u) * (1.0 - v)),
                        scale3(p10, u * (1.0 - v)),
                    ),
                    add3(
                        scale3(p01, (1.0 - u) * v),
                        scale3(p11, u * v),
                    ),
                );
                let coons = sub3(add3(lc_uv, lv_uv), lcv_uv);

                // Biquadratic bump weight: zero on all four edges, maximum at centre.
                let bump = 4.0 * u * (1.0 - u) * v * (1.0 - v);

                // Interior midpoint estimate (centroid of all four boundary midpoints).
                let mid0 = sample_polyline(c0, 0.5);
                let mid1 = sample_polyline(c1, 0.5);
                let mid2 = sample_polyline(c2, 0.5);
                let mid3 = sample_polyline(c3, 0.5);
                let interior = scale3(add3(add3(mid0, mid1), add3(mid2, mid3)), 0.25);

                add3(scale3(coons, 1.0 - bump), scale3(interior, bump))
            }
        }
    }
}

// ---------------------------------------------------------------------------
// CoonsPatch
// ---------------------------------------------------------------------------

/// Coons-patch surface defined by four ordered boundary polylines.
///
/// Implements `GeomFill_CoonsAlgPatch`-style transfinite interpolation.
/// The four sides are indexed as:
///
/// - `0` — bottom boundary  (v = 0, u varies 0 → 1)
/// - `1` — right boundary   (u = 1, v varies 0 → 1)
/// - `2` — top boundary     (v = 1, u varies 0 → 1, reversed)
/// - `3` — left boundary    (u = 0, v varies 0 → 1, reversed)
///
/// Corner compatibility (C0 closure) is not enforced automatically; the
/// caller is responsible for providing consistent boundary data.
// occt: GeomFill_CoonsAlgPatch
#[derive(Clone, Debug)]
pub struct CoonsPatch {
    /// Bottom boundary (v = 0).
    pub c0: Vec<[f64; 3]>,
    /// Right boundary (u = 1).
    pub c1: Vec<[f64; 3]>,
    /// Top boundary (v = 1).
    pub c2: Vec<[f64; 3]>,
    /// Left boundary (u = 0).
    pub c3: Vec<[f64; 3]>,
}

impl CoonsPatch {
    /// Create an empty `CoonsPatch` (all four sides are empty).
    pub fn new() -> Self {
        Self {
            c0: Vec::new(),
            c1: Vec::new(),
            c2: Vec::new(),
            c3: Vec::new(),
        }
    }

    /// Set side `i` (0..=3) to the provided polyline.
    ///
    /// - `i = 0` → bottom (v = 0)
    /// - `i = 1` → right  (u = 1)
    /// - `i = 2` → top    (v = 1)
    /// - `i = 3` → left   (u = 0)
    ///
    /// Values of `i` outside 0..=3 are silently ignored.
    pub fn set_side(&mut self, i: usize, pts: Vec<[f64; 3]>) {
        match i {
            0 => self.c0 = pts,
            1 => self.c1 = pts,
            2 => self.c2 = pts,
            3 => self.c3 = pts,
            _ => {}
        }
    }

    /// Evaluate the Coons patch at parameters `(u, v)` ∈ [0, 1]².
    ///
    /// Uses the standard transfinite interpolation formula:
    ///
    /// ```text
    /// P(u,v) = (1-u)·c3(v) + u·c1(v)
    ///        + (1-v)·c0(u) + v·c2(u)
    ///        − (1-u)(1-v)·P₀₀ − u(1-v)·P₁₀
    ///        − (1-u)v·P₀₁  − u·v·P₁₁
    /// ```
    ///
    /// where `P₀₀, P₁₀, P₀₁, P₁₁` are the four corner points.
    ///
    /// Returns `[0.0, 0.0, 0.0]` if any required boundary is empty.
    pub fn evaluate(&self, u: f64, v: f64) -> [f64; 3] {
        // Guard against missing boundaries.
        if self.c0.is_empty() || self.c1.is_empty() || self.c2.is_empty() || self.c3.is_empty() {
            return [0.0, 0.0, 0.0];
        }

        let u = u.clamp(0.0, 1.0);
        let v = v.clamp(0.0, 1.0);

        // Sample each side.
        let bottom = sample_polyline(&self.c0, u); // c0(u),  v = 0
        let right  = sample_polyline(&self.c1, v); // c1(v),  u = 1
        let top    = sample_polyline(&self.c2, u); // c2(u),  v = 1
        let left   = sample_polyline(&self.c3, v); // c3(v),  u = 0

        // Corner points.
        let p00 = self.c0.first().copied().unwrap_or([0.0; 3]); // (u=0, v=0)
        let p10 = self.c0.last().copied().unwrap_or([0.0; 3]);  // (u=1, v=0)
        let p01 = self.c2.first().copied().unwrap_or([0.0; 3]); // (u=0, v=1)
        let p11 = self.c2.last().copied().unwrap_or([0.0; 3]);  // (u=1, v=1)

        // Transfinite interpolation.
        let ruled_u  = add3(scale3(left,   1.0 - u), scale3(right, u));
        let ruled_v  = add3(scale3(bottom, 1.0 - v), scale3(top,   v));
        let bilinear = add3(
            add3(scale3(p00, (1.0 - u) * (1.0 - v)),
                 scale3(p10,        u  * (1.0 - v))),
            add3(scale3(p01, (1.0 - u) * v),
                 scale3(p11,        u  *        v)),
        );

        sub3(add3(ruled_u, ruled_v), bilinear)
    }
}

impl Default for CoonsPatch {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // Build a unit-square boundary set (bottom, right, top-reversed, left-reversed).
    fn unit_square_boundaries() -> [Vec<[f64; 3]>; 4] {
        [
            vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]], // bottom
            vec![[1.0, 0.0, 0.0], [1.0, 1.0, 0.0]], // right
            vec![[1.0, 1.0, 0.0], [0.0, 1.0, 0.0]], // top (reversed)
            vec![[0.0, 1.0, 0.0], [0.0, 0.0, 0.0]], // left (reversed)
        ]
    }

    #[test]
    fn test_filling_style_variants() {
        let _ = FillingStyle::Curved;
        let _ = FillingStyle::Strech;
        let _ = FillingStyle::Coons;
    }

    #[test]
    fn test_bspline_filling_new() {
        let fill = BSplineFilling::new(FillingStyle::Coons);
        assert_eq!(fill.boundary_curves.len(), 0);
        assert!(!fill.is_done());
    }

    #[test]
    fn test_bspline_filling_add_boundary() {
        let mut fill = BSplineFilling::new(FillingStyle::Strech);
        fill.add_boundary(vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]);
        assert_eq!(fill.boundary_curves.len(), 1);
        assert!(!fill.is_done()); // need at least 2
    }

    #[test]
    fn test_bspline_filling_is_done_two_boundaries() {
        let mut fill = BSplineFilling::new(FillingStyle::Strech);
        fill.add_boundary(vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]);
        fill.add_boundary(vec![[0.0, 1.0, 0.0], [1.0, 1.0, 0.0]]);
        assert!(fill.is_done());
    }

    #[test]
    fn test_bspline_filling_is_done_four_boundaries() {
        let mut fill = BSplineFilling::new(FillingStyle::Coons);
        let [b0, b1, b2, b3] = unit_square_boundaries();
        fill.add_boundary(b0);
        fill.add_boundary(b1);
        fill.add_boundary(b2);
        fill.add_boundary(b3);
        assert!(fill.is_done());
    }

    #[test]
    fn test_bspline_filling_build_coons_nonempty() {
        let mut fill = BSplineFilling::new(FillingStyle::Coons);
        let [b0, b1, b2, b3] = unit_square_boundaries();
        fill.add_boundary(b0);
        fill.add_boundary(b1);
        fill.add_boundary(b2);
        fill.add_boundary(b3);
        let pts = fill.build();
        assert!(!pts.is_empty());
    }

    #[test]
    fn test_bspline_filling_build_returns_empty_when_not_done() {
        let fill = BSplineFilling::new(FillingStyle::Curved);
        let pts = fill.build();
        assert!(pts.is_empty());
    }

    #[test]
    fn test_bspline_filling_build_strech() {
        let mut fill = BSplineFilling::new(FillingStyle::Strech);
        let [b0, b1, b2, b3] = unit_square_boundaries();
        fill.add_boundary(b0);
        fill.add_boundary(b1);
        fill.add_boundary(b2);
        fill.add_boundary(b3);
        let pts = fill.build();
        assert!(!pts.is_empty());
    }

    #[test]
    fn test_bspline_filling_build_curved() {
        let mut fill = BSplineFilling::new(FillingStyle::Curved);
        let [b0, b1, b2, b3] = unit_square_boundaries();
        fill.add_boundary(b0);
        fill.add_boundary(b1);
        fill.add_boundary(b2);
        fill.add_boundary(b3);
        let pts = fill.build();
        assert!(!pts.is_empty());
    }

    #[test]
    fn test_coons_patch_new() {
        let patch = CoonsPatch::new();
        assert!(patch.c0.is_empty());
        assert!(patch.c1.is_empty());
        assert!(patch.c2.is_empty());
        assert!(patch.c3.is_empty());
    }

    #[test]
    fn test_coons_patch_set_side() {
        let mut patch = CoonsPatch::new();
        patch.set_side(0, vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]);
        assert_eq!(patch.c0.len(), 2);
        patch.set_side(1, vec![[1.0, 0.0, 0.0], [1.0, 1.0, 0.0]]);
        assert_eq!(patch.c1.len(), 2);
        patch.set_side(2, vec![[0.0, 1.0, 0.0], [1.0, 1.0, 0.0]]);
        assert_eq!(patch.c2.len(), 2);
        patch.set_side(3, vec![[0.0, 0.0, 0.0], [0.0, 1.0, 0.0]]);
        assert_eq!(patch.c3.len(), 2);
        // out-of-range index is ignored
        patch.set_side(99, vec![[9.0, 9.0, 9.0]]);
    }

    #[test]
    fn test_coons_patch_evaluate_empty_returns_origin() {
        let patch = CoonsPatch::new();
        let p = patch.evaluate(0.5, 0.5);
        assert_eq!(p, [0.0, 0.0, 0.0]);
    }

    /// For a flat unit-square Coons patch the corners must reproduce exactly.
    #[test]
    fn test_coons_patch_evaluate_corners() {
        let mut patch = CoonsPatch::new();
        patch.set_side(0, vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]);
        patch.set_side(1, vec![[1.0, 0.0, 0.0], [1.0, 1.0, 0.0]]);
        patch.set_side(2, vec![[0.0, 1.0, 0.0], [1.0, 1.0, 0.0]]);
        patch.set_side(3, vec![[0.0, 0.0, 0.0], [0.0, 1.0, 0.0]]);

        let eps = 1e-12;
        let p00 = patch.evaluate(0.0, 0.0);
        assert!((p00[0]).abs() < eps && (p00[1]).abs() < eps, "corner (0,0): {:?}", p00);

        let p10 = patch.evaluate(1.0, 0.0);
        assert!((p10[0] - 1.0).abs() < eps && (p10[1]).abs() < eps, "corner (1,0): {:?}", p10);

        let p11 = patch.evaluate(1.0, 1.0);
        assert!((p11[0] - 1.0).abs() < eps && (p11[1] - 1.0).abs() < eps, "corner (1,1): {:?}", p11);

        let p01 = patch.evaluate(0.0, 1.0);
        assert!((p01[0]).abs() < eps && (p01[1] - 1.0).abs() < eps, "corner (0,1): {:?}", p01);
    }

    /// Interior point of the flat unit square must lie in [0,1]².
    #[test]
    fn test_coons_patch_evaluate_interior() {
        let mut patch = CoonsPatch::new();
        patch.set_side(0, vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]);
        patch.set_side(1, vec![[1.0, 0.0, 0.0], [1.0, 1.0, 0.0]]);
        patch.set_side(2, vec![[0.0, 1.0, 0.0], [1.0, 1.0, 0.0]]);
        patch.set_side(3, vec![[0.0, 0.0, 0.0], [0.0, 1.0, 0.0]]);

        let p = patch.evaluate(0.5, 0.5);
        assert!(p[0] >= 0.0 && p[0] <= 1.0, "x out of range: {:?}", p);
        assert!(p[1] >= 0.0 && p[1] <= 1.0, "y out of range: {:?}", p);
        assert!(p[2].abs() < 1e-12, "z should be 0: {:?}", p);
    }

    #[test]
    fn test_coons_patch_default() {
        let patch = CoonsPatch::default();
        assert!(patch.c0.is_empty());
    }

    #[test]
    fn test_sample_polyline_single_point() {
        let pts = vec![[3.0, 4.0, 5.0]];
        let p = sample_polyline(&pts, 0.5);
        assert_eq!(p, [3.0, 4.0, 5.0]);
    }

    #[test]
    fn test_sample_polyline_two_points_midpoint() {
        let pts = vec![[0.0, 0.0, 0.0], [2.0, 0.0, 0.0]];
        let p = sample_polyline(&pts, 0.5);
        let eps = 1e-12;
        assert!((p[0] - 1.0).abs() < eps, "midpoint x: {:?}", p);
    }
}
