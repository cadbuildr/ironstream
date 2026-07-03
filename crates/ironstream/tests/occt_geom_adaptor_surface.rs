// FILE: tests/occt_geom_adaptor_surface.rs
extern crate ironstream;
use ironstream::geom_adaptor_surface::*;
use ironstream::geom_abs::SurfaceType as GeomAbs_SurfaceType;
use std::f64::consts::PI;

const EPS: f64 = 1e-7;

fn approx_eq(a: f64, b: f64, eps: f64) -> bool {
    (a - b).abs() < eps
}

fn vec_approx_eq(a: [f64; 3], b: [f64; 3], eps: f64) -> bool {
    approx_eq(a[0], b[0], eps) && approx_eq(a[1], b[1], eps) && approx_eq(a[2], b[2], eps)
}

#[test]
fn test_sphere_surface_type_is_sphere() {
    let surf = GeomAdaptorSurface::from_sphere(
        [0.0, 0.0, 0.0],
        [0.0, 0.0, 1.0],
        [1.0, 0.0, 0.0],
        5.0,
    );
    assert!(
        matches!(surf.surface_type(), GeomAbs_SurfaceType::Sphere),
        "surface_type should be Sphere"
    );
}

#[test]
fn test_sphere_value_at_zero_zero() {
    let r = 5.0;
    let surf = GeomAdaptorSurface::from_sphere(
        [0.0, 0.0, 0.0],
        [0.0, 0.0, 1.0],
        [1.0, 0.0, 0.0],
        r,
    );
    let pt = surf.value(0.0, 0.0);
    assert!(
        vec_approx_eq(pt, [r, 0.0, 0.0], EPS),
        "sphere at (0,0) should be ({}, 0, 0), got {:?}",
        r,
        pt
    );
}

#[test]
fn test_sphere_value_north_pole() {
    let r = 5.0;
    let surf = GeomAdaptorSurface::from_sphere(
        [0.0, 0.0, 0.0],
        [0.0, 0.0, 1.0],
        [1.0, 0.0, 0.0],
        r,
    );
    let pt = surf.value(0.0, PI / 2.0);
    assert!(
        vec_approx_eq(pt, [0.0, 0.0, r], EPS),
        "sphere north pole should be (0,0,{}), got {:?}",
        r,
        pt
    );
}

#[test]
fn test_sphere_all_points_on_surface() {
    let r = 7.0;
    let surf = GeomAdaptorSurface::from_sphere(
        [0.0, 0.0, 0.0],
        [0.0, 0.0, 1.0],
        [1.0, 0.0, 0.0],
        r,
    );
    for i in 0..16 {
        for j in 0..8 {
            let u = i as f64 * PI / 8.0;
            let v = -PI / 2.0 + j as f64 * PI / 7.0;
            let pt = surf.value(u, v);
            let dist = (pt[0] * pt[0] + pt[1] * pt[1] + pt[2] * pt[2]).sqrt();
            assert!(
                approx_eq(dist, r, EPS),
                "sphere point distance should be {}, got {} at u={} v={}",
                r,
                dist,
                u,
                v
            );
        }
    }
}

#[test]
fn test_sphere_d1_tangents_orthogonal_to_normal() {
    let r = 3.0;
    let surf = GeomAdaptorSurface::from_sphere(
        [0.0, 0.0, 0.0],
        [0.0, 0.0, 1.0],
        [1.0, 0.0, 0.0],
        r,
    );
    let u = 0.7;
    let v = 0.4;
    let (pt, du, dv) = surf.d1(u, v);
    // Normal direction at a sphere point is the point itself (for unit sphere)
    // du and dv should be tangent to the sphere => dot(pt, du) ~ 0, dot(pt, dv) ~ 0
    let dot_pt_du = pt[0] * du[0] + pt[1] * du[1] + pt[2] * du[2];
    let dot_pt_dv = pt[0] * dv[0] + pt[1] * dv[1] + pt[2] * dv[2];
    assert!(
        approx_eq(dot_pt_du, 0.0, 1e-6),
        "du should be tangent to sphere, dot={}", dot_pt_du
    );
    assert!(
        approx_eq(dot_pt_dv, 0.0, 1e-6),
        "dv should be tangent to sphere, dot={}", dot_pt_dv
    );
}

#[test]
fn test_sphere_u_periodic_v_not() {
    let surf = GeomAdaptorSurface::from_sphere(
        [0.0, 0.0, 0.0],
        [0.0, 0.0, 1.0],
        [1.0, 0.0, 0.0],
        4.0,
    );
    assert!(surf.is_u_periodic(), "sphere should be periodic in U");
    assert!(!surf.is_v_periodic(), "sphere should not be periodic in V");
    assert!(surf.is_u_closed(), "sphere should be closed in U");
    assert!(!surf.is_v_closed(), "sphere should not be closed in V");
}

#[test]
fn test_sphere_accessor_returns_correct_data() {
    let center = [1.0, 2.0, 3.0];
    let r = 9.0;
    let surf = GeomAdaptorSurface::from_sphere(center, [0.0, 0.0, 1.0], [1.0, 0.0, 0.0], r);
    let sph = surf.sphere().expect("should return sphere data");
    assert!(approx_eq(sph.radius, r, EPS));
    assert!(vec_approx_eq(sph.center, center, EPS));
    assert!(surf.plane().is_none(), "should not return plane data");
    assert!(surf.cylinder().is_none(), "should not return cylinder data");
    assert!(surf.cone().is_none(), "should not return cone data");
    assert!(surf.torus().is_none(), "should not return torus data");
}

#[test]
fn test_plane_surface_type_and_basic_value() {
    let surf = GeomAdaptorSurface::from_plane(
        [0.0, 0.0, 0.0],
        [0.0, 0.0, 1.0],
        [1.0, 0.0, 0.0],
    );
    assert!(matches!(surf.surface_type(), GeomAbs_SurfaceType::Plane));
    let pt = surf.value(3.0, 4.0);
    assert!(vec_approx_eq(pt, [3.0, 4.0, 0.0], EPS), "got {:?}", pt);
}

#[test]
fn test_cylinder_surface_type_and_value() {
    let surf = GeomAdaptorSurface::from_cylinder(
        [0.0, 0.0, 0.0],
        [0.0, 0.0, 1.0],
        [1.0, 0.0, 0.0],
        2.0,
    );
    assert!(matches!(surf.surface_type(), GeomAbs_SurfaceType::Cylinder));
    let pt = surf.value(PI / 2.0, 5.0);
    assert!(vec_approx_eq(pt, [0.0, 2.0, 5.0], EPS), "got {:?}", pt);
}

#[test]
fn test_torus_surface_type_and_closed() {
    let surf = GeomAdaptorSurface::from_torus(
        [0.0, 0.0, 0.0],
        [0.0, 0.0, 1.0],
        [1.0, 0.0, 0.0],
        5.0,
        1.0,
    );
    assert!(matches!(surf.surface_type(), GeomAbs_SurfaceType::Torus));
    assert!(surf.is_u_closed());
    assert!(surf.is_v_closed());
}

#[test]
fn test_torus_value_outer_point() {
    // At u=0, v=0 the outermost point: (R+r, 0, 0)
    let big_r = 5.0;
    let small_r = 1.0;
    let surf = GeomAdaptorSurface::from_torus(
        [0.0, 0.0, 0.0],
        [0.0, 0.0, 1.0],
        [1.0, 0.0, 0.0],
        big_r,
        small_r,
    );
    let pt = surf.value(0.0, 0.0);
    assert!(vec_approx_eq(pt, [big_r + small_r, 0.0, 0.0], EPS), "got {:?}", pt);
}

#[test]
fn test_torus_value_inner_point() {
    // At u=0, v=PI the innermost point: (R-r, 0, 0)
    let big_r = 5.0;
    let small_r = 1.0;
    let surf = GeomAdaptorSurface::from_torus(
        [0.0, 0.0, 0.0],
        [0.0, 0.0, 1.0],
        [1.0, 0.0, 0.0],
        big_r,
        small_r,
    );
    let pt = surf.value(0.0, PI);
    assert!(vec_approx_eq(pt, [big_r - small_r, 0.0, 0.0], EPS), "got {:?}", pt);
}

#[test]
fn test_cone_surface_type_and_apex() {
    // cone apex at v = -radius/tan(angle)
    let r = 2.0;
    let angle = PI / 4.0;
    let surf = GeomAdaptorSurface::from_cone(
        [0.0, 0.0, 0.0],
        [0.0, 0.0, 1.0],
        [1.0, 0.0, 0.0],
        r,
        angle,
    );
    assert!(matches!(surf.surface_type(), GeomAbs_SurfaceType::Cone));
    // at v = -r/tan(PI/4) = -2, r_at_v = 2 + (-2)*1 = 0 => apex
    let v_apex = -r / angle.tan();
    let pt = surf.value(0.0, v_apex);
    let dist = (pt[0] * pt[0] + pt[1] * pt[1]).sqrt();
    assert!(approx_eq(dist, 0.0, EPS), "apex radial distance should be 0, got {}", dist);
}

#[test]
fn test_extrusion_surface_type() {
    let surf = GeomAdaptorSurface::from_extrusion(
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 0.0],
        1.0,
    );
    assert!(matches!(surf.surface_type(), GeomAbs_SurfaceType::ExtrusionSurface));
}

#[test]
fn test_revolution_surface_type() {
    let surf = GeomAdaptorSurface::from_revolution(
        [0.0, 0.0, 0.0],
        [0.0, 0.0, 1.0],
        [1.0, 0.0, 0.0],
    );
    assert!(matches!(surf.surface_type(), GeomAbs_SurfaceType::RevolutionSurface));
}

#[test]
fn test_cylinder_u_iso_is_line() {
    let surf = GeomAdaptorSurface::from_cylinder(
        [0.0, 0.0, 0.0],
        [0.0, 0.0, 1.0],
        [1.0, 0.0, 0.0],
        2.0,
    );
    let iso = surf.u_iso(0.0);
    match iso {
        ironstream::geom_adaptor_curve::GeomAdaptorCurveKind::RawLine { origin, direction } => {
            assert!(vec_approx_eq(origin, [2.0, 0.0, 0.0], EPS));
            assert!(vec_approx_eq(direction, [0.0, 0.0, 1.0], EPS));
        }
        other => panic!("Expected Line, got {:?}", other),
    }
}

#[test]
fn test_sphere_parameter_bounds() {
    let surf = GeomAdaptorSurface::from_sphere(
        [0.0, 0.0, 0.0],
        [0.0, 0.0, 1.0],
        [1.0, 0.0, 0.0],
        1.0,
    );
    assert!(approx_eq(surf.u_first(), 0.0, EPS));
    assert!(approx_eq(surf.u_last(), 2.0 * PI, EPS));
    assert!(approx_eq(surf.v_first(), -PI / 2.0, EPS));
    assert!(approx_eq(surf.v_last(), PI / 2.0, EPS));
}
