// FILE: tests/occt_osd_sys.rs
extern crate ironstream;
use ironstream::osd_sys::*;

#[test]
fn chrono_start_stop() {
    let mut c = OsdChronometer::new();
    c.start();
    c.advance_tick(1.5);
    c.stop();
    assert!((c.elapsed_time() - 1.5).abs() < 1e-10);
    assert!(!c.is_started);
}

#[test]
fn chrono_reset() {
    let mut c = OsdChronometer::new();
    c.start();
    c.advance_tick(2.0);
    c.stop();
    c.reset();
    assert!(c.elapsed_time().abs() < 1e-10);
}

#[test]
fn timer_wall_clock() {
    let mut t = OsdTimer::new();
    t.start();
    t.advance_tick(3.0);
    t.stop();
    assert!((t.elapsed_time() - 3.0).abs() < 1e-10);
    assert!((t.wall_clock_time() - 3.0).abs() < 1e-10);
    assert!(!t.is_started());
}

#[test]
fn timer_show() {
    let mut t = OsdTimer::new();
    t.start();
    t.advance_tick(0.5);
    t.stop();
    let s = t.show();
    assert!(s.contains("0.500s"));
}

#[test]
fn mem_info_defaults() {
    let m = OsdMemInfo::new();
    assert!(m.virtual_mem() > 0.0);
    assert!(m.working_set() > 0.0);
    let s = m.to_string();
    assert!(s.contains("WorkSet"));
}
