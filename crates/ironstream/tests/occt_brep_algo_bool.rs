// FILE: tests/occt_brep_algo_bool.rs
extern crate ironstream;
use ironstream::brep_algo_bool::*;

#[test]
fn fuse_new_not_done() {
    let f = BRepAlgoFuse::new();
    assert!(!f.is_done());
}

#[test]
fn fuse_build_sets_done() {
    let mut f = BRepAlgoFuse::new();
    f.build();
    assert!(f.is_done());
}

#[test]
fn fuse_operation_type() {
    let f = BRepAlgoFuse::new();
    assert_eq!(f.operation(), BoolOpType::Fuse);
}

#[test]
fn common_build_sets_done() {
    let mut c = BRepAlgoCommon::new();
    c.build();
    assert!(c.is_done());
    assert_eq!(c.operation(), BoolOpType::Common);
}

#[test]
fn cut_build_sets_done() {
    let mut c = BRepAlgoCut::new();
    c.build();
    assert!(c.is_done());
    assert_eq!(c.operation(), BoolOpType::Cut);
}

#[test]
fn section_build_sets_done() {
    let mut s = BRepAlgoSection::new();
    s.build();
    assert!(s.is_done());
    assert_eq!(s.operation(), BoolOpType::Section);
}

#[test]
fn fuzzy_value_default_zero() {
    let f = BRepAlgoFuse::new();
    assert_eq!(f.fuzzy_value(), 0.0);
}

#[test]
fn set_fuzzy_value_persists() {
    let mut c = BRepAlgoCut::new();
    c.set_fuzzy_value(1e-7);
    assert!((c.fuzzy_value() - 1e-7).abs() < 1e-15);
}

#[test]
fn has_errors_always_false() {
    let mut f = BRepAlgoFuse::new();
    f.build();
    assert!(!f.has_errors());

    let mut c = BRepAlgoCut::new();
    c.build();
    assert!(!c.has_errors());
}

#[test]
fn no_warnings_on_new_ops() {
    let f = BRepAlgoFuse::new();
    assert_eq!(f.nb_warnings(), 0);
    assert!(!f.has_warnings());

    let s = BRepAlgoSection::new();
    assert_eq!(s.nb_warnings(), 0);
    assert!(!s.has_warnings());
}

#[test]
fn bool_op_type_equality() {
    assert_eq!(BoolOpType::Fuse, BoolOpType::Fuse);
    assert_ne!(BoolOpType::Fuse, BoolOpType::Cut);
    assert_ne!(BoolOpType::Common, BoolOpType::Section);
}

#[test]
fn bool_op_type_debug() {
    assert_eq!(format!("{:?}", BoolOpType::Fuse), "Fuse");
    assert_eq!(format!("{:?}", BoolOpType::Common), "Common");
    assert_eq!(format!("{:?}", BoolOpType::Cut), "Cut");
    assert_eq!(format!("{:?}", BoolOpType::Section), "Section");
}

#[test]
fn bool_op_error_debug_and_eq() {
    assert_eq!(BoolOpError::NotDone, BoolOpError::NotDone);
    assert_ne!(BoolOpError::EmptyShape, BoolOpError::SelfIntersect);
    assert_eq!(format!("{:?}", BoolOpError::InvalidInput), "InvalidInput");
}

#[test]
fn common_fuzzy_value_set_and_build() {
    let mut c = BRepAlgoCommon::new();
    c.set_fuzzy_value(0.001);
    c.build();
    assert!(c.is_done());
    assert!((c.fuzzy_value() - 0.001).abs() < 1e-12);
    assert!(!c.has_errors());
}
