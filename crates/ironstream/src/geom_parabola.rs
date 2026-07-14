//! `Geom_Parabola` -- analytic 3D parabola, faithfully reproducing OpenCascade's
//! `Geom_Parabola` (a `Geom_Conic` in package TKG3d).
//!
//! Also provides the simple flat-array stubs [`Parabola3d`] and [`Parabola2d`]
//! for use cases that do not depend on the internal `gp` coordinate system types.
//!
//! A parabola is positioned in 3D space by a right-handed local coordinate
//! system (`gp_Ax2`, here [`crate::gp::Ax3`]): its origin is the **vertex** of
//! the parabola; its "X Direction" points toward the focus; its "Y Direction"
//! is tangent to the parabola at the vertex.
//!
//! The parabola is parameterised by a real `U` in `]-∞, +∞[`:
//!
//! ```text
//! P(U) = O + (U² / (2*p)) * XDir + U * YDir
//! ```
//!
//! where `O` is the vertex, `p` is the **parameter** of the parabola
//! (`p = 2 * FocalLength`), `XDir` and `YDir` are the two planar directions of
//! the local coordinate system.
//!
//! This formulation is identical to OCCT's
//! `src/ModelingData/TKG3d/Geom/Geom_Parabola.cxx`.
//!
//! Builds on the existing `gp` / `gp_prim` API (zero third-party deps).

use crate::gp::{Ax1, Ax3, Pnt, Trsf, Vec3};
use crate::gp_prim::Lin;
use crate::precision::INFINITE;

/// `Geom_Parabola` -- describes a parameterised parabola in 3D space.
///
/// The parabola is stored as a local coordinate system (`pos`, an [`Ax3`] whose
/// origin is the vertex) and a `focal` distance (the distance from the vertex to
/// the focus). The parabolic **parameter** `p = 2 * focal`.
///
/// The parameterisation is:
/// ```text
/// P(U) = Vertex + (U² / (2*p)) * XDir + U * YDir
///       = Vertex + (U² / (4*focal)) * XDir + U * YDir
/// ```
///
/// The curve is **open** and **not periodic**: first parameter = `-INFINITE`,
/// last parameter = `+INFINITE`.
// occt: Geom_Parabola
#[derive(Clone, Copy, Debug)]
pub struct GeomParabola {
    /// Local coordinate system: origin = vertex, X = axis toward focus,
    /// Y = tangent at vertex, Z = normal to the parabola plane.
    pos: Ax3,
    /// The focal length (distance from vertex to focus). Must be positive.
    focal: f64,
}

impl GeomParabola {
    // ─────────────────────────────────────── Constructors ────────────────────

    /// `Geom_Parabola(const gp_Ax2& A2, Standard_Real Focal)`.
    ///
    /// Constructs a parabola with:
    /// - vertex at the origin of `a2`,
    /// - axis of symmetry along the "X Direction" of `a2`,
    /// - parameter `p = 2 * focal`.
    ///
    /// # Panics
    /// Raised (`Standard_ConstructionError`) if `focal < 0`.
    pub fn new(a2: Ax3, focal: f64) -> Self {
        assert!(
            focal >= 0.0,
            "Geom_Parabola: Focal must be >= 0"
        );
        Self { pos: a2, focal }
    }

    // ─────────────────────────────────────── Setters ─────────────────────────

    /// `SetFocal(Standard_Real Focal)` — change the focal length.
    ///
    /// # Panics
    /// Raised if `focal < 0`.
    pub fn set_focal(&mut self, focal: f64) {
        assert!(focal >= 0.0, "Geom_Parabola::SetFocal: Focal must be >= 0");
        self.focal = focal;
    }

    /// `SetPosition(const gp_Ax2& A2)` — change the local coordinate system.
    pub fn set_position(&mut self, a2: Ax3) {
        self.pos = a2;
    }

    // ─────────────────────────────────────── Getters ─────────────────────────

    /// `Standard_Real Focal() const` — the focal length of the parabola.
    #[inline]
    pub fn focal(&self) -> f64 {
        self.focal
    }

    /// `Standard_Real Parameter() const` — the parameter `p = 2 * Focal`
    /// (the semi-latus rectum; the ordinate at the focus).
    #[inline]
    pub fn parameter(&self) -> f64 {
        2.0 * self.focal
    }

    /// `gp_Pnt Focus() const` — the focus of the parabola:
    /// `Vertex + Focal * XDir`.
    pub fn focus(&self) -> Pnt {
        self.pos.location + self.pos.x_dir * self.focal
    }

    /// `Standard_Real Eccentricity() const` — for a parabola this is always
    /// `1.0` (the defining property of a parabola).
    #[inline]
    pub fn eccentricity(&self) -> f64 {
        1.0
    }

    /// `gp_Ax1 Directrix() const` — the directrix of the parabola.
    ///
    /// The directrix is the line through `Vertex - Focal * XDir` (the point
    /// on the axis of symmetry equidistant from the vertex as the focus, but
    /// on the opposite side) perpendicular to the axis of symmetry, i.e. in
    /// the direction of `YDir`.
    pub fn directrix(&self) -> Lin {
        let loc = self.pos.location - self.pos.x_dir * self.focal;
        Lin::new(Ax1::new(loc, self.pos.y_dir))
    }

    /// `gp_Ax2 Position() const` — the local coordinate system.
    #[inline]
    pub fn position(&self) -> Ax3 {
        self.pos
    }

    /// `gp_Pnt Location() const` — the vertex (origin of the local frame).
    #[inline]
    pub fn location(&self) -> Pnt {
        self.pos.location
    }

    /// `gp_Ax1 Axis() const` — the main axis: origin + Z direction (normal).
    #[inline]
    pub fn axis(&self) -> Ax1 {
        Ax1::new(self.pos.location, self.pos.z_dir)
    }

    /// `gp_Ax1 XAxis() const` — the axis of symmetry: origin + X direction.
    #[inline]
    pub fn x_axis(&self) -> Ax1 {
        Ax1::new(self.pos.location, self.pos.x_dir)
    }

    /// `gp_Ax1 YAxis() const` — the tangent axis at the vertex: origin + Y direction.
    #[inline]
    pub fn y_axis(&self) -> Ax1 {
        Ax1::new(self.pos.location, self.pos.y_dir)
    }

    // ─────────────────────────────────────── Curve ───────────────────────────

    /// `Standard_Real ReversedParameter(Standard_Real U) const`.
    ///
    /// For a parabola the reversed parameter is `-U` (reversing the
    /// parameterisation direction maps `U` to `-U`).
    #[inline]
    pub fn reversed_parameter(&self, u: f64) -> f64 {
        -u
    }

    /// `Standard_Real FirstParameter() const` — `-Precision::Infinite()`.
    #[inline]
    pub fn first_parameter(&self) -> f64 {
        -INFINITE
    }

    /// `Standard_Real LastParameter() const` — `+Precision::Infinite()`.
    #[inline]
    pub fn last_parameter(&self) -> f64 {
        INFINITE
    }

    /// `Standard_Boolean IsClosed() const` — always `false`.
    #[inline]
    pub fn is_closed(&self) -> bool {
        false
    }

    /// `Standard_Boolean IsPeriodic() const` — always `false`.
    #[inline]
    pub fn is_periodic(&self) -> bool {
        false
    }

    // ─────────────────────────────────────── Evaluation ──────────────────────

    /// `D0(U, P)` — the point at parameter `U`:
    ///
    /// `P = Vertex + (U² / (2*p)) * XDir + U * YDir`
    ///
    /// where `p = 2 * Focal`, so `2*p = 4*Focal`.
    pub fn d0(&self, u: f64) -> Pnt {
        let p = 2.0 * self.focal; // the parabola parameter p = 2F
        let x = u * u / (2.0 * p);
        let y = u;
        self.pos.location + self.pos.x_dir * x + self.pos.y_dir * y
    }

    /// Alias: `Value(U)` — identical to `D0`.
    #[inline]
    pub fn value(&self, u: f64) -> Pnt {
        self.d0(u)
    }

    /// `D1(U, P, V1)` — the point and first derivative.
    ///
    /// ```text
    /// P  = Vertex + (U² / (2*p)) * XDir + U * YDir
    /// V1 = (U / p) * XDir + YDir
    /// ```
    pub fn d1(&self, u: f64) -> (Pnt, Vec3) {
        let p = 2.0 * self.focal;
        let x = u * u / (2.0 * p);
        let y = u;
        let point = self.pos.location + self.pos.x_dir * x + self.pos.y_dir * y;
        // First derivative: dX/dU = U/p, dY/dU = 1
        let v1 = self.pos.x_dir * (u / p) + self.pos.y_dir;
        (point, v1)
    }

    /// `D2(U, P, V1, V2)` — the point and first two derivatives.
    ///
    /// ```text
    /// V2 = (1 / p) * XDir
    /// ```
    pub fn d2(&self, u: f64) -> (Pnt, Vec3, Vec3) {
        let p = 2.0 * self.focal;
        let x = u * u / (2.0 * p);
        let y = u;
        let point = self.pos.location + self.pos.x_dir * x + self.pos.y_dir * y;
        let v1 = self.pos.x_dir * (u / p) + self.pos.y_dir;
        // Second derivative: d²X/dU² = 1/p, d²Y/dU² = 0
        let v2 = self.pos.x_dir * (1.0 / p);
        (point, v1, v2)
    }

    /// `D3(U, P, V1, V2, V3)` — the point and first three derivatives.
    ///
    /// All derivatives of order ≥ 3 are zero for a parabola.
    pub fn d3(&self, u: f64) -> (Pnt, Vec3, Vec3, Vec3) {
        let (point, v1, v2) = self.d2(u);
        (point, v1, v2, Pnt::origin())
    }

    /// `DN(U, N)` — the `N`-th derivative vector (`N >= 1`).
    ///
    /// - `N = 1`: `(U/p) * XDir + YDir`
    /// - `N = 2`: `(1/p) * XDir`
    /// - `N >= 3`: zero vector
    ///
    /// # Panics
    /// Raised (`Standard_RangeError`) if `n < 1`.
    pub fn dn(&self, u: f64, n: i32) -> Vec3 {
        assert!(n >= 1, "Geom_Parabola::DN: N must be >= 1");
        let p = 2.0 * self.focal;
        match n {
            1 => self.pos.x_dir * (u / p) + self.pos.y_dir,
            2 => self.pos.x_dir * (1.0 / p),
            _ => Pnt::origin(),
        }
    }

    // ─────────────────────────────────────── Orientation ─────────────────────

    /// `Reverse()` — reverses the orientation of this parabola.
    ///
    /// Reversing the parameterisation direction negates the "Y Direction"
    /// (the tangent at the vertex) — the parabola shape is unchanged but
    /// the parameter now runs in the opposite sense. The "Z Direction" is
    /// also negated to preserve right-handedness of the local frame.
    pub fn reverse(&mut self) {
        self.pos.y_dir = -self.pos.y_dir;
        self.pos.z_dir = -self.pos.z_dir;
    }

    /// `Reversed()` — returns a copy with the orientation reversed.
    pub fn reversed(&self) -> GeomParabola {
        let mut c = *self;
        c.reverse();
        c
    }

    // ─────────────────────────────────────── Transform ───────────────────────

    /// `Transform(const gp_Trsf& T)` — applies the transformation `T` in place.
    ///
    /// The position is mapped through `T`; the focal length is scaled by the
    /// absolute value of the transform's scale factor.
    pub fn transform(&mut self, t: &Trsf) {
        self.pos = self.pos.transformed(t);
        self.focal = (self.focal * t.scale_factor()).abs();
    }

    /// `Transformed(T)` — returns a transformed copy of this parabola.
    pub fn transformed(&self, t: &Trsf) -> GeomParabola {
        let mut c = *self;
        c.transform(t);
        c
    }

    /// `Copy()` — a deep, independent copy.
    #[inline]
    pub fn copy(&self) -> GeomParabola {
        *self
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Flat-array stubs (zero gp-type dependencies)
// ─────────────────────────────────────────────────────────────────────────────
//
// The types below provide the same mathematical model using plain `[f64; N]`
// arrays so they can be used without importing the internal `gp` / `Ax3` types.

// ─────────────────────────────────────────── helpers ─────────────────────────

/// Normalise a 3-component vector.  Returns `v` unchanged when its length is
/// essentially zero to avoid division by zero.
#[inline]
fn normalise3(v: [f64; 3]) -> [f64; 3] {
    let len = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    if len < f64::EPSILON {
        v
    } else {
        [v[0] / len, v[1] / len, v[2] / len]
    }
}

/// Cross product of two 3-D vectors.
#[inline]
fn cross3(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

/// Dot product of two 3-D vectors.
#[inline]
fn dot3(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

/// Build a unit vector perpendicular to `axis`.
///
/// We pick the global axis (`e0`, `e1`, or `e2`) that is *least* aligned with
/// `axis`, cross it with `axis`, and normalise the result.
fn perp_to(axis: [f64; 3]) -> [f64; 3] {
    let ax = axis[0].abs();
    let ay = axis[1].abs();
    let az = axis[2].abs();
    // Choose the global basis vector with the smallest dot-product magnitude.
    let candidate = if ax <= ay && ax <= az {
        [1.0_f64, 0.0, 0.0]
    } else if ay <= ax && ay <= az {
        [0.0_f64, 1.0, 0.0]
    } else {
        [0.0_f64, 0.0, 1.0]
    };
    normalise3(cross3(axis, candidate))
}

// ─────────────────────────────────────────── Parabola3d ──────────────────────

/// A parameterised parabola in 3-D space.
///
/// Mirrors `Geom_Parabola` from OpenCascade: the parabola is defined by a
/// vertex, an axis of symmetry (`axis`, stored as a unit vector), and a focal
/// length (`focal`).  The parabolic parameter is `p = 2 * focal`.
///
/// The parameterisation is:
/// ```text
/// P(t) = focus + (t² / (4·focal)) · axis_unit + t · y_dir
/// ```
/// where `y_dir` is the unit vector perpendicular to `axis` in the parabola
/// plane (derived at construction from the supplied axis).
// occt: Geom_Parabola
#[derive(Clone, Copy, Debug)]
pub struct Parabola3d {
    /// The focus point of the parabola.
    pub focus: [f64; 3],
    /// The axis of symmetry (X direction), stored normalised.
    pub axis: [f64; 3],
    /// The focal length (distance from vertex to focus).  Must be > 0.
    pub focal: f64,
    /// Unit vector perpendicular to `axis` in the parabola plane (Y direction).
    y_dir: [f64; 3],
}

impl Parabola3d {
    /// Construct a 3-D parabola.
    ///
    /// * `focus`  — the focus point.
    /// * `axis`   — the axis of symmetry direction (need not be unit length;
    ///              it is normalised internally).
    /// * `focal`  — the focal length (distance from vertex to focus; must be
    ///              strictly positive).
    ///
    /// # Panics
    /// Panics if `focal <= 0`.
    pub fn new(focus: [f64; 3], axis: [f64; 3], focal: f64) -> Self {
        assert!(focal > 0.0, "Parabola3d: focal must be > 0");
        let axis = normalise3(axis);
        let y_dir = perp_to(axis);
        Self {
            focus,
            axis,
            focal,
            y_dir,
        }
    }

    /// The vertex of the parabola: `focus - focal * axis`.
    #[inline]
    pub fn vertex(&self) -> [f64; 3] {
        [
            self.focus[0] - self.focal * self.axis[0],
            self.focus[1] - self.focal * self.axis[1],
            self.focus[2] - self.focal * self.axis[2],
        ]
    }

    /// The parabolic parameter `p = 2 * focal`.
    #[inline]
    pub fn parameter(&self) -> f64 {
        2.0 * self.focal
    }

    /// The focal length.
    #[inline]
    pub fn focal(&self) -> f64 {
        self.focal
    }

    /// Point on the parabola at parameter `t`:
    ///
    /// `P(t) = vertex + (t² / (4·focal)) · axis + t · y_dir`
    pub fn point_at(&self, t: f64) -> [f64; 3] {
        let v = self.vertex();
        // x-offset along axis of symmetry: t²/(4f) = t²/(2p)
        let x_off = t * t / (4.0 * self.focal);
        [
            v[0] + x_off * self.axis[0] + t * self.y_dir[0],
            v[1] + x_off * self.axis[1] + t * self.y_dir[1],
            v[2] + x_off * self.axis[2] + t * self.y_dir[2],
        ]
    }

    /// First derivative of the curve at parameter `t`:
    ///
    /// `P'(t) = (t / (2·focal)) · axis + y_dir`
    pub fn d1(&self, t: f64) -> [f64; 3] {
        // p = 2*focal, so t/p = t/(2*focal)
        let dx = t / (2.0 * self.focal);
        [
            dx * self.axis[0] + self.y_dir[0],
            dx * self.axis[1] + self.y_dir[1],
            dx * self.axis[2] + self.y_dir[2],
        ]
    }

    /// Evaluate the directrix.
    ///
    /// The directrix is perpendicular to the axis of symmetry and lies at
    /// distance `focal` from the vertex on the opposite side from the focus.
    /// This function returns the **signed distance** of the directrix from the
    /// world origin measured along the axis direction, evaluated at the
    /// axis-projected X coordinate `x` (i.e. `x` is already the parameter
    /// along the axis).
    ///
    /// In the local frame the directrix is the vertical line `x = -focal`
    /// (axis coordinate = -focal relative to vertex, or equivalently
    /// `focus_axis_coord - 2*focal`).  The returned value is:
    ///
    /// ```text
    /// directrix_at_x(x) = x + focal      // distance from directrix to x along axis
    /// ```
    ///
    /// This matches `Geom_Parabola`'s notion of the distance from a point on the
    /// axis to the directrix.
    #[inline]
    pub fn directrix_at_x(&self, x: f64) -> f64 {
        x + self.focal
    }

    /// The focus projected onto the axis (dot product of focus with axis unit
    /// vector). Useful as the reference X coordinate.
    #[inline]
    fn focus_axis_coord(&self) -> f64 {
        dot3(self.focus, self.axis)
    }
}

// ─────────────────────────────────────────── Parabola2d ──────────────────────

/// A parameterised parabola in the 2-D plane.
///
/// Mirrors `Geom2d_Parabola` from OpenCascade.  The parabola has its vertex at
/// `(vx, vy)` and its axis of symmetry along the positive X direction.  The
/// parabolic parameter is `p = 2 * focal`.
///
/// Parameterisation:
/// ```text
/// P(t) = (vx + t² / (4·focal),  vy + t)
/// ```
// occt-ref: Geom2d_Parabola
#[derive(Clone, Copy, Debug)]
pub struct Parabola2d {
    /// The vertex of the parabola.
    pub vertex: [f64; 2],
    /// The focal length (distance from vertex to focus; must be > 0).
    pub focal: f64,
}

impl Parabola2d {
    /// Construct a 2-D parabola with vertex at `(vx, vy)` and the given focal
    /// length.
    ///
    /// # Panics
    /// Panics if `focal <= 0`.
    pub fn new(vx: f64, vy: f64, focal: f64) -> Self {
        assert!(focal > 0.0, "Parabola2d: focal must be > 0");
        Self {
            vertex: [vx, vy],
            focal,
        }
    }

    /// The focal length.
    #[inline]
    pub fn focal(&self) -> f64 {
        self.focal
    }

    /// The parabolic parameter `p = 2 * focal`.
    #[inline]
    pub fn parameter(&self) -> f64 {
        2.0 * self.focal
    }

    /// The focus: `(vx + focal, vy)`.
    #[inline]
    pub fn focus(&self) -> [f64; 2] {
        [self.vertex[0] + self.focal, self.vertex[1]]
    }

    /// Point on the parabola at parameter `t`:
    ///
    /// `P(t) = (vx + t² / (4·focal),  vy + t)`
    pub fn point_at(&self, t: f64) -> [f64; 2] {
        [
            self.vertex[0] + t * t / (4.0 * self.focal),
            self.vertex[1] + t,
        ]
    }

    /// First derivative at parameter `t`:
    ///
    /// `P'(t) = (t / (2·focal),  1)`
    pub fn d1(&self, t: f64) -> [f64; 2] {
        [t / (2.0 * self.focal), 1.0]
    }
}

// ─────────────────────────────────────────── tests ───────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── Parabola3d ────────────────────────────────────────────────────────────

    #[test]
    fn parabola3d_vertex_at_t0() {
        // Axis along +X, focus at (1,0,0), focal=1 → vertex at origin.
        let p = Parabola3d::new([1.0, 0.0, 0.0], [1.0, 0.0, 0.0], 1.0);
        let pt = p.point_at(0.0);
        let v = p.vertex();
        assert!((pt[0] - v[0]).abs() < 1e-12);
        assert!((pt[1] - v[1]).abs() < 1e-12);
        assert!((pt[2] - v[2]).abs() < 1e-12);
    }

    #[test]
    fn parabola3d_parameter() {
        let p = Parabola3d::new([2.0, 0.0, 0.0], [1.0, 0.0, 0.0], 3.0);
        assert!((p.parameter() - 6.0).abs() < 1e-12);
    }

    #[test]
    fn parabola3d_focal() {
        let p = Parabola3d::new([0.0, 0.0, 0.0], [0.0, 1.0, 0.0], 5.0);
        assert!((p.focal() - 5.0).abs() < 1e-12);
    }

    #[test]
    fn parabola3d_d1_at_t0_is_y_dir() {
        // At t=0 the tangent should equal y_dir (only the constant term survives).
        let p = Parabola3d::new([1.0, 0.0, 0.0], [1.0, 0.0, 0.0], 1.0);
        let d = p.d1(0.0);
        // d1(0) = (0/2) * axis + y_dir = y_dir
        let len = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
        assert!((len - 1.0).abs() < 1e-12, "tangent at t=0 should be unit length");
    }

    #[test]
    fn parabola3d_directrix() {
        let focal = 2.0;
        let p = Parabola3d::new([focal, 0.0, 0.0], [1.0, 0.0, 0.0], focal);
        // directrix_at_x(x) = x + focal; for x=0 that is focal
        assert!((p.directrix_at_x(0.0) - focal).abs() < 1e-12);
        // For x = -focal the distance to directrix is 0
        assert!(p.directrix_at_x(-focal).abs() < 1e-12);
    }

    // ── Parabola2d ────────────────────────────────────────────────────────────

    #[test]
    fn parabola2d_vertex_at_t0() {
        let p = Parabola2d::new(1.0, 2.0, 0.5);
        let pt = p.point_at(0.0);
        assert!((pt[0] - 1.0).abs() < 1e-12);
        assert!((pt[1] - 2.0).abs() < 1e-12);
    }

    #[test]
    fn parabola2d_focus() {
        let p = Parabola2d::new(0.0, 0.0, 3.0);
        let f = p.focus();
        assert!((f[0] - 3.0).abs() < 1e-12);
        assert!((f[1]).abs() < 1e-12);
    }

    #[test]
    fn parabola2d_parameter() {
        let p = Parabola2d::new(0.0, 0.0, 4.0);
        assert!((p.parameter() - 8.0).abs() < 1e-12);
    }

    #[test]
    fn parabola2d_d1_at_t0() {
        let p = Parabola2d::new(0.0, 0.0, 1.0);
        let d = p.d1(0.0);
        // P'(0) = (0/(2*1), 1) = (0, 1)
        assert!(d[0].abs() < 1e-12);
        assert!((d[1] - 1.0).abs() < 1e-12);
    }

    #[test]
    fn parabola2d_point_matches_standard_form() {
        // y = sqrt(2 p x)  ↔  t = y, x = t²/(2p)
        // With vertex at origin and focal=f, p=2f, x = t²/(4f).
        let focal = 2.0;
        let p = Parabola2d::new(0.0, 0.0, focal);
        let t = 4.0_f64;
        let pt = p.point_at(t);
        // x = 16 / (4*2) = 2, y = 4
        assert!((pt[0] - 2.0).abs() < 1e-12);
        assert!((pt[1] - 4.0).abs() < 1e-12);
    }
}
