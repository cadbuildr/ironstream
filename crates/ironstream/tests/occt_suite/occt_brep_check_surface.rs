// FILE: rust/ironstream/crates/ironstream/tests/occt_brep_check_surface.rs
extern crate ironstream;
use ironstream::brep_check_surface::*;

// ---------------------------------------------------------------------------
// SurfaceCheckStatus tests
// ---------------------------------------------------------------------------

#[test]
fn status_no_error_is_ok() {
    assert!(SurfaceCheckStatus::NoError.is_ok());
}

#[test]
fn status_all_errors_are_not_ok() {
    let errors = [
        SurfaceCheckStatus::InvalidSameDomainFlag,
        SurfaceCheckStatus::InvalidRange,
        SurfaceCheckStatus::InvalidCurvatureRadius,
        SurfaceCheckStatus::BadOrientation,
        SurfaceCheckStatus::CheckFail,
    ];
    for e in &errors {
        assert!(!e.is_ok(), "{:?} should not be ok", e);
    }
}

#[test]
fn status_description_non_empty_for_all_variants() {
    let all = [
        SurfaceCheckStatus::NoError,
        SurfaceCheckStatus::InvalidSameDomainFlag,
        SurfaceCheckStatus::InvalidRange,
        SurfaceCheckStatus::InvalidCurvatureRadius,
        SurfaceCheckStatus::BadOrientation,
        SurfaceCheckStatus::CheckFail,
    ];
    for s in &all {
        assert!(!s.description().is_empty(), "{:?} must have a non-empty description", s);
    }
}

#[test]
fn status_variants_are_distinct() {
    assert_ne!(SurfaceCheckStatus::NoError, SurfaceCheckStatus::CheckFail);
    assert_ne!(
        SurfaceCheckStatus::InvalidRange,
        SurfaceCheckStatus::InvalidCurvatureRadius
    );
    assert_ne!(
        SurfaceCheckStatus::BadOrientation,
        SurfaceCheckStatus::InvalidSameDomainFlag
    );
}

#[test]
fn status_copy_and_eq() {
    let s = SurfaceCheckStatus::BadOrientation;
    let s2 = s;
    assert_eq!(s, s2);
}

// ---------------------------------------------------------------------------
// SurfaceCheckResult::new tests
// ---------------------------------------------------------------------------

#[test]
fn result_new_has_no_error_status() {
    let r = SurfaceCheckResult::new("surf_1");
    assert_eq!(r.status(), SurfaceCheckStatus::NoError);
}

#[test]
fn result_new_is_valid() {
    let r = SurfaceCheckResult::new("surf_1");
    assert!(r.is_valid());
}

#[test]
fn result_new_label_matches() {
    let r = SurfaceCheckResult::new("my_surface");
    assert_eq!(r.surface_label(), "my_surface");
}

#[test]
fn result_new_u_range_is_zero() {
    let r = SurfaceCheckResult::new("s");
    assert_eq!(r.u_range(), (0.0, 0.0));
}

#[test]
fn result_new_v_range_is_zero() {
    let r = SurfaceCheckResult::new("s");
    assert_eq!(r.v_range(), (0.0, 0.0));
}

#[test]
fn result_new_max_error_is_zero() {
    let r = SurfaceCheckResult::new("s");
    assert_eq!(r.max_error(), 0.0);
}

// ---------------------------------------------------------------------------
// SurfaceCheckResult setters / getters
// ---------------------------------------------------------------------------

#[test]
fn result_set_status_reflects_in_getter() {
    let mut r = SurfaceCheckResult::new("s");
    r.set_status(SurfaceCheckStatus::InvalidRange);
    assert_eq!(r.status(), SurfaceCheckStatus::InvalidRange);
}

#[test]
fn result_set_error_status_makes_invalid() {
    let mut r = SurfaceCheckResult::new("s");
    r.set_status(SurfaceCheckStatus::BadOrientation);
    assert!(!r.is_valid());
}

#[test]
fn result_set_no_error_status_makes_valid() {
    let mut r = SurfaceCheckResult::new("s");
    r.set_status(SurfaceCheckStatus::CheckFail);
    assert!(!r.is_valid());
    r.set_status(SurfaceCheckStatus::NoError);
    assert!(r.is_valid());
}

#[test]
fn result_set_u_range_roundtrips() {
    let mut r = SurfaceCheckResult::new("s");
    r.set_u_range(-1.0, 2.5);
    assert_eq!(r.u_range(), (-1.0, 2.5));
}

#[test]
fn result_set_v_range_roundtrips() {
    let mut r = SurfaceCheckResult::new("s");
    r.set_v_range(0.0, 6.283_185_307);
    let (v_min, v_max) = r.v_range();
    assert!((v_min - 0.0).abs() < 1e-10);
    assert!((v_max - 6.283_185_307).abs() < 1e-10);
}

#[test]
fn result_set_max_error_roundtrips() {
    let mut r = SurfaceCheckResult::new("s");
    r.set_max_error(1.23e-6);
    assert!((r.max_error() - 1.23e-6).abs() < 1e-15);
}

#[test]
fn result_clone_is_independent() {
    let mut r1 = SurfaceCheckResult::new("orig");
    r1.set_u_range(0.0, 1.0);
    let mut r2 = r1.clone();
    r2.set_u_range(5.0, 10.0);
    // r1 must be unchanged
    assert_eq!(r1.u_range(), (0.0, 1.0));
    assert_eq!(r2.u_range(), (5.0, 10.0));
}

// ---------------------------------------------------------------------------
// SurfaceChecker tests
// ---------------------------------------------------------------------------

#[test]
fn checker_new_has_zero_results() {
    let c = SurfaceChecker::new();
    assert_eq!(c.nb_results(), 0);
}

#[test]
fn checker_new_all_valid_on_empty() {
    let c = SurfaceChecker::new();
    assert!(c.all_valid());
}

#[test]
fn checker_check_valid_returns_no_error_result() {
    let mut c = SurfaceChecker::new();
    let r = c.check("plane_1", true);
    assert_eq!(r.status(), SurfaceCheckStatus::NoError);
    assert!(r.is_valid());
}

#[test]
fn checker_check_invalid_returns_check_fail_result() {
    let mut c = SurfaceChecker::new();
    let r = c.check("bad_surf", false);
    assert_eq!(r.status(), SurfaceCheckStatus::CheckFail);
    assert!(!r.is_valid());
}

#[test]
fn checker_check_appends_to_results() {
    let mut c = SurfaceChecker::new();
    c.check("s1", true);
    assert_eq!(c.nb_results(), 1);
    c.check("s2", false);
    assert_eq!(c.nb_results(), 2);
}

#[test]
fn checker_check_label_preserved_in_result() {
    let mut c = SurfaceChecker::new();
    let r = c.check("label_xyz", true);
    assert_eq!(r.surface_label(), "label_xyz");
}

#[test]
fn checker_all_valid_true_when_all_no_error() {
    let mut c = SurfaceChecker::new();
    c.check("s1", true);
    c.check("s2", true);
    c.check("s3", true);
    assert!(c.all_valid());
}

#[test]
fn checker_all_valid_false_when_one_fails() {
    let mut c = SurfaceChecker::new();
    c.check("good", true);
    c.check("bad", false);
    assert!(!c.all_valid());
}

#[test]
fn checker_add_result_increments_nb_results() {
    let mut c = SurfaceChecker::new();
    let mut r = SurfaceCheckResult::new("external");
    r.set_status(SurfaceCheckStatus::InvalidRange);
    c.add_result(r);
    assert_eq!(c.nb_results(), 1);
}

#[test]
fn checker_add_result_invalid_makes_all_valid_false() {
    let mut c = SurfaceChecker::new();
    c.check("ok", true);
    let mut r = SurfaceCheckResult::new("bad");
    r.set_status(SurfaceCheckStatus::InvalidCurvatureRadius);
    c.add_result(r);
    assert!(!c.all_valid());
}

#[test]
fn checker_add_result_valid_keeps_all_valid_true() {
    let mut c = SurfaceChecker::new();
    c.check("ok1", true);
    let r = SurfaceCheckResult::new("ok2"); // NoError by default
    c.add_result(r);
    assert!(c.all_valid());
}

#[test]
fn checker_nb_results_matches_check_calls() {
    let mut c = SurfaceChecker::new();
    for i in 0..10 {
        c.check(&format!("surf_{}", i), i % 2 == 0);
    }
    assert_eq!(c.nb_results(), 10);
}

#[test]
fn checker_mixed_valid_invalid_all_valid_false() {
    let mut c = SurfaceChecker::new();
    c.check("a", true);
    c.check("b", true);
    c.check("c", false); // one failure
    c.check("d", true);
    assert!(!c.all_valid());
}

#[test]
fn checker_default_same_as_new() {
    let c: SurfaceChecker = Default::default();
    assert_eq!(c.nb_results(), 0);
    assert!(c.all_valid());
}

// ---------------------------------------------------------------------------
// SurfaceCheckStatus debug/hash sanity
// ---------------------------------------------------------------------------

#[test]
fn status_debug_contains_variant_name() {
    let s = format!("{:?}", SurfaceCheckStatus::InvalidRange);
    assert!(s.contains("InvalidRange"));
}

#[test]
fn status_hash_stable() {
    use std::collections::HashSet;
    let mut set = HashSet::new();
    set.insert(SurfaceCheckStatus::NoError);
    set.insert(SurfaceCheckStatus::CheckFail);
    set.insert(SurfaceCheckStatus::NoError); // duplicate
    assert_eq!(set.len(), 2);
}
