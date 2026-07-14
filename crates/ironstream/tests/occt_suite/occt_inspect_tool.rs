extern crate ironstream;
use ironstream::inspect_tool::*;

#[test]
fn brep_tool_face_parameter_range_default_values() {
    let f = BrepToolFace::new(1, 10, SurfaceGeomType::Plane);
    let (u_range, v_range) = f.parameter_range();
    assert!((u_range[0] - 0.0).abs() < 1e-10);
    assert!((u_range[1] - 1.0).abs() < 1e-10);
    assert!((v_range[0] - 0.0).abs() < 1e-10);
    assert!((v_range[1] - 1.0).abs() < 1e-10);
}

#[test]
fn brep_tool_edge_range_tuple_values() {
    let mut e = BrepToolEdge::new(2, 20, CurveGeomType::Circle);
    e.first_param = 0.5;
    e.last_param = 3.0;
    let (first, last) = e.range();
    assert!((first - 0.5).abs() < 1e-10);
    assert!((last - 3.0).abs() < 1e-10);
}

#[test]
fn brep_tool_edge_curve_on_surface_is_none_by_default() {
    let e = BrepToolEdge::new(3, 30, CurveGeomType::Line);
    assert!(e.curve_on_surface().is_none());
    assert!(!e.has_curve_on_surface());
}

#[test]
fn brep_inspector_face_edge_vertex_lookup_by_id() {
    let mut insp = BrepInspector::new(1);
    insp.add_face(BrepToolFace::new(10, 100, SurfaceGeomType::Sphere));
    insp.add_edge(BrepToolEdge::new(20, 200, CurveGeomType::Circle));
    insp.add_vertex(BrepToolVertex::new(30, [1.0, 2.0, 3.0]));
    assert!(insp.face(10).is_some());
    assert_eq!(insp.face(10).unwrap().surface_id, 100);
    assert!(insp.edge(20).is_some());
    assert_eq!(insp.edge(20).unwrap().curve_id, 200);
    assert!(insp.vertex(30).is_some());
    assert_eq!(insp.vertex(30).unwrap().vertex_id, 30);
    // Non-existent ids return None
    assert!(insp.face(99).is_none());
    assert!(insp.edge(99).is_none());
    assert!(insp.vertex(99).is_none());
}

#[test]
fn brep_inspector_degenerated_edges_returns_only_degenerated() {
    let mut insp = BrepInspector::new(1);
    let mut e1 = BrepToolEdge::new(1, 10, CurveGeomType::Line);
    e1.is_degenerated = true;
    let e2 = BrepToolEdge::new(2, 20, CurveGeomType::Circle);
    let mut e3 = BrepToolEdge::new(3, 30, CurveGeomType::BSplineCurve);
    e3.is_degenerated = true;
    insp.add_edge(e1);
    insp.add_edge(e2);
    insp.add_edge(e3);
    let degenerated = insp.degenerated_edges();
    assert_eq!(degenerated.len(), 2);
    assert!(degenerated.contains(&1));
    assert!(degenerated.contains(&3));
    assert!(!degenerated.contains(&2));
}

#[test]
fn brep_inspector_euler_characteristic_sphere_topology() {
    // Sphere: 1 face, 0 edges, 0 vertices => V - E + F = 0 - 0 + 1 = 1
    let mut insp = BrepInspector::new(99);
    insp.add_face(BrepToolFace::new(1, 10, SurfaceGeomType::Sphere));
    assert_eq!(insp.vertex_count(), 0);
    assert_eq!(insp.edge_count(), 0);
    assert_eq!(insp.face_count(), 1);
    assert_eq!(insp.euler_characteristic(), 1);
}
