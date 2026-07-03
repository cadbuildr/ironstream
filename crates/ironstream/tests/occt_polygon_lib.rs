// Integration tests for polygon_lib
use ironstream::polygon_lib::*;

#[test]
fn polygon3d_node_access_one_based() {
    let p = PolyPolygon3d::new(
        vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0, 0.0]],
        1e-4,
    );
    assert_eq!(p.nb_nodes(), 3);
    assert_eq!(p.node(1), Some([0.0, 0.0, 0.0]));
    assert_eq!(p.node(3), Some([1.0, 1.0, 0.0]));
    assert_eq!(p.node(0), None); // 1-based: 0 is invalid
    assert_eq!(p.node(4), None);
}

#[test]
fn polygon3d_arc_length() {
    // Three collinear points along X: total length = 2.0
    let p = PolyPolygon3d::new(
        vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [2.0, 0.0, 0.0]],
        0.0,
    );
    assert!((p.length() - 2.0).abs() < 1e-10);

    // Single node polygon has zero length
    let p1 = PolyPolygon3d::new(vec![[0.0, 0.0, 0.0]], 0.0);
    assert!((p1.length() - 0.0).abs() < 1e-10);
}

#[test]
fn polygon3d_with_parameters() {
    let p = PolyPolygon3d::new(
        vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [2.0, 0.0, 0.0]],
        0.0,
    )
    .with_parameters(vec![0.0, 0.5, 1.0]);

    assert!(p.has_parameters());
    assert_eq!(p.parameter(1), Some(0.0));
    assert_eq!(p.parameter(2), Some(0.5));
    assert_eq!(p.parameter(3), Some(1.0));
    assert_eq!(p.parameter(0), None); // 1-based
}

#[test]
fn polygon2d_node_access_one_based() {
    let p = PolyPolygon2d::new(
        vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0]],
        1e-4,
    );
    assert_eq!(p.nb_nodes(), 3);
    assert_eq!(p.node(1), Some([0.0, 0.0]));
    assert_eq!(p.node(2), Some([1.0, 0.0]));
    assert_eq!(p.node(0), None); // 1-based: 0 is invalid
}

#[test]
fn polygon_on_triangulation_indices_and_parameters() {
    let p = PolyPolygonOnTriangulation::new(vec![10, 20, 30, 40], 1e-4)
        .with_parameters(vec![0.0, 1.0, 2.0, 3.0]);

    assert_eq!(p.nb_nodes(), 4);
    assert_eq!(p.node(1), Some(10));
    assert_eq!(p.node(4), Some(40));
    assert_eq!(p.node(0), None); // 1-based: 0 is invalid
    assert!(p.has_parameters());
    assert_eq!(p.parameter(1), Some(0.0));
    assert_eq!(p.parameter(3), Some(2.0));
    assert_eq!(p.parameter(0), None);

    // Without parameters
    let p2 = PolyPolygonOnTriangulation::new(vec![1, 2, 3], 0.0);
    assert!(!p2.has_parameters());
    assert_eq!(p2.parameter(1), None);
}

#[test]
fn poly_make_loops_edges_and_perform() {
    let mut ml = PolyMakeLoops::new();
    assert_eq!(ml.nb_loops(), 0);
    assert!(!ml.is_done());

    ml.add_edge(10);
    ml.add_edge(20);
    ml.add_edge(30);
    ml.perform();

    assert!(ml.is_done());
    assert_eq!(ml.nb_loops(), 1);
    assert_eq!(ml.loop_edges(0), &[10u32, 20, 30]);

    // Empty perform: no loops formed
    let mut ml2 = PolyMakeLoops::new();
    ml2.perform();
    assert!(!ml2.is_done());
    assert_eq!(ml2.nb_loops(), 0);
}
