//! `Geom` — analytic 3D curves and surfaces with exact parametric evaluation,
//! mirroring OpenCascade's `Geom_Curve` / `Geom_Surface` hierarchy.
//!
//! This is the exact-geometry foundation: a [`Surface`] knows its closed-form
//! point and normal at any `(u, v)`, a [`Curve`] its point and tangent at any
//! `t`. The topology layer ([`crate::topods`]) attaches these to edges/faces;
//! the mesher samples them; the boolean intersects them.

use crate::bsplines::{BSplineCurve, BSplineSurface};
use crate::gp::{Ax3, Pnt};
use std::f64::consts::PI;

/// An analytic or freeform 3D curve (`Geom_Curve`).
#[derive(Clone, Debug)]
// occt: Geom_Line, Geom_Circle, Geom_Ellipse, Geom_Curve
pub enum Curve {
    /// Infinite line: `origin + t * dir`.
    Line { origin: Pnt, dir: Pnt },
    /// Circle of `radius` in the XY plane of `placement`.
    Circle { placement: Ax3, radius: f64 },
    /// Ellipse with `major`/`minor` radii in the XY plane of `placement`.
    Ellipse {
        placement: Ax3,
        major: f64,
        minor: f64,
    },
    /// Freeform B-spline / NURBS curve.
    BSpline(BSplineCurve<Pnt>),
}

impl Curve {
    /// Natural parameter range of the curve.
    pub fn range(&self) -> (f64, f64) {
        match self {
            Curve::Line { .. } => (0.0, 1.0),
            Curve::Circle { .. } | Curve::Ellipse { .. } => (0.0, 2.0 * PI),
            Curve::BSpline(c) => (c.first_param(), c.last_param()),
        }
    }

    /// Point on the curve at parameter `t`.
    pub fn value(&self, t: f64) -> Pnt {
        match self {
            Curve::Line { origin, dir } => *origin + *dir * t,
            Curve::Circle { placement, radius } => {
                placement.to_world(radius * t.cos(), radius * t.sin(), 0.0)
            }
            Curve::Ellipse {
                placement,
                major,
                minor,
            } => placement.to_world(major * t.cos(), minor * t.sin(), 0.0),
            Curve::BSpline(c) => c.value(t),
        }
    }

    /// Tangent vector at parameter `t`.
    pub fn d1(&self, t: f64) -> Pnt {
        match self {
            Curve::Line { dir, .. } => *dir,
            Curve::Circle { placement, radius } => {
                placement.x_dir * (-radius * t.sin()) + placement.y_dir * (radius * t.cos())
            }
            Curve::Ellipse {
                placement,
                major,
                minor,
            } => placement.x_dir * (-major * t.sin()) + placement.y_dir * (minor * t.cos()),
            Curve::BSpline(c) => c.d1(t),
        }
    }

    /// Sample `segments+1` points across the (optionally restricted) range.
    pub fn sample(&self, t0: f64, t1: f64, segments: usize) -> Vec<Pnt> {
        (0..=segments)
            .map(|i| self.value(t0 + (t1 - t0) * i as f64 / segments as f64))
            .collect()
    }
}

/// An analytic or freeform 3D surface (`Geom_Surface`).
#[derive(Clone, Debug)]
// occt: Geom_Plane, Geom_CylindricalSurface, Geom_ConicalSurface, Geom_SphericalSurface, Geom_ToroidalSurface, Geom_SurfaceOfRevolution, Geom_SurfaceOfLinearExtrusion, Geom_Surface
pub enum Surface {
    /// Plane: `origin + u*X + v*Y`, normal `Z`.
    Plane { placement: Ax3 },
    /// Cylinder of `radius` about the placement Z axis.
    Cylinder { placement: Ax3, radius: f64 },
    /// Cone with base `radius` at v=0 growing by `tan(half_angle)`.
    Cone {
        placement: Ax3,
        radius: f64,
        half_angle: f64,
    },
    /// Sphere of `radius` centered at the placement origin.
    Sphere { placement: Ax3, radius: f64 },
    /// Torus with `major` ring radius and `minor` tube radius.
    Torus {
        placement: Ax3,
        major: f64,
        minor: f64,
    },
    /// Surface of revolution: a profile `curve` swept about `axis` (origin +
    /// unit direction). `u` is the revolution angle, `v` the profile parameter.
    Revolution {
        curve: Box<Curve>,
        axis_origin: Pnt,
        axis_dir: Pnt,
    },
    /// Linear extrusion of a profile `curve` along `dir`. `u` is the profile
    /// parameter, `v` the extrusion distance.
    Extrusion { curve: Box<Curve>, dir: Pnt },
    /// Freeform B-spline / NURBS surface.
    BSpline(BSplineSurface),
}

impl Surface {
    /// `(u, v)` parameter ranges. Planes/extrusions use the profile/− where
    /// unbounded; faces clip these with their wire bounds.
    pub fn u_range(&self) -> (f64, f64) {
        match self {
            Surface::Plane { .. } => (0.0, 1.0),
            Surface::Cylinder { .. } | Surface::Cone { .. } | Surface::Torus { .. } => {
                (0.0, 2.0 * PI)
            }
            Surface::Sphere { .. } => (0.0, 2.0 * PI),
            Surface::Revolution { .. } => (0.0, 2.0 * PI),
            Surface::Extrusion { curve, .. } => curve.range(),
            Surface::BSpline(s) => s.u_range(),
        }
    }

    pub fn v_range(&self) -> (f64, f64) {
        match self {
            Surface::Plane { .. } => (0.0, 1.0),
            Surface::Cylinder { .. } | Surface::Cone { .. } => (0.0, 1.0),
            Surface::Sphere { .. } => (-PI / 2.0, PI / 2.0),
            Surface::Torus { .. } => (0.0, 2.0 * PI),
            Surface::Revolution { curve, .. } => curve.range(),
            Surface::Extrusion { .. } => (0.0, 1.0),
            Surface::BSpline(s) => s.v_range(),
        }
    }

    /// Point on the surface at `(u, v)`.
    pub fn value(&self, u: f64, v: f64) -> Pnt {
        match self {
            Surface::Plane { placement } => placement.to_world(u, v, 0.0),
            Surface::Cylinder { placement, radius } => {
                placement.to_world(radius * u.cos(), radius * u.sin(), v)
            }
            Surface::Cone {
                placement,
                radius,
                half_angle,
            } => {
                let r = radius + v * half_angle.tan();
                placement.to_world(r * u.cos(), r * u.sin(), v)
            }
            Surface::Sphere { placement, radius } => placement.to_world(
                radius * v.cos() * u.cos(),
                radius * v.cos() * u.sin(),
                radius * v.sin(),
            ),
            Surface::Torus {
                placement,
                major,
                minor,
            } => {
                let rr = major + minor * v.cos();
                placement.to_world(rr * u.cos(), rr * u.sin(), minor * v.sin())
            }
            Surface::Revolution {
                curve,
                axis_origin,
                axis_dir,
            } => rotate_about(curve.value(v), *axis_origin, *axis_dir, u),
            Surface::Extrusion { curve, dir } => curve.value(u) + *dir * v,
            Surface::BSpline(s) => s.value(u, v),
        }
    }

    /// Outward unit normal at `(u, v)` (closed-form for analytics).
    pub fn normal(&self, u: f64, v: f64) -> Pnt {
        match self {
            Surface::Plane { placement } => placement.z_dir,
            Surface::Cylinder { placement, .. } => {
                (placement.x_dir * u.cos() + placement.y_dir * u.sin()).normalized()
            }
            Surface::Cone {
                placement,
                half_angle,
                ..
            } => {
                // Radial component tilted by the cone angle.
                let radial = placement.x_dir * u.cos() + placement.y_dir * u.sin();
                (radial * half_angle.cos() - placement.z_dir * half_angle.sin()).normalized()
            }
            Surface::Sphere { placement, .. } => {
                (self.value(u, v) - placement.location).normalized()
            }
            _ => {
                // Numerical normal from finite-difference partials.
                let (u0, u1) = self.u_range();
                let (v0, v1) = self.v_range();
                let hu = ((u1 - u0) * 1e-5).max(1e-9);
                let hv = ((v1 - v0) * 1e-5).max(1e-9);
                let du = self.value((u + hu).min(u1), v) - self.value((u - hu).max(u0), v);
                let dv = self.value(u, (v + hv).min(v1)) - self.value(u, (v - hv).max(v0));
                du.cross(dv).normalized()
            }
        }
    }
}

/// Rotate `p` about the axis through `origin` with unit direction `dir` by
/// `angle` radians (Rodrigues).
fn rotate_about(p: Pnt, origin: Pnt, dir: Pnt, angle: f64) -> Pnt {
    let d = dir.normalized();
    let rel = p - origin;
    let (s, c) = angle.sin_cos();
    let parallel = d * rel.dot(d);
    let perp = rel - parallel;
    let w = d.cross(perp);
    origin + parallel + perp * c + w * s
}

/// Map analytic surface hints through a transform.
///
/// Rigid motions and uniform scaling map the simple analytic kinds exactly
/// (placement transformed, radii scaled). Non-uniform scaling would turn a
/// cylinder into an elliptic cylinder we cannot represent, so those hints are
/// dropped; hints are advisory, so dropping only degrades STEP output to the
/// faceted fallback for the affected faces.
pub fn transform_hints(hints: &[Surface], t: &crate::gp::Trsf) -> Vec<Surface> {
    let det = t.linear_det().abs();
    let s = t.scale_factor();
    // uniform iff |det| == s^3 (within tolerance)
    let uniform = (det - s * s * s).abs() <= 1e-9 * det.max(1.0);
    if !uniform || s <= 0.0 {
        return Vec::new();
    }
    hints
        .iter()
        .filter_map(|h| match h {
            Surface::Plane { placement } => Some(Surface::Plane {
                placement: placement.transformed(t),
            }),
            Surface::Cylinder { placement, radius } => Some(Surface::Cylinder {
                placement: placement.transformed(t),
                radius: radius * s,
            }),
            Surface::Cone {
                placement,
                radius,
                half_angle,
            } => Some(Surface::Cone {
                placement: placement.transformed(t),
                radius: radius * s,
                half_angle: *half_angle,
            }),
            Surface::Sphere { placement, radius } => Some(Surface::Sphere {
                placement: placement.transformed(t),
                radius: radius * s,
            }),
            Surface::Torus {
                placement,
                major,
                minor,
            } => Some(Surface::Torus {
                placement: placement.transformed(t),
                major: major * s,
                minor: minor * s,
            }),
            // Curve-backed and freeform kinds: not worth mapping for now.
            Surface::Revolution { .. } | Surface::Extrusion { .. } | Surface::BSpline(_) => None,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn circle_curve_points_on_radius() {
        let c = Curve::Circle {
            placement: Ax3::identity(),
            radius: 2.0,
        };
        let p = c.value(PI / 2.0);
        assert!(p.distance(Pnt::new(0.0, 2.0, 0.0)) < 1e-12, "{p:?}");
    }

    #[test]
    fn sphere_surface_normal_radial() {
        let s = Surface::Sphere {
            placement: Ax3::identity(),
            radius: 3.0,
        };
        let p = s.value(0.0, 0.0);
        assert!(p.distance(Pnt::new(3.0, 0.0, 0.0)) < 1e-12, "{p:?}");
        let n = s.normal(0.0, 0.0);
        assert!(n.distance(Pnt::new(1.0, 0.0, 0.0)) < 1e-9, "{n:?}");
    }

    #[test]
    fn cylinder_surface_value_and_normal() {
        let s = Surface::Cylinder {
            placement: Ax3::identity(),
            radius: 1.0,
        };
        let p = s.value(0.0, 5.0);
        assert!(p.distance(Pnt::new(1.0, 0.0, 5.0)) < 1e-12, "{p:?}");
        let n = s.normal(0.0, 5.0);
        assert!(n.distance(Pnt::new(1.0, 0.0, 0.0)) < 1e-9, "{n:?}");
    }
}
