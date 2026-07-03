// FILE: tests/occt_hlr_ext.rs
extern crate ironstream;
use ironstream::hlr_ext::*;

#[test]
fn projector_orthographic_standard_front() {
    let proj = HlrProjector::standard_front();
    assert_eq!(proj.proj_type, ProjectionType::Orthographic);
    // Standard front: view along -Y, up = Z
    // right = cross([-Y], [+Z]) = [-X], up = cross([-X], [-Y]) = [+Z]
    // For point [5, 100, 3]: x_proj = dot([5,100,3], right), y_proj = dot([5,100,3], up_vec)
    // The projection magnitude in Z (height) should be 3
    let p = proj.project([0.0, 100.0, 3.0]);
    assert!((p[1] - 3.0).abs() < 1e-8, "z-coord should map to y in front view, got {}", p[1]);
}

#[test]
fn projector_standard_views_construct() {
    let _top = HlrProjector::standard_top();
    let _right = HlrProjector::standard_right();
    let _iso = HlrProjector::isometric();
}

#[test]
fn hlr_edge_length_2d() {
    let e = HlrEdge::new(0, [0.0, 0.0], [3.0, 4.0], EdgeVisibility::Visible);
    assert!((e.length_2d() - 5.0).abs() < 1e-10);
}

#[test]
fn hlr_poly_algo_classify_visible_and_hidden() {
    let proj = HlrProjector::standard_front();
    let edges = vec![
        ([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], 0.0),  // visible
        ([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 1.0),  // hidden
        ([1.0, 0.0, 0.0], [2.0, 0.0, 0.0], 0.0),  // visible
    ];
    let r = HlrPolyAlgo::compute(&proj, &edges);
    assert_eq!(r.nb_visible, 2);
    assert_eq!(r.nb_hidden, 1);
    assert_eq!(r.visible_edges().len(), 2);
    assert_eq!(r.hidden_edges().len(), 1);
}

#[test]
fn hlr_result_edge_count() {
    let proj = HlrProjector::standard_top();
    let edges: Vec<_> = (0..5).map(|i| {
        ([i as f64, 0.0, 0.0], [(i + 1) as f64, 0.0, 0.0], 0.0)
    }).collect();
    let r = HlrPolyAlgo::compute(&proj, &edges);
    assert_eq!(r.edges.len(), 5);
    assert_eq!(r.nb_visible, 5);
}
