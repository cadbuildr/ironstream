// FILE: tests/occt_gp_tor.rs
extern crate ironstream;
use ironstream::gp_tor::GpTorus;
use ironstream::gp::{Pnt, Vec3, Ax1, Ax3};
use std::f64::consts::{PI, FRAC_PI_2};

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
fn integration_default_torus_center_at_origin() {
    let t = GpTorus::default();
    assert!(pnt_approx_eq(&t.location(), &Pnt::new(0.0, 0.0, 0.0), EPS));
}

#[test]
fn integration_default_torus_major_radius_is_2() {
    let t = GpTorus::default();
    assert!(approx_eq(t.major_radius(), 2.0, EPS));
}

#[test]
fn integration_default_torus_minor_radius_is_1() {
    let t = GpTorus::default();
    assert!(approx_eq(t.minor_radius(), 1.0, EPS));
}

#[test]
fn integration_default_torus_is_direct() {
    let t = GpTorus::default();
    assert!(t.is_direct());
}

#[test]
fn integration_custom_torus_radii() {
    let t = GpTorus::new(Ax3::identity(), 5.0, 2.0);
    assert!(approx_eq(t.major_radius(), 5.0, EPS));
    assert!(approx_eq(t.minor_radius(), 2.0, EPS));
}

// ---------------------------------------------------------------------------
// Area and Volume
// ---------------------------------------------------------------------------

#[test]
fn integration_area_formula() {
    let t = GpTorus::default(); // R=2, r=1
    let expected = 4.0 * PI * PI * 2.0 * 1.0;
    assert!(approx_eq(t.area(), expected, EPS));
}

#[test]
fn integration_volume_formula() {
    let t = GpTorus::default(); // R=2, r=1
    let expected = 2.0 * PI * PI * 2.0 * 1.0 * 1.0;
    assert!(approx_eq(t.volume(), expected, EPS));
}

#[test]
fn integration_area_scales_as_r_squared() {
    // Scaling by 2 scales R→4, r→2, area = 4π²·4·2 = 4 * original area
    let t = GpTorus::default();
    let t2 = t.scaled(&Pnt::new(0.0, 0.0, 0.0), 2.0);
    assert!(approx_eq(t2.area(), 4.0 * t.area(), 1e-9));
}

// ---------------------------------------------------------------------------
// Iso-circles
// ---------------------------------------------------------------------------

#[test]
fn integration_outer_circle_radius_is_r_plus_r() {
    let t = GpTorus::default(); // R=2, r=1 → outer=3
    assert!(approx_eq(t.outer_circle().radius(), 3.0, EPS));
}

#[test]
fn integration_inner_circle_radius_is_r_minus_r() {
    let t = GpTorus::default(); // R=2, r=1 → inner=1
    assert!(approx_eq(t.inner_circle().radius(), 1.0, EPS));
}

#[test]
fn integration_v_iso_at_zero_gives_outer_equator() {
    let t = GpTorus::default();
    let circ = t.v_iso(0.0);
    // V=0 → radius = R + r = 3, height = 0
    assert!(approx_eq(circ.radius(), 3.0, EPS));
    assert!(approx_eq(circ.location().z, 0.0, EPS));
}

#[test]
fn integration_v_iso_at_pi_over_2_has_height_r() {
    let t = GpTorus::default(); // r=1
    let circ = t.v_iso(FRAC_PI_2);
    // sin(π/2) = 1, height = r·1 = 1
    assert!(approx_eq(circ.location().z, 1.0, EPS));
    // radius = R + r·cos(π/2) = R = 2
    assert!(approx_eq(circ.radius(), 2.0, EPS));
}

#[test]
fn integration_u_iso_tube_center_on_equatorial_plane() {
    let t = GpTorus::default(); // R=2
    // U=0: tube center at (2,0,0)
    let circ = t.u_iso(0.0);
    assert!(approx_eq(circ.location().x, 2.0, EPS));
    assert!(approx_eq(circ.location().y, 0.0, EPS));
    assert!(approx_eq(circ.location().z, 0.0, EPS));
    assert!(approx_eq(circ.radius(), t.minor_radius(), EPS));
}

// ---------------------------------------------------------------------------
// Parametric value and contains
// ---------------------------------------------------------------------------

#[test]
fn integration_value_u0_v0_is_outer_equator_point() {
    let t = GpTorus::default();
    let p = t.value(0.0, 0.0);
    assert!(pnt_approx_eq(&p, &Pnt::new(3.0, 0.0, 0.0), EPS));
}

#[test]
fn integration_value_u0_v_pi_is_inner_equator_point() {
    let t = GpTorus::default();
    let p = t.value(0.0, PI);
    assert!(pnt_approx_eq(&p, &Pnt::new(1.0, 0.0, 0.0), EPS));
}

#[test]
fn integration_value_u0_v_pi_over_2_is_tube_top() {
    let t = GpTorus::default(); // R=2, r=1
    // U=0, V=π/2 → P = (R, 0, r) = (2, 0, 1)
    let p = t.value(0.0, FRAC_PI_2);
    assert!(pnt_approx_eq(&p, &Pnt::new(2.0, 0.0, 1.0), EPS));
}

#[test]
fn integration_all_parametric_points_are_on_surface() {
    let t = GpTorus::default();
    for &(u, v) in &[
        (0.0_f64, 0.0_f64),
        (FRAC_PI_2, 0.0),
        (PI, PI),
        (1.5, 2.3),
        (3.0, 0.7),
    ] {
        let p = t.value(u, v);
        assert!(t.contains(p, 1e-9), "({u:.2}, {v:.2}) not on surface");
    }
}

#[test]
fn integration_center_not_on_surface() {
    let t = GpTorus::default();
    assert!(!t.contains(Pnt::new(0.0, 0.0, 0.0), 1e-9));
}

// ---------------------------------------------------------------------------
// Translation
// ---------------------------------------------------------------------------

#[test]
fn integration_translate_moves_center() {
    let mut t = GpTorus::default();
    t.translate(&Vec3::new(1.0, 2.0, 3.0));
    assert!(pnt_approx_eq(&t.location(), &Pnt::new(1.0, 2.0, 3.0), EPS));
}

#[test]
fn integration_translated_immutable() {
    let t = GpTorus::default();
    let t2 = t.translated(&Vec3::new(5.0, 0.0, 0.0));
    assert!(pnt_approx_eq(&t.location(), &Pnt::new(0.0, 0.0, 0.0), EPS));
    assert!(pnt_approx_eq(&t2.location(), &Pnt::new(5.0, 0.0, 0.0), EPS));
}

#[test]
fn integration_translate_from_to() {
    let mut t = GpTorus::default();
    let p1 = Pnt::new(0.0, 0.0, 0.0);
    let p2 = Pnt::new(3.0, 4.0, 5.0);
    t.translate_from_to(&p1, &p2);
    assert!(pnt_approx_eq(&t.location(), &p2, EPS));
}

#[test]
fn integration_translate_preserves_radii() {
    let mut t = GpTorus::default();
    t.translate(&Vec3::new(100.0, 200.0, 300.0));
    assert!(approx_eq(t.major_radius(), 2.0, EPS));
    assert!(approx_eq(t.minor_radius(), 1.0, EPS));
}

// ---------------------------------------------------------------------------
// Scaling
// ---------------------------------------------------------------------------

#[test]
fn integration_scale_about_origin_scales_radii() {
    let t = GpTorus::default();
    let origin = Pnt::new(0.0, 0.0, 0.0);
    let t2 = t.scaled(&origin, 3.0);
    assert!(approx_eq(t2.major_radius(), 6.0, EPS));
    assert!(approx_eq(t2.minor_radius(), 3.0, EPS));
    assert!(pnt_approx_eq(&t2.location(), &origin, EPS));
}

#[test]
fn integration_scale_moves_off_center_location() {
    let ax3 = Ax3::from_origin_normal(
        Pnt::new(2.0, 0.0, 0.0),
        Pnt::new(0.0, 0.0, 1.0),
        Pnt::new(1.0, 0.0, 0.0),
    );
    let t = GpTorus::new(ax3, 2.0, 1.0);
    let origin = Pnt::new(0.0, 0.0, 0.0);
    let t2 = t.scaled(&origin, 2.0);
    // center (2,0,0) scaled by 2 about (0,0,0) = (4,0,0)
    assert!(pnt_approx_eq(&t2.location(), &Pnt::new(4.0, 0.0, 0.0), EPS));
    assert!(approx_eq(t2.major_radius(), 4.0, EPS));
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
    let t = GpTorus::new(ax3, 2.0, 1.0);
    let z_ax1 = Ax1::new(Pnt::new(0.0, 0.0, 0.0), Pnt::new(0.0, 0.0, 1.0));
    let t2 = t.rotated(&z_ax1, FRAC_PI_2);
    // center (1,0,0) rotated PI/2 about Z → (0,1,0)
    assert!(pnt_approx_eq(&t2.location(), &Pnt::new(0.0, 1.0, 0.0), 1e-9));
    assert!(approx_eq(t2.major_radius(), 2.0, EPS));
    assert!(approx_eq(t2.minor_radius(), 1.0, EPS));
}

#[test]
fn integration_rotate_full_circle_returns_to_start() {
    let ax3 = Ax3::from_origin_normal(
        Pnt::new(3.0, 0.0, 0.0),
        Pnt::new(0.0, 0.0, 1.0),
        Pnt::new(1.0, 0.0, 0.0),
    );
    let t = GpTorus::new(ax3, 2.0, 0.5);
    let z_ax1 = Ax1::new(Pnt::new(0.0, 0.0, 0.0), Pnt::new(0.0, 0.0, 1.0));
    let t2 = t.rotated(&z_ax1, 2.0 * PI);
    assert!(pnt_approx_eq(&t2.location(), &t.location(), 1e-9));
}

#[test]
fn integration_rotate_in_place() {
    let ax3 = Ax3::from_origin_normal(
        Pnt::new(1.0, 0.0, 0.0),
        Pnt::new(0.0, 0.0, 1.0),
        Pnt::new(1.0, 0.0, 0.0),
    );
    let mut t = GpTorus::new(ax3, 2.0, 1.0);
    let z_ax1 = Ax1::new(Pnt::new(0.0, 0.0, 0.0), Pnt::new(0.0, 0.0, 1.0));
    t.rotate(&z_ax1, FRAC_PI_2);
    assert!(pnt_approx_eq(&t.location(), &Pnt::new(0.0, 1.0, 0.0), 1e-9));
}

// ---------------------------------------------------------------------------
// Mirror through point
// ---------------------------------------------------------------------------

#[test]
fn integration_mirror_through_point_moves_center() {
    let t = GpTorus::default(); // center (0,0,0)
    let pt = Pnt::new(2.0, 0.0, 0.0);
    let t2 = t.mirrored_through_point(&pt);
    assert!(pnt_approx_eq(&t2.location(), &Pnt::new(4.0, 0.0, 0.0), EPS));
    assert!(approx_eq(t2.major_radius(), 2.0, EPS));
}

#[test]
fn integration_mirror_through_point_twice_returns_original() {
    let t = GpTorus::default();
    let pt = Pnt::new(3.0, 4.0, 5.0);
    let t2 = t
        .mirrored_through_point(&pt)
        .mirrored_through_point(&pt);
    assert!(pnt_approx_eq(&t2.location(), &t.location(), 1e-9));
}

#[test]
fn integration_mirror_through_point_in_place() {
    let mut t = GpTorus::default();
    let pt = Pnt::new(1.0, 0.0, 0.0);
    t.mirror_through_point(&pt);
    assert!(pnt_approx_eq(&t.location(), &Pnt::new(2.0, 0.0, 0.0), EPS));
}

// ---------------------------------------------------------------------------
// Mirror through Ax1 (line)
// ---------------------------------------------------------------------------

#[test]
fn integration_mirror_through_ax1_flips_center() {
    let ax3 = Ax3::from_origin_normal(
        Pnt::new(1.0, 0.0, 0.0),
        Pnt::new(0.0, 0.0, 1.0),
        Pnt::new(1.0, 0.0, 0.0),
    );
    let t = GpTorus::new(ax3, 2.0, 1.0);
    // Mirror center (1,0,0) through Y-axis → (-1,0,0)
    let y_ax1 = Ax1::new(Pnt::new(0.0, 0.0, 0.0), Pnt::new(0.0, 1.0, 0.0));
    let t2 = t.mirrored_through_ax1(&y_ax1);
    assert!(pnt_approx_eq(&t2.location(), &Pnt::new(-1.0, 0.0, 0.0), EPS));
    assert!(approx_eq(t2.major_radius(), 2.0, EPS));
}

#[test]
fn integration_mirror_through_ax1_preserves_radii() {
    let t = GpTorus::default();
    let ax1 = Ax1::new(Pnt::new(0.0, 0.0, 0.0), Pnt::new(0.0, 1.0, 0.0));
    let t2 = t.mirrored_through_ax1(&ax1);
    assert!(approx_eq(t2.major_radius(), 2.0, EPS));
    assert!(approx_eq(t2.minor_radius(), 1.0, EPS));
}

// ---------------------------------------------------------------------------
// Mirror through Ax2 (plane)
// ---------------------------------------------------------------------------

#[test]
fn integration_mirror_through_ax2_plane_reflects_center() {
    let ax3 = Ax3::from_origin_normal(
        Pnt::new(0.0, 0.0, 1.0),
        Pnt::new(0.0, 0.0, 1.0),
        Pnt::new(1.0, 0.0, 0.0),
    );
    let t = GpTorus::new(ax3, 2.0, 1.0);
    // Mirror through XY-plane (normal Z at origin) → center (0,0,1) → (0,0,-1)
    let xy_plane = Ax3::from_origin_normal(
        Pnt::new(0.0, 0.0, 0.0),
        Pnt::new(0.0, 0.0, 1.0),
        Pnt::new(1.0, 0.0, 0.0),
    );
    let t2 = t.mirrored_through_ax2(&xy_plane);
    assert!(pnt_approx_eq(&t2.location(), &Pnt::new(0.0, 0.0, -1.0), EPS));
    assert!(approx_eq(t2.minor_radius(), 1.0, EPS));
}

// ---------------------------------------------------------------------------
// Setters
// ---------------------------------------------------------------------------

#[test]
fn integration_set_major_radius() {
    let mut t = GpTorus::default();
    t.set_major_radius(5.0);
    assert!(approx_eq(t.major_radius(), 5.0, EPS));
}

#[test]
fn integration_set_minor_radius() {
    let mut t = GpTorus::default();
    t.set_minor_radius(0.5);
    assert!(approx_eq(t.minor_radius(), 0.5, EPS));
}

#[test]
fn integration_set_location() {
    let mut t = GpTorus::default();
    let new_loc = Pnt::new(10.0, 20.0, 30.0);
    t.set_location(new_loc);
    assert!(pnt_approx_eq(&t.location(), &new_loc, EPS));
}

// ---------------------------------------------------------------------------
// Axis
// ---------------------------------------------------------------------------

#[test]
fn integration_default_axis_is_z() {
    let t = GpTorus::default();
    let ax1 = t.axis();
    assert!(approx_eq(ax1.direction.x, 0.0, EPS));
    assert!(approx_eq(ax1.direction.y, 0.0, EPS));
    assert!(approx_eq(ax1.direction.z, 1.0, EPS));
}

#[test]
fn integration_x_axis_direction() {
    let t = GpTorus::default();
    let xax = t.x_axis();
    assert!(approx_eq(xax.direction.x, 1.0, EPS));
    assert!(approx_eq(xax.direction.y, 0.0, EPS));
    assert!(approx_eq(xax.direction.z, 0.0, EPS));
}

// ---------------------------------------------------------------------------
// Edge cases and validation
// ---------------------------------------------------------------------------

#[test]
#[should_panic]
fn integration_zero_major_radius_panics() {
    let _ = GpTorus::new(Ax3::identity(), 0.0, 1.0);
}

#[test]
#[should_panic]
fn integration_zero_minor_radius_panics() {
    let _ = GpTorus::new(Ax3::identity(), 2.0, 0.0);
}

#[test]
#[should_panic]
fn integration_negative_major_radius_panics() {
    let _ = GpTorus::new(Ax3::identity(), -1.0, 1.0);
}

#[test]
fn integration_horn_torus_inner_circle_zero_radius() {
    // When minor_radius == major_radius, inner = 0 (horn torus)
    let t = GpTorus::new(Ax3::identity(), 3.0, 3.0);
    assert!(approx_eq(t.inner_circle().radius(), 0.0, EPS));
}

#[test]
fn integration_spindle_torus_minor_greater_than_major() {
    // Spindle torus: minor > major — valid, just unusual
    let t = GpTorus::new(Ax3::identity(), 1.0, 2.0);
    assert!(approx_eq(t.minor_radius(), 2.0, EPS));
    // Inner circle would be negative; clamped to 0
    assert!(approx_eq(t.inner_circle().radius(), 0.0, EPS));
}

#[test]
fn integration_multiple_transforms_compose() {
    // Translate then rotate, verify final position.
    let t = GpTorus::default(); // center (0,0,0)
    // Step 1: translate by (1,0,0)
    let t1 = t.translated(&Vec3::new(1.0, 0.0, 0.0));
    // Step 2: rotate by PI/2 about Z
    let z_ax1 = Ax1::new(Pnt::new(0.0, 0.0, 0.0), Pnt::new(0.0, 0.0, 1.0));
    let t2 = t1.rotated(&z_ax1, FRAC_PI_2);
    // center (1,0,0) rotated PI/2 → (0,1,0)
    assert!(pnt_approx_eq(&t2.location(), &Pnt::new(0.0, 1.0, 0.0), 1e-9));
    assert!(approx_eq(t2.major_radius(), 2.0, EPS));
    assert!(approx_eq(t2.minor_radius(), 1.0, EPS));
}

#[test]
fn integration_position_accessor_returns_correct_frame() {
    let ax3 = Ax3::from_origin_normal(
        Pnt::new(1.0, 2.0, 3.0),
        Pnt::new(0.0, 0.0, 1.0),
        Pnt::new(1.0, 0.0, 0.0),
    );
    let t = GpTorus::new(ax3, 2.0, 1.0);
    let pos = t.position();
    assert!(pnt_approx_eq(&pos.location, &Pnt::new(1.0, 2.0, 3.0), EPS));
    assert!(approx_eq(pos.z_dir.z, 1.0, EPS));
}
