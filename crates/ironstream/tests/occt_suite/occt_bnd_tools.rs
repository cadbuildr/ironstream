// FILE: rust/ironstream/crates/ironstream/tests/occt_bnd_tools.rs
extern crate ironstream;

use ironstream::bnd_tools::{BndLibBox, BndLibSphereComputer, BndToolsBox2d, BndToolsBox3d};

#[test]
fn test_void_box_is_void() {
    let b = BndLibBox::new_void();
    assert!(b.is_void());
    assert_eq!(b.diagonal(), 0.0);
}

#[test]
fn test_void_box_is_out_point() {
    let b = BndLibBox::new_void();
    assert!(b.is_out_point(0.0, 0.0, 0.0));
}

#[test]
fn test_sphere_box_bounds() {
    let b = BndLibBox::new_from_sphere(1.0, 2.0, 3.0, 5.0, 0.1);
    assert!(!b.is_void());
    assert!((b.xmin - (1.0 - 5.0 - 0.1)).abs() < 1e-12);
    assert!((b.xmax - (1.0 + 5.0 + 0.1)).abs() < 1e-12);
    assert!((b.ymin - (2.0 - 5.0 - 0.1)).abs() < 1e-12);
    assert!((b.ymax - (2.0 + 5.0 + 0.1)).abs() < 1e-12);
    assert!((b.zmin - (3.0 - 5.0 - 0.1)).abs() < 1e-12);
    assert!((b.zmax - (3.0 + 5.0 + 0.1)).abs() < 1e-12);
}

#[test]
fn test_cylinder_box_bounds() {
    let b = BndLibBox::new_from_cylinder(0.0, 0.0, -2.0, 4.0, 3.0, 0.0);
    assert!(!b.is_void());
    assert!((b.xmin - (-3.0)).abs() < 1e-12);
    assert!((b.xmax - 3.0).abs() < 1e-12);
    assert!((b.zmin - (-2.0)).abs() < 1e-12);
    assert!((b.zmax - 4.0).abs() < 1e-12);
}

#[test]
fn test_box_points_bounds() {
    let b = BndLibBox::new_from_box_points(1.0, 3.0, 2.0, 4.0, 0.0, 5.0, 0.5);
    assert!((b.xmin - 0.5).abs() < 1e-12);
    assert!((b.xmax - 3.5).abs() < 1e-12);
    assert!((b.ymin - 1.5).abs() < 1e-12);
    assert!((b.ymax - 4.5).abs() < 1e-12);
    assert!((b.zmin - (-0.5)).abs() < 1e-12);
    assert!((b.zmax - 5.5).abs() < 1e-12);
}

#[test]
fn test_diagonal_unit_cube() {
    let b = BndLibBox::new_from_box_points(0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0);
    let d = b.diagonal();
    assert!((d - 3.0_f64.sqrt()).abs() < 1e-12);
}

#[test]
fn test_is_out_point_inside() {
    let b = BndLibBox::new_from_sphere(0.0, 0.0, 0.0, 10.0, 0.0);
    assert!(!b.is_out_point(0.0, 0.0, 0.0));
    assert!(!b.is_out_point(5.0, 5.0, 5.0));
}

#[test]
fn test_is_out_point_outside() {
    let b = BndLibBox::new_from_sphere(0.0, 0.0, 0.0, 1.0, 0.0);
    assert!(b.is_out_point(2.0, 0.0, 0.0));
    assert!(b.is_out_point(-2.0, 0.0, 0.0));
    assert!(b.is_out_point(0.0, 0.0, 5.0));
}

#[test]
fn test_add_point_to_void() {
    let mut b = BndLibBox::new_void();
    b.add_point(1.0, 2.0, 3.0);
    assert!(!b.is_void());
    assert!((b.xmin - 1.0).abs() < 1e-12);
    assert!((b.xmax - 1.0).abs() < 1e-12);
    b.add_point(4.0, 0.0, 6.0);
    assert!((b.xmin - 1.0).abs() < 1e-12);
    assert!((b.xmax - 4.0).abs() < 1e-12);
    assert!((b.ymin - 0.0).abs() < 1e-12);
    assert!((b.ymax - 2.0).abs() < 1e-12);
    assert!((b.zmax - 6.0).abs() < 1e-12);
}

#[test]
fn test_enlarge() {
    let mut b = BndLibBox::new_from_box_points(0.0, 2.0, 0.0, 2.0, 0.0, 2.0, 0.0);
    b.enlarge(1.0);
    assert!((b.xmin - (-1.0)).abs() < 1e-12);
    assert!((b.xmax - 3.0).abs() < 1e-12);
    assert!((b.gap - 1.0).abs() < 1e-12);
}

#[test]
fn test_merged_two_boxes() {
    let a = BndLibBox::new_from_box_points(0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0);
    let b = BndLibBox::new_from_box_points(2.0, 3.0, 2.0, 3.0, 2.0, 3.0, 0.0);
    let m = a.merged(&b);
    assert!((m.xmin - 0.0).abs() < 1e-12);
    assert!((m.xmax - 3.0).abs() < 1e-12);
    assert!((m.ymin - 0.0).abs() < 1e-12);
    assert!((m.ymax - 3.0).abs() < 1e-12);
    assert!((m.zmin - 0.0).abs() < 1e-12);
    assert!((m.zmax - 3.0).abs() < 1e-12);
}

#[test]
fn test_merged_with_void() {
    let a = BndLibBox::new_from_box_points(1.0, 2.0, 1.0, 2.0, 1.0, 2.0, 0.0);
    let void = BndLibBox::new_void();
    let m = a.merged(&void);
    assert!(!m.is_void());
    assert!((m.xmin - 1.0).abs() < 1e-12);
    assert!((m.xmax - 2.0).abs() < 1e-12);
}

#[test]
fn test_sphere_computer_empty() {
    let sc = BndLibSphereComputer::new();
    let (cx, cy, cz, r) = sc.compute();
    assert_eq!(cx, 0.0);
    assert_eq!(cy, 0.0);
    assert_eq!(cz, 0.0);
    assert_eq!(r, 0.0);
}

#[test]
fn test_sphere_computer_single_point() {
    let mut sc = BndLibSphereComputer::new();
    sc.add(3.0, 4.0, 5.0);
    let (cx, cy, cz, r) = sc.compute();
    assert!((cx - 3.0).abs() < 1e-12);
    assert!((cy - 4.0).abs() < 1e-12);
    assert!((cz - 5.0).abs() < 1e-12);
    assert!(r.abs() < 1e-12);
}

#[test]
fn test_sphere_computer_symmetric_points() {
    let mut sc = BndLibSphereComputer::new();
    sc.add(1.0, 0.0, 0.0);
    sc.add(-1.0, 0.0, 0.0);
    sc.add(0.0, 1.0, 0.0);
    sc.add(0.0, -1.0, 0.0);
    let (cx, cy, cz, r) = sc.compute();
    assert!(cx.abs() < 1e-12);
    assert!(cy.abs() < 1e-12);
    assert!(cz.abs() < 1e-12);
    assert!((r - 1.0).abs() < 1e-12);
}

// ---- BndToolsBox3d ----------------------------------------------------------

#[test]
fn test_box3d_new_is_void() {
    let b = BndToolsBox3d::new();
    assert!(b.is_void());
}

#[test]
fn test_box3d_size_and_center_when_void() {
    let b = BndToolsBox3d::new();
    assert_eq!(b.size(), [0.0, 0.0, 0.0]);
    assert_eq!(b.center(), [0.0, 0.0, 0.0]);
}

#[test]
fn test_box3d_add_single_point() {
    let mut b = BndToolsBox3d::new();
    b.add_point([1.0, 2.0, 3.0]);
    assert!(!b.is_void());
    assert_eq!(b.min(), [1.0, 2.0, 3.0]);
    assert_eq!(b.max(), [1.0, 2.0, 3.0]);
    assert_eq!(b.size(), [0.0, 0.0, 0.0]);
    assert_eq!(b.center(), [1.0, 2.0, 3.0]);
}

#[test]
fn test_box3d_add_two_points() {
    let mut b = BndToolsBox3d::new();
    b.add_point([0.0, 0.0, 0.0]);
    b.add_point([2.0, 4.0, 6.0]);
    assert_eq!(b.min(), [0.0, 0.0, 0.0]);
    assert_eq!(b.max(), [2.0, 4.0, 6.0]);
    assert_eq!(b.size(), [2.0, 4.0, 6.0]);
    let c = b.center();
    assert!((c[0] - 1.0).abs() < 1e-12);
    assert!((c[1] - 2.0).abs() < 1e-12);
    assert!((c[2] - 3.0).abs() < 1e-12);
}

#[test]
fn test_box3d_add_point_expands_existing() {
    let mut b = BndToolsBox3d::new();
    b.add_point([1.0, 1.0, 1.0]);
    b.add_point([-1.0, 3.0, 0.5]);
    assert_eq!(b.min(), [-1.0, 1.0, 0.5]);
    assert_eq!(b.max(), [1.0, 3.0, 1.0]);
}

#[test]
fn test_box3d_add_box_void_into_void() {
    let mut a = BndToolsBox3d::new();
    let b = BndToolsBox3d::new();
    a.add_box(&b);
    assert!(a.is_void());
}

#[test]
fn test_box3d_add_void_box_noop() {
    let mut a = BndToolsBox3d::new();
    a.add_point([1.0, 2.0, 3.0]);
    let b = BndToolsBox3d::new();
    a.add_box(&b);
    assert_eq!(a.min(), [1.0, 2.0, 3.0]);
    assert_eq!(a.max(), [1.0, 2.0, 3.0]);
}

#[test]
fn test_box3d_add_box_into_void() {
    let mut a = BndToolsBox3d::new();
    let mut b = BndToolsBox3d::new();
    b.add_point([5.0, 6.0, 7.0]);
    a.add_box(&b);
    assert!(!a.is_void());
    assert_eq!(a.min(), [5.0, 6.0, 7.0]);
    assert_eq!(a.max(), [5.0, 6.0, 7.0]);
}

#[test]
fn test_box3d_add_box_union() {
    let mut a = BndToolsBox3d::new();
    a.add_point([0.0, 0.0, 0.0]);
    a.add_point([1.0, 1.0, 1.0]);

    let mut b = BndToolsBox3d::new();
    b.add_point([2.0, 2.0, 2.0]);
    b.add_point([3.0, 3.0, 3.0]);

    a.add_box(&b);
    assert_eq!(a.min(), [0.0, 0.0, 0.0]);
    assert_eq!(a.max(), [3.0, 3.0, 3.0]);
}

#[test]
fn test_box3d_enlarge() {
    let mut b = BndToolsBox3d::new();
    b.add_point([0.0, 0.0, 0.0]);
    b.add_point([2.0, 2.0, 2.0]);
    b.enlarge(1.0);
    assert_eq!(b.min(), [-1.0, -1.0, -1.0]);
    assert_eq!(b.max(), [3.0, 3.0, 3.0]);
}

#[test]
fn test_box3d_enlarge_void_noop() {
    let mut b = BndToolsBox3d::new();
    b.enlarge(5.0);
    assert!(b.is_void());
}

// ---- BndToolsBox2d ----------------------------------------------------------

#[test]
fn test_box2d_new_is_void() {
    let b = BndToolsBox2d::new();
    assert!(b.is_void());
}

#[test]
fn test_box2d_center_when_void() {
    let b = BndToolsBox2d::new();
    assert_eq!(b.center(), [0.0, 0.0]);
}

#[test]
fn test_box2d_add_single_point() {
    let mut b = BndToolsBox2d::new();
    b.add_point([3.0, 4.0]);
    assert!(!b.is_void());
    assert_eq!(b.min(), [3.0, 4.0]);
    assert_eq!(b.max(), [3.0, 4.0]);
    assert_eq!(b.center(), [3.0, 4.0]);
}

#[test]
fn test_box2d_add_two_points() {
    let mut b = BndToolsBox2d::new();
    b.add_point([-1.0, -2.0]);
    b.add_point([1.0, 2.0]);
    assert_eq!(b.min(), [-1.0, -2.0]);
    assert_eq!(b.max(), [1.0, 2.0]);
    let c = b.center();
    assert!(c[0].abs() < 1e-12);
    assert!(c[1].abs() < 1e-12);
}

#[test]
fn test_box2d_add_point_expands() {
    let mut b = BndToolsBox2d::new();
    b.add_point([5.0, 5.0]);
    b.add_point([3.0, 7.0]);
    assert_eq!(b.min(), [3.0, 5.0]);
    assert_eq!(b.max(), [5.0, 7.0]);
}

#[test]
fn test_box2d_add_box_void_into_void() {
    let mut a = BndToolsBox2d::new();
    let b = BndToolsBox2d::new();
    a.add_box(&b);
    assert!(a.is_void());
}

#[test]
fn test_box2d_add_void_box_noop() {
    let mut a = BndToolsBox2d::new();
    a.add_point([1.0, 2.0]);
    let b = BndToolsBox2d::new();
    a.add_box(&b);
    assert_eq!(a.min(), [1.0, 2.0]);
    assert_eq!(a.max(), [1.0, 2.0]);
}

#[test]
fn test_box2d_add_box_into_void() {
    let mut a = BndToolsBox2d::new();
    let mut b = BndToolsBox2d::new();
    b.add_point([10.0, 20.0]);
    a.add_box(&b);
    assert!(!a.is_void());
    assert_eq!(a.min(), [10.0, 20.0]);
    assert_eq!(a.max(), [10.0, 20.0]);
}

#[test]
fn test_box2d_add_box_union() {
    let mut a = BndToolsBox2d::new();
    a.add_point([0.0, 0.0]);
    a.add_point([1.0, 1.0]);

    let mut b = BndToolsBox2d::new();
    b.add_point([2.0, -1.0]);
    b.add_point([3.0, 4.0]);

    a.add_box(&b);
    assert_eq!(a.min(), [0.0, -1.0]);
    assert_eq!(a.max(), [3.0, 4.0]);
}

#[test]
fn test_box2d_enlarge() {
    let mut b = BndToolsBox2d::new();
    b.add_point([0.0, 0.0]);
    b.add_point([4.0, 6.0]);
    b.enlarge(0.5);
    let mn = b.min();
    let mx = b.max();
    assert!((mn[0] - (-0.5)).abs() < 1e-12);
    assert!((mn[1] - (-0.5)).abs() < 1e-12);
    assert!((mx[0] - 4.5).abs() < 1e-12);
    assert!((mx[1] - 6.5).abs() < 1e-12);
}

#[test]
fn test_box2d_enlarge_void_noop() {
    let mut b = BndToolsBox2d::new();
    b.enlarge(5.0);
    assert!(b.is_void());
}
