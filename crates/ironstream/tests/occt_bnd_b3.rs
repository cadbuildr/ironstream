//! Ported OpenCascade unit tests — `Bnd_B3` (3D bounding box).
//!
//! Faithful Rust port of OCCT's `Bnd_B3_Test.cxx`
//! (Open-Cascade-SAS/OCCT, `src/FoundationClasses/TKMath/GTests/`). Same
//! numeric inputs, expected values and tolerances (`Precision::Confusion()`).
//! `Bnd_B3d` -> [`BndB3d`], `Bnd_B3f` -> [`BndB3f`], `gp_XYZ`/`gp_Pnt` ->
//! `Pnt`, `gp_Ax1` -> `Ax1`, `gp_Ax3` -> `Ax3`, `gp_Trsf` -> `Trsf`.

use ironstream::bnd_b3::{BndB3d, BndB3f};
use ironstream::gp::{Ax1, Ax3, Pnt, Trsf};
use ironstream::precision::CONFUSION;
use std::f64::consts::PI;

fn near(a: f64, b: f64, tol: f64) {
    assert!((a - b).abs() <= tol, "expected {a} ≈ {b} (tol {tol})");
}

/// `gp_Ax3(loc, dir)` builds a frame whose main direction is the Z axis (the
/// plane normal). IronStream's `Ax3::from_origin_normal` does the same.
fn plane(loc: Pnt, normal: Pnt) -> Ax3 {
    Ax3::from_origin_normal(loc, normal, normal.any_perpendicular())
}

// ====================== Bnd_B3_Test.cxx ======================

#[test]
fn default_constructor() {
    let a_box = BndB3d::new();
    assert!(a_box.is_void());
}

#[test]
fn constructor_with_center_and_hsize() {
    let center = Pnt::new(5.0, 10.0, 15.0);
    let hsize = Pnt::new(2.0, 3.0, 4.0);

    let a_box = BndB3d::from_center_hsize(center, hsize);

    assert!(!a_box.is_void());

    let a_min = a_box.corner_min();
    let a_max = a_box.corner_max();

    near(a_min.x, 3.0, CONFUSION);
    near(a_min.y, 7.0, CONFUSION);
    near(a_min.z, 11.0, CONFUSION);
    near(a_max.x, 7.0, CONFUSION);
    near(a_max.y, 13.0, CONFUSION);
    near(a_max.z, 19.0, CONFUSION);
}

#[test]
fn clear() {
    let mut a_box = BndB3d::from_center_hsize(Pnt::new(0.0, 0.0, 0.0), Pnt::new(1.0, 1.0, 1.0));
    assert!(!a_box.is_void());

    a_box.clear();
    assert!(a_box.is_void());
}

#[test]
fn add_point() {
    let mut a_box = BndB3d::new();

    a_box.add(Pnt::new(1.0, 2.0, 3.0));
    assert!(!a_box.is_void());

    let mut a_min = a_box.corner_min();
    let mut a_max = a_box.corner_max();
    near(a_min.x, 1.0, CONFUSION);
    near(a_min.y, 2.0, CONFUSION);
    near(a_min.z, 3.0, CONFUSION);
    near(a_max.x, 1.0, CONFUSION);
    near(a_max.y, 2.0, CONFUSION);
    near(a_max.z, 3.0, CONFUSION);

    a_box.add(Pnt::new(4.0, 5.0, 6.0));
    a_min = a_box.corner_min();
    a_max = a_box.corner_max();
    near(a_min.x, 1.0, CONFUSION);
    near(a_min.y, 2.0, CONFUSION);
    near(a_min.z, 3.0, CONFUSION);
    near(a_max.x, 4.0, CONFUSION);
    near(a_max.y, 5.0, CONFUSION);
    near(a_max.z, 6.0, CONFUSION);
}

#[test]
fn add_pnt() {
    let mut a_box = BndB3d::new();
    a_box.add(Pnt::new(1.0, 2.0, 3.0));

    assert!(!a_box.is_void());
    let a_min = a_box.corner_min();
    near(a_min.x, 1.0, CONFUSION);
    near(a_min.y, 2.0, CONFUSION);
    near(a_min.z, 3.0, CONFUSION);
}

#[test]
fn add_box() {
    let mut a_box1 = BndB3d::from_center_hsize(Pnt::new(0.0, 0.0, 0.0), Pnt::new(1.0, 1.0, 1.0));
    let a_box2 = BndB3d::from_center_hsize(Pnt::new(3.0, 3.0, 3.0), Pnt::new(1.0, 1.0, 1.0));

    a_box1.add_box(&a_box2);

    let a_min = a_box1.corner_min();
    let a_max = a_box1.corner_max();
    near(a_min.x, -1.0, CONFUSION);
    near(a_min.y, -1.0, CONFUSION);
    near(a_min.z, -1.0, CONFUSION);
    near(a_max.x, 4.0, CONFUSION);
    near(a_max.y, 4.0, CONFUSION);
    near(a_max.z, 4.0, CONFUSION);
}

#[test]
fn square_extent() {
    let a_box = BndB3d::from_center_hsize(Pnt::new(0.0, 0.0, 0.0), Pnt::new(3.0, 4.0, 5.0));
    let a_sq_extent = a_box.square_extent();

    // Square diagonal = 4 * (3^2 + 4^2 + 5^2) = 4 * 50 = 200
    near(a_sq_extent, 200.0, CONFUSION);
}

#[test]
fn enlarge() {
    let mut a_box = BndB3d::from_center_hsize(Pnt::new(0.0, 0.0, 0.0), Pnt::new(1.0, 1.0, 1.0));
    a_box.enlarge(0.5);

    let a_min = a_box.corner_min();
    let a_max = a_box.corner_max();
    near(a_min.x, -1.5, CONFUSION);
    near(a_min.y, -1.5, CONFUSION);
    near(a_min.z, -1.5, CONFUSION);
    near(a_max.x, 1.5, CONFUSION);
    near(a_max.y, 1.5, CONFUSION);
    near(a_max.z, 1.5, CONFUSION);
}

#[test]
fn limit() {
    // Test limiting a large box by a smaller box inside it
    let mut a_box1 = BndB3d::from_center_hsize(Pnt::new(0.0, 0.0, 0.0), Pnt::new(5.0, 5.0, 5.0));
    let a_box2 = BndB3d::from_center_hsize(Pnt::new(2.0, 2.0, 2.0), Pnt::new(2.0, 2.0, 2.0));

    let is_limited = a_box1.limit(&a_box2);
    assert!(is_limited);

    // After limiting, aBox1's min corner should align with aBox2's min corner
    let a_min = a_box1.corner_min();
    let a_max = a_box1.corner_max();
    near(a_min.x, 0.0, CONFUSION);
    near(a_min.y, 0.0, CONFUSION);
    near(a_min.z, 0.0, CONFUSION);
    near(a_max.x, 5.0, CONFUSION);
    near(a_max.y, 5.0, CONFUSION);
    near(a_max.z, 5.0, CONFUSION);

    // Test with non-intersecting boxes
    let mut a_box3 = BndB3d::from_center_hsize(Pnt::new(0.0, 0.0, 0.0), Pnt::new(1.0, 1.0, 1.0));
    let a_box4 = BndB3d::from_center_hsize(Pnt::new(10.0, 10.0, 10.0), Pnt::new(1.0, 1.0, 1.0));
    let is_limited2 = a_box3.limit(&a_box4);
    assert!(!is_limited2);
}

#[test]
fn is_out_point() {
    let a_box = BndB3d::from_center_hsize(Pnt::new(0.0, 0.0, 0.0), Pnt::new(1.0, 1.0, 1.0));

    assert!(!a_box.is_out_point(Pnt::new(0.0, 0.0, 0.0)));
    assert!(!a_box.is_out_point(Pnt::new(0.5, 0.5, 0.5)));
    assert!(a_box.is_out_point(Pnt::new(2.0, 0.0, 0.0)));
    assert!(a_box.is_out_point(Pnt::new(0.0, 2.0, 0.0)));
    assert!(a_box.is_out_point(Pnt::new(0.0, 0.0, 2.0)));
}

#[test]
fn is_out_sphere() {
    let a_box = BndB3d::from_center_hsize(Pnt::new(0.0, 0.0, 0.0), Pnt::new(1.0, 1.0, 1.0));

    // Sphere at (0, 0, 0) with small radius - should intersect
    assert!(!a_box.is_out_sphere(Pnt::new(0.0, 0.0, 0.0), 0.5, false));

    // Sphere far away - should not intersect
    assert!(a_box.is_out_sphere(Pnt::new(10.0, 10.0, 10.0), 1.0, false));
}

#[test]
fn is_out_box() {
    let a_box1 = BndB3d::from_center_hsize(Pnt::new(0.0, 0.0, 0.0), Pnt::new(1.0, 1.0, 1.0));
    let a_box2 = BndB3d::from_center_hsize(Pnt::new(0.5, 0.5, 0.5), Pnt::new(1.0, 1.0, 1.0));
    let a_box3 = BndB3d::from_center_hsize(Pnt::new(5.0, 5.0, 5.0), Pnt::new(1.0, 1.0, 1.0));

    assert!(!a_box1.is_out_box(&a_box2));
    assert!(a_box1.is_out_box(&a_box3));
}

#[test]
fn is_out_line() {
    let a_box = BndB3d::from_center_hsize(Pnt::new(0.0, 0.0, 0.0), Pnt::new(1.0, 1.0, 1.0));

    // Line passing through box
    let a_line1 = Ax1::new(Pnt::new(-2.0, 0.0, 0.0), Pnt::dir(1.0, 0.0, 0.0));
    assert!(!a_box.is_out_line(&a_line1, false, 0.0));

    // Line not intersecting box
    let a_line2 = Ax1::new(Pnt::new(-2.0, 5.0, 5.0), Pnt::dir(1.0, 0.0, 0.0));
    assert!(a_box.is_out_line(&a_line2, false, 0.0));
}

#[test]
fn is_out_plane() {
    let a_box = BndB3d::from_center_hsize(Pnt::new(0.0, 0.0, 0.0), Pnt::new(1.0, 1.0, 1.0));

    // Plane passing through box
    let a_plane1 = plane(Pnt::new(0.0, 0.0, 0.0), Pnt::dir(0.0, 0.0, 1.0));
    assert!(!a_box.is_out_plane(&a_plane1));

    // Plane not intersecting box
    let a_plane2 = plane(Pnt::new(0.0, 0.0, 5.0), Pnt::dir(0.0, 0.0, 1.0));
    assert!(a_box.is_out_plane(&a_plane2));
}

#[test]
fn is_in_box() {
    let a_box1 = BndB3d::from_center_hsize(Pnt::new(0.0, 0.0, 0.0), Pnt::new(0.5, 0.5, 0.5));
    let a_box2 = BndB3d::from_center_hsize(Pnt::new(0.0, 0.0, 0.0), Pnt::new(2.0, 2.0, 2.0));

    assert!(a_box1.is_in_box(&a_box2));
    assert!(!a_box2.is_in_box(&a_box1));
}

#[test]
fn transformed() {
    let a_box = BndB3d::from_center_hsize(Pnt::new(1.0, 1.0, 1.0), Pnt::new(1.0, 1.0, 1.0));

    let mut a_trsf = Trsf::identity();
    a_trsf.set_translation(Pnt::new(2.0, 3.0, 4.0));

    let a_transformed_box = a_box.transformed(&a_trsf);

    let a_min = a_transformed_box.corner_min();
    let a_max = a_transformed_box.corner_max();

    near(a_min.x, 2.0, CONFUSION);
    near(a_min.y, 3.0, CONFUSION);
    near(a_min.z, 4.0, CONFUSION);
    near(a_max.x, 4.0, CONFUSION);
    near(a_max.y, 5.0, CONFUSION);
    near(a_max.z, 6.0, CONFUSION);
}

#[test]
fn transformed_with_rotation() {
    let a_box = BndB3d::from_center_hsize(Pnt::new(1.0, 0.0, 0.0), Pnt::new(0.5, 0.5, 0.5));

    let a_trsf = Trsf::rotation(
        Ax1::new(Pnt::new(0.0, 0.0, 0.0), Pnt::dir(0.0, 0.0, 1.0)),
        PI / 2.0,
    );

    let a_transformed_box = a_box.transformed(&a_trsf);

    // After 90 degree rotation around Z-axis, the center should be at (0, 1, 0)
    let a_min = a_transformed_box.corner_min();
    let a_max = a_transformed_box.corner_max();

    near((a_min.x + a_max.x) / 2.0, 0.0, 1e-10);
    near((a_min.y + a_max.y) / 2.0, 1.0, 1e-10);
    near((a_min.z + a_max.z) / 2.0, 0.0, 1e-10);
}

#[test]
fn set_center_and_hsize() {
    let mut a_box = BndB3d::new();

    a_box.set_center(Pnt::new(5.0, 10.0, 15.0));
    a_box.set_hsize(Pnt::new(2.0, 3.0, 4.0));

    let a_min = a_box.corner_min();
    let a_max = a_box.corner_max();

    near(a_min.x, 3.0, CONFUSION);
    near(a_min.y, 7.0, CONFUSION);
    near(a_min.z, 11.0, CONFUSION);
    near(a_max.x, 7.0, CONFUSION);
    near(a_max.y, 13.0, CONFUSION);
    near(a_max.z, 19.0, CONFUSION);
}

#[test]
fn float_precision() {
    let a_box = BndB3f::from_center_hsize(
        Pnt::new(1.0_f32 as f64, 2.0_f32 as f64, 3.0_f32 as f64),
        Pnt::new(0.5_f32 as f64, 0.5_f32 as f64, 0.5_f32 as f64),
    );

    assert!(!a_box.is_void());

    let a_min = a_box.corner_min();
    let a_max = a_box.corner_max();

    near(a_min.x, 0.5, 1e-5);
    near(a_min.y, 1.5, 1e-5);
    near(a_min.z, 2.5, 1e-5);
    near(a_max.x, 1.5, 1e-5);
    near(a_max.y, 2.5, 1e-5);
    near(a_max.z, 3.5, 1e-5);
}

#[test]
fn is_out_ray() {
    let a_box = BndB3d::from_center_hsize(Pnt::new(1.0, 1.0, 1.0), Pnt::new(1.0, 1.0, 1.0));

    // Ray starting before the box and pointing towards it
    let a_ray1 = Ax1::new(Pnt::new(-1.0, 1.0, 1.0), Pnt::dir(1.0, 0.0, 0.0));
    assert!(!a_box.is_out_line(&a_ray1, true, 0.0));

    // Ray starting inside the box
    let a_ray2 = Ax1::new(Pnt::new(1.0, 1.0, 1.0), Pnt::dir(1.0, 0.0, 0.0));
    assert!(!a_box.is_out_line(&a_ray2, true, 0.0));

    // Ray pointing away from the box
    let a_ray3 = Ax1::new(Pnt::new(-1.0, 1.0, 1.0), Pnt::dir(-1.0, 0.0, 0.0));
    assert!(a_box.is_out_line(&a_ray3, true, 0.0));
}

#[test]
fn is_out_line_with_overthickness() {
    let a_box = BndB3d::from_center_hsize(Pnt::new(0.0, 0.0, 0.0), Pnt::new(1.0, 1.0, 1.0));

    // Line passing just outside the box, but within overthickness
    let a_line = Ax1::new(Pnt::new(-2.0, 1.2, 0.0), Pnt::dir(1.0, 0.0, 0.0));

    // Without overthickness, should be out
    assert!(a_box.is_out_line(&a_line, false, 0.0));

    // With sufficient overthickness, should intersect
    assert!(!a_box.is_out_line(&a_line, false, 0.3));
}
