use ironstream::mesh_data::*;

fn make_tet() -> MeshDataSource {
    let mut m = MeshDataSource::new(1);
    m.add_node([0.0, 0.0, 0.0]);
    m.add_node([1.0, 0.0, 0.0]);
    m.add_node([0.5, 1.0, 0.0]);
    m.add_node([0.5, 0.5, 1.0]);
    m.add_triangle(PolyTriangle::new(1, 2, 3));
    m.add_triangle(PolyTriangle::new(1, 2, 4));
    m.add_triangle(PolyTriangle::new(2, 3, 4));
    m.add_triangle(PolyTriangle::new(1, 3, 4));
    m
}

#[test]
fn poly_triangle_get_set() {
    let mut t = PolyTriangle::new(1, 2, 3);
    assert_eq!(t.get(1), Some(1));
    assert_eq!(t.get(0), None);
    t.set(2, 20);
    assert_eq!(t.n2, 20);
}

#[test]
fn mesh_add_retrieve() {
    let m = make_tet();
    assert_eq!(m.nb_nodes(), 4);
    assert_eq!(m.nb_triangles(), 4);
    assert_eq!(m.node(1), Some([0.0, 0.0, 0.0]));
    assert!(m.node(0).is_none());
}

#[test]
fn mesh_bounding_box() {
    let m = make_tet();
    let (mn, mx) = m.bounding_box().unwrap();
    assert!(mn[0].abs() < 1e-10);
    assert!((mx[0] - 1.0).abs() < 1e-10);
}

#[test]
fn poly_connect_triangles_around_node() {
    let m = make_tet();
    let c = PolyConnect::new(m);
    assert_eq!(c.nb_triangles_around_node(1), 3);
    assert_eq!(c.nb_triangles_around_node(0), 0);
}

#[test]
fn mesh_quality_basic() {
    let m = make_tet();
    let q = m.quality();
    assert!(q.min_edge_length > 0.0);
    assert!(q.max_edge_length >= q.min_edge_length);
}
