// FILE: tests/occt_gp_lin.rs
extern crate ironstream;

use ironstream::gp::{Ax1, Ax3, Pnt, Trsf};
use ironstream::gp_lin::Lin;
use ironstream::precision::{ANGULAR, CONFUSION};
use std::f64::consts::{FRAC_PI_2, PI};

fn near(a: f64, b: f64, tol: f64) {
    assert!(
        (a - b).abs() <= tol,
        "expected {a} ≈ {b} (tol {tol:.2e})"
    );
}

fn pnt_near(a: Pnt, b: Pnt, tol: f64) {
    let d = (a - b).norm();
    assert!(d <= tol, "expected {a:?} ≈ {b:?} (tol {tol:.2e})");
}

/// Standard fixture: X-axis line through origin.
fn x_line() -> Lin {
    Lin::from_point_dir(Pnt::origin(), Pnt::new(1.0, 0.0, 0.0))
}

// TEST(gp_Lin, ConstructorFromAxis)
#[test]
fn constructor_from_axis() {
    let ax = Ax1::new(Pnt::new(1.0, 2.0, 3.0), Pnt::new(0.0, 0.0, 5.0));
    let l = Lin::new(ax);
    near(l.location().x, 1.0, CONFUSION);
    near(l.location().y, 2.0, CONFUSION);
    near(l.location().z, 3.0, CONFUSION);
    near(l.direction().z, 1.0, ANGULAR);
}

// TEST(gp_Lin, ConstructorFromPointDir)
#[test]
fn constructor_from_point_dir_normalises() {
    let l = Lin::from_point_dir(Pnt::new(1.0, 0.0, 0.0), Pnt::new(3.0, 4.0, 0.0));
    let mag = l.direction().norm();
    near(mag, 1.0, ANGULAR);
}

// TEST(gp_Lin, DistancePointOnLine)
#[test]
fn distance_point_on_line_is_zero() {
    let l = x_line();
    near(l.distance(Pnt::new(7.0, 0.0, 0.0)), 0.0, CONFUSION);
    near(l.distance(Pnt::new(-100.0, 0.0, 0.0)), 0.0, CONFUSION);
}

// TEST(gp_Lin, DistancePointOffLine)
#[test]
fn distance_point_off_line() {
    // X-axis line; point (0, 3, 4) is 5 units away (Pythagoras).
    let l = x_line();
    near(l.distance(Pnt::new(0.0, 3.0, 4.0)), 5.0, CONFUSION);
    near(l.distance(Pnt::new(10.0, 0.0, 2.0)), 2.0, CONFUSION);
}

// TEST(gp_Lin, SquareDistance)
#[test]
fn square_distance_equals_distance_squared() {
    let l = x_line();
    let p = Pnt::new(0.0, 3.0, 4.0);
    let d = l.distance(p);
    near(l.square_distance(p), d * d, CONFUSION);
}

// TEST(gp_Lin, ContainsPoint)
#[test]
fn contains_on_and_off_line() {
    let l = x_line();
    assert!(l.contains(Pnt::new(42.0, 0.0, 0.0), CONFUSION));
    assert!(!l.contains(Pnt::new(0.0, 1.0, 0.0), CONFUSION));
    assert!(l.contains(Pnt::new(0.0, CONFUSION * 0.5, 0.0), CONFUSION));
}

// TEST(gp_Lin, AngleBetweenPerpendicularLines)
#[test]
fn angle_perpendicular_lines_is_pi_over_2() {
    let l1 = x_line();
    let l2 = Lin::from_point_dir(Pnt::origin(), Pnt::new(0.0, 1.0, 0.0));
    near(l1.angle(&l2), FRAC_PI_2, ANGULAR);
}

// TEST(gp_Lin, AngleAntiParallel)
#[test]
fn angle_anti_parallel_lines_is_pi() {
    let l1 = x_line();
    let l2 = l1.reversed();
    near(l1.angle(&l2), PI, ANGULAR);
}

// TEST(gp_Lin, Reverse)
#[test]
fn reverse_negates_direction_keeps_location() {
    let mut l = Lin::from_point_dir(Pnt::new(1.0, 2.0, 3.0), Pnt::new(1.0, 0.0, 0.0));
    l.reverse();
    near(l.direction().x, -1.0, ANGULAR);
    near(l.location().x, 1.0, CONFUSION);
    near(l.location().y, 2.0, CONFUSION);
}

// TEST(gp_Lin, Translate)
#[test]
fn translate_shifts_location_keeps_direction() {
    let l = x_line().translated(Pnt::new(0.0, 3.0, 7.0));
    near(l.location().y, 3.0, CONFUSION);
    near(l.location().z, 7.0, CONFUSION);
    near(l.direction().x, 1.0, ANGULAR);
}

// TEST(gp_Lin, Rotate90AboutZ)
#[test]
fn rotate_90_about_z_axis() {
    // X-axis line with loc at (1,0,0) rotating 90° CCW about Z → loc at (0,1,0), dir +Y.
    let l = Lin::from_point_dir(Pnt::new(1.0, 0.0, 0.0), Pnt::new(1.0, 0.0, 0.0));
    let z_ax = Ax1::new(Pnt::origin(), Pnt::new(0.0, 0.0, 1.0));
    let r = l.rotated(z_ax, FRAC_PI_2);
    near(r.location().x, 0.0, CONFUSION);
    near(r.location().y, 1.0, CONFUSION);
    near(r.direction().x, 0.0, ANGULAR);
    near(r.direction().y, 1.0, ANGULAR);
}

// TEST(gp_Lin, ScaleAboutOrigin)
#[test]
fn scale_about_origin_scales_location() {
    let l = Lin::from_point_dir(Pnt::new(2.0, 0.0, 0.0), Pnt::new(1.0, 0.0, 0.0));
    let s = l.scaled(Pnt::origin(), 3.0);
    near(s.location().x, 6.0, CONFUSION);
    near(s.direction().x, 1.0, ANGULAR);
}

// TEST(gp_Lin, ScaleNegativeFlipsDirection)
#[test]
fn scale_negative_flips_direction() {
    let l = Lin::from_point_dir(Pnt::new(1.0, 0.0, 0.0), Pnt::new(1.0, 0.0, 0.0));
    let s = l.scaled(Pnt::origin(), -1.0);
    near(s.location().x, -1.0, CONFUSION);
    near(s.direction().x, -1.0, ANGULAR);
}

// TEST(gp_Lin, IsParallel)
#[test]
fn is_parallel_same_direction_offset() {
    let l1 = x_line();
    let l2 = Lin::from_point_dir(Pnt::new(0.0, 5.0, 0.0), Pnt::new(2.0, 0.0, 0.0));
    assert!(l1.is_parallel(&l2, 1e-10));
    assert!(!l1.is_normal(&l2, 1e-10));
}

// TEST(gp_Lin, IsNormal)
#[test]
fn is_normal_perpendicular_lines() {
    let l1 = x_line();
    let l2 = Lin::from_point_dir(Pnt::origin(), Pnt::new(0.0, 1.0, 0.0));
    assert!(l1.is_normal(&l2, 1e-10));
    assert!(!l1.is_parallel(&l2, 1e-10));
}

// TEST(gp_Lin, IsOpposite)
#[test]
fn is_opposite_reversed_direction() {
    let l1 = x_line();
    let l2 = Lin::from_point_dir(Pnt::new(3.0, 0.0, 0.0), Pnt::new(-1.0, 0.0, 0.0));
    assert!(l1.is_opposite(&l2, 1e-10));
    assert!(l1.is_parallel(&l2, 1e-10));
}

// TEST(gp_Lin, IsCoaxial)
#[test]
fn is_coaxial_same_line_different_origin() {
    let l1 = x_line();
    let l2 = Lin::from_point_dir(Pnt::new(5.0, 0.0, 0.0), Pnt::new(2.0, 0.0, 0.0));
    assert!(l1.is_coaxial(&l2, CONFUSION, 1e-10));
    // Parallel but offset — not coaxial.
    let l3 = Lin::from_point_dir(Pnt::new(0.0, 1.0, 0.0), Pnt::new(1.0, 0.0, 0.0));
    assert!(!l1.is_coaxial(&l3, CONFUSION, 1e-10));
}

// TEST(gp_Lin, DistanceToParallelLine)
#[test]
fn distance_to_parallel_line_is_offset() {
    let l1 = x_line();
    let l2 = Lin::from_point_dir(Pnt::new(0.0, 0.0, 5.0), Pnt::new(1.0, 0.0, 0.0));
    near(l1.distance_to_line(&l2), 5.0, CONFUSION);
}

// TEST(gp_Lin, DistanceToSkewLines)
#[test]
fn distance_between_skew_lines() {
    // X-axis at z=0 and Y-axis at z=1: these are skew, distance = 1.
    let l1 = Lin::from_point_dir(Pnt::new(0.0, 0.0, 0.0), Pnt::new(1.0, 0.0, 0.0));
    let l2 = Lin::from_point_dir(Pnt::new(0.0, 0.0, 1.0), Pnt::new(0.0, 1.0, 0.0));
    near(l1.distance_to_line(&l2), 1.0, CONFUSION);
}

// TEST(gp_Lin, DistanceToIntersectingLine)
#[test]
fn distance_to_intersecting_line_is_zero() {
    let l1 = x_line();
    let l2 = Lin::from_point_dir(Pnt::origin(), Pnt::new(0.0, 1.0, 1.0));
    near(l1.distance_to_line(&l2), 0.0, CONFUSION);
}

// TEST(gp_Lin, NormalAtPoint)
#[test]
fn normal_passes_through_point_and_is_perpendicular() {
    let l = x_line();
    let p = Pnt::new(3.0, 4.0, 0.0);
    let n = l.normal(p);
    pnt_near(n.location(), p, CONFUSION);
    near(n.direction().dot(l.direction()), 0.0, ANGULAR);
}

// TEST(gp_Lin, MirrorPoint)
#[test]
fn mirror_point_symmetry_through_origin() {
    let l = Lin::from_point_dir(Pnt::new(2.0, 1.0, 0.0), Pnt::new(1.0, 0.0, 0.0));
    let m = l.mirrored_point(Pnt::origin());
    near(m.location().x, -2.0, CONFUSION);
    near(m.location().y, -1.0, CONFUSION);
    near(m.direction().x, -1.0, ANGULAR);
}

// TEST(gp_Lin, MirrorAxis)
#[test]
fn mirror_axis_reflects_across_z_axis() {
    // Reflect line at (2,0,0) dir +X across Z axis (loc=origin, dir=+Z).
    // Householder: loc (2,0,0) → 2*(2,0,0)·(0,0,1)*(0,0,1) - (2,0,0) = (-2,0,0).
    // Direction +X → -X.
    let l = Lin::from_point_dir(Pnt::new(2.0, 0.0, 0.0), Pnt::new(1.0, 0.0, 0.0));
    let z_ax = Ax1::new(Pnt::origin(), Pnt::new(0.0, 0.0, 1.0));
    let m = l.mirrored_axis(z_ax);
    near(m.location().x, -2.0, CONFUSION);
    near(m.location().y, 0.0, CONFUSION);
    near(m.direction().x, -1.0, ANGULAR);
}

// TEST(gp_Lin, MirrorPlane)
#[test]
fn mirror_plane_reflects_across_xy_plane() {
    // Line at (1,2,3) dir +Z; reflected across XY plane → loc (1,2,-3), dir -Z.
    let l = Lin::from_point_dir(Pnt::new(1.0, 2.0, 3.0), Pnt::new(0.0, 0.0, 1.0));
    let xy = Ax3::identity();
    let m = l.mirrored_plane(xy);
    near(m.location().x, 1.0, CONFUSION);
    near(m.location().y, 2.0, CONFUSION);
    near(m.location().z, -3.0, CONFUSION);
    near(m.direction().z, -1.0, ANGULAR);
}

// TEST(gp_Lin, Transform)
#[test]
fn transform_with_translation_trsf() {
    let l = x_line();
    let t = Trsf::translation(Pnt::new(0.0, 0.0, 5.0));
    let tl = l.transformed(&t);
    near(tl.location().z, 5.0, CONFUSION);
    near(tl.direction().x, 1.0, ANGULAR);
}

// TEST(gp_Lin, SettersUpdate)
#[test]
fn setters_update_position() {
    let mut l = x_line();
    l.set_location(Pnt::new(10.0, -5.0, 2.0));
    near(l.location().x, 10.0, CONFUSION);
    near(l.location().y, -5.0, CONFUSION);
    l.set_direction(Pnt::new(0.0, 0.0, 3.0));
    near(l.direction().z, 1.0, ANGULAR);
    near(l.direction().x, 0.0, ANGULAR);
}
