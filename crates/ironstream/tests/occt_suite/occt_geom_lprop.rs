// FILE: tests/occt_geom_lprop.rs
//! Integration tests for `geom_lprop` — local properties of 3D curves and
//! surfaces.
//!
//! Covers `CLProps`, `SLProps`, `CurveTool`, and `SurfaceTool` through the
//! public API only, using realistic geometric objects (circles, lines, planes,
//! spheres, cylinders, B-splines).
//!
//! Mirrors the spirit of OCCT's `GeomLProp_Test.cxx` with numerically verified
//! expectations.

extern crate ironstream;

use ironstream::gp::{Ax1, Ax3, Pnt};
use ironstream::geom_circle::GeomCircle;
use ironstream::geom_line::GeomLine;
use ironstream::geom_lprop::{
    CLProps, Curve, CurveTool, SLProps, Surface, SurfaceTool,
};
use std::f64::consts::{FRAC_PI_2, PI, TAU};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn near(a: f64, b: f64, tol: f64, msg: &str) {
    assert!(
        (a - b).abs() <= tol,
        "{msg}: expected {a} ≈ {b} (tol {tol})"
    );
}

fn near_pnt(a: Pnt, b: Pnt, tol: f64, msg: &str) {
    let d = a.distance(b);
    assert!(d <= tol, "{msg}: |{a:?} - {b:?}| = {d} > {tol}");
}

/// Build a unit circle in the XY plane centered at the origin.
fn unit_circle() -> Curve {
    let ax = Ax3::identity();
    Curve::Circle(GeomCircle::from_ax3(ax, 1.0))
}

/// Build a circle of radius `r` in the XY plane.
fn circle_r(r: f64) -> Curve {
    let ax = Ax3::identity();
    Curve::Circle(GeomCircle::from_ax3(ax, r))
}

/// Build a line along the X axis starting at the origin.
fn x_axis_line() -> Curve {
    let ax1 = Ax1::new(Pnt::origin(), Pnt::new(1.0, 0.0, 0.0));
    Curve::Line(GeomLine::new(ax1))
}

/// Build the XY plane surface.
fn xy_plane() -> Surface {
    Surface::Plane {
        origin: Pnt::origin(),
        x_dir: Pnt::new(1.0, 0.0, 0.0),
        y_dir: Pnt::new(0.0, 1.0, 0.0),
        normal: Pnt::new(0.0, 0.0, 1.0),
    }
}

/// Build a unit sphere (r=1) centered at origin.
fn unit_sphere() -> Surface {
    Surface::Sphere {
        center: Pnt::origin(),
        radius: 1.0,
        z_dir: Pnt::new(0.0, 0.0, 1.0),
        x_dir: Pnt::new(1.0, 0.0, 0.0),
        y_dir: Pnt::new(0.0, 1.0, 0.0),
    }
}

/// Build a sphere of radius `r`.
fn sphere_r(r: f64) -> Surface {
    Surface::Sphere {
        center: Pnt::origin(),
        radius: r,
        z_dir: Pnt::new(0.0, 0.0, 1.0),
        x_dir: Pnt::new(1.0, 0.0, 0.0),
        y_dir: Pnt::new(0.0, 1.0, 0.0),
    }
}

/// Build a unit cylinder (r=1) with Z-axis.
fn unit_cylinder() -> Surface {
    Surface::Cylinder {
        origin: Pnt::origin(),
        axis_dir: Pnt::new(0.0, 0.0, 1.0),
        x_dir: Pnt::new(1.0, 0.0, 0.0),
        y_dir: Pnt::new(0.0, 1.0, 0.0),
        radius: 1.0,
    }
}

// ===========================================================================
// CurveTool tests
// ===========================================================================

/// `GeomLProp_CurveTool::FirstParameter` / `LastParameter` for a circle.
#[test]
fn curve_tool_circle_parameter_range() {
    let c = unit_circle();
    near(CurveTool::first_parameter(&c), 0.0, 1e-12, "first_parameter");
    near(CurveTool::last_parameter(&c), TAU, 1e-12, "last_parameter");
}

/// `GeomLProp_CurveTool::Value` samples on the unit circle.
#[test]
fn curve_tool_circle_value_samples() {
    let c = unit_circle();
    // At u = 0: (1, 0, 0)
    near_pnt(CurveTool::value(&c, 0.0), Pnt::new(1.0, 0.0, 0.0), 1e-10, "u=0");
    // At u = π/2: (0, 1, 0)
    let p = CurveTool::value(&c, FRAC_PI_2);
    near(p.x, 0.0, 1e-10, "u=π/2 x");
    near(p.y, 1.0, 1e-10, "u=π/2 y");
    // At u = π: (−1, 0, 0)
    let p = CurveTool::value(&c, PI);
    near(p.x, -1.0, 1e-10, "u=π x");
}

/// `GeomLProp_CurveTool::D1` — the first derivative of a circle is tangential.
#[test]
fn curve_tool_circle_d1_is_tangential() {
    let c = unit_circle();
    let (p, d1) = CurveTool::d1(&c, 0.0);
    // The tangent at u=0 for a unit circle is (0, 1, 0).
    near(d1.x, 0.0, 1e-10, "d1x");
    near(d1.y, 1.0, 1e-10, "d1y");
    near(d1.z, 0.0, 1e-10, "d1z");
    // Tangent is perpendicular to radius.
    let radius_dir = p - Pnt::origin();
    near(radius_dir.dot(d1), 0.0, 1e-10, "radius·tangent = 0");
}

/// `GeomLProp_CurveTool::D2` — the second derivative of a circle points inward.
#[test]
fn curve_tool_circle_d2_centripetal() {
    let c = circle_r(2.0);
    let (_p, _d1, d2) = CurveTool::d2(&c, 0.0);
    // For a circle of radius r: D2(0) = (−r, 0, 0) = (−2, 0, 0).
    near(d2.x, -2.0, 1e-10, "d2x at u=0 for r=2");
    near(d2.y, 0.0, 1e-10, "d2y");
}

/// `GeomLProp_CurveTool::D1` for a line returns the direction vector.
#[test]
fn curve_tool_line_d1_is_direction() {
    let c = x_axis_line();
    let (p, d1) = CurveTool::d1(&c, 5.0);
    near(p.x, 5.0, 1e-12, "position x at u=5");
    near(d1.x, 1.0, 1e-12, "d1 = unit X");
    near(d1.y, 0.0, 1e-12, "d1y = 0");
    near(d1.z, 0.0, 1e-12, "d1z = 0");
}

// ===========================================================================
// CLProps — curve local properties
// ===========================================================================

/// Curvature of a unit circle = 1/r = 1 everywhere.
#[test]
fn clprops_unit_circle_curvature_is_one() {
    let c = unit_circle();
    let mut props = CLProps::new(c, 2, 1e-10);
    for &u in &[0.0, FRAC_PI_2, PI, 3.0 * FRAC_PI_2] {
        props.set_parameter(u);
        near(props.curvature(), 1.0, 1e-8, &format!("curvature at u={u}"));
    }
}

/// Curvature of a circle of radius r = 1/r.
#[test]
fn clprops_circle_radius_3_curvature() {
    let c = circle_r(3.0);
    let mut props = CLProps::new(c, 2, 1e-10);
    props.set_parameter(0.0);
    near(props.curvature(), 1.0 / 3.0, 1e-8, "curvature r=3");
}

/// Centre of curvature of a unit circle lies at the origin.
#[test]
fn clprops_unit_circle_centre_of_curvature() {
    let c = unit_circle();
    let mut props = CLProps::new(c, 2, 1e-10);
    for &u in &[0.0, 1.0, 2.0, PI] {
        props.set_parameter(u);
        let coc = props.centre_of_curvature();
        near_pnt(
            coc,
            Pnt::origin(),
            1e-8,
            &format!("centre of curvature at u={u}"),
        );
    }
}

/// The tangent is always a unit vector.
#[test]
fn clprops_tangent_is_unit() {
    let c = unit_circle();
    let mut props = CLProps::new(c, 2, 1e-10);
    for &u in &[0.0, 0.5, 1.0, 2.0, 3.0] {
        props.set_parameter(u);
        near(props.tangent().norm(), 1.0, 1e-10, &format!("|tangent| at u={u}"));
    }
}

/// The normal is always a unit vector.
#[test]
fn clprops_normal_is_unit() {
    let c = unit_circle();
    let mut props = CLProps::new(c, 2, 1e-10);
    for &u in &[0.0, 0.5, 1.0, PI] {
        props.set_parameter(u);
        near(props.normal().norm(), 1.0, 1e-10, &format!("|normal| at u={u}"));
    }
}

/// Tangent, normal and bi-normal form a right-handed frame.
#[test]
fn clprops_frenet_frame_is_right_handed() {
    let c = unit_circle();
    let mut props = CLProps::new(c, 2, 1e-10);
    props.set_parameter(0.5);
    let t = props.tangent();
    let n = props.normal();
    let b = props.bi_normal();
    // T × N = B.
    let computed_b = t.cross(n).normalized();
    near_pnt(computed_b, b, 1e-8, "T × N = B");
    // All unit vectors.
    near(t.norm(), 1.0, 1e-10, "|T|");
    near(n.norm(), 1.0, 1e-10, "|N|");
    near(b.norm(), 1.0, 1e-10, "|B|");
    // Mutually orthogonal.
    near(t.dot(n), 0.0, 1e-8, "T·N = 0");
    near(t.dot(b), 0.0, 1e-8, "T·B = 0");
    near(n.dot(b), 0.0, 1e-8, "N·B = 0");
}

/// A straight line has zero curvature and the normal is not defined.
#[test]
fn clprops_line_zero_curvature_undefined_normal() {
    let c = x_axis_line();
    let mut props = CLProps::new(c, 2, 1e-10);
    props.set_parameter(7.0);
    near(props.curvature(), 0.0, 1e-12, "line curvature = 0");
    assert!(!props.is_normal_defined(), "normal not defined on a line");
    assert!(props.is_tangent_defined(), "tangent defined on a line");
}

/// Torsion of a planar curve is zero.
#[test]
fn clprops_circle_torsion_zero() {
    let c = unit_circle();
    let mut props = CLProps::new(c, 3, 1e-10);
    for &u in &[0.0, 0.7, 1.5, PI] {
        props.set_parameter(u);
        near(props.torsion(), 0.0, 1e-8, &format!("torsion at u={u}"));
    }
}

// ===========================================================================
// SurfaceTool tests
// ===========================================================================

/// The XY plane returns the correct point and normals from D1.
#[test]
fn surface_tool_plane_d1() {
    let s = xy_plane();
    let (p, du, dv) = SurfaceTool::d1(&s, 3.0, 5.0);
    near_pnt(p, Pnt::new(3.0, 5.0, 0.0), 1e-10, "point on plane");
    near_pnt(du, Pnt::new(1.0, 0.0, 0.0), 1e-10, "dU = X");
    near_pnt(dv, Pnt::new(0.0, 1.0, 0.0), 1e-10, "dV = Y");
}

/// The sphere point lies on the sphere surface.
#[test]
fn surface_tool_sphere_point_on_surface() {
    let s = unit_sphere();
    for (u, v) in [
        (0.0_f64, 0.0_f64),
        (FRAC_PI_2, 0.0),
        (0.0, PI),
        (-FRAC_PI_2, FRAC_PI_2),
    ] {
        let p = SurfaceTool::value(&s, u, v);
        near(
            p.norm(),
            1.0,
            1e-10,
            &format!("point on unit sphere at ({u},{v})"),
        );
    }
}

// ===========================================================================
// SLProps — surface local properties
// ===========================================================================

/// The XY plane has a +Z unit normal everywhere.
#[test]
fn slprops_plane_normal_is_z() {
    let s = xy_plane();
    let mut props = SLProps::new(s, 2, 1e-10);
    for (u, v) in [(0.0, 0.0), (3.0, -2.0), (100.0, 0.5)] {
        props.set_parameters(u, v);
        let n = props.normal();
        near(n.x, 0.0, 1e-10, "plane normal x");
        near(n.y, 0.0, 1e-10, "plane normal y");
        near(n.z, 1.0, 1e-10, "plane normal z");
    }
}

/// A plane has K = 0 and H = 0 everywhere.
#[test]
fn slprops_plane_zero_curvatures() {
    let s = xy_plane();
    let mut props = SLProps::new(s, 2, 1e-10);
    props.set_parameters(1.0, 2.0);
    near(props.gaussian_curvature(), 0.0, 1e-12, "plane K = 0");
    near(props.mean_curvature(), 0.0, 1e-12, "plane H = 0");
    let (k1, k2) = props.principal_curvatures();
    near(k1, 0.0, 1e-12, "plane k1 = 0");
    near(k2, 0.0, 1e-12, "plane k2 = 0");
}

/// A unit sphere has K = 1 (Gaussian) everywhere.
#[test]
fn slprops_unit_sphere_gaussian_curvature() {
    let s = unit_sphere();
    let mut props = SLProps::new(s, 2, 1e-10);
    for (u, v) in [(0.0, 0.0), (0.3, 1.0), (-0.5, 2.5)] {
        props.set_parameters(u, v);
        near(props.gaussian_curvature(), 1.0, 1e-7, &format!("K sphere ({u},{v})"));
    }
}

/// A sphere of radius r has K = 1/r².
#[test]
fn slprops_sphere_gaussian_curvature_scales_with_radius() {
    for r in [1.0_f64, 2.0, 5.0] {
        let s = sphere_r(r);
        let mut props = SLProps::new(s, 2, 1e-10);
        props.set_parameters(0.0, 0.0);
        near(
            props.gaussian_curvature(),
            1.0 / (r * r),
            1e-7,
            &format!("K sphere r={r}"),
        );
    }
}

/// A sphere is an umbilic surface (both principal curvatures are equal).
#[test]
fn slprops_sphere_is_umbilic_everywhere() {
    let s = unit_sphere();
    let mut props = SLProps::new(s, 2, 1e-10);
    for (u, v) in [(0.0, 0.0), (0.3, 1.0)] {
        props.set_parameters(u, v);
        assert!(props.is_umbilic(), "sphere should be umbilic at ({u},{v})");
    }
}

/// A cylinder has K = 0 (one principal curvature is zero).
#[test]
fn slprops_cylinder_gaussian_curvature_zero() {
    let s = unit_cylinder();
    let mut props = SLProps::new(s, 2, 1e-10);
    for (u, v) in [(0.0, 0.0), (1.0, FRAC_PI_2), (3.0, PI)] {
        props.set_parameters(u, v);
        near(
            props.gaussian_curvature(),
            0.0,
            1e-8,
            &format!("cylinder K = 0 at ({u},{v})"),
        );
    }
}

/// A unit cylinder has H = ±0.5 (mean curvature = (κ₁ + κ₂)/2 = (1/r + 0)/2).
#[test]
fn slprops_unit_cylinder_mean_curvature() {
    let s = unit_cylinder();
    let mut props = SLProps::new(s, 2, 1e-10);
    props.set_parameters(0.0, 0.0);
    near(props.mean_curvature().abs(), 0.5, 1e-8, "cylinder |H| = 0.5");
}

/// A cylinder is not an umbilic surface (except at degenerate points).
#[test]
fn slprops_cylinder_is_not_umbilic() {
    let s = unit_cylinder();
    let mut props = SLProps::new(s, 2, 1e-10);
    props.set_parameters(0.0, 0.0);
    assert!(!props.is_umbilic(), "cylinder should not be umbilic");
}

/// The surface normal is a unit vector everywhere.
#[test]
fn slprops_normal_is_always_unit() {
    let surfaces: Vec<Surface> = vec![
        xy_plane(),
        unit_sphere(),
        unit_cylinder(),
    ];
    for s in surfaces {
        let label = format!("{s:?}");
        let mut props = SLProps::new(s, 2, 1e-10);
        for (u, v) in [(0.0_f64, 0.0_f64), (1.0, 1.0), (2.0, 3.0)] {
            props.set_parameters(u, v);
            if props.is_normal_defined() {
                let n = props.normal();
                near(
                    n.norm(),
                    1.0,
                    1e-9,
                    &format!("|normal| = 1 for {label} at ({u},{v})"),
                );
            }
        }
    }
}
