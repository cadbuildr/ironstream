//! `gp_Hypr` -- analytic 3D hyperbola primitive, mirroring OpenCascade's
//! `gp_Hypr` (package TKMath / `gp`).
//!
//! A hyperbola is defined by its centre, a right-handed local coordinate
//! system (`gp_Ax2`, stored as [`Ax3`] throughout IronStream), a semi-major
//! radius `a` and a semi-minor radius `b`.  The local frame convention matches
//! OCCT exactly:
//!
//! | local direction | meaning |
//! |-----------------|---------|
//! | X               | direction of the **major** axis (transverse axis) |
//! | Y               | direction of the **minor** (conjugate) axis |
//! | Z               | normal to the hyperbola plane |
//!
//! The hyperbola is not parameterized here — that is the role of
//! `Geom_Hyperbola`.  `gp_Hypr` is a *geometric descriptor* analogous to
//! [`crate::gp_prim::Circ`] and is intended to seed higher-level algorithms.
//!
//! Key formulas (OCCT convention):
//!
//! ```text
//! Focal distance  c  = sqrt(a² + b²)
//! Eccentricity    e  = c / a            (always > 1)
//! Focus 1            = centre + c · XDir
//! Focus 2            = centre − c · XDir
//! Asymptote 1 dir    = normalise( a · XDir + b · YDir )
//! Asymptote 2 dir    = normalise( a · XDir − b · YDir )
//! ```
//!
//! Builds on the existing `gp` / `gp_prim` API; zero third-party dependencies.

use crate::gp::{Ax1, Ax3, Pnt, Trsf};
use crate::gp_prim::Lin;

/// `gp_Hypr` — an analytic 3D hyperbola primitive.
///
/// The hyperbola is centred at `position.location`, with its major (transverse)
/// axis along `position.x_dir`, minor (conjugate) axis along `position.y_dir`,
/// and its plane normal along `position.z_dir`.
// occt: gp_Hypr
#[derive(Clone, Copy, Debug)]
pub struct Hypr {
    /// Placement: origin = centre, X = major axis, Y = minor axis, Z = normal.
    position: Ax3,
    /// Semi-major radius `a` (≥ 0).
    major_radius: f64,
    /// Semi-minor radius `b` (≥ 0).
    minor_radius: f64,
}

impl Hypr {
    // ─────────────────────────────────────── Constructor ─────────────────────

    /// `gp_Hypr(const gp_Ax2& A2, Standard_Real MajorRadius,
    /// Standard_Real MinorRadius)`.
    ///
    /// Constructs the hyperbola with:
    /// - centre at the origin of `a2`,
    /// - major axis along the X direction of `a2`,
    /// - minor axis along the Y direction of `a2`,
    /// - semi-major radius `major_radius` and semi-minor radius `minor_radius`.
    ///
    /// # Panics
    /// Raised (`Standard_ConstructionError`) if either radius is negative.
    pub fn new(a2: Ax3, major_radius: f64, minor_radius: f64) -> Self {
        assert!(
            major_radius >= 0.0,
            "gp_Hypr: MajorRadius must be >= 0"
        );
        assert!(
            minor_radius >= 0.0,
            "gp_Hypr: MinorRadius must be >= 0"
        );
        Self {
            position: a2,
            major_radius,
            minor_radius,
        }
    }

    // ─────────────────────────────────────── Setters ─────────────────────────

    /// `SetAxis(const gp_Ax1& A1)` — change the main axis (normal direction).
    ///
    /// Adjusts `z_dir` so the normal points in the direction of `a1`.
    pub fn set_axis(&mut self, a1: Ax1) {
        self.position.z_dir = a1.direction.normalized();
        // Re-derive x_dir perpendicular to the new z_dir to keep right-handedness.
        let x_hint = self.position.x_dir;
        let z = self.position.z_dir;
        let mut x = x_hint - z * x_hint.dot(z);
        if x.norm() < crate::gp::EPS {
            x = z.any_perpendicular();
        }
        self.position.x_dir = x.normalized();
        self.position.y_dir = z.cross(self.position.x_dir).normalized();
    }

    /// `SetLocation(const gp_Pnt& P)` — change the centre of the hyperbola.
    #[inline]
    pub fn set_location(&mut self, p: Pnt) {
        self.position.location = p;
    }

    /// `SetMajorRadius(Standard_Real R)`.
    ///
    /// # Panics
    /// Raised if `r < 0`.
    pub fn set_major_radius(&mut self, r: f64) {
        assert!(r >= 0.0, "gp_Hypr::SetMajorRadius: R must be >= 0");
        self.major_radius = r;
    }

    /// `SetMinorRadius(Standard_Real R)`.
    ///
    /// # Panics
    /// Raised if `r < 0`.
    pub fn set_minor_radius(&mut self, r: f64) {
        assert!(r >= 0.0, "gp_Hypr::SetMinorRadius: R must be >= 0");
        self.minor_radius = r;
    }

    /// `SetPosition(const gp_Ax2& A2)` — change the local coordinate system.
    #[inline]
    pub fn set_position(&mut self, a2: Ax3) {
        self.position = a2;
    }

    // ─────────────────────────────────────── Getters ─────────────────────────

    /// `Standard_Real MajorRadius() const`.
    #[inline]
    pub fn major_radius(&self) -> f64 {
        self.major_radius
    }

    /// `Standard_Real MinorRadius() const`.
    #[inline]
    pub fn minor_radius(&self) -> f64 {
        self.minor_radius
    }

    /// `gp_Ax3 Position() const` — the local coordinate system (`gp_Ax2` in
    /// OCCT, mapped to [`Ax3`] in IronStream).
    #[inline]
    pub fn position(&self) -> Ax3 {
        self.position
    }

    /// `gp_Pnt Location() const` — the centre of the hyperbola (origin of the
    /// local frame).
    #[inline]
    pub fn location(&self) -> Pnt {
        self.position.location
    }

    /// `gp_Ax1 Axis() const` — the main axis: origin + normal (Z direction).
    ///
    /// In OCCT this is the axis perpendicular to the hyperbola plane.
    #[inline]
    pub fn axis(&self) -> Ax1 {
        Ax1::new(self.position.location, self.position.z_dir)
    }

    /// `gp_Ax1 XAxis() const` — the major-axis direction: origin + X direction.
    #[inline]
    pub fn x_axis(&self) -> Ax1 {
        Ax1::new(self.position.location, self.position.x_dir)
    }

    /// `gp_Ax1 YAxis() const` — the minor-axis direction: origin + Y direction.
    #[inline]
    pub fn y_axis(&self) -> Ax1 {
        Ax1::new(self.position.location, self.position.y_dir)
    }

    // ─────────────────────────────────────── Conic properties ────────────────

    /// Helper: the focal half-distance `c = sqrt(a² + b²)`.
    #[inline]
    fn focal_c(&self) -> f64 {
        (self.major_radius * self.major_radius
            + self.minor_radius * self.minor_radius)
            .sqrt()
    }

    /// `Standard_Real Eccentricity() const`.
    ///
    /// Returns `e = c / a = sqrt(1 + (b/a)²)`. For any non-degenerate
    /// hyperbola `e > 1`.
    ///
    /// # Panics
    /// Raised (`Standard_DomainError`) if `major_radius == 0`.
    pub fn eccentricity(&self) -> f64 {
        assert!(
            self.major_radius > 0.0,
            "gp_Hypr::Eccentricity: MajorRadius is zero"
        );
        self.focal_c() / self.major_radius
    }

    /// `Standard_Real Focal() const` — the distance between the two foci,
    /// `2 * c` where `c = sqrt(a² + b²)`.
    #[inline]
    pub fn focal(&self) -> f64 {
        2.0 * self.focal_c()
    }

    /// `gp_Pnt Focus1() const` — the first focus: `centre + c · XDir`.
    pub fn focus1(&self) -> Pnt {
        let c = self.focal_c();
        self.position.location + self.position.x_dir * c
    }

    /// `gp_Pnt Focus2() const` — the second focus: `centre − c · XDir`.
    pub fn focus2(&self) -> Pnt {
        let c = self.focal_c();
        self.position.location - self.position.x_dir * c
    }

    /// `gp_Ax1 Asymptote1() const`.
    ///
    /// The first asymptote passes through the centre with direction
    /// `normalise( a · XDir + b · YDir )`.
    ///
    /// # Panics
    /// Raised if `major_radius == 0` (direction would be degenerate).
    pub fn asymptote1(&self) -> Ax1 {
        assert!(
            self.major_radius > 0.0,
            "gp_Hypr::Asymptote1: MajorRadius is zero"
        );
        let dir = (self.position.x_dir * self.major_radius
            + self.position.y_dir * self.minor_radius)
            .normalized();
        Ax1::new(self.position.location, dir)
    }

    /// `gp_Ax1 Asymptote2() const`.
    ///
    /// The second asymptote direction is `normalise( a · XDir − b · YDir )`.
    ///
    /// # Panics
    /// Raised if `major_radius == 0`.
    pub fn asymptote2(&self) -> Ax1 {
        assert!(
            self.major_radius > 0.0,
            "gp_Hypr::Asymptote2: MajorRadius is zero"
        );
        let dir = (self.position.x_dir * self.major_radius
            - self.position.y_dir * self.minor_radius)
            .normalized();
        Ax1::new(self.position.location, dir)
    }

    /// `gp_Ax1 Directrix1() const`.
    ///
    /// The first directrix is the line perpendicular to the major axis, at
    /// distance `a / e = a² / c` from the centre, on the side of `Focus1`.
    ///
    /// # Panics
    /// Raised if `major_radius == 0`.
    pub fn directrix1(&self) -> Lin {
        assert!(
            self.major_radius > 0.0,
            "gp_Hypr::Directrix1: MajorRadius is zero"
        );
        let c = self.focal_c();
        let d = if c > 0.0 {
            self.major_radius * self.major_radius / c
        } else {
            0.0
        };
        let loc = self.position.location + self.position.x_dir * d;
        Lin::new(Ax1::new(loc, self.position.y_dir))
    }

    /// `gp_Ax1 Directrix2() const`.
    ///
    /// The second directrix is symmetric to `Directrix1` about the centre.
    ///
    /// # Panics
    /// Raised if `major_radius == 0`.
    pub fn directrix2(&self) -> Lin {
        assert!(
            self.major_radius > 0.0,
            "gp_Hypr::Directrix2: MajorRadius is zero"
        );
        let c = self.focal_c();
        let d = if c > 0.0 {
            self.major_radius * self.major_radius / c
        } else {
            0.0
        };
        let loc = self.position.location - self.position.x_dir * d;
        Lin::new(Ax1::new(loc, self.position.y_dir))
    }

    // ─────────────────────────────────────── Branch & conjugate ──────────────

    /// `gp_Hypr OtherBranch() const` — the "other" (left) branch of the same
    /// hyperbola.
    ///
    /// The other branch is obtained by negating the X direction of the local
    /// frame (and adjusting Y so the frame remains right-handed).
    pub fn other_branch(&self) -> Hypr {
        let pos = Ax3 {
            location: self.position.location,
            x_dir: -self.position.x_dir,
            y_dir: -self.position.y_dir,
            z_dir: self.position.z_dir,
        };
        Hypr {
            position: pos,
            major_radius: self.major_radius,
            minor_radius: self.minor_radius,
        }
    }

    /// `gp_Hypr ConjugateBranch1() const`.
    ///
    /// The first branch of the **conjugate** hyperbola: the hyperbola whose
    /// transverse axis is the minor axis of `self` and whose minor axis is the
    /// major axis of `self`.  The frame is rotated 90° (X→Y, Y→−X) and the
    /// radii are swapped.
    pub fn conjugate_branch1(&self) -> Hypr {
        let pos = Ax3 {
            location: self.position.location,
            x_dir: self.position.y_dir,
            y_dir: -self.position.x_dir,
            z_dir: self.position.z_dir,
        };
        Hypr {
            position: pos,
            major_radius: self.minor_radius,
            minor_radius: self.major_radius,
        }
    }

    /// `gp_Hypr ConjugateBranch2() const` — the second (opposite) branch of
    /// the conjugate hyperbola.
    pub fn conjugate_branch2(&self) -> Hypr {
        let pos = Ax3 {
            location: self.position.location,
            x_dir: -self.position.y_dir,
            y_dir: self.position.x_dir,
            z_dir: self.position.z_dir,
        };
        Hypr {
            position: pos,
            major_radius: self.minor_radius,
            minor_radius: self.major_radius,
        }
    }

    // ─────────────────────────────────────── Transforms ──────────────────────

    /// `void Mirror(const gp_Pnt& P)` — point symmetry about `p` (in place).
    pub fn mirror_point(&mut self, p: Pnt) {
        self.position.location = self.position.location.mirrored_point(p);
        self.position.x_dir = -self.position.x_dir;
        self.position.y_dir = -self.position.y_dir;
        // z_dir sign: reflecting through a point keeps z (consistent with gp_Ax2)
    }

    /// Returns a copy reflected through point `p`.
    pub fn mirrored_point(&self, p: Pnt) -> Hypr {
        let mut h = *self;
        h.mirror_point(p);
        h
    }

    /// `void Mirror(const gp_Ax1& A1)` — reflection across a line (in place).
    pub fn mirror_axis(&mut self, a1: Ax1) {
        let t = mirror_axis_trsf(a1);
        self.position = self.position.transformed(&t);
    }

    /// Returns a copy reflected across the axis `a1`.
    pub fn mirrored_axis(&self, a1: Ax1) -> Hypr {
        let mut h = *self;
        h.mirror_axis(a1);
        h
    }

    /// `void Mirror(const gp_Ax2& A2)` — reflection across a plane (in place).
    pub fn mirror_plane(&mut self, a2: Ax3) {
        let t = mirror_plane_trsf(a2);
        self.position = self.position.transformed(&t);
    }

    /// Returns a copy reflected across the plane `a2`.
    pub fn mirrored_plane(&self, a2: Ax3) -> Hypr {
        let mut h = *self;
        h.mirror_plane(a2);
        h
    }

    /// `void Rotate(const gp_Ax1& A1, Standard_Real Ang)` — rotation (in place).
    pub fn rotate(&mut self, a1: Ax1, angle: f64) {
        let t = Trsf::rotation(a1, angle);
        self.position = self.position.transformed(&t);
    }

    /// Returns a rotated copy.
    pub fn rotated(&self, a1: Ax1, angle: f64) -> Hypr {
        let mut h = *self;
        h.rotate(a1, angle);
        h
    }

    /// `void Scale(const gp_Pnt& P, Standard_Real S)` — uniform scaling
    /// (in place).  Radii scale by `|S|`.
    pub fn scale(&mut self, center: Pnt, s: f64) {
        let mut t = Trsf::identity();
        let _ = t.set_scale(center, s);
        self.position = self.position.transformed(&t);
        self.major_radius *= s.abs();
        self.minor_radius *= s.abs();
    }

    /// Returns a scaled copy.
    pub fn scaled(&self, center: Pnt, s: f64) -> Hypr {
        let mut h = *self;
        h.scale(center, s);
        h
    }

    /// `void Transform(const gp_Trsf& T)` — general transform (in place).
    ///
    /// Radii are scaled by `|T.ScaleFactor()|`.
    pub fn transform(&mut self, t: &Trsf) {
        let scale = t.scale_factor().abs();
        self.position = self.position.transformed(t);
        self.major_radius *= scale;
        self.minor_radius *= scale;
    }

    /// Returns a transformed copy.
    pub fn transformed(&self, t: &Trsf) -> Hypr {
        let mut h = *self;
        h.transform(t);
        h
    }

    /// `void Translate(const gp_Vec& V)` — pure translation (in place).
    pub fn translate(&mut self, v: Pnt) {
        self.position.location = self.position.location + v;
    }

    /// Returns a translated copy.
    pub fn translated(&self, v: Pnt) -> Hypr {
        let mut h = *self;
        h.translate(v);
        h
    }

    /// `void Translate(const gp_Pnt& P1, const gp_Pnt& P2)` — translate by the
    /// vector P1→P2 (in place).
    pub fn translate_2pts(&mut self, p1: Pnt, p2: Pnt) {
        self.translate(p2 - p1);
    }

    /// Returns a copy translated by the vector P1→P2.
    pub fn translated_2pts(&self, p1: Pnt, p2: Pnt) -> Hypr {
        self.translated(p2 - p1)
    }
}

// ─────────────────────────── private helpers ─────────────────────────────────

/// Build a `Trsf` that reflects points through the axis `a1` (a line).
///
/// The reflection of a point `P` across a line through `q` with unit direction
/// `d` is: `P' = 2*(q + d*((P-q)·d)) - P = 2*proj - P`.
fn mirror_axis_trsf(a: Ax1) -> Trsf {
    let d = a.direction.normalized();
    // Householder-style: R = 2*d*dT - I  (for the linear part, for a line
    // mirror this is actually the Rodrigues formula for 180° rotation about d)
    let (dx, dy, dz) = (d.x, d.y, d.z);
    let m = [
        [2.0 * dx * dx - 1.0, 2.0 * dx * dy, 2.0 * dx * dz],
        [2.0 * dx * dy, 2.0 * dy * dy - 1.0, 2.0 * dy * dz],
        [2.0 * dx * dz, 2.0 * dy * dz, 2.0 * dz * dz - 1.0],
    ];
    // Translation: q - R*q  (rotation about q by 180°)
    let q = [a.location.x, a.location.y, a.location.z];
    let rq = [
        m[0][0] * q[0] + m[0][1] * q[1] + m[0][2] * q[2],
        m[1][0] * q[0] + m[1][1] * q[1] + m[1][2] * q[2],
        m[2][0] * q[0] + m[2][1] * q[1] + m[2][2] * q[2],
    ];
    Trsf {
        m,
        t: [q[0] - rq[0], q[1] - rq[1], q[2] - rq[2]],
    }
}

/// Build a `Trsf` that reflects points across the plane defined by `a2`
/// (origin + normal = z_dir).
fn mirror_plane_trsf(a2: Ax3) -> Trsf {
    let n = a2.z_dir.normalized();
    let (nx, ny, nz) = (n.x, n.y, n.z);
    // Householder reflection: R = I - 2*n*nT
    let m = [
        [1.0 - 2.0 * nx * nx, -2.0 * nx * ny, -2.0 * nx * nz],
        [-2.0 * nx * ny, 1.0 - 2.0 * ny * ny, -2.0 * ny * nz],
        [-2.0 * nx * nz, -2.0 * ny * nz, 1.0 - 2.0 * nz * nz],
    ];
    // Translation: 2*(n·p)*n  where p is the plane origin.
    let p = a2.location;
    let dot = n.x * p.x + n.y * p.y + n.z * p.z;
    Trsf {
        m,
        t: [2.0 * dot * nx, 2.0 * dot * ny, 2.0 * dot * nz],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gp::Ax3;

    fn make_hypr() -> Hypr {
        let a2 = Ax3::identity();
        Hypr::new(a2, 3.0, 4.0)
    }

    #[test]
    fn constructor_stores_radii() {
        let h = make_hypr();
        assert_eq!(h.major_radius(), 3.0);
        assert_eq!(h.minor_radius(), 4.0);
    }

    #[test]
    fn focal_distance() {
        let h = make_hypr(); // a=3, b=4, c=5
        assert!((h.focal() - 10.0).abs() < 1e-12);
    }

    #[test]
    fn eccentricity() {
        let h = make_hypr(); // a=3, b=4, c=5  => e=5/3
        assert!((h.eccentricity() - 5.0 / 3.0).abs() < 1e-12);
    }

    #[test]
    fn foci() {
        let h = make_hypr(); // c=5, centre at origin
        let f1 = h.focus1();
        let f2 = h.focus2();
        assert!((f1.x - 5.0).abs() < 1e-12);
        assert!(f1.y.abs() < 1e-12);
        assert!((f2.x + 5.0).abs() < 1e-12);
        assert!(f2.y.abs() < 1e-12);
    }

    #[test]
    fn asymptote_angle() {
        let h = make_hypr(); // a=3, b=4 => atan(4/3)
        let a1 = h.asymptote1();
        // direction should be (3,4,0)/5
        assert!((a1.direction.x - 0.6).abs() < 1e-12);
        assert!((a1.direction.y - 0.8).abs() < 1e-12);
    }

    #[test]
    fn translate() {
        let mut h = make_hypr();
        h.translate(Pnt::new(1.0, 2.0, 3.0));
        let loc = h.location();
        assert!((loc.x - 1.0).abs() < 1e-12);
        assert!((loc.y - 2.0).abs() < 1e-12);
        assert!((loc.z - 3.0).abs() < 1e-12);
    }

    #[test]
    fn rotate_180_about_z() {
        let h = make_hypr();
        let axis = Ax1::new(Pnt::origin(), Pnt::new(0.0, 0.0, 1.0));
        let hr = h.rotated(axis, std::f64::consts::PI);
        // centre stays at origin, but focus1 moves to (-5,0,0)
        let f1 = hr.focus1();
        assert!((f1.x + 5.0).abs() < 1e-9);
        assert!(f1.y.abs() < 1e-9);
    }

    #[test]
    fn scale_doubles_radii() {
        let h = make_hypr().scaled(Pnt::origin(), 2.0);
        assert!((h.major_radius() - 6.0).abs() < 1e-12);
        assert!((h.minor_radius() - 8.0).abs() < 1e-12);
    }

    #[test]
    fn other_branch_negates_x() {
        let h = make_hypr();
        let ob = h.other_branch();
        assert!((ob.position().x_dir.x + 1.0).abs() < 1e-12);
    }
}
