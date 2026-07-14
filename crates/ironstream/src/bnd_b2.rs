//! `Bnd_B2` — axis-aligned 2D bounding box stored as center + half-size.
//!
//! Faithful Rust reproduction of OpenCascade's `Bnd_B2<RealType>`
//! (`src/FoundationClasses/TKMath/Bnd/Bnd_B2.hxx`). The OCCT class is a
//! template instantiated for `double` (`Bnd_B2d`) and `float` (`Bnd_B2f`);
//! this module provides the `double` variant [`BndB2d`] and a thin `float`
//! variant [`BndB2f`] that rounds its stored center/half-size through `f32`
//! exactly as the OCCT `Bnd_B2<float>` does.
//!
//! The box is represented by `center` and `hsize` (the half-diagonal, all
//! components non-negative when valid). A box is *void* (non-initialised) when
//! `hsize[0] < -1e-5`, matching OCCT's sentinel encoding where a void box has
//! `center = (1e30, 1e30)` and `hsize = (-1e30, -1e30)`.
//!
//! Builds only on [`crate::gp2d`] (`Pnt2d`, `Vec2d`, `Ax2d`, `Trsf2d`); no
//! other kernel modules and zero third-party deps.

use crate::gp2d::{Ax2d, Pnt2d, Trsf2d};

/// OCCT `Bnd_B2::THE_RealLast` — the large sentinel used for void boxes.
const THE_REAL_LAST: f64 = 1.0e30;

// occt: Bnd_B2
/// Axis-aligned 2D bounding box (`Bnd_B2d` = `Bnd_B2<double>`), stored as a
/// `center` and a `hsize` half-diagonal.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BndB2d {
    center: [f64; 2],
    hsize: [f64; 2],
}

impl Default for BndB2d {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl BndB2d {
    /// Empty constructor — produces a *void* box.
    #[inline]
    pub fn new() -> Self {
        Self {
            center: [THE_REAL_LAST, THE_REAL_LAST],
            hsize: [-THE_REAL_LAST, -THE_REAL_LAST],
        }
    }

    /// Constructor from a center and a half-size, taking [`Pnt2d`] (`gp_XY`).
    #[inline]
    pub fn from_center_hsize(center: Pnt2d, hsize: Pnt2d) -> Self {
        Self {
            center: [center.x, center.y],
            hsize: [hsize.x, hsize.y],
        }
    }

    /// Constructor from a center and a half-size, taking arrays.
    #[inline]
    pub fn from_center_hsize_arr(center: [f64; 2], hsize: [f64; 2]) -> Self {
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
        self.center = [THE_REAL_LAST, THE_REAL_LAST];
        self.hsize = [-THE_REAL_LAST, -THE_REAL_LAST];
    }

    /// Update the box by a point (`gp_XY`).
    pub fn add(&mut self, p: Pnt2d) {
        if self.is_void() {
            self.center = [p.x, p.y];
            self.hsize = [0.0, 0.0];
        } else {
            let diff = [p.x - self.center[0], p.y - self.center[1]];
            for i in 0..2 {
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

    /// Update the box by a point (`gp_Pnt2d`). Identical to [`Self::add`]; OCCT
    /// overloads `Add(gp_Pnt2d)` to forward to `Add(thePnt.XY())`.
    #[inline]
    pub fn add_pnt(&mut self, p: Pnt2d) {
        self.add(p);
    }

    /// Update the box by another box.
    pub fn add_box(&mut self, other: &BndB2d) {
        if !other.is_void() {
            self.add(other.corner_min());
            self.add(other.corner_max());
        }
    }

    /// Query the lower corner: `Center - HSize`.
    #[inline]
    pub fn corner_min(&self) -> Pnt2d {
        Pnt2d::new(self.center[0] - self.hsize[0], self.center[1] - self.hsize[1])
    }

    /// Query the upper corner: `Center + HSize`.
    #[inline]
    pub fn corner_max(&self) -> Pnt2d {
        Pnt2d::new(self.center[0] + self.hsize[0], self.center[1] + self.hsize[1])
    }

    /// Query the square diagonal: `4 * (hx^2 + hy^2)`.
    #[inline]
    pub fn square_extent(&self) -> f64 {
        4.0 * (self.hsize[0] * self.hsize[0] + self.hsize[1] * self.hsize[1])
    }

    /// Extend the box by the absolute value of `diff` in every direction.
    #[inline]
    pub fn enlarge(&mut self, diff: f64) {
        let d = diff.abs();
        self.hsize[0] += d;
        self.hsize[1] += d;
    }

    /// Limit the box by the internals of `other`. Returns `true` if the
    /// limitation takes place, `false` if the boxes do not intersect.
    pub fn limit(&mut self, other: &BndB2d) -> bool {
        let diff_c = [
            other.center[0] - self.center[0],
            other.center[1] - self.center[1],
        ];
        let sum_h = [
            other.hsize[0] + self.hsize[0],
            other.hsize[1] + self.hsize[1],
        ];
        // check the condition IsOut
        if compare_dist(&sum_h, &diff_c) {
            return false;
        }
        let diff_h = [
            other.hsize[0] - self.hsize[0],
            other.hsize[1] - self.hsize[1],
        ];
        for i in 0..2 {
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
    /// from the rotational form. With the IronStream [`Trsf2d`] the full linear
    /// part `m` already carries any scale, so `center' = trsf·center` and
    /// `hsize'[i] = Σ_j |m[i][j]|·hsize[j]` reproduces both branches exactly:
    /// for the simple diagonal forms this is `|scale|·hsize`, and for the
    /// rotational form it is the OCCT `aScaleAbs·|aMat|·HSize` expression.
    #[must_use]
    pub fn transformed(&self, trsf: &Trsf2d) -> BndB2d {
        let c = trsf.apply(Pnt2d::new(self.center[0], self.center[1]));
        let m = &trsf.m;
        let mut hsize = [0.0_f64; 2];
        for i in 0..2 {
            hsize[i] = m[i][0].abs() * self.hsize[0] + m[i][1].abs() * self.hsize[1];
        }
        BndB2d {
            center: [c.x, c.y],
            hsize,
        }
    }

    /// Check the given point for inclusion in the box. Returns `true` if the
    /// point is outside.
    #[inline]
    pub fn is_out_point(&self, p: Pnt2d) -> bool {
        (p.x - self.center[0]).abs() > self.hsize[0]
            || (p.y - self.center[1]).abs() > self.hsize[1]
    }

    /// Check a circle for intersection with the box. Returns `true` if there is
    /// no intersection. If `is_circle_hollow` is `true`, a box completely
    /// inside the (hollow) circle is reported as *out* (no intersection).
    pub fn is_out_circle(&self, circle_center: Pnt2d, radius: f64, is_circle_hollow: bool) -> bool {
        let center = [circle_center.x, circle_center.y];
        if !is_circle_hollow {
            // vector from the center of the circle to the nearest box face
            let dist = [
                (center[0] - self.center[0]).abs() - self.hsize[0],
                (center[1] - self.center[1]).abs() - self.hsize[1],
            ];
            let mut d = 0.0;
            if dist[0] > 0.0 {
                d = dist[0] * dist[0];
            }
            if dist[1] > 0.0 {
                d += dist[1] * dist[1];
            }
            d > radius * radius
        } else {
            let dist_c = [
                (center[0] - self.center[0]).abs(),
                (center[1] - self.center[1]).abs(),
            ];
            // vector from the center of the circle to the nearest box face
            let mut dist = [dist_c[0] - self.hsize[0], dist_c[1] - self.hsize[1]];
            let mut d = 0.0;
            if dist[0] > 0.0 {
                d = dist[0] * dist[0];
            }
            if dist[1] > 0.0 {
                d += dist[1] * dist[1];
            }
            let mut result = true;
            if d < radius * radius {
                // the box intersects the solid circle; check if it is completely
                // inside the circle (in such case return is_out == true)
                dist[0] = dist_c[0] + self.hsize[0];
                dist[1] = dist_c[1] + self.hsize[1];
                if dist[0] * dist[0] + dist[1] * dist[1] > radius * radius {
                    result = false;
                }
            }
            result
        }
    }

    /// Check the given box for intersection with the current box. Returns
    /// `true` if there is no intersection.
    #[inline]
    pub fn is_out_box(&self, other: &BndB2d) -> bool {
        (other.center[0] - self.center[0]).abs() > other.hsize[0] + self.hsize[0]
            || (other.center[1] - self.center[1]).abs() > other.hsize[1] + self.hsize[1]
    }

    /// Check the given box, oriented by `trsf`, for intersection with the
    /// current box. Returns `true` if there is no intersection.
    pub fn is_out_box_trsf(&self, other: &BndB2d, trsf: &Trsf2d) -> bool {
        if trsf_is_simple(trsf) {
            let (scale, scale_abs) = trsf_scale(trsf);
            let tp = trsf.t;
            return (other.center[0] * scale + tp[0] - self.center[0]).abs()
                > other.hsize[0] * scale_abs + self.hsize[0]
                || (other.center[1] * scale + tp[1] - self.center[1]).abs()
                    > other.hsize[1] * scale_abs + self.hsize[1];
        }

        // `other` is rotated/scaled/translated. The IronStream matrix `m`
        // already folds the scale into the rotation, so OCCT's
        // `aScaleAbs * aMat[...]` terms equal `|m|`. `m[i][j]` therefore plays
        // the role of OCCT's `aScaleAbs * aMat[2*i + j]`, and the bare
        // orthonormal `aMat` used in the projection test is `m` divided by the
        // signed scale.
        let (scale, scale_abs) = trsf_scale(trsf);
        let m = &trsf.m;
        let center = trsf.apply(Pnt2d::new(other.center[0], other.center[1]));
        let dist = [center.x - self.center[0], center.y - self.center[1]];
        // First (axis-aligned) separating-axis test of `this` against the
        // transformed `other`'s enclosing box.
        if dist[0].abs()
            > m[0][0].abs() * other.hsize[0] + m[0][1].abs() * other.hsize[1] + self.hsize[0]
            || dist[1].abs()
                > m[1][0].abs() * other.hsize[0] + m[1][1].abs() * other.hsize[1] + self.hsize[1]
        {
            return true;
        }
        // Second test: project `this` onto the transformed `other`'s axes.
        let inv = if scale.abs() > 0.0 { 1.0 / scale } else { 0.0 };
        // orthonormal rotation: o(i,j) = m[i][j] / scale
        let o = |i: usize, j: usize| m[i][j] * inv;
        for j in 0..2 {
            let proj = (o(0, j) * dist[0] + o(1, j) * dist[1]).abs();
            let bound = other.hsize[j] * scale_abs
                + (o(0, j).abs() * self.hsize[0] + o(1, j).abs() * self.hsize[1]);
            if proj > bound {
                return true;
            }
        }
        false
    }

    /// Check the given line (`gp_Ax2d`) for intersection with the box. Returns
    /// `true` if there is no intersection.
    pub fn is_out_line(&self, line: &Ax2d) -> bool {
        if self.is_void() {
            return true;
        }
        // Intersect the line containing the segment.
        // aProd[0] = dir ^ (center - location)  (2D cross product)
        let dir = line.direction; // already unit in IronStream Ax2d
        let loc = line.location;
        let rel = Pnt2d::new(self.center[0] - loc.x, self.center[1] - loc.y);
        let prod0 = dir.cross(rel);
        let prod1 = dir.x * self.hsize[1];
        let prod2 = dir.y * self.hsize[0];
        prod0.abs() > prod1.abs() + prod2.abs()
    }

    /// Check the segment `[p0, p1]` for intersection with the box. Returns
    /// `true` if there is no intersection.
    pub fn is_out_segment(&self, p0: Pnt2d, p1: Pnt2d) -> bool {
        if self.is_void() {
            return true;
        }
        // Intersect the line containing the segment.
        let seg_delta = p1 - p0;
        let rel = Pnt2d::new(self.center[0] - p0.x, self.center[1] - p0.y);
        let prod0 = seg_delta.cross(rel);
        let prod1 = seg_delta.x * self.hsize[1];
        let prod2 = seg_delta.y * self.hsize[0];
        if prod0.abs() < prod1.abs() + prod2.abs() {
            // Intersection with line detected; check the segment as a bounding box
            let h_seg = Pnt2d::new(0.5 * seg_delta.x, 0.5 * seg_delta.y);
            let h_seg_abs = Pnt2d::new(h_seg.x.abs(), h_seg.y.abs());
            let hsize = Pnt2d::new(self.hsize[0] + h_seg_abs.x, self.hsize[1] + h_seg_abs.y);
            let d = Pnt2d::new(
                p0.x + h_seg.x - self.center[0],
                p0.y + h_seg.y - self.center[1],
            );
            return d.x.abs() > hsize.x || d.y.abs() > hsize.y;
        }
        true
    }

    /// Check that this box is fully inside `other`. Returns `true` if so.
    #[inline]
    pub fn is_in_box(&self, other: &BndB2d) -> bool {
        (other.center[0] - self.center[0]).abs() < other.hsize[0] - self.hsize[0]
            && (other.center[1] - self.center[1]).abs() < other.hsize[1] - self.hsize[1]
    }

    /// Check that this box is fully inside `other` transformed by `trsf`.
    pub fn is_in_box_trsf(&self, other: &BndB2d, trsf: &Trsf2d) -> bool {
        if trsf_is_simple(trsf) {
            let (scale, scale_abs) = trsf_scale(trsf);
            let tp = trsf.t;
            return (other.center[0] * scale + tp[0] - self.center[0]).abs()
                < other.hsize[0] * scale_abs - self.hsize[0]
                && (other.center[1] * scale + tp[1] - self.center[1]).abs()
                    < other.hsize[1] * scale_abs - self.hsize[1];
        }
        let (_, scale_abs) = trsf_scale(trsf);
        let m = &trsf.m;
        let center = trsf.apply(Pnt2d::new(other.center[0], other.center[1]));
        let dist = [center.x - self.center[0], center.y - self.center[1]];
        // OCCT's IsIn(rotational) uses the raw matrix terms aMat[k] (= m[i][j])
        // against theBox.HSize * aScaleAbs and aMatAbs * myHSize.
        // First projection axis (OCCT aMat[0], aMat[2]):
        if (m[0][0] * dist[0] + m[1][0] * dist[1]).abs()
            < other.hsize[0] * scale_abs
                - (m[0][0].abs() * self.hsize[0] + m[1][0].abs() * self.hsize[1])
            && (m[0][1] * dist[0] + m[1][1] * dist[1]).abs()
                < other.hsize[1] * scale_abs
                    - (m[0][1].abs() * self.hsize[0] + m[1][1].abs() * self.hsize[1])
        {
            return true;
        }
        false
    }

    /// Set the center coordinates.
    #[inline]
    pub fn set_center(&mut self, center: Pnt2d) {
        self.center = [center.x, center.y];
    }

    /// Set the center coordinates from an array.
    #[inline]
    pub fn set_center_arr(&mut self, center: [f64; 2]) {
        self.center = center;
    }

    /// Set the HSize (half-diagonal). All components must be non-negative.
    #[inline]
    pub fn set_hsize(&mut self, hsize: Pnt2d) {
        self.hsize = [hsize.x, hsize.y];
    }

    /// Set the HSize from an array.
    #[inline]
    pub fn set_hsize_arr(&mut self, hsize: [f64; 2]) {
        self.hsize = hsize;
    }

    /// Get the center coordinates.
    #[inline]
    pub fn center(&self) -> [f64; 2] {
        self.center
    }

    /// Get the HSize (half-diagonal) coordinates.
    #[inline]
    pub fn hsize(&self) -> [f64; 2] {
        self.hsize
    }
}

/// OCCT `Bnd_B2::compareDist` — the `IsOut`-style separating-axis predicate.
#[inline]
fn compare_dist(hsize: &[f64; 2], dist: &[f64; 2]) -> bool {
    dist[0].abs() > hsize[0] || dist[1].abs() > hsize[1]
}

/// Whether `trsf`'s linear part is a uniform scale `s·I` (covers OCCT's
/// `gp_Identity`, `gp_Translation`, `gp_Scale` and `gp_PntMirror` forms, where
/// the linear part is diagonal with equal entries). Anything with off-diagonal
/// terms (i.e. a rotation) is *not* simple.
fn trsf_is_simple(trsf: &Trsf2d) -> bool {
    let m = &trsf.m;
    let s = m[0][0];
    const T: f64 = 1.0e-12;
    (m[1][1] - s).abs() <= T && m[0][1].abs() <= T && m[1][0].abs() <= T
}

/// OCCT `gp_Trsf2d::ScaleFactor` (signed) and its absolute value. The signed
/// scale is `sqrt(|det|)` carrying the sign of the determinant so that a
/// `PntMirror`/reflection (det < 0) yields a negative factor exactly as OCCT
/// does.
fn trsf_scale(trsf: &Trsf2d) -> (f64, f64) {
    let m = &trsf.m;
    let det = m[0][0] * m[1][1] - m[0][1] * m[1][0];
    let mag = det.abs().sqrt();
    let scale = if det < 0.0 { -mag } else { mag };
    (scale, scale.abs())
}

// occt-note: Bnd_B2<float>
/// Single-precision variant (`Bnd_B2f` = `Bnd_B2<float>`). Center and half-size
/// are rounded through `f32` on every store, reproducing the precision loss of
/// the OCCT `float` instantiation; all queries run in `f64`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BndB2f {
    inner: BndB2d,
}

impl Default for BndB2f {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl BndB2f {
    #[inline]
    pub fn new() -> Self {
        Self {
            inner: BndB2d::new(),
        }
    }

    /// Constructor from center and half-size, rounding components to `f32`.
    #[inline]
    pub fn from_center_hsize(center: Pnt2d, hsize: Pnt2d) -> Self {
        Self {
            inner: BndB2d {
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
    pub fn corner_min(&self) -> Pnt2d {
        self.inner.corner_min()
    }

    #[inline]
    pub fn corner_max(&self) -> Pnt2d {
        self.inner.corner_max()
    }

    #[inline]
    pub fn center(&self) -> [f64; 2] {
        self.inner.center()
    }

    #[inline]
    pub fn hsize(&self) -> [f64; 2] {
        self.inner.hsize()
    }
}

#[inline]
fn round_f32(p: Pnt2d) -> [f64; 2] {
    [p.x as f32 as f64, p.y as f32 as f64]
}
