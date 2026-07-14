//! Ported OpenCascade unit test — `Bnd_Box`.
//!
//! Faithful Rust port of OCCT's `Bnd_Box_Test.cxx`
//! (Open-Cascade-SAS/OCCT, `src/FoundationClasses/TKMath/GTests/`). Same
//! numeric inputs, expected values and tolerances. `gp_Pnt`/`gp_Dir`/`gp_Vec`
//! map to `ironstream::gp::Pnt`; `gp_Pln`/`gp_Lin` to `ironstream::gp_prim`;
//! `gp_Trsf` to `ironstream::gp::Trsf`. OCCT's `Standard_ConstructionError`
//! throws and `std::optional` returns both map to `Option::None`.

use ironstream::bnd_box::BndBox;
use ironstream::gp::{Ax1, Pnt, Trsf};
use ironstream::gp_prim::{Lin, Pln};
use std::f64::consts::PI;

/// OCCT's `THE_BND_PRECISION_INFINITE` for open boxes (Bnd_Box.cxx).
const BND_INFINITE: f64 = 1.0e100;

/// `EXPECT_DOUBLE_EQ`: bit-exact equality of two doubles.
fn eq(a: f64, b: f64) {
    assert!(a == b, "expected {a} == {b}");
}

/// `EXPECT_NEAR(a, b, tol)`.
fn near(a: f64, b: f64, tol: f64) {
    assert!((a - b).abs() <= tol, "expected {a} ≈ {b} (tol {tol})");
}

// ====================== Constructor Tests ======================

#[test]
fn default_constructor() {
    let a_box = BndBox::new();
    assert!(a_box.is_void(), "Default constructed box should be void");
    assert!(
        !a_box.is_whole(),
        "Default constructed box should not be whole"
    );
    eq(0.0, a_box.gap());
}

#[test]
fn point_constructor() {
    let a_min = Pnt::new(1.0, 2.0, 3.0);
    let a_max = Pnt::new(4.0, 5.0, 6.0);
    let a_box = BndBox::from_corners(a_min, a_max);

    assert!(!a_box.is_void());
    assert!(!a_box.is_whole());

    let l = a_box.get().expect("box not void");
    eq(1.0, l.xmin);
    eq(2.0, l.ymin);
    eq(3.0, l.zmin);
    eq(4.0, l.xmax);
    eq(5.0, l.ymax);
    eq(6.0, l.zmax);
}

// ====================== Set and Update Tests ======================

#[test]
fn set_with_point() {
    let mut a_box = BndBox::new();
    a_box.set_point(Pnt::new(1.5, 2.5, 3.5));

    assert!(!a_box.is_void());

    let l = a_box.get().expect("box not void");
    eq(1.5, l.xmin);
    eq(1.5, l.xmax);
    eq(2.5, l.ymin);
    eq(2.5, l.ymax);
    eq(3.5, l.zmin);
    eq(3.5, l.zmax);
}

#[test]
fn set_with_point_and_direction() {
    let mut a_box = BndBox::new();
    a_box.set_point_dir(Pnt::new(1.0, 2.0, 3.0), Pnt::dir(1.0, 0.0, 0.0));

    assert!(!a_box.is_void());
    assert!(
        a_box.is_open_xmax(),
        "Box should be open in positive X direction"
    );
    assert!(
        !a_box.is_open_xmin(),
        "Box should not be open in negative X direction"
    );
}

#[test]
fn update_with_bounds() {
    let mut a_box = BndBox::new();
    a_box.update_bounds(1.0, 2.0, 3.0, 4.0, 5.0, 6.0);

    assert!(!a_box.is_void());

    let l = a_box.get().expect("box not void");
    eq(1.0, l.xmin);
    eq(2.0, l.ymin);
    eq(3.0, l.zmin);
    eq(4.0, l.xmax);
    eq(5.0, l.ymax);
    eq(6.0, l.zmax);
}

#[test]
fn update_with_point() {
    let mut a_box = BndBox::new();

    a_box.update_point(1.0, 2.0, 3.0);
    let l = a_box.get().expect("box not void");
    eq(1.0, l.xmin);
    eq(1.0, l.xmax);

    a_box.update_point(0.5, 2.5, 3.5);
    let l = a_box.get().expect("box not void");
    eq(0.5, l.xmin);
    eq(1.0, l.xmax);
    eq(2.0, l.ymin);
    eq(2.5, l.ymax);
}

#[test]
fn update_expansion() {
    let mut a_box = BndBox::new();
    a_box.update_bounds(0.0, 0.0, 0.0, 1.0, 1.0, 1.0);
    a_box.update_bounds(-1.0, -1.0, -1.0, 2.0, 2.0, 2.0);

    let l = a_box.get().expect("box not void");
    eq(-1.0, l.xmin);
    eq(-1.0, l.ymin);
    eq(-1.0, l.zmin);
    eq(2.0, l.xmax);
    eq(2.0, l.ymax);
    eq(2.0, l.zmax);
}

// ====================== Gap and Tolerance Tests ======================

#[test]
fn gap_operations() {
    let mut a_box = BndBox::new();
    eq(0.0, a_box.gap());

    a_box.set_gap(0.5);
    eq(0.5, a_box.gap());

    a_box.enlarge(0.3);
    eq(0.5, a_box.gap());

    a_box.enlarge(0.8);
    eq(0.8, a_box.gap());

    a_box.enlarge(-1.0);
    eq(1.0, a_box.gap());
}

// ====================== Corner Methods Tests ======================

#[test]
fn corner_methods() {
    let mut a_box = BndBox::new();
    a_box.update_bounds(1.0, 2.0, 3.0, 4.0, 5.0, 6.0);
    a_box.set_gap(0.1);

    let cmin = a_box.corner_min().expect("box not void");
    let cmax = a_box.corner_max().expect("box not void");

    eq(0.9, cmin.x);
    eq(1.9, cmin.y);
    eq(2.9, cmin.z);
    eq(4.1, cmax.x);
    eq(5.1, cmax.y);
    eq(6.1, cmax.z);
}

#[test]
fn corner_methods_void_box() {
    let a_box = BndBox::new(); // void
                               // OCCT throws Standard_ConstructionError; we return None.
    assert!(
        a_box.corner_min().is_none(),
        "CornerMin should fail for void box"
    );
    assert!(
        a_box.corner_max().is_none(),
        "CornerMax should fail for void box"
    );
}

#[test]
fn corner_methods_open_box() {
    let mut a_box = BndBox::new();
    a_box.update_bounds(1.0, 2.0, 3.0, 4.0, 5.0, 6.0);
    a_box.open_xmax();
    a_box.open_ymin();

    let cmin = a_box.corner_min().expect("box not void");
    let cmax = a_box.corner_max().expect("box not void");

    eq(-BND_INFINITE, cmin.y);
    eq(BND_INFINITE, cmax.x);
}

// ====================== Thin Methods Tests ======================

#[test]
fn thinness_methods() {
    let mut a_box = BndBox::new();

    assert!(a_box.is_x_thin(1.0), "Void box should be considered thin");
    assert!(a_box.is_y_thin(1.0), "Void box should be considered thin");
    assert!(a_box.is_z_thin(1.0), "Void box should be considered thin");
    assert!(a_box.is_thin(1.0), "Void box should be considered thin");

    a_box.update_bounds(1.0, 0.0, 0.0, 1.01, 2.0, 2.0);

    assert!(a_box.is_x_thin(0.1), "Thin X dimension should be detected");
    assert!(
        !a_box.is_y_thin(0.1),
        "Thick Y dimension should not be thin"
    );
    assert!(
        !a_box.is_z_thin(0.1),
        "Thick Z dimension should not be thin"
    );
    assert!(
        !a_box.is_thin(0.1),
        "Box with only one thin dimension should not be overall thin"
    );

    let mut a_thin = BndBox::new();
    a_thin.update_bounds(0.0, 0.0, 0.0, 0.01, 0.01, 0.01);

    assert!(a_thin.is_x_thin(0.1));
    assert!(a_thin.is_y_thin(0.1));
    assert!(a_thin.is_z_thin(0.1));
    assert!(
        a_thin.is_thin(0.1),
        "Box thin in all dimensions should be overall thin"
    );
}

#[test]
fn thinness_with_open_box() {
    let mut a_box = BndBox::new();
    a_box.update_bounds(0.0, 0.0, 0.0, 1.0, 1.0, 1.0);
    a_box.open_xmin();

    assert!(
        !a_box.is_x_thin(0.1),
        "Open dimension should not be considered thin"
    );

    a_box.set_whole();
    assert!(
        !a_box.is_x_thin(0.1),
        "Whole box should not be considered thin"
    );
    assert!(
        !a_box.is_thin(0.1),
        "Whole box should not be considered thin"
    );
}

// ====================== Transformation Tests ======================

#[test]
fn transformation_identity() {
    let mut a_box = BndBox::new();
    a_box.update_bounds(1.0, 2.0, 3.0, 4.0, 5.0, 6.0);
    a_box.set_gap(0.1);

    let identity = Trsf::identity();
    let a_t = a_box.transformed(&identity);

    let l1 = a_box.get().expect("box not void");
    let l2 = a_t.get().expect("box not void");

    eq(l1.xmin, l2.xmin);
    eq(l1.ymin, l2.ymin);
    eq(l1.zmin, l2.zmin);
    eq(l1.xmax, l2.xmax);
    eq(l1.ymax, l2.ymax);
    eq(l1.zmax, l2.zmax);
    eq(a_box.gap(), a_t.gap());
}

#[test]
fn transformation_translation() {
    let mut a_box = BndBox::new();
    a_box.update_bounds(0.0, 0.0, 0.0, 1.0, 1.0, 1.0);

    let mut translation = Trsf::identity();
    translation.set_translation(Pnt::new(2.0, 3.0, 4.0));

    let a_t = a_box.transformed(&translation);

    let l = a_t.get().expect("box not void");
    eq(2.0, l.xmin);
    eq(3.0, l.ymin);
    eq(4.0, l.zmin);
    eq(3.0, l.xmax);
    eq(4.0, l.ymax);
    eq(5.0, l.zmax);
}

#[test]
fn transformation_void_box() {
    let a_void = BndBox::new();
    let rotation = Trsf::rotation(Ax1::new(Pnt::origin(), Pnt::dir(0.0, 0.0, 1.0)), PI / 4.0);

    let a_t = a_void.transformed(&rotation);
    assert!(a_t.is_void(), "Transformed void box should remain void");
}

// ====================== Add Methods Tests ======================

#[test]
fn add_box() {
    let mut box1 = BndBox::new();
    box1.update_bounds(0.0, 0.0, 0.0, 1.0, 1.0, 1.0);
    box1.set_gap(0.1);

    let mut box2 = BndBox::new();
    box2.update_bounds(0.5, 0.5, 0.5, 2.0, 2.0, 2.0);
    box2.set_gap(0.2);

    box1.add(&box2);

    let l = box1.get().expect("box not void");
    // Get() includes the gap (max of both = 0.2).
    eq(-0.2, l.xmin);
    eq(-0.2, l.ymin);
    eq(-0.2, l.zmin);
    eq(2.2, l.xmax);
    eq(2.2, l.ymax);
    eq(2.2, l.zmax);
    eq(0.2, box1.gap());
}

#[test]
fn add_void_box() {
    let mut a_box = BndBox::new();
    a_box.update_bounds(1.0, 1.0, 1.0, 2.0, 2.0, 2.0);

    let a_void = BndBox::new();
    a_box.add(&a_void);

    let l = a_box.get().expect("box not void");
    eq(1.0, l.xmin);
    eq(2.0, l.xmax);
}

#[test]
fn add_to_void_box() {
    let mut a_void = BndBox::new();
    let mut a_box = BndBox::new();
    a_box.update_bounds(1.0, 1.0, 1.0, 2.0, 2.0, 2.0);

    a_void.add(&a_box);

    assert!(
        !a_void.is_void(),
        "Adding to void box should make it non-void"
    );
    let l = a_void.get().expect("box not void");
    eq(1.0, l.xmin);
    eq(2.0, l.xmax);
}

#[test]
fn add_point() {
    let mut a_box = BndBox::new();
    a_box.update_bounds(0.0, 0.0, 0.0, 1.0, 1.0, 1.0);

    a_box.add_point(Pnt::new(2.0, -1.0, 0.5));

    let l = a_box.get().expect("box not void");
    eq(0.0, l.xmin);
    eq(-1.0, l.ymin);
    eq(0.0, l.zmin);
    eq(2.0, l.xmax);
    eq(1.0, l.ymax);
    eq(1.0, l.zmax);
}

#[test]
fn add_direction() {
    let mut a_box = BndBox::new();
    a_box.update_bounds(0.0, 0.0, 0.0, 1.0, 1.0, 1.0);

    a_box.add_dir(Pnt::dir(1.0, 0.0, 0.0));

    assert!(
        a_box.is_open_xmax(),
        "Positive X direction should open Xmax"
    );
    assert!(
        !a_box.is_open_xmin(),
        "Positive X direction should not open Xmin"
    );
    assert!(!a_box.is_open_ymax(), "X direction should not affect Y");
    assert!(!a_box.is_open_ymin(), "X direction should not affect Y");
}

#[test]
fn add_point_with_direction() {
    let mut a_box = BndBox::new();
    a_box.add_point_dir(Pnt::new(1.0, 2.0, 3.0), Pnt::dir(-1.0, 0.0, 0.0));

    assert!(
        !a_box.is_void(),
        "Box should not be void after adding point"
    );
    assert!(
        a_box.is_open_xmin(),
        "Negative X direction should open Xmin"
    );
    assert!(
        !a_box.is_open_xmax(),
        "Negative X direction should not open Xmax"
    );
}

// ====================== IsOut Methods Tests ======================

#[test]
fn is_out_point() {
    let mut a_box = BndBox::new();
    a_box.update_bounds(0.0, 0.0, 0.0, 2.0, 2.0, 2.0);

    assert!(
        !a_box.is_out_point(Pnt::new(1.0, 1.0, 1.0)),
        "Point inside should not be out"
    );
    assert!(
        a_box.is_out_point(Pnt::new(3.0, 1.0, 1.0)),
        "Point outside should be out"
    );
    assert!(
        !a_box.is_out_point(Pnt::new(2.0, 2.0, 2.0)),
        "Point on boundary should not be out"
    );
}

#[test]
fn is_out_point_with_gap() {
    let mut a_box = BndBox::new();
    a_box.update_bounds(0.0, 0.0, 0.0, 2.0, 2.0, 2.0);
    a_box.set_gap(0.1);

    assert!(
        !a_box.is_out_point(Pnt::new(2.05, 1.0, 1.0)),
        "Point within gap should not be out"
    );
    assert!(
        a_box.is_out_point(Pnt::new(2.15, 1.0, 1.0)),
        "Point beyond gap should be out"
    );
}

#[test]
fn is_out_void_box() {
    let a_void = BndBox::new();
    assert!(
        a_void.is_out_point(Pnt::new(1.0, 2.0, 3.0)),
        "Void box should exclude all points"
    );
}

#[test]
fn is_out_whole_box() {
    let mut a_whole = BndBox::new();
    a_whole.set_whole();
    assert!(
        !a_whole.is_out_point(Pnt::new(1000.0, 2000.0, 3000.0)),
        "Whole box should include all points"
    );
}

#[test]
fn is_out_box() {
    let mut box1 = BndBox::new();
    box1.update_bounds(0.0, 0.0, 0.0, 2.0, 2.0, 2.0);

    let mut overlapping = BndBox::new();
    overlapping.update_bounds(1.0, 1.0, 1.0, 3.0, 3.0, 3.0);

    let mut separate = BndBox::new();
    separate.update_bounds(3.0, 3.0, 3.0, 4.0, 4.0, 4.0);

    let mut touching = BndBox::new();
    touching.update_bounds(2.0, 2.0, 2.0, 3.0, 3.0, 3.0);

    assert!(
        !box1.is_out_box(&overlapping),
        "Overlapping boxes should not be out"
    );
    assert!(box1.is_out_box(&separate), "Separate boxes should be out");
    assert!(
        !box1.is_out_box(&touching),
        "Touching boxes should not be out"
    );
}

#[test]
fn is_out_plane() {
    let mut a_box = BndBox::new();
    a_box.update_bounds(-1.0, -1.0, -1.0, 1.0, 1.0, 1.0);

    let intersecting = Pln::new_pd(Pnt::new(0.0, 0.0, 0.0), Pnt::dir(1.0, 0.0, 0.0));
    let separate = Pln::new_pd(Pnt::new(2.0, 0.0, 0.0), Pnt::dir(1.0, 0.0, 0.0));

    assert!(
        !a_box.is_out_plane(&intersecting),
        "Intersecting plane should not be out"
    );
    assert!(
        a_box.is_out_plane(&separate),
        "Separate plane should be out"
    );
}

#[test]
fn is_out_line() {
    let mut a_box = BndBox::new();
    a_box.update_bounds(0.0, 0.0, 0.0, 2.0, 2.0, 2.0);

    let intersecting = Lin::new_pd(Pnt::new(-1.0, 1.0, 1.0), Pnt::dir(1.0, 0.0, 0.0));
    let separate = Lin::new_pd(Pnt::new(-1.0, 3.0, 1.0), Pnt::dir(1.0, 0.0, 0.0));

    assert!(
        !a_box.is_out_line(&intersecting),
        "Intersecting line should not be out"
    );
    assert!(a_box.is_out_line(&separate), "Separate line should be out");
}

// ====================== Distance Method Tests ======================

#[test]
fn distance() {
    let mut box1 = BndBox::new();
    box1.update_bounds(0.0, 0.0, 0.0, 1.0, 1.0, 1.0);

    let mut box2 = BndBox::new();
    box2.update_bounds(2.0, 0.0, 0.0, 3.0, 1.0, 1.0); // 1 unit away in X

    eq(1.0, box1.distance(&box2));

    let mut overlapping = BndBox::new();
    overlapping.update_bounds(0.5, 0.5, 0.5, 1.5, 1.5, 1.5);
    eq(0.0, box1.distance(&overlapping));
}

// ====================== Open/Close Methods Tests ======================

#[test]
fn open_close_methods() {
    let mut a_box = BndBox::new();
    a_box.update_bounds(0.0, 0.0, 0.0, 1.0, 1.0, 1.0);

    assert!(!a_box.is_open(), "Initially box should not be open");

    a_box.open_xmin();
    assert!(a_box.is_open_xmin(), "OpenXmin should make Xmin open");
    assert!(
        a_box.is_open(),
        "Box with any open side should be considered open"
    );

    a_box.open_xmax();
    a_box.open_ymin();
    a_box.open_ymax();
    a_box.open_zmin();
    a_box.open_zmax();

    assert!(a_box.is_open_xmax());
    assert!(a_box.is_open_ymin());
    assert!(a_box.is_open_ymax());
    assert!(a_box.is_open_zmin());
    assert!(a_box.is_open_zmax());
    assert!(
        a_box.is_whole(),
        "Box open in all directions should be whole"
    );
}

// ====================== Void/Whole State Tests ======================

#[test]
fn void_whole_states() {
    let mut a_box = BndBox::new();

    assert!(a_box.is_void(), "Default box should be void");
    assert!(!a_box.is_whole(), "Default box should not be whole");

    a_box.set_whole();
    assert!(!a_box.is_void(), "Whole box should not be void");
    assert!(a_box.is_whole(), "SetWhole should make box whole");

    a_box.set_void();
    assert!(a_box.is_void(), "SetVoid should make box void");
    assert!(!a_box.is_whole(), "Void box should not be whole");

    a_box.update_bounds(0.0, 0.0, 0.0, 1.0, 1.0, 1.0);
    assert!(!a_box.is_void(), "Box with bounds should not be void");
    assert!(!a_box.is_whole(), "Bounded box should not be whole");
    assert!(
        a_box.has_finite_part(),
        "Bounded box should have finite part"
    );
}

// ====================== Edge Cases and Error Conditions ======================

#[test]
fn edge_cases() {
    let mut a_box = BndBox::new();
    let large = 1e10;
    a_box.update_bounds(-large, -large, -large, large, large, large);
    assert!(!a_box.is_void(), "Box with large values should be valid");

    let mut a_small = BndBox::new();
    a_small.update_bounds(0.0, 0.0, 0.0, 1e-10, 1e-10, 1e-10);
    assert!(
        !a_small.is_void(),
        "Box with tiny dimensions should be valid"
    );
    assert!(
        a_small.is_x_thin(1e-9),
        "Very small box should be considered thin"
    );
}

#[test]
fn transformation_with_open_box() {
    let mut a_box = BndBox::new();
    a_box.update_bounds(0.0, 0.0, 0.0, 1.0, 1.0, 1.0);
    a_box.open_xmax();

    let rotation = Trsf::rotation(Ax1::new(Pnt::origin(), Pnt::dir(0.0, 0.0, 1.0)), PI / 2.0);

    let a_t = a_box.transformed(&rotation);
    assert!(a_t.is_open(), "Transformed open box should remain open");
    assert!(
        !a_t.is_void(),
        "Transformed open box should have finite part"
    );
}

// ====================== Additional Coverage Tests ======================

#[test]
fn square_extent() {
    let mut a_box = BndBox::new();
    a_box.update_bounds(0.0, 0.0, 0.0, 2.0, 3.0, 4.0);
    // diagonal^2 = 2^2 + 3^2 + 4^2 = 29
    eq(29.0, a_box.square_extent());
}

#[test]
fn set_void_and_whole_transitions() {
    let mut a_box = BndBox::new();
    a_box.update_bounds(0.0, 0.0, 0.0, 1.0, 1.0, 1.0);
    assert!(!a_box.is_void(), "Box with bounds should not be void");

    a_box.set_void();
    assert!(a_box.is_void(), "SetVoid should make box void");

    a_box.set_whole();
    assert!(a_box.is_whole(), "SetWhole should make box whole");
    assert!(!a_box.is_void(), "Whole box should not be void");
}

#[test]
fn direct_open_operations() {
    let mut a_box = BndBox::new();
    a_box.update_bounds(0.0, 0.0, 0.0, 1.0, 1.0, 1.0);

    a_box.open_xmin();
    assert!(a_box.is_open_xmin());
    a_box.open_xmax();
    assert!(a_box.is_open_xmax());
    a_box.open_ymin();
    assert!(a_box.is_open_ymin());
    a_box.open_ymax();
    assert!(a_box.is_open_ymax());
    a_box.open_zmin();
    assert!(a_box.is_open_zmin());
    a_box.open_zmax();
    assert!(a_box.is_open_zmax());
}

#[test]
fn is_out_with_transformation() {
    let mut box1 = BndBox::new();
    box1.update_bounds(0.0, 0.0, 0.0, 1.0, 1.0, 1.0);

    let mut box2 = BndBox::new();
    box2.update_bounds(2.0, 0.0, 0.0, 3.0, 1.0, 1.0);

    let mut transf = Trsf::identity();
    assert!(
        box1.is_out_box_trsf(&box2, &transf),
        "Separate boxes should be out"
    );

    transf.set_translation(Pnt::new(-1.5, 0.0, 0.0));
    assert!(
        !box1.is_out_box_trsf(&box2, &transf),
        "Overlapping transformed boxes should not be out"
    );
}

#[test]
fn is_out_with_two_transformations() {
    let mut box1 = BndBox::new();
    box1.update_bounds(0.0, 0.0, 0.0, 1.0, 1.0, 1.0);

    let mut box2 = BndBox::new();
    box2.update_bounds(0.0, 0.0, 0.0, 1.0, 1.0, 1.0);

    let mut t1 = Trsf::identity();
    let mut t2 = Trsf::identity();
    t1.set_translation(Pnt::new(0.0, 0.0, 0.0));
    t2.set_translation(Pnt::new(2.0, 0.0, 0.0));

    assert!(
        box1.is_out_box_trsf2(&t1, &box2, &t2),
        "Separated transformed boxes should be out"
    );
}

#[test]
fn is_out_line_segment() {
    let mut a_box = BndBox::new();
    a_box.update_bounds(0.0, 0.0, 0.0, 2.0, 2.0, 2.0);

    let p1 = Pnt::new(-1.0, 1.0, 1.0);
    let p2 = Pnt::new(3.0, 1.0, 1.0);
    let dir = Pnt::dir(1.0, 0.0, 0.0);
    assert!(
        !a_box.is_out_band(p1, p2, dir),
        "Line segment through box should not be out"
    );

    let p3 = Pnt::new(-1.0, 3.0, 1.0);
    let p4 = Pnt::new(3.0, 3.0, 1.0);
    assert!(
        a_box.is_out_band(p3, p4, dir),
        "Line segment missing box should be out"
    );
}

#[test]
fn update_expands_correctly() {
    let mut a_box = BndBox::new();

    a_box.update_point(1.0, 1.0, 1.0);
    let l = a_box.get().expect("box not void");
    eq(1.0, l.xmin);
    eq(1.0, l.xmax);

    a_box.update_point(-0.5, 2.5, 0.5);
    let l = a_box.get().expect("box not void");
    eq(-0.5, l.xmin);
    eq(1.0, l.xmax);
    eq(1.0, l.ymin);
    eq(2.5, l.ymax);
}

#[test]
fn dump_json_and_init_from_json() {
    let mut a_box = BndBox::from_corners(Pnt::new(0.0, 0.0, 0.0), Pnt::new(10.0, 5.0, 3.0));
    a_box.set_gap(0.1);

    let json = a_box.dump_json();

    let mut deserialized = BndBox::new();
    assert!(
        deserialized.init_from_json(&json),
        "Deserialization should succeed with proper JSON format"
    );

    let l1 = a_box.get().expect("box not void");
    let l2 = deserialized.get().expect("box not void");

    eq(l1.xmin, l2.xmin);
    eq(l1.ymin, l2.ymin);
    eq(l1.zmin, l2.zmin);
    eq(l1.xmax, l2.xmax);
    eq(l1.ymax, l2.ymax);
    eq(l1.zmax, l2.zmax);
    eq(a_box.gap(), deserialized.gap());
}

// ====================== Regression Tests ======================

// OCC16485: Bnd_Box tolerance issue with cumulative enlargement.
#[test]
fn occ16485_cumulative_enlarge_tolerance() {
    let a_tol = 1e-3;
    let a_nb_step = 1000;
    let mut a_box = BndBox::new();

    for i in 0..=a_nb_step {
        let p = Pnt::new(i as f64, 0.0, 0.0);
        let mut b = BndBox::new();
        b.add_point(p);
        b.enlarge(a_tol);
        b.add(&a_box);
        a_box = b;
    }

    let l = a_box.get().expect("box not void");
    near(-a_tol, l.xmin, 1e-10);
    near(a_nb_step as f64 + a_tol, l.xmax, 1e-10);
}

#[test]
fn contains_point() {
    let a_box = BndBox::from_corners(Pnt::new(0.0, 0.0, 0.0), Pnt::new(10.0, 10.0, 10.0));
    assert!(a_box.contains(Pnt::new(5.0, 5.0, 5.0)));
    assert!(a_box.contains(Pnt::new(0.0, 0.0, 0.0)));
    assert!(!a_box.contains(Pnt::new(-1.0, 5.0, 5.0)));
    assert!(!a_box.contains(Pnt::new(5.0, 5.0, 11.0)));
}

#[test]
fn intersects_box() {
    let box1 = BndBox::from_corners(Pnt::new(0.0, 0.0, 0.0), Pnt::new(10.0, 10.0, 10.0));
    let box2 = BndBox::from_corners(Pnt::new(5.0, 5.0, 5.0), Pnt::new(15.0, 15.0, 15.0));
    let box3 = BndBox::from_corners(Pnt::new(20.0, 20.0, 20.0), Pnt::new(30.0, 30.0, 30.0));
    assert!(box1.intersects(&box2));
    assert!(!box1.intersects(&box3));
}

#[test]
fn center() {
    let a_box = BndBox::from_corners(Pnt::new(0.0, 0.0, 0.0), Pnt::new(10.0, 20.0, 30.0));
    let c = a_box.center().expect("box not void");
    eq(c.x, 5.0);
    eq(c.y, 10.0);
    eq(c.z, 15.0);
}

#[test]
fn center_void_nullopt() {
    let a_void = BndBox::new();
    assert!(a_void.center().is_none());
}
