// FILE: tests/occt_gp_cone.rs
extern crate ironstream;
use ironstream::gp_cone::*;
use ironstream::gp::{Pnt, Vec3, Ax1, Ax3};
use std::f64::consts::{PI, FRAC_PI_4, FRAC_PI_2};

const EPS: f64 = 1e-10;

fn approx_eq(a: f64, b: f64, eps: f64) -> bool {
    (a - b).abs() < eps
}

fn pnt_approx_eq(a: &Pnt, b: &Pnt, eps: f64) -> bool {
    approx_eq(a.x, b.x, eps) && approx_eq(a.y, b.y, eps) && approx_eq(a.z, b.z, eps)
}

// ---------------------------------------------------------------------------
// Construction and basic accessors
// ---------------------------------------------------------------------------

#[test]
fn integration_default_cone_apex_at_origin() {
    let cone = GpCone::default();
    assert!(pnt_approx_eq(&cone.apex(), &Pnt::new(0.0, 0.0, 0.0), EPS));
}

#[test]
fn integration_default_cone_semi_angle_is_pi_over_4() {
    let cone = GpCone::default();
    assert!(approx_eq(cone.semi_angle(), FRAC_PI_4, EPS));
}

#[test]
fn integration_default_cone_is_direct() {
    let cone = GpCone::default();
    assert!(cone.is_direct());
}

#[test]
fn integration_cone_location_equals_apex() {
    let cone = GpCone::default();
    assert!(pnt_approx_eq(&cone.apex(), &cone.location(), EPS));
}

// ---------------------------------------------------------------------------
// Circle sections (at_angle)
// ---------------------------------------------------------------------------

#[test]
fn integration_at_angle_zero_gives_zero_radius() {
    let cone = GpCone::default();
    let circ = cone.at_angle(0.0);
    assert!(approx_eq(circ.radius(), 0.0, EPS));
}

#[test]
fn integration_at_angle_unit_distance_radius() {
    // half_angle=PI/4 => tan=1, so radius at u=1 is 1.
    let cone = GpCone::default();
    let circ = cone.at_angle(1.0);
    assert!(approx_eq(circ.radius(), 1.0, EPS));
}

#[test]
fn integration_at_angle_negative_same_radius() {
    let cone = GpCone::default();
    assert!(approx_eq(cone.at_angle(3.0).radius(), cone.at_angle(-3.0).radius(), EPS));
}

#[test]
fn integration_at_angle_center_along_z() {
    let cone = GpCone::default();
    let circ = cone.at_angle(7.0);
    let center = circ.location();
    assert!(pnt_approx_eq(&center, &Pnt::new(0.0, 0.0, 7.0), EPS));
}

#[test]
fn integration_custom_half_angle_radius() {
    let angle = 0.3_f64;
    let ax3 = Ax3::from_origin_normal(
        Pnt::new(0.0, 0.0, 0.0),
        Pnt::new(0.0, 0.0, 1.0),
        Pnt::new(1.0, 0.0, 0.0),
    );
    let cone = GpCone::new(ax3, angle);
    let r = cone.radius_at(10.0);
    assert!(approx_eq(r, 10.0 * angle.tan(), EPS));
}

#[test]
fn integration_radius_at_zero() {
    let cone = GpCone::default();
    assert!(approx_eq(cone.radius_at(0.0), 0.0, EPS));
}

// ---------------------------------------------------------------------------
// Translation
// ---------------------------------------------------------------------------

#[test]
fn integration_translate_moves_apex() {
    let mut cone = GpCone::default();
    cone.translate(&Vec3::new(1.0, 2.0, 3.0));
    assert!(pnt_approx_eq(&cone.apex(), &Pnt::new(1.0, 2.0, 3.0), EPS));
}

#[test]
fn integration_translated_immutable() {
    let cone = GpCone::default();
    let cone2 = cone.translated(&Vec3::new(5.0, 0.0, 0.0));
    assert!(pnt_approx_eq(&cone.apex(), &Pnt::new(0.0, 0.0, 0.0), EPS));
    assert!(pnt_approx_eq(&cone2.apex(), &Pnt::new(5.0, 0.0, 0.0), EPS));
}

#[test]
fn integration_translate_from_to() {
    let mut cone = GpCone::default();
    let p1 = Pnt::new(0.0, 0.0, 0.0);
    let p2 = Pnt::new(3.0, 4.0, 5.0);
    cone.translate_from_to(&p1, &p2);
    assert!(pnt_approx_eq(&cone.apex(), &p2, EPS));
}

#[test]
fn integration_translate_preserves_semi_angle() {
    let mut cone = GpCone::default();
    cone.translate(&Vec3::new(100.0, 200.0, 300.0));
    assert!(approx_eq(cone.semi_angle(), FRAC_PI_4, EPS));
}

// ---------------------------------------------------------------------------
// Scaling
// ---------------------------------------------------------------------------

#[test]
fn integration_scale_about_origin_preserves_apex_at_origin() {
    let mut cone = GpCone::default();
    let origin = Pnt::new(0.0, 0.0, 0.0);
    cone.scale(&origin, 5.0);
    assert!(pnt_approx_eq(&cone.apex(), &origin, EPS));
}

#[test]
fn integration_scale_moves_off_center_apex() {
    let ax3 = Ax3::from_origin_normal(
        Pnt::new(2.0, 0.0, 0.0),
        Pnt::new(0.0, 0.0, 1.0),
        Pnt::new(1.0, 0.0, 0.0),
    );
    let cone = GpCone::new(ax3, FRAC_PI_4);
    let origin = Pnt::new(0.0, 0.0, 0.0);
    let cone2 = cone.scaled(&origin, 3.0);
    // apex (2,0,0) scaled by 3 about (0,0,0) = (6,0,0)
    assert!(pnt_approx_eq(&cone2.apex(), &Pnt::new(6.0, 0.0, 0.0), EPS));
    assert!(approx_eq(cone2.semi_angle(), FRAC_PI_4, EPS));
}

#[test]
fn integration_scale_preserves_half_angle() {
    let cone = GpCone::default();
    let pt = Pnt::new(1.0, 1.0, 1.0);
    let cone2 = cone.scaled(&pt, 10.0);
    assert!(approx_eq(cone2.semi_angle(), FRAC_PI_4, EPS));
}

// ---------------------------------------------------------------------------
// Rotation
// ---------------------------------------------------------------------------

#[test]
fn integration_rotate_apex_about_z() {
    let ax3 = Ax3::from_origin_normal(
        Pnt::new(1.0, 0.0, 0.0),
        Pnt::new(0.0, 0.0, 1.0),
        Pnt::new(1.0, 0.0, 0.0),
    );
    let cone = GpCone::new(ax3, FRAC_PI_4);
    let z_ax1 = Ax1::new(Pnt::new(0.0, 0.0, 0.0), Pnt::new(0.0, 0.0, 1.0));
    let cone2 = cone.rotated(&z_ax1, FRAC_PI_2);
    // (1,0,0) rotated PI/2 about Z = (0,1,0)
    assert!(pnt_approx_eq(&cone2.apex(), &Pnt::new(0.0, 1.0, 0.0), EPS));
    assert!(approx_eq(cone2.semi_angle(), FRAC_PI_4, EPS));
}

#[test]
fn integration_rotate_full_circle_returns_to_start() {
    let ax3 = Ax3::from_origin_normal(
        Pnt::new(3.0, 0.0, 0.0),
        Pnt::new(0.0, 0.0, 1.0),
        Pnt::new(1.0, 0.0, 0.0),
    );
    let cone = GpCone::new(ax3, 0.4);
    let z_ax1 = Ax1::new(Pnt::new(0.0, 0.0, 0.0), Pnt::new(0.0, 0.0, 1.0));
    let cone2 = cone.rotated(&z_ax1, 2.0 * PI);
    assert!(pnt_approx_eq(&cone2.apex(), &cone.apex(), EPS));
}

#[test]
fn integration_rotate_in_place() {
    let ax3 = Ax3::from_origin_normal(
        Pnt::new(1.0, 0.0, 0.0),
        Pnt::new(0.0, 0.0, 1.0),
        Pnt::new(1.0, 0.0, 0.0),
    );
    let mut cone = GpCone::new(ax3, FRAC_PI_4);
    let z_ax1 = Ax1::new(Pnt::new(0.0, 0.0, 0.0), Pnt::new(0.0, 0.0, 1.0));
    cone.rotate(&z_ax1, FRAC_PI_2);
    assert!(pnt_approx_eq(&cone.apex(), &Pnt::new(0.0, 1.0, 0.0), EPS));
}

// ---------------------------------------------------------------------------
// Mirror through point
// ---------------------------------------------------------------------------

#[test]
fn integration_mirror_through_point_apex() {
    let cone = GpCone::default(); // apex at (0,0,0)
    let pt = Pnt::new(2.0, 0.0, 0.0);
    let cone2 = cone.mirrored_through_point(&pt);
    // 2*(2,0,0) - (0,0,0) = (4,0,0)
    assert!(pnt_approx_eq(&cone2.apex(), &Pnt::new(4.0, 0.0, 0.0), EPS));
    assert!(approx_eq(cone2.semi_angle(), FRAC_PI_4, EPS));
}

#[test]
fn integration_mirror_through_point_in_place() {
    let mut cone = GpCone::default();
    let pt = Pnt::new(1.0, 0.0, 0.0);
    cone.mirror_through_point(&pt);
    assert!(pnt_approx_eq(&cone.apex(), &Pnt::new(2.0, 0.0, 0.0), EPS));
}

#[test]
fn integration_mirror_through_point_twice_returns_original() {
    let cone = GpCone::default();
    let pt = Pnt::new(3.0, 4.0, 5.0);
    let cone2 = cone.mirrored_through_point(&pt).mirrored_through_point(&pt);
    assert!(pnt_approx_eq(&cone2.apex(), &cone.apex(), EPS));
}

// ---------------------------------------------------------------------------
// Mirror through Ax1 (line)
// ---------------------------------------------------------------------------

#[test]
fn integration_mirror_through_ax1_flips_apex() {
    let ax3 = Ax3::from_origin_normal(
        Pnt::new(1.0, 0.0, 0.0),
        Pnt::new(0.0, 0.0, 1.0),
        Pnt::new(1.0, 0.0, 0.0),
    );
    let cone = GpCone::new(ax3, FRAC_PI_4);
    // Mirror through Y-axis (origin, Y direction)
    let y_ax1 = Ax1::new(Pnt::new(0.0, 0.0, 0.0), Pnt::new(0.0, 1.0, 0.0));
    let cone2 = cone.mirrored_through_ax1(&y_ax1);
    // (1,0,0) mirrored through Y-axis → (-1,0,0)
    assert!(pnt_approx_eq(&cone2.apex(), &Pnt::new(-1.0, 0.0, 0.0), EPS));
}

#[test]
fn integration_mirror_through_ax1_preserves_semi_angle() {
    let cone = GpCone::default();
    let ax1 = Ax1::new(Pnt::new(0.0, 0.0, 0.0), Pnt::new(0.0, 1.0, 0.0));
    let cone2 = cone.mirrored_through_ax1(&ax1);
    assert!(approx_eq(cone2.semi_angle(), FRAC_PI_4, EPS));
}

// ---------------------------------------------------------------------------
// Mirror through Ax2 (plane)
// ---------------------------------------------------------------------------

#[test]
fn integration_mirror_through_ax2_plane() {
    let ax3 = Ax3::from_origin_normal(
        Pnt::new(0.0, 0.0, 1.0),
        Pnt::new(0.0, 0.0, 1.0),
        Pnt::new(1.0, 0.0, 0.0),
    );
    let cone = GpCone::new(ax3, FRAC_PI_4);
    // Mirror through XY-plane (plane at origin, normal = Z)
    let xy_plane = Ax3::from_origin_normal(
        Pnt::new(0.0, 0.0, 0.0),
        Pnt::new(0.0, 0.0, 1.0),
        Pnt::new(1.0, 0.0, 0.0),
    );
    let cone2 = cone.mirrored_through_ax2(&xy_plane);
    // Apex (0,0,1) reflected through z=0 plane gives (0,0,-1)
    assert!(pnt_approx_eq(&cone2.apex(), &Pnt::new(0.0, 0.0, -1.0), EPS));
    assert!(approx_eq(cone2.semi_angle(), FRAC_PI_4, EPS));
}

// ---------------------------------------------------------------------------
// Setters
// ---------------------------------------------------------------------------

#[test]
fn integration_set_location_updates_apex() {
    let mut cone = GpCone::default();
    let new_loc = Pnt::new(10.0, 20.0, 30.0);
    cone.set_location(new_loc.clone());
    assert!(pnt_approx_eq(&cone.apex(), &new_loc, EPS));
}

#[test]
fn integration_set_semi_angle_updates_angle() {
    let mut cone = GpCone::default();
    cone.set_semi_angle(0.25);
    assert!(approx_eq(cone.semi_angle(), 0.25, EPS));
}

#[test]
fn integration_set_axis_updates_direction() {
    let mut cone = GpCone::default();
    let new_ax3 = Ax3::from_origin_normal(
        Pnt::new(1.0, 2.0, 3.0),
        Pnt::new(0.0, 1.0, 0.0),
        Pnt::new(1.0, 0.0, 0.0),
    );
    cone.set_axis(new_ax3.clone());
    assert!(pnt_approx_eq(&cone.apex(), &Pnt::new(1.0, 2.0, 3.0), EPS));
}

// ---------------------------------------------------------------------------
// Derived primitive
// ---------------------------------------------------------------------------

#[test]
fn integration_to_gp_con_half_angle() {
    let cone = GpCone::default();
    let gp = cone.to_gp_con();
    assert!(approx_eq(gp.semi_angle(), FRAC_PI_4, EPS));
}

// ---------------------------------------------------------------------------
// Edge cases
// ---------------------------------------------------------------------------

#[test]
#[should_panic]
fn integration_half_angle_zero_panics() {
    let _ = GpCone::new(Ax3::identity(), 0.0);
}

#[test]
#[should_panic]
fn integration_half_angle_pi_over_2_panics() {
    let _ = GpCone::new(Ax3::identity(), FRAC_PI_2);
}

#[test]
#[should_panic]
fn integration_half_angle_negative_panics() {
    let _ = GpCone::new(Ax3::identity(), -0.1);
}

#[test]
fn integration_small_half_angle_valid() {
    let small = 1e-6_f64;
    let cone = GpCone::new(Ax3::identity(), small);
    assert!(approx_eq(cone.semi_angle(), small, EPS));
    // Radius at large distance with small angle
    let r = cone.radius_at(1000.0);
    assert!(approx_eq(r, 1000.0 * small.tan(), EPS));
}

#[test]
fn integration_large_half_angle_near_pi_over_2_valid() {
    use ironstream::precision::ANGULAR;
    let large = FRAC_PI_2 - 2.0 * ANGULAR;
    let cone = GpCone::new(Ax3::identity(), large);
    assert!(approx_eq(cone.semi_angle(), large, EPS));
}

#[test]
fn integration_circle_at_normal_aligned_with_cone_axis() {
    let cone = GpCone::default();
    let circ = cone.at_angle(4.0);
    let normal = circ.axis();
    let cone_dir = cone.axis().z_dir;
    assert!(approx_eq(normal.x, cone_dir.x, EPS));
    assert!(approx_eq(normal.y, cone_dir.y, EPS));
    assert!(approx_eq(normal.z, cone_dir.z, EPS));
}

#[test]
fn integration_multiple_transforms_compose_correctly() {
    // Translate then rotate, compare to doing both from scratch.
    let ax3 = Ax3::from_origin_normal(
        Pnt::new(0.0, 0.0, 0.0),
        Pnt::new(0.0, 0.0, 1.0),
        Pnt::new(1.0, 0.0, 0.0),
    );
    let cone = GpCone::new(ax3, FRAC_PI_4);

    // Step 1: translate by (1,0,0)
    let cone1 = cone.translated(&Vec3::new(1.0, 0.0, 0.0));
    // Step 2: rotate by PI/2 about Z
    let z_ax1 = Ax1::new(Pnt::new(0.0, 0.0, 0.0), Pnt::new(0.0, 0.0, 1.0));
    let cone2 = cone1.rotated(&z_ax1, FRAC_PI_2);

    // After translate: apex at (1,0,0). After rotate PI/2 about Z: (0,1,0).
    assert!(pnt_approx_eq(&cone2.apex(), &Pnt::new(0.0, 1.0, 0.0), EPS));
    assert!(approx_eq(cone2.semi_angle(), FRAC_PI_4, EPS));
}
