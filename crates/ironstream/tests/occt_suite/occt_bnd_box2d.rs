//! Port of OCCT's `Bnd_Box2d_Test.cxx` GTest suite to Rust `#[test]` functions.
//!
//! Source: OCCT `src/FoundationClasses/TKMath/GTests/Bnd_Box2d_Test.cxx`.
//! Same numeric inputs, expected values and tolerances as the upstream tests.
//! `gp_Pnt2d` / `gp_Dir2d` map to `ironstream::gp2d::Pnt2d`, `gp_Trsf2d` to
//! `ironstream::gp2d::Trsf2d`. `EXPECT_DOUBLE_EQ` is mapped to a strict exact
//! comparison (the upstream values are all exactly representable), and
//! `EXPECT_NEAR(.., 1e-10)` to an absolute tolerance of `1e-10`.

use ironstream::bnd_box2d::BndBox2d;
use ironstream::gp2d::{Pnt2d, Trsf2d};

/// Mirrors `gp_Pnt2d(x, y)`.
fn pnt(x: f64, y: f64) -> Pnt2d {
    Pnt2d::new(x, y)
}

/// `EXPECT_DOUBLE_EQ` — the upstream expected values are exact.
fn expect_double_eq(actual: f64, expected: f64) {
    assert_eq!(actual, expected, "expected {expected}, got {actual}");
}

/// `EXPECT_NEAR(actual, expected, tol)`.
fn expect_near(actual: f64, expected: f64, tol: f64) {
    assert!(
        (actual - expected).abs() <= tol,
        "expected {expected} +/- {tol}, got {actual}"
    );
}

#[test]
fn default_constructor() {
    let a_box = BndBox2d::new();
    assert!(a_box.is_void());
    assert!(!a_box.is_whole());
    expect_double_eq(a_box.gap(), 0.0);
}

#[test]
fn set_void() {
    let mut a_box = BndBox2d::new();
    a_box.update_bounds(0.0, 0.0, 10.0, 10.0);
    assert!(!a_box.is_void());
    a_box.set_void();
    assert!(a_box.is_void());
}

#[test]
fn set_whole() {
    let mut a_box = BndBox2d::new();
    a_box.set_whole();
    assert!(a_box.is_whole());
    assert!(!a_box.is_void());
}

#[test]
fn update_bounds() {
    let mut a_box = BndBox2d::new();
    a_box.update_bounds(1.0, 2.0, 10.0, 20.0);
    let lim = a_box.get().expect("non-void");
    expect_double_eq(lim.xmin, 1.0);
    expect_double_eq(lim.ymin, 2.0);
    expect_double_eq(lim.xmax, 10.0);
    expect_double_eq(lim.ymax, 20.0);
}

#[test]
fn update_single_point() {
    let mut a_box = BndBox2d::new();
    a_box.update_point(5.0, 7.0);
    let lim = a_box.get().expect("non-void");
    expect_double_eq(lim.xmin, 5.0);
    expect_double_eq(lim.ymin, 7.0);
    expect_double_eq(lim.xmax, 5.0);
    expect_double_eq(lim.ymax, 7.0);
}

#[test]
fn update_expansion() {
    let mut a_box = BndBox2d::new();
    a_box.update_bounds(1.0, 2.0, 10.0, 20.0);
    a_box.update_bounds(0.0, 0.0, 15.0, 25.0);
    let lim = a_box.get().expect("non-void");
    expect_double_eq(lim.xmin, 0.0);
    expect_double_eq(lim.ymin, 0.0);
    expect_double_eq(lim.xmax, 15.0);
    expect_double_eq(lim.ymax, 25.0);
}

#[test]
fn gap_operations() {
    let mut a_box = BndBox2d::new();
    a_box.update_bounds(0.0, 0.0, 10.0, 10.0);
    expect_double_eq(a_box.gap(), 0.0);
    a_box.set_gap(2.0);
    expect_double_eq(a_box.gap(), 2.0);
    a_box.set_gap(-3.0);
    expect_double_eq(a_box.gap(), 3.0);
}

#[test]
fn enlarge() {
    let mut a_box = BndBox2d::new();
    a_box.update_bounds(0.0, 0.0, 10.0, 10.0);
    a_box.enlarge(5.0);
    expect_double_eq(a_box.gap(), 5.0);
    a_box.enlarge(3.0);
    expect_double_eq(a_box.gap(), 5.0); // max(5, 3) = 5
    a_box.enlarge(7.0);
    expect_double_eq(a_box.gap(), 7.0); // max(5, 7) = 7
}

#[test]
fn get_with_gap() {
    let mut a_box = BndBox2d::new();
    a_box.update_bounds(1.0, 2.0, 10.0, 20.0);
    a_box.set_gap(0.5);
    let lim = a_box.get().expect("non-void");
    expect_double_eq(lim.xmin, 0.5);
    expect_double_eq(lim.ymin, 1.5);
    expect_double_eq(lim.xmax, 10.5);
    expect_double_eq(lim.ymax, 20.5);
}

#[test]
fn get_structured_bindings() {
    let mut a_box = BndBox2d::new();
    a_box.update_bounds(1.0, 2.0, 10.0, 20.0);
    // OCCT: auto [aXmin, aXmax, aYmin, aYmax] = aBox.Get();
    let lim = a_box.get_limits();
    expect_double_eq(lim.xmin, 1.0);
    expect_double_eq(lim.xmax, 10.0);
    expect_double_eq(lim.ymin, 2.0);
    expect_double_eq(lim.ymax, 20.0);
}

#[test]
fn set_point() {
    let mut a_box = BndBox2d::new();
    a_box.set_point(pnt(5.0, 7.0));
    let lim = a_box.get().expect("non-void");
    expect_double_eq(lim.xmin, 5.0);
    expect_double_eq(lim.ymin, 7.0);
    expect_double_eq(lim.xmax, 5.0);
    expect_double_eq(lim.ymax, 7.0);
}

#[test]
fn add_point() {
    let mut a_box = BndBox2d::new();
    a_box.add_point(pnt(1.0, 2.0));
    a_box.add_point(pnt(10.0, 20.0));
    let lim = a_box.get().expect("non-void");
    expect_double_eq(lim.xmin, 1.0);
    expect_double_eq(lim.ymin, 2.0);
    expect_double_eq(lim.xmax, 10.0);
    expect_double_eq(lim.ymax, 20.0);
}

#[test]
fn add_box() {
    let mut a_box1 = BndBox2d::new();
    a_box1.update_bounds(0.0, 0.0, 5.0, 5.0);
    let mut a_box2 = BndBox2d::new();
    a_box2.update_bounds(3.0, 3.0, 10.0, 10.0);
    a_box1.add(&a_box2);
    let lim = a_box1.get().expect("non-void");
    expect_double_eq(lim.xmin, 0.0);
    expect_double_eq(lim.ymin, 0.0);
    expect_double_eq(lim.xmax, 10.0);
    expect_double_eq(lim.ymax, 10.0);
}

#[test]
fn add_void_box() {
    let mut a_box = BndBox2d::new();
    a_box.update_bounds(1.0, 2.0, 5.0, 6.0);
    let a_void_box = BndBox2d::new();
    a_box.add(&a_void_box);
    let lim = a_box.get().expect("non-void");
    expect_double_eq(lim.xmin, 1.0);
    expect_double_eq(lim.ymin, 2.0);
}

#[test]
fn add_to_void_box() {
    let mut a_void_box = BndBox2d::new();
    let mut a_box = BndBox2d::new();
    a_box.update_bounds(1.0, 2.0, 5.0, 6.0);
    a_void_box.add(&a_box);
    let lim = a_void_box.get().expect("non-void");
    expect_double_eq(lim.xmin, 1.0);
    expect_double_eq(lim.ymin, 2.0);
    expect_double_eq(lim.xmax, 5.0);
    expect_double_eq(lim.ymax, 6.0);
}

#[test]
fn open_directions() {
    let mut a_box = BndBox2d::new();
    a_box.update_bounds(0.0, 0.0, 10.0, 10.0);
    assert!(!a_box.is_open_xmin());
    assert!(!a_box.is_open_xmax());
    assert!(!a_box.is_open_ymin());
    assert!(!a_box.is_open_ymax());

    a_box.open_xmin();
    assert!(a_box.is_open_xmin());
    a_box.open_ymax();
    assert!(a_box.is_open_ymax());
    assert!(!a_box.is_whole());
}

#[test]
fn is_out_point() {
    let mut a_box = BndBox2d::new();
    a_box.update_bounds(0.0, 0.0, 10.0, 10.0);
    assert!(!a_box.is_out_point(pnt(5.0, 5.0)));
    assert!(a_box.is_out_point(pnt(15.0, 5.0)));
    assert!(a_box.is_out_point(pnt(-1.0, 5.0)));
}

#[test]
fn is_out_box_overlapping() {
    let mut a_box1 = BndBox2d::new();
    a_box1.update_bounds(0.0, 0.0, 10.0, 10.0);
    let mut a_box2 = BndBox2d::new();
    a_box2.update_bounds(5.0, 5.0, 15.0, 15.0);
    assert!(!a_box1.is_out_box(&a_box2));
}

#[test]
fn is_out_box_separated() {
    let mut a_box1 = BndBox2d::new();
    a_box1.update_bounds(0.0, 0.0, 5.0, 5.0);
    let mut a_box2 = BndBox2d::new();
    a_box2.update_bounds(10.0, 10.0, 15.0, 15.0);
    assert!(a_box1.is_out_box(&a_box2));
}

#[test]
fn is_out_box_fast_path() {
    // Both non-void, non-whole, non-open -> fast path
    let mut a_box1 = BndBox2d::new();
    a_box1.update_bounds(0.0, 0.0, 10.0, 10.0);
    let mut a_box2 = BndBox2d::new();
    a_box2.update_bounds(5.0, 5.0, 15.0, 15.0);
    assert!(!a_box1.is_out_box(&a_box2));

    let mut a_box3 = BndBox2d::new();
    a_box3.update_bounds(20.0, 20.0, 30.0, 30.0);
    assert!(a_box1.is_out_box(&a_box3));
}

#[test]
fn is_out_box_fast_path_with_gap() {
    // Boxes separated by 1.0, but gap bridges it
    let mut a_box1 = BndBox2d::new();
    a_box1.update_bounds(0.0, 0.0, 5.0, 5.0);
    a_box1.set_gap(1.0);
    let mut a_box2 = BndBox2d::new();
    a_box2.update_bounds(6.0, 0.0, 10.0, 5.0);
    // Gap from box1 = 1.0, gap from box2 = 0.0, total = 1.0
    // Xmin2 - Xmax1 = 6 - 5 = 1 (not > 1.0, equal) -> not out
    assert!(!a_box1.is_out_box(&a_box2));

    // Now increase separation
    let mut a_box3 = BndBox2d::new();
    a_box3.update_bounds(7.0, 0.0, 10.0, 5.0);
    // Xmin3 - Xmax1 = 7 - 5 = 2 > 1.0 -> out
    assert!(a_box1.is_out_box(&a_box3));
}

#[test]
fn is_out_void_box() {
    let mut a_box = BndBox2d::new();
    a_box.update_bounds(0.0, 0.0, 10.0, 10.0);
    let a_void = BndBox2d::new();
    assert!(a_box.is_out_box(&a_void));
    assert!(a_void.is_out_box(&a_box));
}

#[test]
fn is_out_whole_box() {
    let mut a_box = BndBox2d::new();
    a_box.update_bounds(0.0, 0.0, 10.0, 10.0);
    let mut a_whole = BndBox2d::new();
    a_whole.set_whole();
    assert!(!a_box.is_out_box(&a_whole));
    assert!(!a_whole.is_out_box(&a_box));
}

#[test]
fn is_out_open_box() {
    let mut a_box1 = BndBox2d::new();
    a_box1.update_bounds(0.0, 0.0, 10.0, 10.0);
    a_box1.open_xmin(); // extends to -infinity in X

    let mut a_box2 = BndBox2d::new();
    a_box2.update_bounds(-100.0, 0.0, -50.0, 10.0);

    // a_box1 is open in Xmin, so extends to -infinity: should overlap
    assert!(!a_box1.is_out_box(&a_box2));
}

#[test]
fn is_out_line() {
    let mut a_box = BndBox2d::new();
    a_box.update_bounds(0.0, 0.0, 10.0, 10.0);

    // Line that intersects: gp_Lin2d(gp_Pnt2d(5,-5), gp_Dir2d(0,1))
    assert!(!a_box.is_out_line(pnt(5.0, -5.0), pnt(0.0, 1.0)));

    // Line that doesn't intersect: gp_Lin2d(gp_Pnt2d(20,-5), gp_Dir2d(0,1))
    assert!(a_box.is_out_line(pnt(20.0, -5.0), pnt(0.0, 1.0)));
}

#[test]
fn is_out_segment() {
    let mut a_box = BndBox2d::new();
    a_box.update_bounds(0.0, 0.0, 10.0, 10.0);

    // Segment that crosses
    assert!(!a_box.is_out_segment(pnt(5.0, -5.0), pnt(5.0, 15.0)));

    // Segment fully outside
    assert!(a_box.is_out_segment(pnt(20.0, 0.0), pnt(20.0, 10.0)));
}

#[test]
fn transformed() {
    let mut a_box = BndBox2d::new();
    a_box.update_bounds(0.0, 0.0, 10.0, 10.0);

    // gp_Trsf2d::SetTranslation(gp_Pnt2d(0,0), gp_Pnt2d(5,5)) == translate by (5,5)
    let a_trsf = Trsf2d::translation(pnt(5.0, 5.0));
    let a_transformed = a_box.transformed(&a_trsf);

    let lim = a_transformed.get().expect("non-void");
    expect_double_eq(lim.xmin, 5.0);
    expect_double_eq(lim.ymin, 5.0);
    expect_double_eq(lim.xmax, 15.0);
    expect_double_eq(lim.ymax, 15.0);
}

#[test]
fn square_extent() {
    let mut a_box = BndBox2d::new();
    a_box.update_bounds(0.0, 0.0, 3.0, 4.0);
    // Diagonal^2 = 3^2 + 4^2 = 25
    expect_double_eq(a_box.square_extent(), 25.0);
}

#[test]
fn square_extent_void() {
    let a_box = BndBox2d::new();
    expect_double_eq(a_box.square_extent(), 0.0);
}

#[test]
fn add_direction() {
    let mut a_box = BndBox2d::new();
    a_box.set_point(pnt(5.0, 5.0));
    a_box.add_dir(pnt(1.0, 0.0));
    assert!(a_box.is_open_xmax());
    assert!(!a_box.is_open_xmin());
}

#[test]
fn contains_point() {
    let mut a_box = BndBox2d::new();
    a_box.update_bounds(0.0, 0.0, 10.0, 10.0);
    assert!(a_box.contains(pnt(5.0, 5.0)));
    assert!(a_box.contains(pnt(0.0, 0.0)));
    assert!(!a_box.contains(pnt(-1.0, 5.0)));
    assert!(!a_box.contains(pnt(5.0, 11.0)));
}

#[test]
fn intersects_box() {
    let mut a_box1 = BndBox2d::new();
    a_box1.update_bounds(0.0, 0.0, 10.0, 10.0);
    let mut a_box2 = BndBox2d::new();
    a_box2.update_bounds(5.0, 5.0, 15.0, 15.0);
    let mut a_box3 = BndBox2d::new();
    a_box3.update_bounds(20.0, 20.0, 30.0, 30.0);
    assert!(a_box1.intersects(&a_box2));
    assert!(!a_box1.intersects(&a_box3));
}

#[test]
fn distance_separated() {
    let mut a_box1 = BndBox2d::new();
    a_box1.update_bounds(0.0, 0.0, 1.0, 1.0);
    let mut a_box2 = BndBox2d::new();
    a_box2.update_bounds(4.0, 0.0, 5.0, 1.0);
    expect_near(a_box1.distance(&a_box2), 3.0, 1e-10);
}

#[test]
fn distance_overlapping() {
    let mut a_box1 = BndBox2d::new();
    a_box1.update_bounds(0.0, 0.0, 10.0, 10.0);
    let mut a_box2 = BndBox2d::new();
    a_box2.update_bounds(5.0, 5.0, 15.0, 15.0);
    expect_double_eq(a_box1.distance(&a_box2), 0.0);
}

#[test]
fn distance_diagonal() {
    let mut a_box1 = BndBox2d::new();
    a_box1.update_bounds(0.0, 0.0, 1.0, 1.0);
    let mut a_box2 = BndBox2d::new();
    a_box2.update_bounds(4.0, 4.0, 5.0, 5.0);
    expect_near(a_box1.distance(&a_box2), 18.0_f64.sqrt(), 1e-10);
}

#[test]
fn distance_void() {
    let a_box1 = BndBox2d::new();
    let mut a_box2 = BndBox2d::new();
    a_box2.update_bounds(0.0, 0.0, 1.0, 1.0);
    expect_double_eq(a_box1.distance(&a_box2), 0.0);
}

#[test]
fn center() {
    let mut a_box = BndBox2d::new();
    a_box.update_bounds(0.0, 0.0, 10.0, 20.0);
    let a_center = a_box.center();
    assert!(a_center.is_some());
    let c = a_center.unwrap();
    expect_double_eq(c.x, 5.0);
    expect_double_eq(c.y, 10.0);
}

#[test]
fn center_void_nullopt() {
    let a_void = BndBox2d::new();
    assert!(a_void.center().is_none());
}
