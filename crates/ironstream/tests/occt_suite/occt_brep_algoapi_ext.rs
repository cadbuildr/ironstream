// FILE: tests/occt_brep_algoapi_ext.rs
extern crate ironstream;

use ironstream::brep_algoapi_ext::{
    AlgoApiStatus, ShapeDefect, BrepAlgoApiCheck, BrepAlgoApiDefeaturing, BopAlgoOptions,
};

#[test]
fn check_valid_shape_no_defects() {
    let mut c = BrepAlgoApiCheck::new(42);
    c.perform();
    assert!(c.is_valid());
    assert_eq!(c.nb_defects(), 0);
    assert!(!c.has_defect(ShapeDefect::NullShape));
}

#[test]
fn check_null_shape_reports_defect() {
    let mut c = BrepAlgoApiCheck::new(0);
    c.perform();
    assert!(!c.is_valid());
    assert!(c.has_defect(ShapeDefect::NullShape));
    assert_eq!(c.nb_defects(), 1);
    assert_eq!(c.defect(0), Some(ShapeDefect::NullShape));
}

#[test]
fn defeaturing_done_with_faces() {
    let mut d = BrepAlgoApiDefeaturing::new(100);
    d.add_face_to_remove(1);
    d.add_face_to_remove(2);
    d.add_face_to_remove(3);
    d.build();
    assert!(d.is_done());
    assert_eq!(d.status, AlgoApiStatus::Done);
    assert_eq!(d.nb_faces_removed(), 3);
    assert_eq!(d.shape(), 100 + 20000);
}

#[test]
fn defeaturing_null_input_status() {
    let mut d = BrepAlgoApiDefeaturing::new(0);
    d.add_face_to_remove(5);
    d.build();
    assert!(!d.is_done());
    assert_eq!(d.status, AlgoApiStatus::NullInput);
    assert_eq!(d.nb_faces_removed(), 0);
}

#[test]
fn defeaturing_error_when_no_faces() {
    let mut d = BrepAlgoApiDefeaturing::new(7);
    d.build();
    assert_eq!(d.status, AlgoApiStatus::Error);
    assert!(!d.is_done());
}

#[test]
fn bop_algo_options_setters() {
    let mut opt = BopAlgoOptions::new();
    opt.set_fuzzy_value(1e-5);
    opt.set_run_parallel(true);
    opt.set_check_inverted(true);
    opt.set_nondestructive(true);
    opt.set_glue(2);
    assert!(opt.run_parallel);
    assert!(opt.check_inverted);
    assert!(opt.nondestructive);
    assert_eq!(opt.glue, 2);
    assert!((opt.fuzzy_value() - 1e-5).abs() < 1e-15);
}
