// Integration tests for contap_lib module
extern crate ironstream;

use ironstream::contap_lib::*;

#[test]
fn contap_status_default_is_not_done() {
    let s: ContapStatus = ContapStatus::default();
    assert_eq!(s, ContapStatus::NotDone);
    assert!(!s.is_done());
}

#[test]
fn contap_status_done_is_done() {
    let s = ContapStatus::Done;
    assert!(s.is_done());
}

#[test]
fn contap_point_new_stores_coordinates() {
    let pt = ContapPoint::new(1.0, 2.0, 3.0, 0.3, 0.7);
    assert_eq!(pt.value(), [1.0, 2.0, 3.0]);
    let (u, v) = pt.parameters_on_surface();
    assert!((u - 0.3).abs() < 1e-9);
    assert!((v - 0.7).abs() < 1e-9);
    assert!(!pt.is_tangent());
}

#[test]
fn contap_line_add_and_retrieve_vertices() {
    let mut line = ContapLine::new(ContapLineType::Walking);
    assert_eq!(line.nb_vertex(), 0);
    assert_eq!(line.kind(), ContapLineType::Walking);
    line.add_vertex(ContapPoint::new(0.0, 0.0, 0.0, 0.0, 0.0));
    line.add_vertex(ContapPoint::new(1.0, 0.0, 0.0, 1.0, 0.0));
    assert_eq!(line.nb_vertex(), 2);
    let v = line.vertex(0).unwrap();
    assert_eq!(v.value(), [0.0, 0.0, 0.0]);
    assert!(line.vertex(99).is_none());
}

#[test]
fn contap_fill_iterator_iteration() {
    let mut it = ContapContourFillIterator::new();
    it.add_line(ContapLine::new(ContapLineType::Lin));
    it.add_line(ContapLine::new(ContapLineType::Circle));
    it.add_line(ContapLine::new(ContapLineType::Restriction));
    assert_eq!(it.nb_lines(), 3);
    it.init();
    let mut count = 0;
    while it.more() {
        let line = it.value().unwrap();
        let _ = line.kind();
        it.next();
        count += 1;
    }
    assert_eq!(count, 3);
}

#[test]
fn contap_search_perform_valid_direction() {
    let mut s = ContapTheSearch::new();
    s.perform(1, [0.0, 0.0, 1.0], 10);
    assert!(s.is_done());
    assert_eq!(s.nb_points(), 0);
    assert_eq!(s.nb_tangent_points(), 0);
    assert!(s.is_empty());
    assert!(s.point(0).is_none());
}

#[test]
fn contap_contour_orthogonal_and_perspective() {
    let mut co = ContapContour::new_orthogonal(1, [0.0, 0.0, 1.0]);
    co.perform();
    assert!(co.is_done());
    assert_eq!(co.nb_lines(), 0);
    assert!(co.line(0).is_none());

    let mut cp = ContapContour::new_perspective(2, [0.0, 0.0, 10.0]);
    assert!(cp.is_perspective);
    cp.perform();
    assert!(cp.is_done());

    // null surface_id should error
    let mut ce = ContapContour::new_orthogonal(0, [0.0, 1.0, 0.0]);
    ce.perform();
    assert_eq!(ce.status, ContapStatus::Error);
    assert!(!ce.is_done());
}
