//! Ported OpenCascade unit test — `Geom2d_Line`.
//!
//! Faithful Rust port of OCCT's GoogleTest suite
//! `src/ModelingData/TKG2d/GTests/Geom2d_Line_Test.cxx`
//! (Open-Cascade-SAS/OCCT). Same numeric inputs, same expected values, same
//! tolerances (`Precision::Confusion` -> `ironstream::precision::CONFUSION`).
//! OCCT is the reference; IronStream must reproduce it.
//!
//! The OCCT fixture `SetUp()` builds a horizontal line through the origin:
//! `new Geom2d_Line(gp_Pnt2d(0,0), gp_Dir2d(1,0))`. Because there is no shared
//! mutable GoogleTest fixture in Rust, each test constructs that line locally
//! via [`my_line`], which is exactly equivalent.

use ironstream::geom2d_line::{Geom2dLine, GeomAbsShape, Lin2d};
use ironstream::gp2d::{Pnt2d, Trsf2d, Vec2d};
use ironstream::precision::CONFUSION;
use std::f64::consts::PI;

/// `EXPECT_NEAR(a, b, tol)`.
fn near(a: f64, b: f64, tol: f64) {
    assert!((a - b).abs() <= tol, "expected {a} ≈ {b} (tol {tol})");
}

/// Mirrors the OCCT fixture `SetUp()`:
/// `myLine = new Geom2d_Line(gp_Pnt2d(0,0), gp_Dir2d(1,0));`
fn my_line() -> Geom2dLine {
    Geom2dLine::from_point_dir(Pnt2d::new(0.0, 0.0), Pnt2d::new(1.0, 0.0))
}

// ===================== Geom2d_Line_Test.cxx =====================

#[test]
fn construct_from_point_and_dir() {
    let line = my_line();
    near(line.direction().x, 1.0, CONFUSION);
    near(line.direction().y, 0.0, CONFUSION);
    near(line.location().x, 0.0, CONFUSION);
    near(line.location().y, 0.0, CONFUSION);
}

#[test]
fn construct_from_lin2d() {
    let lin = Lin2d::new(Pnt2d::new(1.0, 2.0), Pnt2d::new(0.0, 1.0));
    let line = Geom2dLine::from_lin2d(lin);

    near(line.direction().x, 0.0, CONFUSION);
    near(line.direction().y, 1.0, CONFUSION);
}

#[test]
fn construct_from_axis() {
    let axis = ironstream::gp2d::Ax2d::new(Pnt2d::new(3.0, 4.0), Pnt2d::new(1.0, 0.0));
    let line = Geom2dLine::new(axis);

    near(line.location().x, 3.0, CONFUSION);
}

#[test]
fn parameter_bounds() {
    // OCCT uses large finite bounds (not numeric_limits::max).
    let line = my_line();
    assert!(line.first_parameter() < -1e+90);
    assert!(line.last_parameter() > 1e+90);
}

#[test]
fn is_not_closed_not_periodic() {
    let line = my_line();
    assert!(!line.is_closed());
    assert!(!line.is_periodic());
}

#[test]
fn continuity() {
    let line = my_line();
    assert_eq!(line.continuity(), GeomAbsShape::CN);
    assert!(line.is_cn(100));
}

#[test]
fn eval_d0() {
    let line = my_line();
    let pnt = line.eval_d0(5.0);
    near(pnt.x, 5.0, CONFUSION);
    near(pnt.y, 0.0, CONFUSION);
}

#[test]
fn eval_d1_direction_is_constant() {
    let line = my_line();
    let res = line.eval_d1(5.0);
    near(res.v1.x, 1.0, CONFUSION);
    near(res.v1.y, 0.0, CONFUSION);
}

#[test]
fn eval_d2_is_zero() {
    let line = my_line();
    let res = line.eval_d2(5.0);
    near(res.v2.norm(), 0.0, CONFUSION);
}

#[test]
fn eval_d3_is_zero() {
    let line = my_line();
    let res = line.eval_d3(5.0);
    near(res.v3.norm(), 0.0, CONFUSION);
}

#[test]
fn eval_dn_higher_order() {
    let line = my_line();
    let d4 = line.eval_dn(0.0, 4);
    near(d4.norm(), 0.0, CONFUSION);
}

#[test]
fn distance() {
    let line = my_line();
    near(line.distance(Pnt2d::new(5.0, 3.0)), 3.0, CONFUSION);
    near(line.distance(Pnt2d::new(0.0, 0.0)), 0.0, CONFUSION);
}

#[test]
fn set_direction() {
    let mut line = my_line();
    line.set_direction(Pnt2d::new(0.0, 1.0));
    near(line.direction().x, 0.0, CONFUSION);
    near(line.direction().y, 1.0, CONFUSION);
}

#[test]
fn set_location() {
    let mut line = my_line();
    line.set_location(Pnt2d::new(3.0, 4.0));
    near(line.location().x, 3.0, CONFUSION);
    near(line.location().y, 4.0, CONFUSION);
}

#[test]
fn lin2d() {
    let line = my_line();
    let lin = line.lin2d();
    near(lin.direction().x, 1.0, CONFUSION);
    near(lin.location().x, 0.0, CONFUSION);
}

#[test]
fn copy() {
    let line = my_line();
    let mut copy = line.copy();

    near(copy.direction().x, 1.0, CONFUSION);

    copy.set_direction(Pnt2d::new(0.0, 1.0));
    // Mutating the copy must not affect the original.
    near(line.direction().x, 1.0, CONFUSION);
}

#[test]
fn transform_translation() {
    let mut line = my_line();
    let trsf = Trsf2d::translation(Vec2d::new(5.0, 10.0));
    line.transform(&trsf);

    near(line.location().x, 5.0, CONFUSION);
    near(line.location().y, 10.0, CONFUSION);
}

#[test]
fn transform_rotation() {
    let mut line = my_line();
    let trsf = Trsf2d::rotation(Pnt2d::new(0.0, 0.0), PI / 2.0);
    line.transform(&trsf);

    // Horizontal line rotated 90 degrees becomes vertical.
    near(line.direction().x, 0.0, CONFUSION);
    near(line.direction().y.abs(), 1.0, CONFUSION);
}

#[test]
fn reversed_parameter() {
    let line = my_line();
    assert_eq!(line.reversed_parameter(5.0), -5.0);
}

#[test]
fn diagonal_line_value() {
    let diag = Geom2dLine::from_point_dir(Pnt2d::new(0.0, 0.0), Pnt2d::new(1.0, 1.0));

    let pnt = diag.eval_d0(2.0_f64.sqrt());
    near(pnt.x, 1.0, CONFUSION);
    near(pnt.y, 1.0, CONFUSION);
}
