// FILE: tests/occt_message_report.rs
extern crate ironstream;

use ironstream::message_report::{
    MessageAlert, MessageGravity, MessageProgress, MessageReport,
};

#[test]
fn gravity_values_and_order() {
    assert_eq!(MessageGravity::Trace as u8, 0);
    assert_eq!(MessageGravity::Info as u8, 1);
    assert_eq!(MessageGravity::Warning as u8, 2);
    assert_eq!(MessageGravity::Alarm as u8, 3);
    assert_eq!(MessageGravity::Fail as u8, 4);
    assert!(MessageGravity::Trace < MessageGravity::Fail);
}

#[test]
fn alert_is_error_boundary() {
    assert!(!MessageAlert::new(MessageGravity::Warning, "w").is_error());
    assert!(MessageAlert::new(MessageGravity::Alarm, "a").is_error());
    assert!(MessageAlert::new(MessageGravity::Fail, "f").is_error());
}

#[test]
fn alert_location_builder() {
    let a = MessageAlert::new(MessageGravity::Trace, "debug info")
        .with_location("module/file.rs:10");
    assert_eq!(a.location.as_deref(), Some("module/file.rs:10"));
    assert_eq!(a.message, "debug info");
    assert_eq!(a.gravity, MessageGravity::Trace);
}

#[test]
fn report_default_records_all_gravities() {
    let mut r = MessageReport::new();
    for g in [
        MessageGravity::Trace,
        MessageGravity::Info,
        MessageGravity::Warning,
        MessageGravity::Alarm,
        MessageGravity::Fail,
    ] {
        r.add_alert(MessageAlert::new(g, "x"));
    }
    assert_eq!(r.nb_alerts(), 5);
}

#[test]
fn report_nb_alerts_of_each_gravity() {
    let mut r = MessageReport::new();
    r.add_alert(MessageAlert::new(MessageGravity::Info, "a"));
    r.add_alert(MessageAlert::new(MessageGravity::Info, "b"));
    r.add_alert(MessageAlert::new(MessageGravity::Warning, "c"));
    assert_eq!(r.nb_alerts_of(MessageGravity::Info), 2);
    assert_eq!(r.nb_alerts_of(MessageGravity::Warning), 1);
    assert_eq!(r.nb_alerts_of(MessageGravity::Trace), 0);
    assert_eq!(r.nb_alerts_of(MessageGravity::Fail), 0);
}

#[test]
fn report_has_alerts_of() {
    let mut r = MessageReport::new();
    assert!(!r.has_alerts_of(MessageGravity::Warning));
    r.add_alert(MessageAlert::new(MessageGravity::Warning, "w"));
    assert!(r.has_alerts_of(MessageGravity::Warning));
    assert!(!r.has_alerts_of(MessageGravity::Fail));
}

#[test]
fn report_has_warnings_and_errors_independent() {
    let mut r = MessageReport::new();
    r.add_alert(MessageAlert::new(MessageGravity::Warning, "w"));
    assert!(r.has_warnings());
    assert!(!r.has_errors());
    r.add_alert(MessageAlert::new(MessageGravity::Fail, "f"));
    assert!(r.has_warnings());
    assert!(r.has_errors());
}

#[test]
fn report_set_min_gravity_filters_below() {
    let mut r = MessageReport::new();
    r.set_min_gravity(MessageGravity::Alarm);
    r.add_alert(MessageAlert::new(MessageGravity::Trace, "t"));
    r.add_alert(MessageAlert::new(MessageGravity::Info, "i"));
    r.add_alert(MessageAlert::new(MessageGravity::Warning, "w"));
    r.add_alert(MessageAlert::new(MessageGravity::Alarm, "a"));
    r.add_alert(MessageAlert::new(MessageGravity::Fail, "f"));
    assert_eq!(r.nb_alerts(), 2);
    assert!(r.has_alerts_of(MessageGravity::Alarm));
    assert!(r.has_alerts_of(MessageGravity::Fail));
}

#[test]
fn report_clear_empties_all() {
    let mut r = MessageReport::new();
    r.add_alert(MessageAlert::new(MessageGravity::Info, "i"));
    r.add_alert(MessageAlert::new(MessageGravity::Fail, "f"));
    assert_eq!(r.nb_alerts(), 2);
    r.clear();
    assert_eq!(r.nb_alerts(), 0);
    assert!(!r.has_errors());
    assert!(!r.has_warnings());
}

#[test]
fn report_dump_contains_gravity_labels() {
    let mut r = MessageReport::new();
    r.add_alert(MessageAlert::new(MessageGravity::Warning, "watch out"));
    r.add_alert(MessageAlert::new(MessageGravity::Fail, "fatal"));
    let d = r.dump();
    assert!(d.contains("[WARNING] watch out"));
    assert!(d.contains("[FAIL] fatal"));
}

#[test]
fn report_dump_each_alert_on_own_line() {
    let mut r = MessageReport::new();
    r.add_alert(MessageAlert::new(MessageGravity::Info, "first"));
    r.add_alert(MessageAlert::new(MessageGravity::Info, "second"));
    let d = r.dump();
    let lines: Vec<&str> = d.lines().collect();
    assert_eq!(lines.len(), 2);
}

#[test]
fn progress_increment_and_percent() {
    let mut p = MessageProgress::new(200.0, "export");
    p.increment(50.0);
    assert_eq!(p.current(), 50.0);
    assert_eq!(p.percent(), 25.0);
    p.increment(150.0);
    assert!(p.is_complete());
    assert_eq!(p.percent(), 100.0);
}

#[test]
fn progress_set_current_directly() {
    let mut p = MessageProgress::new(100.0, "step");
    p.set_current(75.0);
    assert_eq!(p.current(), 75.0);
    assert!(!p.is_complete());
    p.set_current(100.0);
    assert!(p.is_complete());
}

#[test]
fn progress_percent_does_not_exceed_100() {
    let mut p = MessageProgress::new(10.0, "t");
    p.set_current(1000.0);
    assert_eq!(p.percent(), 100.0);
}

#[test]
fn progress_abort_flag() {
    let mut p = MessageProgress::new(100.0, "job");
    assert!(!p.is_aborted());
    p.abort();
    assert!(p.is_aborted());
    assert!(!p.is_complete());
}
