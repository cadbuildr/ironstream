//! Ported OpenCascade unit test — `Geom_Line`.
//!
//! Faithful Rust port of OCCT's `Geom_Line_Test.cxx`
//! (Open-Cascade-SAS/OCCT, `src/ModelingData/TKG3d/GTests/`). Same numeric
//! inputs, expected values and tolerances (`Precision::Confusion` ->
//! [`CONFUSION`], `Precision::Angular` -> [`ANGULAR`],
//! `Precision::Infinite` -> [`INFINITE`]).

use ironstream::geom_line::GeomLine;
use ironstream::gp::{Ax1, Pnt, Trsf};
use ironstream::precision::{ANGULAR, CONFUSION, INFINITE};

fn near(a: f64, b: f64, tol: f64) {
    assert!((a - b).abs() <= tol, "expected {a} ≈ {b} (tol {tol})");
}

// TEST(Geom_LineTest, ConstructFromAx1)
#[test]
fn construct_from_ax1() {
    let an_ax1 = Ax1::new(Pnt::new(0.0, 0.0, 0.0), Pnt::dir(1.0, 0.0, 0.0));
    let a_line = GeomLine::new(an_ax1);

    let a_loc = a_line.lin().location();
    assert!(a_loc.is_equal(Pnt::new(0.0, 0.0, 0.0), CONFUSION));
}

// TEST(Geom_LineTest, ConstructFromPointAndDir)
#[test]
fn construct_from_point_and_dir() {
    let a_line = GeomLine::from_point_dir(Pnt::new(1.0, 2.0, 3.0), Pnt::dir(0.0, 0.0, 1.0));

    let a_loc = a_line.lin().location();
    assert!(a_loc.is_equal(Pnt::new(1.0, 2.0, 3.0), CONFUSION));
    assert!(a_line
        .lin()
        .direction()
        .is_equal_dir(Pnt::dir(0.0, 0.0, 1.0), ANGULAR));
}

// TEST(Geom_LineTest, D0Evaluation)
#[test]
fn d0_evaluation() {
    let a_line = GeomLine::from_point_dir(Pnt::new(0.0, 0.0, 0.0), Pnt::dir(1.0, 0.0, 0.0));
    let a_pnt = a_line.d0(5.0);
    near(a_pnt.x, 5.0, CONFUSION);
    near(a_pnt.y, 0.0, CONFUSION);
    near(a_pnt.z, 0.0, CONFUSION);
}

// TEST(Geom_LineTest, D1Evaluation)
#[test]
fn d1_evaluation() {
    let a_line = GeomLine::from_point_dir(Pnt::new(0.0, 0.0, 0.0), Pnt::dir(0.0, 1.0, 0.0));
    let (a_pnt, a_v1) = a_line.d1(3.0);
    near(a_pnt.y, 3.0, CONFUSION);
    // First derivative is the direction vector (constant for a line).
    near(a_v1.x, 0.0, CONFUSION);
    near(a_v1.y, 1.0, CONFUSION);
    near(a_v1.z, 0.0, CONFUSION);
}

// TEST(Geom_LineTest, D2Evaluation)
#[test]
fn d2_evaluation() {
    let a_line = GeomLine::from_point_dir(Pnt::new(0.0, 0.0, 0.0), Pnt::dir(1.0, 0.0, 0.0));
    let (_a_pnt, _a_v1, a_v2) = a_line.d2(1.0);
    // Second derivative of a line is zero.
    near(a_v2.norm(), 0.0, CONFUSION);
}

// TEST(Geom_LineTest, InfiniteParameters)
#[test]
fn infinite_parameters() {
    let a_line = GeomLine::from_point_dir(Pnt::new(0.0, 0.0, 0.0), Pnt::dir(1.0, 0.0, 0.0));
    assert_eq!(a_line.first_parameter(), -INFINITE);
    assert_eq!(a_line.last_parameter(), INFINITE);
}

// TEST(Geom_LineTest, Reverse)
#[test]
fn reverse() {
    let mut a_line = GeomLine::from_point_dir(Pnt::new(0.0, 0.0, 0.0), Pnt::dir(1.0, 0.0, 0.0));
    a_line.reverse();
    assert!(a_line
        .lin()
        .direction()
        .is_equal_dir(Pnt::dir(-1.0, 0.0, 0.0), ANGULAR));
}

// TEST(Geom_LineTest, Reversed)
#[test]
fn reversed() {
    let a_line = GeomLine::from_point_dir(Pnt::new(0.0, 0.0, 0.0), Pnt::dir(1.0, 0.0, 0.0));
    let a_rev_line = a_line.reversed();
    assert!(a_rev_line
        .lin()
        .direction()
        .is_equal_dir(Pnt::dir(-1.0, 0.0, 0.0), ANGULAR));
}

// TEST(Geom_LineTest, Transform)
#[test]
fn transform() {
    let mut a_line = GeomLine::from_point_dir(Pnt::new(0.0, 0.0, 0.0), Pnt::dir(1.0, 0.0, 0.0));
    let mut a_trsf = Trsf::identity();
    a_trsf.set_translation(Pnt::new(0.0, 0.0, 5.0));
    a_line.transform(&a_trsf);
    near(a_line.lin().location().z, 5.0, CONFUSION);
}

// TEST(Geom_LineTest, Copy)
#[test]
fn copy() {
    let a_line = GeomLine::from_point_dir(Pnt::new(1.0, 2.0, 3.0), Pnt::dir(0.0, 0.0, 1.0));
    let a_copy_line = a_line.copy();
    assert!(a_copy_line
        .lin()
        .location()
        .is_equal(Pnt::new(1.0, 2.0, 3.0), CONFUSION));
}
