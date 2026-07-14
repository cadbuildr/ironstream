//! Ported OpenCascade unit tests -- `gp_Hypr2d`.
//!
//! Tests mirror the geometry and numerical expectations from the OCCT
//! `gp_Hypr2d` test suite. The fixture is a hyperbola centred at the origin
//! with its major axis along +X, major radius `a = 5`, minor radius `b = 3`.
//!
//! Tolerances: `precision::CONFUSION = 1e-7`, `precision::ANGULAR = 1e-12`.

use ironstream::gp2d::{Ax2d, Pnt2d, Trsf2d};
use ironstream::gp_hypr2d::Hypr2d;
use ironstream::precision::{ANGULAR, CONFUSION};

/// Build the standard fixture hyperbola used throughout this file.
fn make_hypr() -> Hypr2d {
    Hypr2d::new(
        Ax2d::new(Pnt2d::new(0.0, 0.0), Pnt2d::new(1.0, 0.0)),
        5.0,
        3.0,
        true,
    )
}

/// `EXPECT_NEAR`-equivalent.
fn assert_near(a: f64, b: f64, tol: f64) {
    assert!(
        (a - b).abs() <= tol,
        "expected {a} ≈ {b} (tolerance {tol}); diff = {}",
        (a - b).abs()
    );
}

// ---------------------------------------------------------------------------
// 1. Basic radii and location
// ---------------------------------------------------------------------------

#[test]
fn radii_stored_correctly() {
    let h = make_hypr();
    assert_near(h.major_radius(), 5.0, CONFUSION);
    assert_near(h.minor_radius(), 3.0, CONFUSION);
}

#[test]
fn centre_at_origin() {
    let h = make_hypr();
    assert_near(h.location().x, 0.0, CONFUSION);
    assert_near(h.location().y, 0.0, CONFUSION);
}

// ---------------------------------------------------------------------------
// 2. Eccentricity  e = sqrt(a^2 + b^2) / a  (> 1 for any hyperbola)
// ---------------------------------------------------------------------------

#[test]
fn eccentricity_greater_than_one() {
    let h = make_hypr();
    assert!(h.eccentricity() > 1.0 + ANGULAR);
}

#[test]
fn eccentricity_value() {
    let h = make_hypr();
    // c = sqrt(25 + 9) = sqrt(34); e = sqrt(34) / 5
    let expected = (34.0_f64).sqrt() / 5.0;
    assert_near(h.eccentricity(), expected, CONFUSION);
}

// ---------------------------------------------------------------------------
// 3. Focal distance  c = sqrt(a^2 + b^2)
// ---------------------------------------------------------------------------

#[test]
fn focal_distance() {
    let h = make_hypr();
    let expected = (25.0_f64 + 9.0).sqrt();
    assert_near(h.focal(), expected, CONFUSION);
}

// ---------------------------------------------------------------------------
// 4. Foci  F1 = Centre + c*XDir, F2 = Centre - c*XDir
// ---------------------------------------------------------------------------

#[test]
fn focus1_position() {
    let h = make_hypr();
    let c = h.focal();
    let f1 = h.focus1();
    assert_near(f1.x, c, CONFUSION);
    assert_near(f1.y, 0.0, CONFUSION);
}

#[test]
fn focus2_position() {
    let h = make_hypr();
    let c = h.focal();
    let f2 = h.focus2();
    assert_near(f2.x, -c, CONFUSION);
    assert_near(f2.y, 0.0, CONFUSION);
}

// ---------------------------------------------------------------------------
// 5. Semi-latus rectum  p = b^2 / a
// ---------------------------------------------------------------------------

#[test]
fn parameter_semi_latus_rectum() {
    let h = make_hypr();
    let expected = 9.0 / 5.0; // b^2 / a = 9/5
    assert_near(h.parameter(), expected, CONFUSION);
}

// ---------------------------------------------------------------------------
// 6. Asymptotes — should pass through the centre
// ---------------------------------------------------------------------------

#[test]
fn asymptotes_through_centre() {
    let h = make_hypr();
    let a1 = h.asymptote1();
    let a2 = h.asymptote2();
    // Both asymptote origins == centre
    assert_near(a1.location.x, 0.0, CONFUSION);
    assert_near(a1.location.y, 0.0, CONFUSION);
    assert_near(a2.location.x, 0.0, CONFUSION);
    assert_near(a2.location.y, 0.0, CONFUSION);
}

#[test]
fn asymptote_slopes() {
    let h = make_hypr();
    // Asymptote directions (normalised): (a, b)/norm and (a, -b)/norm
    let a = h.major_radius();
    let b = h.minor_radius();
    let norm = (a * a + b * b).sqrt();
    let a1 = h.asymptote1();
    let a2 = h.asymptote2();
    // asymptote1 direction ~ (a, b)/norm
    assert_near(a1.direction.x, a / norm, CONFUSION);
    assert_near(a1.direction.y, b / norm, CONFUSION);
    // asymptote2 direction ~ (a, -b)/norm
    assert_near(a2.direction.x, a / norm, CONFUSION);
    assert_near(a2.direction.y, -b / norm, CONFUSION);
}

// ---------------------------------------------------------------------------
// 7. Translate
// ---------------------------------------------------------------------------

#[test]
fn translate_shifts_centre() {
    let h = make_hypr().translated(Pnt2d::new(3.0, -4.0));
    assert_near(h.location().x, 3.0, CONFUSION);
    assert_near(h.location().y, -4.0, CONFUSION);
}

#[test]
fn translate_preserves_radii() {
    let h = make_hypr().translated(Pnt2d::new(10.0, 10.0));
    assert_near(h.major_radius(), 5.0, CONFUSION);
    assert_near(h.minor_radius(), 3.0, CONFUSION);
}

// ---------------------------------------------------------------------------
// 8. Rotate
// ---------------------------------------------------------------------------

#[test]
fn rotate_90_degrees() {
    let h = make_hypr();
    let rot = h.rotated(Pnt2d::origin(), std::f64::consts::FRAC_PI_2);
    // XDirection should now point in +Y
    let xd = rot.x_axis().direction;
    assert_near(xd.x, 0.0, CONFUSION);
    assert_near(xd.y, 1.0, CONFUSION);
    // Radii unchanged
    assert_near(rot.major_radius(), 5.0, CONFUSION);
    assert_near(rot.minor_radius(), 3.0, CONFUSION);
}

#[test]
fn rotate_about_offset_centre() {
    let h = make_hypr();
    // Rotate about (1, 0) by PI: centre (0,0) maps to (2, 0).
    let rot = h.rotated(Pnt2d::new(1.0, 0.0), std::f64::consts::PI);
    assert_near(rot.location().x, 2.0, CONFUSION);
    assert_near(rot.location().y, 0.0, CONFUSION);
}

// ---------------------------------------------------------------------------
// 9. Mirror about a point
// ---------------------------------------------------------------------------

#[test]
fn mirror_point() {
    let h = make_hypr();
    let mirrored = h.mirrored_point(Pnt2d::new(1.0, 0.0));
    // Centre (0,0) maps to (2, 0)
    assert_near(mirrored.location().x, 2.0, CONFUSION);
    assert_near(mirrored.location().y, 0.0, CONFUSION);
    // Radii unchanged
    assert_near(mirrored.major_radius(), 5.0, CONFUSION);
}

// ---------------------------------------------------------------------------
// 10. Mirror about a line (axis)
// ---------------------------------------------------------------------------

#[test]
fn mirror_x_axis_flips_y() {
    // A hyperbola translated to (0, 2) mirrored about the X axis goes to (0, -2).
    let h = make_hypr().translated(Pnt2d::new(0.0, 2.0));
    let x_axis = Ax2d::new(Pnt2d::origin(), Pnt2d::new(1.0, 0.0));
    let m = h.mirrored_axis(&x_axis);
    assert_near(m.location().x, 0.0, CONFUSION);
    assert_near(m.location().y, -2.0, CONFUSION);
}

// ---------------------------------------------------------------------------
// 11. Transform (uniform scale)
// ---------------------------------------------------------------------------

#[test]
fn transform_uniform_scale() {
    let h = make_hypr();
    let scale = 2.0;
    let t = Trsf2d {
        m: [[scale, 0.0], [0.0, scale]],
        t: [0.0, 0.0],
    };
    let ht = h.transformed(&t);
    assert_near(ht.major_radius(), 5.0 * scale, CONFUSION);
    assert_near(ht.minor_radius(), 3.0 * scale, CONFUSION);
}

// ---------------------------------------------------------------------------
// 12. is_direct
// ---------------------------------------------------------------------------

#[test]
fn direct_frame_sense() {
    let h = make_hypr();
    assert!(h.is_direct());
}

#[test]
fn indirect_frame_sense() {
    let h = Hypr2d::new(
        Ax2d::new(Pnt2d::origin(), Pnt2d::new(1.0, 0.0)),
        5.0,
        3.0,
        false, // indirect
    );
    assert!(!h.is_direct());
}

// ---------------------------------------------------------------------------
// 13. Conjugate hyperbola
// ---------------------------------------------------------------------------

#[test]
fn conjugate_swaps_radii() {
    let h = make_hypr();
    let c = h.conjugate();
    assert_near(c.major_radius(), h.minor_radius(), CONFUSION);
    assert_near(c.minor_radius(), h.major_radius(), CONFUSION);
}

// ---------------------------------------------------------------------------
// 14. Axes through centre
// ---------------------------------------------------------------------------

#[test]
fn x_axis_direction_along_x() {
    let h = make_hypr();
    let xa = h.x_axis();
    assert_near(xa.direction.x, 1.0, CONFUSION);
    assert_near(xa.direction.y, 0.0, CONFUSION);
}

#[test]
fn y_axis_direction_along_y() {
    let h = make_hypr();
    let ya = h.y_axis();
    assert_near(ya.direction.x, 0.0, CONFUSION);
    assert_near(ya.direction.y, 1.0, CONFUSION);
}

// ---------------------------------------------------------------------------
// 15. Hyperbola constructed at non-zero location
// ---------------------------------------------------------------------------

#[test]
fn non_zero_location_constructor() {
    let h = Hypr2d::new(
        Ax2d::new(Pnt2d::new(1.0, 2.0), Pnt2d::new(1.0, 0.0)),
        4.0,
        2.0,
        true,
    );
    assert_near(h.location().x, 1.0, CONFUSION);
    assert_near(h.location().y, 2.0, CONFUSION);
    assert_near(h.major_radius(), 4.0, CONFUSION);
    assert_near(h.minor_radius(), 2.0, CONFUSION);
    // Focus1 should be at (1 + sqrt(20), 2)
    let c = (16.0_f64 + 4.0).sqrt();
    assert_near(h.focus1().x, 1.0 + c, CONFUSION);
    assert_near(h.focus1().y, 2.0, CONFUSION);
}
