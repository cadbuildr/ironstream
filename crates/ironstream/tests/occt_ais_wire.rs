use ironstream::ais_wire::*;

#[test]
fn ais_wire_new_has_no_edges() {
    let w = AisWire::new(42);
    assert_eq!(w.wire_id, 42);
    assert_eq!(w.nb_edges(), 0);
    assert!(!w.is_closed());
}

#[test]
fn ais_wire_add_edge_dedup() {
    let mut w = AisWire::new(1);
    w.add_edge(10);
    w.add_edge(11);
    w.add_edge(12);
    w.add_edge(10); // duplicate
    assert_eq!(w.nb_edges(), 3);
    assert!(w.is_closed());
}

#[test]
fn ais_wire_not_closed_with_two_edges() {
    let mut w = AisWire::new(2);
    w.add_edge(5);
    w.add_edge(6);
    assert_eq!(w.nb_edges(), 2);
    assert!(!w.is_closed());
}

#[test]
fn ais_face_transparency_clamp() {
    let mut f = AisFace::new(1, 10);
    f.set_transparency(1.5);
    assert!((f.transparency - 1.0).abs() < 1e-6);
    f.set_transparency(-0.3);
    assert!(f.transparency.abs() < 1e-6);
}

#[test]
fn ais_plane_constructors_and_project() {
    let xoy = AisPlane::xoy();
    assert!((xoy.normal[2] - 1.0).abs() < 1e-14);
    let xoz = AisPlane::xoz();
    assert!((xoz.normal[1] - 1.0).abs() < 1e-14);
    let yoz = AisPlane::yoz();
    assert!((yoz.normal[0] - 1.0).abs() < 1e-14);

    // project [3,4,5] onto XOY plane → z drops to 0
    let proj = xoy.project_point([3.0, 4.0, 5.0]);
    assert!((proj[0] - 3.0).abs() < 1e-10);
    assert!((proj[1] - 4.0).abs() < 1e-10);
    assert!(proj[2].abs() < 1e-10);
}

#[test]
fn ais_plane_trihedron_and_edge_relation() {
    let mut t = AisPlaneTrihedron::new(1, 50.0);
    t.set_x_label("U");
    t.set_y_label("V");
    assert_eq!(t.x_label, "U");
    assert_eq!(t.y_label, "V");
    assert!((t.size - 50.0).abs() < 1e-14);

    let mut r = AisEdgeRelation::new(1, 2, AisEdgeRelationKind::Parallel);
    r.set_value(3.14);
    r.display();
    assert!(r.is_displayed);
    assert_eq!(r.kind, AisEdgeRelationKind::Parallel);
    r.erase();
    assert!(!r.is_displayed);
}
