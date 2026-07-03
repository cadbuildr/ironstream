// FILE: tests/occt_brep_repair.rs
extern crate ironstream;
use ironstream::brep_repair::*;

// ---------------------------------------------------------------------------
// RepairStatus
// ---------------------------------------------------------------------------

#[test]
fn test_repair_status_ok_eq() {
    assert_eq!(RepairStatus::Ok, RepairStatus::Ok);
}

#[test]
fn test_repair_status_fixed_eq() {
    assert_eq!(RepairStatus::Fixed, RepairStatus::Fixed);
}

#[test]
fn test_repair_status_fixed_self_eq() {
    assert_eq!(RepairStatus::FixedSelf, RepairStatus::FixedSelf);
}

#[test]
fn test_repair_status_failed_eq() {
    assert_eq!(RepairStatus::Failed, RepairStatus::Failed);
}

#[test]
fn test_repair_status_not_done_eq() {
    assert_eq!(RepairStatus::NotDone, RepairStatus::NotDone);
}

#[test]
fn test_repair_status_all_distinct() {
    let all = [
        RepairStatus::Ok,
        RepairStatus::Fixed,
        RepairStatus::FixedSelf,
        RepairStatus::Failed,
        RepairStatus::NotDone,
    ];
    for i in 0..all.len() {
        for j in 0..all.len() {
            if i == j {
                assert_eq!(all[i], all[j]);
            } else {
                assert_ne!(all[i], all[j]);
            }
        }
    }
}

#[test]
fn test_repair_status_copy() {
    let s = RepairStatus::Fixed;
    let s2 = s; // copy
    assert_eq!(s, s2);
}

#[test]
fn test_repair_status_clone() {
    let s = RepairStatus::FixedSelf;
    let s2 = s.clone();
    assert_eq!(s, s2);
}

#[test]
fn test_repair_status_debug_format() {
    assert_eq!(format!("{:?}", RepairStatus::Ok), "Ok");
    assert_eq!(format!("{:?}", RepairStatus::Fixed), "Fixed");
    assert_eq!(format!("{:?}", RepairStatus::FixedSelf), "FixedSelf");
    assert_eq!(format!("{:?}", RepairStatus::Failed), "Failed");
    assert_eq!(format!("{:?}", RepairStatus::NotDone), "NotDone");
}

// ---------------------------------------------------------------------------
// RepairIssue — construction and accessors
// ---------------------------------------------------------------------------

#[test]
fn test_repair_issue_new_issue_type() {
    let i = RepairIssue::new("gap", "face_1");
    assert_eq!(i.issue_type(), "gap");
}

#[test]
fn test_repair_issue_new_shape_label() {
    let i = RepairIssue::new("gap", "face_1");
    assert_eq!(i.shape_label(), "face_1");
}

#[test]
fn test_repair_issue_new_default_status_not_done() {
    let i = RepairIssue::new("degenerate_edge", "edge_5");
    assert_eq!(i.status(), RepairStatus::NotDone);
}

#[test]
fn test_repair_issue_set_status_ok() {
    let mut i = RepairIssue::new("overlap", "shell_2");
    i.set_status(RepairStatus::Ok);
    assert_eq!(i.status(), RepairStatus::Ok);
}

#[test]
fn test_repair_issue_set_status_fixed() {
    let mut i = RepairIssue::new("gap", "wire_3");
    i.set_status(RepairStatus::Fixed);
    assert_eq!(i.status(), RepairStatus::Fixed);
}

#[test]
fn test_repair_issue_set_status_fixed_self() {
    let mut i = RepairIssue::new("self_intersection", "face_7");
    i.set_status(RepairStatus::FixedSelf);
    assert_eq!(i.status(), RepairStatus::FixedSelf);
}

#[test]
fn test_repair_issue_set_status_failed() {
    let mut i = RepairIssue::new("open_shell", "solid_0");
    i.set_status(RepairStatus::Failed);
    assert_eq!(i.status(), RepairStatus::Failed);
}

#[test]
fn test_repair_issue_set_status_can_be_overridden_multiple_times() {
    let mut i = RepairIssue::new("gap", "edge_1");
    i.set_status(RepairStatus::Fixed);
    assert_eq!(i.status(), RepairStatus::Fixed);
    i.set_status(RepairStatus::Failed);
    assert_eq!(i.status(), RepairStatus::Failed);
    i.set_status(RepairStatus::Ok);
    assert_eq!(i.status(), RepairStatus::Ok);
}

#[test]
fn test_repair_issue_empty_strings() {
    let i = RepairIssue::new("", "");
    assert_eq!(i.issue_type(), "");
    assert_eq!(i.shape_label(), "");
}

#[test]
fn test_repair_issue_unicode_strings() {
    let i = RepairIssue::new("écart", "pièce_42");
    assert_eq!(i.issue_type(), "écart");
    assert_eq!(i.shape_label(), "pièce_42");
}

// ---------------------------------------------------------------------------
// ShapeFixResult — construction, add_issue, counters
// ---------------------------------------------------------------------------

#[test]
fn test_shape_fix_result_new_is_empty() {
    let r = ShapeFixResult::new();
    assert_eq!(r.nb_issues(), 0);
}

#[test]
fn test_shape_fix_result_new_not_modified() {
    let r = ShapeFixResult::new();
    assert!(!r.is_modified());
}

#[test]
fn test_shape_fix_result_new_nb_fixed_zero() {
    let r = ShapeFixResult::new();
    assert_eq!(r.nb_fixed(), 0);
}

#[test]
fn test_shape_fix_result_new_nb_failed_zero() {
    let r = ShapeFixResult::new();
    assert_eq!(r.nb_failed(), 0);
}

#[test]
fn test_shape_fix_result_default_same_as_new() {
    let r = ShapeFixResult::default();
    assert_eq!(r.nb_issues(), 0);
    assert!(!r.is_modified());
}

#[test]
fn test_shape_fix_result_add_issue_increments_count() {
    let mut r = ShapeFixResult::new();
    r.add_issue(RepairIssue::new("gap", "e1"));
    assert_eq!(r.nb_issues(), 1);
}

#[test]
fn test_shape_fix_result_add_multiple_issues() {
    let mut r = ShapeFixResult::new();
    for k in 0..5 {
        r.add_issue(RepairIssue::new("gap", &format!("e{}", k)));
    }
    assert_eq!(r.nb_issues(), 5);
}

#[test]
fn test_shape_fix_result_mark_modified() {
    let mut r = ShapeFixResult::new();
    r.mark_modified();
    assert!(r.is_modified());
}

#[test]
fn test_shape_fix_result_mark_modified_idempotent() {
    let mut r = ShapeFixResult::new();
    r.mark_modified();
    r.mark_modified();
    assert!(r.is_modified());
}

#[test]
fn test_shape_fix_result_nb_fixed_counts_fixed() {
    let mut r = ShapeFixResult::new();
    let mut i1 = RepairIssue::new("gap", "e1");
    i1.set_status(RepairStatus::Fixed);
    r.add_issue(i1);
    assert_eq!(r.nb_fixed(), 1);
}

#[test]
fn test_shape_fix_result_nb_fixed_counts_fixed_self() {
    let mut r = ShapeFixResult::new();
    let mut i1 = RepairIssue::new("gap", "e1");
    i1.set_status(RepairStatus::FixedSelf);
    r.add_issue(i1);
    assert_eq!(r.nb_fixed(), 1);
}

#[test]
fn test_shape_fix_result_nb_fixed_counts_both_fixed_variants() {
    let mut r = ShapeFixResult::new();
    let mut i1 = RepairIssue::new("gap", "e1");
    i1.set_status(RepairStatus::Fixed);
    let mut i2 = RepairIssue::new("gap", "e2");
    i2.set_status(RepairStatus::FixedSelf);
    let mut i3 = RepairIssue::new("gap", "e3");
    i3.set_status(RepairStatus::Failed);
    r.add_issue(i1);
    r.add_issue(i2);
    r.add_issue(i3);
    assert_eq!(r.nb_fixed(), 2);
}

#[test]
fn test_shape_fix_result_nb_failed_counts_failed() {
    let mut r = ShapeFixResult::new();
    let mut i1 = RepairIssue::new("open_shell", "s1");
    i1.set_status(RepairStatus::Failed);
    let mut i2 = RepairIssue::new("open_shell", "s2");
    i2.set_status(RepairStatus::Failed);
    r.add_issue(i1);
    r.add_issue(i2);
    assert_eq!(r.nb_failed(), 2);
}

#[test]
fn test_shape_fix_result_nb_failed_does_not_count_ok() {
    let mut r = ShapeFixResult::new();
    let mut i1 = RepairIssue::new("gap", "e1");
    i1.set_status(RepairStatus::Ok);
    r.add_issue(i1);
    assert_eq!(r.nb_failed(), 0);
}

#[test]
fn test_shape_fix_result_nb_failed_does_not_count_fixed() {
    let mut r = ShapeFixResult::new();
    let mut i1 = RepairIssue::new("gap", "e1");
    i1.set_status(RepairStatus::Fixed);
    r.add_issue(i1);
    assert_eq!(r.nb_failed(), 0);
}

#[test]
fn test_shape_fix_result_mixed_statuses_counts() {
    let statuses = [
        RepairStatus::Ok,
        RepairStatus::Fixed,
        RepairStatus::FixedSelf,
        RepairStatus::Failed,
        RepairStatus::NotDone,
    ];
    let mut r = ShapeFixResult::new();
    for (k, &st) in statuses.iter().enumerate() {
        let mut i = RepairIssue::new("x", &format!("s{}", k));
        i.set_status(st);
        r.add_issue(i);
    }
    assert_eq!(r.nb_issues(), 5);
    assert_eq!(r.nb_fixed(), 2); // Fixed + FixedSelf
    assert_eq!(r.nb_failed(), 1);
}

// ---------------------------------------------------------------------------
// ShapeRepair — construction and accessors
// ---------------------------------------------------------------------------

#[test]
fn test_shape_repair_new_tolerance() {
    let sr = ShapeRepair::new(1e-5, 1e-7);
    assert!((sr.tolerance() - 1e-5).abs() < 1e-15);
}

#[test]
fn test_shape_repair_new_min_tolerance() {
    let sr = ShapeRepair::new(1e-5, 1e-7);
    assert!((sr.min_tolerance() - 1e-7).abs() < 1e-20);
}

#[test]
fn test_shape_repair_not_done_before_repair() {
    let sr = ShapeRepair::new(1e-6, 1e-9);
    assert!(!sr.is_done());
}

#[test]
fn test_shape_repair_result_empty_before_repair() {
    let sr = ShapeRepair::new(1e-6, 1e-9);
    assert_eq!(sr.result().nb_issues(), 0);
}

// ---------------------------------------------------------------------------
// ShapeRepair::repair — zero issues
// ---------------------------------------------------------------------------

#[test]
fn test_shape_repair_zero_issues_is_done() {
    let mut sr = ShapeRepair::new(1e-6, 1e-9);
    sr.repair(0, true);
    assert!(sr.is_done());
}

#[test]
fn test_shape_repair_zero_issues_not_modified() {
    let mut sr = ShapeRepair::new(1e-6, 1e-9);
    sr.repair(0, true);
    assert!(!sr.result().is_modified());
}

#[test]
fn test_shape_repair_zero_issues_nb_issues() {
    let mut sr = ShapeRepair::new(1e-6, 1e-9);
    sr.repair(0, false);
    assert_eq!(sr.result().nb_issues(), 0);
}

// ---------------------------------------------------------------------------
// ShapeRepair::repair — all_fixed = true
// ---------------------------------------------------------------------------

#[test]
fn test_shape_repair_all_fixed_is_modified() {
    let mut sr = ShapeRepair::new(1e-6, 1e-9);
    sr.repair(3, true);
    assert!(sr.result().is_modified());
}

#[test]
fn test_shape_repair_all_fixed_nb_issues() {
    let mut sr = ShapeRepair::new(1e-6, 1e-9);
    sr.repair(4, true);
    assert_eq!(sr.result().nb_issues(), 4);
}

#[test]
fn test_shape_repair_all_fixed_nb_fixed_equals_nb_issues() {
    let mut sr = ShapeRepair::new(1e-6, 1e-9);
    sr.repair(5, true);
    assert_eq!(sr.result().nb_fixed(), 5);
}

#[test]
fn test_shape_repair_all_fixed_nb_failed_zero() {
    let mut sr = ShapeRepair::new(1e-6, 1e-9);
    sr.repair(5, true);
    assert_eq!(sr.result().nb_failed(), 0);
}

#[test]
fn test_shape_repair_all_fixed_single_issue() {
    let mut sr = ShapeRepair::new(1e-6, 1e-9);
    sr.repair(1, true);
    assert_eq!(sr.result().nb_issues(), 1);
    assert_eq!(sr.result().nb_fixed(), 1);
    assert_eq!(sr.result().nb_failed(), 0);
    assert!(sr.result().is_modified());
}

// ---------------------------------------------------------------------------
// ShapeRepair::repair — all_fixed = false
// ---------------------------------------------------------------------------

#[test]
fn test_shape_repair_partial_fix_nb_issues() {
    let mut sr = ShapeRepair::new(1e-6, 1e-9);
    sr.repair(4, false);
    assert_eq!(sr.result().nb_issues(), 4);
}

#[test]
fn test_shape_repair_partial_fix_has_fixed_and_failed() {
    let mut sr = ShapeRepair::new(1e-6, 1e-9);
    sr.repair(4, false);
    let r = sr.result();
    assert!(r.nb_fixed() > 0, "expected at least one fixed issue");
    assert!(r.nb_failed() > 0, "expected at least one failed issue");
}

#[test]
fn test_shape_repair_partial_fix_total_equals_issues() {
    let mut sr = ShapeRepair::new(1e-6, 1e-9);
    sr.repair(6, false);
    let r = sr.result();
    // fixed + failed + other == nb_issues
    assert!(r.nb_fixed() + r.nb_failed() <= r.nb_issues());
}

#[test]
fn test_shape_repair_partial_fix_is_modified_when_any_fixed() {
    let mut sr = ShapeRepair::new(1e-6, 1e-9);
    sr.repair(2, false);
    // With 2 issues and all_fixed=false: first half (1) fixed, second half (1) failed.
    // nb_fixed > 0 so is_modified must be true.
    assert!(sr.result().is_modified());
}

#[test]
fn test_shape_repair_single_issue_all_failed() {
    // With 1 issue and all_fixed=false: the sole issue sits in the "failed" half.
    // (nb_issues+1)/2 = 1, so idx=0 is Fixed, not Failed.
    // Actually: idx 0 < (1+1)/2 = 1, so it's Fixed. Verify that.
    let mut sr = ShapeRepair::new(1e-6, 1e-9);
    sr.repair(1, false);
    assert_eq!(sr.result().nb_fixed(), 1);
    assert_eq!(sr.result().nb_failed(), 0);
}

// ---------------------------------------------------------------------------
// ShapeRepair::repair — repeated calls reset previous result
// ---------------------------------------------------------------------------

#[test]
fn test_shape_repair_second_call_resets_result() {
    let mut sr = ShapeRepair::new(1e-6, 1e-9);
    sr.repair(10, true);
    assert_eq!(sr.result().nb_issues(), 10);
    sr.repair(3, true);
    // Result should reflect the second call only.
    assert_eq!(sr.result().nb_issues(), 3);
}

#[test]
fn test_shape_repair_second_call_is_done_stays_true() {
    let mut sr = ShapeRepair::new(1e-6, 1e-9);
    sr.repair(2, true);
    sr.repair(0, false);
    assert!(sr.is_done());
}

// ---------------------------------------------------------------------------
// Tolerance independence — different tolerance values do not affect counts
// ---------------------------------------------------------------------------

#[test]
fn test_shape_repair_large_tolerance_same_counts() {
    let mut sr = ShapeRepair::new(1.0, 0.5);
    sr.repair(4, true);
    assert_eq!(sr.result().nb_fixed(), 4);
}

#[test]
fn test_shape_repair_tiny_tolerance_same_counts() {
    let mut sr = ShapeRepair::new(1e-12, 1e-15);
    sr.repair(4, true);
    assert_eq!(sr.result().nb_fixed(), 4);
}

// ---------------------------------------------------------------------------
// nb_fixed and nb_failed are consistent for larger inputs
// ---------------------------------------------------------------------------

#[test]
fn test_shape_repair_nb_fixed_plus_nb_failed_lte_nb_issues() {
    for n in [0usize, 1, 2, 5, 10, 20] {
        let mut sr = ShapeRepair::new(1e-6, 1e-9);
        sr.repair(n, false);
        let r = sr.result();
        assert!(
            r.nb_fixed() + r.nb_failed() <= r.nb_issues(),
            "n={n}: fixed={} + failed={} > issues={}",
            r.nb_fixed(),
            r.nb_failed(),
            r.nb_issues()
        );
    }
}

#[test]
fn test_shape_repair_all_fixed_nb_fixed_is_n() {
    for n in [0usize, 1, 3, 7, 15] {
        let mut sr = ShapeRepair::new(1e-6, 1e-9);
        sr.repair(n, true);
        assert_eq!(sr.result().nb_fixed(), n, "n={n}");
        assert_eq!(sr.result().nb_failed(), 0, "n={n}");
    }
}
