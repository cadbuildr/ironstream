//! Lightweight stub types mirroring the four analytic 2D conic classes from
//! OpenCascade (`Geom2d_Circle`, `Geom2d_Ellipse`, `Geom2d_Hyperbola`,
//! `Geom2d_Parabola`).
//!
//! These stubs use only `std` (`f64` trigonometric / hyperbolic functions) and
//! carry no external-crate dependencies.  They expose the minimal surface asked
//! for by the IronStream OCCT reimplementation layer and are intended to be
//! fleshed out incrementally as higher-level algorithms are ported.
//!
//! # Coordinate convention
//!
//! All geometry is expressed in a flat `[f64; 2]` array `[x, y]` to keep the
//! API dependency-free.  The full frame-aware types (`Ax22d2`, `Pnt2d`, …) live
//! in `geom2d_circle` / `gp2d`; these stubs intentionally avoid that coupling
//! so they can be compiled and tested in isolation.

use std::f64::consts::PI;

// ─────────────────────────────────────────────────────────────────────────────
// Circle2dGeom
// ─────────────────────────────────────────────────────────────────────────────

/// Stub for `Geom2d_Circle` — an analytic 2D circle parameterised by angle `U`.
///
/// The parameterisation is the standard unit-circle scaled by `radius`:
/// ```text
/// P(U) = [center[0] + radius * cos(U),
///          center[1] + radius * sin(U)]
/// ```
/// The curve is closed and periodic with period `2π`.
// occt-ref: Geom2d_Circle
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Circle2dGeom {
    /// Centre of the circle (`gp_Pnt2d` / `Location()`).
    pub center: [f64; 2],
    /// Radius of the circle.  Must be non-negative.
    pub radius: f64,
}

impl Circle2dGeom {
    /// `Geom2d_Circle(cx, cy, r)` — construct a circle centred at `(cx, cy)`
    /// with the given `r`adius.
    ///
    /// # Panics
    /// Panics (`Standard_ConstructionError`) if `r < 0`.
    #[inline]
    pub fn new(cx: f64, cy: f64, r: f64) -> Self {
        assert!(r >= 0.0, "Circle2dGeom::new: radius must be >= 0");
        Self { center: [cx, cy], radius: r }
    }

    /// `D0(U)` — the point at parameter `U`:
    /// `P = [cx + r*cos(U), cy + r*sin(U)]`.
    #[inline]
    pub fn point_at(&self, u: f64) -> [f64; 2] {
        let (s, c) = u.sin_cos();
        [self.center[0] + self.radius * c, self.center[1] + self.radius * s]
    }

    /// `D1(U)` — the first derivative (tangent) vector at parameter `U`:
    /// `V1 = [-r*sin(U), r*cos(U)]`.
    #[inline]
    pub fn d1(&self, u: f64) -> [f64; 2] {
        let (s, c) = u.sin_cos();
        [-self.radius * s, self.radius * c]
    }

    /// `Period()` — the period of the circle: `2π`.
    #[inline]
    pub fn period(&self) -> f64 {
        2.0 * PI
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Ellipse2dGeom
// ─────────────────────────────────────────────────────────────────────────────

/// Stub for `Geom2d_Ellipse` — an analytic 2D ellipse parameterised by angle `U`.
///
/// The major axis is aligned with the X axis of the implicit frame.  The
/// parameterisation is:
/// ```text
/// P(U) = [center[0] + major_radius * cos(U),
///          center[1] + minor_radius * sin(U)]
/// ```
/// The curve is closed and periodic with period `2π`.
// occt-ref: Geom2d_Ellipse
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ellipse2dGeom {
    /// Centre of the ellipse (`gp_Pnt2d` / `Location()`).
    pub center: [f64; 2],
    /// Semi-major radius (along X).  Must satisfy `major_radius >= minor_radius >= 0`.
    pub major_radius: f64,
    /// Semi-minor radius (along Y).  Must satisfy `major_radius >= minor_radius >= 0`.
    pub minor_radius: f64,
}

impl Ellipse2dGeom {
    /// `Geom2d_Ellipse(cx, cy, a, b)` — ellipse centred at `(cx, cy)` with
    /// semi-major radius `a` and semi-minor radius `b`.
    ///
    /// # Panics
    /// Panics (`Standard_ConstructionError`) if `b < 0` or `a < b`.
    #[inline]
    pub fn new(cx: f64, cy: f64, a: f64, b: f64) -> Self {
        assert!(
            b >= 0.0 && a >= b,
            "Ellipse2dGeom::new: require minor_radius >= 0 and major_radius >= minor_radius"
        );
        Self { center: [cx, cy], major_radius: a, minor_radius: b }
    }

    /// `D0(U)` — the point at parameter `U`:
    /// `P = [cx + a*cos(U), cy + b*sin(U)]`.
    #[inline]
    pub fn point_at(&self, u: f64) -> [f64; 2] {
        let (s, c) = u.sin_cos();
        [
            self.center[0] + self.major_radius * c,
            self.center[1] + self.minor_radius * s,
        ]
    }

    /// `Focus1()` — the first focus, at `center + c * [1, 0]` where
    /// `c = sqrt(a² − b²)`.
    #[inline]
    pub fn focus1(&self) -> [f64; 2] {
        let c = (self.major_radius * self.major_radius
            - self.minor_radius * self.minor_radius)
            .sqrt();
        [self.center[0] + c, self.center[1]]
    }

    /// `Focus2()` — the second focus, at `center - c * [1, 0]` where
    /// `c = sqrt(a² − b²)`.
    #[inline]
    pub fn focus2(&self) -> [f64; 2] {
        let c = (self.major_radius * self.major_radius
            - self.minor_radius * self.minor_radius)
            .sqrt();
        [self.center[0] - c, self.center[1]]
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Hyperbola2d
// ─────────────────────────────────────────────────────────────────────────────

/// Stub for `Geom2d_Hyperbola` — an analytic 2D hyperbola (main branch)
/// parameterised by a real `U ∈ ]−∞, +∞[`.
///
/// The real axis is aligned with the X axis of the implicit frame.  The
/// parameterisation of the **main branch** is:
/// ```text
/// P(U) = [center[0] + real_radius * cosh(U),
///          center[1] + imag_radius * sinh(U)]
/// ```
/// The curve is open and not periodic.
// occt-ref: Geom2d_Hyperbola
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Hyperbola2d {
    /// Centre of the hyperbola (mid-point between vertices of the two branches).
    pub center: [f64; 2],
    /// Semi-real (transverse) radius `a`.  Must be >= 0.
    pub real_radius: f64,
    /// Semi-imaginary (conjugate) radius `b`.  Must be >= 0.
    pub imag_radius: f64,
}

impl Hyperbola2d {
    /// `Geom2d_Hyperbola(cx, cy, r, i)` — hyperbola centred at `(cx, cy)` with
    /// real radius `r` and imaginary radius `i`.
    ///
    /// # Panics
    /// Panics (`Standard_ConstructionError`) if `r < 0` or `i < 0`.
    #[inline]
    pub fn new(cx: f64, cy: f64, r: f64, i: f64) -> Self {
        assert!(r >= 0.0, "Hyperbola2d::new: real_radius must be >= 0");
        assert!(i >= 0.0, "Hyperbola2d::new: imag_radius must be >= 0");
        Self { center: [cx, cy], real_radius: r, imag_radius: i }
    }

    /// `D0(U)` — the point at parameter `U` on the main branch:
    /// `P = [cx + a*cosh(U), cy + b*sinh(U)]`.
    #[inline]
    pub fn point_at(&self, u: f64) -> [f64; 2] {
        [
            self.center[0] + self.real_radius * u.cosh(),
            self.center[1] + self.imag_radius * u.sinh(),
        ]
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Parabola2dGeom
// ─────────────────────────────────────────────────────────────────────────────

/// Stub for `Geom2d_Parabola` — an analytic 2D parabola parameterised by a
/// real `U ∈ ]−∞, +∞[`.
///
/// The axis of symmetry points from the `focus` along the positive X direction
/// of the implicit frame; the vertex lies at distance `focal` behind the focus.
/// The parameterisation is:
/// ```text
/// P(U) = [vertex[0] + U² / (4*focal),
///          vertex[1] + U]
/// ```
/// where `vertex = focus - [focal, 0]`.  Equivalently, in terms of the
/// parabola parameter `p = 2*focal` (semi-latus rectum):
/// ```text
/// P(U) = [vertex[0] + U² / (2*p),
///          vertex[1] + U]
/// ```
/// The curve is open and not periodic.
// occt-ref: Geom2d_Parabola
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Parabola2dGeom {
    /// The focus of the parabola.
    pub focus: [f64; 2],
    /// The focal length: distance from the vertex to the focus.  Must be > 0.
    pub focal: f64,
}

impl Parabola2dGeom {
    /// `Geom2d_Parabola(fx, fy, focal)` — parabola whose focus is at `(fx, fy)`
    /// with the given `focal` length (distance from vertex to focus).
    ///
    /// The axis of symmetry is aligned with the X axis: the vertex is at
    /// `[fx − focal, fy]` and the parabola opens in the `+X` direction.
    ///
    /// # Panics
    /// Panics (`Standard_ConstructionError`) if `focal <= 0`.
    #[inline]
    pub fn new(fx: f64, fy: f64, focal: f64) -> Self {
        assert!(focal > 0.0, "Parabola2dGeom::new: focal must be > 0");
        Self { focus: [fx, fy], focal }
    }

    /// The vertex of the parabola: `[focus[0] − focal, focus[1]]`.
    #[inline]
    fn vertex(&self) -> [f64; 2] {
        [self.focus[0] - self.focal, self.focus[1]]
    }

    /// `D0(U)` — the point at parameter `U`:
    /// ```text
    /// P = [vertex_x + U² / (4*focal),  vertex_y + U]
    /// ```
    ///
    /// This is the standard OCCT parabola parametric form with the axis of
    /// symmetry along X.
    #[inline]
    pub fn point_at(&self, u: f64) -> [f64; 2] {
        let v = self.vertex();
        [v[0] + u * u / (4.0 * self.focal), v[1] + u]
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    const EPS: f64 = 1e-12;

    // ── Circle2dGeom ──────────────────────────────────────────────────────────

    #[test]
    fn circle_point_at_zero() {
        let c = Circle2dGeom::new(1.0, 2.0, 5.0);
        let p = c.point_at(0.0);
        // P(0) = [cx + r, cy]
        assert!((p[0] - 6.0).abs() < EPS, "x = {}", p[0]);
        assert!((p[1] - 2.0).abs() < EPS, "y = {}", p[1]);
    }

    #[test]
    fn circle_point_at_half_pi() {
        let c = Circle2dGeom::new(0.0, 0.0, 3.0);
        let p = c.point_at(PI / 2.0);
        assert!(p[0].abs() < EPS, "x = {}", p[0]);
        assert!((p[1] - 3.0).abs() < EPS, "y = {}", p[1]);
    }

    #[test]
    fn circle_d1_tangent_perpendicular_to_radius() {
        let c = Circle2dGeom::new(0.0, 0.0, 4.0);
        for u in [0.0_f64, 0.5, 1.0, 2.0, PI] {
            let p = c.point_at(u);
            let v = c.d1(u);
            // Dot product of radius vector and tangent must be zero.
            let dot = p[0] * v[0] + p[1] * v[1];
            assert!(dot.abs() < EPS, "u={u}, dot={dot}");
        }
    }

    #[test]
    fn circle_period() {
        let c = Circle2dGeom::new(0.0, 0.0, 1.0);
        assert!((c.period() - 2.0 * PI).abs() < EPS);
    }

    #[test]
    fn circle_closes_after_period() {
        let c = Circle2dGeom::new(2.0, 3.0, 7.0);
        let p0 = c.point_at(0.0);
        let p1 = c.point_at(c.period());
        assert!((p0[0] - p1[0]).abs() < EPS);
        assert!((p0[1] - p1[1]).abs() < EPS);
    }

    // ── Ellipse2dGeom ─────────────────────────────────────────────────────────

    #[test]
    fn ellipse_major_vertex_at_zero() {
        let e = Ellipse2dGeom::new(0.0, 0.0, 5.0, 3.0);
        let p = e.point_at(0.0);
        assert!((p[0] - 5.0).abs() < EPS, "x = {}", p[0]);
        assert!(p[1].abs() < EPS, "y = {}", p[1]);
    }

    #[test]
    fn ellipse_minor_vertex_at_half_pi() {
        let e = Ellipse2dGeom::new(0.0, 0.0, 5.0, 3.0);
        let p = e.point_at(PI / 2.0);
        assert!(p[0].abs() < EPS, "x = {}", p[0]);
        assert!((p[1] - 3.0).abs() < EPS, "y = {}", p[1]);
    }

    #[test]
    fn ellipse_equation_satisfied() {
        let (a, b) = (5.0_f64, 3.0_f64);
        let e = Ellipse2dGeom::new(0.0, 0.0, a, b);
        for u in [0.0_f64, 0.5, 1.0, PI / 3.0, PI, 5.0] {
            let p = e.point_at(u);
            let lhs = (p[0] / a).powi(2) + (p[1] / b).powi(2);
            assert!((lhs - 1.0).abs() < EPS, "u={u}, lhs={lhs}");
        }
    }

    #[test]
    fn ellipse_foci_positions() {
        // a=5, b=3 => c=4
        let e = Ellipse2dGeom::new(1.0, 0.0, 5.0, 3.0);
        let f1 = e.focus1();
        let f2 = e.focus2();
        // c = sqrt(25 - 9) = 4; foci at center ± 4 along X
        assert!((f1[0] - 5.0).abs() < EPS, "f1.x = {}", f1[0]);
        assert!(f1[1].abs() < EPS, "f1.y = {}", f1[1]);
        assert!((f2[0] + 3.0).abs() < EPS, "f2.x = {}", f2[0]);
        assert!(f2[1].abs() < EPS, "f2.y = {}", f2[1]);
    }

    // ── Hyperbola2d ───────────────────────────────────────────────────────────

    #[test]
    fn hyperbola_vertex_at_u0() {
        let h = Hyperbola2d::new(0.0, 0.0, 3.0, 4.0);
        let p = h.point_at(0.0);
        // cosh(0) = 1, sinh(0) = 0 => P = (a, 0)
        assert!((p[0] - 3.0).abs() < EPS, "x = {}", p[0]);
        assert!(p[1].abs() < EPS, "y = {}", p[1]);
    }

    #[test]
    fn hyperbola_equation_satisfied() {
        // Main branch: (x/a)^2 - (y/b)^2 = 1
        let (a, b) = (3.0_f64, 4.0_f64);
        let h = Hyperbola2d::new(0.0, 0.0, a, b);
        for u in [-2.0_f64, -1.0, 0.0, 1.0, 2.0] {
            let p = h.point_at(u);
            let lhs = (p[0] / a).powi(2) - (p[1] / b).powi(2);
            assert!((lhs - 1.0).abs() < 1e-10, "u={u}, lhs={lhs}");
        }
    }

    #[test]
    fn hyperbola_with_offset_center() {
        let h = Hyperbola2d::new(1.0, 2.0, 1.0, 1.0);
        let p = h.point_at(0.0);
        // cosh(0) = 1 => x = 1 + 1 = 2; sinh(0) = 0 => y = 2
        assert!((p[0] - 2.0).abs() < EPS, "x = {}", p[0]);
        assert!((p[1] - 2.0).abs() < EPS, "y = {}", p[1]);
    }

    // ── Parabola2dGeom ────────────────────────────────────────────────────────

    #[test]
    fn parabola_vertex_at_u0() {
        // focus at (2, 0), focal = 2 => vertex at (0, 0)
        let p = Parabola2dGeom::new(2.0, 0.0, 2.0);
        let pt = p.point_at(0.0);
        assert!(pt[0].abs() < EPS, "x = {}", pt[0]);
        assert!(pt[1].abs() < EPS, "y = {}", pt[1]);
    }

    #[test]
    fn parabola_focus_on_curve_at_sqrt_4f() {
        // For focal length f, the point at parameter U = 2*sqrt(f) satisfies
        // y^2 = 4*f*x  (standard parabola equation in vertex form).
        let f = 3.0_f64;
        let par = Parabola2dGeom::new(f, 0.0, f); // vertex at origin
        for u in [-6.0_f64, -3.0, 0.0, 3.0, 6.0] {
            let pt = par.point_at(u);
            // y^2 should equal 4*f*x
            let lhs = pt[1] * pt[1];
            let rhs = 4.0 * f * pt[0];
            assert!((lhs - rhs).abs() < EPS, "u={u}: y^2={lhs}, 4fx={rhs}");
        }
    }

    #[test]
    fn parabola_focus_coordinates() {
        let par = Parabola2dGeom::new(5.0, 3.0, 5.0);
        assert!((par.focus[0] - 5.0).abs() < EPS);
        assert!((par.focus[1] - 3.0).abs() < EPS);
    }

    #[test]
    fn parabola_vertex_derived() {
        let par = Parabola2dGeom::new(4.0, 1.0, 4.0);
        let v = par.vertex();
        // vertex = [focus_x - focal, focus_y] = [0, 1]
        assert!((v[0] - 0.0).abs() < EPS, "vx = {}", v[0]);
        assert!((v[1] - 1.0).abs() < EPS, "vy = {}", v[1]);
    }
}
