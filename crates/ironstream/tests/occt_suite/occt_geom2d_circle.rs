//! Ported OCCT GTest for `Geom2d_Circle`.
//!
//! Source: `src/ModelingData/TKG2d/GTests/Geom2d_Circle_Test.cxx`
//! (Open CASCADE Technology, master).
//!
//! Each `TEST_F(Geom2d_CircleTest, ...)` is reproduced 1:1 as a Rust `#[test]`,
//! with the SAME numeric inputs, expected values and tolerances
//! (`Precision::Confusion()` -> `ironstream::precision::CONFUSION`).
//!
//! The fixture `SetUp()` builds a circle centred at the origin with radius 5,
//! from `gp_Circ2d(gp_Ax2d(gp_Pnt2d(0,0), gp_Dir2d(1,0)), 5.0)`. The helper
//! [`make_fixture_circle`] reproduces it.

use ironstream::geom2d_circle::{Ax22d2, Circ2d, Geom2dCircle};
use ironstream::gp2d::{Pnt2d, Trsf2d};
use ironstream::precision::CONFUSION;
use std::f64::consts::PI;

/// `Precision::Confusion()`.
fn confusion() -> f64 {
    CONFUSION
}

/// Mirrors the GTest `SetUp()`:
/// `gp_Circ2d aCirc(gp_Ax2d(gp_Pnt2d(0,0), gp_Dir2d(1,0)), 5.0);`
/// `myCircle = new Geom2d_Circle(aCirc);`
fn make_fixture_circle() -> Geom2dCircle {
    let circ = Circ2d::new(Pnt2d::new(0.0, 0.0), Pnt2d::new(1.0, 0.0), 5.0, true);
    Geom2dCircle::from_circ2d(circ)
}

/// `EXPECT_NEAR(a, b, tol)`.
fn expect_near(a: f64, b: f64, tol: f64) {
    assert!(
        (a - b).abs() <= tol,
        "expected {a} ~= {b} within {tol} (diff {})",
        (a - b).abs()
    );
}

#[test]
fn construct_from_circ2d() {
    let circle = make_fixture_circle();
    expect_near(circle.radius(), 5.0, confusion());
}

#[test]
fn construct_from_axis_and_radius() {
    // new Geom2d_Circle(gp_Ax2d(gp_Pnt2d(1,2), gp_Dir2d(1,0)), 3.0)
    let circle = Geom2dCircle::from_x_axis(Pnt2d::new(1.0, 2.0), Pnt2d::new(1.0, 0.0), 3.0, true);

    expect_near(circle.radius(), 3.0, confusion());
    expect_near(circle.location().x, 1.0, confusion());
    expect_near(circle.location().y, 2.0, confusion());
}

#[test]
fn construct_from_ax22d() {
    // gp_Ax22d anAxis(gp_Pnt2d(3,4), gp_Dir2d(1,0), gp_Dir2d(0,1));
    // new Geom2d_Circle(anAxis, 7.0)
    let axis = Ax22d2::new(
        Pnt2d::new(3.0, 4.0),
        Pnt2d::new(1.0, 0.0),
        Pnt2d::new(0.0, 1.0),
    );
    let circle = Geom2dCircle::from_axis(axis, 7.0);

    expect_near(circle.radius(), 7.0, confusion());
    expect_near(circle.location().x, 3.0, confusion());
}

#[test]
fn parameter_bounds() {
    let circle = make_fixture_circle();
    // EXPECT_DOUBLE_EQ — exact equality.
    assert_eq!(circle.first_parameter(), 0.0);
    assert_eq!(circle.last_parameter(), 2.0 * PI);
}

#[test]
fn is_closed_and_periodic() {
    let circle = make_fixture_circle();
    assert!(circle.is_closed());
    assert!(circle.is_periodic());
}

#[test]
fn eccentricity() {
    let circle = make_fixture_circle();
    expect_near(circle.eccentricity(), 0.0, confusion());
}

#[test]
fn set_radius() {
    let mut circle = make_fixture_circle();
    circle.set_radius(10.0);
    expect_near(circle.radius(), 10.0, confusion());
}

#[test]
fn circ2d() {
    let circle = make_fixture_circle();
    let circ = circle.circ2d();
    expect_near(circ.radius(), 5.0, confusion());
}

#[test]
fn eval_d0_at_zero() {
    let circle = make_fixture_circle();
    let p = circle.eval_d0(0.0);
    expect_near(p.x, 5.0, confusion());
    expect_near(p.y, 0.0, confusion());
}

#[test]
fn eval_d0_at_pi_half() {
    let circle = make_fixture_circle();
    let p = circle.eval_d0(PI / 2.0);
    expect_near(p.x, 0.0, confusion());
    expect_near(p.y, 5.0, confusion());
}

#[test]
fn eval_d0_at_pi() {
    let circle = make_fixture_circle();
    let p = circle.eval_d0(PI);
    expect_near(p.x, -5.0, confusion());
    expect_near(p.y, 0.0, confusion());
}

#[test]
fn eval_d1_at_zero() {
    let circle = make_fixture_circle();
    let (_p, d1) = circle.eval_d1(0.0);
    // Tangent at U=0 is (0, R) = (0, 5)
    expect_near(d1.x, 0.0, confusion());
    expect_near(d1.y, 5.0, confusion());
}

#[test]
fn eval_d2_at_zero() {
    let circle = make_fixture_circle();
    let (_p, _d1, d2) = circle.eval_d2(0.0);
    // Second derivative at U=0 is (-R, 0) = (-5, 0)
    expect_near(d2.x, -5.0, confusion());
    expect_near(d2.y, 0.0, confusion());
}

#[test]
fn eval_dn_order1() {
    let circle = make_fixture_circle();
    let d1 = circle.eval_dn(0.0, 1);
    expect_near(d1.x, 0.0, confusion());
    expect_near(d1.y, 5.0, confusion());
}

#[test]
fn reversed_parameter() {
    let circle = make_fixture_circle();
    let u = PI / 3.0;
    let rev = circle.reversed_parameter(u);
    expect_near(rev, 2.0 * PI - u, confusion());
}

#[test]
fn copy() {
    let circle = make_fixture_circle();
    let mut copy = circle.copy();

    expect_near(copy.radius(), 5.0, confusion());

    // Verify independence: mutating the copy must not change the original.
    copy.set_radius(1.0);
    expect_near(circle.radius(), 5.0, confusion());
}

#[test]
fn transform_translation() {
    let mut circle = make_fixture_circle();
    // gp_Trsf2d aTrsf; aTrsf.SetTranslation(gp_Vec2d(10, 20));
    let trsf = Trsf2d::translation(Pnt2d::new(10.0, 20.0));
    circle.transform(&trsf);

    expect_near(circle.location().x, 10.0, confusion());
    expect_near(circle.location().y, 20.0, confusion());
    expect_near(circle.radius(), 5.0, confusion());
}

#[test]
fn point_on_circle_distance_from_center() {
    let circle = make_fixture_circle();
    // All points on the circle should be at distance R from the centre.
    let mut u = 0.0;
    while u < 2.0 * PI {
        let p = circle.eval_d0(u);
        let dist = p.distance(circle.location());
        expect_near(dist, 5.0, confusion());
        u += PI / 6.0;
    }
}
