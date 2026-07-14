//! `Bnd_B3` — axis-aligned 3D bounding box stored as center + half-size.
//!
//! Faithful Rust reproduction of OpenCascade's `Bnd_B3<RealType>`
//! (`src/FoundationClasses/TKMath/Bnd/Bnd_B3.hxx`). The OCCT class is a
//! template instantiated for `double` (`Bnd_B3d`) and `float` (`Bnd_B3f`);
//! this module provides the `double` variant [`BndB3d`] and a thin `float`
//! variant [`BndB3f`] that rounds its stored center/half-size through `f32`
//! exactly as the OCCT `Bnd_B3<float>` does.
//!
//! The box is represented by `center` and `hsize` (the half-diagonal, all
//! components non-negative when valid). A box is *void* (non-initialised) when
//! `hsize[0] < -1e-5`, matching OCCT's sentinel encoding where a void box has
//! `center = (1e30, 1e30, 1e30)` and `hsize = (-1e30, -1e30, -1e30)`.
//!
//! Builds only on [`crate::gp`] (`Pnt`, `Ax1`, `Ax3`, `Trsf`); no other kernel
//! modules and zero third-party deps.

use crate::gp::{Ax1, Ax3, Pnt, Trsf};

/// OCCT `Bnd_B3::THE_RealLast` — the large sentinel used for void boxes.
const THE_REAL_LAST: f64 = 1.0e30;

/// OCCT `gp::Resolution()` (= `1e-12`); the line/ray test uses `Resolution*100`.
const GP_RESOLUTION: f64 = 1.0e-12;

/// OCCT `RealLast()` — largest finite double, used as `±∞` parameter bounds in
/// the line-intersection interval arithmetic.
const REAL_LAST: f64 = f64::MAX;

// occt: Bnd_B3
/// Axis-aligned 3D bounding box (`Bnd_B3d` = `Bnd_B3<double>`), stored as a
/// `center` and a `hsize` half-diagonal.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BndB3d {
    center: [f64; 3],
    hsize: [f64; 3],
}

impl Default for BndB3d {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl BndB3d {
    /// Empty constructor — produces a *void* box.
    #[inline]
    pub fn new() -> Self {
        Self {
            center: [THE_REAL_LAST, THE_REAL_LAST, THE_REAL_LAST],
            hsize: [-THE_REAL_LAST, -THE_REAL_LAST, -THE_REAL_LAST],
        }
    }

    /// Constructor from a center and a half-size, taking [`Pnt`] (`gp_XYZ`).
    #[inline]
    pub fn from_center_hsize(center: Pnt, hsize: Pnt) -> Self {
        Self {
            center: [center.x, center.y, center.z],
            hsize: [hsize.x, hsize.y, hsize.z],
        }
    }

    /// Constructor from a center and a half-size, taking arrays.
    #[inline]
    pub fn from_center_hsize_arr(center: [f64; 3], hsize: [f64; 3]) -> Self {
        Self { center, hsize }
    }

    /// Returns `true` if the box is void (non-initialised).
    #[inline]
    pub fn is_void(&self) -> bool {
        self.hsize[0] < -1.0e-5
    }

    /// Reset the box data, making it void.
    #[inline]
    pub fn clear(&mut self) {
        self.center = [THE_REAL_LAST, THE_REAL_LAST, THE_REAL_LAST];
        self.hsize = [-THE_REAL_LAST, -THE_REAL_LAST, -THE_REAL_LAST];
    }

    /// Update the box by a point (`gp_XYZ` / `gp_Pnt`).
    pub fn add(&mut self, p: Pnt) {
        if self.is_void() {
            self.center = [p.x, p.y, p.z];
            self.hsize = [0.0, 0.0, 0.0];
        } else {
            let diff = [
                p.x - self.center[0],
                p.y - self.center[1],
                p.z - self.center[2],
            ];
            for i in 0..3 {
                if diff[i] > self.hsize[i] {
                    let shift = (diff[i] - self.hsize[i]) / 2.0;
                    self.center[i] += shift;
                    self.hsize[i] += shift;
                } else if diff[i] < -self.hsize[i] {
                    let shift = (diff[i] + self.hsize[i]) / 2.0;
                    self.center[i] += shift;
                    self.hsize[i] -= shift;
                }
            }
        }
    }

    /// Update the box by another box.
    pub fn add_box(&mut self, other: &BndB3d) {
        if !other.is_void() {
            self.add(other.corner_min());
            self.add(other.corner_max());
        }
    }

    /// Query the lower corner: `Center - HSize`.
    #[inline]
    pub fn corner_min(&self) -> Pnt {
        Pnt::new(
            self.center[0] - self.hsize[0],
            self.center[1] - self.hsize[1],
            self.center[2] - self.hsize[2],
        )
    }

    /// Query the upper corner: `Center + HSize`.
    #[inline]
    pub fn corner_max(&self) -> Pnt {
        Pnt::new(
            self.center[0] + self.hsize[0],
            self.center[1] + self.hsize[1],
            self.center[2] + self.hsize[2],
        )
    }

    /// Query the square diagonal: `4 * (hx^2 + hy^2 + hz^2)`.
    #[inline]
    pub fn square_extent(&self) -> f64 {
        4.0 * (self.hsize[0] * self.hsize[0]
            + self.hsize[1] * self.hsize[1]
            + self.hsize[2] * self.hsize[2])
    }

    /// Extend the box by the absolute value of `diff` in every direction.
    #[inline]
    pub fn enlarge(&mut self, diff: f64) {
        let d = diff.abs();
        self.hsize[0] += d;
        self.hsize[1] += d;
        self.hsize[2] += d;
    }

    /// Limit the box by the internals of `other`. Returns `true` if the
    /// limitation takes place, `false` if the boxes do not intersect.
    pub fn limit(&mut self, other: &BndB3d) -> bool {
        let diff_c = [
            other.center[0] - self.center[0],
            other.center[1] - self.center[1],
            other.center[2] - self.center[2],
        ];
        let sum_h = [
            other.hsize[0] + self.hsize[0],
            other.hsize[1] + self.hsize[1],
            other.hsize[2] + self.hsize[2],
        ];
        // check the condition IsOut
        if compare_dist(&sum_h, &diff_c) {
            return false;
        }
        let diff_h = [
            other.hsize[0] - self.hsize[0],
            other.hsize[1] - self.hsize[1],
            other.hsize[2] - self.hsize[2],
        ];
        for i in 0..3 {
            if diff_c[i] - diff_h[i] > 0.0 {
                let shift = (diff_c[i] - diff_h[i]) / 2.0; // positive
                self.center[i] += shift;
                self.hsize[i] -= shift;
            } else if diff_c[i] + diff_h[i] < 0.0 {
                let shift = (diff_c[i] + diff_h[i]) / 2.0; // negative
                self.center[i] += shift;
                self.hsize[i] += shift;
            }
        }
        true
    }

    /// Transform the bounding box. The resulting box is larger if `trsf`
    /// contains a rotation.
    ///
    /// OCCT distinguishes a simple form (Identity/Translation/PntMirror/Scale)
    /// from the rotational form. With the IronStream [`Trsf`] the full linear
    /// part `m` already carries any scale, so `center' = trsf·center` and
    /// `hsize'[i] = Σ_j |m[i][j]|·hsize[j]` reproduces both branches exactly:
    /// for the simple diagonal forms this is `|scale|·hsize`, and for the
    /// rotational form it is the OCCT `aScaleAbs·|aMat|·HSize` expression.
    #[must_use]
    pub fn transformed(&self, trsf: &Trsf) -> BndB3d {
        let c = trsf.apply_point(Pnt::new(self.center[0], self.center[1], self.center[2]));
        let m = &trsf.m;
        let mut hsize = [0.0_f64; 3];
        for i in 0..3 {
            hsize[i] = m[i][0].abs() * self.hsize[0]
                + m[i][1].abs() * self.hsize[1]
                + m[i][2].abs() * self.hsize[2];
        }
        BndB3d {
            center: [c.x, c.y, c.z],
            hsize,
        }
    }

    /// Check the given point for inclusion in the box. Returns `true` if the
    /// point is outside.
    #[inline]
    pub fn is_out_point(&self, p: Pnt) -> bool {
        (p.x - self.center[0]).abs() > self.hsize[0]
            || (p.y - self.center[1]).abs() > self.hsize[1]
            || (p.z - self.center[2]).abs() > self.hsize[2]
    }

    /// Check a sphere for intersection with the box. Returns `true` if there is
    /// no intersection. If `is_sphere_hollow` is `true`, a box completely
    /// inside the (hollow) sphere is reported as *out* (no intersection).
    pub fn is_out_sphere(&self, sphere_center: Pnt, radius: f64, is_sphere_hollow: bool) -> bool {
        let center = [sphere_center.x, sphere_center.y, sphere_center.z];
        if !is_sphere_hollow {
            // vector from the center of the sphere to the nearest box face
            let dist = [
                (center[0] - self.center[0]).abs() - self.hsize[0],
                (center[1] - self.center[1]).abs() - self.hsize[1],
                (center[2] - self.center[2]).abs() - self.hsize[2],
            ];
            let mut d = 0.0;
            if dist[0] > 0.0 {
                d = dist[0] * dist[0];
            }
            if dist[1] > 0.0 {
                d += dist[1] * dist[1];
            }
            if dist[2] > 0.0 {
                d += dist[2] * dist[2];
            }
            d > radius * radius
        } else {
            let dist_c = [
                (center[0] - self.center[0]).abs(),
                (center[1] - self.center[1]).abs(),
                (center[2] - self.center[2]).abs(),
            ];
            // vector from the center of the sphere to the nearest box face
            let mut dist = [
                dist_c[0] - self.hsize[0],
                dist_c[1] - self.hsize[1],
                dist_c[2] - self.hsize[2],
            ];
            let mut d = 0.0;
            if dist[0] > 0.0 {
                d = dist[0] * dist[0];
            }
            if dist[1] > 0.0 {
                d += dist[1] * dist[1];
            }
            if dist[2] > 0.0 {
                d += dist[2] * dist[2];
            }
            let mut result = true;
            if d < radius * radius {
                // the box intersects the solid sphere; check if it is completely
                // inside the sphere (in such case return is_out == true)
                dist[0] = dist_c[0] + self.hsize[0];
                dist[1] = dist_c[1] + self.hsize[1];
                dist[2] = dist_c[2] + self.hsize[2];
                if dist[0] * dist[0] + dist[1] * dist[1] + dist[2] * dist[2] > radius * radius {
                    result = false;
                }
            }
            result
        }
    }

    /// Check the given box for intersection with the current box. Returns
    /// `true` if there is no intersection.
    #[inline]
    pub fn is_out_box(&self, other: &BndB3d) -> bool {
        (other.center[0] - self.center[0]).abs() > other.hsize[0] + self.hsize[0]
            || (other.center[1] - self.center[1]).abs() > other.hsize[1] + self.hsize[1]
            || (other.center[2] - self.center[2]).abs() > other.hsize[2] + self.hsize[2]
    }

    /// Check the given box, oriented by `trsf`, for intersection with the
    /// current box. Returns `true` if there is no intersection.
    pub fn is_out_box_trsf(&self, other: &BndB3d, trsf: &Trsf) -> bool {
        if trsf_is_simple(trsf) {
            let (scale, scale_abs) = trsf_scale(trsf);
            let tp = trsf.t;
            return (other.center[0] * scale + tp[0] - self.center[0]).abs()
                > other.hsize[0] * scale_abs + self.hsize[0]
                || (other.center[1] * scale + tp[1] - self.center[1]).abs()
                    > other.hsize[1] * scale_abs + self.hsize[1]
                || (other.center[2] * scale + tp[2] - self.center[2]).abs()
                    > other.hsize[2] * scale_abs + self.hsize[2];
        }

        // `other` is rotated/scaled/translated; the IronStream matrix `m`
        // already folds the scale into the rotation, so the OCCT
        // `aScaleAbs * aMatAbs` terms equal `|m|`. `mat[i][j]` therefore plays
        // the role of OCCT's `aScaleAbs * aMat[3*i + j]`, and the bare
        // orthonormal `aMat` used in the second (projection) test is `m`
        // divided by the signed scale.
        let (scale, scale_abs) = trsf_scale(trsf);
        let m = &trsf.m;
        let center = trsf.apply_point(Pnt::new(other.center[0], other.center[1], other.center[2]));
        let dist = [
            center.x - self.center[0],
            center.y - self.center[1],
            center.z - self.center[2],
        ];
        // First (axis-aligned) separating-axis test of `this` against the
        // transformed `other`'s enclosing box.
        if dist[0].abs()
            > m[0][0].abs() * other.hsize[0]
                + m[0][1].abs() * other.hsize[1]
                + m[0][2].abs() * other.hsize[2]
                + self.hsize[0]
            || dist[1].abs()
                > m[1][0].abs() * other.hsize[0]
                    + m[1][1].abs() * other.hsize[1]
                    + m[1][2].abs() * other.hsize[2]
                    + self.hsize[1]
            || dist[2].abs()
                > m[2][0].abs() * other.hsize[0]
                    + m[2][1].abs() * other.hsize[1]
                    + m[2][2].abs() * other.hsize[2]
                    + self.hsize[2]
        {
            return true;
        }
        // Second test: project `this` onto the transformed `other`'s axes.
        // `aMat[col][row]` orthonormal = m[row][col] / scale. The OCCT terms
        // `theBox.HSize[k] * aScaleAbs` and `aMatAbs * myHSize` use this.
        let inv = if scale.abs() > 0.0 { 1.0 / scale } else { 0.0 };
        // orthonormal rotation columns: o[j][i] = m[i][j] / scale
        let o = |i: usize, j: usize| m[i][j] * inv;
        for j in 0..3 {
            let proj = (o(0, j) * dist[0] + o(1, j) * dist[1] + o(2, j) * dist[2]).abs();
            let bound = other.hsize[j] * scale_abs
                + (o(0, j).abs() * self.hsize[0]
                    + o(1, j).abs() * self.hsize[1]
                    + o(2, j).abs() * self.hsize[2]);
            if proj > bound {
                return true;
            }
        }
        false
    }

    /// Check the given line (`gp_Ax1`) for intersection with the box. Returns
    /// `true` if there is no intersection. `is_ray == true` restricts the test
    /// to the positive half-line; `overthickness` enlarges the box (may be
    /// negative).
    pub fn is_out_line(&self, line: &Ax1, is_ray: bool, overthickness: f64) -> bool {
        let res = GP_RESOLUTION * 100.0;
        if self.is_void() {
            return true;
        }
        let dir = line.direction; // already unit in IronStream Ax1
        let loc = line.location;
        let mut inter0 = [-REAL_LAST, REAL_LAST];
        let mut inter1 = [-REAL_LAST, REAL_LAST];
        let diff = [
            self.center[0] - loc.x,
            self.center[1] - loc.y,
            self.center[2] - loc.z,
        ];

        // Find the parameter interval in X dimension
        let mut hsize = self.hsize[0] + overthickness;
        if dir.x > res {
            inter0[0] = (diff[0] - hsize) / dir.x;
            inter0[1] = (diff[0] + hsize) / dir.x;
        } else if dir.x < -res {
            inter0[0] = (diff[0] + hsize) / dir.x;
            inter0[1] = (diff[0] - hsize) / dir.x;
        } else if diff[0].abs() > hsize {
            // line is orthogonal to OX; outside the slab
            return true;
        }

        // Find the parameter interval in Y dimension
        hsize = self.hsize[1] + overthickness;
        if dir.y > res {
            inter1[0] = (diff[1] - hsize) / dir.y;
            inter1[1] = (diff[1] + hsize) / dir.y;
        } else if dir.y < -res {
            inter1[0] = (diff[1] + hsize) / dir.y;
            inter1[1] = (diff[1] - hsize) / dir.y;
        } else if diff[1].abs() > hsize {
            return true;
        }

        // Intersect Y-interval with X-interval
        if inter0[0] > (inter1[1] + res) || inter0[1] < (inter1[0] - res) {
            return true;
        }
        if inter1[0] > inter0[0] {
            inter0[0] = inter1[0];
        }
        if inter1[1] < inter0[1] {
            inter0[1] = inter1[1];
        }
        if is_ray && inter0[1] < -res {
            return true;
        }

        // Find the parameter interval in Z dimension
        hsize = self.hsize[2] + overthickness;
        if dir.z > res {
            inter1[0] = (diff[2] - hsize) / dir.z;
            inter1[1] = (diff[2] + hsize) / dir.z;
        } else if dir.z < -res {
            inter1[0] = (diff[2] + hsize) / dir.z;
            inter1[1] = (diff[2] - hsize) / dir.z;
        } else {
            // line is orthogonal to OZ; test inclusion in box limits
            return diff[2].abs() > hsize;
        }
        if is_ray && inter1[1] < -res {
            return true;
        }

        inter0[0] > (inter1[1] + res) || inter0[1] < (inter1[0] - res)
    }

    /// Check the given plane (`gp_Ax3`) for intersection with the box. Returns
    /// `true` if there is no intersection.
    pub fn is_out_plane(&self, plane: &Ax3) -> bool {
        if self.is_void() {
            return true;
        }
        let origin = plane.location;
        let dir = plane.z_dir;
        let box_center = Pnt::new(self.center[0], self.center[1], self.center[2]);
        let dist0 = (box_center - origin).dot(dir);
        // proj of HSize on dir
        let dist1 =
            self.hsize[0] * dir.x.abs() + self.hsize[1] * dir.y.abs() + self.hsize[2] * dir.z.abs();
        (dist0 + dist1) * (dist0 - dist1) > 0.0
    }

    /// Check that this box is fully inside `other`. Returns `true` if so.
    #[inline]
    pub fn is_in_box(&self, other: &BndB3d) -> bool {
        (other.center[0] - self.center[0]).abs() < other.hsize[0] - self.hsize[0]
            && (other.center[1] - self.center[1]).abs() < other.hsize[1] - self.hsize[1]
            && (other.center[2] - self.center[2]).abs() < other.hsize[2] - self.hsize[2]
    }

    /// Check that this box is fully inside `other` transformed by `trsf`.
    pub fn is_in_box_trsf(&self, other: &BndB3d, trsf: &Trsf) -> bool {
        if trsf_is_simple(trsf) {
            let (scale, scale_abs) = trsf_scale(trsf);
            let tp = trsf.t;
            return (other.center[0] * scale + tp[0] - self.center[0]).abs()
                < other.hsize[0] * scale_abs - self.hsize[0]
                && (other.center[1] * scale + tp[1] - self.center[1]).abs()
                    < other.hsize[1] * scale_abs - self.hsize[1]
                && (other.center[2] * scale + tp[2] - self.center[2]).abs()
                    < other.hsize[2] * scale_abs - self.hsize[2];
        }
        let (scale, scale_abs) = trsf_scale(trsf);
        let m = &trsf.m;
        let center = trsf.apply_point(Pnt::new(other.center[0], other.center[1], other.center[2]));
        let dist = [
            center.x - self.center[0],
            center.y - self.center[1],
            center.z - self.center[2],
        ];
        let inv = if scale.abs() > 0.0 { 1.0 / scale } else { 0.0 };
        let o = |i: usize, j: usize| m[i][j] * inv;
        for j in 0..3 {
            let proj = (o(0, j) * dist[0] + o(1, j) * dist[1] + o(2, j) * dist[2]).abs();
            let bound = other.hsize[j] * scale_abs
                - (o(0, j).abs() * self.hsize[0]
                    + o(1, j).abs() * self.hsize[1]
                    + o(2, j).abs() * self.hsize[2]);
            if proj >= bound {
                return false;
            }
        }
        true
    }

    /// Set the center coordinates.
    #[inline]
    pub fn set_center(&mut self, center: Pnt) {
        self.center = [center.x, center.y, center.z];
    }

    /// Set the center coordinates from an array.
    #[inline]
    pub fn set_center_arr(&mut self, center: [f64; 3]) {
        self.center = center;
    }

    /// Set the HSize (half-diagonal). All components must be non-negative.
    #[inline]
    pub fn set_hsize(&mut self, hsize: Pnt) {
        self.hsize = [hsize.x, hsize.y, hsize.z];
    }

    /// Set the HSize from an array.
    #[inline]
    pub fn set_hsize_arr(&mut self, hsize: [f64; 3]) {
        self.hsize = hsize;
    }

    /// Get the center coordinates.
    #[inline]
    pub fn center(&self) -> [f64; 3] {
        self.center
    }

    /// Get the HSize (half-diagonal) coordinates.
    #[inline]
    pub fn hsize(&self) -> [f64; 3] {
        self.hsize
    }
}

/// OCCT `Bnd_B3::compareDist` — the `IsOut`-style separating-axis predicate.
#[inline]
fn compare_dist(hsize: &[f64; 3], dist: &[f64; 3]) -> bool {
    dist[0].abs() > hsize[0] || dist[1].abs() > hsize[1] || dist[2].abs() > hsize[2]
}

/// Whether `trsf`'s linear part is a uniform scale `s·I` (covers OCCT's
/// `gp_Identity`, `gp_Translation`, `gp_Scale` and `gp_PntMirror` forms, where
/// the linear part is diagonal with equal entries). Anything with off-diagonal
/// terms (i.e. a rotation) is *not* simple.
fn trsf_is_simple(trsf: &Trsf) -> bool {
    let m = &trsf.m;
    let s = m[0][0];
    const T: f64 = 1.0e-12;
    (m[1][1] - s).abs() <= T
        && (m[2][2] - s).abs() <= T
        && m[0][1].abs() <= T
        && m[0][2].abs() <= T
        && m[1][0].abs() <= T
        && m[1][2].abs() <= T
        && m[2][0].abs() <= T
        && m[2][1].abs() <= T
}

/// OCCT `gp_Trsf::ScaleFactor` (signed) and its absolute value. The signed
/// scale is `cbrt(det)` so that a `PntMirror` (det < 0) yields a negative
/// factor exactly as OCCT does.
fn trsf_scale(trsf: &Trsf) -> (f64, f64) {
    let det = trsf.linear_det();
    let scale = det.cbrt(); // cbrt preserves sign
    (scale, scale.abs())
}

// occt-note: Bnd_B3<float>
/// Single-precision variant (`Bnd_B3f` = `Bnd_B3<float>`). Center and half-size
/// are rounded through `f32` on every store, reproducing the precision loss of
/// the OCCT `float` instantiation; all queries run in `f64`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BndB3f {
    inner: BndB3d,
}

impl Default for BndB3f {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl BndB3f {
    #[inline]
    pub fn new() -> Self {
        Self {
            inner: BndB3d::new(),
        }
    }

    /// Constructor from center and half-size, rounding components to `f32`.
    #[inline]
    pub fn from_center_hsize(center: Pnt, hsize: Pnt) -> Self {
        Self {
            inner: BndB3d {
                center: round_f32(center),
                hsize: round_f32(hsize),
            },
        }
    }

    #[inline]
    pub fn is_void(&self) -> bool {
        self.inner.is_void()
    }

    #[inline]
    pub fn corner_min(&self) -> Pnt {
        self.inner.corner_min()
    }

    #[inline]
    pub fn corner_max(&self) -> Pnt {
        self.inner.corner_max()
    }

    #[inline]
    pub fn center(&self) -> [f64; 3] {
        self.inner.center()
    }

    #[inline]
    pub fn hsize(&self) -> [f64; 3] {
        self.inner.hsize()
    }
}

#[inline]
fn round_f32(p: Pnt) -> [f64; 3] {
    [p.x as f32 as f64, p.y as f32 as f64, p.z as f32 as f64]
}
