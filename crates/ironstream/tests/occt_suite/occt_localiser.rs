extern crate ironstream;
use ironstream::localiser::*;

#[test]
fn local_state_unknown_is_known_returns_false() {
    let state = LocalState::Unknown;
    assert!(!state.is_known());
    // Known states return true
    assert!(LocalState::In.is_known());
    assert!(LocalState::Out.is_known());
    assert!(LocalState::On.is_known());
    assert!(LocalState::OnBoundary.is_known());
}

#[test]
fn localize_point_is_in_s1_is_in_s2_when_state_is_in() {
    let mut pt = LocalizePoint::new(0, [1.0, 1.0, 0.0]);
    pt.state_s1 = LocalState::In;
    pt.state_s2 = LocalState::In;
    assert!(pt.is_in_s1());
    assert!(pt.is_in_s2());
    // On is not In
    pt.state_s1 = LocalState::On;
    assert!(!pt.is_in_s1());
    assert!(pt.is_on_s1());
}

#[test]
fn edges_filler_add_point_with_full_localize_point() {
    let mut ef = EdgesFiller::new(1, 2);
    let mut pt = LocalizePoint::new(0, [0.5, 0.0, 0.0]);
    pt.set_on_edge_s1(7, 0.5);
    pt.set_on_edge_s2(8, 0.3);
    ef.add_point(pt);
    assert_eq!(ef.nb_points(), 1);
    assert_eq!(ef.points_on_a(), 1);
    assert_eq!(ef.points_on_b(), 1);
    let p = ef.point(0).unwrap();
    assert_eq!(p.edge_id_s1, Some(7));
    assert_eq!(p.edge_id_s2, Some(8));
}

#[test]
fn localisation_data_set_vertex_state_and_query() {
    let mut data = LocalisationData::new();
    data.set_vertex_state(5, LocalState::In);
    data.set_vertex_state(6, LocalState::Out);
    data.set_vertex_state(7, LocalState::On);
    assert_eq!(data.vertex_state(5), LocalState::In);
    assert_eq!(data.vertex_state(6), LocalState::Out);
    assert_eq!(data.vertex_state(7), LocalState::On);
    // Unknown vertex returns default Unknown
    assert_eq!(data.vertex_state(99), LocalState::Unknown);
}

#[test]
fn localisation_data_nb_edges_on_counts_correctly() {
    let mut data = LocalisationData::new();
    data.set_edge_state(1, LocalState::On);
    data.set_edge_state(2, LocalState::OnBoundary);
    data.set_edge_state(3, LocalState::In);
    data.set_edge_state(4, LocalState::Out);
    data.set_edge_state(5, LocalState::On);
    // On + OnBoundary + On = 3
    assert_eq!(data.nb_edges_on(), 3);
    assert_eq!(data.nb_edges_in(), 1);
}

#[test]
fn localizer_state_of_vertex() {
    let mut loc = Localizer::new(42);
    loc.classify_vertex(10, LocalState::In);
    loc.classify_vertex(11, LocalState::Out);
    loc.classify_vertex(12, LocalState::On);
    assert_eq!(loc.state_of_vertex(10), LocalState::In);
    assert_eq!(loc.state_of_vertex(11), LocalState::Out);
    assert_eq!(loc.state_of_vertex(12), LocalState::On);
    // Unclassified vertex
    assert_eq!(loc.state_of_vertex(99), LocalState::Unknown);
}
