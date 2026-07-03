// FILE: geom_rect_trimmed.rs
// occt: Geom_RectangularTrimmedSurface

use crate::gp::{Pnt, Vec3};
use crate::geom_plane::GeomPlane;
use crate::geom_bspline_surface::GeomBSplineSurface;

// We need to import the cylinder, cone, sphere, and torus surface types.
// These may live in dedicated modules; we reference what's available.
// occt: Geom_CylindricalSurface
pub struct GeomCylindricalSurface {
    /// axis: [ox, oy, oz, dx, dy, dz] (origin + direction)
    pub axis: [f64; 6],
    pub radius: f64,
}

impl GeomCylindricalSurface {
    pub fn new(origin: [f64; 3], direction: [f64; 3], radius: f64) -> Self {
        Self {
            axis: [
                origin[0], origin[1], origin[2],
                direction[0], direction[1], direction[2],
            ],
            radius,
        }
    }

    /// Evaluate point at (u, v): u is angle in radians, v is height along axis.
    pub fn value(&self, u: f64, v: f64) -> Pnt {
        let dz = [self.axis[3], self.axis[4], self.axis[5]];
        let (ex, ey) = perp_frame(dz);
        let ox = self.axis[0];
        let oy = self.axis[1];
        let oz = self.axis[2];
        let r = self.radius;
        Pnt::new(
            ox + r * u.cos() * ex[0] + r * u.sin() * ey[0] + v * dz[0],
            oy + r * u.cos() * ex[1] + r * u.sin() * ey[1] + v * dz[1],
            oz + r * u.cos() * ex[2] + r * u.sin() * ey[2] + v * dz[2],
        )
    }

    /// First derivatives at (u, v).
    pub fn d1(&self, u: f64, v: f64) -> (Pnt, Vec3, Vec3) {
        let dz = [self.axis[3], self.axis[4], self.axis[5]];
        let (ex, ey) = perp_frame(dz);
        let r = self.radius;
        let p = self.value(u, v);
        let du = [
            -r * u.sin() * ex[0] + r * u.cos() * ey[0],
            -r * u.sin() * ex[1] + r * u.cos() * ey[1],
            -r * u.sin() * ex[2] + r * u.cos() * ey[2],
        ];
        let dv = dz;
        (p, Pnt::new(du[0], du[1], du[2]), Pnt::new(dv[0], dv[1], dv[2]))
    }
}

// occt: Geom_ConicalSurface
pub struct GeomConicalSurface {
    /// axis: origin + direction (Z axis)
    pub axis: [f64; 6],
    pub radius: f64,
    pub semi_angle: f64,
}

impl GeomConicalSurface {
    pub fn new(origin: [f64; 3], direction: [f64; 3], radius: f64, semi_angle: f64) -> Self {
        Self {
            axis: [
                origin[0], origin[1], origin[2],
                direction[0], direction[1], direction[2],
            ],
            radius,
            semi_angle,
        }
    }

    /// Evaluate point at (u, v): u is angle, v is height parameter.
    pub fn value(&self, u: f64, v: f64) -> Pnt {
        let dz = [self.axis[3], self.axis[4], self.axis[5]];
        let (ex, ey) = perp_frame(dz);
        let ox = self.axis[0];
        let oy = self.axis[1];
        let oz = self.axis[2];
        let r = self.radius + v * self.semi_angle.tan();
        Pnt::new(
            ox + r * u.cos() * ex[0] + r * u.sin() * ey[0] + v * dz[0],
            oy + r * u.cos() * ex[1] + r * u.sin() * ey[1] + v * dz[1],
            oz + r * u.cos() * ex[2] + r * u.sin() * ey[2] + v * dz[2],
        )
    }

    pub fn d1(&self, u: f64, v: f64) -> (Pnt, Vec3, Vec3) {
        let dz = [self.axis[3], self.axis[4], self.axis[5]];
        let (ex, ey) = perp_frame(dz);
        let r = self.radius + v * self.semi_angle.tan();
        let tan_a = self.semi_angle.tan();
        let p = self.value(u, v);
        let du = [
            -r * u.sin() * ex[0] + r * u.cos() * ey[0],
            -r * u.sin() * ex[1] + r * u.cos() * ey[1],
            -r * u.sin() * ex[2] + r * u.cos() * ey[2],
        ];
        let dv = [
            tan_a * u.cos() * ex[0] + tan_a * u.sin() * ey[0] + dz[0],
            tan_a * u.cos() * ex[1] + tan_a * u.sin() * ey[1] + dz[1],
            tan_a * u.cos() * ex[2] + tan_a * u.sin() * ey[2] + dz[2],
        ];
        (p, Pnt::new(du[0], du[1], du[2]), Pnt::new(dv[0], dv[1], dv[2]))
    }
}

// occt: Geom_SphericalSurface
pub struct GeomSphericalSurface {
    /// center
    pub center: [f64; 3],
    pub radius: f64,
}

impl GeomSphericalSurface {
    pub fn new(center: [f64; 3], radius: f64) -> Self {
        Self { center, radius }
    }

    /// u = longitude (0..2pi), v = latitude (-pi/2..pi/2)
    pub fn value(&self, u: f64, v: f64) -> Pnt {
        let r = self.radius;
        Pnt::new(
            self.center[0] + r * v.cos() * u.cos(),
            self.center[1] + r * v.cos() * u.sin(),
            self.center[2] + r * v.sin(),
        )
    }

    pub fn d1(&self, u: f64, v: f64) -> (Pnt, Vec3, Vec3) {
        let r = self.radius;
        let p = self.value(u, v);
        let du = [
            -r * v.cos() * u.sin(),
            r * v.cos() * u.cos(),
            0.0,
        ];
        let dv = [
            -r * v.sin() * u.cos(),
            -r * v.sin() * u.sin(),
            r * v.cos(),
        ];
        (p, Pnt::new(du[0], du[1], du[2]), Pnt::new(dv[0], dv[1], dv[2]))
    }
}

// occt: Geom_ToroidalSurface
pub struct GeomToroidalSurface {
    pub center: [f64; 3],
    pub major_radius: f64,
    pub minor_radius: f64,
}

impl GeomToroidalSurface {
    pub fn new(center: [f64; 3], major_radius: f64, minor_radius: f64) -> Self {
        Self { center, major_radius, minor_radius }
    }

    /// u = major angle (0..2pi), v = minor angle (0..2pi)
    pub fn value(&self, u: f64, v: f64) -> Pnt {
        let r = self.major_radius + self.minor_radius * v.cos();
        Pnt::new(
            self.center[0] + r * u.cos(),
            self.center[1] + r * u.sin(),
            self.center[2] + self.minor_radius * v.sin(),
        )
    }

    pub fn d1(&self, u: f64, v: f64) -> (Pnt, Vec3, Vec3) {
        let r = self.major_radius + self.minor_radius * v.cos();
        let p = self.value(u, v);
        let du = [
            -r * u.sin(),
            r * u.cos(),
            0.0,
        ];
        let dv = [
            -self.minor_radius * v.sin() * u.cos(),
            -self.minor_radius * v.sin() * u.sin(),
            self.minor_radius * v.cos(),
        ];
        (p, Pnt::new(du[0], du[1], du[2]), Pnt::new(dv[0], dv[1], dv[2]))
    }
}

/// Build two unit vectors perpendicular to dz forming a right-handed frame.
fn perp_frame(dz: [f64; 3]) -> ([f64; 3], [f64; 3]) {
    let candidate = if dz[0].abs() <= dz[1].abs() && dz[0].abs() <= dz[2].abs() {
        [1.0_f64, 0.0, 0.0]
    } else if dz[1].abs() <= dz[2].abs() {
        [0.0_f64, 1.0, 0.0]
    } else {
        [0.0_f64, 0.0, 1.0]
    };
    // Gram-Schmidt: project candidate onto the plane perpendicular to dz
    let dot = candidate[0]*dz[0] + candidate[1]*dz[1] + candidate[2]*dz[2];
    let ex_raw = [
        candidate[0] - dot * dz[0],
        candidate[1] - dot * dz[1],
        candidate[2] - dot * dz[2],
    ];
    let len_ex = (ex_raw[0]*ex_raw[0] + ex_raw[1]*ex_raw[1] + ex_raw[2]*ex_raw[2]).sqrt();
    let ex = [ex_raw[0]/len_ex, ex_raw[1]/len_ex, ex_raw[2]/len_ex];
    let ey = cross(dz, ex);
    (ex, ey)
}

fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1]*b[2] - a[2]*b[1],
        a[2]*b[0] - a[0]*b[2],
        a[0]*b[1] - a[1]*b[0],
    ]
}

/// Enum unifying all supported basis surface types.
// occt: Geom_Surface
pub enum GeomSurfaceKind {
    Plane(GeomPlane),
    Cylinder(GeomCylindricalSurface),
    Cone(GeomConicalSurface),
    Sphere(GeomSphericalSurface),
    Torus(GeomToroidalSurface),
    BSplineSurface(GeomBSplineSurface),
}

impl GeomSurfaceKind {
    pub fn value(&self, u: f64, v: f64) -> Pnt {
        match self {
            GeomSurfaceKind::Plane(s) => s.value(u, v),
            GeomSurfaceKind::Cylinder(s) => s.value(u, v),
            GeomSurfaceKind::Cone(s) => s.value(u, v),
            GeomSurfaceKind::Sphere(s) => s.value(u, v),
            GeomSurfaceKind::Torus(s) => s.value(u, v),
            GeomSurfaceKind::BSplineSurface(s) => s.value(u, v),
        }
    }

    /// Returns (point, d/du, d/dv).
    pub fn d1(&self, u: f64, v: f64) -> (Pnt, Vec3, Vec3) {
        match self {
            GeomSurfaceKind::Plane(s) => s.d1(u, v),
            GeomSurfaceKind::Cylinder(s) => s.d1(u, v),
            GeomSurfaceKind::Cone(s) => s.d1(u, v),
            GeomSurfaceKind::Sphere(s) => s.d1(u, v),
            GeomSurfaceKind::Torus(s) => s.d1(u, v),
            GeomSurfaceKind::BSplineSurface(s) => s.d1(u, v),
        }
    }
}

/// A surface trimmed to a rectangular [u1,u2] x [v1,v2] parametric domain.
// occt: Geom_RectangularTrimmedSurface
pub struct GeomRectangularTrimmedSurface {
    pub basis: GeomSurfaceKind,
    pub u1: f64,
    pub u2: f64,
    pub v1: f64,
    pub v2: f64,
    /// Whether the U parameter range is actively trimmed.
    pub u_trim: bool,
    /// Whether the V parameter range is actively trimmed.
    pub v_trim: bool,
}

impl GeomRectangularTrimmedSurface {
    pub fn new(
        basis: GeomSurfaceKind,
        u1: f64,
        u2: f64,
        v1: f64,
        v2: f64,
        u_trim: bool,
        v_trim: bool,
    ) -> Self {
        let (u1, u2) = if u1 <= u2 { (u1, u2) } else { (u2, u1) };
        let (v1, v2) = if v1 <= v2 { (v1, v2) } else { (v2, v1) };
        Self { basis, u1, u2, v1, v2, u_trim, v_trim }
    }

    pub fn u_first(&self) -> f64 { self.u1 }
    pub fn u_last(&self) -> f64 { self.u2 }
    pub fn v_first(&self) -> f64 { self.v1 }
    pub fn v_last(&self) -> f64 { self.v2 }

    fn clamp_u(&self, u: f64) -> f64 {
        if self.u_trim { u.clamp(self.u1, self.u2) } else { u }
    }

    fn clamp_v(&self, v: f64) -> f64 {
        if self.v_trim { v.clamp(self.v1, self.v2) } else { v }
    }

    pub fn value(&self, u: f64, v: f64) -> Pnt {
        self.basis.value(self.clamp_u(u), self.clamp_v(v))
    }

    pub fn d1(&self, u: f64, v: f64) -> (Pnt, Vec3, Vec3) {
        self.basis.d1(self.clamp_u(u), self.clamp_v(v))
    }

    pub fn set_trim_u(&mut self, u1: f64, u2: f64) {
        let (u1, u2) = if u1 <= u2 { (u1, u2) } else { (u2, u1) };
        self.u1 = u1;
        self.u2 = u2;
        self.u_trim = true;
    }

    pub fn set_trim_v(&mut self, v1: f64, v2: f64) {
        let (v1, v2) = if v1 <= v2 { (v1, v2) } else { (v2, v1) };
        self.v1 = v1;
        self.v2 = v2;
        self.v_trim = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn xy_plane() -> GeomPlane {
        use crate::gp::Ax3;
        GeomPlane::from_ax3(Ax3::identity())
    }

    #[test]
    fn test_plane_trim_bounds() {
        let plane = xy_plane();
        let surf = GeomRectangularTrimmedSurface::new(
            GeomSurfaceKind::Plane(plane),
            -1.0, 1.0, -2.0, 2.0,
            true, true,
        );
        assert!((surf.u_first() - (-1.0)).abs() < 1e-14);
        assert!((surf.u_last() - 1.0).abs() < 1e-14);
        assert!((surf.v_first() - (-2.0)).abs() < 1e-14);
        assert!((surf.v_last() - 2.0).abs() < 1e-14);
    }

    #[test]
    fn test_plane_trim_bounds_reversed_input() {
        let plane = xy_plane();
        let surf = GeomRectangularTrimmedSurface::new(
            GeomSurfaceKind::Plane(plane),
            1.0, -1.0, 2.0, -2.0,
            true, true,
        );
        assert!((surf.u_first() - (-1.0)).abs() < 1e-14);
        assert!((surf.u_last() - 1.0).abs() < 1e-14);
        assert!((surf.v_first() - (-2.0)).abs() < 1e-14);
        assert!((surf.v_last() - 2.0).abs() < 1e-14);
    }

    #[test]
    fn test_plane_value_interior() {
        let plane = xy_plane();
        let surf = GeomRectangularTrimmedSurface::new(
            GeomSurfaceKind::Plane(plane),
            -1.0, 1.0, -1.0, 1.0,
            true, true,
        );
        let p = surf.value(0.5, 0.3);
        assert!((p.x - 0.5).abs() < 1e-12, "x={}", p.x);
        assert!((p.y - 0.3).abs() < 1e-12, "y={}", p.y);
        assert!(p.z.abs() < 1e-12, "z={}", p.z);
    }

    #[test]
    fn test_plane_value_clamped_u() {
        let plane = xy_plane();
        let surf = GeomRectangularTrimmedSurface::new(
            GeomSurfaceKind::Plane(plane),
            -1.0, 1.0, -1.0, 1.0,
            true, true,
        );
        let p = surf.value(2.0, 0.0);
        assert!((p.x - 1.0).abs() < 1e-12, "x should be 1.0, got {}", p.x);
    }

    #[test]
    fn test_plane_value_clamped_v() {
        let plane = xy_plane();
        let surf = GeomRectangularTrimmedSurface::new(
            GeomSurfaceKind::Plane(plane),
            -1.0, 1.0, -1.0, 1.0,
            true, true,
        );
        let p = surf.value(0.0, -5.0);
        assert!((p.y - (-1.0)).abs() < 1e-12, "y should be -1.0, got {}", p.y);
    }

    #[test]
    fn test_no_trim_flag_passes_through() {
        let plane = xy_plane();
        let surf = GeomRectangularTrimmedSurface::new(
            GeomSurfaceKind::Plane(plane),
            -1.0, 1.0, -1.0, 1.0,
            false, false,
        );
        let p = surf.value(2.0, 3.0);
        assert!((p.x - 2.0).abs() < 1e-12);
        assert!((p.y - 3.0).abs() < 1e-12);
    }

    #[test]
    fn test_plane_d1() {
        let plane = xy_plane();
        let surf = GeomRectangularTrimmedSurface::new(
            GeomSurfaceKind::Plane(plane),
            -1.0, 1.0, -1.0, 1.0,
            true, true,
        );
        let (p, du, dv) = surf.d1(0.0, 0.0);
        assert!(p.z.abs() < 1e-12);
        assert!((du.x - 1.0).abs() < 1e-12, "du.x={}", du.x);
        assert!(du.y.abs() < 1e-12, "du.y={}", du.y);
        assert!(du.z.abs() < 1e-12, "du.z={}", du.z);
        assert!(dv.x.abs() < 1e-12, "dv.x={}", dv.x);
        assert!((dv.y - 1.0).abs() < 1e-12, "dv.y={}", dv.y);
        assert!(dv.z.abs() < 1e-12, "dv.z={}", dv.z);
    }

    #[test]
    fn test_set_trim_u() {
        let plane = xy_plane();
        let mut surf = GeomRectangularTrimmedSurface::new(
            GeomSurfaceKind::Plane(plane),
            -1.0, 1.0, -1.0, 1.0,
            true, true,
        );
        surf.set_trim_u(0.0, 3.0);
        assert!((surf.u_first() - 0.0).abs() < 1e-14);
        assert!((surf.u_last() - 3.0).abs() < 1e-14);
        let p = surf.value(5.0, 0.0);
        assert!((p.x - 3.0).abs() < 1e-12);
    }

    #[test]
    fn test_set_trim_v() {
        let plane = xy_plane();
        let mut surf = GeomRectangularTrimmedSurface::new(
            GeomSurfaceKind::Plane(plane),
            -1.0, 1.0, -1.0, 1.0,
            true, true,
        );
        surf.set_trim_v(0.5, 2.5);
        assert!((surf.v_first() - 0.5).abs() < 1e-14);
        assert!((surf.v_last() - 2.5).abs() < 1e-14);
        let p = surf.value(0.0, 0.0);
        assert!((p.y - 0.5).abs() < 1e-12);
    }

    #[test]
    fn test_set_trim_u_reversed() {
        let plane = xy_plane();
        let mut surf = GeomRectangularTrimmedSurface::new(
            GeomSurfaceKind::Plane(plane),
            -1.0, 1.0, -1.0, 1.0,
            true, true,
        );
        surf.set_trim_u(3.0, 0.0);
        assert!((surf.u_first() - 0.0).abs() < 1e-14);
        assert!((surf.u_last() - 3.0).abs() < 1e-14);
    }

    #[test]
    fn test_sphere_value() {
        let sphere = GeomSphericalSurface::new([0.0, 0.0, 0.0], 1.0);
        let surf = GeomRectangularTrimmedSurface::new(
            GeomSurfaceKind::Sphere(sphere),
            0.0, std::f64::consts::PI,
            -std::f64::consts::FRAC_PI_2, std::f64::consts::FRAC_PI_2,
            true, true,
        );
        let p = surf.value(0.0, 0.0);
        assert!((p.x - 1.0).abs() < 1e-12, "x={}", p.x);
        assert!(p.y.abs() < 1e-12, "y={}", p.y);
        assert!(p.z.abs() < 1e-12, "z={}", p.z);
    }

    #[test]
    fn test_sphere_north_pole() {
        let sphere = GeomSphericalSurface::new([0.0, 0.0, 0.0], 2.0);
        let surf = GeomRectangularTrimmedSurface::new(
            GeomSurfaceKind::Sphere(sphere),
            0.0, 2.0 * std::f64::consts::PI,
            -std::f64::consts::FRAC_PI_2, std::f64::consts::FRAC_PI_2,
            true, true,
        );
        let p = surf.value(0.0, std::f64::consts::FRAC_PI_2);
        assert!(p.x.abs() < 1e-12);
        assert!(p.y.abs() < 1e-12);
        assert!((p.z - 2.0).abs() < 1e-12);
    }

    #[test]
    fn test_cylinder_value() {
        let cyl = GeomCylindricalSurface::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 1.0);
        let surf = GeomRectangularTrimmedSurface::new(
            GeomSurfaceKind::Cylinder(cyl),
            0.0, std::f64::consts::PI,
            0.0, 5.0,
            true, true,
        );
        let p = surf.value(0.0, 0.0);
        assert!((p.x - 1.0).abs() < 1e-12, "x={}", p.x);
        assert!(p.y.abs() < 1e-12, "y={}", p.y);
        assert!(p.z.abs() < 1e-12, "z={}", p.z);
    }

    #[test]
    fn test_cylinder_height() {
        let cyl = GeomCylindricalSurface::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 1.0);
        let surf = GeomRectangularTrimmedSurface::new(
            GeomSurfaceKind::Cylinder(cyl),
            0.0, std::f64::consts::TAU,
            0.0, 5.0,
            true, true,
        );
        let p = surf.value(0.0, 3.0);
        assert!((p.x - 1.0).abs() < 1e-12);
        assert!(p.y.abs() < 1e-12);
        assert!((p.z - 3.0).abs() < 1e-12);
    }

    #[test]
    fn test_torus_value() {
        let torus = GeomToroidalSurface::new([0.0, 0.0, 0.0], 3.0, 1.0);
        let surf = GeomRectangularTrimmedSurface::new(
            GeomSurfaceKind::Torus(torus),
            0.0, std::f64::consts::TAU,
            0.0, std::f64::consts::TAU,
            true, true,
        );
        let p = surf.value(0.0, 0.0);
        assert!((p.x - 4.0).abs() < 1e-12);
        assert!(p.y.abs() < 1e-12);
        assert!(p.z.abs() < 1e-12);
    }

    #[test]
    fn test_cone_value_at_apex() {
        let cone = GeomConicalSurface::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 0.0, std::f64::consts::FRAC_PI_4);
        let surf = GeomRectangularTrimmedSurface::new(
            GeomSurfaceKind::Cone(cone),
            0.0, std::f64::consts::TAU,
            0.0, 2.0,
            true, true,
        );
        let p = surf.value(0.0, 0.0);
        assert!(p.x.abs() < 1e-12);
        assert!(p.y.abs() < 1e-12);
        assert!(p.z.abs() < 1e-12);
    }

    #[test]
    fn test_cone_value_at_height() {
        let cone = GeomConicalSurface::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 0.0, std::f64::consts::FRAC_PI_4);
        let surf = GeomRectangularTrimmedSurface::new(
            GeomSurfaceKind::Cone(cone),
            0.0, std::f64::consts::TAU,
            0.0, 2.0,
            true, true,
        );
        let p = surf.value(0.0, 1.0);
        assert!((p.x - 1.0).abs() < 1e-12, "x={}", p.x);
        assert!(p.y.abs() < 1e-12, "y={}", p.y);
        assert!((p.z - 1.0).abs() < 1e-12, "z={}", p.z);
    }

    #[test]
    fn test_sphere_d1() {
        let sphere = GeomSphericalSurface::new([0.0, 0.0, 0.0], 1.0);
        let surf = GeomRectangularTrimmedSurface::new(
            GeomSurfaceKind::Sphere(sphere),
            0.0, std::f64::consts::TAU,
            -std::f64::consts::FRAC_PI_2, std::f64::consts::FRAC_PI_2,
            true, true,
        );
        let (_p, du, dv) = surf.d1(0.0, 0.0);
        assert!(du.x.abs() < 1e-12);
        assert!((du.y - 1.0).abs() < 1e-12);
        assert!(du.z.abs() < 1e-12);
        assert!(dv.x.abs() < 1e-12);
        assert!(dv.y.abs() < 1e-12);
        assert!((dv.z - 1.0).abs() < 1e-12);
    }

    #[test]
    fn test_clamp_at_boundary() {
        let plane = xy_plane();
        let surf = GeomRectangularTrimmedSurface::new(
            GeomSurfaceKind::Plane(plane),
            0.0, 1.0, 0.0, 1.0,
            true, true,
        );
        let p = surf.value(1.0, 1.0);
        assert!((p.x - 1.0).abs() < 1e-12);
        assert!((p.y - 1.0).abs() < 1e-12);
    }
}
