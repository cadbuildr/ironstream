use ironstream::message_exec::*;

#[test]
fn exec_status_bits() {
    let mut s = MessageExecStatus::new();
    s.set_done();
    assert!(s.is_done());
    assert!(!s.is_fail());
    assert!(s.is_ok());
    s.set_fail();
    assert!(!s.is_ok());
    s.clear_fail();
    assert!(s.is_ok());
}

#[test]
fn exec_status_warn_alarm() {
    let mut s = MessageExecStatus::new();
    s.set_warn();
    s.set_alarm();
    assert!(s.has_warnings());
    assert!(s.has_alarms());
    assert_eq!(s.bits(), MessageExecStatus::WARN | MessageExecStatus::ALARM);
}

#[test]
fn progress_range_fraction() {
    let mut r = MessageProgressRange::new(0.0, 10.0);
    r.advance(5.0);
    assert!((r.fraction() - 0.5).abs() < 1e-10);
    assert!(!r.is_complete());
    r.advance(5.0);
    assert!(r.is_complete());
    assert!((r.percent() - 100.0).abs() < 1e-10);
}

#[test]
fn progress_scope_steps() {
    let mut s = MessageProgressScope::new("test", 4);
    assert_eq!(s.steps_left(), 4);
    s.next();
    s.next();
    assert_eq!(s.steps_done(), 2);
    assert!((s.fraction() - 0.5).abs() < 1e-10);
    assert!(!s.is_complete());
    s.next(); s.next();
    assert!(s.is_complete());
}

#[test]
fn indicator_total_fraction() {
    let mut ind = MessageProgressIndicator::new();
    let i = ind.add_scope("load", 2);
    ind.next_step(i);
    assert!((ind.total_fraction() - 0.5).abs() < 1e-10);
    ind.next_step(i);
    assert!(ind.is_complete());
}

#[test]
fn exec_status_add() {
    let mut a = MessageExecStatus::new();
    a.set_done();
    let mut b = MessageExecStatus::new();
    b.set_warn();
    a.add(b);
    assert!(a.is_done());
    assert!(a.has_warnings());
}
