use ironstream::bndlib_ext::*;

#[test]
fn bbox3d_void_by_default() {
    let b = Bbox3d::new();
    assert!(b.is_void());
    assert!((b.diagonal() - 0.0).abs() < 1e-10);
}

#[test]
fn bbox3d_enlarge_expands_bounds() {
    let mut b = Bbox3d::new();
    b.add_point([0.0, 0.0, 0.0]);
    b.add_point([2.0, 2.0, 2.0]);
    b.enlarge(0.5);
    assert!((b.min[0] - (-0.5)).abs() < 1e-10);
    assert!((b.max[0] - 2.5).abs() < 1e-10);
}

#[test]
fn bbox3d_merged_with_void() {
    let mut b = Bbox3d::new();
    b.add_point([1.0, 1.0, 1.0]);
    let empty = Bbox3d::new();
    let merged = b.merged_with(&empty);
    assert_eq!(merged.min, [1.0, 1.0, 1.0]);
    assert_eq!(merged.max, [1.0, 1.0, 1.0]);
}

#[test]
fn bbox3d_no_intersection_with_void() {
    let mut b = Bbox3d::new();
    b.add_point([0.0, 0.0, 0.0]);
    b.add_point([1.0, 1.0, 1.0]);
    let empty = Bbox3d::new();
    assert!(!b.intersects(&empty));
    assert!(!empty.intersects(&b));
}

#[test]
fn bbox2d_area_computation() {
    let mut b = Bbox2d::new();
    b.add_point([1.0, 2.0]);
    b.add_point([4.0, 6.0]);
    assert!((b.area() - 12.0).abs() < 1e-10);
}

#[test]
fn bndlib_add_surface_plane_patch() {
    let corners = [
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [1.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
    ];
    let b = BndLibAddSurface::add_plane_patch(&corners, 0.0);
    assert!(!b.is_void());
    assert!(b.contains([0.5, 0.5, 0.0]));
}
