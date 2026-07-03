// FILE: tests/occt_gp_sph.rs
extern crate ironstream;

use ironstream::gp_sph::GpSphere;
use ironstream::gp::{Ax1, Ax3, Pnt, Vec3};
use std::f64::consts::{FRAC_PI_2, PI};

const EPS: f64 = 1e-10;

fn approx_eq(a: f64, b: f64) -> bool {
    (a - b).abs() < EPS
}

fn pnt_eq(a: Pnt, b: Pnt) -> bool {
    approx_eq(a.x, b.x) && approx_eq(a.y, b.y) && approx_eq(a.z, b.z)
}

// ---------------------------------------------------------------------------
// Construction and default
// ---------------------------------------------------------------------------

#[test]
fn integration_default_center_at_origin() {
    let s = GpSphere::default();
    assert!(pnt_eq(s.location(), Pnt::new(0.0, 0.0, 0.0)));
}

#[test]
fn integration_default_radius_one() {
    let s = GpSphere::default();
    assert!(approx_eq(s.radius(), 1.0));
}

#[test]
fn integration_custom_center_and_radius() {
    let center = Pnt::new(3.0, -1.0, 2.0);
    let ax3 = Ax3::from_origin_normal(center, Pnt::new(0.0, 0.0, 1.0), Pnt::new(1.0, 0.0, 0.0));
    let s = GpSphere::new(ax3, 7.0);
    assert!(pnt_eq(s.location(), center));
    assert!(approx_eq(s.radius(), 7.0));
}

// ---------------------------------------------------------------------------
// Geometry: area and volume
// ---------------------------------------------------------------------------

#[test]
fn integration_area_unit_sphere() {
    let s = GpSphere::default();
    assert!(approx_eq(s.area(), 4.0 * PI));
}

#[test]
fn integration_volume_unit_sphere() {
    let s = GpSphere::default();
    assert!(approx_eq(s.volume(), (4.0 / 3.0) * PI));
}

#[test]
fn integration_area_radius_5() {
    let s = GpSphere::new(Ax3::identity(), 5.0);
    assert!(approx_eq(s.area(), 4.0 * PI * 25.0));
}

#[test]
fn integration_volume_radius_2() {
    let s = GpSphere::new(Ax3::identity(), 2.0);
    let expected = (4.0 / 3.0) * PI * 8.0;
    assert!(approx_eq(s.volume(), expected));
}

// ---------------------------------------------------------------------------
// Equator and iso-circles
// ---------------------------------------------------------------------------

#[test]
fn integration_equator_radius_matches_sphere() {
    let s = GpSphere::new(Ax3::identity(), 4.0);
    assert!(approx_eq(s.equator().radius(), 4.0));
}

#[test]
fn integration_equator_center_at_sphere_center() {
    let center = Pnt::new(1.0, 2.0, 3.0);
    let ax3 = Ax3::from_origin_normal(center, Pnt::new(0.0, 0.0, 1.0), Pnt::new(1.0, 0.0, 0.0));
    let s = GpSphere::new(ax3, 3.0);
    assert!(pnt_eq(s.equator().location(), center));
}

#[test]
fn integration_circle_at_v_north_pole_zero_radius() {
    let s = GpSphere::new(Ax3::identity(), 5.0);
    let c = s.circle_at_v(FRAC_PI_2);
    assert!(c.radius() < EPS);
}

#[test]
fn integration_circle_at_v_south_pole_zero_radius() {
    let s = GpSphere::new(Ax3::identity(), 5.0);
    let c = s.circle_at_v(-FRAC_PI_2);
    assert!(c.radius() < EPS);
}

#[test]
fn integration_circle_at_v_equator_equals_equator_radius() {
    let r = 6.0;
    let s = GpSphere::new(Ax3::identity(), r);
    let c = s.circle_at_v(0.0);
    assert!(approx_eq(c.radius(), r));
}

#[test]
fn integration_circle_at_v_center_offset_along_z() {
    let r = 4.0_f64;
    let v = PI / 4.0; // 45 degrees
    let s = GpSphere::new(Ax3::identity(), r);
    let c = s.circle_at_v(v);
    let expected_z = r * v.sin();
    assert!(approx_eq(c.location().z, expected_z));
}

#[test]
fn integration_circle_at_u_is_great_circle_radius() {
    let r = 3.0;
    let s = GpSphere::new(Ax3::identity(), r);
    for &u in &[0.0, 0.5, PI, 1.8 * PI] {
        let c = s.circle_at_u(u);
        assert!(approx_eq(c.radius(), r), "u={u} radius={}", c.radius());
    }
}

// ---------------------------------------------------------------------------
// Value / parametric evaluation
// ---------------------------------------------------------------------------

#[test]
fn integration_value_north_pole() {
    let s = GpSphere::default();
    let p = s.value(0.0, FRAC_PI_2);
    assert!(pnt_eq(p, Pnt::new(0.0, 0.0, 1.0)));
}

#[test]
fn integration_value_south_pole() {
    let s = GpSphere::default();
    let p = s.value(0.0, -FRAC_PI_2);
    assert!(pnt_eq(p, Pnt::new(0.0, 0.0, -1.0)));
}

#[test]
fn integration_value_equator_x() {
    let s = GpSphere::default();
    let p = s.value(0.0, 0.0);
    assert!(pnt_eq(p, Pnt::new(1.0, 0.0, 0.0)));
}

#[test]
fn integration_value_equator_y() {
    let s = GpSphere::default();
    let p = s.value(FRAC_PI_2, 0.0);
    assert!(approx_eq(p.x, 0.0));
    assert!(approx_eq(p.y, 1.0));
    assert!(approx_eq(p.z, 0.0));
}

#[test]
fn integration_value_always_at_radius_distance() {
    let r = 8.5;
    let s = GpSphere::new(Ax3::identity(), r);
    for &u in &[0.0, 1.0, 2.5, 4.0] {
        for &v in &[-FRAC_PI_2, -0.3, 0.0, 0.6, FRAC_PI_2] {
            let p = s.value(u, v);
            let dist = (p - s.location()).norm();
            assert!((dist - r).abs() < EPS, "u={u} v={v} dist={dist}");
        }
    }
}

// ---------------------------------------------------------------------------
// contains
// ---------------------------------------------------------------------------

#[test]
fn integration_contains_surface_point() {
    let s = GpSphere::new(Ax3::identity(), 3.0);
    let p = s.value(0.8, 0.4);
    assert!(s.contains(p, 1e-9));
}

#[test]
fn integration_contains_outside_point_false() {
    let s = GpSphere::default();
    assert!(!s.contains(Pnt::new(2.0, 0.0, 0.0), 1e-9));
}

#[test]
fn integration_contains_center_false() {
    let s = GpSphere::default();
    assert!(!s.contains(Pnt::new(0.0, 0.0, 0.0), 1e-9));
}

// ---------------------------------------------------------------------------
// Setters
// ---------------------------------------------------------------------------

#[test]
fn integration_set_radius_updates_area() {
    let mut s = GpSphere::default();
    s.set_radius(3.0);
    assert!(approx_eq(s.area(), 4.0 * PI * 9.0));
}

#[test]
fn integration_set_location_updates_center() {
    let mut s = GpSphere::default();
    let new_loc = Pnt::new(10.0, 20.0, 30.0);
    s.set_location(new_loc);
    assert!(pnt_eq(s.location(), new_loc));
}

// ---------------------------------------------------------------------------
// Translation
// ---------------------------------------------------------------------------

#[test]
fn integration_translate_moves_center() {
    let mut s = GpSphere::default();
    s.translate(&Vec3::new(3.0, 4.0, 5.0));
    assert!(pnt_eq(s.location(), Pnt::new(3.0, 4.0, 5.0)));
    assert!(approx_eq(s.radius(), 1.0));
}

#[test]
fn integration_translated_immutable() {
    let s = GpSphere::default();
    let s2 = s.translated(&Vec3::new(10.0, 0.0, 0.0));
    assert!(pnt_eq(s.location(), Pnt::new(0.0, 0.0, 0.0)));
    assert!(pnt_eq(s2.location(), Pnt::new(10.0, 0.0, 0.0)));
}

#[test]
fn integration_translated_points() {
    let s = GpSphere::default();
    let p1 = Pnt::new(0.0, 0.0, 0.0);
    let p2 = Pnt::new(1.0, 2.0, 3.0);
    let s2 = s.translated_points(&p1, &p2);
    assert!(pnt_eq(s2.location(), p2));
}

// ---------------------------------------------------------------------------
// Scaling
// ---------------------------------------------------------------------------

#[test]
fn integration_scale_about_origin_radius() {
    let s = GpSphere::new(Ax3::identity(), 2.0);
    let s2 = s.scaled(&Pnt::new(0.0, 0.0, 0.0), 3.0);
    assert!(approx_eq(s2.radius(), 6.0));
    assert!(pnt_eq(s2.location(), Pnt::new(0.0, 0.0, 0.0)));
}

#[test]
fn integration_scale_negative_factor_radius_absolute() {
    let s = GpSphere::default();
    let s2 = s.scaled(&Pnt::new(0.0, 0.0, 0.0), -4.0);
    assert!(approx_eq(s2.radius(), 4.0));
}

#[test]
fn integration_scale_off_center_moves_location() {
    let ax3 = Ax3::from_origin_normal(
        Pnt::new(2.0, 0.0, 0.0),
        Pnt::new(0.0, 0.0, 1.0),
        Pnt::new(1.0, 0.0, 0.0),
    );
    let s = GpSphere::new(ax3, 1.0);
    let origin = Pnt::new(0.0, 0.0, 0.0);
    let s2 = s.scaled(&origin, 2.0);
    assert!(pnt_eq(s2.location(), Pnt::new(4.0, 0.0, 0.0)));
    assert!(approx_eq(s2.radius(), 2.0));
}

// ---------------------------------------------------------------------------
// Rotation
// ---------------------------------------------------------------------------

#[test]
fn integration_rotate_center_about_z() {
    let ax3 = Ax3::from_origin_normal(
        Pnt::new(1.0, 0.0, 0.0),
        Pnt::new(0.0, 0.0, 1.0),
        Pnt::new(1.0, 0.0, 0.0),
    );
    let s = GpSphere::new(ax3, 2.0);
    let z_ax1 = Ax1::new(Pnt::new(0.0, 0.0, 0.0), Pnt::new(0.0, 0.0, 1.0));
    let s2 = s.rotated(&z_ax1, FRAC_PI_2);
    assert!(pnt_eq(s2.location(), Pnt::new(0.0, 1.0, 0.0)));
    assert!(approx_eq(s2.radius(), 2.0));
}

#[test]
fn integration_rotate_full_circle_returns_to_start() {
    let ax3 = Ax3::from_origin_normal(
        Pnt::new(3.0, 0.0, 0.0),
        Pnt::new(0.0, 0.0, 1.0),
        Pnt::new(1.0, 0.0, 0.0),
    );
    let s = GpSphere::new(ax3, 5.0);
    let z_ax1 = Ax1::new(Pnt::new(0.0, 0.0, 0.0), Pnt::new(0.0, 0.0, 1.0));
    let s2 = s.rotated(&z_ax1, 2.0 * PI);
    assert!(pnt_eq(s2.location(), s.location()));
}

#[test]
fn integration_rotate_in_place() {
    let ax3 = Ax3::from_origin_normal(
        Pnt::new(1.0, 0.0, 0.0),
        Pnt::new(0.0, 0.0, 1.0),
        Pnt::new(1.0, 0.0, 0.0),
    );
    let mut s = GpSphere::new(ax3, 1.0);
    let z_ax1 = Ax1::new(Pnt::new(0.0, 0.0, 0.0), Pnt::new(0.0, 0.0, 1.0));
    s.rotate(&z_ax1, FRAC_PI_2);
    assert!(pnt_eq(s.location(), Pnt::new(0.0, 1.0, 0.0)));
}

// ---------------------------------------------------------------------------
// Mirror through point
// ---------------------------------------------------------------------------

#[test]
fn integration_mirror_through_point() {
    let s = GpSphere::default();
    let pt = Pnt::new(3.0, 0.0, 0.0);
    let s2 = s.mirrored_through_point(&pt);
    assert!(pnt_eq(s2.location(), Pnt::new(6.0, 0.0, 0.0)));
    assert!(approx_eq(s2.radius(), 1.0));
}

#[test]
fn integration_mirror_through_point_twice_is_identity() {
    let ax3 = Ax3::from_origin_normal(
        Pnt::new(2.0, 3.0, 1.0),
        Pnt::new(0.0, 0.0, 1.0),
        Pnt::new(1.0, 0.0, 0.0),
    );
    let s = GpSphere::new(ax3, 4.0);
    let pt = Pnt::new(1.0, 1.0, 1.0);
    let s2 = s.mirrored_through_point(&pt).mirrored_through_point(&pt);
    assert!(pnt_eq(s2.location(), s.location()));
    assert!(approx_eq(s2.radius(), s.radius()));
}

// ---------------------------------------------------------------------------
// Mirror through Ax1 (line)
// ---------------------------------------------------------------------------

#[test]
fn integration_mirror_through_ax1_flips_along_x() {
    let ax3 = Ax3::from_origin_normal(
        Pnt::new(1.0, 0.0, 0.0),
        Pnt::new(0.0, 0.0, 1.0),
        Pnt::new(1.0, 0.0, 0.0),
    );
    let s = GpSphere::new(ax3, 2.0);
    let y_ax1 = Ax1::new(Pnt::new(0.0, 0.0, 0.0), Pnt::new(0.0, 1.0, 0.0));
    let s2 = s.mirrored_through_ax1(&y_ax1);
    assert!(pnt_eq(s2.location(), Pnt::new(-1.0, 0.0, 0.0)));
    assert!(approx_eq(s2.radius(), 2.0));
}

// ---------------------------------------------------------------------------
// Mirror through Ax2 (plane)
// ---------------------------------------------------------------------------

#[test]
fn integration_mirror_through_xy_plane() {
    let ax3 = Ax3::from_origin_normal(
        Pnt::new(0.0, 0.0, 5.0),
        Pnt::new(0.0, 0.0, 1.0),
        Pnt::new(1.0, 0.0, 0.0),
    );
    let s = GpSphere::new(ax3, 1.5);
    let xy_plane = Ax3::from_origin_normal(
        Pnt::new(0.0, 0.0, 0.0),
        Pnt::new(0.0, 0.0, 1.0),
        Pnt::new(1.0, 0.0, 0.0),
    );
    let s2 = s.mirrored_through_ax2(&xy_plane);
    assert!(pnt_eq(s2.location(), Pnt::new(0.0, 0.0, -5.0)));
    assert!(approx_eq(s2.radius(), 1.5));
}

// ---------------------------------------------------------------------------
// Axis accessors
// ---------------------------------------------------------------------------

#[test]
fn integration_axis_direction_is_z() {
    let s = GpSphere::default();
    let ax = s.axis();
    assert!(approx_eq(ax.direction.x, 0.0));
    assert!(approx_eq(ax.direction.y, 0.0));
    assert!(approx_eq(ax.direction.z, 1.0));
}

#[test]
fn integration_x_axis_direction() {
    let s = GpSphere::default();
    let xax = s.x_axis();
    assert!(approx_eq(xax.direction.x, 1.0));
    assert!(approx_eq(xax.direction.y, 0.0));
    assert!(approx_eq(xax.direction.z, 0.0));
}

#[test]
fn integration_y_axis_direction() {
    let s = GpSphere::default();
    let yax = s.y_axis();
    assert!(approx_eq(yax.direction.x, 0.0));
    assert!(approx_eq(yax.direction.y, 1.0));
    assert!(approx_eq(yax.direction.z, 0.0));
}

// ---------------------------------------------------------------------------
// is_direct
// ---------------------------------------------------------------------------

#[test]
fn integration_is_direct_standard_frame() {
    let s = GpSphere::default();
    assert!(s.is_direct());
}

// ---------------------------------------------------------------------------
// Edge cases / panics
// ---------------------------------------------------------------------------

#[test]
#[should_panic]
fn integration_zero_radius_panics() {
    let _ = GpSphere::new(Ax3::identity(), 0.0);
}

#[test]
#[should_panic]
fn integration_negative_radius_panics() {
    let _ = GpSphere::new(Ax3::identity(), -3.0);
}

#[test]
#[should_panic]
fn integration_set_radius_zero_panics() {
    let mut s = GpSphere::default();
    s.set_radius(0.0);
}

// ---------------------------------------------------------------------------
// Composed transforms
// ---------------------------------------------------------------------------

#[test]
fn integration_translate_then_rotate() {
    // Translate sphere center to (1,0,0), then rotate PI/2 about Z.
    let s = GpSphere::default();
    let s1 = s.translated(&Vec3::new(1.0, 0.0, 0.0));
    let z_ax1 = Ax1::new(Pnt::new(0.0, 0.0, 0.0), Pnt::new(0.0, 0.0, 1.0));
    let s2 = s1.rotated(&z_ax1, FRAC_PI_2);
    // (1,0,0) rotated PI/2 about Z → (0,1,0)
    assert!(pnt_eq(s2.location(), Pnt::new(0.0, 1.0, 0.0)));
    assert!(approx_eq(s2.radius(), 1.0));
}
