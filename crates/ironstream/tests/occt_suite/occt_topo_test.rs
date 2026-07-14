// FILE: tests/occt_topo_test.rs
extern crate ironstream;
use ironstream::topo_test::*;

// ---------------------------------------------------------------------------
// BrepCheckStatus
// ---------------------------------------------------------------------------

#[test]
fn test_status_ok_is_ok() {
    assert!(BrepCheckStatus::Ok.is_ok());
}

#[test]
fn test_status_no_error_is_ok() {
    assert!(BrepCheckStatus::NoError.is_ok());
}

#[test]
fn test_status_invalid_point_on_curve_not_ok() {
    assert!(!BrepCheckStatus::InvalidPointOnCurve.is_ok());
}

#[test]
fn test_status_invalid_point_on_surface_not_ok() {
    assert!(!BrepCheckStatus::InvalidPointOnSurface.is_ok());
}

#[test]
fn test_status_invalid_range_not_ok() {
    assert!(!BrepCheckStatus::InvalidRange.is_ok());
}

#[test]
fn test_status_empty_wire_not_ok() {
    assert!(!BrepCheckStatus::EmptyWire.is_ok());
}

#[test]
fn test_status_empty_shell_not_ok() {
    assert!(!BrepCheckStatus::EmptyShell.is_ok());
}

#[test]
fn test_status_redundant_edge_not_ok() {
    assert!(!BrepCheckStatus::RedundantEdge.is_ok());
}

#[test]
fn test_status_self_intersecting_wire_not_ok() {
    assert!(!BrepCheckStatus::SelfIntersectingWire.is_ok());
}

#[test]
fn test_status_geometry_inconsistency_not_ok() {
    assert!(!BrepCheckStatus::GeometryInconsistency.is_ok());
}

#[test]
fn test_status_invalid_curvature_radius_not_ok() {
    assert!(!BrepCheckStatus::InvalidCurvatureRadius.is_ok());
}

#[test]
fn test_status_unknown_not_ok() {
    assert!(!BrepCheckStatus::Unknown.is_ok());
}

#[test]
fn test_status_clone_and_copy() {
    let s = BrepCheckStatus::EmptyWire;
    let s2 = s;
    assert_eq!(s, s2);
    let s3 = s.clone();
    assert_eq!(s, s3);
}

#[test]
fn test_status_eq_ok_variants_both_pass() {
    assert!(BrepCheckStatus::Ok.is_ok());
    assert!(BrepCheckStatus::NoError.is_ok());
}

#[test]
fn test_status_debug_format() {
    let s = format!("{:?}", BrepCheckStatus::SelfIntersectingWire);
    assert_eq!(s, "SelfIntersectingWire");
}

// ---------------------------------------------------------------------------
// BrepCheckError
// ---------------------------------------------------------------------------

#[test]
fn test_error_new_stores_status() {
    let e = BrepCheckError::new(BrepCheckStatus::EmptyWire, "Wire_1");
    assert_eq!(e.status(), BrepCheckStatus::EmptyWire);
}

#[test]
fn test_error_new_stores_label() {
    let e = BrepCheckError::new(BrepCheckStatus::EmptyShell, "Shell_outer");
    assert_eq!(e.shape_label(), "Shell_outer");
}

#[test]
fn test_error_unknown_status() {
    let e = BrepCheckError::new(BrepCheckStatus::Unknown, "Face_42");
    assert_eq!(e.status(), BrepCheckStatus::Unknown);
    assert_eq!(e.shape_label(), "Face_42");
}

#[test]
fn test_error_clone() {
    let e = BrepCheckError::new(BrepCheckStatus::InvalidRange, "Edge_3");
    let e2 = e.clone();
    assert_eq!(e2.status(), BrepCheckStatus::InvalidRange);
    assert_eq!(e2.shape_label(), "Edge_3");
}

#[test]
fn test_error_empty_label() {
    let e = BrepCheckError::new(BrepCheckStatus::RedundantEdge, "");
    assert_eq!(e.shape_label(), "");
}

#[test]
fn test_error_label_with_spaces() {
    let e = BrepCheckError::new(BrepCheckStatus::GeometryInconsistency, "My Shape Label");
    assert_eq!(e.shape_label(), "My Shape Label");
}

#[test]
fn test_error_debug_format() {
    let e = BrepCheckError::new(BrepCheckStatus::EmptyWire, "W1");
    let s = format!("{:?}", e);
    assert!(s.contains("EmptyWire"));
    assert!(s.contains("W1"));
}

// ---------------------------------------------------------------------------
// BrepCheckResult
// ---------------------------------------------------------------------------

#[test]
fn test_result_new_is_valid() {
    let r = BrepCheckResult::new("Solid_1");
    assert!(r.is_valid());
}

#[test]
fn test_result_new_has_zero_errors() {
    let r = BrepCheckResult::new("Solid_1");
    assert_eq!(r.nb_errors(), 0);
}

#[test]
fn test_result_new_errors_slice_is_empty() {
    let r = BrepCheckResult::new("Shell_2");
    assert!(r.errors().is_empty());
}

#[test]
fn test_result_shape_label() {
    let r = BrepCheckResult::new("Edge_99");
    assert_eq!(r.shape_label(), "Edge_99");
}

#[test]
fn test_result_add_error_makes_invalid() {
    let mut r = BrepCheckResult::new("Wire_outer");
    r.add_error(BrepCheckError::new(BrepCheckStatus::EmptyWire, "Wire_outer"));
    assert!(!r.is_valid());
}

#[test]
fn test_result_add_error_increments_count() {
    let mut r = BrepCheckResult::new("Face_1");
    r.add_error(BrepCheckError::new(BrepCheckStatus::SelfIntersectingWire, "Face_1"));
    assert_eq!(r.nb_errors(), 1);
}

#[test]
fn test_result_add_multiple_errors() {
    let mut r = BrepCheckResult::new("Shell_broken");
    r.add_error(BrepCheckError::new(BrepCheckStatus::EmptyShell, "Shell_broken"));
    r.add_error(BrepCheckError::new(BrepCheckStatus::GeometryInconsistency, "Shell_broken"));
    assert_eq!(r.nb_errors(), 2);
    assert!(!r.is_valid());
}

#[test]
fn test_result_errors_slice_length_matches_nb_errors() {
    let mut r = BrepCheckResult::new("Solid_X");
    r.add_error(BrepCheckError::new(BrepCheckStatus::InvalidRange, "Edge_1"));
    r.add_error(BrepCheckError::new(BrepCheckStatus::RedundantEdge, "Edge_2"));
    assert_eq!(r.errors().len(), r.nb_errors());
}

#[test]
fn test_result_errors_preserves_order() {
    let mut r = BrepCheckResult::new("Shape_0");
    r.add_error(BrepCheckError::new(BrepCheckStatus::EmptyWire, "W1"));
    r.add_error(BrepCheckError::new(BrepCheckStatus::InvalidPointOnCurve, "E1"));
    let errs = r.errors();
    assert_eq!(errs[0].status(), BrepCheckStatus::EmptyWire);
    assert_eq!(errs[1].status(), BrepCheckStatus::InvalidPointOnCurve);
}

#[test]
fn test_result_clone() {
    let mut r = BrepCheckResult::new("Face_clone");
    r.add_error(BrepCheckError::new(BrepCheckStatus::Unknown, "Face_clone"));
    let r2 = r.clone();
    assert_eq!(r2.nb_errors(), 1);
    assert_eq!(r2.shape_label(), "Face_clone");
}

#[test]
fn test_result_debug_format() {
    let r = BrepCheckResult::new("Vertex_1");
    let s = format!("{:?}", r);
    assert!(s.contains("Vertex_1"));
}

// ---------------------------------------------------------------------------
// BrepCheckAnalyzer
// ---------------------------------------------------------------------------

#[test]
fn test_analyzer_new_is_valid() {
    let a = BrepCheckAnalyzer::new();
    assert!(a.is_valid());
}

#[test]
fn test_analyzer_new_has_zero_results() {
    let a = BrepCheckAnalyzer::new();
    assert_eq!(a.nb_results(), 0);
}

#[test]
fn test_analyzer_check_shape_valid_returns_valid_result() {
    let mut a = BrepCheckAnalyzer::new();
    let r = a.check_shape("Solid_1", true);
    assert!(r.is_valid());
    assert_eq!(r.nb_errors(), 0);
}

#[test]
fn test_analyzer_check_shape_invalid_returns_one_error() {
    let mut a = BrepCheckAnalyzer::new();
    let r = a.check_shape("Wire_broken", false);
    assert!(!r.is_valid());
    assert_eq!(r.nb_errors(), 1);
}

#[test]
fn test_analyzer_check_shape_invalid_error_status_is_unknown() {
    let mut a = BrepCheckAnalyzer::new();
    let r = a.check_shape("Shell_open", false);
    assert_eq!(r.errors()[0].status(), BrepCheckStatus::Unknown);
}

#[test]
fn test_analyzer_check_shape_invalid_error_label_matches() {
    let mut a = BrepCheckAnalyzer::new();
    let r = a.check_shape("Edge_bad", false);
    assert_eq!(r.errors()[0].shape_label(), "Edge_bad");
}

#[test]
fn test_analyzer_stores_result_after_check_shape() {
    let mut a = BrepCheckAnalyzer::new();
    a.check_shape("Face_1", true);
    assert_eq!(a.nb_results(), 1);
}

#[test]
fn test_analyzer_all_valid_check_shapes_keeps_is_valid_true() {
    let mut a = BrepCheckAnalyzer::new();
    a.check_shape("Solid_A", true);
    a.check_shape("Solid_B", true);
    assert!(a.is_valid());
}

#[test]
fn test_analyzer_one_invalid_makes_is_valid_false() {
    let mut a = BrepCheckAnalyzer::new();
    a.check_shape("Solid_A", true);
    a.check_shape("Wire_bad", false);
    assert!(!a.is_valid());
}

#[test]
fn test_analyzer_nb_results_counts_all_check_shape_calls() {
    let mut a = BrepCheckAnalyzer::new();
    a.check_shape("S1", true);
    a.check_shape("S2", false);
    a.check_shape("S3", true);
    assert_eq!(a.nb_results(), 3);
}

#[test]
fn test_analyzer_add_result_valid() {
    let mut a = BrepCheckAnalyzer::new();
    let r = BrepCheckResult::new("Face_ok");
    a.add_result(r);
    assert_eq!(a.nb_results(), 1);
    assert!(a.is_valid());
}

#[test]
fn test_analyzer_add_result_invalid_makes_is_valid_false() {
    let mut a = BrepCheckAnalyzer::new();
    let mut r = BrepCheckResult::new("Shell_bad");
    r.add_error(BrepCheckError::new(BrepCheckStatus::EmptyShell, "Shell_bad"));
    a.add_result(r);
    assert!(!a.is_valid());
}

#[test]
fn test_analyzer_add_result_increments_nb_results() {
    let mut a = BrepCheckAnalyzer::new();
    a.add_result(BrepCheckResult::new("R1"));
    a.add_result(BrepCheckResult::new("R2"));
    assert_eq!(a.nb_results(), 2);
}

#[test]
fn test_analyzer_mix_add_result_and_check_shape() {
    let mut a = BrepCheckAnalyzer::new();
    a.add_result(BrepCheckResult::new("Prebuilt_ok"));
    a.check_shape("Checked_ok", true);
    assert_eq!(a.nb_results(), 2);
    assert!(a.is_valid());
}

#[test]
fn test_analyzer_empty_label_check_shape() {
    let mut a = BrepCheckAnalyzer::new();
    let r = a.check_shape("", false);
    assert!(!r.is_valid());
    assert_eq!(r.errors()[0].shape_label(), "");
}

#[test]
fn test_analyzer_debug_format() {
    let a = BrepCheckAnalyzer::new();
    let s = format!("{:?}", a);
    assert!(s.contains("BrepCheckAnalyzer"));
}

#[test]
fn test_analyzer_default_equals_new() {
    let a1 = BrepCheckAnalyzer::new();
    let a2 = BrepCheckAnalyzer::default();
    assert_eq!(a1.nb_results(), a2.nb_results());
    assert_eq!(a1.is_valid(), a2.is_valid());
}
