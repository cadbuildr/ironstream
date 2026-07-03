// FILE: tests/occt_gp_cylinder.rs
extern crate ironstream;

use ironstream::gp_cylinder::GpCylinder;
use ironstream::gp::{Ax1, Ax3, Pnt, Vec3};
use std::f64::consts::{FRAC_PI_2, FRAC_PI_4, PI};

const EPS: f64 = 1e-10;

fn make_cylinder(cx: f64, cy: f64, cz: f64, radius: f64) -> GpCylinder {
    let origin = Pnt::new(cx, cy, cz);
    let z = Pnt::new(0.0, 0.0, 1.0);
    let x = Pnt::new(1.0, 0.0, 0.0);
    let ax3 = Ax3::from_origin_normal(origin, z, x);
    GpCylinder::new(ax3, radius)
}

#[test]
fn test_default_cylinder_properties() {
    let cyl = make_cylinder(0.0, 0.0, 0.0, 5.0);
    assert!((cyl.radius() - 5.0).abs() < EPS);
    let loc = cyl.location();
    assert!(loc.x.abs() < EPS);
    assert!(loc.y.abs() < EPS);
    assert!(loc.z.abs() < EPS);
}

#[test]
fn test_at_angle_surface_points() {
    let cyl = make_cylinder(0.0, 0.0, 0.0, 4.0);

    // angle = 0 -> (4, 0, 0)
    let l0 = cyl.at_angle(0.0);
    let p0 = l0.location();
    assert!((p0.x - 4.0).abs() < EPS);
    assert!(p0.y.abs() < EPS);

    // angle = pi/2 -> (0, 4, 0)
    let l90 = cyl.at_angle(FRAC_PI_2);
    let p90 = l90.location();
    assert!(p90.x.abs() < EPS);
    assert!((p90.y - 4.0).abs() < EPS);

    // angle = pi -> (-4, 0, 0)
    let l180 = cyl.at_angle(PI);
    let p180 = l180.location();
    assert!((p180.x + 4.0).abs() < EPS);
    assert!(p180.y.abs() < EPS);

    // angle = 3*pi/2 -> (0, -4, 0)
    let l270 = cyl.at_angle(3.0 * FRAC_PI_2);
    let p270 = l270.location();
    assert!(p270.x.abs() < EPS);
    assert!((p270.y + 4.0).abs() < EPS);
}

#[test]
fn test_at_angle_line_direction_is_z_axis() {
    let cyl = make_cylinder(1.0, 2.0, 3.0, 2.0);
    for &angle in &[0.0, FRAC_PI_4, FRAC_PI_2, PI, 2.0 * PI] {
        let line = cyl.at_angle(angle);
        let d = line.direction();
        // Direction must be parallel to Z axis
        assert!(d.x.abs() < EPS, "angle={} dx={}", angle, d.x);
        assert!(d.y.abs() < EPS, "angle={} dy={}", angle, d.y);
        assert!((d.z.abs() - 1.0).abs() < EPS, "angle={} dz={}", angle, d.z);
    }
}

#[test]
fn test_at_angle_offset_cylinder() {
    // Cylinder with centre at (1,1,0)
    let cyl = make_cylinder(1.0, 1.0, 0.0, 2.0);
    let l0 = cyl.at_angle(0.0);
    let p = l0.location();
    // (1 + 2*cos(0), 1 + 2*sin(0), 0) = (3, 1, 0)
    assert!((p.x - 3.0).abs() < EPS, "x={}", p.x);
    assert!((p.y - 1.0).abs() < EPS, "y={}", p.y);
}

#[test]
fn test_translate_preserves_radius() {
    let cyl = make_cylinder(0.0, 0.0, 0.0, 3.0);
    let moved = cyl.translate(&Vec3::new(10.0, -5.0, 2.0));
    assert!((moved.radius() - 3.0).abs() < EPS);
    assert!((moved.location().x - 10.0).abs() < EPS);
    assert!((moved.location().y + 5.0).abs() < EPS);
    assert!((moved.location().z - 2.0).abs() < EPS);
}

#[test]
fn test_scale_from_origin() {
    let cyl = make_cylinder(0.0, 0.0, 0.0, 1.0);
    let p = Pnt::new(0.0, 0.0, 0.0);
    let scaled = cyl.scale(&p, 3.0);
    assert!((scaled.radius() - 3.0).abs() < EPS);
    assert!(scaled.location().x.abs() < EPS);
}

#[test]
fn test_scale_off_origin() {
    // Cylinder at (2,0,0), radius 1.  Scale by 2 from origin.
    let cyl = make_cylinder(2.0, 0.0, 0.0, 1.0);
    let p = Pnt::new(0.0, 0.0, 0.0);
    let scaled = cyl.scale(&p, 2.0);
    // new centre = 2 * (2,0,0) = (4,0,0)
    assert!((scaled.location().x - 4.0).abs() < EPS);
    assert!((scaled.radius() - 2.0).abs() < EPS);
}

#[test]
fn test_scale_negative_factor() {
    let cyl = make_cylinder(0.0, 0.0, 0.0, 5.0);
    let p = Pnt::new(0.0, 0.0, 0.0);
    let scaled = cyl.scale(&p, -1.0);
    // Radius must remain positive
    assert!((scaled.radius() - 5.0).abs() < EPS);
}

#[test]
fn test_rotate_around_z() {
    let cyl = make_cylinder(0.0, 0.0, 0.0, 2.0);
    let z_ax = Ax1::new(Pnt::new(0.0, 0.0, 0.0), Pnt::new(0.0, 0.0, 1.0));
    let rotated = cyl.rotate(&z_ax, FRAC_PI_2);
    // Location stays at origin
    assert!(rotated.location().x.abs() < EPS);
    assert!(rotated.location().y.abs() < EPS);
    // After 90° rotation, x_axis of frame = old y direction = (0,1,0)
    let xax = rotated.x_axis();
    assert!(xax.direction.x.abs() < EPS, "dx={}", xax.direction.x);
    assert!((xax.direction.y - 1.0).abs() < EPS, "dy={}", xax.direction.y);
}

#[test]
fn test_rotate_cylinder_offset_around_z() {
    // Cylinder at (1,0,0). Rotate 90° around Z axis => centre moves to (0,1,0)
    let cyl = make_cylinder(1.0, 0.0, 0.0, 1.0);
    let z_ax = Ax1::new(Pnt::new(0.0, 0.0, 0.0), Pnt::new(0.0, 0.0, 1.0));
    let rotated = cyl.rotate(&z_ax, FRAC_PI_2);
    assert!(rotated.location().x.abs() < EPS, "x={}", rotated.location().x);
    assert!((rotated.location().y - 1.0).abs() < EPS, "y={}", rotated.location().y);
    assert!((rotated.radius() - 1.0).abs() < EPS);
}

#[test]
fn test_mirror_point_centre() {
    // Mirror cylinder at (1,0,0) through point (3,0,0) => cylinder at (5,0,0)
    let cyl = make_cylinder(1.0, 0.0, 0.0, 2.0);
    let mp = Pnt::new(3.0, 0.0, 0.0);
    let mirrored = cyl.mirror_point(&mp);
    assert!((mirrored.location().x - 5.0).abs() < EPS, "x={}", mirrored.location().x);
    assert!((mirrored.radius() - 2.0).abs() < EPS);
}

#[test]
fn test_vaxis_is_z_direction() {
    let cyl = make_cylinder(0.0, 0.0, 0.0, 1.0);
    let vax = cyl.vaxis();
    let ax = cyl.axis();
    assert!((vax.direction.x - ax.direction.x).abs() < EPS);
    assert!((vax.direction.y - ax.direction.y).abs() < EPS);
    assert!((vax.direction.z - ax.direction.z).abs() < EPS);
}

#[test]
fn test_translate_points() {
    let cyl = make_cylinder(0.0, 0.0, 0.0, 3.0);
    let p1 = Pnt::new(0.0, 0.0, 0.0);
    let p2 = Pnt::new(3.0, 4.0, 5.0);
    let moved = cyl.translate_points(&p1, &p2);
    assert!((moved.location().x - 3.0).abs() < EPS);
    assert!((moved.location().y - 4.0).abs() < EPS);
    assert!((moved.location().z - 5.0).abs() < EPS);
    assert!((moved.radius() - 3.0).abs() < EPS);
}

#[test]
fn test_clone_and_set_radius() {
    let mut cyl = make_cylinder(0.0, 0.0, 0.0, 5.0);
    let original_radius = cyl.radius();
    cyl.set_radius(10.0);
    assert!((original_radius - 5.0).abs() < EPS);
    assert!((cyl.radius() - 10.0).abs() < EPS);
}

#[test]
fn test_at_angle_point_distance_equals_radius() {
    let radius = 7.0;
    let cyl = make_cylinder(0.0, 0.0, 0.0, radius);
    for i in 0..16 {
        let angle = (i as f64) * PI / 8.0;
        let line = cyl.at_angle(angle);
        let p = line.location();
        let dist = (p.x * p.x + p.y * p.y).sqrt();
        assert!((dist - radius).abs() < EPS, "angle={} dist={}", angle, dist);
    }
}

#[test]
fn test_tilted_cylinder_at_angle() {
    // Cylinder with axis tilted: Z direction = (0,1,0), X direction = (1,0,0)
    // Y direction = Z x X = (0,0,-1) or computed from Ax3 right-hand rule
    let origin = Pnt::new(0.0, 0.0, 0.0);
    // Z axis = (0,1,0), X axis = (1,0,0)
    let z = Pnt::new(0.0, 1.0, 0.0);
    let x = Pnt::new(1.0, 0.0, 0.0);
    let ax3 = Ax3::from_origin_normal(origin, z, x);
    let cyl = GpCylinder::new(ax3, 2.0);

    let line = cyl.at_angle(0.0);
    // point should be at (2,0,0) — radius along X direction
    let p = line.location();
    assert!((p.x - 2.0).abs() < EPS, "x={}", p.x);
    assert!(p.y.abs() < EPS, "y={}", p.y);

    // Line direction should be the Z axis of placement = (0,1,0)
    let d = line.direction();
    assert!(d.x.abs() < EPS);
    assert!((d.y - 1.0).abs() < EPS);
    assert!(d.z.abs() < EPS);
}
