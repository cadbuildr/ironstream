// FILE: tests/occt_gp_circ2d.rs
//! Ported OpenCascade integration tests -- `gp_Circ2d`.
//!
//! Faithful Rust ports of OCCT's `gp_Circ2d` tests. Same numeric inputs,
//! expected values, and tolerances as upstream. The standard fixture is a
//! circle of radius 5 at the origin with its X axis along +X.

extern crate ironstream;

use ironstream::geom2d_circle::Ax22d2;
use ironstream::gp2d::{Ax2d, Pnt2d, Trsf2d};
use ironstream::gp_circ2d::Circ2d;
use ironstream::precision::{ANGULAR, CONFUSION};
use std::f64::consts::{FRAC_PI_2, PI};

// ----------------------------------------------------------------- helpers

fn near(a: f64, b: f64, tol: f64) {
    assert!((a - b).abs() <= tol, "expected {a} ≈ {b} (tol {tol})");
}

fn pnt_near(a: Pnt2d, b: Pnt2d, tol: f64) {
    assert!(
        (a - b).norm() <= tol,
        "expected {a:?} ≈ {b:?} (tol {tol})"
    );
}

/// Build the standard fixture: circle of radius 5 at the origin, X axis along +X.
fn c5() -> Circ2d {
    Circ2d::new(
        Ax2d::new(Pnt2d::origin(), Pnt2d::new(1.0, 0.0)),
        5.0,
    )
}

// ================================================================== tests

#[test]
fn constructor_stores_radius_and_centre() {
    let c = c5();
    near(c.radius(), 5.0, CONFUSION);
    pnt_near(c.location(), Pnt2d::origin(), CONFUSION);
}

#[test]
fn from_axis_constructor() {
    // Build from a full gp_Ax22d; verify that radius and centre round-trip.
    let pos = Ax22d2::from_x_axis(Pnt2d::new(3.0, 4.0), Pnt2d::new(1.0, 0.0), true);
    let c = Circ2d::from_axis(pos, 7.0);
    near(c.radius(), 7.0, CONFUSION);
    pnt_near(c.location(), Pnt2d::new(3.0, 4.0), CONFUSION);
}

#[test]
fn area_matches_formula() {
    // Area = PI * R^2 = PI * 25 ≈ 78.539...
    let c = c5();
    near(c.area(), PI * 25.0, CONFUSION);
}

#[test]
fn length_is_circumference() {
    // Length = 2 * PI * R = 10 * PI ≈ 31.415...
    let c = c5();
    near(c.length(), 2.0 * PI * 5.0, CONFUSION);
}

#[test]
fn distance_from_point_on_circle_is_zero() {
    // (5, 0) lies on the circle of radius 5 centred at origin.
    let c = c5();
    near(c.distance(Pnt2d::new(5.0, 0.0)), 0.0, CONFUSION);
    near(c.distance(Pnt2d::new(0.0, 5.0)), 0.0, CONFUSION);
    near(c.distance(Pnt2d::new(-5.0, 0.0)), 0.0, CONFUSION);
}

#[test]
fn distance_from_centre_equals_radius() {
    // Centre of the circle is at distance R from the circle itself.
    let c = c5();
    near(c.distance(Pnt2d::origin()), 5.0, CONFUSION);
}

#[test]
fn distance_from_external_point() {
    // Point (10, 0) is 5 units outside the circle (radius 5, centre origin).
    let c = c5();
    near(c.distance(Pnt2d::new(10.0, 0.0)), 5.0, CONFUSION);
}

#[test]
fn contains_point_on_circle() {
    let c = c5();
    let on_top = Pnt2d::new(0.0, 5.0);
    assert!(c.contains(on_top, 1e-6), "point on circle must be contained");
    // Centre is NOT on the circle.
    assert!(
        !c.contains(Pnt2d::origin(), 1e-6),
        "centre must not be contained (within 1e-6)"
    );
}

#[test]
fn is_direct_default_frame() {
    let c = c5();
    assert!(c.is_direct(), "default frame (sense=true) must be direct");
}

#[test]
fn is_direct_indirect_frame() {
    let c_ind = Circ2d::new_with_sense(
        Ax2d::new(Pnt2d::origin(), Pnt2d::new(1.0, 0.0)),
        5.0,
        false,
    );
    assert!(!c_ind.is_direct(), "sense=false must be indirect");
}

#[test]
fn x_and_y_axes_at_origin() {
    // For the standard circle, XAxis should point along +X, YAxis along +Y.
    let c = c5();
    let xd = c.x_axis().direction.normalized();
    let yd = c.y_axis().direction.normalized();
    near(xd.x, 1.0, ANGULAR);
    near(xd.y, 0.0, ANGULAR);
    near(yd.x, 0.0, ANGULAR);
    near(yd.y, 1.0, ANGULAR);
}

#[test]
fn translate_moves_centre_preserves_radius() {
    let c = c5();
    let t = c.translated(Pnt2d::new(7.0, -4.0));
    pnt_near(t.location(), Pnt2d::new(7.0, -4.0), CONFUSION);
    near(t.radius(), 5.0, CONFUSION);
}

#[test]
fn rotate_moves_centre_preserves_radius() {
    // Circle at (1, 0); rotate 90° CCW about origin → centre at (0, 1).
    let c = Circ2d::new(
        Ax2d::new(Pnt2d::new(1.0, 0.0), Pnt2d::new(1.0, 0.0)),
        3.0,
    );
    let r = c.rotated(Pnt2d::origin(), FRAC_PI_2);
    near(r.location().x, 0.0, CONFUSION);
    near(r.location().y, 1.0, CONFUSION);
    near(r.radius(), 3.0, CONFUSION);
}

#[test]
fn scale_adjusts_radius_and_location() {
    // Circle at (1, 0) radius 3; scale by 2 about origin → centre (2, 0), radius 6.
    let c = Circ2d::new(
        Ax2d::new(Pnt2d::new(1.0, 0.0), Pnt2d::new(1.0, 0.0)),
        3.0,
    );
    let s = c.scaled(Pnt2d::origin(), 2.0);
    near(s.radius(), 6.0, CONFUSION);
    pnt_near(s.location(), Pnt2d::new(2.0, 0.0), CONFUSION);
}

#[test]
fn transform_with_translation_trsf() {
    let c = c5();
    let t = Trsf2d::translation(Pnt2d::new(3.0, 5.0));
    let tc = c.transformed(&t);
    pnt_near(tc.location(), Pnt2d::new(3.0, 5.0), CONFUSION);
    near(tc.radius(), 5.0, CONFUSION);
}

#[test]
fn mirror_point_reflects_centre_preserves_radius() {
    // Circle at (2, 3); mirror through origin → centre at (-2, -3).
    let c = Circ2d::new(
        Ax2d::new(Pnt2d::new(2.0, 3.0), Pnt2d::new(1.0, 0.0)),
        4.0,
    );
    let m = c.mirrored_point(Pnt2d::origin());
    pnt_near(m.location(), Pnt2d::new(-2.0, -3.0), CONFUSION);
    near(m.radius(), 4.0, CONFUSION);
}

#[test]
fn mirror_axis_reflects_centre_preserves_radius() {
    // Circle at (0, 3); reflect across X axis (Y=0) → centre at (0, -3).
    let c = Circ2d::new(
        Ax2d::new(Pnt2d::new(0.0, 3.0), Pnt2d::new(1.0, 0.0)),
        2.0,
    );
    let axis = Ax2d::new(Pnt2d::origin(), Pnt2d::new(1.0, 0.0));
    let m = c.mirrored_axis(axis);
    pnt_near(m.location(), Pnt2d::new(0.0, -3.0), CONFUSION);
    near(m.radius(), 2.0, CONFUSION);
}

#[test]
fn set_radius_updates_area() {
    let mut c = c5();
    c.set_radius(10.0);
    near(c.radius(), 10.0, CONFUSION);
    near(c.area(), PI * 100.0, CONFUSION);
    near(c.length(), 2.0 * PI * 10.0, CONFUSION);
}

#[test]
fn set_location_moves_centre_keeps_radius() {
    let mut c = c5();
    c.set_location(Pnt2d::new(11.0, 22.0));
    pnt_near(c.location(), Pnt2d::new(11.0, 22.0), CONFUSION);
    near(c.radius(), 5.0, CONFUSION);
}

#[test]
fn square_distance_is_distance_squared() {
    let c = c5();
    let p = Pnt2d::new(10.0, 0.0); // 5 units outside
    let d = c.distance(p);
    near(c.square_distance(p), d * d, CONFUSION);
}

#[test]
fn scale_negative_factor_reverses_frame_and_scales_radius() {
    // A negative scale reverses the frame (makes it indirect if it was direct)
    // but the radius only scales by the absolute value.
    let c = c5(); // direct frame, radius 5, centre at origin
    let s = c.scaled(Pnt2d::origin(), -2.0);
    near(s.radius(), 10.0, CONFUSION); // radius = 5 * |−2| = 10
    // Handedness flips.
    assert!(!s.is_direct(), "negative scale must produce indirect frame");
}
