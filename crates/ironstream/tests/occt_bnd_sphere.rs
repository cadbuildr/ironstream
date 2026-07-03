//! Ported OpenCascade unit tests — `Bnd_Sphere`.
//!
//! Faithful Rust port of OCCT's `Bnd_Sphere_Test.cxx`
//! (Open-Cascade-SAS/OCCT, `src/FoundationClasses/TKMath/GTests/`). Same
//! numeric inputs, same expected values, same tolerances. OCCT's `gp_XYZ` maps
//! to [`ironstream::gp::Pnt`]; the class under test is
//! [`ironstream::bnd_sphere::BndSphere`].

use ironstream::bnd_sphere::BndSphere;
use ironstream::gp::Pnt;

/// Mirror of GTest's `EXPECT_DOUBLE_EQ`. All inputs in these tests are exact
/// integers or halves, so the expected results are exactly representable in
/// `f64`; we assert bit-exact equality just like the original.
fn double_eq(a: f64, b: f64) {
    assert_eq!(a, b, "expected {a} == {b}");
}

// ====================== Bnd_Sphere_Test.cxx ======================

#[test]
fn default_constructor() {
    let sphere = BndSphere::new();
    double_eq(sphere.center().x, 0.0);
    double_eq(sphere.center().y, 0.0);
    double_eq(sphere.center().z, 0.0);
    double_eq(sphere.radius(), 0.0);
    assert!(!sphere.is_valid());
    assert_eq!(sphere.u(), 0);
    assert_eq!(sphere.v(), 0);
}

#[test]
fn parameterized_constructor() {
    let center = Pnt::new(1.0, 2.0, 3.0);
    let sphere = BndSphere::with_center(center, 5.0, 10, 20);
    double_eq(sphere.center().x, 1.0);
    double_eq(sphere.center().y, 2.0);
    double_eq(sphere.center().z, 3.0);
    double_eq(sphere.radius(), 5.0);
    assert_eq!(sphere.u(), 10);
    assert_eq!(sphere.v(), 20);
}

#[test]
fn validity() {
    let mut sphere = BndSphere::new();
    assert!(!sphere.is_valid());
    sphere.set_valid(true);
    assert!(sphere.is_valid());
    sphere.set_valid(false);
    assert!(!sphere.is_valid());
}

#[test]
fn distance() {
    let center = Pnt::new(0.0, 0.0, 0.0);
    let sphere = BndSphere::with_center(center, 1.0, 0, 0);
    let point = Pnt::new(3.0, 4.0, 0.0);
    let dist = sphere.distance(point);
    double_eq(dist, 5.0);
}

#[test]
fn square_distance() {
    let center = Pnt::new(0.0, 0.0, 0.0);
    let sphere = BndSphere::with_center(center, 1.0, 0, 0);
    let point = Pnt::new(3.0, 4.0, 0.0);
    double_eq(sphere.square_distance(point), 25.0);
}

#[test]
fn distances_point_outside() {
    let center = Pnt::new(0.0, 0.0, 0.0);
    let sphere = BndSphere::with_center(center, 2.0, 0, 0);
    let point = Pnt::new(5.0, 0.0, 0.0);
    let (min, max) = sphere.distances(point);
    double_eq(min, 3.0);
    double_eq(max, 7.0);
}

#[test]
fn distances_point_inside() {
    let center = Pnt::new(0.0, 0.0, 0.0);
    let sphere = BndSphere::with_center(center, 5.0, 0, 0);
    let point = Pnt::new(1.0, 0.0, 0.0);
    let (min, max) = sphere.distances(point);
    double_eq(min, 0.0);
    double_eq(max, 6.0);
}

#[test]
fn square_distances_point_outside() {
    let center = Pnt::new(0.0, 0.0, 0.0);
    let sphere = BndSphere::with_center(center, 2.0, 0, 0);
    let point = Pnt::new(5.0, 0.0, 0.0);
    let (min, max) = sphere.square_distances(point);
    // d^2 = 25, r^2 = 4
    // min = d^2 - r^2 = 21, max = d^2 + r^2 = 29
    double_eq(min, 21.0);
    double_eq(max, 29.0);
}

#[test]
fn square_distances_point_inside() {
    let center = Pnt::new(0.0, 0.0, 0.0);
    let sphere = BndSphere::with_center(center, 5.0, 0, 0);
    let point = Pnt::new(1.0, 0.0, 0.0);
    let (min, max) = sphere.square_distances(point);
    // d^2 = 1, r^2 = 25
    // d^2 < r^2 -> min = 0, max = d^2 + r^2 = 26
    double_eq(min, 0.0);
    double_eq(max, 26.0);
}

#[test]
fn square_distances_point_at_center() {
    let center = Pnt::new(0.0, 0.0, 0.0);
    let sphere = BndSphere::with_center(center, 3.0, 0, 0);
    let point = Pnt::new(0.0, 0.0, 0.0);
    let (min, max) = sphere.square_distances(point);
    // d^2 = 0, r^2 = 9
    // d^2 < r^2 -> min = 0, max = 0 + 9 = 9
    double_eq(min, 0.0);
    double_eq(max, 9.0);
}

#[test]
fn square_distances_point_on_surface() {
    let center = Pnt::new(0.0, 0.0, 0.0);
    let sphere = BndSphere::with_center(center, 3.0, 0, 0);
    let point = Pnt::new(3.0, 0.0, 0.0);
    let (min, max) = sphere.square_distances(point);
    // d^2 = 9, r^2 = 9
    // d^2 < r^2 is false (equal), so min = d^2 - r^2 = 0, max = d^2 + r^2 = 18
    double_eq(min, 0.0);
    double_eq(max, 18.0);
}

#[test]
fn project() {
    let center = Pnt::new(1.0, 2.0, 3.0);
    let sphere = BndSphere::with_center(center, 5.0, 0, 0);
    let node = Pnt::new(10.0, 20.0, 30.0);
    let (proj_node, _dist, inside, is_ok) = sphere.project(node);
    assert!(is_ok);
    assert!(inside);
    double_eq(proj_node.x, center.x);
    double_eq(proj_node.y, center.y);
    double_eq(proj_node.z, center.z);
}

#[test]
fn add_enclosing_sphere() {
    let mut sphere1 = BndSphere::with_center(Pnt::new(0.0, 0.0, 0.0), 10.0, 0, 0);
    let sphere2 = BndSphere::with_center(Pnt::new(1.0, 0.0, 0.0), 2.0, 0, 0);
    sphere1.add(&sphere2);
    // sphere1 already encloses sphere2, should remain unchanged
    double_eq(sphere1.radius(), 10.0);
}

#[test]
fn add_enclosed_by_sphere() {
    let mut sphere1 = BndSphere::with_center(Pnt::new(0.0, 0.0, 0.0), 2.0, 0, 0);
    let sphere2 = BndSphere::with_center(Pnt::new(0.0, 0.0, 0.0), 10.0, 0, 0);
    sphere1.add(&sphere2);
    // sphere2 encloses sphere1, should take sphere2
    double_eq(sphere1.radius(), 10.0);
}

#[test]
fn add_partial_overlap() {
    let mut sphere1 = BndSphere::with_center(Pnt::new(0.0, 0.0, 0.0), 3.0, 0, 0);
    let sphere2 = BndSphere::with_center(Pnt::new(5.0, 0.0, 0.0), 3.0, 0, 0);
    sphere1.add(&sphere2);
    // Combined radius should be (5 + 3 + 3) / 2 = 5.5
    double_eq(sphere1.radius(), 5.5);
}

#[test]
fn is_out_separated() {
    let sphere1 = BndSphere::with_center(Pnt::new(0.0, 0.0, 0.0), 1.0, 0, 0);
    let sphere2 = BndSphere::with_center(Pnt::new(10.0, 0.0, 0.0), 1.0, 0, 0);
    assert!(sphere1.is_out_sphere(&sphere2));
}

#[test]
fn is_out_overlapping() {
    let sphere1 = BndSphere::with_center(Pnt::new(0.0, 0.0, 0.0), 3.0, 0, 0);
    let sphere2 = BndSphere::with_center(Pnt::new(4.0, 0.0, 0.0), 3.0, 0, 0);
    assert!(!sphere1.is_out_sphere(&sphere2));
}

#[test]
fn is_out_point_with_max_dist() {
    let mut sphere = BndSphere::with_center(Pnt::new(0.0, 0.0, 0.0), 5.0, 0, 0);
    let point = Pnt::new(20.0, 0.0, 0.0);
    let mut max_dist = 100.0;
    sphere.set_valid(true);
    assert!(!sphere.is_out_point(point, &mut max_dist));
    // max_dist should be updated to cur_max_dist = 20 + 5 = 25
    double_eq(max_dist, 25.0);
}

#[test]
fn is_out_point_too_far() {
    let sphere = BndSphere::with_center(Pnt::new(0.0, 0.0, 0.0), 1.0, 0, 0);
    let point = Pnt::new(20.0, 0.0, 0.0);
    let mut max_dist = 10.0;
    assert!(sphere.is_out_point(point, &mut max_dist));
}

#[test]
fn square_extent() {
    let sphere = BndSphere::with_center(Pnt::new(0.0, 0.0, 0.0), 3.0, 0, 0);
    double_eq(sphere.square_extent(), 36.0);
}
