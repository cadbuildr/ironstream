extern crate ironstream;
use ironstream::shape_checker::*;

#[test]
fn brep_check_analyzer_result_lookup_by_shape_id() {
    let mut ana = BrepCheckAnalyzer::new(1);
    let mut r2 = BrepCheckResult::new(2);
    r2.add(CheckResult::ok());
    let mut r3 = BrepCheckResult::new(3);
    r3.add(CheckResult::error(CheckStatus::FreeEdge, 3));
    ana.add_result(r2);
    ana.add_result(r3);
    assert_eq!(ana.nb_results(), 2);
    assert!(ana.result(2).is_some());
    assert!(ana.result(3).is_some());
    assert!(ana.result(99).is_none());
    assert!(ana.result(2).unwrap().is_valid());
    assert!(!ana.result(3).unwrap().is_valid());
}

#[test]
fn brep_check_result_statuses_returns_all_statuses() {
    let mut r = BrepCheckResult::new(10);
    r.add(CheckResult::ok());
    r.add(CheckResult::error(CheckStatus::NotClosed, 10));
    r.add(CheckResult::error(CheckStatus::FreeEdge, 10));
    let statuses = r.statuses();
    assert_eq!(statuses.len(), 3);
    assert!(statuses.contains(&CheckStatus::NoError));
    assert!(statuses.contains(&CheckStatus::NotClosed));
    assert!(statuses.contains(&CheckStatus::FreeEdge));
}

#[test]
fn brep_check_shell_set_connected_false_sets_not_connected() {
    let mut sh = BrepCheckShell::new(5);
    assert!(sh.is_connected);
    sh.set_connected(false);
    assert!(!sh.is_connected);
    assert_eq!(sh.status, CheckStatus::NotConnected);
    assert!(!sh.is_valid());
}

#[test]
fn check_small_face_with_tolerance_sets_custom_tolerances() {
    let csf = CheckSmallFace::with_tolerance(1e-4);
    assert!((csf.tolerance - 1e-4).abs() < 1e-20);
    assert!((csf.spot_tol - 1e-4).abs() < 1e-20);
    // strip_tol should be 100x the base tolerance
    assert!((csf.strip_tol - 1e-2).abs() < 1e-15);
}

#[test]
fn small_face_kind_default_is_not_small() {
    let kind = SmallFaceKind::default();
    assert_eq!(kind, SmallFaceKind::NotSmall);
}

#[test]
fn brep_check_result_is_valid_for_all_ok_results() {
    let mut r = BrepCheckResult::new(7);
    // No results at all: valid
    assert!(r.is_valid());
    r.add(CheckResult::ok());
    r.add(CheckResult::ok());
    assert!(r.is_valid());
    assert_eq!(r.nb_errors(), 0);
    // Adding an error invalidates it
    r.add(CheckResult::error(CheckStatus::InvalidRange, 7));
    assert!(!r.is_valid());
    assert_eq!(r.nb_errors(), 1);
}
