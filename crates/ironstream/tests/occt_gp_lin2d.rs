// FILE: tests/occt_gp_lin2d.rs
//! Ported OpenCascade unit tests — `gp_Lin2d`.
//!
//! Faithful Rust ports of OCCT's `gp_Lin2d` tests. Same numeric inputs,
//! expected values, and tolerances as upstream. The standard fixture is a line
//! along the X axis through the origin.
//!
//! OCCT helper-type mapping:
//! - `gp_Pnt2d`  -> [`ironstream::gp2d::Pnt2d`]
//! - `gp_Dir2d`  -> [`ironstream::gp2d::Pnt2d`] (unit vector)
//! - `gp_Ax2d`   -> [`ironstream::gp_ax2d::Ax2d`]
//! - `gp_Trsf2d` -> [`ironstream::gp_trsf2d::Trsf2d`]
//! - `gp_Lin2d`  -> [`ironstream::gp_lin2d::Lin2d`]

extern crate ironstream;

use ironstream::gp2d::{Ax2d, Pnt2d, Trsf2d};
use ironstream::gp_lin2d::Lin2d;
use ironstream::precision::{ANGULAR, CONFUSION};
use std::f64::consts::{FRAC_PI_2, FRAC_PI_4, PI};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn near(a: f64, b: f64, tol: f64) {
    assert!(
        (a - b).abs() <= tol,
        "expected {a} ≈ {b} (tol {tol:.2e})"
    );
}

fn pnt_near(a: Pnt2d, b: Pnt2d, tol: f64) {
    assert!(
        (a - b).norm() <= tol,
        "expected {a:?} ≈ {b:?} (tol {tol:.2e})"
    );
}

/// Standard fixture: line along the X axis through the origin.
fn x_line() -> Lin2d {
    Lin2d::from_point_dir(Pnt2d::origin(), Pnt2d::new(1.0, 0.0))
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

// TEST(gp_Lin2d, ConstructorFromAxis)
#[test]
fn constructor_from_axis() {
    let a = Ax2d::new(Pnt2d::new(2.0, 3.0), Pnt2d::new(0.0, 5.0));
    let l = Lin2d::new(a);
    near(l.location().x, 2.0, CONFUSION);
    near(l.location().y, 3.0, CONFUSION);
    // Direction normalised to (0, 1)
    near(l.direction().x, 0.0, ANGULAR);
    near(l.direction().y, 1.0, ANGULAR);
}

// TEST(gp_Lin2d, ConstructorFromPointDir)
#[test]
fn constructor_from_point_dir_normalises() {
    let l = Lin2d::from_point_dir(Pnt2d::new(1.0, -1.0), Pnt2d::new(3.0, 4.0));
    near(l.location().x, 1.0, CONFUSION);
    near(l.location().y, -1.0, CONFUSION);
    let mag = (l.direction().x * l.direction().x + l.direction().y * l.direction().y).sqrt();
    near(mag, 1.0, ANGULAR);
}

// TEST(gp_Lin2d, DistancePointOnLine)
#[test]
fn distance_point_on_line_is_zero() {
    let l = x_line();
    near(l.distance(Pnt2d::new(7.0, 0.0)), 0.0, CONFUSION);
    near(l.distance(Pnt2d::new(-100.0, 0.0)), 0.0, CONFUSION);
}

// TEST(gp_Lin2d, DistancePointOffLine)
#[test]
fn distance_point_off_line() {
    // X-axis line; point (5, 3) is 3 units above.
    let l = x_line();
    near(l.distance(Pnt2d::new(5.0, 3.0)), 3.0, CONFUSION);
    near(l.distance(Pnt2d::new(-2.0, -4.0)), 4.0, CONFUSION);
}

// TEST(gp_Lin2d, SquareDistance)
#[test]
fn square_distance_is_distance_squared() {
    let l = x_line();
    let p = Pnt2d::new(1.0, 5.0);
    let d = l.distance(p);
    near(l.square_distance(p), d * d, CONFUSION);
}

// TEST(gp_Lin2d, ContainsPoint)
#[test]
fn contains_on_and_off_line() {
    let l = x_line();
    assert!(l.contains(Pnt2d::new(42.0, 0.0), CONFUSION));
    assert!(!l.contains(Pnt2d::new(0.0, 1.0), CONFUSION));
    // Within tolerance boundary
    assert!(l.contains(Pnt2d::new(0.0, CONFUSION * 0.5), CONFUSION));
}

// TEST(gp_Lin2d, NormalAtPoint)
#[test]
fn normal_passes_through_point_and_is_perpendicular() {
    let l = x_line();
    let p = Pnt2d::new(4.0, 0.0);
    let n = l.normal(p);
    // Normal passes through p
    pnt_near(n.location(), p, CONFUSION);
    // Normal direction is perpendicular to the line direction
    let dot = n.direction().x * l.direction().x + n.direction().y * l.direction().y;
    near(dot, 0.0, ANGULAR);
}

// TEST(gp_Lin2d, Reverse)
#[test]
fn reverse_negates_direction_keeps_location() {
    let mut l = Lin2d::from_point_dir(Pnt2d::new(1.0, 2.0), Pnt2d::new(1.0, 0.0));
    l.reverse();
    near(l.direction().x, -1.0, ANGULAR);
    near(l.direction().y, 0.0, ANGULAR);
    near(l.location().x, 1.0, CONFUSION);
    near(l.location().y, 2.0, CONFUSION);
}

// TEST(gp_Lin2d, ReversedFunctional)
#[test]
fn reversed_returns_copy_with_negated_direction() {
    let l = x_line();
    let r = l.reversed();
    near(r.direction().x, -1.0, ANGULAR);
    near(r.direction().y, 0.0, ANGULAR);
    // Original unchanged
    near(l.direction().x, 1.0, ANGULAR);
}

// TEST(gp_Lin2d, AngleBetweenLines)
#[test]
fn angle_between_perpendicular_lines() {
    let l1 = x_line();
    let l2 = Lin2d::from_point_dir(Pnt2d::origin(), Pnt2d::new(0.0, 1.0));
    near(l1.angle(&l2), FRAC_PI_2, ANGULAR);
    near(l2.angle(&l1), -FRAC_PI_2, ANGULAR);
}

// TEST(gp_Lin2d, Angle45Degrees)
#[test]
fn angle_45_degrees() {
    let l1 = x_line();
    let l2 = Lin2d::from_point_dir(Pnt2d::origin(), Pnt2d::new(1.0, 1.0));
    near(l1.angle(&l2), FRAC_PI_4, ANGULAR);
}

// TEST(gp_Lin2d, Angle180Degrees)
#[test]
fn angle_anti_parallel_lines() {
    let l1 = x_line();
    let l2 = l1.reversed();
    near(l1.angle(&l2).abs(), PI, ANGULAR);
}

// TEST(gp_Lin2d, IsParallel)
#[test]
fn is_parallel_same_and_anti_parallel() {
    let l1 = x_line();
    let l2 = Lin2d::from_point_dir(Pnt2d::new(0.0, 5.0), Pnt2d::new(2.0, 0.0));
    let l3 = Lin2d::from_point_dir(Pnt2d::origin(), Pnt2d::new(-1.0, 0.0));
    assert!(l1.is_parallel(&l2, 1e-10));
    assert!(l1.is_parallel(&l3, 1e-10));
}

// TEST(gp_Lin2d, IsNormal)
#[test]
fn is_normal_perpendicular_lines() {
    let l1 = x_line();
    let l2 = Lin2d::from_point_dir(Pnt2d::origin(), Pnt2d::new(0.0, 1.0));
    assert!(l1.is_normal(&l2, 1e-10));
    assert!(!l1.is_parallel(&l2, 1e-10));
}

// TEST(gp_Lin2d, IsOpposite)
#[test]
fn is_opposite_reversed_direction() {
    let l1 = x_line();
    let l2 = Lin2d::from_point_dir(Pnt2d::new(3.0, 0.0), Pnt2d::new(-1.0, 0.0));
    assert!(l1.is_opposite(&l2, 1e-10));
    assert!(l1.is_parallel(&l2, 1e-10));
    assert!(!l1.is_normal(&l2, 1e-10));
}

// TEST(gp_Lin2d, IsCoaxial)
#[test]
fn is_coaxial_coincident_lines() {
    let l1 = x_line();
    // Same line, different origin and direction magnitude — should be coaxial.
    let l2 = Lin2d::from_point_dir(Pnt2d::new(5.0, 0.0), Pnt2d::new(2.0, 0.0));
    assert!(l1.is_coaxial(&l2, CONFUSION, 1e-10));
    // Parallel but offset — not coaxial.
    let l3 = Lin2d::from_point_dir(Pnt2d::new(0.0, 1.0), Pnt2d::new(1.0, 0.0));
    assert!(!l1.is_coaxial(&l3, CONFUSION, 1e-10));
}

// TEST(gp_Lin2d, Translate)
#[test]
fn translate_shifts_origin_keeps_direction() {
    let l = x_line().translated(Pnt2d::new(3.0, 7.0));
    near(l.location().x, 3.0, CONFUSION);
    near(l.location().y, 7.0, CONFUSION);
    near(l.direction().x, 1.0, ANGULAR);
    near(l.direction().y, 0.0, ANGULAR);
}

// TEST(gp_Lin2d, Rotate90)
#[test]
fn rotate_90_about_origin() {
    // Line at (1,0) pointing +X → after 90° CCW about origin: loc at (0,1), dir +Y.
    let l = Lin2d::from_point_dir(Pnt2d::new(1.0, 0.0), Pnt2d::new(1.0, 0.0));
    let r = l.rotated(Pnt2d::origin(), FRAC_PI_2);
    near(r.location().x, 0.0, CONFUSION);
    near(r.location().y, 1.0, CONFUSION);
    near(r.direction().x, 0.0, ANGULAR);
    near(r.direction().y, 1.0, ANGULAR);
}

// TEST(gp_Lin2d, MirrorPoint)
#[test]
fn mirror_point_symmetry_through_origin() {
    // Line at (2,0) dir +X; mirror through origin → loc (-2,0), dir -X.
    let l = Lin2d::from_point_dir(Pnt2d::new(2.0, 0.0), Pnt2d::new(1.0, 0.0));
    let m = l.mirrored_point(Pnt2d::origin());
    near(m.location().x, -2.0, CONFUSION);
    near(m.location().y, 0.0, CONFUSION);
    near(m.direction().x, -1.0, ANGULAR);
    near(m.direction().y, 0.0, ANGULAR);
}

// TEST(gp_Lin2d, MirrorAxis)
#[test]
fn mirror_axis_across_y_axis() {
    // Reflect line at (2,1) dir +X across the Y axis (loc origin, dir +Y).
    // The X axis symmetry: loc (2,1) → (-2,1), dir +X → -X.
    let l = Lin2d::from_point_dir(Pnt2d::new(2.0, 1.0), Pnt2d::new(1.0, 0.0));
    let y_axis = Lin2d::from_point_dir(Pnt2d::origin(), Pnt2d::new(0.0, 1.0));
    let m = l.mirrored_axis(&y_axis);
    near(m.location().x, -2.0, CONFUSION);
    near(m.location().y, 1.0, CONFUSION);
    near(m.direction().x.abs(), 1.0, ANGULAR);
}

// TEST(gp_Lin2d, Scale)
#[test]
fn scale_about_origin_scales_location() {
    // Line at (2,0) dir +X; scale by 3 about origin → loc (6,0), dir unchanged.
    let l = Lin2d::from_point_dir(Pnt2d::new(2.0, 0.0), Pnt2d::new(1.0, 0.0));
    let s = l.scaled(Pnt2d::origin(), 3.0);
    near(s.location().x, 6.0, CONFUSION);
    near(s.location().y, 0.0, CONFUSION);
    near(s.direction().x, 1.0, ANGULAR);
    near(s.direction().y, 0.0, ANGULAR);
}

// TEST(gp_Lin2d, ScaleNegativeFlipsDirection)
#[test]
fn scale_negative_flips_direction() {
    // Line at (1,0) dir +X; scale by -1 about origin → loc (-1,0), dir -X.
    let l = Lin2d::from_point_dir(Pnt2d::new(1.0, 0.0), Pnt2d::new(1.0, 0.0));
    let s = l.scaled(Pnt2d::origin(), -1.0);
    near(s.location().x, -1.0, CONFUSION);
    near(s.direction().x, -1.0, ANGULAR);
}

// TEST(gp_Lin2d, DistanceToParallelLine)
#[test]
fn distance_to_parallel_line_is_offset() {
    // Two horizontal lines: y=0 and y=5 → distance = 5.
    let l1 = x_line();
    let l2 = Lin2d::from_point_dir(Pnt2d::new(0.0, 5.0), Pnt2d::new(1.0, 0.0));
    near(l1.distance_to_line(&l2), 5.0, CONFUSION);
}

// TEST(gp_Lin2d, DistanceToIntersectingLine)
#[test]
fn distance_to_intersecting_line_is_zero() {
    let l1 = x_line();
    let l2 = Lin2d::from_point_dir(Pnt2d::origin(), Pnt2d::new(1.0, 1.0));
    near(l1.distance_to_line(&l2), 0.0, CONFUSION);
}

// TEST(gp_Lin2d, SetDirectionUpdates)
#[test]
fn set_direction_updates_unit_dir() {
    let mut l = x_line();
    l.set_direction(Pnt2d::new(0.0, 3.0)); // normalised to (0,1)
    near(l.direction().x, 0.0, ANGULAR);
    near(l.direction().y, 1.0, ANGULAR);
}

// TEST(gp_Lin2d, SetLocationUpdates)
#[test]
fn set_location_updates_origin() {
    let mut l = x_line();
    l.set_location(Pnt2d::new(10.0, -5.0));
    near(l.location().x, 10.0, CONFUSION);
    near(l.location().y, -5.0, CONFUSION);
    near(l.direction().x, 1.0, ANGULAR);
}

// TEST(gp_Lin2d, TransformWithRotation)
#[test]
fn transform_with_rotation_trsf2d() {
    // Line at (2,0) dir +X; apply 90° rotation about origin via Trsf2d.
    let l = Lin2d::from_point_dir(Pnt2d::new(2.0, 0.0), Pnt2d::new(1.0, 0.0));
    let t = Trsf2d::rotation(Pnt2d::origin(), FRAC_PI_2);
    let tl = l.transformed(&t);
    near(tl.location().x, 0.0, CONFUSION);
    near(tl.location().y, 2.0, CONFUSION);
    near(tl.direction().x, 0.0, ANGULAR);
    near(tl.direction().y, 1.0, ANGULAR);
}

// TEST(gp_Lin2d, NormalDistanceProperty)
#[test]
fn normal_at_point_distance_equals_zero_from_line() {
    // The foot of the perpendicular from any point P onto the line lies ON the
    // line.  We verify that the location of the normal line (which is P itself)
    // satisfies: the foot point is on the line.
    //
    // For the X-axis line and point P=(3,4), the foot is (3,0) which lies on
    // the X axis.  We verify that the normal line through P=(3,4) intersects
    // the X axis at (3,0), i.e. the X-axis line contains (3,0).
    let l = x_line();
    let p = Pnt2d::new(3.0, 4.0);
    let n = l.normal(p);
    // Foot: project P onto l:  foot = loc + ((P - loc) . dir) * dir
    let dir = l.direction();
    let rel = p - l.location();
    let proj = rel.x * dir.x + rel.y * dir.y;
    let foot = l.location() + dir * proj;
    // Foot must lie on the original line.
    near(l.distance(foot), 0.0, CONFUSION);
    // The distance from P to the foot equals the distance from P to l.
    let dist_p_foot = (p - foot).norm();
    near(dist_p_foot, l.distance(p), CONFUSION);
    // Foot must lie on the normal line too (within its reach).
    near(n.distance(foot), 0.0, CONFUSION);
}
