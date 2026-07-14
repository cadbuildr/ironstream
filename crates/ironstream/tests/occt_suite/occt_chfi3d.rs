// FILE: tests/occt_chfi3d.rs
extern crate ironstream;

use ironstream::chfi3d::{ChFi3dBuilder, ChFi3dError};

#[test]
fn new_builder_is_not_done() {
    let b = ChFi3dBuilder::new(1e-7);
    assert!(!b.is_done());
}

#[test]
fn new_builder_has_zero_contours() {
    let b = ChFi3dBuilder::new(1e-7);
    assert_eq!(b.nb_contours(), 0);
}

#[test]
fn tolerance_round_trips() {
    let b = ChFi3dBuilder::new(0.005);
    assert!((b.tolerance() - 0.005).abs() < 1e-15);
}

#[test]
fn add_fillet_success() {
    let mut b = ChFi3dBuilder::new(1e-7);
    assert!(b.add_fillet(1, 2.5).is_ok());
    assert_eq!(b.nb_contours(), 1);
}

#[test]
fn add_fillet_negative_radius_rejected() {
    let mut b = ChFi3dBuilder::new(1e-7);
    assert_eq!(b.add_fillet(1, -0.5), Err(ChFi3dError::NegativeRadius));
    assert_eq!(b.nb_contours(), 0);
}

#[test]
fn add_chamfer_success() {
    let mut b = ChFi3dBuilder::new(1e-7);
    assert!(b.add_chamfer(2, 1.0, 1.5).is_ok());
    assert_eq!(b.nb_contours(), 1);
}

#[test]
fn add_chamfer_negative_dist1_rejected() {
    let mut b = ChFi3dBuilder::new(1e-7);
    assert_eq!(b.add_chamfer(3, -1.0, 1.0), Err(ChFi3dError::NegativeRadius));
}

#[test]
fn add_chamfer_negative_dist2_rejected() {
    let mut b = ChFi3dBuilder::new(1e-7);
    assert_eq!(b.add_chamfer(3, 1.0, -1.0), Err(ChFi3dError::NegativeRadius));
}

#[test]
fn build_done_after_adding_entries() {
    let mut b = ChFi3dBuilder::new(1e-7);
    b.add_fillet(10, 3.0).unwrap();
    b.add_chamfer(11, 1.0, 2.0).unwrap();
    b.build();
    assert!(b.is_done());
    assert_eq!(b.nb_contours(), 2);
}

#[test]
fn build_not_done_when_empty() {
    let mut b = ChFi3dBuilder::new(1e-7);
    b.build();
    assert!(!b.is_done());
}

#[test]
fn fillet_radius_returns_correct_value() {
    let mut b = ChFi3dBuilder::new(1e-7);
    b.add_fillet(5, 7.25).unwrap();
    assert_eq!(b.fillet_radius(5), Some(7.25));
}

#[test]
fn fillet_radius_missing_edge_returns_none() {
    let b = ChFi3dBuilder::new(1e-7);
    assert_eq!(b.fillet_radius(42), None);
}

#[test]
fn remove_fillet_reduces_contour_count() {
    let mut b = ChFi3dBuilder::new(1e-7);
    b.add_fillet(1, 1.0).unwrap();
    b.add_fillet(2, 2.0).unwrap();
    b.remove_contour(1);
    assert_eq!(b.nb_contours(), 1);
    assert_eq!(b.fillet_radius(1), None);
    assert_eq!(b.fillet_radius(2), Some(2.0));
}

#[test]
fn remove_chamfer_reduces_contour_count() {
    let mut b = ChFi3dBuilder::new(1e-7);
    b.add_chamfer(9, 0.5, 0.5).unwrap();
    b.remove_contour(9);
    assert_eq!(b.nb_contours(), 0);
}
