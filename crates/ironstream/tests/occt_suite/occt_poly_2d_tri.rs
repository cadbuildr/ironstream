use ironstream::poly_2d_tri::*;

#[test]
fn poly_triangle_indices_get_reversed() {
    let t = PolyTriangle::new(1, 2, 3);
    assert_eq!(t.indices(), [1, 2, 3]);
    assert_eq!(t.get(0), 1);
    assert_eq!(t.get(1), 2);
    assert_eq!(t.get(2), 3);
    let rev = t.reversed();
    assert_eq!(rev.n1, 3);
    assert_eq!(rev.n2, 2);
    assert_eq!(rev.n3, 1);
}

#[test]
fn poly_array1_of_triangle_set_value_get() {
    let mut a = PolyArray1OfTriangle::new(1, 4);
    assert_eq!(a.length(), 4);
    assert_eq!(a.upper(), 4);
    a.set_value(1, PolyTriangle::new(1, 2, 3));
    a.set_value(3, PolyTriangle::new(4, 5, 6));
    assert_eq!(a.value(1).unwrap().n1, 1);
    assert_eq!(a.value(3).unwrap().n2, 5);
    // out-of-range returns None
    assert!(a.value(10).is_none());
}

#[test]
fn poly2d_triangulation_add_node_triangle_accessors() {
    let mut mesh = Poly2DTriangulation::new();
    let a = mesh.add_node([0.0, 0.0]);
    let b = mesh.add_node([1.0, 0.0]);
    let c = mesh.add_node([0.0, 1.0]);
    assert_eq!(a, 1);
    assert_eq!(b, 2);
    assert_eq!(c, 3);
    mesh.add_triangle(PolyTriangle::new(a, b, c));
    assert_eq!(mesh.nb_nodes(), 3);
    assert_eq!(mesh.nb_triangles(), 1);
    assert_eq!(mesh.node(1), Some([0.0, 0.0]));
    assert_eq!(mesh.triangle(1).unwrap().n3, 3);
}

#[test]
fn poly2d_triangulation_is_valid_and_area() {
    let mut mesh = Poly2DTriangulation::new();
    mesh.add_node([0.0, 0.0]);
    mesh.add_node([1.0, 0.0]);
    mesh.add_node([0.0, 1.0]);
    mesh.add_triangle(PolyTriangle::new(1, 2, 3));
    assert!(mesh.is_valid());

    // area of a right triangle with legs 1,1 = 0.5
    assert!((mesh.total_area_uv() - 0.5).abs() < 1e-10);

    // bad index -> invalid
    mesh.add_triangle(PolyTriangle::new(1, 2, 99));
    assert!(!mesh.is_valid());
}

#[test]
fn poly_coherent_triangle_neighbours_boundary() {
    let mut ct = PolyCoherentTriangle::new(0, 1, 2);
    assert_eq!(ct.nb_neighbours(), 0);
    assert!(ct.is_boundary_edge(0));
    assert!(ct.is_boundary_edge(1));
    assert!(ct.is_boundary_edge(2));

    ct.set_neighbour(0, 5);
    assert_eq!(ct.nb_neighbours(), 1);
    assert!(!ct.is_boundary_edge(0));
    assert!(ct.is_boundary_edge(1));
}

#[test]
fn poly_make_loops_single_triangle() {
    let mut ml = PolyMakeLoops::new();
    // Triangle loop: 0->1->2->0
    ml.add_edge(0, 1);
    ml.add_edge(1, 2);
    ml.add_edge(2, 0);
    ml.perform();
    assert_eq!(ml.nb_loops(), 1);
    assert_eq!(ml.loops[0].len(), 3);
}
