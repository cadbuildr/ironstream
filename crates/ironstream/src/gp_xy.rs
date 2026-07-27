//! `gp_XY` — 2D Cartesian coordinate pair, mirroring OpenCascade's `gp_XY`
//! class from the TKMath package.
//!
//! `gp_XY` is the raw arithmetic counterpart of `gp_Pnt2d` / `gp_Vec2d`; it
//! holds a plain `(x, y)` pair and provides the full OCCT arithmetic surface:
//! addition, subtraction, scalar multiply/divide, dot product, 2-D cross
//! magnitude, modulus, normalization, reversal, and coordinate equality.

// occt-ref: gp_XY

/// A 2D Cartesian coordinate pair `(x, y)`.
///
/// Mirrors OCCT's `gp_XY` class exactly: no unit-length invariant, no notion
/// of "point" vs. "vector" — just raw arithmetic on two `f64` values.
// occt-ref: gp_XY
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct XY {
    pub x: f64,
    pub y: f64,
}

impl XY {
    // ── Construction ──────────────────────────────────────────────────────────

    /// `gp_XY(X, Y)` — construct from two coordinates.
    #[inline]
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    /// Zero coordinate pair — origin of the 2D parameter space.
    #[inline]
    pub const fn zero() -> Self {
        Self::new(0.0, 0.0)
    }

    // ── Mutating setters ──────────────────────────────────────────────────────

    /// `SetCoord(Index, Xi)` — set a single coordinate by index (1 = X, 2 = Y).
    ///
    /// Panics when `index` is not 1 or 2, matching OCCT's
    /// `Standard_OutOfRange`.
    #[inline]
    pub fn set_coord_index(&mut self, index: usize, value: f64) {
        match index {
            1 => self.x = value,
            2 => self.y = value,
            _ => panic!("gp_XY::SetCoord — index {index} out of range [1, 2]"),
        }
    }

    /// `SetCoord(X, Y)` — set both coordinates at once.
    #[inline]
    pub fn set_coord(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
    }

    /// `SetX(X)`.
    #[inline]
    pub fn set_x(&mut self, x: f64) {
        self.x = x;
    }

    /// `SetY(Y)`.
    #[inline]
    pub fn set_y(&mut self, y: f64) {
        self.y = y;
    }

    // ── Accessors ─────────────────────────────────────────────────────────────

    /// `Coord(Index)` — retrieve a single coordinate by index (1 = X, 2 = Y).
    #[inline]
    pub fn coord_index(&self, index: usize) -> f64 {
        match index {
            1 => self.x,
            2 => self.y,
            _ => panic!("gp_XY::Coord — index {index} out of range [1, 2]"),
        }
    }

    /// `Coord(X, Y)` — retrieve both coordinates as a tuple.
    #[inline]
    pub fn coord(&self) -> (f64, f64) {
        (self.x, self.y)
    }

    /// `X()`.
    #[inline]
    pub fn x(&self) -> f64 {
        self.x
    }

    /// `Y()`.
    #[inline]
    pub fn y(&self) -> f64 {
        self.y
    }

    // ── Modulus ───────────────────────────────────────────────────────────────

    /// `Modulus()` — Euclidean length `sqrt(x² + y²)`.
    #[inline]
    pub fn modulus(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    /// `SquareModulus()` — `x² + y²`.
    #[inline]
    pub fn square_modulus(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }

    // ── Arithmetic ────────────────────────────────────────────────────────────

    /// `Added(Right)` / `operator+` — component-wise addition.
    #[inline]
    pub fn added(&self, other: &XY) -> XY {
        XY::new(self.x + other.x, self.y + other.y)
    }

    /// In-place `Add`.
    #[inline]
    pub fn add_assign(&mut self, other: &XY) {
        self.x += other.x;
        self.y += other.y;
    }

    /// `Subtracted(Right)` / `operator-` — component-wise subtraction.
    #[inline]
    pub fn subtracted(&self, other: &XY) -> XY {
        XY::new(self.x - other.x, self.y - other.y)
    }

    /// In-place `Subtract`.
    #[inline]
    pub fn subtract_assign(&mut self, other: &XY) {
        self.x -= other.x;
        self.y -= other.y;
    }

    /// `Multiplied(Scalar)` — scalar multiplication.
    #[inline]
    pub fn multiplied(&self, scalar: f64) -> XY {
        XY::new(self.x * scalar, self.y * scalar)
    }

    /// In-place `Multiply(Scalar)`.
    #[inline]
    pub fn multiply(&mut self, scalar: f64) {
        self.x *= scalar;
        self.y *= scalar;
    }

    /// `Multiplied(Matrix)` — component-wise product with a 2×2 matrix
    /// `[[a, b], [c, d]]`:  result = Matrix * self  (column vector on right).
    ///
    /// In OCCT `gp_XY::Multiplied(gp_Mat2d)` computes  `M * V`.
    #[inline]
    pub fn multiplied_mat(&self, m: [[f64; 2]; 2]) -> XY {
        XY::new(
            m[0][0] * self.x + m[0][1] * self.y,
            m[1][0] * self.x + m[1][1] * self.y,
        )
    }

    /// `Divided(Scalar)` — scalar division.
    #[inline]
    pub fn divided(&self, scalar: f64) -> XY {
        let inv = 1.0 / scalar;
        XY::new(self.x * inv, self.y * inv)
    }

    /// In-place `Divide(Scalar)`.
    #[inline]
    pub fn divide(&mut self, scalar: f64) {
        let inv = 1.0 / scalar;
        self.x *= inv;
        self.y *= inv;
    }

    /// `Reversed()` — negate both components.
    #[inline]
    pub fn reversed(&self) -> XY {
        XY::new(-self.x, -self.y)
    }

    /// In-place `Reverse`.
    #[inline]
    pub fn reverse(&mut self) {
        self.x = -self.x;
        self.y = -self.y;
    }

    // ── Dot & cross ───────────────────────────────────────────────────────────

    /// `Dot(Right)` — scalar dot product `x₁·x₂ + y₁·y₂`.
    #[inline]
    pub fn dot(&self, other: &XY) -> f64 {
        self.x * other.x + self.y * other.y
    }

    /// `CrossMagnitude(Right)` — magnitude of the 2-D cross product
    /// (i.e. the z-component of the 3-D cross): `|x₁·y₂ − y₁·x₂|`.
    #[inline]
    pub fn cross_magnitude(&self, other: &XY) -> f64 {
        (self.x * other.y - self.y * other.x).abs()
    }

    /// `CrossSquareMagnitude(Right)` — square of `CrossMagnitude`.
    #[inline]
    pub fn cross_square_magnitude(&self, other: &XY) -> f64 {
        let c = self.x * other.y - self.y * other.x;
        c * c
    }

    // ── Normalize ─────────────────────────────────────────────────────────────

    /// `Normalized()` — return a unit-length copy.
    ///
    /// Returns `(1, 0)` for a zero vector, mirroring the kernel's "safe
    /// fallback" convention (OCCT raises `Standard_ConstructionError`).
    #[inline]
    pub fn normalized(&self) -> XY {
        let m = self.modulus();
        if m < 1.0e-300 {
            XY::new(1.0, 0.0)
        } else {
            self.multiplied(1.0 / m)
        }
    }

    /// In-place `Normalize`.
    #[inline]
    pub fn normalize(&mut self) {
        let n = self.normalized();
        self.x = n.x;
        self.y = n.y;
    }

    // ── Predicate ─────────────────────────────────────────────────────────────

    /// `IsEqual(Other, Tolerance)` — true when both `|x₁-x₂|` and `|y₁-y₂|`
    /// are at most `tolerance` (OCCT checks each component separately).
    #[inline]
    pub fn is_equal(&self, other: &XY, tolerance: f64) -> bool {
        (self.x - other.x).abs() <= tolerance && (self.y - other.y).abs() <= tolerance
    }
}

// ── std::ops trait impls ──────────────────────────────────────────────────────

impl std::ops::Add for XY {
    type Output = XY;
    #[inline]
    fn add(self, rhs: XY) -> XY {
        self.added(&rhs)
    }
}

impl std::ops::Sub for XY {
    type Output = XY;
    #[inline]
    fn sub(self, rhs: XY) -> XY {
        self.subtracted(&rhs)
    }
}

impl std::ops::Mul<f64> for XY {
    type Output = XY;
    #[inline]
    fn mul(self, s: f64) -> XY {
        self.multiplied(s)
    }
}

impl std::ops::Div<f64> for XY {
    type Output = XY;
    #[inline]
    fn div(self, s: f64) -> XY {
        self.divided(s)
    }
}

impl std::ops::Neg for XY {
    type Output = XY;
    #[inline]
    fn neg(self) -> XY {
        self.reversed()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::precision::CONFUSION;

    #[test]
    fn constructor_and_accessors() {
        let p = XY::new(3.0, 4.0);
        assert_eq!(p.x(), 3.0);
        assert_eq!(p.y(), 4.0);
        let (x, y) = p.coord();
        assert_eq!(x, 3.0);
        assert_eq!(y, 4.0);
    }

    #[test]
    fn modulus_3_4() {
        let p = XY::new(3.0, 4.0);
        assert!((p.modulus() - 5.0).abs() < CONFUSION);
        assert!((p.square_modulus() - 25.0).abs() < CONFUSION);
    }

    #[test]
    fn dot_product() {
        let a = XY::new(1.0, 2.0);
        let b = XY::new(3.0, 4.0);
        assert!((a.dot(&b) - 11.0).abs() < CONFUSION);
    }

    #[test]
    fn cross_magnitude() {
        let a = XY::new(1.0, 0.0);
        let b = XY::new(0.0, 1.0);
        assert!((a.cross_magnitude(&b) - 1.0).abs() < CONFUSION);
    }

    #[test]
    fn is_equal_within_tolerance() {
        let a = XY::new(1.0, 2.0);
        let b = XY::new(1.0 + 1e-8, 2.0 - 1e-8);
        assert!(a.is_equal(&b, 1e-7));
        assert!(!a.is_equal(&b, 1e-9));
    }
}
