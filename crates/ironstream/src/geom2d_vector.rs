//! `Geom2d_VectorWithMagnitude` -- a 2D vector that retains its magnitude,
//! mirroring OpenCascade's `Geom2d_VectorWithMagnitude` class (package TKG2d).
//!
//! Unlike a unit-direction (`gp_Dir2d`), a `VectorWithMagnitude` may have
//! any non-zero length. The class wraps a [`Vec2d`] (`gp_Vec2d`) and exposes
//! the standard vector algebra: addition, subtraction, scaling, dot product,
//! 2D cross product ("crossed"), normalization and transform.
//!
//! OCCT raises `Standard_ConstructionError` whenever an operation would produce
//! a zero vector that is then normalised. This implementation panics with a
//! descriptive message in those cases.

use crate::gp2d::{Pnt2d, Trsf2d, Vec2d};

// occt: Geom2d_VectorWithMagnitude
/// A 2D vector retaining its magnitude (`Geom2d_VectorWithMagnitude`).
///
/// Internally stores a [`Vec2d`] (alias for [`Pnt2d`]) whose coordinates are
/// `(x, y)`. The magnitude is always `sqrt(x² + y²)`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VectorWithMagnitude {
    v: Vec2d,
}

impl VectorWithMagnitude {
    // -----------------------------------------------------------------------
    // Constructors
    // -----------------------------------------------------------------------

    /// Builds a vector from explicit `x` and `y` coordinates.
    ///
    /// Mirrors `Geom2d_VectorWithMagnitude(const Standard_Real X, const Standard_Real Y)`.
    #[inline]
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            v: Vec2d::new(x, y),
        }
    }

    /// Builds a vector from an existing `gp_Vec2d`.
    ///
    /// Mirrors `Geom2d_VectorWithMagnitude(const gp_Vec2d& V)`.
    #[inline]
    pub fn from_vec2d(v: Vec2d) -> Self {
        Self { v }
    }

    /// Builds the vector `p1 -> p2` (difference of two points).
    ///
    /// Mirrors `Geom2d_VectorWithMagnitude(const gp_Pnt2d& P1, const gp_Pnt2d& P2)`.
    #[inline]
    pub fn from_points(p1: Pnt2d, p2: Pnt2d) -> Self {
        Self { v: p2 - p1 }
    }

    // -----------------------------------------------------------------------
    // Component accessors
    // -----------------------------------------------------------------------

    /// The X component.
    #[inline]
    pub fn x(&self) -> f64 {
        self.v.x
    }

    /// The Y component.
    #[inline]
    pub fn y(&self) -> f64 {
        self.v.y
    }

    /// The underlying `gp_Vec2d`.
    #[inline]
    pub fn vec2d(&self) -> Vec2d {
        self.v
    }

    // -----------------------------------------------------------------------
    // Magnitude
    // -----------------------------------------------------------------------

    /// Returns `sqrt(x² + y²)`.
    ///
    /// Mirrors `Magnitude()`.
    #[inline]
    pub fn magnitude(&self) -> f64 {
        self.v.norm()
    }

    /// Returns `x² + y²`.
    ///
    /// Mirrors `SquareMagnitude()`.
    #[inline]
    pub fn square_magnitude(&self) -> f64 {
        self.v.dot(self.v)
    }

    // -----------------------------------------------------------------------
    // Algebra -- binary operations returning a new vector
    // -----------------------------------------------------------------------

    /// Vector addition (`Other = Self + Other`). Mirrors `Added(V)`.
    #[inline]
    pub fn added(&self, other: &VectorWithMagnitude) -> VectorWithMagnitude {
        VectorWithMagnitude { v: self.v + other.v }
    }

    /// In-place addition. Mirrors `Add(V)`.
    #[inline]
    pub fn add(&mut self, other: &VectorWithMagnitude) {
        self.v = self.v + other.v;
    }

    /// Vector subtraction. Mirrors `Subtracted(V)`.
    #[inline]
    pub fn subtracted(&self, other: &VectorWithMagnitude) -> VectorWithMagnitude {
        VectorWithMagnitude { v: self.v - other.v }
    }

    /// In-place subtraction. Mirrors `Subtract(V)`.
    #[inline]
    pub fn subtract(&mut self, other: &VectorWithMagnitude) {
        self.v = self.v - other.v;
    }

    /// 2D "cross product" — the scalar `x1*y2 - y1*x2` (the Z component of
    /// the 3D cross product of the lifted vectors).
    ///
    /// Mirrors `Crossed(V)` / `operator^`.
    #[inline]
    pub fn crossed(&self, other: &VectorWithMagnitude) -> f64 {
        self.v.cross(other.v)
    }

    /// Dot product `x1*x2 + y1*y2`.
    ///
    /// Mirrors `Dot(V)` / `operator*`.
    #[inline]
    pub fn dot(&self, other: &VectorWithMagnitude) -> f64 {
        self.v.dot(other.v)
    }

    /// Returns a scaled copy. Mirrors `Multiplied(Scalar)`.
    #[inline]
    pub fn multiplied(&self, scalar: f64) -> VectorWithMagnitude {
        VectorWithMagnitude { v: self.v * scalar }
    }

    /// Scales in place. Mirrors `Multiply(Scalar)`.
    #[inline]
    pub fn multiply(&mut self, scalar: f64) {
        self.v = self.v * scalar;
    }

    // -----------------------------------------------------------------------
    // Normalisation
    // -----------------------------------------------------------------------

    /// Returns a unit vector in the same direction.
    ///
    /// Mirrors `Normalized()`. Panics (as OCCT raises
    /// `Standard_ConstructionError`) when the vector is zero.
    pub fn normalized(&self) -> VectorWithMagnitude {
        let m = self.magnitude();
        assert!(
            m > 0.0,
            "Geom2d_VectorWithMagnitude::Normalized: cannot normalise a zero vector"
        );
        VectorWithMagnitude {
            v: self.v * (1.0 / m),
        }
    }

    /// Normalises in place. Mirrors `Normalize()`.
    ///
    /// Panics when the vector is zero.
    pub fn normalize(&mut self) {
        let m = self.magnitude();
        assert!(
            m > 0.0,
            "Geom2d_VectorWithMagnitude::Normalize: cannot normalise a zero vector"
        );
        self.v = self.v * (1.0 / m);
    }

    // -----------------------------------------------------------------------
    // Reversal
    // -----------------------------------------------------------------------

    /// Returns a copy with reversed direction. Mirrors `Reversed()`.
    #[inline]
    pub fn reversed(&self) -> VectorWithMagnitude {
        VectorWithMagnitude { v: -self.v }
    }

    /// Reverses the direction in place. Mirrors `Reverse()`.
    #[inline]
    pub fn reverse(&mut self) {
        self.v = -self.v;
    }

    // -----------------------------------------------------------------------
    // Transform
    // -----------------------------------------------------------------------

    /// Returns a copy transformed by `t`. Mirrors `Transformed(T)`.
    ///
    /// Only the linear (rotation + scale) part of the transform is applied —
    /// the translation does not affect free vectors.
    pub fn transformed(&self, t: &Trsf2d) -> VectorWithMagnitude {
        VectorWithMagnitude {
            v: apply_vector_linear(t, self.v),
        }
    }

    /// Transforms in place. Mirrors `Transform(T)`.
    pub fn transform(&mut self, t: &Trsf2d) {
        self.v = apply_vector_linear(t, self.v);
    }
}

// -----------------------------------------------------------------------
// Helpers
// -----------------------------------------------------------------------

/// Applies only the linear (2×2) part of a `Trsf2d` to a vector,
/// omitting the translation (which must not move free vectors).
#[inline]
fn apply_vector_linear(t: &Trsf2d, v: Vec2d) -> Vec2d {
    Vec2d::new(
        t.m[0][0] * v.x + t.m[0][1] * v.y,
        t.m[1][0] * v.x + t.m[1][1] * v.y,
    )
}

// -----------------------------------------------------------------------
// Unit tests (internal)
// -----------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::precision;

    #[test]
    fn magnitude_of_unit_x() {
        let v = VectorWithMagnitude::new(1.0, 0.0);
        assert!((v.magnitude() - 1.0).abs() < precision::CONFUSION);
    }

    #[test]
    fn normalized_has_unit_magnitude() {
        let v = VectorWithMagnitude::new(3.0, 4.0);
        let n = v.normalized();
        assert!((n.magnitude() - 1.0).abs() < precision::CONFUSION);
    }

    #[test]
    fn crossed_is_signed_area() {
        let a = VectorWithMagnitude::new(1.0, 0.0);
        let b = VectorWithMagnitude::new(0.0, 1.0);
        assert!((a.crossed(&b) - 1.0).abs() < precision::CONFUSION);
        assert!((b.crossed(&a) + 1.0).abs() < precision::CONFUSION);
    }
}
