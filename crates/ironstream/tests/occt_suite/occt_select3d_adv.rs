// FILE: tests/occt_select3d_adv.rs
extern crate ironstream;
use ironstream::select3d_adv::*;

#[test]
fn test_sensitive_box_hit_and_miss() {
    let b = SensitiveBox::new(0, [0.0, 0.0, 0.0], [2.0, 2.0, 2.0]);
    assert!(b.contains_point(&[1.0, 1.0, 1.0]));
    assert!(!b.contains_point(&[3.0, 1.0, 1.0]));
    let hit = b.test_point([1.0, 1.0, 1.0]);
    assert!(hit.is_inside);
    assert_eq!(hit.sensitive_index, 0);
    let miss = b.test_point([5.0, 5.0, 5.0]);
    assert!(!miss.is_inside);
    assert!(miss.depth.is_infinite());
}

#[test]
fn test_sensitive_box_geometry() {
    let b = SensitiveBox::new(7, [0.0, 0.0, 0.0], [2.0, 2.0, 2.0]);
    let c = b.center();
    assert!((c[0] - 1.0).abs() < 1e-12);
    let diag = b.diagonal();
    assert!((diag - (12.0_f64).sqrt()).abs() < 1e-12);
}

#[test]
fn test_sensitive_sphere() {
    let s = SensitiveSphere::new(1, [0.0, 0.0, 0.0], 1.0);
    assert!(s.contains_point(&[0.5, 0.0, 0.0]));
    assert!(!s.contains_point(&[2.0, 0.0, 0.0]));
    assert!((s.surface_area() - 4.0 * std::f64::consts::PI).abs() < 1e-10);
    let hit = s.test_point([0.0, 0.0, 0.0]);
    assert!(hit.is_inside);
}

#[test]
fn test_sensitive_triangulation() {
    let verts = vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.5, 1.0, 0.0]];
    let tris = vec![[0u32, 1, 2]];
    let st = SensitiveTriangulation::new(42, verts, tris);
    assert_eq!(st.nb_triangles(), 1);
    assert_eq!(st.nb_vertices(), 3);
    let c = st.triangle_center(0).unwrap();
    assert!((c[1] - 1.0 / 3.0).abs() < 1e-12);
    let (mn, mx) = st.bounding_box();
    assert!(mn[0].abs() < 1e-12);
    assert!((mx[1] - 1.0).abs() < 1e-12);
}

#[test]
fn test_bvh_tree_select_overlapping() {
    let mut tree = Select3dBvhTree::new();
    tree.add_leaf(0, [0.0, 0.0, 0.0], [1.0, 1.0, 1.0]);
    tree.add_leaf(1, [2.0, 0.0, 0.0], [3.0, 1.0, 1.0]);
    tree.add_leaf(2, [0.0, 0.0, 0.0], [0.5, 0.5, 0.5]);
    assert_eq!(tree.nb_leaves(), 3);
    let hits = tree.select_overlapping([0.3, 0.0, 0.0], [1.5, 1.0, 1.0]);
    // Leaf 0 and leaf 2 overlap; leaf 1 does not
    assert!(hits.contains(&0));
    assert!(hits.contains(&2));
    assert!(!hits.contains(&1));
}
