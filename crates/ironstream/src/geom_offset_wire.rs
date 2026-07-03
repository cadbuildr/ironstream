// FILE: geom_offset_wire.rs
//! Wire offset operations for closed planar polylines.
//!
//! This module provides two offset builders that mirror OCCT's
//! `BRepOffsetAPI_MakeOffset` (general join-type wire offset) and
//! `BRepOffset_MakeSimpleOffset` (simplified constant-distance offset), plus a
//! free function [`offset_closed_wire`] for quick one-shot use.
//!
//! All types are pure-Rust, zero-dependency (only `std`).  Input wires are
//! represented as `Vec<[f64; 2]>` — sequences of 2-D points in the XY plane.
//!
//! # Algorithm
//!
//! Each edge of the closed polygon is shifted outward (positive distance) or
//! inward (negative distance) by `distance` along the edge's outward unit
//! normal.  Adjacent shifted edges are then intersected to find the new
//! vertices.  For the `Arc` join type the stub treats the join the same as
//! `Intersection`; a full implementation would insert circular arc segments
//! at convex corners.

// ─────────────────────────────────────────────────────────────────────────────
// Internal 2-D geometry helpers (no external crates)
// ─────────────────────────────────────────────────────────────────────────────

/// Subtract two 2-D points.
#[inline]
fn sub2(a: [f64; 2], b: [f64; 2]) -> [f64; 2] {
    [a[0] - b[0], a[1] - b[1]]
}

/// Add two 2-D vectors.
#[inline]
fn add2(a: [f64; 2], b: [f64; 2]) -> [f64; 2] {
    [a[0] + b[0], a[1] + b[1]]
}

/// Scale a 2-D vector.
#[inline]
fn scale2(v: [f64; 2], s: f64) -> [f64; 2] {
    [v[0] * s, v[1] * s]
}

/// Euclidean length of a 2-D vector.
#[inline]
fn len2(v: [f64; 2]) -> f64 {
    (v[0] * v[0] + v[1] * v[1]).sqrt()
}

/// Normalize a 2-D vector; returns `[0, 0]` for zero-length input.
#[inline]
fn normalize2(v: [f64; 2]) -> [f64; 2] {
    let l = len2(v);
    if l < 1e-12 {
        [0.0, 0.0]
    } else {
        [v[0] / l, v[1] / l]
    }
}

/// Right-hand 2-D perpendicular (outward normal for CCW polygon): rotate 90° CW.
/// For a CCW polygon with edge direction `d`, the outward normal is `[d[1], -d[0]]`.
#[inline]
fn perp_right(d: [f64; 2]) -> [f64; 2] {
    [d[1], -d[0]]
}

/// 2-D cross product (scalar): `a × b = a[0]*b[1] - a[1]*b[0]`.
#[inline]
fn cross2(a: [f64; 2], b: [f64; 2]) -> f64 {
    a[0] * b[1] - a[1] * b[0]
}

/// Signed area of a closed polygon via the shoelace formula.
/// Positive for CCW winding in standard (Y-up) coordinates.
fn signed_area_2d(pts: &[[f64; 2]]) -> f64 {
    let n = pts.len();
    if n < 3 {
        return 0.0;
    }
    let mut sum = 0.0_f64;
    for i in 0..n {
        let a = pts[i];
        let b = pts[(i + 1) % n];
        sum += a[0] * b[1] - b[0] * a[1];
    }
    sum * 0.5
}

/// Intersect two 2-D lines: line A = p + s*d, line B = q + t*e.
/// Returns `Some(point)` when the lines are not parallel; `None` otherwise.
fn line_intersect_2d(p: [f64; 2], d: [f64; 2], q: [f64; 2], e: [f64; 2]) -> Option<[f64; 2]> {
    let denom = cross2(d, e);
    if denom.abs() < 1e-14 {
        return None; // parallel
    }
    let diff = sub2(q, p);
    let s = cross2(diff, e) / denom;
    Some(add2(p, scale2(d, s)))
}

/// Compute the offset polygon for a closed polyline.
///
/// Moves every edge along its outward normal by `dist`, then intersects
/// adjacent shifted edges.  Returns the empty `Vec` if the input has fewer
/// than 3 points or all edges are degenerate.
fn compute_offset_2d(pts: &[[f64; 2]], dist: f64) -> Vec<[f64; 2]> {
    let n = pts.len();
    if n < 3 {
        return Vec::new();
    }
    // Zero-distance offset: return a copy of the input unchanged.
    if dist == 0.0 {
        return pts.to_vec();
    }

    // Determine winding; if CW, flip the sign of dist so the algorithm still
    // produces the correct outward direction.
    let area = signed_area_2d(pts);
    let effective_dist = if area >= 0.0 { dist } else { -dist };

    // Shift each edge along its outward normal.
    let mut shifted: Vec<([f64; 2], [f64; 2])> = Vec::with_capacity(n);
    for i in 0..n {
        let a = pts[i];
        let b = pts[(i + 1) % n];
        let dir = normalize2(sub2(b, a));
        let normal = perp_right(dir); // outward normal for CCW polygon
        let offset_vec = scale2(normal, effective_dist);
        shifted.push((add2(a, offset_vec), add2(b, offset_vec)));
    }

    // Intersect consecutive shifted edges to find new vertices.
    let mut result: Vec<[f64; 2]> = Vec::with_capacity(n);
    for i in 0..n {
        let (p0, p1) = shifted[i];
        let (q0, q1) = shifted[(i + 1) % n];
        let d = sub2(p1, p0);
        let e = sub2(q1, q0);
        match line_intersect_2d(p0, d, q0, e) {
            Some(pt) => result.push(pt),
            None => result.push(p1), // parallel edges: use endpoint of first shifted edge
        }
    }

    result
}

// ─────────────────────────────────────────────────────────────────────────────
// JoinType
// ─────────────────────────────────────────────────────────────────────────────

/// Strategy for joining adjacent offset edges at corners.
///
/// Mirrors OCCT's `GeomAbs_JoinType` as used by `BRepOffsetAPI_MakeOffset`.
// occt: BRepOffsetAPI_MakeOffset (GeomAbs_JoinType)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JoinType {
    /// Insert a circular arc at convex corners.
    /// Mirrors `GeomAbs_Arc`.
    // occt: GeomAbs_Arc
    Arc,
    /// Extend and intersect adjacent offset edges at corners.
    /// Mirrors `GeomAbs_Intersection`.
    // occt: GeomAbs_Intersection
    Intersection,
    /// Join by a tangent line segment at corners.
    /// Mirrors `GeomAbs_Tangent`.
    // occt: GeomAbs_Tangent
    Tangent,
}

// ─────────────────────────────────────────────────────────────────────────────
// WireOffset
// ─────────────────────────────────────────────────────────────────────────────

/// Builder for a general planar wire offset.
///
/// Inflates or deflates a closed 2-D polygon by `distance`, using one of three
/// join strategies for corners.  Mirrors OCCT's `BRepOffsetAPI_MakeOffset`.
///
/// # Example
///
/// ```
/// use ironstream::geom_offset_wire::{WireOffset, JoinType};
///
/// let square = vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]];
/// let result = WireOffset::new(square, 0.5)
///     .with_join(JoinType::Intersection)
///     .build();
/// assert_eq!(result.len(), 4);
/// ```
// occt: BRepOffsetAPI_MakeOffset
#[derive(Debug, Clone)]
pub struct WireOffset {
    /// Input closed polygon (2-D points).
    pub wire: Vec<[f64; 2]>,
    /// Signed offset distance (positive = outward, negative = inward).
    pub distance: f64,
    /// Corner join strategy.
    pub join_type: JoinType,
    /// Cached result after `build()`.
    result: Option<Vec<[f64; 2]>>,
}

impl WireOffset {
    /// Create a new `WireOffset` builder with default join type `Intersection`.
    ///
    /// Mirrors `BRepOffsetAPI_MakeOffset::BRepOffsetAPI_MakeOffset(wire, dist)`.
    // occt: BRepOffsetAPI_MakeOffset::BRepOffsetAPI_MakeOffset
    pub fn new(wire: Vec<[f64; 2]>, dist: f64) -> Self {
        Self {
            wire,
            distance: dist,
            join_type: JoinType::Intersection,
            result: None,
        }
    }

    /// Set the corner join strategy, consuming and returning `self`.
    ///
    /// Mirrors `BRepOffsetAPI_MakeOffset::SetJoin`.
    // occt: BRepOffsetAPI_MakeOffset::SetJoin
    pub fn with_join(mut self, j: JoinType) -> Self {
        self.join_type = j;
        self.result = None; // invalidate cache
        self
    }

    /// Compute and return the offset polygon.
    ///
    /// The result is cached; subsequent calls return the same `Vec` without
    /// recomputing.
    ///
    /// Mirrors `BRepOffsetAPI_MakeOffset::Build` / `Shape`.
    // occt: BRepOffsetAPI_MakeOffset::Build
    pub fn build(&mut self) -> Vec<[f64; 2]> {
        if self.result.is_none() {
            // Arc and Tangent joins are structurally identical to Intersection
            // in this stub; a full implementation would insert arc/tangent
            // segments at convex corners.
            self.result = Some(compute_offset_2d(&self.wire, self.distance));
        }
        self.result.clone().unwrap()
    }

    /// Return `true` if `build()` produced a non-empty result.
    ///
    /// Mirrors `BRepOffsetAPI_MakeOffset::IsDone`.
    // occt: BRepOffsetAPI_MakeOffset::IsDone
    pub fn is_done(&mut self) -> bool {
        !self.build().is_empty()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// SimpleOffset
// ─────────────────────────────────────────────────────────────────────────────

/// Builder for a simplified constant-distance shape offset.
///
/// Accepts a shape descriptor as a `String` (e.g. a serialised name or ID) and
/// an offset distance, then produces an offset descriptor string on `build()`.
/// Mirrors OCCT's `BRepOffset_MakeSimpleOffset`.
///
/// # Example
///
/// ```
/// use ironstream::geom_offset_wire::SimpleOffset;
///
/// let mut op = SimpleOffset::new("Box_1", 2.0);
/// let result = op.build();
/// assert!(op.is_done());
/// assert!(!result.is_empty());
/// ```
// occt: BRepOffset_MakeSimpleOffset
#[derive(Debug, Clone)]
pub struct SimpleOffset {
    /// Input shape descriptor (name, ID, or serialised representation).
    pub shape: String,
    /// Signed offset distance.
    pub offset: f64,
    /// Cached result after `build()`.
    result: Option<String>,
}

impl SimpleOffset {
    /// Create a new `SimpleOffset` builder.
    ///
    /// Mirrors `BRepOffset_MakeSimpleOffset::BRepOffset_MakeSimpleOffset(shape, offset)`.
    // occt: BRepOffset_MakeSimpleOffset::BRepOffset_MakeSimpleOffset
    pub fn new(shape: &str, offset: f64) -> Self {
        Self {
            shape: shape.to_owned(),
            offset,
            result: None,
        }
    }

    /// Compute the offset and return the resulting shape descriptor.
    ///
    /// The stub appends `"+offset=<value>"` to the input shape string.
    ///
    /// Mirrors `BRepOffset_MakeSimpleOffset::Build` / `GetResultShape`.
    // occt: BRepOffset_MakeSimpleOffset::Build
    pub fn build(&mut self) -> String {
        if self.result.is_none() {
            // Stub: annotate the shape string with the applied offset.
            self.result = Some(format!("{}+offset={}", self.shape, self.offset));
        }
        self.result.clone().unwrap()
    }

    /// Return `true` if `build()` has been called and produced a result.
    ///
    /// Mirrors `BRepOffset_MakeSimpleOffset::IsDone`.
    // occt: BRepOffset_MakeSimpleOffset::IsDone
    pub fn is_done(&mut self) -> bool {
        let r = self.build();
        !r.is_empty()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// offset_closed_wire
// ─────────────────────────────────────────────────────────────────────────────

/// Offset a closed 2-D polygon by `dist`.
///
/// Positive `dist` expands the polygon outward; negative `dist` shrinks it
/// inward.  The polygon is treated as closed: the edge from the last point
/// back to the first point is included.
///
/// Returns an empty `Vec` if `pts` has fewer than 3 points.
///
/// This is a convenience wrapper around [`WireOffset`] that skips the builder
/// pattern for one-shot use.
///
/// # Example
///
/// ```
/// use ironstream::geom_offset_wire::offset_closed_wire;
///
/// let square = &[[0.0_f64, 0.0], [2.0, 0.0], [2.0, 2.0], [0.0, 2.0]];
/// let expanded = offset_closed_wire(square, 1.0);
/// // Expect a 4-point polygon shifted outward by 1 unit on all sides.
/// assert_eq!(expanded.len(), 4);
/// ```
// occt: BRepOffsetAPI_MakeOffset (free-function convenience wrapper)
pub fn offset_closed_wire(pts: &[[f64; 2]], dist: f64) -> Vec<[f64; 2]> {
    compute_offset_2d(pts, dist)
}

// ─────────────────────────────────────────────────────────────────────────────
// Unit tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn approx_eq(a: f64, b: f64, tol: f64) -> bool {
        (a - b).abs() < tol
    }

    // ── JoinType ──────────────────────────────────────────────────────────────

    #[test]
    fn join_type_variants_are_distinct() {
        assert_ne!(JoinType::Arc, JoinType::Intersection);
        assert_ne!(JoinType::Arc, JoinType::Tangent);
        assert_ne!(JoinType::Intersection, JoinType::Tangent);
    }

    // ── WireOffset ────────────────────────────────────────────────────────────

    #[test]
    fn wire_offset_new_defaults() {
        let w = WireOffset::new(vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0]], 0.5);
        assert!(approx_eq(w.distance, 0.5, 1e-15));
        assert_eq!(w.join_type, JoinType::Intersection);
        assert_eq!(w.wire.len(), 3);
    }

    #[test]
    fn wire_offset_with_join_sets_join_type() {
        let w = WireOffset::new(vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0]], 0.1)
            .with_join(JoinType::Arc);
        assert_eq!(w.join_type, JoinType::Arc);
    }

    #[test]
    fn wire_offset_square_expand_has_four_points() {
        // Unit square expanded by 1.0.
        let square = vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]];
        let mut wo = WireOffset::new(square, 1.0);
        let result = wo.build();
        assert_eq!(result.len(), 4, "offset square should still have 4 vertices");
    }

    #[test]
    fn wire_offset_square_expand_is_done() {
        let square = vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]];
        let mut wo = WireOffset::new(square, 0.5);
        assert!(wo.is_done(), "valid offset must report is_done = true");
    }

    #[test]
    fn wire_offset_square_expand_grows_bounds() {
        // Unit square offset +1 should have corners near (−1,−1) and (2,2).
        let square = vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]];
        let mut wo = WireOffset::new(square, 1.0);
        let result = wo.build();
        let min_x = result.iter().map(|p| p[0]).fold(f64::INFINITY, f64::min);
        let max_x = result.iter().map(|p| p[0]).fold(f64::NEG_INFINITY, f64::max);
        let min_y = result.iter().map(|p| p[1]).fold(f64::INFINITY, f64::min);
        let max_y = result.iter().map(|p| p[1]).fold(f64::NEG_INFINITY, f64::max);
        assert!(min_x < -0.5, "left side must expand: min_x={min_x}");
        assert!(max_x > 1.5, "right side must expand: max_x={max_x}");
        assert!(min_y < -0.5, "bottom side must expand: min_y={min_y}");
        assert!(max_y > 1.5, "top side must expand: max_y={max_y}");
    }

    #[test]
    fn wire_offset_square_shrink_tightens_bounds() {
        // 4×4 square shrunk by 1 should have corners near (1,1) and (3,3).
        let square = vec![[0.0, 0.0], [4.0, 0.0], [4.0, 4.0], [0.0, 4.0]];
        let mut wo = WireOffset::new(square, -1.0);
        let result = wo.build();
        let min_x = result.iter().map(|p| p[0]).fold(f64::INFINITY, f64::min);
        let max_x = result.iter().map(|p| p[0]).fold(f64::NEG_INFINITY, f64::max);
        assert!(approx_eq(min_x, 1.0, 0.1), "shrink min_x={min_x}");
        assert!(approx_eq(max_x, 3.0, 0.1), "shrink max_x={max_x}");
    }

    #[test]
    fn wire_offset_too_few_points_is_not_done() {
        let mut wo = WireOffset::new(vec![[0.0, 0.0], [1.0, 0.0]], 0.5);
        assert!(!wo.is_done(), "fewer than 3 points must not be done");
        assert!(wo.build().is_empty());
    }

    #[test]
    fn wire_offset_build_caches_result() {
        let square = vec![[0.0, 0.0], [2.0, 0.0], [2.0, 2.0], [0.0, 2.0]];
        let mut wo = WireOffset::new(square, 0.5);
        let r1 = wo.build();
        let r2 = wo.build();
        assert_eq!(r1, r2, "consecutive build() calls must return equal results");
    }

    #[test]
    fn wire_offset_arc_join_returns_result() {
        // Arc join in the stub behaves like Intersection; just confirm it builds.
        let tri = vec![[0.0, 0.0], [3.0, 0.0], [1.5, 2.0]];
        let mut wo = WireOffset::new(tri, 0.2).with_join(JoinType::Arc);
        assert!(wo.is_done());
    }

    #[test]
    fn wire_offset_tangent_join_returns_result() {
        let tri = vec![[0.0, 0.0], [3.0, 0.0], [1.5, 2.0]];
        let mut wo = WireOffset::new(tri, 0.2).with_join(JoinType::Tangent);
        assert!(wo.is_done());
    }

    // ── SimpleOffset ─────────────────────────────────────────────────────────

    #[test]
    fn simple_offset_new_stores_fields() {
        let so = SimpleOffset::new("Sphere_1", 3.5);
        assert_eq!(so.shape, "Sphere_1");
        assert!(approx_eq(so.offset, 3.5, 1e-15));
    }

    #[test]
    fn simple_offset_build_contains_shape_name() {
        let mut so = SimpleOffset::new("Box_A", 1.0);
        let result = so.build();
        assert!(result.contains("Box_A"), "result must contain the shape name");
    }

    #[test]
    fn simple_offset_build_contains_offset_value() {
        let mut so = SimpleOffset::new("Shell_3", 2.5);
        let result = so.build();
        assert!(result.contains("2.5"), "result must encode the offset distance");
    }

    #[test]
    fn simple_offset_is_done_after_build() {
        let mut so = SimpleOffset::new("Face_7", -0.5);
        assert!(so.is_done(), "is_done must return true after implicit build");
    }

    #[test]
    fn simple_offset_build_caches() {
        let mut so = SimpleOffset::new("Edge_2", 1.0);
        let r1 = so.build();
        let r2 = so.build();
        assert_eq!(r1, r2, "consecutive build() calls must return equal strings");
    }

    #[test]
    fn simple_offset_negative_distance() {
        let mut so = SimpleOffset::new("Solid_X", -3.14);
        let result = so.build();
        assert!(!result.is_empty());
        assert!(so.is_done());
    }

    // ── offset_closed_wire ───────────────────────────────────────────────────

    #[test]
    fn offset_closed_wire_unit_square_expand() {
        let square = &[[0.0_f64, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]];
        let result = offset_closed_wire(square, 1.0);
        assert_eq!(result.len(), 4);
        let min_x = result.iter().map(|p| p[0]).fold(f64::INFINITY, f64::min);
        let max_x = result.iter().map(|p| p[0]).fold(f64::NEG_INFINITY, f64::max);
        assert!(min_x < -0.5, "expand: min_x={min_x}");
        assert!(max_x > 1.5, "expand: max_x={max_x}");
    }

    #[test]
    fn offset_closed_wire_unit_square_shrink() {
        let square = &[[0.0_f64, 0.0], [4.0, 0.0], [4.0, 4.0], [0.0, 4.0]];
        let result = offset_closed_wire(square, -1.0);
        assert_eq!(result.len(), 4);
        let min_x = result.iter().map(|p| p[0]).fold(f64::INFINITY, f64::min);
        let max_x = result.iter().map(|p| p[0]).fold(f64::NEG_INFINITY, f64::max);
        assert!(approx_eq(min_x, 1.0, 0.1), "shrink min_x={min_x}");
        assert!(approx_eq(max_x, 3.0, 0.1), "shrink max_x={max_x}");
    }

    #[test]
    fn offset_closed_wire_too_few_points_returns_empty() {
        let pts = &[[0.0_f64, 0.0], [1.0, 0.0]];
        let result = offset_closed_wire(pts, 0.5);
        assert!(result.is_empty(), "< 3 points must return empty Vec");
    }

    #[test]
    fn offset_closed_wire_zero_distance_is_identity() {
        let square = &[[0.0_f64, 0.0], [2.0, 0.0], [2.0, 2.0], [0.0, 2.0]];
        let result = offset_closed_wire(square, 0.0);
        assert_eq!(result.len(), square.len());
        for (orig, off) in square.iter().zip(result.iter()) {
            assert!(approx_eq(orig[0], off[0], 1e-9), "x mismatch");
            assert!(approx_eq(orig[1], off[1], 1e-9), "y mismatch");
        }
    }

    #[test]
    fn offset_closed_wire_triangle_expand() {
        // Equilateral-ish triangle; just confirm vertex count and outward expansion.
        let tri = &[[0.0_f64, 0.0], [4.0, 0.0], [2.0, 3.0]];
        let result = offset_closed_wire(tri, 0.5);
        assert_eq!(result.len(), 3, "triangle must stay 3-vertex after offset");
        // Centroid of original is ~ (2, 1); offset points should be further away.
        let cx = 2.0_f64;
        let cy = 1.0_f64;
        for (orig, off) in tri.iter().zip(result.iter()) {
            let d_orig = ((orig[0] - cx).powi(2) + (orig[1] - cy).powi(2)).sqrt();
            let d_off = ((off[0] - cx).powi(2) + (off[1] - cy).powi(2)).sqrt();
            assert!(
                d_off > d_orig - 1e-9,
                "offset point must be at least as far from centroid as original: d_orig={d_orig} d_off={d_off}"
            );
        }
    }

    #[test]
    fn offset_closed_wire_cw_polygon_expand() {
        // CW winding: same square in reverse order.  Positive dist should still expand.
        let cw_square = &[[0.0_f64, 1.0], [1.0, 1.0], [1.0, 0.0], [0.0, 0.0]];
        let result = offset_closed_wire(cw_square, 1.0);
        assert_eq!(result.len(), 4);
        let min_x = result.iter().map(|p| p[0]).fold(f64::INFINITY, f64::min);
        let max_x = result.iter().map(|p| p[0]).fold(f64::NEG_INFINITY, f64::max);
        // The offset polygon must be larger than the unit square.
        assert!(min_x < 0.0, "CW expand: min_x={min_x}");
        assert!(max_x > 1.0, "CW expand: max_x={max_x}");
    }

    // ── Internal helpers ─────────────────────────────────────────────────────

    #[test]
    fn signed_area_ccw_square_positive() {
        let sq = &[[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]];
        assert!(signed_area_2d(sq) > 0.0, "CCW square must have positive area");
    }

    #[test]
    fn signed_area_cw_square_negative() {
        let sq = &[[0.0, 0.0], [0.0, 1.0], [1.0, 1.0], [1.0, 0.0]];
        assert!(signed_area_2d(sq) < 0.0, "CW square must have negative area");
    }

    #[test]
    fn line_intersect_2d_basic() {
        // Two perpendicular lines: X-axis and Y-axis should meet at origin.
        let p = [0.0_f64, -1.0];
        let d = [0.0, 1.0];
        let q = [-1.0_f64, 0.0];
        let e = [1.0, 0.0];
        let pt = line_intersect_2d(p, d, q, e).unwrap();
        assert!(approx_eq(pt[0], 0.0, 1e-10));
        assert!(approx_eq(pt[1], 0.0, 1e-10));
    }

    #[test]
    fn line_intersect_2d_parallel_returns_none() {
        let p = [0.0_f64, 0.0];
        let d = [1.0, 0.0];
        let q = [0.0_f64, 1.0];
        let e = [1.0, 0.0];
        assert!(line_intersect_2d(p, d, q, e).is_none());
    }
}
