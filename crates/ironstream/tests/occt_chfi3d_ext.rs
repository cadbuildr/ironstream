// FILE: tests/occt_chfi3d_ext.rs
extern crate ironstream;
use ironstream::chfi3d_ext::*;

#[test]
fn spine_edge_length() {
    let e = SpineEdge::new(1, 1.0, 4.5);
    assert!((e.length() - 3.5).abs() < 1e-10);
}

#[test]
fn chfi_spine_total_length_and_done() {
    let mut spine = ChFiSpine::new(ChFiMode::Fillet);
    spine.add_edge(SpineEdge::new(10, 0.0, 3.0));
    spine.add_edge(SpineEdge::new(11, 3.0, 7.0));
    assert_eq!(spine.nb_edges(), 2);
    assert!(spine.is_done());
    assert!((spine.total_length() - 7.0).abs() < 1e-10);
}

#[test]
fn fil_spine_constant_radius() {
    let s = FilSpine::new(5.0);
    assert!(s.is_constant());
    assert!((s.radius_at(0.0) - 5.0).abs() < 1e-10);
    assert!((s.radius_at(1.0) - 5.0).abs() < 1e-10);
}

#[test]
fn fil_spine_variable_radius_interpolation() {
    let mut s = FilSpine::new(2.0);
    s.set_radius(1.0, 4.0);
    assert!(!s.is_constant());
    assert!((s.radius_at(0.5) - 3.0).abs() < 1e-8);
}

#[test]
fn chamf_spine_equal_distances() {
    let s = ChamfSpine::two_distance(3.0, 3.0);
    assert!(s.is_equal());
    assert!((s.distance1 - 3.0).abs() < 1e-10);
}

#[test]
fn chamf_spine_distance_and_angle() {
    let s = ChamfSpine::distance_and_angle(2.0, 0.5);
    assert!(s.angle.is_some());
    assert!(!s.is_equal());
}

#[test]
fn surf_data_param_span() {
    let sd = ChFiSurfData::new(0, 0.0, 5.5, ChFiSurfKind::Torus);
    assert!((sd.param_span() - 5.5).abs() < 1e-10);
    assert_eq!(sd.surface_kind, ChFiSurfKind::Torus);
}
