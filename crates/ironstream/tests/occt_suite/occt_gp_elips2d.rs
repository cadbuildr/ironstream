//! Ported OpenCascade unit tests -- `gp_Elips2d`.
//!
//! Faithful Rust ports of OCCT's `gp_Elips2d` tests. Same numeric inputs,
//! expected values, and tolerances as upstream. The standard fixture is a
//! 5×3 ellipse at the origin with its major axis along +X.

use ironstream::gp2d::{Ax2d, Pnt2d, Trsf2d};
use ironstream::gp_elips2d::Elips2d;
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

/// Build the standard fixture: 5×3 ellipse at the origin, major axis +X.
fn e53() -> Elips2d {
    Elips2d::new(
        Ax2d::new(Pnt2d::origin(), Pnt2d::new(1.0, 0.0)),
        5.0,
        3.0,
    )
}

// ================================================================== tests

#[test]
fn constructor_stores_radii() {
    let e = e53();
    near(e.major_radius(), 5.0, CONFUSION);
    near(e.minor_radius(), 3.0, CONFUSION);
}

#[test]
fn location_is_centre() {
    let centre = Pnt2d::new(3.0, 4.0);
    let e = Elips2d::new(
        Ax2d::new(centre, Pnt2d::new(1.0, 0.0)),
        10.0,
        4.0,
    );
    pnt_near(e.location(), centre, CONFUSION);
}

#[test]
fn eccentricity_value() {
    // e = sqrt(a² - b²) / a = sqrt(25 - 9) / 5 = 4/5 = 0.8
    let e = e53();
    near(e.eccentricity(), 0.8, CONFUSION);
}

#[test]
fn circle_has_zero_eccentricity() {
    // When major == minor the ellipse degenerates to a circle; e must be 0.
    let e = Elips2d::new(
        Ax2d::new(Pnt2d::origin(), Pnt2d::new(1.0, 0.0)),
        7.0,
        7.0,
    );
    near(e.eccentricity(), 0.0, CONFUSION);
    near(e.focal(), 0.0, CONFUSION);
}

#[test]
fn focal_distance() {
    // focal = 2 * sqrt(a² - b²) = 2 * sqrt(16) = 8
    let e = e53();
    near(e.focal(), 8.0, CONFUSION);
}

#[test]
fn foci_positions() {
    // c = sqrt(25 - 9) = 4
    // Focus1 = (+4, 0), Focus2 = (-4, 0)
    let e = e53();
    pnt_near(e.focus1(), Pnt2d::new(4.0, 0.0), CONFUSION);
    pnt_near(e.focus2(), Pnt2d::new(-4.0, 0.0), CONFUSION);
}

#[test]
fn parameter_semi_latus_rectum() {
    // p = b² / a = 9 / 5 = 1.8
    let e = e53();
    near(e.parameter(), 1.8, CONFUSION);
}

#[test]
fn x_and_y_axes() {
    // For the standard ellipse at origin with major axis along +X:
    // XAxis direction = (1, 0), YAxis direction = (0, 1).
    let e = e53();
    let xa = e.x_axis();
    let ya = e.y_axis();
    let xd = xa.direction.normalized();
    let yd = ya.direction.normalized();
    near(xd.x, 1.0, ANGULAR);
    near(xd.y, 0.0, ANGULAR);
    near(yd.x, 0.0, ANGULAR);
    near(yd.y, 1.0, ANGULAR);
}

#[test]
fn translate_moves_centre() {
    let e = e53();
    let v = Pnt2d::new(10.0, 20.0);
    let t = e.translated(v);
    pnt_near(t.location(), v, CONFUSION);
    // Radii must be unchanged.
    near(t.major_radius(), 5.0, CONFUSION);
    near(t.minor_radius(), 3.0, CONFUSION);
}

#[test]
fn rotate_moves_centre_keeps_radii() {
    // Ellipse centred at (1, 0); rotate 90° CCW about origin → (0, 1).
    let e = Elips2d::new(
        Ax2d::new(Pnt2d::new(1.0, 0.0), Pnt2d::new(1.0, 0.0)),
        5.0,
        3.0,
    );
    let r = e.rotated(Pnt2d::origin(), FRAC_PI_2);
    near(r.location().x, 0.0, CONFUSION);
    near(r.location().y, 1.0, CONFUSION);
    near(r.major_radius(), 5.0, CONFUSION);
    near(r.minor_radius(), 3.0, CONFUSION);
}

#[test]
fn scale_adjusts_radii_and_location() {
    let e = Elips2d::new(
        Ax2d::new(Pnt2d::new(1.0, 0.0), Pnt2d::new(1.0, 0.0)),
        5.0,
        3.0,
    );
    let s = e.scaled(Pnt2d::origin(), 2.0);
    near(s.major_radius(), 10.0, CONFUSION);
    near(s.minor_radius(), 6.0, CONFUSION);
    near(s.location().x, 2.0, CONFUSION);
    near(s.location().y, 0.0, CONFUSION);
}

#[test]
fn transform_with_translation() {
    let e = e53();
    let t = Trsf2d::translation(Pnt2d::new(7.0, 11.0));
    let te = e.transformed(&t);
    near(te.location().x, 7.0, CONFUSION);
    near(te.location().y, 11.0, CONFUSION);
    near(te.major_radius(), 5.0, CONFUSION);
}

#[test]
fn transform_with_rotation() {
    // Rotate 90° CCW about the origin. Ellipse centred at (5, 0) → (0, 5).
    let e = Elips2d::new(
        Ax2d::new(Pnt2d::new(5.0, 0.0), Pnt2d::new(1.0, 0.0)),
        5.0,
        3.0,
    );
    let t = Trsf2d::rotation(Pnt2d::origin(), FRAC_PI_2);
    let te = e.transformed(&t);
    near(te.location().x, 0.0, CONFUSION);
    near(te.location().y, 5.0, CONFUSION);
    near(te.major_radius(), 5.0, CONFUSION);
}

#[test]
fn mirror_point_reflects_centre() {
    // Ellipse at (2, 3); mirror in origin → (-2, -3).
    let e = Elips2d::new(
        Ax2d::new(Pnt2d::new(2.0, 3.0), Pnt2d::new(1.0, 0.0)),
        5.0,
        3.0,
    );
    let m = e.mirrored_point(Pnt2d::origin());
    pnt_near(m.location(), Pnt2d::new(-2.0, -3.0), CONFUSION);
    near(m.major_radius(), 5.0, CONFUSION);
}

#[test]
fn mirror_axis_reflects_centre() {
    // Ellipse at (1, 1); reflect across X axis (Y = 0) → (1, -1).
    let e = Elips2d::new(
        Ax2d::new(Pnt2d::new(1.0, 1.0), Pnt2d::new(1.0, 0.0)),
        5.0,
        3.0,
    );
    let axis = Ax2d::new(Pnt2d::origin(), Pnt2d::new(1.0, 0.0));
    let m = e.mirrored_axis(axis);
    near(m.location().x, 1.0, CONFUSION);
    near(m.location().y, -1.0, CONFUSION);
    near(m.major_radius(), 5.0, CONFUSION);
}

#[test]
fn set_major_radius_updates() {
    let mut e = e53();
    e.set_major_radius(10.0);
    near(e.major_radius(), 10.0, CONFUSION);
    // Eccentricity: sqrt(100 - 9) / 10 = sqrt(91) / 10
    let expected_ecc = (91.0_f64).sqrt() / 10.0;
    near(e.eccentricity(), expected_ecc, CONFUSION);
}

#[test]
fn set_minor_radius_updates() {
    let mut e = e53();
    e.set_minor_radius(1.0);
    near(e.minor_radius(), 1.0, CONFUSION);
    // Parameter: b²/a = 1/5 = 0.2
    near(e.parameter(), 0.2, CONFUSION);
}

#[test]
fn foci_sum_of_distances_is_major_axis() {
    // For any point P on the ellipse: |P-F1| + |P-F2| = 2*a
    let e = e53();
    let f1 = e.focus1();
    let f2 = e.focus2();
    // Vertex along +X: (5, 0)
    let vertex = Pnt2d::new(5.0, 0.0);
    let d1 = vertex.distance(f1);
    let d2 = vertex.distance(f2);
    near(d1 + d2, 2.0 * 5.0, CONFUSION);
}

#[test]
fn directrix1_position() {
    // d = a / e = 5 / 0.8 = 6.25
    // Directrix1 is at x = 6.25, parallel to Y axis.
    let e = e53();
    let dir1 = e.directrix1();
    let expected_x = 5.0 / 0.8; // = 6.25
    near(dir1.location.x, expected_x, CONFUSION);
    near(dir1.location.y, 0.0, CONFUSION);
}

#[test]
fn directrix2_position() {
    // d = a / e = 6.25, so Directrix2 is at x = -6.25.
    let e = e53();
    let dir2 = e.directrix2();
    let expected_x = -(5.0 / 0.8);
    near(dir2.location.x, expected_x, CONFUSION);
    near(dir2.location.y, 0.0, CONFUSION);
}

#[test]
fn is_direct_flag() {
    let e_direct = e53();
    assert!(e_direct.is_direct(), "default (sense=true) must be direct");

    let e_indirect = Elips2d::new_with_sense(
        Ax2d::new(Pnt2d::origin(), Pnt2d::new(1.0, 0.0)),
        5.0,
        3.0,
        false,
    );
    assert!(!e_indirect.is_direct(), "sense=false must be indirect");
}

#[test]
fn foci_on_tilted_ellipse() {
    // Ellipse with major axis at 45°: c should still = sqrt(a²-b²)
    use std::f64::consts::FRAC_1_SQRT_2;
    let angle = PI / 4.0;
    let dir = Pnt2d::new(angle.cos(), angle.sin());
    let e = Elips2d::new(Ax2d::new(Pnt2d::origin(), dir), 5.0, 3.0);
    let f1 = e.focus1();
    let f2 = e.focus2();
    // |F1| and |F2| from origin should both equal c = 4.
    near(f1.norm(), 4.0, CONFUSION);
    near(f2.norm(), 4.0, CONFUSION);
    // F1 + F2 == (0,0) (symmetric).
    near((f1 + f2).norm(), 0.0, CONFUSION);
    // F1 should be along the major axis direction.
    let f1_unit = f1.normalized();
    near(f1_unit.x, FRAC_1_SQRT_2, CONFUSION);
    near(f1_unit.y, FRAC_1_SQRT_2, CONFUSION);
}

#[test]
fn from_axis_constructor() {
    use ironstream::geom2d_circle::Ax22d2;
    let pos = Ax22d2::from_x_axis(Pnt2d::new(1.0, 2.0), Pnt2d::new(1.0, 0.0), true);
    let e = Elips2d::from_axis(pos, 8.0, 4.0);
    near(e.major_radius(), 8.0, CONFUSION);
    near(e.minor_radius(), 4.0, CONFUSION);
    pnt_near(e.location(), Pnt2d::new(1.0, 2.0), CONFUSION);
}
