// occt-ref: gp_XYZ
// occt-ref: gp_XY

use std::ops::{Add, Div, Mul, Neg, Sub};

/// A 3D Cartesian coordinate triple `(x, y, z)`.
///
/// Mirrors OCCT's `gp_XYZ` class from the `TKMath` package.
/// `gp_XYZ` is the primitive coordinate type shared by `gp_Pnt`, `gp_Vec`,
/// and `gp_Dir`. It provides direct arithmetic and geometric operations on raw
/// coordinates without any semantic distinction between points and vectors.
// occt-ref: gp_XYZ
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct XYZ {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl XYZ {
    /// `gp_XYZ(X, Y, Z)` — construct from three Cartesian coordinates.
    #[inline]
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    /// Origin `(0, 0, 0)`.
    #[inline]
    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    /// Origin `(0, 0, 0)` — OCCT alias for `zero()`.
    #[inline]
    pub fn origin() -> Self {
        Self::zero()
    }

    // ── Accessors ──────────────────────────────────────────────────────────────

    /// `X()` — return the X coordinate.
    #[inline]
    pub fn x(&self) -> f64 {
        self.x
    }

    /// `Y()` — return the Y coordinate.
    #[inline]
    pub fn y(&self) -> f64 {
        self.y
    }

    /// `Z()` — return the Z coordinate.
    #[inline]
    pub fn z(&self) -> f64 {
        self.z
    }

    /// `Coord()` — return all three coordinates as a tuple `(x, y, z)`.
    #[inline]
    pub fn coord(&self) -> (f64, f64, f64) {
        (self.x, self.y, self.z)
    }

    /// `SetCoord(X, Y, Z)` — replace all three coordinates in-place.
    #[inline]
    pub fn set_coord(&mut self, x: f64, y: f64, z: f64) {
        self.x = x;
        self.y = y;
        self.z = z;
    }

    /// `SetX(X)` — replace the X coordinate.
    #[inline]
    pub fn set_x(&mut self, x: f64) {
        self.x = x;
    }

    /// `SetY(Y)` — replace the Y coordinate.
    #[inline]
    pub fn set_y(&mut self, y: f64) {
        self.y = y;
    }

    /// `SetZ(Z)` — replace the Z coordinate.
    #[inline]
    pub fn set_z(&mut self, z: f64) {
        self.z = z;
    }

    // ── In-place arithmetic (OCCT-style) ──────────────────────────────────────

    /// `Add(Other)` — add `other` to `self` in-place.
    #[inline]
    pub fn add(&mut self, other: &XYZ) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }

    /// `Subtract(Other)` — subtract `other` from `self` in-place.
    #[inline]
    pub fn subtract(&mut self, other: &XYZ) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }

    /// `Cross(Other)` — replace `self` with `self × other` in-place.
    #[inline]
    pub fn cross(&mut self, other: &XYZ) {
        let nx = self.y * other.z - self.z * other.y;
        let ny = self.z * other.x - self.x * other.z;
        let nz = self.x * other.y - self.y * other.x;
        self.x = nx;
        self.y = ny;
        self.z = nz;
    }

    /// `Multiply(Scalar)` — scale `self` by `s` in-place.
    #[inline]
    pub fn multiply(&mut self, s: f64) {
        self.x *= s;
        self.y *= s;
        self.z *= s;
    }

    /// `Divide(Scalar)` — divide `self` by `s` in-place.
    #[inline]
    pub fn divide(&mut self, s: f64) {
        self.x /= s;
        self.y /= s;
        self.z /= s;
    }

    /// `Normalize()` — normalize `self` to unit length in-place.
    ///
    /// No-op if the modulus is zero.
    #[inline]
    pub fn normalize(&mut self) {
        let m = self.modulus();
        if m != 0.0 {
            self.x /= m;
            self.y /= m;
            self.z /= m;
        }
    }

    /// `Reverse()` — negate all components in-place.
    #[inline]
    pub fn reverse(&mut self) {
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
    }

    // ── Non-mutating arithmetic (OCCT-style, returns new value) ───────────────

    /// `Added(Other)` — component-wise addition, returns new `XYZ`.
    #[inline]
    pub fn added(&self, other: &XYZ) -> XYZ {
        XYZ::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }

    /// `Subtracted(Other)` — component-wise subtraction, returns new `XYZ`.
    #[inline]
    pub fn subtracted(&self, other: &XYZ) -> XYZ {
        XYZ::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }

    /// `Crossed(Other)` — cross product `self × other`, returns new `XYZ`.
    #[inline]
    pub fn crossed(&self, other: &XYZ) -> XYZ {
        XYZ::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    /// `Multiplied(s)` — scalar multiplication, returns new `XYZ`.
    #[inline]
    pub fn multiplied(&self, s: f64) -> XYZ {
        XYZ::new(self.x * s, self.y * s, self.z * s)
    }

    /// Legacy alias for `multiplied` (keeps old callers working).
    #[inline]
    pub fn scale(&self, s: f64) -> XYZ {
        self.multiplied(s)
    }

    /// `Divided(s)` — scalar division, returns new `XYZ`.
    #[inline]
    pub fn divided(&self, s: f64) -> XYZ {
        XYZ::new(self.x / s, self.y / s, self.z / s)
    }

    /// `Normalized()` — return a unit-length copy. Panics on zero vector.
    #[inline]
    pub fn normalized(&self) -> XYZ {
        let m = self.modulus();
        debug_assert!(m != 0.0, "XYZ::normalized called on zero vector");
        self.multiplied(1.0 / m)
    }

    /// `Reversed()` — return a copy with all components negated.
    #[inline]
    pub fn reversed(&self) -> XYZ {
        XYZ::new(-self.x, -self.y, -self.z)
    }

    // ── Scalar products and magnitudes ────────────────────────────────────────

    /// `Dot(Other)` — scalar dot product `self · other`.
    #[inline]
    pub fn dot(&self, other: &XYZ) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// `CrossMagnitude(Other)` — magnitude of `self × other`.
    #[inline]
    pub fn cross_magnitude(&self, other: &XYZ) -> f64 {
        self.crossed(other).modulus()
    }

    /// `CrossSquareMagnitude(Other)` — squared magnitude of `self × other`.
    #[inline]
    pub fn cross_square_magnitude(&self, other: &XYZ) -> f64 {
        self.crossed(other).square_modulus()
    }

    /// `Modulus()` — Euclidean length `sqrt(x² + y² + z²)`.
    #[inline]
    pub fn modulus(&self) -> f64 {
        self.square_modulus().sqrt()
    }

    /// `SquareModulus()` — squared Euclidean length `x² + y² + z²`.
    #[inline]
    pub fn square_modulus(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// `IsEqual(Other, tol)` — true when the Euclidean distance between `self`
    /// and `other` is less than or equal to `tol`.
    #[inline]
    pub fn is_equal(&self, other: &XYZ, tol: f64) -> bool {
        self.subtracted(other).modulus() <= tol
    }
}

// ── std::ops impls ────────────────────────────────────────────────────────────

impl Add for XYZ {
    type Output = XYZ;
    fn add(self, rhs: XYZ) -> XYZ {
        XYZ::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for XYZ {
    type Output = XYZ;
    fn sub(self, rhs: XYZ) -> XYZ {
        XYZ::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Mul<f64> for XYZ {
    type Output = XYZ;
    fn mul(self, s: f64) -> XYZ {
        XYZ::new(self.x * s, self.y * s, self.z * s)
    }
}

impl Div<f64> for XYZ {
    type Output = XYZ;
    fn div(self, s: f64) -> XYZ {
        XYZ::new(self.x / s, self.y / s, self.z / s)
    }
}

impl Neg for XYZ {
    type Output = XYZ;
    fn neg(self) -> XYZ {
        XYZ::new(-self.x, -self.y, -self.z)
    }
}

// ── XY ────────────────────────────────────────────────────────────────────────

/// A 2D Cartesian coordinate pair `(x, y)`.
///
/// Mirrors OCCT's `gp_XY` class from the `TKMath` package.
/// `gp_XY` is the raw arithmetic counterpart of `gp_Pnt2d` / `gp_Vec2d`; it
/// holds a plain `(x, y)` pair and provides the full OCCT arithmetic surface.
// occt-ref: gp_XY
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct XY {
    pub x: f64,
    pub y: f64,
}

impl XY {
    /// `gp_XY(X, Y)` — construct from two coordinates.
    #[inline]
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    /// Zero coordinate pair — origin of the 2D parameter space.
    #[inline]
    pub fn zero() -> Self {
        Self::new(0.0, 0.0)
    }

    /// `Added(Right)` — component-wise addition, returns new `XY`.
    #[inline]
    pub fn add(&self, other: &XY) -> XY {
        XY::new(self.x + other.x, self.y + other.y)
    }

    /// `Subtracted(Right)` — component-wise subtraction, returns new `XY`.
    #[inline]
    pub fn sub(&self, other: &XY) -> XY {
        XY::new(self.x - other.x, self.y - other.y)
    }

    /// `Multiplied(Scalar)` — scalar multiplication, returns new `XY`.
    #[inline]
    pub fn scale(&self, s: f64) -> XY {
        XY::new(self.x * s, self.y * s)
    }

    /// `Dot(Right)` — scalar dot product `x₁·x₂ + y₁·y₂`.
    #[inline]
    pub fn dot(&self, other: &XY) -> f64 {
        self.x * other.x + self.y * other.y
    }

    /// `CrossMagnitude(Right)` — signed z-component of the 3-D cross product:
    /// `x₁·y₂ − y₁·x₂`.
    #[inline]
    pub fn crossed(&self, other: &XY) -> f64 {
        self.x * other.y - self.y * other.x
    }

    /// `Modulus()` — Euclidean length `sqrt(x² + y²)`.
    #[inline]
    pub fn modulus(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    /// `Normalized()` — returns a unit-length copy, or `None` if the vector
    /// is null (modulus is zero).
    #[inline]
    pub fn normalize(&self) -> Option<XY> {
        let m = self.modulus();
        if m == 0.0 {
            None
        } else {
            Some(self.scale(1.0 / m))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn near(a: f64, b: f64, tol: f64) {
        assert!(
            (a - b).abs() <= tol,
            "expected {a} ~= {b} within {tol} (diff {})",
            (a - b).abs()
        );
    }

    const TOL: f64 = 1.0e-12;

    // ── XYZ tests ──────────────────────────────────────────────────────────────

    #[test]
    fn xyz_new_and_zero() {
        let v = XYZ::new(1.0, 2.0, 3.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);
        let o = XYZ::zero();
        assert_eq!(o.x, 0.0);
        assert_eq!(o.y, 0.0);
        assert_eq!(o.z, 0.0);
    }

    #[test]
    fn xyz_added_subtracted_scaled() {
        let a = XYZ::new(1.0, 2.0, 3.0);
        let b = XYZ::new(4.0, 5.0, 6.0);
        let s = a.added(&b);
        assert_eq!(s.x, 5.0);
        assert_eq!(s.y, 7.0);
        assert_eq!(s.z, 9.0);
        let d = a.subtracted(&b);
        assert_eq!(d.x, -3.0);
        assert_eq!(d.y, -3.0);
        assert_eq!(d.z, -3.0);
        let m = a.scale(2.0);
        assert_eq!(m.x, 2.0);
        assert_eq!(m.y, 4.0);
        assert_eq!(m.z, 6.0);
    }

    #[test]
    fn xyz_dot() {
        let a = XYZ::new(1.0, 0.0, 0.0);
        let b = XYZ::new(0.0, 1.0, 0.0);
        near(a.dot(&b), 0.0, TOL);
        near(a.dot(&a), 1.0, TOL);
        let c = XYZ::new(1.0, 2.0, 3.0);
        let d = XYZ::new(4.0, 5.0, 6.0);
        near(c.dot(&d), 32.0, TOL);
    }

    #[test]
    fn xyz_crossed() {
        let x = XYZ::new(1.0, 0.0, 0.0);
        let y = XYZ::new(0.0, 1.0, 0.0);
        let z = x.crossed(&y);
        near(z.x, 0.0, TOL);
        near(z.y, 0.0, TOL);
        near(z.z, 1.0, TOL);
        // antisymmetry
        let z2 = y.crossed(&x);
        near(z2.z, -1.0, TOL);
    }

    #[test]
    fn xyz_modulus_and_square_modulus() {
        let v = XYZ::new(3.0, 4.0, 0.0);
        near(v.modulus(), 5.0, TOL);
        near(v.square_modulus(), 25.0, TOL);
    }

    #[test]
    fn xyz_normalize() {
        let v = XYZ::new(0.0, 3.0, 4.0);
        let n = v.normalized();
        near(n.x, 0.0, TOL);
        near(n.y, 0.6, TOL);
        near(n.z, 0.8, TOL);
        near(n.modulus(), 1.0, TOL);
    }

    #[test]
    fn xyz_is_equal() {
        let a = XYZ::new(1.0, 2.0, 3.0);
        let b = XYZ::new(1.0 + 1e-8, 2.0, 3.0);
        assert!(a.is_equal(&b, 1e-7));
        assert!(!a.is_equal(&b, 1e-9));
        assert!(a.is_equal(&a, 0.0));
    }

    // ── XY tests ───────────────────────────────────────────────────────────────

    #[test]
    fn xy_new_and_zero() {
        let p = XY::new(3.0, 4.0);
        assert_eq!(p.x, 3.0);
        assert_eq!(p.y, 4.0);
        let o = XY::zero();
        assert_eq!(o.x, 0.0);
        assert_eq!(o.y, 0.0);
    }

    #[test]
    fn xy_add_sub_scale() {
        let a = XY::new(1.0, 2.0);
        let b = XY::new(3.0, 4.0);
        let s = a.add(&b);
        assert_eq!(s.x, 4.0);
        assert_eq!(s.y, 6.0);
        let d = a.sub(&b);
        assert_eq!(d.x, -2.0);
        assert_eq!(d.y, -2.0);
        let m = a.scale(3.0);
        assert_eq!(m.x, 3.0);
        assert_eq!(m.y, 6.0);
    }

    #[test]
    fn xy_dot() {
        let a = XY::new(1.0, 2.0);
        let b = XY::new(3.0, 4.0);
        near(a.dot(&b), 11.0, TOL);
    }

    #[test]
    fn xy_crossed() {
        let a = XY::new(1.0, 0.0);
        let b = XY::new(0.0, 1.0);
        near(a.crossed(&b), 1.0, TOL);
        near(b.crossed(&a), -1.0, TOL);
    }

    #[test]
    fn xy_modulus() {
        let p = XY::new(3.0, 4.0);
        near(p.modulus(), 5.0, TOL);
    }

    #[test]
    fn xy_normalize() {
        let v = XY::new(3.0, 4.0);
        let n = v.normalize().expect("non-zero vector should normalize");
        near(n.x, 0.6, TOL);
        near(n.y, 0.8, TOL);
        near(n.modulus(), 1.0, TOL);
        // null vector returns None
        assert!(XY::zero().normalize().is_none());
    }
}
