// FILE: tests/occt_hlr_algo.rs
extern crate ironstream;
use ironstream::hlr_algo::*;

#[test]
fn orthographic_projector_type() {
    let p = HlrAlgoProjector::orthographic();
    assert_eq!(p.projection_type(), HlrProjectionType::Orthographic);
}

#[test]
fn orthographic_focal_none() {
    let p = HlrAlgoProjector::orthographic();
    assert!(p.focal().is_none());
}

#[test]
fn perspective_projector_type() {
    let p = HlrAlgoProjector::perspective(5.0);
    assert_eq!(p.projection_type(), HlrProjectionType::Perspective);
}

#[test]
fn perspective_focal_some() {
    let p = HlrAlgoProjector::perspective(5.0);
    assert_eq!(p.focal(), Some(5.0));
}

#[test]
fn orthographic_project_ignores_z() {
    let p = HlrAlgoProjector::orthographic();
    let screen = p.project([7.0, -3.0, 100.0]);
    assert_eq!(screen, [7.0, -3.0]);
}

#[test]
fn perspective_project_divides_by_z() {
    let p = HlrAlgoProjector::perspective(4.0);
    // x/z*focal = 8/2*4 = 16, y/z*focal = 4/2*4 = 8
    let screen = p.project([8.0, 4.0, 2.0]);
    let eps = 1e-12;
    assert!((screen[0] - 16.0).abs() < eps, "x screen coord");
    assert!((screen[1] - 8.0).abs() < eps, "y screen coord");
}

#[test]
fn interference_list_add_count_get() {
    let mut list = HlrAlgoInterferenceList::new();
    assert_eq!(list.count(), 0);
    list.add(0.0, 0.3, HlrAlgoEdgeStatus::AllVisible);
    list.add(0.3, 0.7, HlrAlgoEdgeStatus::Partial);
    list.add(0.7, 1.0, HlrAlgoEdgeStatus::AllHidden);
    assert_eq!(list.count(), 3);
    let (s, e, st) = list.get(1).unwrap();
    assert_eq!(s, 0.3);
    assert_eq!(e, 0.7);
    assert_eq!(st, HlrAlgoEdgeStatus::Partial);
}

#[test]
fn interference_list_get_out_of_bounds() {
    let list = HlrAlgoInterferenceList::new();
    assert!(list.get(0).is_none());
}

#[test]
fn poly_algo_new_and_tolerance() {
    let algo = HlrAlgoPolyAlgo::new(HlrAlgoProjector::perspective(1.0));
    assert!((algo.tolerance() - 1e-7).abs() < 1e-15);
    assert_eq!(algo.interference_count(), 0);
}

#[test]
fn poly_algo_set_tolerance() {
    let mut algo = HlrAlgoPolyAlgo::new(HlrAlgoProjector::orthographic());
    algo.set_tolerance(1e-5);
    assert!((algo.tolerance() - 1e-5).abs() < 1e-15);
}

#[test]
fn poly_algo_update_noop() {
    let mut algo = HlrAlgoPolyAlgo::new(HlrAlgoProjector::orthographic());
    algo.update();
    assert_eq!(algo.interference_count(), 0);
}

#[test]
fn edge_status_variants_eq() {
    assert_eq!(HlrAlgoEdgeStatus::AllVisible, HlrAlgoEdgeStatus::AllVisible);
    assert_ne!(HlrAlgoEdgeStatus::AllVisible, HlrAlgoEdgeStatus::AllHidden);
    assert_ne!(HlrAlgoEdgeStatus::AllHidden, HlrAlgoEdgeStatus::Partial);
}

#[test]
fn projection_type_variants_eq() {
    assert_eq!(HlrProjectionType::Orthographic, HlrProjectionType::Orthographic);
    assert_ne!(HlrProjectionType::Orthographic, HlrProjectionType::Perspective);
}
