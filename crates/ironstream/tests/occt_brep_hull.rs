// FILE: rust/ironstream/crates/ironstream/tests/occt_brep_hull.rs
extern crate ironstream;
use ironstream::brep_hull::*;

// ── HullVertex ───────────────────────────────────────────────────────────────

#[test]
fn hull_vertex_new_and_accessors() {
    let v = HullVertex::new([1.0, 2.0, 3.0], 7);
    assert_eq!(v.point(), [1.0, 2.0, 3.0]);
    assert_eq!(v.index(), 7);
}

#[test]
fn hull_vertex_zero_index() {
    let v = HullVertex::new([0.0, 0.0, 0.0], 0);
    assert_eq!(v.index(), 0);
    assert_eq!(v.point(), [0.0, 0.0, 0.0]);
}

#[test]
fn hull_vertex_clone_eq() {
    let v = HullVertex::new([-1.0, 5.5, 99.0], 3);
    let w = v.clone();
    assert_eq!(v, w);
}

// ── ConvexHull3D ─────────────────────────────────────────────────────────────

#[test]
fn hull3d_new_not_done() {
    let h = ConvexHull3D::new();
    assert!(!h.is_done());
    assert_eq!(h.nb_vertices(), 0);
    assert_eq!(h.nb_faces(), 0);
}

#[test]
fn hull3d_default_not_done() {
    let h = ConvexHull3D::default();
    assert!(!h.is_done());
}

#[test]
fn hull3d_perform_sets_done() {
    let mut h = ConvexHull3D::new();
    h.perform(vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]);
    assert!(h.is_done());
}

#[test]
fn hull3d_perform_stores_all_points() {
    let pts = vec![
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 0.0, 1.0],
    ];
    let mut h = ConvexHull3D::new();
    h.perform(pts.clone());
    assert_eq!(h.nb_vertices(), 4);
    for (i, pt) in pts.iter().enumerate() {
        assert_eq!(h.vertex(i).point(), *pt);
        assert_eq!(h.vertex(i).index(), i);
    }
}

#[test]
fn hull3d_perform_empty_input() {
    let mut h = ConvexHull3D::new();
    h.perform(vec![]);
    assert!(h.is_done());
    assert_eq!(h.nb_vertices(), 0);
    assert_eq!(h.nb_faces(), 0);
}

#[test]
fn hull3d_nb_faces_zero_for_stub() {
    let mut h = ConvexHull3D::new();
    h.perform(vec![[0.0, 0.0, 0.0], [1.0, 1.0, 1.0], [2.0, 0.0, 0.0]]);
    assert_eq!(h.nb_faces(), 0);
}

#[test]
fn hull3d_perform_replaces_previous() {
    let mut h = ConvexHull3D::new();
    h.perform(vec![[0.0, 0.0, 0.0]; 10]);
    assert_eq!(h.nb_vertices(), 10);
    h.perform(vec![[1.0, 1.0, 1.0]; 3]);
    assert_eq!(h.nb_vertices(), 3);
}

#[test]
fn hull3d_bounding_box_empty() {
    let h = ConvexHull3D::new();
    let (lo, hi) = h.bounding_box();
    assert_eq!(lo, [0.0, 0.0, 0.0]);
    assert_eq!(hi, [0.0, 0.0, 0.0]);
}

#[test]
fn hull3d_bounding_box_single_point() {
    let mut h = ConvexHull3D::new();
    h.perform(vec![[3.0, -1.0, 5.0]]);
    let (lo, hi) = h.bounding_box();
    assert_eq!(lo, [3.0, -1.0, 5.0]);
    assert_eq!(hi, [3.0, -1.0, 5.0]);
}

#[test]
fn hull3d_bounding_box_multiple_points() {
    let mut h = ConvexHull3D::new();
    h.perform(vec![
        [1.0, 2.0, 3.0],
        [-1.0, 5.0, 0.0],
        [4.0, -3.0, 7.0],
        [0.0, 0.0, 0.0],
    ]);
    let (lo, hi) = h.bounding_box();
    assert_eq!(lo, [-1.0, -3.0, 0.0]);
    assert_eq!(hi, [4.0, 5.0, 7.0]);
}

#[test]
fn hull3d_bounding_box_negative_coords() {
    let mut h = ConvexHull3D::new();
    h.perform(vec![[-10.0, -20.0, -30.0], [-1.0, -2.0, -3.0]]);
    let (lo, hi) = h.bounding_box();
    assert_eq!(lo, [-10.0, -20.0, -30.0]);
    assert_eq!(hi, [-1.0, -2.0, -3.0]);
}

#[test]
fn hull3d_vertex_indices_sequential() {
    let mut h = ConvexHull3D::new();
    let pts: Vec<[f64; 3]> = (0..5).map(|i| [i as f64, 0.0, 0.0]).collect();
    h.perform(pts);
    for i in 0..5 {
        assert_eq!(h.vertex(i).index(), i);
    }
}

// ── ConvexHull2D ─────────────────────────────────────────────────────────────

#[test]
fn hull2d_new_not_done() {
    let h = ConvexHull2D::new();
    assert!(!h.is_done());
    assert_eq!(h.nb_vertices(), 0);
}

#[test]
fn hull2d_default_not_done() {
    let h = ConvexHull2D::default();
    assert!(!h.is_done());
}

#[test]
fn hull2d_perform_sets_done() {
    let mut h = ConvexHull2D::new();
    h.perform(vec![[0.0, 0.0], [1.0, 0.0]]);
    assert!(h.is_done());
}

#[test]
fn hull2d_perform_stores_points() {
    let pts = vec![[0.0, 0.0], [1.0, 0.0], [0.5, 1.0]];
    let mut h = ConvexHull2D::new();
    h.perform(pts.clone());
    assert_eq!(h.nb_vertices(), 3);
    for (i, &pt) in pts.iter().enumerate() {
        assert_eq!(h.vertex(i), pt);
    }
}

#[test]
fn hull2d_perform_empty_input() {
    let mut h = ConvexHull2D::new();
    h.perform(vec![]);
    assert!(h.is_done());
    assert_eq!(h.nb_vertices(), 0);
}

#[test]
fn hull2d_perform_replaces_previous() {
    let mut h = ConvexHull2D::new();
    h.perform(vec![[0.0, 0.0]; 6]);
    assert_eq!(h.nb_vertices(), 6);
    h.perform(vec![[1.0, 1.0]; 2]);
    assert_eq!(h.nb_vertices(), 2);
}

#[test]
fn hull2d_area_approx_zero_for_empty() {
    let h = ConvexHull2D::new();
    assert_eq!(h.area_approx(), 0.0);
}

#[test]
fn hull2d_area_approx_zero_for_one_point() {
    let mut h = ConvexHull2D::new();
    h.perform(vec![[1.0, 2.0]]);
    assert_eq!(h.area_approx(), 0.0);
}

#[test]
fn hull2d_area_approx_zero_for_two_points() {
    let mut h = ConvexHull2D::new();
    h.perform(vec![[0.0, 0.0], [1.0, 0.0]]);
    assert_eq!(h.area_approx(), 0.0);
}

#[test]
fn hull2d_area_approx_unit_square() {
    let mut h = ConvexHull2D::new();
    // CCW unit square
    h.perform(vec![
        [0.0, 0.0],
        [1.0, 0.0],
        [1.0, 1.0],
        [0.0, 1.0],
    ]);
    let area = h.area_approx();
    assert!((area - 1.0).abs() < 1e-12, "expected 1.0 got {area}");
}

#[test]
fn hull2d_area_approx_right_triangle() {
    let mut h = ConvexHull2D::new();
    // Right triangle with legs 3 and 4 → area = 6
    h.perform(vec![[0.0, 0.0], [3.0, 0.0], [0.0, 4.0]]);
    let area = h.area_approx();
    assert!((area - 6.0).abs() < 1e-12, "expected 6.0 got {area}");
}

#[test]
fn hull2d_area_approx_cw_same_as_ccw() {
    let mut h_ccw = ConvexHull2D::new();
    h_ccw.perform(vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]]);

    let mut h_cw = ConvexHull2D::new();
    h_cw.perform(vec![[0.0, 1.0], [1.0, 1.0], [1.0, 0.0], [0.0, 0.0]]);

    let diff = (h_ccw.area_approx() - h_cw.area_approx()).abs();
    assert!(diff < 1e-12);
}

#[test]
fn hull2d_area_approx_rectangle() {
    let mut h = ConvexHull2D::new();
    h.perform(vec![
        [0.0, 0.0],
        [5.0, 0.0],
        [5.0, 3.0],
        [0.0, 3.0],
    ]);
    let area = h.area_approx();
    assert!((area - 15.0).abs() < 1e-12, "expected 15.0 got {area}");
}
