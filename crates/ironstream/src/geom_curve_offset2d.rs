//! `geom_curve_offset2d` — pure-Rust zero-dependency stub for a 2D offset curve.
//!
//! Models [`OffsetCurve2d`] after OCCT's `Geom2d_OffsetCurve`.  The basis curve
//! is identified by name (a `String`) so that this stub can be used in contexts
//! where the full curve graph is not yet wired up.
//!
//! The evaluation methods (`evaluate`, `d1`) use a finite-difference /
//! stub implementation that returns the zero vector when no concrete basis
//! curve data is available — callers that need accurate geometry should use
//! the full [`crate::geom2d_offset_curve::Geom2dOffsetCurve`] instead.
//!
//! This module is zero-dependency: it uses only `std` (`f64::sqrt`).

// occt: Geom2d_OffsetCurve

/// A 2D offset curve identified by the name of its basis curve and a signed
/// offset distance.
///
/// Corresponds to `Geom2d_OffsetCurve` in OpenCASCADE Technology.
// occt: Geom2d_OffsetCurve
#[derive(Clone, Debug, PartialEq)]
pub struct OffsetCurve2d {
    /// Name identifying the basis curve (e.g. `"circle_1"`).
    pub basis_curve: String,
    /// Signed offset distance along the CCW normal to the curve tangent.
    pub offset: f64,
}

impl OffsetCurve2d {
    // ──────────────────────────────────────────── Constructor ─────────────────

    /// Create a new `OffsetCurve2d` from a basis-curve name and offset distance.
    ///
    /// Mirrors `Geom2d_OffsetCurve(C, Offset)`.
    pub fn new(basis: &str, offset: f64) -> Self {
        Self {
            basis_curve: basis.to_owned(),
            offset,
        }
    }

    // ──────────────────────────────────────────── Evaluation stubs ────────────

    /// Evaluate the offset curve at parameter `t`.
    ///
    /// This stub returns `[t, self.offset]` — a placeholder that satisfies the
    /// API contract without requiring a live basis curve.  Replace with a
    /// proper basis-curve lookup when concrete geometry is available.
    ///
    /// Mirrors `Geom2d_OffsetCurve::Value(U)`.
    pub fn evaluate(&self, t: f64) -> [f64; 2] {
        // Stub: basis curve treated as the x-axis; offset applied in y.
        [t, self.offset]
    }

    /// First derivative of the offset curve at parameter `t`.
    ///
    /// For the stub (basis = x-axis line), the tangent is always `[1, 0]` and
    /// the normal is `[0, 1]`.  The offset derivative of `N(u)` is zero for a
    /// straight basis, so `D1 = [1, 0]`.
    ///
    /// Mirrors `Geom2d_OffsetCurve::D1(U, P, V1)`.
    pub fn d1(&self, _t: f64) -> [f64; 2] {
        // Stub: unit tangent along x for a straight basis curve.
        [1.0, 0.0]
    }

    // ──────────────────────────────────────────── Parameter range ─────────────

    /// First parameter of the offset curve (inherited from basis curve stub).
    ///
    /// Returns `0.0` — override when the basis curve supplies its own range.
    ///
    /// Mirrors `Geom2d_OffsetCurve::FirstParameter()`.
    pub fn first_parameter(&self) -> f64 {
        0.0
    }

    /// Last parameter of the offset curve (inherited from basis curve stub).
    ///
    /// Returns `1.0` — override when the basis curve supplies its own range.
    ///
    /// Mirrors `Geom2d_OffsetCurve::LastParameter()`.
    pub fn last_parameter(&self) -> f64 {
        1.0
    }

    // ──────────────────────────────────────────── Topology ────────────────────

    /// Whether the offset curve is closed.
    ///
    /// The stub checks whether the endpoints coincide within a small tolerance.
    ///
    /// Mirrors `Geom2d_OffsetCurve::IsClosed()`.
    pub fn is_closed(&self) -> bool {
        let p0 = self.evaluate(self.first_parameter());
        let p1 = self.evaluate(self.last_parameter());
        let dx = p1[0] - p0[0];
        let dy = p1[1] - p0[1];
        f64::sqrt(dx * dx + dy * dy) <= 1.0e-10
    }

    // ──────────────────────────────────────────── Getters ─────────────────────

    /// The signed offset distance.
    ///
    /// Mirrors `Geom2d_OffsetCurve::Offset()`.
    pub fn offset_value(&self) -> f64 {
        self.offset
    }

    /// A reference to the basis-curve name.
    ///
    /// Mirrors `Geom2d_OffsetCurve::BasisCurve()`.
    pub fn basis_curve(&self) -> &str {
        &self.basis_curve
    }

    // ──────────────────────────────────────────── Orientation ─────────────────

    /// Reverse the orientation of the offset curve in place.
    ///
    /// Negates the offset distance (equivalent to reversing the traversal
    /// direction while preserving the geometric shape of the offset).
    ///
    /// Mirrors `Geom2d_OffsetCurve::Reverse()`.
    pub fn reverse(&mut self) {
        self.offset = -self.offset;
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Polyline offset utility
// ─────────────────────────────────────────────────────────────────────────────

/// Offset a 2D polyline by `dist` along the outward CCW normal of each segment.
///
/// Each segment `pts[i] -> pts[i+1]` is offset by translating both endpoints
/// perpendicularly (CCW rotation of the segment tangent, scaled to `dist`).
/// The resulting offset segments are then intersected / averaged at joints to
/// produce a single output vertex per input vertex.
///
/// * For a single-point input the function returns that point unchanged.
/// * For a two-point input the function offsets both endpoints of the single
///   segment and returns the two offset points.
/// * For three or more points the interior joints are placed at the
///   intersection of adjacent offset segments; if two adjacent segments are
///   (anti-)parallel the mid-point of their offset endpoints is used instead.
///
/// Uses only `std` (`f64::sqrt`).
pub fn offset_polyline_2d(pts: &[[f64; 2]], dist: f64) -> Vec<[f64; 2]> {
    let n = pts.len();
    if n == 0 {
        return Vec::new();
    }
    if n == 1 {
        return vec![pts[0]];
    }

    // Build per-segment offset lines: each segment has a start and end point
    // already shifted by `dist` along its CCW normal.
    // A segment from A to B has tangent T = (B-A)/|B-A| and normal N = (-Ty, Tx).
    // Offset endpoints: A' = A + dist*N,  B' = B + dist*N.

    let seg_count = n - 1;
    // (start, end) of each offset segment
    let mut off_segs: Vec<([f64; 2], [f64; 2])> = Vec::with_capacity(seg_count);

    for i in 0..seg_count {
        let a = pts[i];
        let b = pts[i + 1];
        let dx = b[0] - a[0];
        let dy = b[1] - a[1];
        let len = f64::sqrt(dx * dx + dy * dy);
        let (tx, ty) = if len < 1.0e-15 {
            (1.0, 0.0)
        } else {
            (dx / len, dy / len)
        };
        // CCW normal: rotate tangent +90 degrees
        let nx = -ty;
        let ny = tx;
        let a_off = [a[0] + dist * nx, a[1] + dist * ny];
        let b_off = [b[0] + dist * nx, b[1] + dist * ny];
        off_segs.push((a_off, b_off));
    }

    // Assemble output vertices.
    let mut out: Vec<[f64; 2]> = Vec::with_capacity(n);

    // First vertex: start of the first offset segment.
    out.push(off_segs[0].0);

    // Interior joints: intersection of segment i and segment i+1.
    for i in 0..seg_count - 1 {
        let (p0, p1) = off_segs[i];       // segment i:   P0 + t*(P1-P0)
        let (q0, q1) = off_segs[i + 1];   // segment i+1: Q0 + s*(Q1-Q0)

        let dx1 = p1[0] - p0[0];
        let dy1 = p1[1] - p0[1];
        let dx2 = q1[0] - q0[0];
        let dy2 = q1[1] - q0[1];

        // Solve P0 + t*d1 = Q0 + s*d2  (2 equations, 2 unknowns t and s)
        // d1 x d2 = dx1*dy2 - dy1*dx2
        let cross = dx1 * dy2 - dy1 * dx2;

        let joint = if cross.abs() < 1.0e-12 {
            // Parallel segments — average the touching endpoints.
            [(p1[0] + q0[0]) * 0.5, (p1[1] + q0[1]) * 0.5]
        } else {
            // t = ((Q0 - P0) x d2) / (d1 x d2)
            let rhs_x = q0[0] - p0[0];
            let rhs_y = q0[1] - p0[1];
            let t = (rhs_x * dy2 - rhs_y * dx2) / cross;
            [p0[0] + t * dx1, p0[1] + t * dy1]
        };

        out.push(joint);
    }

    // Last vertex: end of the last offset segment.
    out.push(off_segs[seg_count - 1].1);

    out
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_stores_fields() {
        let c = OffsetCurve2d::new("arc_1", 2.5);
        assert_eq!(c.basis_curve(), "arc_1");
        assert!((c.offset_value() - 2.5).abs() < 1.0e-15);
    }

    #[test]
    fn evaluate_stub() {
        let c = OffsetCurve2d::new("line_x", 3.0);
        let p = c.evaluate(5.0);
        assert!((p[0] - 5.0).abs() < 1.0e-15);
        assert!((p[1] - 3.0).abs() < 1.0e-15);
    }

    #[test]
    fn d1_stub() {
        let c = OffsetCurve2d::new("line_x", 1.0);
        let v = c.d1(0.0);
        assert!((v[0] - 1.0).abs() < 1.0e-15);
        assert!(v[1].abs() < 1.0e-15);
    }

    #[test]
    fn parameter_range() {
        let c = OffsetCurve2d::new("line_x", 1.0);
        assert!((c.first_parameter() - 0.0).abs() < 1.0e-15);
        assert!((c.last_parameter() - 1.0).abs() < 1.0e-15);
    }

    #[test]
    fn is_closed_open_curve() {
        // For the stub (evaluate returns [t, offset]) with t in [0,1] and
        // non-zero offset, the start and end differ -> not closed.
        let c = OffsetCurve2d::new("line_x", 1.0);
        assert!(!c.is_closed());
    }

    #[test]
    fn reverse_negates_offset() {
        let mut c = OffsetCurve2d::new("circle_1", 4.0);
        c.reverse();
        assert!((c.offset_value() - (-4.0)).abs() < 1.0e-15);
    }

    #[test]
    fn offset_polyline_2d_axis_aligned() {
        // Horizontal polyline along x-axis, offset upward by 1.0.
        let pts = [[0.0, 0.0], [1.0, 0.0], [2.0, 0.0]];
        let off = offset_polyline_2d(&pts, 1.0);
        assert_eq!(off.len(), 3);
        for p in &off {
            assert!((p[1] - 1.0).abs() < 1.0e-10, "expected y=1, got {}", p[1]);
        }
    }

    #[test]
    fn offset_polyline_2d_single_point() {
        let pts = [[3.0, 4.0]];
        let off = offset_polyline_2d(&pts, 5.0);
        assert_eq!(off.len(), 1);
        assert!((off[0][0] - 3.0).abs() < 1.0e-15);
        assert!((off[0][1] - 4.0).abs() < 1.0e-15);
    }

    #[test]
    fn offset_polyline_2d_right_angle() {
        // L-shaped polyline: (0,0) -> (1,0) -> (1,1).
        // Offset CCW by 0.5 should place the miter joint at (0.5, 0.5).
        let pts = [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0]];
        let off = offset_polyline_2d(&pts, 0.5);
        assert_eq!(off.len(), 3);
        // First point: (0, 0.5)
        assert!((off[0][0] - 0.0).abs() < 1.0e-10);
        assert!((off[0][1] - 0.5).abs() < 1.0e-10);
        // Miter joint at (0.5, 0.5)
        assert!((off[1][0] - 0.5).abs() < 1.0e-10, "joint x={}", off[1][0]);
        assert!((off[1][1] - 0.5).abs() < 1.0e-10, "joint y={}", off[1][1]);
        // Last point: (0.5, 1.0)
        assert!((off[2][0] - 0.5).abs() < 1.0e-10);
        assert!((off[2][1] - 1.0).abs() < 1.0e-10);
    }
}
