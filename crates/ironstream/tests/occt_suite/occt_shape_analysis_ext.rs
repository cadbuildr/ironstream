// FILE: tests/occt_shape_analysis_ext.rs
extern crate ironstream;
use ironstream::shape_analysis_ext::*;

#[test]
fn test_edge_analysis_valid() {
    let e = EdgeAnalysis::new(1);
    assert!(e.is_valid());
    assert!((e.max_gap()).abs() < 1e-12);
    assert!(e.has_3d_curve);
    assert!(!e.is_degenerate);
}

#[test]
fn test_wire_analysis_gaps() {
    let mut w = WireAnalysis::new(0, 3);
    w.add_gap(WireGap::new(0, 1, 0.002, [0.5, 0.0, 0.0]));
    assert_eq!(w.nb_gaps(), 1);
    assert!((w.max_gap() - 0.002).abs() < 1e-12);
    assert!(w.is_valid(0.01));
    assert!(!w.is_valid(0.001));
    w.add_self_intersection([0.0, 0.0, 0.0]);
    assert_eq!(w.nb_self_intersections(), 1);
    assert!(!w.is_valid(1.0));
}

#[test]
fn test_surface_analysis_domain() {
    let s = SurfaceAnalysis::new(0, (0.0, 2.0), (-1.0, 1.0));
    assert!((s.u_extent() - 2.0).abs() < 1e-12);
    assert!((s.v_extent() - 2.0).abs() < 1e-12);
    assert!(s.is_in_domain(1.0, 0.0));
    assert!(!s.is_in_domain(3.0, 0.0));
    assert!(s.is_planar());
    let uv = s.project_point([0.0, 0.0, 0.0]);
    assert!((uv[0] - 1.0).abs() < 1e-12); // midpoint of (0,2)
    assert!((uv[1]).abs() < 1e-12);        // midpoint of (-1,1)
}

#[test]
fn test_small_face_check() {
    let mut c = SmallFaceCheck::new(1e-6);
    assert!(c.is_ok());
    c.add_small(10);
    c.add_spot(20);
    c.add_strip(30);
    assert_eq!(c.nb_small(), 1);
    assert_eq!(c.nb_spot(), 1);
    assert_eq!(c.nb_strip(), 1);
    assert_eq!(c.nb_total_issues(), 3);
    assert!(!c.is_ok());
}

#[test]
fn test_shape_contents() {
    let mut sc = ShapeContents::new();
    assert!(sc.is_empty());
    sc.nb_faces = 6;
    sc.nb_edges = 12;
    sc.nb_vertices = 8;
    assert_eq!(sc.total_shapes(), 26);
    assert!(!sc.is_empty());
    assert!(!sc.has_free_entities());
    sc.nb_free_edges = 2;
    assert!(sc.has_free_entities());
}
