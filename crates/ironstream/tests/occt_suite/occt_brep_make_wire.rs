extern crate ironstream;
use ironstream::brep_make_wire::*;

#[test]
fn test_make_wire_connected_edges() {
    let mut w = MakeWire::new();
    w.add(WireEdgeDef::new(0, [0.0, 0.0, 0.0], [1.0, 0.0, 0.0]));
    w.add(WireEdgeDef::new(1, [1.0, 0.0, 0.0], [1.0, 1.0, 0.0]));
    w.add(WireEdgeDef::new(2, [1.0, 1.0, 0.0], [0.0, 1.0, 0.0]));
    assert!(w.is_done());
    assert_eq!(w.nb_edges(), 3);
    assert!((w.total_length() - 3.0).abs() < 1e-10);
}

#[test]
fn test_make_wire_disconnected_fails() {
    let mut w = MakeWire::new();
    w.add(WireEdgeDef::new(0, [0.0, 0.0, 0.0], [1.0, 0.0, 0.0]));
    w.add(WireEdgeDef::new(1, [5.0, 0.0, 0.0], [6.0, 0.0, 0.0]));
    assert!(!w.is_done());
}

#[test]
fn test_make_wire_closed() {
    let mut w = MakeWire::new();
    w.add(WireEdgeDef::new(0, [0.0, 0.0, 0.0], [1.0, 0.0, 0.0]));
    w.add(WireEdgeDef::new(1, [1.0, 0.0, 0.0], [0.5, 1.0, 0.0]));
    w.add(WireEdgeDef::new(2, [0.5, 1.0, 0.0], [0.0, 0.0, 0.0]));
    assert!(w.is_closed);
}

#[test]
fn test_wire_bounding_box() {
    let mut w = MakeWire::new();
    w.add(WireEdgeDef::new(0, [0.0, 0.0, 0.0], [3.0, 0.0, 0.0]));
    w.add(WireEdgeDef::new(1, [3.0, 0.0, 0.0], [3.0, 4.0, 0.0]));
    let (mn, mx) = w.bounding_box().unwrap();
    assert!((mn[0]).abs() < 1e-10);
    assert!((mx[0] - 3.0).abs() < 1e-10);
    assert!((mx[1] - 4.0).abs() < 1e-10);
}

#[test]
fn test_wire_gap_analysis() {
    let edges = vec![
        WireEdgeDef::new(0, [0.0, 0.0, 0.0], [1.0, 0.0, 0.0]),
        WireEdgeDef::new(1, [2.0, 0.0, 0.0], [3.0, 0.0, 0.0]),
    ];
    assert!(WireGapAnalysis::has_gaps(&edges, 1e-6));
    assert_eq!(WireGapAnalysis::nb_gaps(&edges, 1e-6), 1);
    assert!((WireGapAnalysis::max_gap(&edges) - 1.0).abs() < 1e-10);
}
