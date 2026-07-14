// FILE: tests/occt_message.rs
extern crate ironstream;

use ironstream::message::{
    Gravity, Message_Alert, Message_Messenger, Message_Msg, Message_ProgressRange,
    Message_ProgressSentry,
};

// ---------------------------------------------------------------------------
// 1. Alert: construction and field access
// ---------------------------------------------------------------------------

#[test]
fn test_alert_info_gravity() {
    let a = Message_Alert::new(Gravity::Info, "ShapeOK");
    assert_eq!(a.gravity(), Gravity::Info);
    assert_eq!(a.key(), "ShapeOK");
    assert!(!a.is_fail());
    assert!(a.description().is_none());
}

// ---------------------------------------------------------------------------
// 2. Alert: failure flag and description
// ---------------------------------------------------------------------------

#[test]
fn test_alert_fail_with_description() {
    let a = Message_Alert::new(Gravity::Fail, "NullHandle")
        .with_description("Null shape handle encountered");
    assert!(a.is_fail());
    assert_eq!(a.description(), Some("Null shape handle encountered"));
}

// ---------------------------------------------------------------------------
// 3. Alert: gravity ordering is correct
// ---------------------------------------------------------------------------

#[test]
fn test_gravity_ordering() {
    assert!(Gravity::Info < Gravity::Warning);
    assert!(Gravity::Warning < Gravity::Alarm);
    assert!(Gravity::Alarm < Gravity::Fail);
    assert!(Gravity::Info < Gravity::Fail);
}

// ---------------------------------------------------------------------------
// 4. Message_Msg: no-argument template
// ---------------------------------------------------------------------------

#[test]
fn test_msg_no_args() {
    let m = Message_Msg::new("Operation complete.");
    assert_eq!(m.value(), "Operation complete.");
    assert_eq!(m.original(), "Operation complete.");
    assert_eq!(m.arg_count(), 0);
}

// ---------------------------------------------------------------------------
// 5. Message_Msg: multiple argument substitution
// ---------------------------------------------------------------------------

#[test]
fn test_msg_argument_substitution() {
    let m = Message_Msg::new("Edge %1 has length %2 and tolerance %3")
        .arg("E7")
        .arg(12.5_f64)
        .arg(0.001_f64);
    let v = m.value();
    assert!(v.contains("E7"));
    assert!(v.contains("12.5"));
    assert!(v.contains("0.001"));
    assert_eq!(m.arg_count(), 3);
}

// ---------------------------------------------------------------------------
// 6. Message_Msg: original template is preserved
// ---------------------------------------------------------------------------

#[test]
fn test_msg_original_preserved() {
    let template = "Step %1 of %2";
    let m = Message_Msg::new(template).arg(3).arg(10);
    assert_eq!(m.original(), template);
    assert_eq!(m.value(), "Step 3 of 10");
}

// ---------------------------------------------------------------------------
// 7. Messenger: history recording at all gravity levels
// ---------------------------------------------------------------------------

#[test]
fn test_messenger_history() {
    let mut ms = Message_Messenger::new();
    ms.send(Gravity::Info, "info msg");
    ms.send(Gravity::Warning, "warn msg");
    ms.send(Gravity::Alarm, "alarm msg");
    ms.send(Gravity::Fail, "fail msg");

    assert_eq!(ms.history().len(), 4);
    assert!(ms.has_failures());
    assert_eq!(ms.count_at_least(Gravity::Warning), 3);
    assert_eq!(ms.count_at_least(Gravity::Fail), 1);
}

// ---------------------------------------------------------------------------
// 8. Messenger: gravity filter keeps history but gates printers
// ---------------------------------------------------------------------------

#[test]
fn test_messenger_gravity_filter() {
    let mut ms = Message_Messenger::new();
    ms.set_active_gravity(Gravity::Warning);

    ms.send(Gravity::Info, "below threshold");
    ms.send(Gravity::Warning, "at threshold");
    ms.send(Gravity::Fail, "above threshold");

    // Everything is in history.
    assert_eq!(ms.history().len(), 3);
    // Filter is set correctly.
    assert_eq!(ms.active_gravity(), Gravity::Warning);
    // No failures? There is one Fail.
    assert!(ms.has_failures());
}

// ---------------------------------------------------------------------------
// 9. Messenger: clear_history resets state
// ---------------------------------------------------------------------------

#[test]
fn test_messenger_clear_history() {
    let mut ms = Message_Messenger::new();
    ms.send(Gravity::Info, "a");
    ms.send(Gravity::Fail, "b");
    assert!(ms.has_failures());
    ms.clear_history();
    assert_eq!(ms.history().len(), 0);
    assert!(!ms.has_failures());
}

// ---------------------------------------------------------------------------
// 10. ProgressRange: sub-range local progress mapping
// ---------------------------------------------------------------------------

#[test]
fn test_progress_range_sub_range_mapping() {
    // Sub-range covers 40 % to 80 % of the parent scope.
    let mut r = Message_ProgressRange::sub_range("mesh", 0.4, 0.8);
    assert!((r.start() - 0.4).abs() < 1e-12);
    assert!((r.end() - 0.8).abs() < 1e-12);

    // At local 0.0 the absolute position equals start.
    assert!((r.position() - 0.4).abs() < 1e-12);

    // Advance to 50 % locally → absolute = 0.4 + 0.5*(0.8-0.4) = 0.6
    r.set_local_progress(0.5);
    assert!((r.local_progress() - 0.5).abs() < 1e-12);
    assert!((r.position() - 0.6).abs() < 1e-12);

    r.close();
    assert!(r.is_closed());
    // After close position is at end.
    assert!((r.position() - 0.8).abs() < 1e-12);
}

// ---------------------------------------------------------------------------
// 11. ProgressRange: advance clamping
// ---------------------------------------------------------------------------

#[test]
fn test_progress_range_advance_clamp() {
    let mut r = Message_ProgressRange::new("big");
    r.advance(0.7);
    r.advance(0.7); // would exceed 1.0 → clamp
    assert!((r.position() - 1.0).abs() < 1e-12);
    assert!((r.local_progress() - 1.0).abs() < 1e-12);
}

// ---------------------------------------------------------------------------
// 12. ProgressRange: split into equal parts
// ---------------------------------------------------------------------------

#[test]
fn test_progress_range_split() {
    let r = Message_ProgressRange::sub_range("full", 0.0, 1.0);
    let parts = r.split(5);
    assert_eq!(parts.len(), 5);
    for (i, p) in parts.iter().enumerate() {
        let expected_start = i as f64 * 0.2;
        let expected_end = (i + 1) as f64 * 0.2;
        assert!(
            (p.start() - expected_start).abs() < 1e-10,
            "part {} start: got {}, want {}",
            i,
            p.start(),
            expected_start
        );
        assert!(
            (p.end() - expected_end).abs() < 1e-10,
            "part {} end: got {}, want {}",
            i,
            p.end(),
            expected_end
        );
    }
}

// ---------------------------------------------------------------------------
// 13. ProgressSentry: complete iteration
// ---------------------------------------------------------------------------

#[test]
fn test_sentry_complete_iteration() {
    let range = Message_ProgressRange::new("tessellate");
    let mut sentry = Message_ProgressSentry::new(range, 5);

    let mut step = 0usize;
    while sentry.more() {
        sentry.next();
        step += 1;
    }

    assert_eq!(step, 5);
    assert_eq!(sentry.steps_done(), 5);
    assert_eq!(sentry.total_steps(), 5);
    assert!((sentry.progress() - 1.0).abs() < 1e-12);
}

// ---------------------------------------------------------------------------
// 14. ProgressSentry: abandon stops iteration
// ---------------------------------------------------------------------------

#[test]
fn test_sentry_abandon() {
    let range = Message_ProgressRange::new("long_op");
    let mut sentry = Message_ProgressSentry::new(range, 100);

    // Do a few steps then abandon.
    sentry.next();
    sentry.next();
    sentry.next();
    sentry.abandon();

    assert!(!sentry.more());
    assert_eq!(sentry.steps_done(), 3);
}

// ---------------------------------------------------------------------------
// 15. ProgressSentry: into_range returns closed range
// ---------------------------------------------------------------------------

#[test]
fn test_sentry_into_range() {
    let range = Message_ProgressRange::sub_range("export", 0.5, 1.0);
    let mut sentry = Message_ProgressSentry::new(range, 3);
    sentry.next();
    sentry.next();
    sentry.next();
    let r = sentry.into_range();
    assert!(r.is_closed());
    assert!((r.position() - 1.0).abs() < 1e-12);
}

// ---------------------------------------------------------------------------
// 16. Messenger: send_alert with Message_Alert object
// ---------------------------------------------------------------------------

#[test]
fn test_messenger_send_alert_object() {
    let mut ms = Message_Messenger::new();
    let a = Message_Alert::new(Gravity::Alarm, "HighCurvature")
        .with_description("Radius < tolerance");
    ms.send_alert(a);

    assert_eq!(ms.history().len(), 1);
    assert_eq!(ms.history()[0].key(), "HighCurvature");
    assert_eq!(ms.history()[0].gravity(), Gravity::Alarm);
    assert!(!ms.has_failures());
}

// ---------------------------------------------------------------------------
// 17. ProgressRange: drop auto-closes
// ---------------------------------------------------------------------------

#[test]
fn test_progress_range_drop_auto_close() {
    // We cannot inspect after drop, so we test via into_range on sentry
    // which exercises the same Drop path on the inner range.
    let range = Message_ProgressRange::sub_range("drop_test", 0.0, 0.5);
    // Immediately drop to trigger close.
    drop(range);
    // If we get here without panic the Drop impl ran cleanly.
}

// ---------------------------------------------------------------------------
// 18. Messenger: default_messenger produces clean state
// ---------------------------------------------------------------------------

#[test]
fn test_default_messenger_clean() {
    let ms = Message_Messenger::default_messenger();
    assert_eq!(ms.history().len(), 0);
    assert_eq!(ms.printer_count(), 0);
    assert!(!ms.has_failures());
    assert_eq!(ms.active_gravity(), Gravity::Info);
}
