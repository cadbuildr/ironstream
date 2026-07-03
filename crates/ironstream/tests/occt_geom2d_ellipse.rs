//! Port of OCCT's `Geom2d_Ellipse_Test.cxx` GTest suite to Rust `#[test]`s.
//!
//! Same numeric inputs, expected values and tolerances as upstream
//! (`Precision::Confusion()` -> `ironstream::precision::CONFUSION`). The OCCT
//! fixture builds the ellipse from a `gp_Elips2d` centred at the origin with a
//! major radius of 10 and a minor radius of 5, its major axis along +X:
//!
//! ```cpp
//! gp_Elips2d anElips(gp_Ax2d(gp_Pnt2d(0,0), gp_Dir2d(1,0)), 10.0, 5.0);
//! myEllipse = new Geom2d_Ellipse(anElips);
//! ```

use ironstream::geom2d_ellipse::{Elips2d, Geom2dEllipse};
use ironstream::gp2d::{Ax2d, Pnt2d, Trsf2d};
use ironstream::precision::CONFUSION;
use std::f64::consts::PI;

/// Builds the fixture ellipse used by every OCCT `TEST_F` (a fresh instance per
/// test, mirroring GTest fixture semantics).
fn make_ellipse() -> Geom2dEllipse {
    let elips = Elips2d::new(
        Ax2d::new(Pnt2d::new(0.0, 0.0), Pnt2d::new(1.0, 0.0)),
        10.0,
        5.0,
        true,
    );
    Geom2dEllipse::from_elips2d(elips)
}

/// `EXPECT_NEAR(a, b, tol)`.
fn assert_near(a: f64, b: f64, tol: f64) {
    assert!((a - b).abs() <= tol, "expected {a} ~= {b} (tol {tol})");
}

#[test]
fn radii() {
    let e = make_ellipse();
    assert_near(e.major_radius(), 10.0, CONFUSION);
    assert_near(e.minor_radius(), 5.0, CONFUSION);
}

#[test]
fn parameter_bounds() {
    let e = make_ellipse();
    // EXPECT_DOUBLE_EQ
    assert_eq!(e.first_parameter(), 0.0);
    assert_eq!(e.last_parameter(), 2.0 * PI);
}

#[test]
fn is_closed_and_periodic() {
    let e = make_ellipse();
    assert!(e.is_closed());
    assert!(e.is_periodic());
}

#[test]
fn eccentricity() {
    let e = make_ellipse();
    // e = sqrt(1 - (b/a)^2) = sqrt(1 - 0.25) = sqrt(0.75)
    let expected = (1.0_f64 - (5.0 * 5.0) / (10.0 * 10.0)).sqrt();
    assert_near(e.eccentricity(), expected, CONFUSION);
}

#[test]
fn focal() {
    let e = make_ellipse();
    // Focal distance = sqrt(a^2 - b^2) * 2
    let expected = 2.0 * (10.0 * 10.0 - 5.0 * 5.0_f64).sqrt();
    assert_near(e.focal(), expected, CONFUSION);
}

#[test]
fn foci() {
    let e = make_ellipse();
    let f1 = e.focus1();
    let f2 = e.focus2();
    let c = (100.0 - 25.0_f64).sqrt(); // sqrt(a^2 - b^2)

    assert_near(f1.x, c, CONFUSION);
    assert_near(f1.y, 0.0, CONFUSION);
    assert_near(f2.x, -c, CONFUSION);
    assert_near(f2.y, 0.0, CONFUSION);
}

#[test]
fn parameter() {
    let e = make_ellipse();
    // Semi-latus rectum = b^2/a = 25/10 = 2.5
    assert_near(e.parameter(), 2.5, CONFUSION);
}

#[test]
fn eval_d0_major_vertex() {
    let e = make_ellipse();
    // At U=0: point on major axis = (a, 0) = (10, 0)
    let p = e.eval_d0(0.0);
    assert_near(p.x, 10.0, CONFUSION);
    assert_near(p.y, 0.0, CONFUSION);
}

#[test]
fn eval_d0_minor_vertex() {
    let e = make_ellipse();
    // At U=PI/2: point on minor axis = (0, b) = (0, 5)
    let p = e.eval_d0(PI / 2.0);
    assert_near(p.x, 0.0, CONFUSION);
    assert_near(p.y, 5.0, CONFUSION);
}

#[test]
fn eval_d1_at_zero() {
    let e = make_ellipse();
    let (_p, d1) = e.eval_d1(0.0);
    // At U=0: tangent is (0, b) = (0, 5)
    assert_near(d1.x, 0.0, CONFUSION);
    assert_near(d1.y, 5.0, CONFUSION);
}

#[test]
fn eval_d2_at_zero() {
    let e = make_ellipse();
    let (_p, _d1, d2) = e.eval_d2(0.0);
    // At U=0: second derivative is (-a, 0) = (-10, 0)
    assert_near(d2.x, -10.0, CONFUSION);
    assert_near(d2.y, 0.0, CONFUSION);
}

#[test]
fn set_major_radius() {
    let mut e = make_ellipse();
    e.set_major_radius(20.0);
    assert_near(e.major_radius(), 20.0, CONFUSION);
}

#[test]
fn set_minor_radius() {
    let mut e = make_ellipse();
    e.set_minor_radius(3.0);
    assert_near(e.minor_radius(), 3.0, CONFUSION);
}

#[test]
fn point_on_ellipse_foci_distance_sum() {
    let e = make_ellipse();
    // For any point on ellipse: d(P, F1) + d(P, F2) = 2a
    let f1 = e.focus1();
    let f2 = e.focus2();

    let mut u = 0.0;
    while u < 2.0 * PI {
        let p = e.eval_d0(u);
        let sum = p.distance(f1) + p.distance(f2);
        assert_near(sum, 2.0 * 10.0, CONFUSION);
        u += PI / 6.0;
    }
}

#[test]
fn copy() {
    let e = make_ellipse();
    let copy = e.copy();
    assert_near(copy.major_radius(), 10.0, CONFUSION);
    assert_near(copy.minor_radius(), 5.0, CONFUSION);
}

#[test]
fn transform_translation() {
    let mut e = make_ellipse();
    let trsf = Trsf2d::translation(Pnt2d::new(5.0, 10.0));
    e.transform(&trsf);

    assert_near(e.location().x, 5.0, CONFUSION);
    assert_near(e.location().y, 10.0, CONFUSION);
    assert_near(e.major_radius(), 10.0, CONFUSION);
}

#[test]
fn reversed_parameter() {
    let e = make_ellipse();
    let u = PI / 4.0;
    assert_near(e.reversed_parameter(u), 2.0 * PI - u, CONFUSION);
}
