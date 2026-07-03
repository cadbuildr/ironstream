extern crate ironstream;
use ironstream::topo_wire_builder::*;

#[test]
fn test_topo_wire_builder_basic() {
    // WireBuilder
    let mut wb = WireBuilder::new();
    assert!(!wb.is_done());
    wb.add_edge("e1");
    wb.add_edge("e2");
    wb.add_edge("e3");
    assert!(wb.is_done());
    assert!(!wb.closed);
    wb.close();
    assert!(wb.closed);
    let w = wb.wire();
    assert_eq!(w.len(), 3);
    assert_eq!(w[0], "e1");

    // WireAnalysis: a closed wire has each edge appearing twice (or paired)
    // Use a wire where each edge appears twice (even count -> no free edges -> closed)
    let mut wa = WireAnalysis::new(vec![
        "e1".to_string(), "e2".to_string(), "e1".to_string(), "e2".to_string(),
    ]);
    let ok = wa.analyze();
    assert!(ok, "closed wire (each edge twice) should be closed");
    assert_eq!(wa.num_free_edges, 0);

    // An open wire: each edge appears once (free)
    let mut wa2 = WireAnalysis::new(vec!["e1".to_string(), "e2".to_string(), "e3".to_string()]);
    let closed = wa2.analyze();
    assert!(!closed, "open wire should not be closed");
    assert_eq!(wa2.num_free_edges, 3);

    // wire_from_edges
    let wb2 = wire_from_edges(&["a", "b", "c"]);
    assert_eq!(wb2.edges.len(), 3);
    assert_eq!(wb2.edges[0], "a");

    // connect_edges
    let edges = vec!["x".to_string(), "y".to_string(), "z".to_string()];
    let groups = connect_edges(&edges);
    assert!(!groups.is_empty());
}
