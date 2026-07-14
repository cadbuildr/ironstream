use ironstream::poly_algo::*;

#[test]
fn merge_nodes_within_tolerance() {
    let mut m = PolyMergeNodesTool::new(0.01);
    let i0 = m.add_node([0.0, 0.0, 0.0]);
    let i1 = m.add_node([0.005, 0.0, 0.0]); // within tol -> merged
    let i2 = m.add_node([1.0, 0.0, 0.0]);
    assert_eq!(i0, 0);
    assert_eq!(i1, 0);
    assert_ne!(i2, 0);
    assert_eq!(m.nb_merged(), 2);
    assert_eq!(m.nb_duplicates_removed(), 1);
}

#[test]
fn merge_nodes_canonical_index() {
    let mut m = PolyMergeNodesTool::new(0.001);
    m.add_node([0.0, 0.0, 0.0]);
    m.add_node([1.0, 0.0, 0.0]);
    assert_eq!(m.canonical_index(0), Some(0));
    assert_eq!(m.canonical_index(1), Some(1));
    assert_eq!(m.canonical_index(99), None);
}

#[test]
fn coherence_patch_edge_stats() {
    let mut p = PolyCoherencePatch::new();
    p.add_triangle(0, 1, 2);
    p.add_triangle(1, 2, 3);
    p.compute_edge_stats();
    assert_eq!(p.nb_triangles(), 2);
    assert!(p.nb_free_edges > 0);
    assert!(!p.is_closed());
}

#[test]
fn self_intersection_filter() {
    let nodes = [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 0.0, 0.005]];
    let mut f = PolySelfIntersectionNodeFilter::new();
    f.filter(&nodes, 0.01);
    assert_eq!(f.nb_removed(), 1);
    assert!(f.is_removed(2));
}

#[test]
fn mesh_quality_equilateral() {
    let nodes = [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.5, 0.866, 0.0]];
    let tris = [[0usize, 1, 2]];
    let mut q = PolyMeshQuality::new();
    q.analyze(&nodes, &tris);
    assert!(q.min_aspect_ratio > 0.9 && q.min_aspect_ratio < 1.2);
    assert_eq!(q.nb_degenerate, 0);
}

#[test]
fn mesh_quality_degenerate() {
    let nodes = [[0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]];
    let tris = [[0usize, 1, 2]];
    let mut q = PolyMeshQuality::new();
    q.analyze(&nodes, &tris);
    assert_eq!(q.nb_degenerate, 1);
}
