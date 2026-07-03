// FILE: rust/ironstream/crates/ironstream/tests/occt_message_progress.rs
extern crate ironstream;
use ironstream::message_progress::*;

// --- MessageProgressState ---

#[test]
fn state_initial_not_started() {
    let r = MessageProgressRange::new("init", 0.0, 100.0, 10);
    assert_eq!(r.state(), MessageProgressState::NotStarted);
}

#[test]
fn state_start_transitions_to_in_progress() {
    let mut r = MessageProgressRange::new("work", 0.0, 100.0, 10);
    r.start();
    assert_eq!(r.state(), MessageProgressState::InProgress);
}

#[test]
fn state_complete_transitions_to_completed() {
    let mut r = MessageProgressRange::new("work", 0.0, 100.0, 10);
    r.start();
    r.complete();
    assert_eq!(r.state(), MessageProgressState::Completed);
}

#[test]
fn state_abort_transitions_to_aborted() {
    let mut r = MessageProgressRange::new("work", 0.0, 100.0, 10);
    r.start();
    r.abort();
    assert_eq!(r.state(), MessageProgressState::Aborted);
}

// --- MessageProgressRange ---

#[test]
fn range_name() {
    let r = MessageProgressRange::new("my_range", 0.0, 50.0, 5);
    assert_eq!(r.name(), "my_range");
}

#[test]
fn range_progress_starts_at_zero() {
    let r = MessageProgressRange::new("r", 0.0, 100.0, 10);
    assert!((r.progress() - 0.0).abs() < 1e-12);
}

#[test]
fn range_advance_increases_progress() {
    let mut r = MessageProgressRange::new("r", 0.0, 100.0, 4);
    r.advance(); // step_size = 25.0 => current = 25.0
    let p = r.progress();
    assert!((p - 0.25).abs() < 1e-12, "expected 0.25, got {p}");
}

#[test]
fn range_progress_clamped_at_one_when_over_max() {
    let mut r = MessageProgressRange::new("r", 0.0, 10.0, 2);
    // advance 5 times => current = 25.0 > max=10.0
    for _ in 0..5 {
        r.advance();
    }
    assert!((r.progress() - 1.0).abs() < 1e-12);
}

#[test]
fn range_step_size_computed_correctly() {
    // step_size = (max - min) / nb_steps = 100/4 = 25
    let mut r = MessageProgressRange::new("r", 0.0, 100.0, 4);
    r.advance();
    r.advance();
    let p = r.progress(); // current = 50.0 => 0.5
    assert!((p - 0.5).abs() < 1e-12, "expected 0.5, got {p}");
}

#[test]
fn range_progress_with_nonzero_min() {
    let mut r = MessageProgressRange::new("r", 10.0, 20.0, 2);
    // step_size = (20-10)/2 = 5; advance once => current = 15 => progress = 0.5
    r.advance();
    let p = r.progress();
    assert!((p - 0.5).abs() < 1e-12, "expected 0.5, got {p}");
}

#[test]
fn range_is_done_false_initially() {
    let r = MessageProgressRange::new("r", 0.0, 1.0, 1);
    assert!(!r.is_done());
}

#[test]
fn range_is_done_true_after_complete() {
    let mut r = MessageProgressRange::new("r", 0.0, 1.0, 1);
    r.complete();
    assert!(r.is_done());
}

#[test]
fn range_is_done_true_after_abort() {
    let mut r = MessageProgressRange::new("r", 0.0, 1.0, 1);
    r.abort();
    assert!(r.is_done());
}

#[test]
fn range_is_done_false_while_in_progress() {
    let mut r = MessageProgressRange::new("r", 0.0, 1.0, 1);
    r.start();
    assert!(!r.is_done());
}

#[test]
fn range_zero_span_progress_is_one() {
    let r = MessageProgressRange::new("zero", 5.0, 5.0, 0);
    assert!((r.progress() - 1.0).abs() < 1e-12);
}

// --- MessageProgressScope ---

#[test]
fn scope_name() {
    let s = MessageProgressScope::new("my_scope");
    assert_eq!(s.name(), "my_scope");
}

#[test]
fn scope_empty_nb_ranges() {
    let s = MessageProgressScope::new("s");
    assert_eq!(s.nb_ranges(), 0);
}

#[test]
fn scope_overall_progress_empty_is_zero() {
    let s = MessageProgressScope::new("s");
    assert!((s.overall_progress() - 0.0).abs() < 1e-12);
}

#[test]
fn scope_add_range_increments_nb_ranges() {
    let mut s = MessageProgressScope::new("s");
    s.add_range(MessageProgressRange::new("r1", 0.0, 10.0, 10), 1.0);
    s.add_range(MessageProgressRange::new("r2", 0.0, 10.0, 10), 2.0);
    assert_eq!(s.nb_ranges(), 2);
}

#[test]
fn scope_overall_progress_single_range() {
    let mut s = MessageProgressScope::new("s");
    let mut r = MessageProgressRange::new("r", 0.0, 100.0, 4);
    r.advance(); // progress = 0.25
    s.add_range(r, 1.0);
    let p = s.overall_progress();
    assert!((p - 0.25).abs() < 1e-12, "expected 0.25, got {p}");
}

#[test]
fn scope_overall_progress_equal_weights() {
    let mut s = MessageProgressScope::new("s");
    // r1: progress = 0.5, r2: progress = 1.0, equal weight => average = 0.75
    let mut r1 = MessageProgressRange::new("r1", 0.0, 10.0, 2);
    r1.advance(); // current=5 => 0.5
    let mut r2 = MessageProgressRange::new("r2", 0.0, 10.0, 2);
    r2.advance();
    r2.advance(); // current=10 => 1.0
    s.add_range(r1, 1.0);
    s.add_range(r2, 1.0);
    let p = s.overall_progress();
    assert!((p - 0.75).abs() < 1e-12, "expected 0.75, got {p}");
}

#[test]
fn scope_overall_progress_unequal_weights() {
    let mut s = MessageProgressScope::new("s");
    // r1: progress=0.0, weight=1; r2: progress=1.0, weight=3
    // weighted avg = (0.0*1 + 1.0*3) / 4 = 0.75
    let r1 = MessageProgressRange::new("r1", 0.0, 10.0, 1);
    let mut r2 = MessageProgressRange::new("r2", 0.0, 10.0, 1);
    r2.advance();
    s.add_range(r1, 1.0);
    s.add_range(r2, 3.0);
    let p = s.overall_progress();
    assert!((p - 0.75).abs() < 1e-12, "expected 0.75, got {p}");
}

#[test]
fn scope_total_weight_accumulates() {
    let mut s = MessageProgressScope::new("s");
    s.add_range(MessageProgressRange::new("a", 0.0, 1.0, 1), 2.0);
    s.add_range(MessageProgressRange::new("b", 0.0, 1.0, 1), 3.0);
    // total_weight = 5.0; both at 0 => overall = 0.0
    assert!((s.overall_progress() - 0.0).abs() < 1e-12);
}
