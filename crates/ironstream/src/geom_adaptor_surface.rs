// FILE: geom_adaptor_surface.rs
// occt: GeomAdaptor_Surface

use crate::gp::{Pnt, Vec3, Ax1, Ax2, Ax3};
use crate::gp_dir::Dir;
use crate::gp_prim::Pln;
use crate::geom_bspline_surface::GeomBSplineSurface;
use crate::geom_abs::{SurfaceType, CurveType};
use crate::geom_adaptor_curve::GeomAdaptorCurveKind;
use crate::precision::{CONFUSION, ANGULAR};

use std::f64::consts::PI;

/// Wraps a geometric plane for surface adaptation.
// occt: Geom_Plane
#[derive(Clone, Debug)]
pub struct GeomPlaneData {
    pub origin: [f64; 3],
    pub normal: [f64; 3],
    pub x_axis: [f64; 3],
    pub y_axis: [f64; 3],
}

/// Wraps a geometric cylinder for surface adaptation.
// occt: Geom_CylindricalSurface
#[derive(Clone, Debug)]
pub struct GeomCylinderData {
    pub origin: [f64; 3],
    pub axis: [f64; 3],
    pub x_axis: [f64; 3],
    pub y_axis: [f64; 3],
    pub radius: f64,
}

/// Wraps a geometric cone for surface adaptation.
// occt: Geom_ConicalSurface
#[derive(Clone, Debug)]
pub struct GeomConeData {
    pub origin: [f64; 3],
    pub axis: [f64; 3],
    pub x_axis: [f64; 3],
    pub y_axis: [f64; 3],
    pub radius: f64,
    pub half_angle: f64,
}

/// Wraps a geometric sphere for surface adaptation.
// occt: Geom_SphericalSurface
#[derive(Clone, Debug)]
pub struct GeomSphereData {
    pub center: [f64; 3],
    pub axis: [f64; 3],
    pub x_axis: [f64; 3],
    pub y_axis: [f64; 3],
    pub radius: f64,
}

/// Wraps a geometric torus for surface adaptation.
// occt: Geom_ToroidalSurface
#[derive(Clone, Debug)]
pub struct GeomTorusData {
    pub center: [f64; 3],
    pub axis: [f64; 3],
    pub x_axis: [f64; 3],
    pub y_axis: [f64; 3],
    pub major_radius: f64,
    pub minor_radius: f64,
}

/// Discriminated union of all parametric surface kinds.
// occt: GeomAdaptor_Surface (kind field)
#[derive(Clone, Debug)]
pub enum GeomAdaptorSurfaceKind {
    Plane(GeomPlaneData),
    Cylinder(GeomCylinderData),
    Cone(GeomConeData),
    Sphere(GeomSphereData),
    Torus(GeomTorusData),
    BSpline(GeomBSplineSurface),
    Extrusion(GeomExtrusionData),
    Revolution(GeomRevolutionData),
}

/// Data for a surface of extrusion (linear sweep).
// occt: Geom_SurfaceOfLinearExtrusion
#[derive(Clone, Debug)]
pub struct GeomExtrusionData {
    pub direction: [f64; 3],
    pub basis_curve_origin: [f64; 3],
    pub basis_curve_radius: f64,
}

/// Data for a surface of revolution.
// occt: Geom_SurfaceOfRevolution
#[derive(Clone, Debug)]
pub struct GeomRevolutionData {
    pub axis_origin: [f64; 3],
    pub axis_dir: [f64; 3],
    pub basis_point: [f64; 3],
}

fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn norm(a: [f64; 3]) -> f64 {
    dot(a, a).sqrt()
}

fn add(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

fn sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn scale(a: [f64; 3], s: f64) -> [f64; 3] {
    [a[0] * s, a[1] * s, a[2] * s]
}

fn normalize(a: [f64; 3]) -> [f64; 3] {
    let n = norm(a);
    if n < CONFUSION {
        [0.0, 0.0, 1.0]
    } else {
        scale(a, 1.0 / n)
    }
}

/// Adapts a 3D parametric surface to a uniform interface, analogous to OCCT's GeomAdaptor_Surface.
// occt: GeomAdaptor_Surface
#[derive(Clone, Debug)]
pub struct GeomAdaptorSurface {
    pub kind: GeomAdaptorSurfaceKind,
    pub u1: f64,
    pub u2: f64,
    pub v1: f64,
    pub v2: f64,
}

impl GeomAdaptorSurface {
    /// Construct from a plane. U in [0, 2*PI), V in (-inf, inf) trimmed to [-1e6, 1e6].
    pub fn from_plane(origin: [f64; 3], normal: [f64; 3], x_axis: [f64; 3]) -> Self {
        let n = normalize(normal);
        let x = normalize(x_axis);
        let y = cross(n, x);
        let data = GeomPlaneData {
            origin,
            normal: n,
            x_axis: x,
            y_axis: y,
        };
        GeomAdaptorSurface {
            kind: GeomAdaptorSurfaceKind::Plane(data),
            u1: -1e6,
            u2: 1e6,
            v1: -1e6,
            v2: 1e6,
        }
    }

    /// Construct from a cylinder. U in [0, 2*PI), V in (-inf, inf) trimmed to [-1e6, 1e6].
    pub fn from_cylinder(
        origin: [f64; 3],
        axis: [f64; 3],
        x_axis: [f64; 3],
        radius: f64,
    ) -> Self {
        let a = normalize(axis);
        let x = normalize(x_axis);
        let y = cross(a, x);
        let data = GeomCylinderData {
            origin,
            axis: a,
            x_axis: x,
            y_axis: y,
            radius,
        };
        GeomAdaptorSurface {
            kind: GeomAdaptorSurfaceKind::Cylinder(data),
            u1: 0.0,
            u2: 2.0 * PI,
            v1: -1e6,
            v2: 1e6,
        }
    }

    /// Construct from a cone. U in [0, 2*PI), V in (-inf, inf) trimmed to [-1e6, 1e6].
    pub fn from_cone(
        origin: [f64; 3],
        axis: [f64; 3],
        x_axis: [f64; 3],
        radius: f64,
        half_angle: f64,
    ) -> Self {
        let a = normalize(axis);
        let x = normalize(x_axis);
        let y = cross(a, x);
        let data = GeomConeData {
            origin,
            axis: a,
            x_axis: x,
            y_axis: y,
            radius,
            half_angle,
        };
        GeomAdaptorSurface {
            kind: GeomAdaptorSurfaceKind::Cone(data),
            u1: 0.0,
            u2: 2.0 * PI,
            v1: -1e6,
            v2: 1e6,
        }
    }

    /// Construct from a sphere. U in [0, 2*PI), V in [-PI/2, PI/2].
    pub fn from_sphere(center: [f64; 3], axis: [f64; 3], x_axis: [f64; 3], radius: f64) -> Self {
        let a = normalize(axis);
        let x = normalize(x_axis);
        let y = cross(a, x);
        let data = GeomSphereData {
            center,
            axis: a,
            x_axis: x,
            y_axis: y,
            radius,
        };
        GeomAdaptorSurface {
            kind: GeomAdaptorSurfaceKind::Sphere(data),
            u1: 0.0,
            u2: 2.0 * PI,
            v1: -PI / 2.0,
            v2: PI / 2.0,
        }
    }

    /// Construct from a torus. U in [0, 2*PI), V in [0, 2*PI).
    pub fn from_torus(
        center: [f64; 3],
        axis: [f64; 3],
        x_axis: [f64; 3],
        major_radius: f64,
        minor_radius: f64,
    ) -> Self {
        let a = normalize(axis);
        let x = normalize(x_axis);
        let y = cross(a, x);
        let data = GeomTorusData {
            center,
            axis: a,
            x_axis: x,
            y_axis: y,
            major_radius,
            minor_radius,
        };
        GeomAdaptorSurface {
            kind: GeomAdaptorSurfaceKind::Torus(data),
            u1: 0.0,
            u2: 2.0 * PI,
            v1: 0.0,
            v2: 2.0 * PI,
        }
    }

    /// Construct from a BSpline surface.
    pub fn from_bspline(surface: GeomBSplineSurface) -> Self {
        let u1 = surface.u_knot(surface.first_u_knot_index());
        let u2 = surface.u_knot(surface.last_u_knot_index());
        let v1 = surface.v_knot(surface.first_v_knot_index());
        let v2 = surface.v_knot(surface.last_v_knot_index());
        GeomAdaptorSurface {
            kind: GeomAdaptorSurfaceKind::BSpline(surface),
            u1,
            u2,
            v1,
            v2,
        }
    }

    /// Construct a surface of extrusion.
    pub fn from_extrusion(
        direction: [f64; 3],
        basis_curve_origin: [f64; 3],
        basis_curve_radius: f64,
    ) -> Self {
        let d = normalize(direction);
        let data = GeomExtrusionData {
            direction: d,
            basis_curve_origin,
            basis_curve_radius,
        };
        GeomAdaptorSurface {
            kind: GeomAdaptorSurfaceKind::Extrusion(data),
            u1: 0.0,
            u2: 2.0 * PI,
            v1: -1e6,
            v2: 1e6,
        }
    }

    /// Construct a surface of revolution.
    pub fn from_revolution(
        axis_origin: [f64; 3],
        axis_dir: [f64; 3],
        basis_point: [f64; 3],
    ) -> Self {
        let d = normalize(axis_dir);
        let data = GeomRevolutionData {
            axis_origin,
            axis_dir: d,
            basis_point,
        };
        GeomAdaptorSurface {
            kind: GeomAdaptorSurfaceKind::Revolution(data),
            u1: 0.0,
            u2: 2.0 * PI,
            v1: 0.0,
            v2: 1.0,
        }
    }

    /// Returns the SurfaceType discriminant for this surface.
    pub fn surface_type(&self) -> SurfaceType {
        match &self.kind {
            GeomAdaptorSurfaceKind::Plane(_) => SurfaceType::Plane,
            GeomAdaptorSurfaceKind::Cylinder(_) => SurfaceType::Cylinder,
            GeomAdaptorSurfaceKind::Cone(_) => SurfaceType::Cone,
            GeomAdaptorSurfaceKind::Sphere(_) => SurfaceType::Sphere,
            GeomAdaptorSurfaceKind::Torus(_) => SurfaceType::Torus,
            GeomAdaptorSurfaceKind::BSpline(_) => SurfaceType::BSplineSurface,
            GeomAdaptorSurfaceKind::Extrusion(_) => SurfaceType::ExtrusionSurface,
            GeomAdaptorSurfaceKind::Revolution(_) => SurfaceType::RevolutionSurface,
        }
    }

    /// First parameter in U direction.
    pub fn u_first(&self) -> f64 {
        self.u1
    }

    /// Last parameter in U direction.
    pub fn u_last(&self) -> f64 {
        self.u2
    }

    /// First parameter in V direction.
    pub fn v_first(&self) -> f64 {
        self.v1
    }

    /// Last parameter in V direction.
    pub fn v_last(&self) -> f64 {
        self.v2
    }

    /// Evaluate point on surface at parameters (u, v).
    pub fn value(&self, u: f64, v: f64) -> [f64; 3] {
        match &self.kind {
            GeomAdaptorSurfaceKind::Plane(p) => {
                let x = scale(p.x_axis, u);
                let y = scale(p.y_axis, v);
                add(p.origin, add(x, y))
            }
            GeomAdaptorSurfaceKind::Cylinder(c) => {
                let cu = u.cos();
                let su = u.sin();
                let radial = add(scale(c.x_axis, cu), scale(c.y_axis, su));
                let radial = scale(radial, c.radius);
                let axial = scale(c.axis, v);
                add(c.origin, add(radial, axial))
            }
            GeomAdaptorSurfaceKind::Cone(c) => {
                let r = c.radius + v * c.half_angle.tan();
                let cu = u.cos();
                let su = u.sin();
                let radial = add(scale(c.x_axis, cu), scale(c.y_axis, su));
                let radial = scale(radial, r);
                let axial = scale(c.axis, v);
                add(c.origin, add(radial, axial))
            }
            GeomAdaptorSurfaceKind::Sphere(s) => {
                let cv = v.cos();
                let sv = v.sin();
                let cu = u.cos();
                let su = u.sin();
                let p = [
                    s.center[0]
                        + s.radius * (cv * (cu * s.x_axis[0] + su * s.y_axis[0])
                            + sv * s.axis[0]),
                    s.center[1]
                        + s.radius * (cv * (cu * s.x_axis[1] + su * s.y_axis[1])
                            + sv * s.axis[1]),
                    s.center[2]
                        + s.radius * (cv * (cu * s.x_axis[2] + su * s.y_axis[2])
                            + sv * s.axis[2]),
                ];
                p
            }
            GeomAdaptorSurfaceKind::Torus(t) => {
                let cu = u.cos();
                let su = u.sin();
                let cv = v.cos();
                let sv = v.sin();
                let center_circle = add(
                    t.center,
                    add(scale(t.x_axis, t.major_radius * cu), scale(t.y_axis, t.major_radius * su)),
                );
                let radial_dir = add(scale(t.x_axis, cu), scale(t.y_axis, su));
                add(
                    center_circle,
                    add(scale(radial_dir, t.minor_radius * cv), scale(t.axis, t.minor_radius * sv)),
                )
            }
            GeomAdaptorSurfaceKind::BSpline(b) => {
                let p = b.value(u, v);
                [p.x, p.y, p.z]
            }
            GeomAdaptorSurfaceKind::Extrusion(e) => {
                let cu = u.cos();
                let su = u.sin();
                let r = e.basis_curve_radius;
                let base = [
                    e.basis_curve_origin[0] + r * cu,
                    e.basis_curve_origin[1] + r * su,
                    e.basis_curve_origin[2],
                ];
                add(base, scale(e.direction, v))
            }
            GeomAdaptorSurfaceKind::Revolution(r) => {
                let t_vec = sub(r.basis_point, r.axis_origin);
                let ax = r.axis_dir;
                let proj_len = dot(t_vec, ax);
                let proj = scale(ax, proj_len);
                let radial = sub(t_vec, proj);
                let radial_norm = norm(radial);
                let radial_dir = if radial_norm < CONFUSION {
                    perp_to(ax)
                } else {
                    scale(radial, 1.0 / radial_norm)
                };
                let tangent = cross(ax, radial_dir);
                let cu = u.cos();
                let su = u.sin();
                let swept_radial = add(scale(radial_dir, cu * radial_norm), scale(tangent, su * radial_norm));
                add(add(r.axis_origin, proj), swept_radial)
            }
        }
    }

    /// Evaluate point and first partial derivatives at (u, v).
    /// Returns (point, d/du, d/dv).
    pub fn d1(&self, u: f64, v: f64) -> ([f64; 3], [f64; 3], [f64; 3]) {
        match &self.kind {
            GeomAdaptorSurfaceKind::Plane(p) => {
                let pt = add(p.origin, add(scale(p.x_axis, u), scale(p.y_axis, v)));
                (pt, p.x_axis, p.y_axis)
            }
            GeomAdaptorSurfaceKind::Cylinder(c) => {
                let cu = u.cos();
                let su = u.sin();
                let radial = add(scale(c.x_axis, cu), scale(c.y_axis, su));
                let radial_pt = scale(radial, c.radius);
                let axial = scale(c.axis, v);
                let pt = add(c.origin, add(radial_pt, axial));
                let du = scale(add(scale(c.x_axis, -su), scale(c.y_axis, cu)), c.radius);
                let dv = c.axis;
                (pt, du, dv)
            }
            GeomAdaptorSurfaceKind::Cone(c) => {
                let r = c.radius + v * c.half_angle.tan();
                let tan_a = c.half_angle.tan();
                let cu = u.cos();
                let su = u.sin();
                let radial_dir = add(scale(c.x_axis, cu), scale(c.y_axis, su));
                let radial = scale(radial_dir, r);
                let axial = scale(c.axis, v);
                let pt = add(c.origin, add(radial, axial));
                let du = scale(add(scale(c.x_axis, -su), scale(c.y_axis, cu)), r);
                let dv = add(scale(radial_dir, tan_a), c.axis);
                (pt, du, dv)
            }
            GeomAdaptorSurfaceKind::Sphere(s) => {
                let cv = v.cos();
                let sv = v.sin();
                let cu = u.cos();
                let su = u.sin();
                let pt = self.value(u, v);
                let du = scale(
                    add(scale(s.x_axis, -su * cv), scale(s.y_axis, cu * cv)),
                    s.radius,
                );
                let dv = scale(
                    add(
                        add(scale(s.x_axis, -cu * sv), scale(s.y_axis, -su * sv)),
                        scale(s.axis, cv),
                    ),
                    s.radius,
                );
                (pt, du, dv)
            }
            GeomAdaptorSurfaceKind::Torus(t) => {
                let cu = u.cos();
                let su = u.sin();
                let cv = v.cos();
                let sv = v.sin();
                let pt = self.value(u, v);
                let radial_dir = add(scale(t.x_axis, cu), scale(t.y_axis, su));
                let d_radial_du = add(scale(t.x_axis, -su), scale(t.y_axis, cu));
                let du = add(
                    scale(d_radial_du, t.major_radius),
                    scale(d_radial_du, t.minor_radius * cv),
                );
                let dv = add(scale(radial_dir, -t.minor_radius * sv), scale(t.axis, t.minor_radius * cv));
                (pt, du, dv)
            }
            GeomAdaptorSurfaceKind::BSpline(b) => {
                let (pt, du, dv) = b.d1(u, v);
                ([pt.x, pt.y, pt.z], [du.x, du.y, du.z], [dv.x, dv.y, dv.z])
            }
            GeomAdaptorSurfaceKind::Extrusion(e) => {
                let cu = u.cos();
                let su = u.sin();
                let r = e.basis_curve_radius;
                let pt = self.value(u, v);
                let du = [r * -su, r * cu, 0.0];
                let dv = e.direction;
                (pt, du, dv)
            }
            GeomAdaptorSurfaceKind::Revolution(r) => {
                let t_vec = sub(r.basis_point, r.axis_origin);
                let ax = r.axis_dir;
                let proj_len = dot(t_vec, ax);
                let proj = scale(ax, proj_len);
                let radial = sub(t_vec, proj);
                let radial_norm = norm(radial);
                let radial_dir = if radial_norm < CONFUSION {
                    perp_to(ax)
                } else {
                    scale(radial, 1.0 / radial_norm)
                };
                let tangent = cross(ax, radial_dir);
                let cu = u.cos();
                let su = u.sin();
                let pt = self.value(u, v);
                let du = add(
                    scale(radial_dir, -su * radial_norm),
                    scale(tangent, cu * radial_norm),
                );
                let dv = [0.0, 0.0, 0.0];
                (pt, du, dv)
            }
        }
    }

    /// Returns true if the surface is closed in the U direction.
    pub fn is_u_closed(&self) -> bool {
        match &self.kind {
            GeomAdaptorSurfaceKind::Plane(_) => false,
            GeomAdaptorSurfaceKind::Cylinder(_) => true,
            GeomAdaptorSurfaceKind::Cone(_) => true,
            GeomAdaptorSurfaceKind::Sphere(_) => true,
            GeomAdaptorSurfaceKind::Torus(_) => true,
            // GeomBSplineSurface has no is_u_closed method; approximate as false
            GeomAdaptorSurfaceKind::BSpline(_) => false,
            GeomAdaptorSurfaceKind::Extrusion(_) => true,
            GeomAdaptorSurfaceKind::Revolution(_) => true,
        }
    }

    /// Returns true if the surface is closed in the V direction.
    pub fn is_v_closed(&self) -> bool {
        match &self.kind {
            GeomAdaptorSurfaceKind::Plane(_) => false,
            GeomAdaptorSurfaceKind::Cylinder(_) => false,
            GeomAdaptorSurfaceKind::Cone(_) => false,
            GeomAdaptorSurfaceKind::Sphere(_) => false,
            GeomAdaptorSurfaceKind::Torus(_) => true,
            // GeomBSplineSurface has no is_v_closed method; approximate as false
            GeomAdaptorSurfaceKind::BSpline(_) => false,
            GeomAdaptorSurfaceKind::Extrusion(_) => false,
            GeomAdaptorSurfaceKind::Revolution(_) => false,
        }
    }

    /// Returns true if the surface is periodic in U.
    pub fn is_u_periodic(&self) -> bool {
        match &self.kind {
            GeomAdaptorSurfaceKind::Plane(_) => false,
            GeomAdaptorSurfaceKind::Cylinder(_) => true,
            GeomAdaptorSurfaceKind::Cone(_) => true,
            GeomAdaptorSurfaceKind::Sphere(_) => true,
            GeomAdaptorSurfaceKind::Torus(_) => true,
            GeomAdaptorSurfaceKind::BSpline(b) => b.is_u_periodic(),
            GeomAdaptorSurfaceKind::Extrusion(_) => true,
            GeomAdaptorSurfaceKind::Revolution(_) => true,
        }
    }

    /// Returns true if the surface is periodic in V.
    pub fn is_v_periodic(&self) -> bool {
        match &self.kind {
            GeomAdaptorSurfaceKind::Plane(_) => false,
            GeomAdaptorSurfaceKind::Cylinder(_) => false,
            GeomAdaptorSurfaceKind::Cone(_) => false,
            GeomAdaptorSurfaceKind::Sphere(_) => false,
            GeomAdaptorSurfaceKind::Torus(_) => true,
            GeomAdaptorSurfaceKind::BSpline(b) => b.is_v_periodic(),
            GeomAdaptorSurfaceKind::Extrusion(_) => false,
            GeomAdaptorSurfaceKind::Revolution(_) => false,
        }
    }

    /// Returns plane data if this is a planar surface.
    pub fn plane(&self) -> Option<&GeomPlaneData> {
        match &self.kind {
            GeomAdaptorSurfaceKind::Plane(p) => Some(p),
            _ => None,
        }
    }

    /// Returns cylinder data if this is a cylindrical surface.
    pub fn cylinder(&self) -> Option<&GeomCylinderData> {
        match &self.kind {
            GeomAdaptorSurfaceKind::Cylinder(c) => Some(c),
            _ => None,
        }
    }

    /// Returns cone data if this is a conical surface.
    pub fn cone(&self) -> Option<&GeomConeData> {
        match &self.kind {
            GeomAdaptorSurfaceKind::Cone(c) => Some(c),
            _ => None,
        }
    }

    /// Returns sphere data if this is a spherical surface.
    pub fn sphere(&self) -> Option<&GeomSphereData> {
        match &self.kind {
            GeomAdaptorSurfaceKind::Sphere(s) => Some(s),
            _ => None,
        }
    }

    /// Returns torus data if this is a toroidal surface.
    pub fn torus(&self) -> Option<&GeomTorusData> {
        match &self.kind {
            GeomAdaptorSurfaceKind::Torus(t) => Some(t),
            _ => None,
        }
    }

    /// Returns BSpline surface data if this is a BSpline surface.
    pub fn bspline(&self) -> Option<&GeomBSplineSurface> {
        match &self.kind {
            GeomAdaptorSurfaceKind::BSpline(b) => Some(b),
            _ => None,
        }
    }

    /// Returns extrusion data if this is a surface of extrusion.
    pub fn extrusion(&self) -> Option<&GeomExtrusionData> {
        match &self.kind {
            GeomAdaptorSurfaceKind::Extrusion(e) => Some(e),
            _ => None,
        }
    }

    /// Returns revolution data if this is a surface of revolution.
    pub fn revolution(&self) -> Option<&GeomRevolutionData> {
        match &self.kind {
            GeomAdaptorSurfaceKind::Revolution(r) => Some(r),
            _ => None,
        }
    }

    /// Return the iso-curve at fixed u parameter as a GeomAdaptorCurveKind.
    pub fn u_iso(&self, u: f64) -> GeomAdaptorCurveKind {
        match &self.kind {
            GeomAdaptorSurfaceKind::Plane(p) => {
                let cu = u;
                let point_on_u = add(p.origin, scale(p.x_axis, cu));
                GeomAdaptorCurveKind::RawLine {
                    origin: point_on_u,
                    direction: p.y_axis,
                }
            }
            GeomAdaptorSurfaceKind::Cylinder(c) => {
                let cu = u.cos();
                let su = u.sin();
                let radial = scale(add(scale(c.x_axis, cu), scale(c.y_axis, su)), c.radius);
                let origin = add(c.origin, radial);
                GeomAdaptorCurveKind::RawLine {
                    origin,
                    direction: c.axis,
                }
            }
            GeomAdaptorSurfaceKind::Cone(c) => {
                let cu = u.cos();
                let su = u.sin();
                let radial_dir = add(scale(c.x_axis, cu), scale(c.y_axis, su));
                let tan_a = c.half_angle.tan();
                let direction = add(scale(radial_dir, tan_a), c.axis);
                let direction = normalize(direction);
                let origin = add(c.origin, scale(radial_dir, c.radius));
                GeomAdaptorCurveKind::RawLine { origin, direction }
            }
            GeomAdaptorSurfaceKind::Sphere(s) => {
                let cu = u.cos();
                let su = u.sin();
                let meridian_x = add(scale(s.x_axis, cu), scale(s.y_axis, su));
                GeomAdaptorCurveKind::RawCircle {
                    center: s.center,
                    x_axis: meridian_x,
                    y_axis: s.axis,
                    radius: s.radius,
                }
            }
            GeomAdaptorSurfaceKind::Torus(t) => {
                let cu = u.cos();
                let su = u.sin();
                let radial_dir = add(scale(t.x_axis, cu), scale(t.y_axis, su));
                let circle_center = add(t.center, scale(radial_dir, t.major_radius));
                GeomAdaptorCurveKind::RawCircle {
                    center: circle_center,
                    x_axis: radial_dir,
                    y_axis: t.axis,
                    radius: t.minor_radius,
                }
            }
            GeomAdaptorSurfaceKind::BSpline(b) => {
                let curve = b.u_iso(u);
                GeomAdaptorCurveKind::RawBSpline(curve)
            }
            GeomAdaptorSurfaceKind::Extrusion(e) => {
                let cu = u.cos();
                let su = u.sin();
                let r = e.basis_curve_radius;
                let origin = [
                    e.basis_curve_origin[0] + r * cu,
                    e.basis_curve_origin[1] + r * su,
                    e.basis_curve_origin[2],
                ];
                GeomAdaptorCurveKind::RawLine {
                    origin,
                    direction: e.direction,
                }
            }
            GeomAdaptorSurfaceKind::Revolution(r) => {
                let t_vec = sub(r.basis_point, r.axis_origin);
                let ax = r.axis_dir;
                let proj_len = dot(t_vec, ax);
                let proj = scale(ax, proj_len);
                let radial = sub(t_vec, proj);
                let radial_norm = norm(radial);
                let radial_dir = if radial_norm < CONFUSION {
                    perp_to(ax)
                } else {
                    scale(radial, 1.0 / radial_norm)
                };
                let tangent = cross(ax, radial_dir);
                let cu = u.cos();
                let su = u.sin();
                let swept_dir = add(scale(radial_dir, cu), scale(tangent, su));
                let origin = add(add(r.axis_origin, proj), scale(swept_dir, radial_norm));
                GeomAdaptorCurveKind::RawLine {
                    origin,
                    direction: ax,
                }
            }
        }
    }

    /// Return the iso-curve at fixed v parameter as a GeomAdaptorCurveKind.
    pub fn v_iso(&self, v: f64) -> GeomAdaptorCurveKind {
        match &self.kind {
            GeomAdaptorSurfaceKind::Plane(p) => {
                let point_on_v = add(p.origin, scale(p.y_axis, v));
                GeomAdaptorCurveKind::RawLine {
                    origin: point_on_v,
                    direction: p.x_axis,
                }
            }
            GeomAdaptorSurfaceKind::Cylinder(c) => {
                let origin = add(c.origin, scale(c.axis, v));
                GeomAdaptorCurveKind::RawCircle {
                    center: origin,
                    x_axis: c.x_axis,
                    y_axis: c.y_axis,
                    radius: c.radius,
                }
            }
            GeomAdaptorSurfaceKind::Cone(c) => {
                let r = c.radius + v * c.half_angle.tan();
                let center = add(c.origin, scale(c.axis, v));
                GeomAdaptorCurveKind::RawCircle {
                    center,
                    x_axis: c.x_axis,
                    y_axis: c.y_axis,
                    radius: r,
                }
            }
            GeomAdaptorSurfaceKind::Sphere(s) => {
                let cv = v.cos();
                let sv = v.sin();
                let circle_center = add(s.center, scale(s.axis, s.radius * sv));
                let circle_radius = s.radius * cv;
                GeomAdaptorCurveKind::RawCircle {
                    center: circle_center,
                    x_axis: s.x_axis,
                    y_axis: s.y_axis,
                    radius: circle_radius,
                }
            }
            GeomAdaptorSurfaceKind::Torus(t) => {
                let cv = v.cos();
                let sv = v.sin();
                let major_r = t.major_radius + t.minor_radius * cv;
                let center = add(t.center, scale(t.axis, t.minor_radius * sv));
                GeomAdaptorCurveKind::RawCircle {
                    center,
                    x_axis: t.x_axis,
                    y_axis: t.y_axis,
                    radius: major_r,
                }
            }
            GeomAdaptorSurfaceKind::BSpline(b) => {
                let curve = b.v_iso(v);
                GeomAdaptorCurveKind::RawBSpline(curve)
            }
            GeomAdaptorSurfaceKind::Extrusion(e) => {
                let origin = add(e.basis_curve_origin, scale(e.direction, v));
                GeomAdaptorCurveKind::RawCircle {
                    center: origin,
                    x_axis: [1.0, 0.0, 0.0],
                    y_axis: [0.0, 1.0, 0.0],
                    radius: e.basis_curve_radius,
                }
            }
            GeomAdaptorSurfaceKind::Revolution(r) => {
                let t_vec = sub(r.basis_point, r.axis_origin);
                let ax = r.axis_dir;
                let proj_len = dot(t_vec, ax);
                let proj = scale(ax, proj_len);
                let radial = sub(t_vec, proj);
                let radial_norm = norm(radial);
                let radial_dir = if radial_norm < CONFUSION {
                    perp_to(ax)
                } else {
                    scale(radial, 1.0 / radial_norm)
                };
                let tangent = cross(ax, radial_dir);
                let circle_center = add(r.axis_origin, proj);
                GeomAdaptorCurveKind::RawCircle {
                    center: circle_center,
                    x_axis: radial_dir,
                    y_axis: tangent,
                    radius: radial_norm,
                }
            }
        }
    }
}

/// Compute a vector perpendicular to the given direction.
fn perp_to(d: [f64; 3]) -> [f64; 3] {
    let ax = [1.0f64, 0.0, 0.0];
    let ay = [0.0f64, 1.0, 0.0];
    let c1 = cross(d, ax);
    let c2 = cross(d, ay);
    if norm(c1) >= norm(c2) {
        normalize(c1)
    } else {
        normalize(c2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    const EPS: f64 = 1e-7;

    fn approx_eq(a: f64, b: f64, eps: f64) -> bool {
        (a - b).abs() < eps
    }

    fn vec_approx_eq(a: [f64; 3], b: [f64; 3], eps: f64) -> bool {
        approx_eq(a[0], b[0], eps) && approx_eq(a[1], b[1], eps) && approx_eq(a[2], b[2], eps)
    }

    #[test]
    fn test_sphere_surface_type() {
        let surf = GeomAdaptorSurface::from_sphere(
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            [1.0, 0.0, 0.0],
            5.0,
        );
        assert!(matches!(surf.surface_type(), SurfaceType::Sphere));
    }

    #[test]
    fn test_sphere_parameter_range() {
        let surf = GeomAdaptorSurface::from_sphere(
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            [1.0, 0.0, 0.0],
            3.0,
        );
        assert!(approx_eq(surf.u_first(), 0.0, EPS));
        assert!(approx_eq(surf.u_last(), 2.0 * PI, EPS));
        assert!(approx_eq(surf.v_first(), -PI / 2.0, EPS));
        assert!(approx_eq(surf.v_last(), PI / 2.0, EPS));
    }

    #[test]
    fn test_sphere_value_at_zero() {
        let r = 5.0;
        let surf = GeomAdaptorSurface::from_sphere(
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            [1.0, 0.0, 0.0],
            r,
        );
        // At (u=0, v=0): point should be (r, 0, 0)
        let pt = surf.value(0.0, 0.0);
        assert!(vec_approx_eq(pt, [r, 0.0, 0.0], EPS), "got {:?}", pt);
    }

    #[test]
    fn test_sphere_value_at_north_pole() {
        let r = 5.0;
        let surf = GeomAdaptorSurface::from_sphere(
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            [1.0, 0.0, 0.0],
            r,
        );
        // At (u=0, v=PI/2): north pole (0, 0, r)
        let pt = surf.value(0.0, PI / 2.0);
        assert!(vec_approx_eq(pt, [0.0, 0.0, r], EPS), "got {:?}", pt);
    }

    #[test]
    fn test_sphere_value_various_angles() {
        let r = 1.0;
        let surf = GeomAdaptorSurface::from_sphere(
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            [1.0, 0.0, 0.0],
            r,
        );
        // u=PI/2, v=0 => (0, 1, 0)
        let pt = surf.value(PI / 2.0, 0.0);
        assert!(vec_approx_eq(pt, [0.0, 1.0, 0.0], EPS), "got {:?}", pt);
        // u=PI, v=0 => (-1, 0, 0)
        let pt = surf.value(PI, 0.0);
        assert!(vec_approx_eq(pt, [-1.0, 0.0, 0.0], EPS), "got {:?}", pt);
    }

    #[test]
    fn test_sphere_on_unit_sphere() {
        let r = 1.0;
        let surf = GeomAdaptorSurface::from_sphere(
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            [1.0, 0.0, 0.0],
            r,
        );
        // All points should lie on the unit sphere
        for i in 0..8 {
            for j in 0..4 {
                let u = i as f64 * PI / 4.0;
                let v = -PI / 2.0 + j as f64 * PI / 3.0;
                let pt = surf.value(u, v);
                let dist = (pt[0] * pt[0] + pt[1] * pt[1] + pt[2] * pt[2]).sqrt();
                assert!(approx_eq(dist, r, EPS), "u={} v={} dist={}", u, v, dist);
            }
        }
    }

    #[test]
    fn test_sphere_d1_at_zero() {
        let r = 5.0;
        let surf = GeomAdaptorSurface::from_sphere(
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            [1.0, 0.0, 0.0],
            r,
        );
        let (pt, du, dv) = surf.d1(0.0, 0.0);
        // pt = (r, 0, 0)
        assert!(vec_approx_eq(pt, [r, 0.0, 0.0], EPS));
        // du at (0,0): d/du [r*cos(v)*cos(u), r*cos(v)*sin(u), r*sin(v)] = [-r*cos(v)*sin(u), r*cos(v)*cos(u), 0]
        // at u=0,v=0: [0, r, 0]
        assert!(vec_approx_eq(du, [0.0, r, 0.0], EPS), "du={:?}", du);
        // dv at (0,0): [r*-sin(v)*cos(u), r*-sin(v)*sin(u), r*cos(v)]
        // at u=0,v=0: [0, 0, r]
        assert!(vec_approx_eq(dv, [0.0, 0.0, r], EPS), "dv={:?}", dv);
    }

    #[test]
    fn test_sphere_periodic_and_closed() {
        let surf = GeomAdaptorSurface::from_sphere(
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            [1.0, 0.0, 0.0],
            2.0,
        );
        assert!(surf.is_u_closed());
        assert!(!surf.is_v_closed());
        assert!(surf.is_u_periodic());
        assert!(!surf.is_v_periodic());
    }

    #[test]
    fn test_sphere_accessor() {
        let surf = GeomAdaptorSurface::from_sphere(
            [1.0, 2.0, 3.0],
            [0.0, 0.0, 1.0],
            [1.0, 0.0, 0.0],
            7.0,
        );
        let s = surf.sphere().expect("should be Sphere");
        assert!(approx_eq(s.radius, 7.0, EPS));
        assert!(vec_approx_eq(s.center, [1.0, 2.0, 3.0], EPS));
        assert!(surf.plane().is_none());
        assert!(surf.cylinder().is_none());
    }

    #[test]
    fn test_plane_surface_type_and_value() {
        let surf = GeomAdaptorSurface::from_plane(
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            [1.0, 0.0, 0.0],
        );
        assert!(matches!(surf.surface_type(), SurfaceType::Plane));
        // value(2, 3) = (2, 3, 0)
        let pt = surf.value(2.0, 3.0);
        assert!(vec_approx_eq(pt, [2.0, 3.0, 0.0], EPS), "got {:?}", pt);
    }

    #[test]
    fn test_plane_d1() {
        let surf = GeomAdaptorSurface::from_plane(
            [1.0, 1.0, 1.0],
            [0.0, 0.0, 1.0],
            [1.0, 0.0, 0.0],
        );
        let (pt, du, dv) = surf.d1(2.0, 3.0);
        assert!(vec_approx_eq(pt, [3.0, 4.0, 1.0], EPS));
        assert!(vec_approx_eq(du, [1.0, 0.0, 0.0], EPS));
        assert!(vec_approx_eq(dv, [0.0, 1.0, 0.0], EPS));
    }

    #[test]
    fn test_cylinder_surface_type() {
        let surf = GeomAdaptorSurface::from_cylinder(
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            [1.0, 0.0, 0.0],
            3.0,
        );
        assert!(matches!(surf.surface_type(), SurfaceType::Cylinder));
        // at u=0, v=0 => (3, 0, 0)
        let pt = surf.value(0.0, 0.0);
        assert!(vec_approx_eq(pt, [3.0, 0.0, 0.0], EPS));
        // at u=PI/2, v=1 => (0, 3, 1)
        let pt = surf.value(PI / 2.0, 1.0);
        assert!(vec_approx_eq(pt, [0.0, 3.0, 1.0], EPS), "got {:?}", pt);
    }

    #[test]
    fn test_cylinder_d1() {
        let r = 2.0;
        let surf = GeomAdaptorSurface::from_cylinder(
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            [1.0, 0.0, 0.0],
            r,
        );
        let (pt, du, dv) = surf.d1(0.0, 0.0);
        assert!(vec_approx_eq(pt, [r, 0.0, 0.0], EPS));
        // du = r * (-sin(0)*x + cos(0)*y) = (0, r, 0)
        assert!(vec_approx_eq(du, [0.0, r, 0.0], EPS), "du={:?}", du);
        // dv = axis = (0, 0, 1)
        assert!(vec_approx_eq(dv, [0.0, 0.0, 1.0], EPS));
    }

    #[test]
    fn test_cone_value() {
        // cone with radius=1 at origin, half_angle=PI/4
        let surf = GeomAdaptorSurface::from_cone(
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            [1.0, 0.0, 0.0],
            1.0,
            PI / 4.0,
        );
        // at u=0, v=1: r = 1 + 1*tan(PI/4) = 2, pt = (2, 0, 1)
        let pt = surf.value(0.0, 1.0);
        assert!(vec_approx_eq(pt, [2.0, 0.0, 1.0], EPS), "got {:?}", pt);
    }

    #[test]
    fn test_torus_surface_type() {
        let surf = GeomAdaptorSurface::from_torus(
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            [1.0, 0.0, 0.0],
            3.0,
            1.0,
        );
        assert!(matches!(surf.surface_type(), SurfaceType::Torus));
    }

    #[test]
    fn test_torus_value_at_zero() {
        // major=3, minor=1; at (0,0) point is at (major+minor, 0, 0) = (4,0,0)
        let surf = GeomAdaptorSurface::from_torus(
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            [1.0, 0.0, 0.0],
            3.0,
            1.0,
        );
        let pt = surf.value(0.0, 0.0);
        assert!(vec_approx_eq(pt, [4.0, 0.0, 0.0], EPS), "got {:?}", pt);
    }

    #[test]
    fn test_torus_value_at_pi() {
        // at u=0, v=PI: (major-minor, 0, 0) = (2, 0, 0)
        let surf = GeomAdaptorSurface::from_torus(
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            [1.0, 0.0, 0.0],
            3.0,
            1.0,
        );
        let pt = surf.value(0.0, PI);
        assert!(vec_approx_eq(pt, [2.0, 0.0, 0.0], EPS), "got {:?}", pt);
    }

    #[test]
    fn test_torus_v_closed_and_periodic() {
        let surf = GeomAdaptorSurface::from_torus(
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            [1.0, 0.0, 0.0],
            3.0,
            1.0,
        );
        assert!(surf.is_u_closed());
        assert!(surf.is_v_closed());
        assert!(surf.is_u_periodic());
        assert!(surf.is_v_periodic());
    }

    #[test]
    fn test_plane_not_closed() {
        let surf = GeomAdaptorSurface::from_plane(
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            [1.0, 0.0, 0.0],
        );
        assert!(!surf.is_u_closed());
        assert!(!surf.is_v_closed());
        assert!(!surf.is_u_periodic());
        assert!(!surf.is_v_periodic());
    }

    #[test]
    fn test_sphere_u_iso() {
        let r = 5.0;
        let surf = GeomAdaptorSurface::from_sphere(
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            [1.0, 0.0, 0.0],
            r,
        );
        let iso = surf.u_iso(0.0);
        // u_iso at u=0 should be a circle in the x-z plane with radius r
        match iso {
            GeomAdaptorCurveKind::RawCircle { center, radius, .. } => {
                assert!(vec_approx_eq(center, [0.0, 0.0, 0.0], EPS));
                assert!(approx_eq(radius, r, EPS));
            }
            other => panic!("Expected RawCircle, got {:?}", other),
        }
    }

    #[test]
    fn test_sphere_v_iso() {
        let r = 5.0;
        let surf = GeomAdaptorSurface::from_sphere(
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            [1.0, 0.0, 0.0],
            r,
        );
        // v_iso at v=0: parallel circle at equator with radius r
        let iso = surf.v_iso(0.0);
        match iso {
            GeomAdaptorCurveKind::RawCircle { center, radius, .. } => {
                assert!(vec_approx_eq(center, [0.0, 0.0, 0.0], EPS));
                assert!(approx_eq(radius, r, EPS));
            }
            other => panic!("Expected RawCircle, got {:?}", other),
        }
    }

    #[test]
    fn test_cylinder_v_iso() {
        let r = 2.0;
        let surf = GeomAdaptorSurface::from_cylinder(
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            [1.0, 0.0, 0.0],
            r,
        );
        let iso = surf.v_iso(3.0);
        match iso {
            GeomAdaptorCurveKind::RawCircle { center, radius, .. } => {
                assert!(vec_approx_eq(center, [0.0, 0.0, 3.0], EPS));
                assert!(approx_eq(radius, r, EPS));
            }
            other => panic!("Expected RawCircle, got {:?}", other),
        }
    }

    #[test]
    fn test_plane_u_iso() {
        let surf = GeomAdaptorSurface::from_plane(
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            [1.0, 0.0, 0.0],
        );
        let iso = surf.u_iso(2.0);
        match iso {
            GeomAdaptorCurveKind::RawLine { origin, direction } => {
                assert!(vec_approx_eq(origin, [2.0, 0.0, 0.0], EPS));
                assert!(vec_approx_eq(direction, [0.0, 1.0, 0.0], EPS));
            }
            other => panic!("Expected RawLine, got {:?}", other),
        }
    }

    #[test]
    fn test_cone_surface_type() {
        let surf = GeomAdaptorSurface::from_cone(
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            [1.0, 0.0, 0.0],
            1.0,
            PI / 6.0,
        );
        assert!(matches!(surf.surface_type(), SurfaceType::Cone));
    }

    #[test]
    fn test_extrusion_surface_type() {
        let surf = GeomAdaptorSurface::from_extrusion(
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 0.0],
            1.0,
        );
        assert!(matches!(surf.surface_type(), SurfaceType::ExtrusionSurface));
        let pt = surf.value(0.0, 0.0);
        assert!(vec_approx_eq(pt, [1.0, 0.0, 0.0], EPS), "got {:?}", pt);
        let pt2 = surf.value(0.0, 2.0);
        assert!(vec_approx_eq(pt2, [1.0, 0.0, 2.0], EPS), "got {:?}", pt2);
    }

    #[test]
    fn test_revolution_surface_type() {
        let surf = GeomAdaptorSurface::from_revolution(
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            [1.0, 0.0, 0.0],
        );
        assert!(matches!(surf.surface_type(), SurfaceType::RevolutionSurface));
    }

    #[test]
    fn test_sphere_centered_nonzero() {
        let cx = 1.0;
        let cy = 2.0;
        let cz = 3.0;
        let r = 4.0;
        let surf = GeomAdaptorSurface::from_sphere(
            [cx, cy, cz],
            [0.0, 0.0, 1.0],
            [1.0, 0.0, 0.0],
            r,
        );
        let pt = surf.value(0.0, 0.0);
        assert!(vec_approx_eq(pt, [cx + r, cy, cz], EPS), "got {:?}", pt);
        let pt2 = surf.value(0.0, PI / 2.0);
        assert!(vec_approx_eq(pt2, [cx, cy, cz + r], EPS), "got {:?}", pt2);
    }

    #[test]
    fn test_sphere_value_matches_parametric() {
        let r = 3.0;
        let surf = GeomAdaptorSurface::from_sphere(
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            [1.0, 0.0, 0.0],
            r,
        );
        for i in 0..12 {
            for j in 0..6 {
                let u = i as f64 * PI / 6.0;
                let v = -PI / 2.0 + j as f64 * PI / 5.0;
                let pt = surf.value(u, v);
                let expected = [
                    r * v.cos() * u.cos(),
                    r * v.cos() * u.sin(),
                    r * v.sin(),
                ];
                assert!(vec_approx_eq(pt, expected, EPS), "u={} v={} got {:?} expected {:?}", u, v, pt, expected);
            }
        }
    }
}
