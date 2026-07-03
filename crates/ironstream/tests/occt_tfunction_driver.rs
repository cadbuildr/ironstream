use ironstream::tfunction_driver::*;

#[test]
fn logbook_touched_impacted() {
    let mut log = TFunctionLogbook::new();
    log.set_touched(10);
    log.set_impacted(20);
    assert!(log.is_touched(10));
    assert!(!log.is_touched(20));
    assert!(log.is_impacted(20));
    // Duplicates should not grow the vec.
    log.set_touched(10);
    assert_eq!(log.touched.len(), 1);
}

#[test]
fn logbook_valid_and_clear() {
    let mut log = TFunctionLogbook::new();
    log.add_valid(5);
    assert!(log.is_valid(5));
    assert!(!log.is_valid(6));
    log.set_done(true);
    assert!(log.is_done());
    log.clear();
    assert!(!log.is_done());
    assert!(!log.is_valid(5));
}

#[test]
fn driver_execute() {
    let mut d = TFunctionDriver::new(1);
    d.add_result(100);
    d.add_result(101);
    let mut log = TFunctionLogbook::new();
    let rc = d.execute(&mut log);
    assert_eq!(rc, 0);
    assert!(log.is_done());
    assert!(log.is_valid(100));
    assert_eq!(d.exec_status, TFunctionExecStatus::ExecutionUpToDate);
}

#[test]
fn driver_must_execute() {
    let mut d = TFunctionDriver::new(2);
    d.add_argument(5);
    let mut log = TFunctionLogbook::new();
    assert!(d.must_execute(&log)); // NotExecuted
    d.exec_status = TFunctionExecStatus::ExecutionUpToDate;
    assert!(!d.must_execute(&log));
    log.set_touched(5);
    assert!(d.must_execute(&log)); // argument touched
}

#[test]
fn driver_table_find() {
    let mut t = TFunctionDriverTable::new();
    t.add(1, "MyDriver");
    t.add(2, "OtherDriver");
    assert_eq!(t.find(1), Some("MyDriver"));
    assert!(t.has_driver(2));
    assert!(!t.has_driver(99));
    assert_eq!(t.nb_drivers(), 2);
}

#[test]
fn ifunction_lifecycle() {
    let mut f = TFunctionIFunction::new(10, 1);
    assert!(!f.is_up_to_date());
    f.mark_up_to_date();
    assert!(f.is_up_to_date());
    f.invalidate();
    assert!(!f.is_up_to_date());
    assert_eq!(f.status, TFunctionExecStatus::ExecutionTooOld);
}
