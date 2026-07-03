// FILE: tests/occt_brep_fillet.rs
extern crate ironstream;
use ironstream::brep_fillet::*;

#[test]
fn fillet_build_with_edges_is_done() {
    let mut f = BRepFilletApiMakeFillet::new();
    f.add(1, 0.5).unwrap();
    f.add(2, 1.0).unwrap();
    f.build();
    assert!(f.is_done());
    assert_eq!(f.nb_edges(), 2);
}

#[test]
fn fillet_build_no_edges_not_done() {
    let mut f = BRepFilletApiMakeFillet::new();
    f.build();
    assert!(!f.is_done());
}

#[test]
fn fillet_negative_radius_returns_error() {
    let mut f = BRepFilletApiMakeFillet::new();
    assert_eq!(f.add(5, -0.1), Err(FilletError::NegativeRadius));
}

#[test]
fn fillet_zero_radius_is_valid() {
    let mut f = BRepFilletApiMakeFillet::new();
    assert!(f.add(3, 0.0).is_ok());
    f.build();
    assert!(f.is_done());
}

#[test]
fn fillet_radius_lookup_by_edge_id() {
    let mut f = BRepFilletApiMakeFillet::new();
    f.add(10, 3.14).unwrap();
    f.add(20, 6.28).unwrap();
    assert!((f.radius(10).unwrap() - 3.14).abs() < 1e-15);
    assert!((f.radius(20).unwrap() - 6.28).abs() < 1e-15);
    assert!(f.radius(99).is_none());
}

#[test]
fn fillet_nb_faulted_contours_default_zero() {
    let f = BRepFilletApiMakeFillet::new();
    assert_eq!(f.nb_faulted_contours(), 0);
}

#[test]
fn chamfer_build_with_edges_is_done() {
    let mut c = BRepFilletApiMakeChamfer::new();
    c.add(1, 0.25).unwrap();
    c.add(2, 0.75).unwrap();
    c.build();
    assert!(c.is_done());
    assert_eq!(c.nb_edges(), 2);
}

#[test]
fn chamfer_build_no_edges_not_done() {
    let mut c = BRepFilletApiMakeChamfer::new();
    c.build();
    assert!(!c.is_done());
}

#[test]
fn chamfer_zero_distance_returns_error() {
    let mut c = BRepFilletApiMakeChamfer::new();
    assert_eq!(c.add(1, 0.0), Err(FilletError::ZeroDistance));
}

#[test]
fn chamfer_negative_distance_returns_error() {
    let mut c = BRepFilletApiMakeChamfer::new();
    assert_eq!(c.add(1, -2.0), Err(FilletError::ZeroDistance));
}

#[test]
fn chamfer_multiple_edges_counted() {
    let mut c = BRepFilletApiMakeChamfer::new();
    for i in 0..5 {
        c.add(i, 1.0 + i as f64).unwrap();
    }
    assert_eq!(c.nb_edges(), 5);
}

#[test]
fn fillet_error_variants_are_distinct() {
    assert_ne!(FilletError::NegativeRadius, FilletError::ZeroDistance);
    assert_ne!(FilletError::NotDone, FilletError::FilletFailed);
    assert_ne!(FilletError::BadInput, FilletError::NotDone);
}
