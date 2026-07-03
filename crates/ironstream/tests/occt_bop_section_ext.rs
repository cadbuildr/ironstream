extern crate ironstream;
use ironstream::bop_section_ext::*;

#[test]
fn section_status_message_variants() {
    assert_eq!(SectionStatus::NotDone.message(), "Not performed");
    assert_eq!(SectionStatus::Done.message(), "Done");
    assert_eq!(SectionStatus::NoIntersection.message(), "No intersection");
    assert!(SectionStatus::Done.has_result());
    assert!(!SectionStatus::EmptyResult.has_result());
}

#[test]
fn section_edge_on_s1_s2_flags() {
    let mut e = SectionEdge::new(1, [0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
    assert!(e.on_s1);
    assert!(e.on_s2);
    e.on_s2 = false;
    assert!(!e.on_s2);
    assert!(e.on_s1);
}

#[test]
fn brep_algo_section_with_plane() {
    let mut s = BrepAlgoSection::new(1, 2)
        .with_plane([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
    // plane_origin is on BrepAlgoSection itself
    assert!(s.plane_origin.is_some());
    assert!(s.plane_normal.is_some());
    s.perform();
    assert!(s.is_done());
}

#[test]
fn brep_algo_section_edges_on_shapes() {
    let mut sec = BrepAlgoSection::new(1, 2);
    let mut e1 = SectionEdge::new(10, [0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
    e1.on_s1 = true; e1.on_s2 = false;
    let mut e2 = SectionEdge::new(11, [1.0, 0.0, 0.0], [2.0, 0.0, 0.0]);
    e2.on_s1 = false; e2.on_s2 = true;
    sec.inner.add_section_edge(e1);
    sec.inner.add_section_edge(e2);
    sec.perform();
    assert!(sec.has_result());
    assert_eq!(sec.edges_on_shape_a(), vec![10]);
    assert_eq!(sec.edges_on_shape_b(), vec![11]);
}

#[test]
fn bop_algo_section_fuzzy_pcurve_settings() {
    let mut sec = BopAlgoSection::new(3, 4);
    sec.set_fuzzy_value(1e-5);
    sec.set_compute_pcurve_on_s1(true);
    sec.set_compute_pcurve_on_s2(true);
    assert!((sec.fuzzy_value - 1e-5).abs() < 1e-15);
    assert!(sec.compute_pcurve1);
    assert!(sec.compute_pcurve2);
}

#[test]
fn brep_algo_section_legacy_with_edges() {
    let mut leg = BrepAlgoSectionLegacy::new(1, 2);
    leg.edges.push(SectionEdge::new(5, [0.0, 0.0, 0.0], [3.0, 0.0, 0.0]));
    leg.build();
    assert!(leg.is_done());
    assert_eq!(leg.nb_edges(), 1);
}
