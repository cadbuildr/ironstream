// FILE: tests/occt_interface_check.rs
//! Integration tests for `ironstream::interface_check` — the validation
//! message types ported from OpenCascade's `Interface_Check` and
//! `Interface_CheckIterator`.
//!
//! Tests exercise the public API from an external-crate perspective, mirroring
//! the way a STEP/IGES reader or shape analyser would collect and inspect
//! validation messages per entity.

extern crate ironstream;

use ironstream::interface_check::*;

// ---------------------------------------------------------------------------
// CheckMessageType
// ---------------------------------------------------------------------------

#[test]
fn check_message_type_variants_are_distinct() {
    assert_ne!(CheckMessageType::Info,    CheckMessageType::Warning);
    assert_ne!(CheckMessageType::Warning, CheckMessageType::Fail);
    assert_ne!(CheckMessageType::Info,    CheckMessageType::Fail);
}

#[test]
fn check_message_type_is_copy() {
    let t = CheckMessageType::Fail;
    let t2 = t; // Copy — original is still usable
    assert_eq!(t, t2);
}

#[test]
fn check_message_type_clone_eq() {
    let t = CheckMessageType::Warning;
    assert_eq!(t.clone(), t);
}

// ---------------------------------------------------------------------------
// CheckMessage
// ---------------------------------------------------------------------------

#[test]
fn check_message_new_stores_all_fields() {
    let m = CheckMessage::new(CheckMessageType::Fail, "degenerate wire", 12);
    assert_eq!(m.msg_type(),  CheckMessageType::Fail);
    assert_eq!(m.text(),      "degenerate wire");
    assert_eq!(m.entity_id(), 12);
}

#[test]
fn check_message_new_info_type() {
    let m = CheckMessage::new(CheckMessageType::Info, "processed ok", 0);
    assert_eq!(m.msg_type(), CheckMessageType::Info);
}

#[test]
fn check_message_new_warning_type() {
    let m = CheckMessage::new(CheckMessageType::Warning, "tolerance exceeded", 3);
    assert_eq!(m.msg_type(), CheckMessageType::Warning);
    assert_eq!(m.text(),     "tolerance exceeded");
}

#[test]
fn check_message_clone_is_independent() {
    let m  = CheckMessage::new(CheckMessageType::Info, "original", 9);
    let m2 = m.clone();
    assert_eq!(m2.text(),      "original");
    assert_eq!(m2.entity_id(), 9);
    assert_eq!(m2.msg_type(),  CheckMessageType::Info);
}

#[test]
fn check_message_empty_text() {
    let m = CheckMessage::new(CheckMessageType::Warning, "", 5);
    assert_eq!(m.text(), "");
}

// ---------------------------------------------------------------------------
// InterfaceCheck — construction & entity_id
// ---------------------------------------------------------------------------

#[test]
fn interface_check_new_is_empty() {
    let c = InterfaceCheck::new(0);
    assert_eq!(c.entity_id(),   0);
    assert_eq!(c.nb_messages(), 0);
    assert_eq!(c.nb_warnings(), 0);
    assert_eq!(c.nb_fails(),    0);
    assert!(c.is_ok());
}

#[test]
fn interface_check_entity_id_preserved() {
    let c = InterfaceCheck::new(99);
    assert_eq!(c.entity_id(), 99);
}

// ---------------------------------------------------------------------------
// InterfaceCheck — add_warning
// ---------------------------------------------------------------------------

#[test]
fn interface_check_add_warning_is_still_ok() {
    let mut c = InterfaceCheck::new(1);
    c.add_warning("edge gap too large");
    assert!(c.is_ok());
    assert_eq!(c.nb_warnings(), 1);
    assert_eq!(c.nb_fails(),    0);
    assert_eq!(c.nb_messages(), 1);
}

#[test]
fn interface_check_warning_message_fields() {
    let mut c = InterfaceCheck::new(4);
    c.add_warning("self-intersecting face");
    let m = c.message(0);
    assert_eq!(m.msg_type(),  CheckMessageType::Warning);
    assert_eq!(m.text(),      "self-intersecting face");
    assert_eq!(m.entity_id(), 4);
}

#[test]
fn interface_check_multiple_warnings() {
    let mut c = InterfaceCheck::new(2);
    c.add_warning("first warning");
    c.add_warning("second warning");
    assert_eq!(c.nb_warnings(), 2);
    assert_eq!(c.nb_messages(), 2);
    assert!(c.is_ok());
}

// ---------------------------------------------------------------------------
// InterfaceCheck — add_fail
// ---------------------------------------------------------------------------

#[test]
fn interface_check_add_fail_makes_not_ok() {
    let mut c = InterfaceCheck::new(7);
    c.add_fail("missing geometry");
    assert!(!c.is_ok());
    assert_eq!(c.nb_fails(),    1);
    assert_eq!(c.nb_warnings(), 0);
    assert_eq!(c.nb_messages(), 1);
}

#[test]
fn interface_check_fail_message_fields() {
    let mut c = InterfaceCheck::new(8);
    c.add_fail("topology inconsistency");
    let m = c.message(0);
    assert_eq!(m.msg_type(),  CheckMessageType::Fail);
    assert_eq!(m.text(),      "topology inconsistency");
    assert_eq!(m.entity_id(), 8);
}

#[test]
fn interface_check_multiple_fails() {
    let mut c = InterfaceCheck::new(0);
    c.add_fail("fail A");
    c.add_fail("fail B");
    assert_eq!(c.nb_fails(),    2);
    assert_eq!(c.nb_messages(), 2);
    assert!(!c.is_ok());
}

// ---------------------------------------------------------------------------
// InterfaceCheck — add_info
// ---------------------------------------------------------------------------

#[test]
fn interface_check_add_info_is_ok() {
    let mut c = InterfaceCheck::new(6);
    c.add_info("entity mapped to shell");
    assert!(c.is_ok());
    assert_eq!(c.nb_messages(), 1);
    let m = c.message(0);
    assert_eq!(m.msg_type(), CheckMessageType::Info);
    assert_eq!(m.text(),     "entity mapped to shell");
}

// ---------------------------------------------------------------------------
// InterfaceCheck — mixed messages
// ---------------------------------------------------------------------------

#[test]
fn interface_check_mixed_preserves_order() {
    let mut c = InterfaceCheck::new(20);
    c.add_info("step 1");
    c.add_warning("step 2");
    c.add_fail("step 3");
    c.add_info("step 4");

    assert_eq!(c.nb_messages(), 4);
    assert_eq!(c.message(0).text(), "step 1");
    assert_eq!(c.message(1).text(), "step 2");
    assert_eq!(c.message(2).text(), "step 3");
    assert_eq!(c.message(3).text(), "step 4");
}

#[test]
fn interface_check_mixed_counts() {
    let mut c = InterfaceCheck::new(0);
    c.add_warning("w1");
    c.add_warning("w2");
    c.add_fail("f1");
    c.add_info("i1");
    c.add_info("i2");
    c.add_info("i3");

    assert_eq!(c.nb_warnings(), 2);
    assert_eq!(c.nb_fails(),    1);
    assert_eq!(c.nb_messages(), 6);
    assert!(!c.is_ok());
}

#[test]
fn interface_check_warning_only_not_counted_as_fail() {
    let mut c = InterfaceCheck::new(0);
    c.add_warning("w");
    assert_eq!(c.nb_fails(), 0);
    assert!(c.is_ok());
}

// ---------------------------------------------------------------------------
// InterfaceCheckList — construction & empty
// ---------------------------------------------------------------------------

#[test]
fn interface_check_list_new_is_empty() {
    let list = InterfaceCheckList::new();
    assert_eq!(list.nb_checks(),       0);
    assert_eq!(list.nb_fails_total(),   0);
    assert_eq!(list.nb_warnings_total(), 0);
    assert!(!list.has_fails());
}

#[test]
fn interface_check_list_default_is_empty() {
    let list: InterfaceCheckList = Default::default();
    assert_eq!(list.nb_checks(), 0);
}

// ---------------------------------------------------------------------------
// InterfaceCheckList — add & check
// ---------------------------------------------------------------------------

#[test]
fn interface_check_list_add_single() {
    let mut list = InterfaceCheckList::new();
    let mut c = InterfaceCheck::new(1);
    c.add_info("loaded");
    list.add(c);
    assert_eq!(list.nb_checks(), 1);
    assert_eq!(list.check(0).entity_id(), 1);
}

#[test]
fn interface_check_list_add_multiple_order_preserved() {
    let mut list = InterfaceCheckList::new();
    list.add(InterfaceCheck::new(10));
    list.add(InterfaceCheck::new(20));
    list.add(InterfaceCheck::new(30));
    assert_eq!(list.nb_checks(), 3);
    assert_eq!(list.check(0).entity_id(), 10);
    assert_eq!(list.check(1).entity_id(), 20);
    assert_eq!(list.check(2).entity_id(), 30);
}

// ---------------------------------------------------------------------------
// InterfaceCheckList — nb_fails_total
// ---------------------------------------------------------------------------

#[test]
fn interface_check_list_nb_fails_total_sums_across_checks() {
    let mut list = InterfaceCheckList::new();

    let mut c1 = InterfaceCheck::new(1);
    c1.add_fail("f1");
    c1.add_fail("f2");
    list.add(c1);

    let mut c2 = InterfaceCheck::new(2);
    c2.add_fail("f3");
    list.add(c2);

    let c3 = InterfaceCheck::new(3); // no fails
    list.add(c3);

    assert_eq!(list.nb_fails_total(), 3);
}

#[test]
fn interface_check_list_nb_fails_total_zero_when_only_warnings() {
    let mut list = InterfaceCheckList::new();
    let mut c = InterfaceCheck::new(1);
    c.add_warning("w");
    list.add(c);
    assert_eq!(list.nb_fails_total(), 0);
}

// ---------------------------------------------------------------------------
// InterfaceCheckList — nb_warnings_total
// ---------------------------------------------------------------------------

#[test]
fn interface_check_list_nb_warnings_total_sums_across_checks() {
    let mut list = InterfaceCheckList::new();

    let mut c1 = InterfaceCheck::new(1);
    c1.add_warning("w1");
    c1.add_warning("w2");
    list.add(c1);

    let mut c2 = InterfaceCheck::new(2);
    c2.add_warning("w3");
    list.add(c2);

    assert_eq!(list.nb_warnings_total(), 3);
}

#[test]
fn interface_check_list_nb_warnings_total_zero_when_only_fails() {
    let mut list = InterfaceCheckList::new();
    let mut c = InterfaceCheck::new(1);
    c.add_fail("f");
    list.add(c);
    assert_eq!(list.nb_warnings_total(), 0);
}

// ---------------------------------------------------------------------------
// InterfaceCheckList — has_fails
// ---------------------------------------------------------------------------

#[test]
fn interface_check_list_has_fails_false_when_no_fails() {
    let mut list = InterfaceCheckList::new();
    let mut c = InterfaceCheck::new(1);
    c.add_warning("w");
    c.add_info("i");
    list.add(c);
    assert!(!list.has_fails());
}

#[test]
fn interface_check_list_has_fails_true_when_any_check_has_fail() {
    let mut list = InterfaceCheckList::new();

    let mut c1 = InterfaceCheck::new(1);
    c1.add_warning("w");
    list.add(c1); // no fails

    let mut c2 = InterfaceCheck::new(2);
    c2.add_fail("bad");
    list.add(c2); // has fail

    assert!(list.has_fails());
}

#[test]
fn interface_check_list_has_fails_true_when_first_check_fails() {
    let mut list = InterfaceCheckList::new();
    let mut c = InterfaceCheck::new(0);
    c.add_fail("immediate fail");
    list.add(c);
    assert!(list.has_fails());
}

// ---------------------------------------------------------------------------
// End-to-end scenario: STEP-like transfer validation
// ---------------------------------------------------------------------------

#[test]
fn step_like_validation_scenario() {
    // Simulate validating three entities from a STEP file.
    let mut list = InterfaceCheckList::new();

    // Entity 1: transferred cleanly.
    let mut c1 = InterfaceCheck::new(1);
    c1.add_info("B-Rep solid mapped");
    list.add(c1);

    // Entity 2: transferred with approximate geometry.
    let mut c2 = InterfaceCheck::new(2);
    c2.add_warning("curve approximated within tolerance");
    list.add(c2);

    // Entity 3: failed to transfer.
    let mut c3 = InterfaceCheck::new(3);
    c3.add_fail("unsupported STEP entity type");
    c3.add_warning("skipped sub-shape");
    list.add(c3);

    assert_eq!(list.nb_checks(),          3);
    assert_eq!(list.nb_fails_total(),     1);
    assert_eq!(list.nb_warnings_total(),  2);
    assert!(list.has_fails());

    // Only entity 3 is not OK.
    assert!( list.check(0).is_ok());
    assert!( list.check(1).is_ok());
    assert!(!list.check(2).is_ok());

    // Inspect the fail on entity 3.
    let fail_msg = list.check(2).message(0);
    assert_eq!(fail_msg.msg_type(),  CheckMessageType::Fail);
    assert_eq!(fail_msg.entity_id(), 3);
    assert_eq!(fail_msg.text(),      "unsupported STEP entity type");
}
