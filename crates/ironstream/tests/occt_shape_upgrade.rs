// FILE: tests/occt_shape_upgrade.rs
extern crate ironstream;
use ironstream::shape_upgrade::{
    ShapeUpgradeShapeDivide, ShapeUpgradeShapeTolerances, ShapeUpgradeStatus,
    ShapeUpgradeUnifySameDomain,
};

// --- ShapeUpgradeUnifySameDomain ---

#[test]
fn test_unify_new_not_done() {
    let u = ShapeUpgradeUnifySameDomain::new();
    assert!(!u.is_done());
}

#[test]
fn test_unify_build_marks_done() {
    let mut u = ShapeUpgradeUnifySameDomain::new();
    u.build();
    assert!(u.is_done());
}

#[test]
fn test_unify_default_flags_true() {
    let u = ShapeUpgradeUnifySameDomain::new();
    assert!(u.unify_faces());
    assert!(u.unify_edges());
}

#[test]
fn test_unify_disable_faces() {
    let mut u = ShapeUpgradeUnifySameDomain::new();
    u.set_unify_faces(false);
    assert!(!u.unify_faces());
    assert!(u.unify_edges());
}

#[test]
fn test_unify_disable_edges() {
    let mut u = ShapeUpgradeUnifySameDomain::new();
    u.set_unify_edges(false);
    assert!(!u.unify_edges());
    assert!(u.unify_faces());
}

#[test]
fn test_unify_configure_and_build() {
    let mut u = ShapeUpgradeUnifySameDomain::new();
    u.set_unify_faces(true);
    u.set_unify_edges(false);
    u.set_concat_b_splines(false);
    u.set_angular_tolerance(1e-6);
    u.set_linear_tolerance(1e-5);
    u.build();
    assert!(u.is_done());
    assert!(u.unify_faces());
    assert!(!u.unify_edges());
}

// --- ShapeUpgradeShapeTolerances ---

#[test]
fn test_tolerances_new_stores_tolerance() {
    let st = ShapeUpgradeShapeTolerances::new(5e-4);
    assert!((st.tolerance() - 5e-4).abs() < 1e-15);
}

#[test]
fn test_tolerances_default_mode_is_all() {
    let st = ShapeUpgradeShapeTolerances::new(1e-7);
    assert_eq!(st.mode(), 3);
}

#[test]
fn test_tolerances_set_mode_vertex() {
    let mut st = ShapeUpgradeShapeTolerances::new(1e-7);
    st.set_mode(0);
    assert_eq!(st.mode(), 0);
}

#[test]
fn test_tolerances_perform_sets_done() {
    let mut st = ShapeUpgradeShapeTolerances::new(1e-7);
    assert!(!st.is_done());
    st.perform();
    assert!(st.is_done());
}

// --- ShapeUpgradeShapeDivide ---

#[test]
fn test_divide_initially_not_done() {
    let sd = ShapeUpgradeShapeDivide::new(1e-3);
    assert!(!sd.is_done());
    assert_eq!(sd.status(), ShapeUpgradeStatus::Ok);
}

#[test]
fn test_divide_perform_returns_done_status() {
    let mut sd = ShapeUpgradeShapeDivide::new(1e-3);
    let s = sd.perform(4);
    assert_eq!(s, ShapeUpgradeStatus::Done);
}

#[test]
fn test_divide_is_done_after_perform() {
    let mut sd = ShapeUpgradeShapeDivide::new(1e-3);
    sd.perform(1);
    assert!(sd.is_done());
    assert_eq!(sd.status(), ShapeUpgradeStatus::Done);
}

#[test]
fn test_divide_set_max_min_tolerance() {
    let mut sd = ShapeUpgradeShapeDivide::new(1e-4);
    sd.set_max_tolerance(2.0);
    sd.set_min_tolerance(1e-10);
    sd.perform(8);
    assert!(sd.is_done());
}

// --- ShapeUpgradeStatus enum ---

#[test]
fn test_status_debug_format() {
    let s = format!("{:?}", ShapeUpgradeStatus::Warn);
    assert_eq!(s, "Warn");
}

#[test]
fn test_status_all_variants_distinct() {
    assert_ne!(ShapeUpgradeStatus::Ok, ShapeUpgradeStatus::Done);
    assert_ne!(ShapeUpgradeStatus::Done, ShapeUpgradeStatus::Fail);
    assert_ne!(ShapeUpgradeStatus::Fail, ShapeUpgradeStatus::Warn);
    assert_ne!(ShapeUpgradeStatus::Ok, ShapeUpgradeStatus::Warn);
}
