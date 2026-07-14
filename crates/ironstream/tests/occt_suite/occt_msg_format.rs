use ironstream::msg_format::*;

#[test]
fn message_gravity_labels() {
    assert_eq!(MessageGravity::Trace.label(), "TRACE");
    assert_eq!(MessageGravity::Warning.label(), "WARNING");
    assert_eq!(MessageGravity::Fail.label(), "FAIL");
}

#[test]
fn message_alert_fail_threshold() {
    let alarm = MessageAlert::new(MessageGravity::Alarm, "system alarm");
    assert!(alarm.is_fail());
    let info = MessageAlert::new(MessageGravity::Info, "all good");
    assert!(!info.is_fail());
}

#[test]
fn message_formatter_multiple_args() {
    let msg = MessageFormatter::new("(%1, %2, %3)")
        .arg("x")
        .arg("y")
        .arg("z")
        .format();
    assert_eq!(msg, "(x, y, z)");
}

#[test]
fn message_printer_prefix() {
    let mut p = MessagePrinter::with_prefix("KERNEL");
    p.print(MessageGravity::Warning, "tol relaxed");
    let last = p.last_message().unwrap();
    assert!(last.contains("KERNEL"));
    assert!(last.contains("WARNING"));
}

#[test]
fn message_report_no_failures_when_empty() {
    let r = MessageReport::new();
    assert!(!r.has_failures());
    assert!(r.is_empty());
    assert_eq!(r.nb_alerts(), 0);
}

#[test]
fn message_report_clear_and_reuse() {
    let mut r = MessageReport::new();
    r.add(MessageGravity::Fail, "error");
    assert!(r.has_failures());
    r.clear();
    assert!(!r.has_failures());
    assert!(r.is_empty());
    r.add(MessageGravity::Info, "retry ok");
    assert!(!r.has_failures());
    assert_eq!(r.nb_alerts(), 1);
}
