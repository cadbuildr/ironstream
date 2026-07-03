// FILE: tests/occt_select3d.rs
extern crate ironstream;

use ironstream::select3d::{
    Select3dSensitiveBox, Select3dSensitiveEntity, Select3dSensitivePoint,
    Select3dSensitiveSphere, Select3dTypeOfSensitivity, SelectMgrSelectionManager,
};

#[test]
fn sensitive_point_new_stores_coords() {
    let p = Select3dSensitivePoint::new(7.0, -3.0, 0.5);
    assert_eq!(p.point, [7.0, -3.0, 0.5]);
}

#[test]
fn sensitive_point_sensitivity_type_is_interior() {
    let p = Select3dSensitivePoint::new(0.0, 0.0, 0.0);
    assert_eq!(p.sensitivity_type(), Select3dTypeOfSensitivity::Interior);
}

#[test]
fn sensitive_point_nb_sub_elements_is_one() {
    let p = Select3dSensitivePoint::new(1.0, 2.0, 3.0);
    assert_eq!(p.nb_sub_elements(), 1);
}

#[test]
fn sensitive_box_contains_interior_point() {
    let b = Select3dSensitiveBox::new([-5.0, -5.0, -5.0], [5.0, 5.0, 5.0]);
    assert!(b.contains_point([0.0, 0.0, 0.0]));
    assert!(b.contains_point([2.5, -2.5, 1.0]));
}

#[test]
fn sensitive_box_rejects_exterior_point() {
    let b = Select3dSensitiveBox::new([0.0, 0.0, 0.0], [1.0, 1.0, 1.0]);
    assert!(!b.contains_point([2.0, 0.5, 0.5]));
    assert!(!b.contains_point([0.5, 0.5, -0.1]));
}

#[test]
fn sensitive_box_sensitivity_type_is_any() {
    let b = Select3dSensitiveBox::new([0.0, 0.0, 0.0], [1.0, 1.0, 1.0]);
    assert_eq!(b.sensitivity_type(), Select3dTypeOfSensitivity::Any);
}

#[test]
fn sensitive_box_nb_sub_elements_is_six() {
    let b = Select3dSensitiveBox::new([0.0, 0.0, 0.0], [1.0, 1.0, 1.0]);
    assert_eq!(b.nb_sub_elements(), 6);
}

#[test]
fn sensitive_sphere_contains_point_inside_radius() {
    let s = Select3dSensitiveSphere::new([1.0, 1.0, 1.0], 3.0);
    assert!(s.contains_point([1.0, 1.0, 1.0]));
    assert!(s.contains_point([3.0, 1.0, 1.0]));
}

#[test]
fn sensitive_sphere_rejects_point_outside_radius() {
    let s = Select3dSensitiveSphere::new([0.0, 0.0, 0.0], 1.0);
    assert!(!s.contains_point([1.0, 1.0, 0.0]));
}

#[test]
fn sensitive_sphere_sensitivity_type_is_boundary() {
    let s = Select3dSensitiveSphere::new([0.0, 0.0, 0.0], 1.0);
    assert_eq!(s.sensitivity_type(), Select3dTypeOfSensitivity::Boundary);
}

#[test]
fn selection_manager_starts_empty() {
    let mgr = SelectMgrSelectionManager::new();
    assert_eq!(mgr.count(), 0);
}

#[test]
fn selection_manager_add_mixed_entities() {
    let mut mgr = SelectMgrSelectionManager::new();
    mgr.add(Box::new(Select3dSensitivePoint::new(0.0, 0.0, 0.0)));
    mgr.add(Box::new(Select3dSensitiveBox::new(
        [0.0, 0.0, 0.0],
        [1.0, 1.0, 1.0],
    )));
    mgr.add(Box::new(Select3dSensitiveSphere::new([5.0, 5.0, 5.0], 2.0)));
    assert_eq!(mgr.count(), 3);
}

#[test]
fn type_of_sensitivity_variants_are_distinct() {
    assert_ne!(
        Select3dTypeOfSensitivity::Interior,
        Select3dTypeOfSensitivity::Boundary
    );
    assert_ne!(
        Select3dTypeOfSensitivity::Interior,
        Select3dTypeOfSensitivity::Any
    );
    assert_ne!(
        Select3dTypeOfSensitivity::Boundary,
        Select3dTypeOfSensitivity::Any
    );
}
