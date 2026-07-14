extern crate ironstream;
use ironstream::geom_builder::*;

#[test]
fn test_builder_status_predicates() {
    assert!(BuilderStatus::Done.is_done());
    assert!(!BuilderStatus::NotDone.is_done());
    assert!(BuilderStatus::NullCurve.is_error());
    assert!(BuilderStatus::NullSurface.is_error());
    assert!(!BuilderStatus::Done.is_error());
    assert!(!BuilderStatus::NotDone.is_error());
}

#[test]
fn test_make_edge_from_curve() {
    let e = MakeEdge::from_curve(7, 0.0, 2.0);
    assert!(e.is_done());
    assert!(e.edge() > 0);
    assert!((e.first_parameter()).abs() < 1e-10);
    assert!((e.last_parameter() - 2.0).abs() < 1e-10);
}

#[test]
fn test_make_edge_null_curve() {
    let e = MakeEdge::from_curve(0, 0.0, 1.0);
    assert!(!e.is_done());
    assert_eq!(e.status(), BuilderStatus::NullCurve);
    assert_eq!(e.edge(), 0);
}

#[test]
fn test_make_edge_from_line() {
    let e = MakeEdge::from_line([0.0, 0.0, 0.0], [3.0, 4.0, 0.0]);
    assert!(e.is_done());
    // length = 5.0
    assert!((e.last_parameter() - 5.0).abs() < 1e-10);
    assert!(e.edge() > 0);

    let e_degenerate = MakeEdge::from_line([1.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
    assert!(!e_degenerate.is_done());
}

#[test]
fn test_make_wire_and_closure() {
    let mut w = MakeWire::new();
    assert!(!w.is_done());
    w.add_edge(100);
    assert!(w.is_done());
    assert_eq!(w.nb_edges(), 1);
    w.add_edge(101);
    assert_eq!(w.nb_edges(), 2);
    // two edges => closed after perform
    w.perform();
    assert!(w.is_closed());
    assert!(w.wire() > 0);
}

#[test]
fn test_make_face_and_make_shell() {
    // MakeFace from surface
    let f = MakeFace::from_surface(5, 0.0, 1.0, 0.0, 1.0, 1e-6);
    assert!(f.is_done());
    assert_eq!(f.nb_inner_wires(), 0);
    assert!(f.face() > 0);

    // add inner wire
    let mut f2 = MakeFace::from_surface(5, 0.0, 1.0, 0.0, 1.0, 1e-6);
    f2.add_inner_wire(99);
    assert_eq!(f2.nb_inner_wires(), 1);

    // MakeShell
    let s = MakeShell::new(10, 0.0, 1.0, 0.0, 1.0, false);
    assert!(s.is_done());
    assert_eq!(s.nb_faces(), 1);
    assert!(s.shell() > 0);

    let s_null = MakeShell::new(0, 0.0, 1.0, 0.0, 1.0, false);
    assert!(!s_null.is_done());
    assert_eq!(s_null.nb_faces(), 0);
}
