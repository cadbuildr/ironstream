//! Ported OpenCascade unit tests — `gp` package.
//!
//! These are faithful Rust ports of OCCT's GoogleTest suites
//! `src/FoundationClasses/TKMath/GTests/gp_Pnt_Test.cxx` and `gp_Dir_Test.cxx`
//! (Open-Cascade-SAS/OCCT). Same inputs, same expected values, same tolerances
//! (`Precision::Confusion`/`Angular`). OCCT is the reference; IronStream must
//! reproduce it.

use ironstream::gp::{Ax1, Ax3, Pnt, Trsf};
use ironstream::precision::{ANGULAR, CONFUSION};
use std::f64::consts::FRAC_PI_2;

fn near(a: f64, b: f64, tol: f64) {
    assert!((a - b).abs() <= tol, "expected {a} ≈ {b} (tol {tol})");
}

// ====================== gp_Pnt_Test.cxx ======================

#[test]
fn pnt_default_constructor() {
    let p = Pnt::origin();
    assert_eq!(p.x, 0.0);
    assert_eq!(p.y, 0.0);
    assert_eq!(p.z, 0.0);
}

#[test]
fn pnt_coordinate_constructor() {
    let p = Pnt::new(1.0, 2.0, 3.0);
    assert_eq!(p.x, 1.0);
    assert_eq!(p.y, 2.0);
    assert_eq!(p.z, 3.0);
}

#[test]
fn pnt_distance() {
    let p1 = Pnt::new(0.0, 0.0, 0.0);
    let p2 = Pnt::new(3.0, 4.0, 0.0);
    near(p1.distance(p2), 5.0, CONFUSION);
}

#[test]
fn pnt_square_distance() {
    let p1 = Pnt::new(1.0, 2.0, 3.0);
    let p2 = Pnt::new(4.0, 6.0, 3.0);
    // 9 + 16 + 0 = 25
    near(p1.square_distance(p2), 25.0, CONFUSION);
}

#[test]
fn pnt_is_equal() {
    let p1 = Pnt::new(1.0, 2.0, 3.0);
    let p2 = Pnt::new(1.0, 2.0, 3.0);
    let p3 = Pnt::new(1.0, 2.0, 4.0);
    assert!(p1.is_equal(p2, CONFUSION));
    assert!(!p1.is_equal(p3, CONFUSION));
}

#[test]
fn pnt_bary_center() {
    let mut p1 = Pnt::new(0.0, 0.0, 0.0);
    let p2 = Pnt::new(10.0, 0.0, 0.0);
    p1.bary_center(1.0, p2, 1.0); // equal weights -> midpoint
    near(p1.x, 5.0, CONFUSION);
    near(p1.y, 0.0, CONFUSION);
    near(p1.z, 0.0, CONFUSION);
}

#[test]
fn pnt_translate_by_vec() {
    let p = Pnt::new(1.0, 2.0, 3.0);
    let r = p.translated(Pnt::new(10.0, 20.0, 30.0));
    near(r.x, 11.0, CONFUSION);
    near(r.y, 22.0, CONFUSION);
    near(r.z, 33.0, CONFUSION);
}

#[test]
fn pnt_translate_by_two_points() {
    let p = Pnt::new(1.0, 2.0, 3.0);
    let r = p.translated_2(Pnt::new(0.0, 0.0, 0.0), Pnt::new(5.0, 5.0, 5.0));
    near(r.x, 6.0, CONFUSION);
    near(r.y, 7.0, CONFUSION);
    near(r.z, 8.0, CONFUSION);
}

#[test]
fn pnt_scale() {
    let p = Pnt::new(2.0, 4.0, 6.0);
    let r = p.scaled(Pnt::origin(), 2.0);
    near(r.x, 4.0, CONFUSION);
    near(r.y, 8.0, CONFUSION);
    near(r.z, 12.0, CONFUSION);
}

#[test]
fn pnt_rotate() {
    let p = Pnt::new(1.0, 0.0, 0.0);
    let axis = Ax1::new(Pnt::origin(), Pnt::new(0.0, 0.0, 1.0));
    let r = p.rotated(axis, FRAC_PI_2);
    near(r.x, 0.0, CONFUSION);
    near(r.y, 1.0, CONFUSION);
    near(r.z, 0.0, CONFUSION);
}

#[test]
fn pnt_mirror_point() {
    let p = Pnt::new(1.0, 0.0, 0.0);
    let r = p.mirrored_point(Pnt::origin());
    near(r.x, -1.0, CONFUSION);
    near(r.y, 0.0, CONFUSION);
    near(r.z, 0.0, CONFUSION);
}

#[test]
fn pnt_mirror_axis() {
    let p = Pnt::new(1.0, 1.0, 0.0);
    let axis = Ax1::new(Pnt::origin(), Pnt::new(1.0, 0.0, 0.0));
    let r = p.mirrored_axis(axis);
    near(r.x, 1.0, CONFUSION);
    near(r.y, -1.0, CONFUSION);
    near(r.z, 0.0, CONFUSION);
}

#[test]
fn pnt_mirror_plane() {
    let p = Pnt::new(1.0, 2.0, 3.0);
    // Mirror through the XY plane (Z=0).
    let ax2 = Ax3::from_origin_normal(
        Pnt::origin(),
        Pnt::new(0.0, 0.0, 1.0),
        Pnt::new(1.0, 0.0, 0.0),
    );
    let r = p.mirrored_plane(ax2);
    near(r.x, 1.0, CONFUSION);
    near(r.y, 2.0, CONFUSION);
    near(r.z, -3.0, CONFUSION);
}

#[test]
fn pnt_transformed() {
    let p = Pnt::new(1.0, 0.0, 0.0);
    let mut t = Trsf::identity();
    t.set_translation(Pnt::new(0.0, 0.0, 5.0));
    let r = p.transformed(&t);
    near(r.x, 1.0, CONFUSION);
    near(r.y, 0.0, CONFUSION);
    near(r.z, 5.0, CONFUSION);
}

// ====================== gp_Dir_Test.cxx ======================

#[test]
fn dir_coordinate_constructor() {
    let d = Pnt::dir(0.0, 0.0, 1.0);
    near(d.x, 0.0, CONFUSION);
    near(d.y, 0.0, CONFUSION);
    near(d.z, 1.0, CONFUSION);
}

#[test]
fn dir_normalizes_on_construction() {
    // gp_Dir from (0,3,4) -> (0, 0.6, 0.8)
    let d = Pnt::dir(0.0, 3.0, 4.0);
    near(d.x, 0.0, CONFUSION);
    near(d.y, 0.6, CONFUSION);
    near(d.z, 0.8, CONFUSION);
}

#[test]
fn dir_from_vec() {
    let d = Pnt::dir(3.0, 0.0, 0.0);
    near(d.x, 1.0, CONFUSION);
    near(d.y, 0.0, CONFUSION);
    near(d.z, 0.0, CONFUSION);
}

#[test]
fn dir_angle() {
    let d1 = Pnt::dir(1.0, 0.0, 0.0);
    let d2 = Pnt::dir(0.0, 1.0, 0.0);
    near(d1.angle(d2), FRAC_PI_2, ANGULAR);
}

#[test]
fn dir_angle_with_ref() {
    let d1 = Pnt::dir(1.0, 0.0, 0.0);
    let d2 = Pnt::dir(0.0, 1.0, 0.0);
    near(
        d1.angle_with_ref(d2, Pnt::dir(0.0, 0.0, 1.0)),
        FRAC_PI_2,
        ANGULAR,
    );
    // Reversed reference negates the angle.
    near(
        d1.angle_with_ref(d2, Pnt::dir(0.0, 0.0, -1.0)),
        -FRAC_PI_2,
        ANGULAR,
    );
}

#[test]
fn dir_is_equal() {
    let d1 = Pnt::dir(1.0, 0.0, 0.0);
    let d2 = Pnt::dir(1.0, 0.0, 0.0);
    assert!(d1.is_equal_dir(d2, ANGULAR));
}

#[test]
fn dir_is_normal() {
    let d1 = Pnt::dir(1.0, 0.0, 0.0);
    let d2 = Pnt::dir(0.0, 1.0, 0.0);
    assert!(d1.is_normal(d2, ANGULAR));
}

#[test]
fn dir_is_opposite() {
    let d1 = Pnt::dir(1.0, 0.0, 0.0);
    let d2 = Pnt::dir(-1.0, 0.0, 0.0);
    assert!(d1.is_opposite(d2, ANGULAR));
}

#[test]
fn dir_is_parallel() {
    let d1 = Pnt::dir(1.0, 0.0, 0.0);
    let d2 = Pnt::dir(-1.0, 0.0, 0.0);
    assert!(d1.is_parallel(d2, ANGULAR));
}

#[test]
fn dir_predefined() {
    assert!(Pnt::dir(1.0, 0.0, 0.0).is_equal_dir(Pnt::new(1.0, 0.0, 0.0), ANGULAR));
    assert!(Pnt::dir(0.0, 1.0, 0.0).is_equal_dir(Pnt::new(0.0, 1.0, 0.0), ANGULAR));
    assert!(Pnt::dir(0.0, 0.0, 1.0).is_equal_dir(Pnt::new(0.0, 0.0, 1.0), ANGULAR));
}
