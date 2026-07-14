// FILE: tests/occt_bop_algo.rs
extern crate ironstream;

use ironstream::bop_algo::{
    BopAlgoBop, BopAlgoBuilder, BopAlgoCheckStatus, BopAlgoCheckerSI, BopAlgoOperation,
};

#[test]
fn builder_fuse_build_is_done() {
    let mut b = BopAlgoBuilder::new(BopAlgoOperation::Fuse);
    assert!(!b.is_done());
    b.build();
    assert!(b.is_done());
}

#[test]
fn builder_fuzzy_value_set_and_get() {
    let mut b = BopAlgoBuilder::new(BopAlgoOperation::Common);
    b.set_fuzzy_value(1e-7);
    assert!((b.fuzzy_value() - 1e-7).abs() < 1e-15);
}

#[test]
fn builder_operation_preserved() {
    assert_eq!(
        BopAlgoBuilder::new(BopAlgoOperation::Cut).operation(),
        BopAlgoOperation::Cut
    );
    assert_eq!(
        BopAlgoBuilder::new(BopAlgoOperation::Section).operation(),
        BopAlgoOperation::Section
    );
    assert_eq!(
        BopAlgoBuilder::new(BopAlgoOperation::Cut21).operation(),
        BopAlgoOperation::Cut21
    );
}

#[test]
fn builder_non_destructive_default_false() {
    let b = BopAlgoBuilder::new(BopAlgoOperation::Fuse);
    assert!(!b.non_destructive());
}

#[test]
fn builder_set_non_destructive() {
    let mut b = BopAlgoBuilder::new(BopAlgoOperation::Fuse);
    b.set_non_destructive(true);
    assert!(b.non_destructive());
}

#[test]
fn builder_no_errors_no_warnings_after_build() {
    let mut b = BopAlgoBuilder::new(BopAlgoOperation::Fuse);
    b.build();
    assert!(!b.has_errors());
    assert!(!b.has_warnings());
    assert_eq!(b.nb_errors(), 0);
    assert_eq!(b.nb_warnings(), 0);
}

#[test]
fn checker_si_perform_marks_done() {
    let mut c = BopAlgoCheckerSI::new();
    assert!(!c.is_done());
    c.perform();
    assert!(c.is_done());
}

#[test]
fn checker_si_no_interferences_after_perform() {
    let mut c = BopAlgoCheckerSI::new();
    c.perform();
    assert!(!c.has_interferences());
    assert_eq!(c.nb_interferences(), 0);
}

#[test]
fn checker_si_level_of_check_default_and_set() {
    let mut c = BopAlgoCheckerSI::new();
    assert_eq!(c.level_of_check(), 2);
    c.set_level_of_check(0);
    assert_eq!(c.level_of_check(), 0);
    c.set_level_of_check(1);
    assert_eq!(c.level_of_check(), 1);
}

#[test]
fn bop_algo_bop_fuse_build_done() {
    let mut bop = BopAlgoBop::new(BopAlgoOperation::Fuse);
    assert_eq!(bop.operation(), BopAlgoOperation::Fuse);
    bop.set_fuzzy_value(1e-6);
    assert!((bop.fuzzy_value() - 1e-6).abs() < 1e-14);
    bop.build();
    assert!(bop.is_done());
}

#[test]
fn bop_algo_bop_cut_non_destructive() {
    let mut bop = BopAlgoBop::new(BopAlgoOperation::Cut);
    bop.set_non_destructive(true);
    bop.build();
    assert!(bop.non_destructive());
    assert!(bop.is_done());
    assert!(!bop.has_errors());
}

#[test]
fn bop_algo_bop_run_parallel_flag() {
    let mut bop = BopAlgoBop::new(BopAlgoOperation::Common);
    assert!(!bop.run_parallel());
    bop.set_run_parallel(true);
    assert!(bop.run_parallel());
}

#[test]
fn check_status_no_fault_is_default_variant() {
    let s = BopAlgoCheckStatus::NoFault;
    assert_eq!(s, BopAlgoCheckStatus::NoFault);
    assert_ne!(s, BopAlgoCheckStatus::BadType);
}

#[test]
fn all_operations_copy_and_debug() {
    let ops = [
        BopAlgoOperation::Fuse,
        BopAlgoOperation::Common,
        BopAlgoOperation::Cut,
        BopAlgoOperation::Cut21,
        BopAlgoOperation::Section,
    ];
    for op in ops {
        let _ = format!("{op:?}");
        let copy = op;
        assert_eq!(op, copy);
    }
}
