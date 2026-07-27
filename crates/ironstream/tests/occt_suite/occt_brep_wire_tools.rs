extern crate ironstream;
use ironstream::brep_wire_tools::*;

fn triangle_wire() -> Vec<WireEdge> {
    vec![
        WireEdge::new(0, [0.0,0.0,0.0], [1.0,0.0,0.0]),
        WireEdge::new(1, [1.0,0.0,0.0], [0.5,1.0,0.0]),
        WireEdge::new(2, [0.5,1.0,0.0], [0.0,0.0,0.0]),
    ]
}

#[test]
fn wire_explorer_advance() {
    let mut exp = WireExplorer::new(triangle_wire());
    assert_eq!(exp.nb_edges(), 3);
    assert!(exp.more());
    exp.next();
    exp.next();
    exp.next();
    assert!(!exp.more());
    exp.reset();
    assert!(exp.more());
}

#[test]
fn wire_check_closed_triangle() {
    let c = WireCheck::new(triangle_wire());
    assert!(c.is_valid());
    assert_eq!(c.nb_errors(), 0);
}

#[test]
fn wire_check_open_wire() {
    let edges = vec![
        WireEdge::new(0, [0.0,0.0,0.0], [1.0,0.0,0.0]),
        WireEdge::new(1, [1.0,0.0,0.0], [2.0,0.0,0.0]),
    ];
    let c = WireCheck::new(edges);
    assert!(!c.is_valid());
}

#[test]
fn wire_analysis_perimeter() {
    let a = WireAnalysis::new(triangle_wire());
    assert!(a.is_closed());
    assert!(a.check_order());
    assert!(a.perimeter() > 0.0);
}

#[test]
fn wire_edge_reversed() {
    let e = WireEdge::new(5, [1.0,2.0,3.0], [4.0,5.0,6.0]);
    let r = e.reversed();
    assert_eq!(r.start, [4.0,5.0,6.0]);
    assert_eq!(r.end, [1.0,2.0,3.0]);
    assert!(!r.is_forward);
    assert_eq!(r.id, 5);
}
