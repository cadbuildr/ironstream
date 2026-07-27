extern crate ironstream;
use ironstream::geom_offset_wire::*;

#[test]
fn occt_offset_wire_square() {
    let square = vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]];
    let result = offset_closed_wire(&square, 0.1);
    assert!(!result.is_empty());
}

#[test]
fn occt_wire_offset_builder() {
    let square = vec![[0.0, 0.0], [2.0, 0.0], [2.0, 2.0], [0.0, 2.0]];
    let mut wo = WireOffset::new(square, 0.5);
    assert!(wo.is_done());
}
