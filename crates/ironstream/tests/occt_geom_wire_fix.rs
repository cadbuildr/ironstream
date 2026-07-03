extern crate ironstream;
use ironstream::geom_wire_fix::*;

#[test]
fn test_geom_wire_fix_basic() {
    let wire = vec!["edge1".to_string(), "edge2".to_string(), "edge3".to_string()];
    let mut analysis = WireAnalysis::new(wire.clone(), 1e-7);
    assert_eq!(analysis.nb_edges(), 3);

    let ok = analysis.perform();
    assert!(ok, "non-empty error-free wire should perform OK");
    assert_eq!(analysis.error_count(), 0);
}

#[test]
fn test_wire_fixer_basic() {
    let wire = vec!["edge_b".to_string(), "edge_a".to_string(), "edge_c".to_string()];
    let mut fixer = WireFixer::new(wire);
    let reordered = fixer.fix_reorder();
    // Should return whether order changed
    let _ = reordered;

    let result = fixer.wire();
    assert_eq!(result.len(), 3);
}

#[test]
fn test_wire_fixer_fix_gaps() {
    let wire = vec!["e1".to_string(), "e2".to_string()];
    let mut fixer = WireFixer::new(wire);
    let _ = fixer.fix_gaps();
    // wire length may have grown if gaps were inserted
    let _ = fixer.wire();
}

#[test]
fn test_wire_analysis_load() {
    let wire = vec!["a".to_string(), "b".to_string()];
    let mut analysis = WireAnalysis::new(vec![], 1e-7);
    analysis.load(wire);
    assert_eq!(analysis.nb_edges(), 2);
}

#[test]
fn test_wire_error_variants() {
    let e1 = WireError::NotConnected;
    let e2 = WireError::Gap;
    let e3 = WireError::SelfIntersecting;
    let e4 = WireError::BadEdge;
    let _ = (e1, e2, e3, e4);
}
