//! `geom_curve_offset3d` — pure-Rust zero-dependency stub for a 3D offset curve.
//!
//! Models [`OffsetCurve3d`] after OCCT's `Geom_OffsetCurve`.  The basis curve
//! is identified by name (a `String`) so that this stub can be used in contexts
//! where the full curve graph is not yet wired up.
//!
//! The evaluation methods (`evaluate`, `d1`, `d2`) use a stub implementation
//! that returns the zero vector when no concrete basis curve data is available —
//! callers that need accurate geometry should use the full curve graph instead.
//!
//! The offset is applied along the supplied `direction` vector, which is
//! normalised on construction (or treated as zero if degenerate).
//!
//! This module is zero-dependency: it uses only `std` (`f64::sqrt`).

// occt: Geom_OffsetCurve

// ─────────────────────────────────────────────────────────────────────────────
// Internal helpers
// ─────────────────────────────────────────────────────────────────────────────

#[inline]
fn dot3(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

#[inline]
fn norm3(v: [f64; 3]) -> f64 {
    f64::sqrt(dot3(v, v))
}

/// Normalise a 3-vector; returns the zero vector if it is degenerate.
#[inline]
fn normalize3(v: [f64; 3]) -> [f64; 3] {
    let n = norm3(v);
    if n < f64::EPSILON {
        [0.0, 0.0, 0.0]
    } else {
        [v[0] / n, v[1] / n, v[2] / n]
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// OffsetCurve3d
// ─────────────────────────────────────────────────────────────────────────────

/// A 3D offset curve identified by the name of its basis curve, a scalar
/// offset distance, and an offset direction vector.
///
/// Corresponds to `Geom_OffsetCurve` in OpenCASCADE Technology.
// occt: Geom_OffsetCurve
#[derive(Clone, Debug, PartialEq)]
pub struct OffsetCurve3d {
    /// Name identifying the basis curve (e.g. `"circle_1"`).
    pub basis_curve: String,
    /// Signed offset distance along `direction`.
    pub offset: f64,
    /// Unit direction vector along which the offset is applied.
    ///
    /// Stored as supplied; normalised internally for evaluation.
    pub direction: [f64; 3],
}

impl OffsetCurve3d {
    // ──────────────────────────────────────────── Constructor ─────────────────

    /// Create a new `OffsetCurve3d` from a basis-curve name, offset distance,
    /// and offset direction.
    ///
    /// The `dir` vector is stored as-is; `direction` is part of the public
    /// interface and callers may normalise it themselves.
    ///
    /// Mirrors `Geom_OffsetCurve(C, Offset, V)`.
    pub fn new(basis: &str, offset: f64, dir: [f64; 3]) -> Self {
        Self {
            basis_curve: basis.to_owned(),
            offset,
            direction: dir,
        }
    }

    // ──────────────────────────────────────────── Evaluation stubs ────────────

    /// Evaluate the offset curve at parameter `t`.
    ///
    /// The stub models the basis curve as a straight line along the x-axis:
    /// `P(t) = [t, 0, 0]`.  The offset is applied along the (normalised)
    /// `direction` vector scaled by `self.offset`.
    ///
    /// Replace with a proper basis-curve lookup when concrete geometry is
    /// available.
    ///
    /// Mirrors `Geom_OffsetCurve::Value(U)`.
    pub fn evaluate(&self, t: f64) -> [f64; 3] {
        // Stub basis point: straight line along x-axis.
        let basis_pt = [t, 0.0, 0.0];
        let d = normalize3(self.direction);
        [
            basis_pt[0] + self.offset * d[0],
            basis_pt[1] + self.offset * d[1],
            basis_pt[2] + self.offset * d[2],
        ]
    }

    /// First derivative of the offset curve at parameter `t`.
    ///
    /// For the stub (basis = x-axis line) the basis tangent is always
    /// `[1, 0, 0]` and the normal derivative is zero, so `D1 = [1, 0, 0]`.
    ///
    /// Mirrors `Geom_OffsetCurve::D1(U, P, V1)`.
    pub fn d1(&self, _t: f64) -> [f64; 3] {
        // Stub: unit tangent along x for a straight basis curve.
        [1.0, 0.0, 0.0]
    }

    /// Second derivative of the offset curve at parameter `t`.
    ///
    /// For the stub (basis = x-axis line) the second derivative is always
    /// `[0, 0, 0]` since the tangent is constant.
    ///
    /// Mirrors `Geom_OffsetCurve::D2(U, P, V1, V2)`.
    pub fn d2(&self, _t: f64) -> [f64; 3] {
        // Stub: zero curvature for a straight basis curve.
        [0.0, 0.0, 0.0]
    }

    // ──────────────────────────────────────────── Parameter range ─────────────

    /// First parameter of the offset curve (inherited from basis curve stub).
    ///
    /// Returns `0.0` — override when the basis curve supplies its own range.
    ///
    /// Mirrors `Geom_OffsetCurve::FirstParameter()`.
    pub fn first_parameter(&self) -> f64 {
        0.0
    }

    /// Last parameter of the offset curve (inherited from basis curve stub).
    ///
    /// Returns `1.0` — override when the basis curve supplies its own range.
    ///
    /// Mirrors `Geom_OffsetCurve::LastParameter()`.
    pub fn last_parameter(&self) -> f64 {
        1.0
    }

    // ──────────────────────────────────────────── Getters ─────────────────────

    /// The signed offset distance.
    ///
    /// Mirrors `Geom_OffsetCurve::Offset()`.
    pub fn offset_value(&self) -> f64 {
        self.offset
    }

    /// A reference to the basis-curve name.
    ///
    /// Mirrors `Geom_OffsetCurve::BasisCurve()`.
    pub fn basis_curve(&self) -> &str {
        &self.basis_curve
    }

    // ──────────────────────────────────────────── Orientation ─────────────────

    /// Reverse the orientation of the offset curve in place.
    ///
    /// Negates the offset distance and flips the direction vector so that the
    /// traversal direction is reversed while the geometric shape is preserved.
    ///
    /// Mirrors `Geom_OffsetCurve::Reverse()`.
    pub fn reverse(&mut self) {
        self.offset = -self.offset;
        self.direction = [-self.direction[0], -self.direction[1], -self.direction[2]];
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_stores_fields() {
        let c = OffsetCurve3d::new("arc_1", 2.5, [0.0, 1.0, 0.0]);
        assert_eq!(c.basis_curve(), "arc_1");
        assert!((c.offset_value() - 2.5).abs() < 1.0e-15);
        assert!((c.direction[1] - 1.0).abs() < 1.0e-15);
    }

    #[test]
    fn evaluate_stub_y_offset() {
        // Offset along y by 3.0: evaluate(t) should give [t, 3, 0].
        let c = OffsetCurve3d::new("line_x", 3.0, [0.0, 1.0, 0.0]);
        let p = c.evaluate(5.0);
        assert!((p[0] - 5.0).abs() < 1.0e-12);
        assert!((p[1] - 3.0).abs() < 1.0e-12);
        assert!(p[2].abs() < 1.0e-12);
    }

    #[test]
    fn evaluate_stub_diagonal_direction() {
        // Direction [1,1,0] (normalised to [1/√2, 1/√2, 0]), offset 1.0.
        let c = OffsetCurve3d::new("line_x", 1.0, [1.0, 1.0, 0.0]);
        let p = c.evaluate(0.0);
        let expected = 1.0 / f64::sqrt(2.0);
        assert!((p[0] - expected).abs() < 1.0e-12);
        assert!((p[1] - expected).abs() < 1.0e-12);
        assert!(p[2].abs() < 1.0e-12);
    }

    #[test]
    fn evaluate_zero_direction_returns_basis_point() {
        // Degenerate direction: offset contribution is zero.
        let c = OffsetCurve3d::new("line_x", 5.0, [0.0, 0.0, 0.0]);
        let p = c.evaluate(3.0);
        assert!((p[0] - 3.0).abs() < 1.0e-12);
        assert!(p[1].abs() < 1.0e-12);
        assert!(p[2].abs() < 1.0e-12);
    }

    #[test]
    fn d1_stub() {
        let c = OffsetCurve3d::new("line_x", 1.0, [0.0, 1.0, 0.0]);
        let v = c.d1(0.5);
        assert!((v[0] - 1.0).abs() < 1.0e-15);
        assert!(v[1].abs() < 1.0e-15);
        assert!(v[2].abs() < 1.0e-15);
    }

    #[test]
    fn d2_stub() {
        let c = OffsetCurve3d::new("line_x", 1.0, [0.0, 1.0, 0.0]);
        let a = c.d2(0.5);
        assert!(a[0].abs() < 1.0e-15);
        assert!(a[1].abs() < 1.0e-15);
        assert!(a[2].abs() < 1.0e-15);
    }

    #[test]
    fn parameter_range() {
        let c = OffsetCurve3d::new("line_x", 1.0, [0.0, 0.0, 1.0]);
        assert!((c.first_parameter() - 0.0).abs() < 1.0e-15);
        assert!((c.last_parameter() - 1.0).abs() < 1.0e-15);
    }

    #[test]
    fn reverse_negates_offset_and_flips_direction() {
        let mut c = OffsetCurve3d::new("circle_1", 4.0, [0.0, 1.0, 0.0]);
        c.reverse();
        assert!((c.offset_value() - (-4.0)).abs() < 1.0e-15);
        assert!((c.direction[1] - (-1.0)).abs() < 1.0e-15);
    }

    #[test]
    fn basis_curve_returns_name() {
        let c = OffsetCurve3d::new("spline_42", 0.1, [1.0, 0.0, 0.0]);
        assert_eq!(c.basis_curve(), "spline_42");
    }
}
