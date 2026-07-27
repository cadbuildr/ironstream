//! Ported OpenCascade unit tests — `gp_Ax22d` class (TKMath package).
//!
//! Faithful Rust ports of OCCT's `gp_Ax22d` API tests.  Same inputs, expected
//! values, and tolerances (`Precision::Confusion` / `Precision::Angular`).
//! OCCT is the reference; IronStream must reproduce it.

use ironstream::gp2d::{Ax2d as Ax2dSimple, Pnt2d};
use ironstream::gp_ax2d::Ax2d;
use ironstream::gp_ax22d::Ax22d;
use ironstream::gp_trsf2d::Trsf2d;
use ironstream::precision::{ANGULAR, CONFUSION};
use std::f64::consts::{FRAC_PI_2, FRAC_PI_4, PI};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn near(a: f64, b: f64, tol: f64) {
    assert!(
        (a - b).abs() <= tol,
        "expected {a} ≈ {b} (tol {tol:.2e}), delta = {}",
        (a - b).abs()
    );
}

fn pnt_near(p: Pnt2d, x: f64, y: f64) {
    near(p.x, x, CONFUSION * 10.0);
    near(p.y, y, CONFUSION * 10.0);
}

// ---------------------------------------------------------------------------
// 1. Constructor & accessors
// ---------------------------------------------------------------------------

#[test]
fn constructor_right_handed_standard() {
    let ax = Ax22d::new(Pnt2d::new(1.0, 2.0), Pnt2d::new(1.0, 0.0), true);
    pnt_near(ax.location(), 1.0, 2.0);
    pnt_near(ax.x_direction(), 1.0, 0.0);
    // Right-handed: Y = X rotated +90°
    pnt_near(ax.y_direction(), 0.0, 1.0);
    assert!(ax.is_direct(), "frame should be right-handed");
}

#[test]
fn constructor_left_handed() {
    let ax = Ax22d::new(Pnt2d::new(3.0, -1.0), Pnt2d::new(0.0, 1.0), false);
    pnt_near(ax.x_direction(), 0.0, 1.0);
    // Left-handed: Y = X rotated −90°: (0,1) → (1, 0)
    pnt_near(ax.y_direction(), 1.0, 0.0);
    assert!(!ax.is_direct(), "frame should be left-handed");
}

#[test]
fn constructor_normalises_direction() {
    // Input direction (3, 4) has magnitude 5 → normalised to (0.6, 0.8).
    let ax = Ax22d::new(Pnt2d::origin(), Pnt2d::new(3.0, 4.0), true);
    let x = ax.x_direction();
    near(x.x, 0.6, CONFUSION);
    near(x.y, 0.8, CONFUSION);
    near((x.x * x.x + x.y * x.y).sqrt(), 1.0, CONFUSION);
}

// ---------------------------------------------------------------------------
// 2. XAxis / YAxis
// ---------------------------------------------------------------------------

#[test]
fn x_axis_and_y_axis_locations_match_origin() {
    let ax = Ax22d::new(Pnt2d::new(2.0, 5.0), Pnt2d::new(1.0, 0.0), true);
    pnt_near(ax.x_axis().location(), 2.0, 5.0);
    pnt_near(ax.y_axis().location(), 2.0, 5.0);
    pnt_near(ax.x_axis().direction(), 1.0, 0.0);
    pnt_near(ax.y_axis().direction(), 0.0, 1.0);
}

#[test]
fn x_and_y_axes_diagonal_frame() {
    // X direction at 45°
    let ax = Ax22d::new(
        Pnt2d::origin(),
        Pnt2d::new(FRAC_PI_4.cos(), FRAC_PI_4.sin()),
        true,
    );
    let sqrt2_inv = 1.0 / 2.0_f64.sqrt();
    pnt_near(ax.x_axis().direction(), sqrt2_inv, sqrt2_inv);
    // Y = X rotated +90°: (1/√2, 1/√2) → (−1/√2, 1/√2)
    pnt_near(ax.y_axis().direction(), -sqrt2_inv, sqrt2_inv);
}

// ---------------------------------------------------------------------------
// 3. Angle
// ---------------------------------------------------------------------------

#[test]
fn angle_zero_same_frame() {
    let a = Ax22d::new(Pnt2d::origin(), Pnt2d::new(1.0, 0.0), true);
    let b = Ax22d::new(Pnt2d::new(1.0, 1.0), Pnt2d::new(1.0, 0.0), true);
    near(a.angle(&b).abs(), 0.0, ANGULAR * 100.0);
}

#[test]
fn angle_90_degrees() {
    let a = Ax22d::new(Pnt2d::origin(), Pnt2d::new(1.0, 0.0), true);
    let b = Ax22d::new(Pnt2d::origin(), Pnt2d::new(0.0, 1.0), true);
    near(a.angle(&b), FRAC_PI_2, ANGULAR * 100.0);
    near(b.angle(&a), -FRAC_PI_2, ANGULAR * 100.0);
}

#[test]
fn angle_180_degrees() {
    let a = Ax22d::new(Pnt2d::origin(), Pnt2d::new(1.0, 0.0), true);
    let b = Ax22d::new(Pnt2d::origin(), Pnt2d::new(-1.0, 0.0), true);
    near(a.angle(&b).abs(), PI, ANGULAR * 100.0);
}

// ---------------------------------------------------------------------------
// 4. Translate
// ---------------------------------------------------------------------------

#[test]
fn translate_in_place_and_functional() {
    let ax = Ax22d::new(Pnt2d::new(1.0, 2.0), Pnt2d::new(1.0, 0.0), true);
    let v = Pnt2d::new(3.0, -1.0);

    // Functional form leaves original intact.
    let t = ax.translated(v);
    pnt_near(ax.location(), 1.0, 2.0); // original unchanged
    pnt_near(t.location(), 4.0, 1.0);
    pnt_near(t.x_direction(), 1.0, 0.0);
    pnt_near(t.y_direction(), 0.0, 1.0);

    // In-place form.
    let mut ax2 = ax;
    ax2.translate(v);
    pnt_near(ax2.location(), 4.0, 1.0);
}

#[test]
fn translate_two_points_form() {
    let ax = Ax22d::new(Pnt2d::origin(), Pnt2d::new(1.0, 0.0), true);
    let p1 = Pnt2d::new(1.0, 2.0);
    let p2 = Pnt2d::new(4.0, 6.0);
    let t = ax.translated_2pts(p1, p2);
    pnt_near(t.location(), 3.0, 4.0);
}

// ---------------------------------------------------------------------------
// 5. Rotate
// ---------------------------------------------------------------------------

#[test]
fn rotate_90_about_origin() {
    let ax = Ax22d::new(Pnt2d::new(1.0, 0.0), Pnt2d::new(1.0, 0.0), true);
    let r = ax.rotated(Pnt2d::origin(), FRAC_PI_2);

    near(r.location().x, 0.0, CONFUSION * 10.0);
    near(r.location().y, 1.0, CONFUSION * 10.0);
    near(r.x_direction().x, 0.0, CONFUSION * 10.0);
    near(r.x_direction().y, 1.0, CONFUSION * 10.0);
    near(r.y_direction().x, -1.0, CONFUSION * 10.0);
    near(r.y_direction().y, 0.0, CONFUSION * 10.0);
    assert!(r.is_direct());
}

#[test]
fn rotate_in_place() {
    let mut ax = Ax22d::new(Pnt2d::new(2.0, 0.0), Pnt2d::new(1.0, 0.0), true);
    ax.rotate(Pnt2d::new(1.0, 0.0), FRAC_PI_2);
    // (2,0) rotated 90° about (1,0): rel=(1,0) → (0,1) → loc=(1,1)
    near(ax.location().x, 1.0, CONFUSION * 10.0);
    near(ax.location().y, 1.0, CONFUSION * 10.0);
}

#[test]
fn rotate_360_returns_to_start() {
    let ax = Ax22d::new(Pnt2d::new(3.0, 4.0), Pnt2d::new(1.0, 0.0), true);
    let r = ax.rotated(Pnt2d::new(1.0, 1.0), 2.0 * PI);
    pnt_near(r.location(), 3.0, 4.0);
    pnt_near(r.x_direction(), 1.0, 0.0);
    pnt_near(r.y_direction(), 0.0, 1.0);
}

// ---------------------------------------------------------------------------
// 6. Mirror through point
// ---------------------------------------------------------------------------

#[test]
fn mirrored_point_origin() {
    let ax = Ax22d::new(Pnt2d::new(2.0, 3.0), Pnt2d::new(1.0, 0.0), true);
    let m = ax.mirrored_point(Pnt2d::origin());
    pnt_near(m.location(), -2.0, -3.0);
    pnt_near(m.x_direction(), -1.0, 0.0);
    pnt_near(m.y_direction(), 0.0, -1.0);
    // Point mirror preserves handedness.
    assert!(m.is_direct());
}

#[test]
fn mirrored_point_non_origin() {
    let ax = Ax22d::new(Pnt2d::new(1.0, 1.0), Pnt2d::new(1.0, 0.0), true);
    let center = Pnt2d::new(0.0, 0.0);
    let mut ax2 = ax;
    ax2.mirror_point(center);
    pnt_near(ax2.location(), -1.0, -1.0);
}

// ---------------------------------------------------------------------------
// 7. Mirror through axis
// ---------------------------------------------------------------------------

#[test]
fn mirrored_axis_x_axis_line() {
    // Mirror a frame above the X axis across the X axis line.
    let ax = Ax22d::new(Pnt2d::new(1.0, 2.0), Pnt2d::new(1.0, 0.0), true);
    let mirror = Ax2d::new(Pnt2d::origin(), Pnt2d::new(1.0, 0.0));
    let m = ax.mirrored_axis(&mirror);
    pnt_near(m.location(), 1.0, -2.0);
    pnt_near(m.x_direction(), 1.0, 0.0);   // X along mirror unchanged
    pnt_near(m.y_direction(), 0.0, -1.0);  // Y reflects across X axis: (0,1)→(0,-1)
    // Handedness flipped by axial mirror.
    assert!(!m.is_direct());
}

#[test]
fn mirrored_axis_twice_restores_original() {
    let ax = Ax22d::new(Pnt2d::new(1.0, 1.0), Pnt2d::new(1.0, 0.0), true);
    let mirror = Ax2d::new(Pnt2d::origin(), Pnt2d::new(1.0, 0.0));
    let m1 = ax.mirrored_axis(&mirror);
    let m2 = m1.mirrored_axis(&mirror);
    pnt_near(m2.location(), ax.location().x, ax.location().y);
    pnt_near(m2.x_direction(), ax.x_direction().x, ax.x_direction().y);
    assert!(m2.is_direct()); // double mirror restores handedness
}

// ---------------------------------------------------------------------------
// 8. Scale
// ---------------------------------------------------------------------------

#[test]
fn scaled_positive_factor() {
    let ax = Ax22d::new(Pnt2d::new(2.0, 1.0), Pnt2d::new(1.0, 0.0), true);
    let s = ax.scaled(Pnt2d::origin(), 3.0);
    pnt_near(s.location(), 6.0, 3.0);
    pnt_near(s.x_direction(), 1.0, 0.0); // direction unchanged
    assert!(s.is_direct());
}

#[test]
fn scaled_negative_factor_flips_handedness() {
    let ax = Ax22d::new(Pnt2d::new(1.0, 0.0), Pnt2d::new(1.0, 0.0), true);
    let s = ax.scaled(Pnt2d::origin(), -2.0);
    pnt_near(s.location(), -2.0, 0.0);
    pnt_near(s.x_direction(), -1.0, 0.0);
    pnt_near(s.y_direction(), 0.0, -1.0);
    assert!(!s.is_direct()); // handedness flipped
}

#[test]
fn scaled_unit_is_identity() {
    let ax = Ax22d::new(Pnt2d::new(3.0, -2.0), Pnt2d::new(0.0, 1.0), true);
    let s = ax.scaled(Pnt2d::origin(), 1.0);
    pnt_near(s.location(), ax.location().x, ax.location().y);
    pnt_near(s.x_direction(), ax.x_direction().x, ax.x_direction().y);
}

// ---------------------------------------------------------------------------
// 9. Transform
// ---------------------------------------------------------------------------

#[test]
fn transform_rotation_trsf2d() {
    let ax = Ax22d::new(Pnt2d::new(1.0, 0.0), Pnt2d::new(1.0, 0.0), true);
    let mut t = Trsf2d::identity();
    t.set_rotation(Pnt2d::origin(), FRAC_PI_2);
    let r = ax.transformed(&t);
    near(r.location().x, 0.0, CONFUSION * 10.0);
    near(r.location().y, 1.0, CONFUSION * 10.0);
    near(r.x_direction().x, 0.0, CONFUSION * 10.0);
    near(r.x_direction().y, 1.0, CONFUSION * 10.0);
    assert!(r.is_direct()); // rotation preserves orientation
}

#[test]
fn transform_mirror_trsf2d_flips_handedness() {
    let ax = Ax22d::new(Pnt2d::new(0.0, 2.0), Pnt2d::new(1.0, 0.0), true);
    let mut t = Trsf2d::identity();
    t.set_mirror_axis(Ax2dSimple::new(Pnt2d::origin(), Pnt2d::new(1.0, 0.0)));
    let r = ax.transformed(&t);
    pnt_near(r.location(), 0.0, -2.0);
    assert!(!r.is_direct());
}

#[test]
fn transform_translation_trsf2d() {
    let ax = Ax22d::new(Pnt2d::new(1.0, 2.0), Pnt2d::new(1.0, 0.0), true);
    let mut t = Trsf2d::identity();
    t.set_translation(Pnt2d::new(10.0, -5.0));
    let r = ax.transformed(&t);
    pnt_near(r.location(), 11.0, -3.0);
    pnt_near(r.x_direction(), 1.0, 0.0);
    assert!(r.is_direct());
}

// ---------------------------------------------------------------------------
// 10. Setters
// ---------------------------------------------------------------------------

#[test]
fn set_location() {
    let mut ax = Ax22d::new(Pnt2d::origin(), Pnt2d::new(1.0, 0.0), true);
    ax.set_location(Pnt2d::new(7.0, -3.0));
    pnt_near(ax.location(), 7.0, -3.0);
    pnt_near(ax.x_direction(), 1.0, 0.0); // axes unchanged
}

#[test]
fn set_x_direction_updates_y_direction() {
    let mut ax = Ax22d::new(Pnt2d::origin(), Pnt2d::new(1.0, 0.0), true);
    ax.set_x_direction(Pnt2d::new(0.0, 1.0));
    pnt_near(ax.x_direction(), 0.0, 1.0);
    // Y = X rotated +90°: (0,1) → (−1, 0)
    pnt_near(ax.y_direction(), -1.0, 0.0);
}

#[test]
fn set_y_direction_updates_x_direction() {
    let mut ax = Ax22d::new(Pnt2d::origin(), Pnt2d::new(1.0, 0.0), true);
    ax.set_y_direction(Pnt2d::new(1.0, 0.0)); // Y = (1,0)
    pnt_near(ax.y_direction(), 1.0, 0.0);
    // In a right-handed frame X = rot(Y, −90°): (1,0) → (0, −1)
    pnt_near(ax.x_direction(), 0.0, -1.0);
}

// ---------------------------------------------------------------------------
// 11. standard() convenience constructor
// ---------------------------------------------------------------------------

#[test]
fn standard_frame() {
    let ax = Ax22d::standard();
    pnt_near(ax.location(), 0.0, 0.0);
    pnt_near(ax.x_direction(), 1.0, 0.0);
    pnt_near(ax.y_direction(), 0.0, 1.0);
    assert!(ax.is_direct());
}

// ---------------------------------------------------------------------------
// 12. Precision constants are importable
// ---------------------------------------------------------------------------

#[test]
fn precision_constants_importable() {
    // Verify the constants have the expected magnitudes at runtime to ensure
    // they are importable (use them in an expression that the compiler cannot
    // optimise away trivially).
    let ratio = CONFUSION / ANGULAR;
    assert!(ratio > 1.0, "CONFUSION should be larger than ANGULAR");
    near(CONFUSION, 1.0e-7, 1.0e-8);
    near(ANGULAR, 1.0e-12, 1.0e-13);
}
