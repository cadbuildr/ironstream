//! Ported OpenCascade unit tests — `Bnd_B2` (2D bounding box).
//!
//! Faithful Rust port of OCCT's `Bnd_B2_Test.cxx`
//! (Open-Cascade-SAS/OCCT, `src/FoundationClasses/TKMath/GTests/`). Same
//! numeric inputs, expected values and tolerances (`Precision::Confusion()`).
//! `Bnd_B2d` -> [`BndB2d`], `Bnd_B2f` -> [`BndB2f`], `gp_XY`/`gp_Pnt2d` ->
//! `Pnt2d`, `gp_Ax2d` -> `Ax2d`, `gp_Trsf2d` -> `Trsf2d`.

use ironstream::bnd_b2::{BndB2d, BndB2f};
use ironstream::gp2d::{Ax2d, Pnt2d, Vec2d};
use ironstream::precision::CONFUSION;

fn near(a: f64, b: f64, tol: f64) {
    assert!((a - b).abs() <= tol, "expected {a} ≈ {b} (tol {tol})");
}

// ====================== Bnd_B2_Test.cxx ======================

#[test]
fn default_constructor() {
    let a_box = BndB2d::new();
    assert!(a_box.is_void());
}

#[test]
fn constructor_with_center_and_hsize() {
    let center = Pnt2d::new(5.0, 10.0);
    let hsize = Pnt2d::new(2.0, 3.0);

    let a_box = BndB2d::from_center_hsize(center, hsize);

    assert!(!a_box.is_void());

    let a_min = a_box.corner_min();
    let a_max = a_box.corner_max();

    near(a_min.x, 3.0, CONFUSION);
    near(a_min.y, 7.0, CONFUSION);
    near(a_max.x, 7.0, CONFUSION);
    near(a_max.y, 13.0, CONFUSION);
}

#[test]
fn clear() {
    let mut a_box = BndB2d::from_center_hsize(Pnt2d::new(0.0, 0.0), Pnt2d::new(1.0, 1.0));
    assert!(!a_box.is_void());

    a_box.clear();
    assert!(a_box.is_void());
}

#[test]
fn add_point() {
    let mut a_box = BndB2d::new();

    a_box.add(Pnt2d::new(1.0, 2.0));
    assert!(!a_box.is_void());

    let a_min = a_box.corner_min();
    let a_max = a_box.corner_max();
    near(a_min.x, 1.0, CONFUSION);
    near(a_min.y, 2.0, CONFUSION);
    near(a_max.x, 1.0, CONFUSION);
    near(a_max.y, 2.0, CONFUSION);

    a_box.add(Pnt2d::new(4.0, 5.0));
    let a_min = a_box.corner_min();
    let a_max = a_box.corner_max();
    near(a_min.x, 1.0, CONFUSION);
    near(a_min.y, 2.0, CONFUSION);
    near(a_max.x, 4.0, CONFUSION);
    near(a_max.y, 5.0, CONFUSION);
}

#[test]
fn add_pnt2d() {
    let mut a_box = BndB2d::new();
    a_box.add_pnt(Pnt2d::new(1.0, 2.0));

    assert!(!a_box.is_void());
    let a_min = a_box.corner_min();
    near(a_min.x, 1.0, CONFUSION);
    near(a_min.y, 2.0, CONFUSION);
}

#[test]
fn add_box() {
    let mut a_box1 = BndB2d::from_center_hsize(Pnt2d::new(0.0, 0.0), Pnt2d::new(1.0, 1.0));
    let a_box2 = BndB2d::from_center_hsize(Pnt2d::new(3.0, 3.0), Pnt2d::new(1.0, 1.0));

    a_box1.add_box(&a_box2);

    let a_min = a_box1.corner_min();
    let a_max = a_box1.corner_max();
    near(a_min.x, -1.0, CONFUSION);
    near(a_min.y, -1.0, CONFUSION);
    near(a_max.x, 4.0, CONFUSION);
    near(a_max.y, 4.0, CONFUSION);
}

#[test]
fn square_extent() {
    let a_box = BndB2d::from_center_hsize(Pnt2d::new(0.0, 0.0), Pnt2d::new(3.0, 4.0));
    let a_sq_extent = a_box.square_extent();

    // Square diagonal = 4 * (3^2 + 4^2) = 4 * 25 = 100
    near(a_sq_extent, 100.0, CONFUSION);
}

#[test]
fn enlarge() {
    let mut a_box = BndB2d::from_center_hsize(Pnt2d::new(0.0, 0.0), Pnt2d::new(1.0, 1.0));
    a_box.enlarge(0.5);

    let a_min = a_box.corner_min();
    let a_max = a_box.corner_max();
    near(a_min.x, -1.5, CONFUSION);
    near(a_min.y, -1.5, CONFUSION);
    near(a_max.x, 1.5, CONFUSION);
    near(a_max.y, 1.5, CONFUSION);
}

#[test]
fn limit() {
    // Test limiting a large box by a smaller box inside it
    let mut a_box1 = BndB2d::from_center_hsize(Pnt2d::new(0.0, 0.0), Pnt2d::new(5.0, 5.0));
    let a_box2 = BndB2d::from_center_hsize(Pnt2d::new(2.0, 2.0), Pnt2d::new(2.0, 2.0));

    let is_limited = a_box1.limit(&a_box2);
    assert!(is_limited);

    // After limiting, aBox1's min corner should align with aBox2's min corner
    let a_min = a_box1.corner_min();
    let a_max = a_box1.corner_max();
    near(a_min.x, 0.0, CONFUSION);
    near(a_min.y, 0.0, CONFUSION);
    near(a_max.x, 5.0, CONFUSION);
    near(a_max.y, 5.0, CONFUSION);

    // Test with non-intersecting boxes
    let mut a_box3 = BndB2d::from_center_hsize(Pnt2d::new(0.0, 0.0), Pnt2d::new(1.0, 1.0));
    let a_box4 = BndB2d::from_center_hsize(Pnt2d::new(10.0, 10.0), Pnt2d::new(1.0, 1.0));
    let is_limited2 = a_box3.limit(&a_box4);
    assert!(!is_limited2);
}

#[test]
fn is_out_point() {
    let a_box = BndB2d::from_center_hsize(Pnt2d::new(0.0, 0.0), Pnt2d::new(1.0, 1.0));

    assert!(!a_box.is_out_point(Pnt2d::new(0.0, 0.0)));
    assert!(!a_box.is_out_point(Pnt2d::new(0.5, 0.5)));
    assert!(a_box.is_out_point(Pnt2d::new(2.0, 0.0)));
    assert!(a_box.is_out_point(Pnt2d::new(0.0, 2.0)));
}

#[test]
fn is_out_circle() {
    let a_box = BndB2d::from_center_hsize(Pnt2d::new(0.0, 0.0), Pnt2d::new(1.0, 1.0));

    // Circle at (0, 0) with small radius - should intersect
    assert!(!a_box.is_out_circle(Pnt2d::new(0.0, 0.0), 0.5, false));

    // Circle far away - should not intersect
    assert!(a_box.is_out_circle(Pnt2d::new(10.0, 10.0), 1.0, false));
}

#[test]
fn is_out_box() {
    let a_box1 = BndB2d::from_center_hsize(Pnt2d::new(0.0, 0.0), Pnt2d::new(1.0, 1.0));
    let a_box2 = BndB2d::from_center_hsize(Pnt2d::new(0.5, 0.5), Pnt2d::new(1.0, 1.0));
    let a_box3 = BndB2d::from_center_hsize(Pnt2d::new(5.0, 5.0), Pnt2d::new(1.0, 1.0));

    assert!(!a_box1.is_out_box(&a_box2));
    assert!(a_box1.is_out_box(&a_box3));
}

#[test]
fn is_out_line() {
    let a_box = BndB2d::from_center_hsize(Pnt2d::new(0.0, 0.0), Pnt2d::new(1.0, 1.0));

    // Line passing through box
    let a_line1 = Ax2d::new(Pnt2d::new(-2.0, 0.0), Vec2d::new(1.0, 0.0));
    assert!(!a_box.is_out_line(&a_line1));

    // Line not intersecting box
    let a_line2 = Ax2d::new(Pnt2d::new(-2.0, 5.0), Vec2d::new(1.0, 0.0));
    assert!(a_box.is_out_line(&a_line2));
}

#[test]
fn is_out_segment() {
    let a_box = BndB2d::from_center_hsize(Pnt2d::new(0.0, 0.0), Pnt2d::new(1.0, 1.0));

    // Segment intersecting box
    assert!(!a_box.is_out_segment(Pnt2d::new(-2.0, 0.0), Pnt2d::new(2.0, 0.0)));

    // Segment not intersecting box
    assert!(a_box.is_out_segment(Pnt2d::new(5.0, 5.0), Pnt2d::new(6.0, 6.0)));
}

#[test]
fn is_in_box() {
    let a_box1 = BndB2d::from_center_hsize(Pnt2d::new(0.0, 0.0), Pnt2d::new(0.5, 0.5));
    let a_box2 = BndB2d::from_center_hsize(Pnt2d::new(0.0, 0.0), Pnt2d::new(2.0, 2.0));

    assert!(a_box1.is_in_box(&a_box2));
    assert!(!a_box2.is_in_box(&a_box1));
}

#[test]
fn transformed() {
    let a_box = BndB2d::from_center_hsize(Pnt2d::new(1.0, 1.0), Pnt2d::new(1.0, 1.0));

    let a_trsf = ironstream::gp2d::Trsf2d::translation(Vec2d::new(2.0, 3.0));

    let a_transformed_box = a_box.transformed(&a_trsf);

    let a_min = a_transformed_box.corner_min();
    let a_max = a_transformed_box.corner_max();

    near(a_min.x, 2.0, CONFUSION);
    near(a_min.y, 3.0, CONFUSION);
    near(a_max.x, 4.0, CONFUSION);
    near(a_max.y, 5.0, CONFUSION);
}

#[test]
fn set_center_and_hsize() {
    let mut a_box = BndB2d::new();

    a_box.set_center(Pnt2d::new(5.0, 10.0));
    a_box.set_hsize(Pnt2d::new(2.0, 3.0));

    let a_min = a_box.corner_min();
    let a_max = a_box.corner_max();

    near(a_min.x, 3.0, CONFUSION);
    near(a_min.y, 7.0, CONFUSION);
    near(a_max.x, 7.0, CONFUSION);
    near(a_max.y, 13.0, CONFUSION);
}

#[test]
fn float_precision() {
    let a_box = BndB2f::from_center_hsize(
        Pnt2d::new(1.0_f32 as f64, 2.0_f32 as f64),
        Pnt2d::new(0.5_f32 as f64, 0.5_f32 as f64),
    );

    assert!(!a_box.is_void());

    let a_min = a_box.corner_min();
    let a_max = a_box.corner_max();

    near(a_min.x, 0.5, 1e-5);
    near(a_min.y, 1.5, 1e-5);
    near(a_max.x, 1.5, 1e-5);
    near(a_max.y, 2.5, 1e-5);
}
