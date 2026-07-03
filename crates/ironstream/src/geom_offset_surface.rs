// FILE: geom_offset_surface.rs
// occt: Geom_OffsetSurface

use crate::geom_plane::GeomPlane;
use crate::geom_bspline_surface::GeomBSplineSurface;

// occt: Geom_Surface (trait for surface evaluation)
pub trait GeomSurfaceEval {
    fn value(&self, u: f64, v: f64) -> [f64; 3];
    fn d1(&self, u: f64, v: f64) -> ([f64; 3], [f64; 3], [f64; 3]); // P, dU, dV
    fn normal(&self, u: f64, v: f64) -> [f64; 3]; // unit normal = dU x dV / |dU x dV|
}

fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

fn norm(v: [f64; 3]) -> f64 {
    (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt()
}

fn normalize(v: [f64; 3]) -> [f64; 3] {
    let n = norm(v);
    if n < 1e-12 {
        [0.0, 0.0, 0.0]
    } else {
        [v[0] / n, v[1] / n, v[2] / n]
    }
}

fn add3(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

fn scale3(v: [f64; 3], s: f64) -> [f64; 3] {
    [v[0] * s, v[1] * s, v[2] * s]
}

fn sub3(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

// occt: Geom_Plane (wrapper for offset surface support)
#[derive(Clone, Debug)]
pub struct GeomOffsetPlane {
    /// Origin of the plane
    pub origin: [f64; 3],
    /// First axis direction (u direction)
    pub u_dir: [f64; 3],
    /// Second axis direction (v direction)
    pub v_dir: [f64; 3],
    /// Normal direction (precomputed)
    pub normal: [f64; 3],
}

impl GeomOffsetPlane {
    pub fn new(origin: [f64; 3], u_dir: [f64; 3], v_dir: [f64; 3]) -> Self {
        let n = normalize(cross(u_dir, v_dir));
        Self {
            origin,
            u_dir: normalize(u_dir),
            v_dir: normalize(v_dir),
            normal: n,
        }
    }

    /// Construct from origin and normal; v_dir is derived from normal and an up vector.
    pub fn from_origin_normal(origin: [f64; 3], normal: [f64; 3]) -> Self {
        let n = normalize(normal);
        // pick an up vector not parallel to n
        let up = if n[2].abs() < 0.9 {
            [0.0, 0.0, 1.0]
        } else {
            [1.0, 0.0, 0.0]
        };
        let u = normalize(cross(up, n));
        let v = normalize(cross(n, u));
        Self {
            origin,
            u_dir: u,
            v_dir: v,
            normal: n,
        }
    }
}

impl GeomSurfaceEval for GeomOffsetPlane {
    fn value(&self, u: f64, v: f64) -> [f64; 3] {
        [
            self.origin[0] + u * self.u_dir[0] + v * self.v_dir[0],
            self.origin[1] + u * self.u_dir[1] + v * self.v_dir[1],
            self.origin[2] + u * self.u_dir[2] + v * self.v_dir[2],
        ]
    }

    fn d1(&self, u: f64, v: f64) -> ([f64; 3], [f64; 3], [f64; 3]) {
        let p = self.value(u, v);
        (p, self.u_dir, self.v_dir)
    }

    fn normal(&self, _u: f64, _v: f64) -> [f64; 3] {
        self.normal
    }
}

// occt: Geom_CylindricalSurface (wrapper for offset surface support)
#[derive(Clone, Debug)]
pub struct GeomOffsetCylinder {
    /// Center axis origin
    pub origin: [f64; 3],
    /// Axis direction (unit)
    pub axis: [f64; 3],
    /// Reference direction (u=0 direction, unit)
    pub x_dir: [f64; 3],
    /// Y direction completing right-hand frame
    pub y_dir: [f64; 3],
    /// Radius
    pub radius: f64,
}

impl GeomOffsetCylinder {
    /// Create a cylinder with given origin, axis, x_dir and radius.
    /// The axis and x_dir should be orthogonal.
    pub fn new(origin: [f64; 3], axis: [f64; 3], x_dir: [f64; 3], radius: f64) -> Self {
        let a = normalize(axis);
        let x = normalize(x_dir);
        let y = normalize(cross(a, x));
        Self {
            origin,
            axis: a,
            x_dir: x,
            y_dir: y,
            radius,
        }
    }

    /// Standard cylinder along Z axis with radius.
    pub fn along_z(origin: [f64; 3], radius: f64) -> Self {
        Self::new(origin, [0.0, 0.0, 1.0], [1.0, 0.0, 0.0], radius)
    }
}

impl GeomSurfaceEval for GeomOffsetCylinder {
    /// u is the angle parameter (radians), v is the height along axis.
    fn value(&self, u: f64, v: f64) -> [f64; 3] {
        let cos_u = u.cos();
        let sin_u = u.sin();
        [
            self.origin[0]
                + self.radius * (cos_u * self.x_dir[0] + sin_u * self.y_dir[0])
                + v * self.axis[0],
            self.origin[1]
                + self.radius * (cos_u * self.x_dir[1] + sin_u * self.y_dir[1])
                + v * self.axis[1],
            self.origin[2]
                + self.radius * (cos_u * self.x_dir[2] + sin_u * self.y_dir[2])
                + v * self.axis[2],
        ]
    }

    fn d1(&self, u: f64, v: f64) -> ([f64; 3], [f64; 3], [f64; 3]) {
        let p = self.value(u, v);
        let cos_u = u.cos();
        let sin_u = u.sin();
        // dP/du = radius * (-sin(u)*x_dir + cos(u)*y_dir)
        let du = [
            self.radius * (-sin_u * self.x_dir[0] + cos_u * self.y_dir[0]),
            self.radius * (-sin_u * self.x_dir[1] + cos_u * self.y_dir[1]),
            self.radius * (-sin_u * self.x_dir[2] + cos_u * self.y_dir[2]),
        ];
        // dP/dv = axis
        let dv = self.axis;
        (p, du, dv)
    }

    fn normal(&self, u: f64, _v: f64) -> [f64; 3] {
        // outward radial direction
        let cos_u = u.cos();
        let sin_u = u.sin();
        normalize([
            cos_u * self.x_dir[0] + sin_u * self.y_dir[0],
            cos_u * self.x_dir[1] + sin_u * self.y_dir[1],
            cos_u * self.x_dir[2] + sin_u * self.y_dir[2],
        ])
    }
}

// occt: Geom_SphericalSurface (wrapper for offset surface support)
#[derive(Clone, Debug)]
pub struct GeomOffsetSphere {
    /// Center
    pub center: [f64; 3],
    /// Reference x direction
    pub x_dir: [f64; 3],
    /// Reference y direction
    pub y_dir: [f64; 3],
    /// Axis (z) direction
    pub z_dir: [f64; 3],
    /// Radius
    pub radius: f64,
}

impl GeomOffsetSphere {
    /// Sphere centered at center with given radius; standard orientation.
    pub fn new(center: [f64; 3], radius: f64) -> Self {
        Self {
            center,
            x_dir: [1.0, 0.0, 0.0],
            y_dir: [0.0, 1.0, 0.0],
            z_dir: [0.0, 0.0, 1.0],
            radius,
        }
    }

    /// Sphere with explicit frame.
    pub fn with_frame(
        center: [f64; 3],
        x_dir: [f64; 3],
        y_dir: [f64; 3],
        z_dir: [f64; 3],
        radius: f64,
    ) -> Self {
        Self {
            center,
            x_dir: normalize(x_dir),
            y_dir: normalize(y_dir),
            z_dir: normalize(z_dir),
            radius,
        }
    }
}

impl GeomSurfaceEval for GeomOffsetSphere {
    /// u is longitude (azimuth, radians), v is latitude (elevation, radians).
    /// P = center + R*(cos(v)*cos(u)*x + cos(v)*sin(u)*y + sin(v)*z)
    fn value(&self, u: f64, v: f64) -> [f64; 3] {
        let cos_v = v.cos();
        let sin_v = v.sin();
        let cos_u = u.cos();
        let sin_u = u.sin();
        let rx = self.radius * cos_v * cos_u;
        let ry = self.radius * cos_v * sin_u;
        let rz = self.radius * sin_v;
        [
            self.center[0] + rx * self.x_dir[0] + ry * self.y_dir[0] + rz * self.z_dir[0],
            self.center[1] + rx * self.x_dir[1] + ry * self.y_dir[1] + rz * self.z_dir[1],
            self.center[2] + rx * self.x_dir[2] + ry * self.y_dir[2] + rz * self.z_dir[2],
        ]
    }

    fn d1(&self, u: f64, v: f64) -> ([f64; 3], [f64; 3], [f64; 3]) {
        let p = self.value(u, v);
        let cos_v = v.cos();
        let sin_v = v.sin();
        let cos_u = u.cos();
        let sin_u = u.sin();
        // dP/du = R*cos(v)*(-sin(u)*x + cos(u)*y)
        let du_x = self.radius * cos_v * (-sin_u);
        let du_y = self.radius * cos_v * cos_u;
        let du_z = 0.0_f64;
        let du = [
            du_x * self.x_dir[0] + du_y * self.y_dir[0] + du_z * self.z_dir[0],
            du_x * self.x_dir[1] + du_y * self.y_dir[1] + du_z * self.z_dir[1],
            du_x * self.x_dir[2] + du_y * self.y_dir[2] + du_z * self.z_dir[2],
        ];
        // dP/dv = R*(-sin(v)*cos(u)*x - sin(v)*sin(u)*y + cos(v)*z)
        let dv_x = self.radius * (-sin_v) * cos_u;
        let dv_y = self.radius * (-sin_v) * sin_u;
        let dv_z = self.radius * cos_v;
        let dv = [
            dv_x * self.x_dir[0] + dv_y * self.y_dir[0] + dv_z * self.z_dir[0],
            dv_x * self.x_dir[1] + dv_y * self.y_dir[1] + dv_z * self.z_dir[1],
            dv_x * self.x_dir[2] + dv_y * self.y_dir[2] + dv_z * self.z_dir[2],
        ];
        (p, du, dv)
    }

    fn normal(&self, u: f64, v: f64) -> [f64; 3] {
        // outward normal for a sphere is the radial direction
        let p = self.value(u, v);
        normalize(sub3(p, self.center))
    }
}

// occt: Geom_BasisSurface (enum of supported surface kinds for offset)
pub enum GeomBasisSurface {
    Plane(GeomOffsetPlane),
    Cylinder(GeomOffsetCylinder),
    Sphere(GeomOffsetSphere),
}

impl GeomSurfaceEval for GeomBasisSurface {
    fn value(&self, u: f64, v: f64) -> [f64; 3] {
        match self {
            GeomBasisSurface::Plane(s) => s.value(u, v),
            GeomBasisSurface::Cylinder(s) => s.value(u, v),
            GeomBasisSurface::Sphere(s) => s.value(u, v),
        }
    }

    fn d1(&self, u: f64, v: f64) -> ([f64; 3], [f64; 3], [f64; 3]) {
        match self {
            GeomBasisSurface::Plane(s) => s.d1(u, v),
            GeomBasisSurface::Cylinder(s) => s.d1(u, v),
            GeomBasisSurface::Sphere(s) => s.d1(u, v),
        }
    }

    fn normal(&self, u: f64, v: f64) -> [f64; 3] {
        match self {
            GeomBasisSurface::Plane(s) => s.normal(u, v),
            GeomBasisSurface::Cylinder(s) => s.normal(u, v),
            GeomBasisSurface::Sphere(s) => s.normal(u, v),
        }
    }
}

/// Compute the unit normal from two tangent vectors using cross product.
/// Returns zero vector if the cross product is degenerate.
pub fn compute_normal(du: [f64; 3], dv: [f64; 3]) -> [f64; 3] {
    normalize(cross(du, dv))
}

/// Derivative of offset surface with respect to u parameter.
/// Uses finite differences on the normal field for the normal derivative term.
/// For the offset surface Q(u,v) = P(u,v) + d * N(u,v),
/// dQ/du = dP/du + d * dN/du
/// dN/du is approximated numerically using central differences.
fn offset_d1_u(
    basis: &dyn GeomSurfaceEval,
    u: f64,
    v: f64,
    offset: f64,
) -> [f64; 3] {
    let h = 1e-7_f64;
    let n_plus = basis.normal(u + h, v);
    let n_minus = basis.normal(u - h, v);
    let dn_du = [
        (n_plus[0] - n_minus[0]) / (2.0 * h),
        (n_plus[1] - n_minus[1]) / (2.0 * h),
        (n_plus[2] - n_minus[2]) / (2.0 * h),
    ];
    let (_, du, _) = basis.d1(u, v);
    add3(du, scale3(dn_du, offset))
}

/// Derivative of offset surface with respect to v parameter.
fn offset_d1_v(
    basis: &dyn GeomSurfaceEval,
    u: f64,
    v: f64,
    offset: f64,
) -> [f64; 3] {
    let h = 1e-7_f64;
    let n_plus = basis.normal(u, v + h);
    let n_minus = basis.normal(u, v - h);
    let dn_dv = [
        (n_plus[0] - n_minus[0]) / (2.0 * h),
        (n_plus[1] - n_minus[1]) / (2.0 * h),
        (n_plus[2] - n_minus[2]) / (2.0 * h),
    ];
    let (_, _, dv) = basis.d1(u, v);
    add3(dv, scale3(dn_dv, offset))
}

// occt: Geom_OffsetSurface
/// A surface offset by a fixed distance along its normal.
/// Implements the same semantics as OCCT's Geom_OffsetSurface.
pub struct GeomOffsetSurface {
    basis: GeomBasisSurface,
    offset: f64,
    osculat_flag: bool,
}

impl GeomOffsetSurface {
    /// Create an offset surface from a plane.
    pub fn new_with_plane(plane: GeomOffsetPlane, offset: f64) -> Self {
        Self {
            basis: GeomBasisSurface::Plane(plane),
            offset,
            osculat_flag: false,
        }
    }

    /// Create an offset surface from a cylinder.
    pub fn new_with_cylinder(cyl: GeomOffsetCylinder, offset: f64) -> Self {
        Self {
            basis: GeomBasisSurface::Cylinder(cyl),
            offset,
            osculat_flag: false,
        }
    }

    /// Create an offset surface from a sphere.
    pub fn new_with_sphere(sph: GeomOffsetSphere, offset: f64) -> Self {
        Self {
            basis: GeomBasisSurface::Sphere(sph),
            offset,
            osculat_flag: false,
        }
    }

    /// Set the osculating surface flag.
    /// When true, the offset is computed using the osculating surface at degenerate points.
    pub fn set_osculat_flag(&mut self, flag: bool) {
        self.osculat_flag = flag;
    }

    /// Return the signed offset distance.
    pub fn offset_value(&self) -> f64 {
        self.offset
    }

    /// Return a reference to the basis surface evaluator.
    pub fn basis_surface_offset(&self) -> &GeomBasisSurface {
        &self.basis
    }

    /// Evaluate the offset surface point: Q(u,v) = P(u,v) + d * N(u,v)
    pub fn value(&self, u: f64, v: f64) -> [f64; 3] {
        let p = self.basis.value(u, v);
        let n = self.basis.normal(u, v);
        add3(p, scale3(n, self.offset))
    }

    /// Evaluate first derivatives of the offset surface.
    /// Returns (Q, dQ/du, dQ/dv).
    /// dQ/du = dP/du + d * dN/du  (normal derivative via finite differences)
    /// dQ/dv = dP/dv + d * dN/dv
    pub fn d1(&self, u: f64, v: f64) -> ([f64; 3], [f64; 3], [f64; 3]) {
        let q = self.value(u, v);
        let dqu = offset_d1_u(&self.basis, u, v, self.offset);
        let dqv = offset_d1_v(&self.basis, u, v, self.offset);
        (q, dqu, dqv)
    }

    /// Compute the unit normal to the offset surface at (u,v).
    pub fn normal(&self, u: f64, v: f64) -> [f64; 3] {
        let (_, du, dv) = self.d1(u, v);
        compute_normal(du, dv)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx_eq(a: f64, b: f64, tol: f64) -> bool {
        (a - b).abs() < tol
    }

    fn vec3_approx_eq(a: [f64; 3], b: [f64; 3], tol: f64) -> bool {
        approx_eq(a[0], b[0], tol) && approx_eq(a[1], b[1], tol) && approx_eq(a[2], b[2], tol)
    }

    // --- GeomOffsetPlane tests ---

    #[test]
    fn test_plane_value() {
        // XY plane at origin
        let plane = GeomOffsetPlane::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
        let p = plane.value(3.0, 4.0);
        assert!(vec3_approx_eq(p, [3.0, 4.0, 0.0], 1e-10));
    }

    #[test]
    fn test_plane_normal() {
        let plane = GeomOffsetPlane::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
        let n = plane.normal(0.0, 0.0);
        // Normal of XY plane should be +Z
        assert!(vec3_approx_eq(n, [0.0, 0.0, 1.0], 1e-10));
    }

    #[test]
    fn test_plane_d1() {
        let plane = GeomOffsetPlane::new([1.0, 2.0, 3.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
        let (p, du, dv) = plane.d1(5.0, 7.0);
        // P should be origin + u*x + v*y
        assert!(vec3_approx_eq(p, [6.0, 9.0, 3.0], 1e-10));
        assert!(vec3_approx_eq(du, [1.0, 0.0, 0.0], 1e-10));
        assert!(vec3_approx_eq(dv, [0.0, 1.0, 0.0], 1e-10));
    }

    // --- Offset plane tests ---

    #[test]
    fn test_offset_plane_value() {
        // XY plane offset by 5 along +Z
        let plane = GeomOffsetPlane::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
        let surf = GeomOffsetSurface::new_with_plane(plane, 5.0);
        let q = surf.value(3.0, 4.0);
        // Should be (3, 4, 5)
        assert!(vec3_approx_eq(q, [3.0, 4.0, 5.0], 1e-7));
    }

    #[test]
    fn test_offset_plane_value_negative() {
        // XY plane offset by -2 along +Z (so offset in -Z direction)
        let plane = GeomOffsetPlane::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
        let surf = GeomOffsetSurface::new_with_plane(plane, -2.0);
        let q = surf.value(1.0, 1.0);
        assert!(vec3_approx_eq(q, [1.0, 1.0, -2.0], 1e-7));
    }

    #[test]
    fn test_offset_plane_d1() {
        // Offset plane: derivatives should still be same as basis (normal doesn't change)
        let plane = GeomOffsetPlane::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
        let surf = GeomOffsetSurface::new_with_plane(plane, 3.0);
        let (q, du, dv) = surf.d1(2.0, 2.0);
        // For a flat plane, dN/du = dN/dv = 0, so offset derivs = basis derivs
        assert!(vec3_approx_eq(q, [2.0, 2.0, 3.0], 1e-7));
        assert!(vec3_approx_eq(du, [1.0, 0.0, 0.0], 1e-5));
        assert!(vec3_approx_eq(dv, [0.0, 1.0, 0.0], 1e-5));
    }

    #[test]
    fn test_offset_plane_normal() {
        // Offset plane has same normal as original plane
        let plane = GeomOffsetPlane::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
        let surf = GeomOffsetSurface::new_with_plane(plane, 5.0);
        let n = surf.normal(1.0, 1.0);
        assert!(vec3_approx_eq(n, [0.0, 0.0, 1.0], 1e-5));
    }

    // --- GeomOffsetCylinder tests ---

    #[test]
    fn test_cylinder_value_at_zero() {
        // Cylinder of radius 3 along Z axis
        let cyl = GeomOffsetCylinder::along_z([0.0, 0.0, 0.0], 3.0);
        let p = cyl.value(0.0, 0.0); // angle=0, height=0 -> (3,0,0)
        assert!(vec3_approx_eq(p, [3.0, 0.0, 0.0], 1e-10));
    }

    #[test]
    fn test_cylinder_value_at_pi_half() {
        let cyl = GeomOffsetCylinder::along_z([0.0, 0.0, 0.0], 4.0);
        let p = cyl.value(std::f64::consts::PI / 2.0, 2.0);
        // angle=pi/2 -> (0, 4, 2)
        assert!(approx_eq(p[0], 0.0, 1e-10));
        assert!(approx_eq(p[1], 4.0, 1e-10));
        assert!(approx_eq(p[2], 2.0, 1e-10));
    }

    #[test]
    fn test_cylinder_normal_outward() {
        let cyl = GeomOffsetCylinder::along_z([0.0, 0.0, 0.0], 5.0);
        // At angle=0, height=0, normal should point in +X
        let n = cyl.normal(0.0, 0.0);
        assert!(vec3_approx_eq(n, [1.0, 0.0, 0.0], 1e-10));
        // At angle=pi, normal should point in -X
        let n2 = cyl.normal(std::f64::consts::PI, 0.0);
        assert!(approx_eq(n2[0], -1.0, 1e-10));
        assert!(approx_eq(n2[1], 0.0, 1e-10));
    }

    // --- Offset cylinder tests ---

    #[test]
    fn test_offset_cylinder_radius_increase() {
        // Offsetting a cylinder of radius R by d should give a cylinder of radius R+d
        let r = 5.0_f64;
        let d = 2.0_f64;
        let cyl = GeomOffsetCylinder::along_z([0.0, 0.0, 0.0], r);
        let surf = GeomOffsetSurface::new_with_cylinder(cyl, d);

        // Check at several angles
        for i in 0..8 {
            let angle = i as f64 * std::f64::consts::PI / 4.0;
            let q = surf.value(angle, 0.0);
            // Distance from Z axis should be R+d
            let dist = (q[0] * q[0] + q[1] * q[1]).sqrt();
            assert!(
                approx_eq(dist, r + d, 1e-7),
                "Expected radius {} got {} at angle {}",
                r + d,
                dist,
                angle
            );
        }
    }

    #[test]
    fn test_offset_cylinder_radius_decrease() {
        // Offsetting inward (negative offset) reduces radius
        let r = 10.0_f64;
        let d = -3.0_f64;
        let cyl = GeomOffsetCylinder::along_z([0.0, 0.0, 0.0], r);
        let surf = GeomOffsetSurface::new_with_cylinder(cyl, d);

        let angle = std::f64::consts::PI / 3.0;
        let q = surf.value(angle, 5.0);
        let dist = (q[0] * q[0] + q[1] * q[1]).sqrt();
        assert!(approx_eq(dist, r + d, 1e-7));
    }

    #[test]
    fn test_offset_cylinder_preserves_height() {
        // Height along the axis should be preserved
        let r = 3.0_f64;
        let d = 1.0_f64;
        let h = 7.5_f64;
        let cyl = GeomOffsetCylinder::along_z([0.0, 0.0, 0.0], r);
        let surf = GeomOffsetSurface::new_with_cylinder(cyl, d);

        let q = surf.value(0.0, h);
        // Z component should equal h (offset is radial only for cylinder)
        assert!(approx_eq(q[2], h, 1e-7));
    }

    #[test]
    fn test_offset_cylinder_d1() {
        let r = 4.0_f64;
        let d = 1.0_f64;
        let cyl = GeomOffsetCylinder::along_z([0.0, 0.0, 0.0], r);
        let surf = GeomOffsetSurface::new_with_cylinder(cyl, d);

        // Verify d1 is consistent with finite differences
        let u = 0.5_f64;
        let v = 1.0_f64;
        let (q, du, dv) = surf.d1(u, v);
        let eps = 1e-5_f64;
        let q_plus_u = surf.value(u + eps, v);
        let q_minus_u = surf.value(u - eps, v);
        let fd_du = [
            (q_plus_u[0] - q_minus_u[0]) / (2.0 * eps),
            (q_plus_u[1] - q_minus_u[1]) / (2.0 * eps),
            (q_plus_u[2] - q_minus_u[2]) / (2.0 * eps),
        ];
        assert!(
            vec3_approx_eq(du, fd_du, 1e-4),
            "du mismatch: {:?} vs {:?}",
            du,
            fd_du
        );

        let q_plus_v = surf.value(u, v + eps);
        let q_minus_v = surf.value(u, v - eps);
        let fd_dv = [
            (q_plus_v[0] - q_minus_v[0]) / (2.0 * eps),
            (q_plus_v[1] - q_minus_v[1]) / (2.0 * eps),
            (q_plus_v[2] - q_minus_v[2]) / (2.0 * eps),
        ];
        assert!(
            vec3_approx_eq(dv, fd_dv, 1e-4),
            "dv mismatch: {:?} vs {:?}",
            dv,
            fd_dv
        );
    }

    // --- Sphere tests ---

    #[test]
    fn test_sphere_value_poles() {
        let sph = GeomOffsetSphere::new([0.0, 0.0, 0.0], 5.0);
        // North pole: v = pi/2 -> (0,0,5)
        let p_north = sph.value(0.0, std::f64::consts::PI / 2.0);
        assert!(approx_eq(p_north[0], 0.0, 1e-10));
        assert!(approx_eq(p_north[1], 0.0, 1e-10));
        assert!(approx_eq(p_north[2], 5.0, 1e-10));

        // South pole: v = -pi/2 -> (0,0,-5)
        let p_south = sph.value(0.0, -std::f64::consts::PI / 2.0);
        assert!(approx_eq(p_south[2], -5.0, 1e-10));
    }

    #[test]
    fn test_sphere_value_equator() {
        let sph = GeomOffsetSphere::new([0.0, 0.0, 0.0], 3.0);
        // At equator (v=0), angle u=0: (3,0,0)
        let p = sph.value(0.0, 0.0);
        assert!(vec3_approx_eq(p, [3.0, 0.0, 0.0], 1e-10));
        // angle u=pi/2: (0,3,0)
        let p2 = sph.value(std::f64::consts::PI / 2.0, 0.0);
        assert!(approx_eq(p2[0], 0.0, 1e-10));
        assert!(approx_eq(p2[1], 3.0, 1e-10));
    }

    #[test]
    fn test_sphere_points_on_surface() {
        // All points should be at radius R from center
        let r = 7.0_f64;
        let sph = GeomOffsetSphere::new([1.0, 2.0, 3.0], r);
        let center = [1.0, 2.0, 3.0];
        for i in 0..6 {
            for j in 0..4 {
                let u = i as f64 * std::f64::consts::PI / 3.0;
                let v = -std::f64::consts::PI / 2.0 + j as f64 * std::f64::consts::PI / 3.0;
                let p = sph.value(u, v);
                let d = sub3(p, center);
                let dist = norm(d);
                assert!(
                    approx_eq(dist, r, 1e-10),
                    "Point not on sphere: dist={} at u={}, v={}",
                    dist,
                    u,
                    v
                );
            }
        }
    }

    #[test]
    fn test_sphere_normal_outward() {
        let sph = GeomOffsetSphere::new([0.0, 0.0, 0.0], 4.0);
        // Normal at equator u=0 should be +X
        let n = sph.normal(0.0, 0.0);
        assert!(vec3_approx_eq(n, [1.0, 0.0, 0.0], 1e-10));
        // Normal at north pole should be +Z
        let n2 = sph.normal(0.0, std::f64::consts::PI / 2.0);
        assert!(approx_eq(n2[2], 1.0, 1e-7));
    }

    // --- Offset sphere tests ---

    #[test]
    fn test_offset_sphere_radius_increase() {
        // Offsetting a sphere of radius R by d should give a sphere of radius R+d
        let r = 5.0_f64;
        let d = 2.0_f64;
        let sph = GeomOffsetSphere::new([0.0, 0.0, 0.0], r);
        let surf = GeomOffsetSurface::new_with_sphere(sph, d);

        let expected_r = r + d;
        let center = [0.0, 0.0, 0.0];

        // Check multiple points
        for i in 0..8 {
            for j in 0..4 {
                let u = i as f64 * std::f64::consts::PI / 4.0;
                let v = -std::f64::consts::PI / 2.0 + j as f64 * std::f64::consts::PI / 3.0;
                let q = surf.value(u, v);
                let dist = norm(sub3(q, center));
                assert!(
                    approx_eq(dist, expected_r, 1e-7),
                    "Offset sphere radius mismatch: expected {} got {} at u={} v={}",
                    expected_r,
                    dist,
                    u,
                    v
                );
            }
        }
    }

    #[test]
    fn test_offset_sphere_radius_decrease() {
        let r = 10.0_f64;
        let d = -4.0_f64;
        let sph = GeomOffsetSphere::new([0.0, 0.0, 0.0], r);
        let surf = GeomOffsetSurface::new_with_sphere(sph, d);

        let expected_r = r + d; // 6.0
        let q = surf.value(1.0, 0.5);
        let dist = norm(q);
        assert!(approx_eq(dist, expected_r, 1e-7));
    }

    #[test]
    fn test_offset_sphere_off_center() {
        // Sphere centered at (1,2,3)
        let center = [1.0, 2.0, 3.0];
        let r = 5.0_f64;
        let d = 1.5_f64;
        let sph = GeomOffsetSphere::new(center, r);
        let surf = GeomOffsetSurface::new_with_sphere(sph, d);

        let q = surf.value(0.3, 0.7);
        let dist = norm(sub3(q, center));
        assert!(approx_eq(dist, r + d, 1e-7));
    }

    #[test]
    fn test_offset_sphere_d1_consistency() {
        let sph = GeomOffsetSphere::new([0.0, 0.0, 0.0], 5.0);
        let surf = GeomOffsetSurface::new_with_sphere(sph, 2.0);

        let u = 0.8_f64;
        let v = 0.4_f64;
        let (_, du, dv) = surf.d1(u, v);
        let eps = 1e-5_f64;

        let q_plus_u = surf.value(u + eps, v);
        let q_minus_u = surf.value(u - eps, v);
        let fd_du = [
            (q_plus_u[0] - q_minus_u[0]) / (2.0 * eps),
            (q_plus_u[1] - q_minus_u[1]) / (2.0 * eps),
            (q_plus_u[2] - q_minus_u[2]) / (2.0 * eps),
        ];
        assert!(
            vec3_approx_eq(du, fd_du, 1e-4),
            "Sphere offset du mismatch: {:?} vs {:?}",
            du,
            fd_du
        );

        let q_plus_v = surf.value(u, v + eps);
        let q_minus_v = surf.value(u, v - eps);
        let fd_dv = [
            (q_plus_v[0] - q_minus_v[0]) / (2.0 * eps),
            (q_plus_v[1] - q_minus_v[1]) / (2.0 * eps),
            (q_plus_v[2] - q_minus_v[2]) / (2.0 * eps),
        ];
        assert!(
            vec3_approx_eq(dv, fd_dv, 1e-4),
            "Sphere offset dv mismatch: {:?} vs {:?}",
            dv,
            fd_dv
        );
    }

    // --- offset_value and accessors ---

    #[test]
    fn test_offset_value_accessor() {
        let plane = GeomOffsetPlane::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
        let surf = GeomOffsetSurface::new_with_plane(plane, 3.14);
        assert!(approx_eq(surf.offset_value(), 3.14, 1e-15));
    }

    #[test]
    fn test_basis_surface_offset_returns_basis() {
        let cyl = GeomOffsetCylinder::along_z([0.0, 0.0, 0.0], 5.0);
        let surf = GeomOffsetSurface::new_with_cylinder(cyl, 1.0);
        // basis_surface_offset returns reference; verify it evaluates correctly
        let basis = surf.basis_surface_offset();
        let p = basis.value(0.0, 0.0);
        assert!(vec3_approx_eq(p, [5.0, 0.0, 0.0], 1e-10));
    }

    #[test]
    fn test_osculat_flag_default_false() {
        let plane = GeomOffsetPlane::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
        let mut surf = GeomOffsetSurface::new_with_plane(plane, 1.0);
        // default is false; setting it changes internal flag
        surf.set_osculat_flag(true);
        // Just verify it doesn't panic and functionality still works
        let q = surf.value(0.0, 0.0);
        assert!(vec3_approx_eq(q, [0.0, 0.0, 1.0], 1e-7));
    }

    // --- Normal from finite differences consistency ---

    #[test]
    fn test_offset_plane_normal_from_d1() {
        let plane = GeomOffsetPlane::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
        let surf = GeomOffsetSurface::new_with_plane(plane, 4.0);
        let n = surf.normal(1.0, 1.0);
        // Plane offset: normal should still be +Z
        assert!(vec3_approx_eq(n, [0.0, 0.0, 1.0], 1e-5));
    }

    #[test]
    fn test_offset_cylinder_normal_radial() {
        let r = 3.0_f64;
        let d = 1.0_f64;
        let cyl = GeomOffsetCylinder::along_z([0.0, 0.0, 0.0], r);
        let surf = GeomOffsetSurface::new_with_cylinder(cyl, d);
        // At u=0, normal should point in +X
        let n = surf.normal(0.0, 0.0);
        assert!(approx_eq(n[0], 1.0, 1e-5));
        assert!(approx_eq(n[1], 0.0, 1e-5));
        assert!(approx_eq(n[2], 0.0, 1e-5));
    }

    // --- Cross product and normalize helpers ---

    #[test]
    fn test_cross_product() {
        let a = [1.0, 0.0, 0.0];
        let b = [0.0, 1.0, 0.0];
        let c = cross(a, b);
        assert!(vec3_approx_eq(c, [0.0, 0.0, 1.0], 1e-15));
    }

    #[test]
    fn test_normalize() {
        let v = [3.0, 4.0, 0.0];
        let n = normalize(v);
        assert!(approx_eq(n[0], 0.6, 1e-10));
        assert!(approx_eq(n[1], 0.8, 1e-10));
        assert!(approx_eq(n[2], 0.0, 1e-10));
    }

    #[test]
    fn test_normalize_zero_vector() {
        let v = [0.0, 0.0, 0.0];
        let n = normalize(v);
        assert!(vec3_approx_eq(n, [0.0, 0.0, 0.0], 1e-15));
    }

    // --- Tilted plane tests ---

    #[test]
    fn test_tilted_plane_offset() {
        // Plane tilted 45 degrees; u_dir = (1,0,0), v_dir = (0, 1/sqrt2, 1/sqrt2)
        let s2 = 2.0_f64.sqrt() / 2.0;
        let plane = GeomOffsetPlane::new(
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, s2, s2],
        );
        // Normal = u x v = (1,0,0) x (0, s2, s2) = (0*s2 - 0*s2, 0*0 - 1*s2, 1*s2 - 0*0)
        //                = (0, -s2, s2) normalized = (0, -1/sqrt2, 1/sqrt2)
        let n = plane.normal(0.0, 0.0);
        assert!(approx_eq(n[0], 0.0, 1e-10));
        assert!(approx_eq(n[1], -s2, 1e-10));
        assert!(approx_eq(n[2], s2, 1e-10));

        let surf = GeomOffsetSurface::new_with_plane(plane, 2.0);
        let q = surf.value(0.0, 0.0);
        // Should be origin + 2*normal = (0, -2*s2, 2*s2)
        assert!(approx_eq(q[0], 0.0, 1e-7));
        assert!(approx_eq(q[1], -2.0 * s2, 1e-7));
        assert!(approx_eq(q[2], 2.0 * s2, 1e-7));
    }

    // --- compute_normal helper ---

    #[test]
    fn test_compute_normal_orthogonal() {
        let du = [1.0, 0.0, 0.0];
        let dv = [0.0, 1.0, 0.0];
        let n = compute_normal(du, dv);
        assert!(vec3_approx_eq(n, [0.0, 0.0, 1.0], 1e-10));
    }

    #[test]
    fn test_compute_normal_is_unit() {
        let du = [2.0, 1.0, 0.0];
        let dv = [0.0, 3.0, 1.0];
        let n = compute_normal(du, dv);
        let mag = norm(n);
        assert!(approx_eq(mag, 1.0, 1e-10));
    }
}
