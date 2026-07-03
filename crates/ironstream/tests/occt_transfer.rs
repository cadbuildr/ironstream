// FILE: tests/occt_transfer.rs
//! Integration tests for `ironstream::transfer` — the Transfer framework
//! ported from OpenCascade's `Transfer` package.
//!
//! These tests exercise the public API from an external-crate perspective,
//! mirroring the way a STEP/IGES reader would use the Transfer package:
//! building a process, registering bindings, querying results, and inspecting
//! statistics.

extern crate ironstream;

use ironstream::transfer::{
    BinderStatus, Transfer_Binder, Transfer_FinderProcess, Transfer_TransientProcess,
    Transfer_VoidBinder, TransferError,
};

// ---------------------------------------------------------------------------
// BinderStatus
// ---------------------------------------------------------------------------

#[test]
fn binder_status_variants_display_correctly() {
    assert_eq!(BinderStatus::Void.to_string(),  "Void");
    assert_eq!(BinderStatus::Run.to_string(),   "Run");
    assert_eq!(BinderStatus::Done.to_string(),  "Done");
    assert_eq!(BinderStatus::Error.to_string(), "Error");
}

#[test]
fn binder_status_default_is_void() {
    let s: BinderStatus = Default::default();
    assert_eq!(s, BinderStatus::Void);
}

// ---------------------------------------------------------------------------
// Transfer_Binder
// ---------------------------------------------------------------------------

#[test]
fn binder_new_starts_in_void_state() {
    let b = Transfer_Binder::new();
    assert_eq!(b.status(), BinderStatus::Void);
    assert!(!b.has_result());
    assert!(!b.is_done());
    assert!(!b.has_failed());
    assert!(!b.has_next());
}

#[test]
fn binder_with_result_stores_f64_and_reports_done() {
    let b = Transfer_Binder::with_result(2.718_f64);
    assert!(b.is_done());
    assert!(b.has_result());
    let v = b.result::<f64>().copied().unwrap();
    assert!((v - 2.718).abs() < 1e-9);
}

#[test]
fn binder_result_wrong_type_returns_none() {
    let b = Transfer_Binder::with_result(100_i32);
    assert_eq!(b.result::<f64>(), None);
    assert_eq!(b.result::<i32>(), Some(&100_i32));
}

#[test]
fn binder_error_status_stores_message() {
    let mut b = Transfer_Binder::new();
    b.set_status_error("failed to read entity 42");
    assert!(b.has_failed());
    assert_eq!(b.status(), BinderStatus::Error);
    assert_eq!(b.error_message(), Some("failed to read entity 42"));
}

#[test]
fn binder_void_is_done_and_empty() {
    let b = Transfer_Binder::void();
    assert!(b.is_void_binder());
    assert!(b.is_done());
    assert!(!b.has_result());
}

#[test]
fn binder_chain_links_and_counts_correctly() {
    let mut root = Transfer_Binder::with_result("solid".to_string());
    let link1   = Transfer_Binder::with_result("face".to_string());
    let link2   = Transfer_Binder::with_result("edge".to_string());
    let mut mid = link1;
    mid.set_next(link2);
    root.set_next(mid);

    assert_eq!(root.chain_length(), 3);
    let n1 = root.next().unwrap();
    let n2 = n1.next().unwrap();
    assert_eq!(n1.result::<String>().map(String::as_str), Some("face"));
    assert_eq!(n2.result::<String>().map(String::as_str), Some("edge"));
}

// ---------------------------------------------------------------------------
// Transfer_VoidBinder
// ---------------------------------------------------------------------------

#[test]
fn void_binder_new_has_no_annotation() {
    let vb = Transfer_VoidBinder::new();
    assert!(vb.annotation().is_none());
    assert_eq!(vb.status(), BinderStatus::Done);
}

#[test]
fn void_binder_with_annotation_round_trips() {
    let vb = Transfer_VoidBinder::with_annotation("structural entity — no geometry");
    assert_eq!(vb.annotation(), Some("structural entity — no geometry"));
}

#[test]
fn void_binder_converts_to_generic_binder() {
    let vb = Transfer_VoidBinder::with_annotation("no shape");
    let b = vb.into_binder();
    assert!(b.is_void_binder());
    assert!(b.is_done());
    assert!(!b.has_result());
}

// ---------------------------------------------------------------------------
// TransferError
// ---------------------------------------------------------------------------

#[test]
fn transfer_error_message_and_display() {
    let e = TransferError::new("entity lookup failed");
    assert_eq!(e.message, "entity lookup failed");
    let s = e.to_string();
    assert!(s.contains("entity lookup failed"));
    assert!(s.contains("Transfer error"));
}

// ---------------------------------------------------------------------------
// Transfer_FinderProcess
// ---------------------------------------------------------------------------

#[test]
fn finder_process_empty_after_construction() {
    let fp = Transfer_FinderProcess::new();
    assert_eq!(fp.nb_mapped(), 0);
    assert_eq!(fp.nb_done(), 0);
    assert_eq!(fp.nb_errors(), 0);
    assert!(fp.roots().is_empty());
}

#[test]
fn finder_process_bind_result_and_find() {
    let mut fp = Transfer_FinderProcess::new();
    fp.bind_result(1u64, "ShapeA".to_string());
    assert!(fp.is_bound(1));
    assert!(!fp.is_bound(99));
    assert_eq!(
        fp.find_result::<String>(1).map(String::as_str),
        Some("ShapeA")
    );
}

#[test]
fn finder_process_bind_void_is_done() {
    let mut fp = Transfer_FinderProcess::new();
    fp.bind_void(7u64);
    let b = fp.find(7).expect("key 7 must be bound");
    assert!(b.is_void_binder());
    assert!(b.is_done());
}

#[test]
fn finder_process_start_finish_transfer() {
    let mut fp = Transfer_FinderProcess::new();
    fp.start_transfer(42u64).unwrap();
    // While running, the binder should be in Run state.
    assert_eq!(fp.find(42).unwrap().status(), BinderStatus::Run);
    // Finish with a result.
    fp.finish_transfer(42u64, 1000_u32).unwrap();
    assert!(fp.find(42).unwrap().is_done());
    assert_eq!(fp.find_result::<u32>(42).copied(), Some(1000));
}

#[test]
fn finder_process_cycle_detection() {
    let mut fp = Transfer_FinderProcess::new();
    fp.start_transfer(5u64).unwrap();
    // Attempting to start the same key while it is running triggers a cycle error.
    let r = fp.start_transfer(5u64);
    assert!(r.is_err());
    assert!(r.unwrap_err().message.contains("cycle"));
}

#[test]
fn finder_process_roots_ordered() {
    let mut fp = Transfer_FinderProcess::new();
    fp.bind_result(10u64, 10_i32);
    fp.bind_result(20u64, 20_i32);
    fp.bind_result(30u64, 30_i32);
    fp.set_root(30).unwrap();
    fp.set_root(10).unwrap();
    assert_eq!(fp.roots(), &[30u64, 10u64]);
}

#[test]
fn finder_process_set_root_unbound_returns_error() {
    let mut fp = Transfer_FinderProcess::new();
    let r = fp.set_root(999u64);
    assert!(r.is_err());
}

#[test]
fn finder_process_statistics_after_mixed_transfers() {
    let mut fp = Transfer_FinderProcess::new();
    fp.bind_result(1u64, "ok".to_string());
    fp.bind_void(2u64);
    let mut b = Transfer_Binder::new();
    b.set_status_error("parse error");
    fp.bind(3u64, b);
    fp.fail_transfer(3u64, "parse error").ok(); // already in error state
    assert_eq!(fp.nb_mapped(), 3);
    assert_eq!(fp.nb_done(),   2); // key 1 (result) and key 2 (void)
    assert_eq!(fp.nb_errors(), 1);
}

#[test]
fn finder_process_clear_resets_everything() {
    let mut fp = Transfer_FinderProcess::new();
    fp.bind_result(1u64, 1_i32);
    fp.set_root(1).unwrap();
    fp.clear();
    assert_eq!(fp.nb_mapped(), 0);
    assert!(fp.roots().is_empty());
    assert!(!fp.is_bound(1));
}

// ---------------------------------------------------------------------------
// Transfer_TransientProcess
// ---------------------------------------------------------------------------

#[test]
fn transient_process_set_model_activates() {
    let mut tp = Transfer_TransientProcess::new();
    assert!(!tp.is_active());
    tp.set_model(vec![100, 200, 300]);
    assert!(tp.is_active());
    assert_eq!(tp.nb_entities(), 3);
}

#[test]
fn transient_process_entity_rank_and_at() {
    let mut tp = Transfer_TransientProcess::new();
    tp.set_model(vec![10, 20, 30, 40]);
    assert_eq!(tp.entity_rank(10), Some(1));
    assert_eq!(tp.entity_rank(30), Some(3));
    assert_eq!(tp.entity_rank(99), None);
    assert_eq!(tp.entity_at(4), Some(40));
    assert_eq!(tp.entity_at(5), None);
    assert_eq!(tp.entity_at(0), None);
}

#[test]
fn transient_process_transfer_model_all_success() {
    let mut tp = Transfer_TransientProcess::new();
    tp.set_model(vec![1, 2, 3]);
    let count = tp.transfer_model(|key| Ok(Some(key * 100)));
    assert_eq!(count, 3);
    assert_eq!(tp.find_result::<u64>(1).copied(), Some(100));
    assert_eq!(tp.find_result::<u64>(2).copied(), Some(200));
    assert_eq!(tp.find_result::<u64>(3).copied(), Some(300));
    assert_eq!(tp.roots().len(), 3);
}

#[test]
fn transient_process_transfer_model_mixed() {
    let mut tp = Transfer_TransientProcess::new();
    tp.set_model(vec![1, 2, 3]);
    let count = tp.transfer_model(|key| {
        match key {
            1 => Ok(Some(key)),
            2 => Ok(None),          // void
            _ => Err("unsupported".to_string()),
        }
    });
    assert_eq!(count, 1); // only key 1 produced a value
    assert!(tp.find(1).unwrap().is_done());
    assert!(tp.find(2).unwrap().is_void_binder());
    assert!(tp.find(3).unwrap().has_failed());
    // Roots include all three because they were processed (even void/error).
    // Root count: key 1 (ok) and key 2 (void) are registered as roots.
    // Key 3 (error) is NOT a root because the closure returned Err.
    assert_eq!(tp.roots().len(), 2);
}

#[test]
fn transient_process_start_finish_and_cycle_guard() {
    let mut tp = Transfer_TransientProcess::new();
    tp.start_transfer(55u64).unwrap();
    assert_eq!(tp.find(55).unwrap().status(), BinderStatus::Run);
    // Cycle detection.
    let r = tp.start_transfer(55u64);
    assert!(r.is_err());
    // Finish clears the run state.
    tp.finish_transfer(55u64, "done value".to_string()).unwrap();
    assert!(tp.find(55).unwrap().is_done());
}

#[test]
fn transient_process_bind_error_directly() {
    let mut tp = Transfer_TransientProcess::new();
    tp.bind_error(9u64, "unknown entity type");
    let b = tp.find(9).unwrap();
    assert!(b.has_failed());
    assert_eq!(b.error_message(), Some("unknown entity type"));
}

#[test]
fn transient_process_clear_resets_all_state() {
    let mut tp = Transfer_TransientProcess::with_label("STEP reader");
    tp.set_model(vec![1, 2]);
    tp.bind_result(1u64, 42_i32);
    tp.set_root(1).unwrap();
    tp.clear();
    assert!(!tp.is_active());
    assert_eq!(tp.nb_entities(), 0);
    assert_eq!(tp.nb_mapped(),   0);
    assert!(tp.roots().is_empty());
    assert_eq!(tp.label(), "STEP reader"); // label is not cleared
}

#[test]
fn transient_process_add_entity_incremental() {
    let mut tp = Transfer_TransientProcess::new();
    tp.add_entity(10);
    tp.add_entity(20);
    tp.add_entity(10); // duplicate — should not grow
    assert_eq!(tp.nb_entities(), 2);
    assert!(tp.is_active());
}
