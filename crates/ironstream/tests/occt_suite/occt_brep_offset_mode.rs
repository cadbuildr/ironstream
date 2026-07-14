use ironstream::brep_offset_mode::*;

#[test]
fn offset_good_is_done_status_and_offset_value() {
    let o = BRepMakeOffset::new(1, 2.0);
    assert!(o.is_done());
    assert_eq!(o.status(), BRepOffsetStatus::Good);
    assert!((o.offset() - 2.0).abs() < 1e-10);
}

#[test]
fn offset_invalid_source_not_done() {
    let o = BRepMakeOffset::new(0, 1.0);
    assert!(!o.is_done());
    assert_eq!(o.status(), BRepOffsetStatus::NotDone);
}

#[test]
fn offset_mode_and_join_type_setters() {
    let mut o = BRepMakeOffset::new(1, 1.0);
    o.set_mode(BRepOffsetMode::Pipe);
    o.set_join_type(BRepOffsetJoinType::Intersection);
    assert_eq!(o.mode(), BRepOffsetMode::Pipe);
    assert_eq!(o.join_type(), BRepOffsetJoinType::Intersection);
}

#[test]
fn thru_sections_build_success_two_wires() {
    let mut b = BRepThruSections::new(1, true, false);
    b.add_wire(10);
    b.add_wire(20);
    b.build();
    assert!(b.is_done());
    assert!(b.result_shape_id() > 0);
    assert_eq!(b.nb_wires(), 2);
}

#[test]
fn thru_sections_need_two_wires_fails_with_one() {
    let mut b = BRepThruSections::new(1, false, false);
    b.add_wire(10);
    b.build();
    assert!(!b.is_done());
}
