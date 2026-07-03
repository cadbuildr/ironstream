use ironstream::topo_vertex::*;

#[test]
fn vertex_new_not_null_point_tolerance() {
    let v = TopoVertex::new(1, [1.0, 2.0, 3.0], 1e-7);
    assert!(!v.is_null());
    assert_eq!(v.point(), [1.0, 2.0, 3.0]);
    assert!((v.tolerance() - 1e-7).abs() < 1e-14);
}

#[test]
fn vertex_default_is_null() {
    let v = TopoVertex::default();
    assert!(v.is_null());
}

#[test]
fn edge_new_not_null_range_set_degenerated() {
    let mut e = TopoEdge::new(1, 10, 0.0, 1.0);
    assert!(!e.is_null());
    assert!(!e.is_degenerated());
    e.set_range(0.0, 2.0);
    assert!((e.last_param() - 2.0).abs() < 1e-10);
    e.set_degenerated(true);
    assert!(e.is_degenerated());
}

#[test]
fn face_add_wire() {
    let mut f = TopoFace::new(1, 100);
    assert!(!f.is_null());
    assert_eq!(f.nb_wires, 0);
    f.add_wire();
    f.add_wire();
    assert_eq!(f.nb_wires, 2);
}

#[test]
fn wire_closed() {
    let mut w = TopoWire::new(1);
    assert!(!w.is_null());
    w.add_edge(10);
    w.add_edge(20);
    w.set_closed(true);
    assert_eq!(w.nb_edges(), 2);
    assert!(w.is_closed());
}
