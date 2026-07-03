// FILE: tests/occt_geom_rect_trimmed.rs
extern crate ironstream;
use ironstream::geom_rect_trimmed::{
    GeomRectangularTrimmedSurface,
    GeomSurfaceKind,
    GeomSphericalSurface,
    GeomCylindricalSurface,
    GeomConicalSurface,
    GeomToroidalSurface,
};
use ironstream::geom_plane::GeomPlane;
use ironstream::gp::{Ax3, Pnt};

fn xy_plane() -> GeomPlane {
    GeomPlane::from_ax3(Ax3::identity())
}

#[test]
fn integration_plane_trim_bounds() {
    let surf = GeomRectangularTrimmedSurface::new(
        GeomSurfaceKind::Plane(xy_plane()),
        -5.0, 5.0, -3.0, 3.0,
        true, true,
    );
    assert!((surf.u_first() - (-5.0)).abs() < 1e-14);
    assert!((surf.u_last() - 5.0).abs() < 1e-14);
    assert!((surf.v_first() - (-3.0)).abs() < 1e-14);
    assert!((surf.v_last() - 3.0).abs() < 1e-14);
}

#[test]
fn integration_plane_interior_point() {
    let surf = GeomRectangularTrimmedSurface::new(
        GeomSurfaceKind::Plane(xy_plane()),
        -5.0, 5.0, -3.0, 3.0,
        true, true,
    );
    let p = surf.value(2.0, -1.5);
    assert!((p.x - 2.0).abs() < 1e-12);
    assert!((p.y - (-1.5)).abs() < 1e-12);
    assert!(p.z.abs() < 1e-12);
}

#[test]
fn integration_plane_u_clamped_high() {
    let surf = GeomRectangularTrimmedSurface::new(
        GeomSurfaceKind::Plane(xy_plane()),
        -5.0, 5.0, -3.0, 3.0,
        true, true,
    );
    let p = surf.value(100.0, 0.0);
    assert!((p.x - 5.0).abs() < 1e-12);
}

#[test]
fn integration_plane_u_clamped_low() {
    let surf = GeomRectangularTrimmedSurface::new(
        GeomSurfaceKind::Plane(xy_plane()),
        -5.0, 5.0, -3.0, 3.0,
        true, true,
    );
    let p = surf.value(-100.0, 0.0);
    assert!((p.x - (-5.0)).abs() < 1e-12);
}

#[test]
fn integration_plane_v_clamped() {
    let surf = GeomRectangularTrimmedSurface::new(
        GeomSurfaceKind::Plane(xy_plane()),
        -5.0, 5.0, -3.0, 3.0,
        true, true,
    );
    let p = surf.value(0.0, 10.0);
    assert!((p.y - 3.0).abs() < 1e-12);
}

#[test]
fn integration_plane_d1_at_origin() {
    let surf = GeomRectangularTrimmedSurface::new(
        GeomSurfaceKind::Plane(xy_plane()),
        -5.0, 5.0, -3.0, 3.0,
        true, true,
    );
    let (p, du, dv) = surf.d1(0.0, 0.0);
    assert!(p.x.abs() < 1e-12);
    assert!(p.y.abs() < 1e-12);
    assert!(p.z.abs() < 1e-12);
    assert!((du.x - 1.0).abs() < 1e-12);
    assert!(du.y.abs() < 1e-12);
    assert!(du.z.abs() < 1e-12);
    assert!(dv.x.abs() < 1e-12);
    assert!((dv.y - 1.0).abs() < 1e-12);
    assert!(dv.z.abs() < 1e-12);
}

#[test]
fn integration_set_trim_u_and_evaluate() {
    let mut surf = GeomRectangularTrimmedSurface::new(
        GeomSurfaceKind::Plane(xy_plane()),
        -1.0, 1.0, -1.0, 1.0,
        true, true,
    );
    surf.set_trim_u(0.0, 10.0);
    assert!((surf.u_first() - 0.0).abs() < 1e-14);
    assert!((surf.u_last() - 10.0).abs() < 1e-14);
    let p = surf.value(-5.0, 0.0);
    assert!(p.x.abs() < 1e-12);
}

#[test]
fn integration_set_trim_v_and_evaluate() {
    let mut surf = GeomRectangularTrimmedSurface::new(
        GeomSurfaceKind::Plane(xy_plane()),
        -1.0, 1.0, -1.0, 1.0,
        true, true,
    );
    surf.set_trim_v(2.0, 8.0);
    assert!((surf.v_first() - 2.0).abs() < 1e-14);
    assert!((surf.v_last() - 8.0).abs() < 1e-14);
    let p = surf.value(0.0, 0.0);
    assert!((p.y - 2.0).abs() < 1e-12);
    let p2 = surf.value(0.0, 100.0);
    assert!((p2.y - 8.0).abs() < 1e-12);
}

#[test]
fn integration_sphere_unit_radius_equator() {
    let sphere = GeomSphericalSurface::new([0.0, 0.0, 0.0], 1.0);
    let surf = GeomRectangularTrimmedSurface::new(
        GeomSurfaceKind::Sphere(sphere),
        0.0, std::f64::consts::TAU,
        0.0, std::f64::consts::FRAC_PI_2,
        true, true,
    );
    let p = surf.value(std::f64::consts::FRAC_PI_2, 0.0);
    assert!(p.x.abs() < 1e-12);
    assert!((p.y - 1.0).abs() < 1e-12);
    assert!(p.z.abs() < 1e-12);
}

#[test]
fn integration_sphere_radius_2_point_on_surface() {
    let sphere = GeomSphericalSurface::new([1.0, 2.0, 3.0], 2.0);
    let surf = GeomRectangularTrimmedSurface::new(
        GeomSurfaceKind::Sphere(sphere),
        0.0, std::f64::consts::TAU,
        -std::f64::consts::FRAC_PI_2, std::f64::consts::FRAC_PI_2,
        true, true,
    );
    for ui in 0..8 {
        for vi in 0..5 {
            let u = ui as f64 * std::f64::consts::TAU / 8.0;
            let v = -std::f64::consts::FRAC_PI_2 + vi as f64 * std::f64::consts::PI / 4.0;
            let p = surf.value(u, v);
            let dx = p.x - 1.0;
            let dy = p.y - 2.0;
            let dz = p.z - 3.0;
            let dist = (dx*dx + dy*dy + dz*dz).sqrt();
            assert!((dist - 2.0).abs() < 1e-10, "dist={} at u={} v={}", dist, u, v);
        }
    }
}

#[test]
fn integration_cylinder_point_on_surface() {
    let cyl = GeomCylindricalSurface::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 3.0);
    let surf = GeomRectangularTrimmedSurface::new(
        GeomSurfaceKind::Cylinder(cyl),
        0.0, std::f64::consts::TAU,
        -10.0, 10.0,
        true, true,
    );
    for ui in 0..12 {
        for vi in 0..5 {
            let u = ui as f64 * std::f64::consts::TAU / 12.0;
            let v = -5.0 + vi as f64 * 2.5;
            let p = surf.value(u, v);
            let r = (p.x*p.x + p.y*p.y).sqrt();
            assert!((r - 3.0).abs() < 1e-10, "r={} at u={} v={}", r, u, v);
        }
    }
}

#[test]
fn integration_torus_major_minor() {
    let torus = GeomToroidalSurface::new([0.0, 0.0, 0.0], 5.0, 2.0);
    let surf = GeomRectangularTrimmedSurface::new(
        GeomSurfaceKind::Torus(torus),
        0.0, std::f64::consts::TAU,
        0.0, std::f64::consts::TAU,
        true, true,
    );
    let p = surf.value(0.0, 0.0);
    assert!((p.x - 7.0).abs() < 1e-12);
    assert!(p.y.abs() < 1e-12);
    assert!(p.z.abs() < 1e-12);

    let p2 = surf.value(0.0, std::f64::consts::PI);
    assert!((p2.x - 3.0).abs() < 1e-12);
    assert!(p2.y.abs() < 1e-12);
    assert!(p2.z.abs() < 1e-12);
}

#[test]
fn integration_cone_increasing_radius() {
    let half_angle = std::f64::consts::PI / 6.0;
    let cone = GeomConicalSurface::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 1.0, half_angle);
    let surf = GeomRectangularTrimmedSurface::new(
        GeomSurfaceKind::Cone(cone),
        0.0, std::f64::consts::TAU,
        0.0, 3.0,
        true, true,
    );
    let p0 = surf.value(0.0, 0.0);
    assert!((p0.x - 1.0).abs() < 1e-12);

    let expected_r = 1.0 + half_angle.tan();
    let p1 = surf.value(0.0, 1.0);
    assert!((p1.x - expected_r).abs() < 1e-10);
    assert!((p1.z - 1.0).abs() < 1e-12);
}

#[test]
fn integration_no_trim_passthrough() {
    let surf = GeomRectangularTrimmedSurface::new(
        GeomSurfaceKind::Plane(xy_plane()),
        -1.0, 1.0, -1.0, 1.0,
        false, false,
    );
    let p = surf.value(50.0, -30.0);
    assert!((p.x - 50.0).abs() < 1e-12);
    assert!((p.y - (-30.0)).abs() < 1e-12);
}

#[test]
fn integration_partial_trim_u_only() {
    let surf = GeomRectangularTrimmedSurface::new(
        GeomSurfaceKind::Plane(xy_plane()),
        -1.0, 1.0, -1.0, 1.0,
        true, false,
    );
    let p = surf.value(5.0, 5.0);
    assert!((p.x - 1.0).abs() < 1e-12);
    assert!((p.y - 5.0).abs() < 1e-12);
}

#[test]
fn integration_partial_trim_v_only() {
    let surf = GeomRectangularTrimmedSurface::new(
        GeomSurfaceKind::Plane(xy_plane()),
        -1.0, 1.0, -1.0, 1.0,
        false, true,
    );
    let p = surf.value(5.0, 5.0);
    assert!((p.x - 5.0).abs() < 1e-12);
    assert!((p.y - 1.0).abs() < 1e-12);
}
