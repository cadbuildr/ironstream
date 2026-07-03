// FILE: rust/ironstream/crates/ironstream/tests/occt_brep_algo_check.rs

extern crate ironstream;
use ironstream::brep_algo_check::*;

#[test]
fn test_new_checker_is_valid_and_empty() {
    let checker = BrepAlgoCheck::new();
    assert!(checker.is_valid());
    assert_eq!(checker.nb_errors(), 0);
    assert!(!checker.has_faulty_shapes());
}

#[test]
fn test_check_shape_valid_no_errors() {
    let mut checker = BrepAlgoCheck::new();
    checker.check_shape(true, vec![]);
    assert!(checker.is_valid());
    assert_eq!(checker.nb_errors(), 0);
    assert!(!checker.has_faulty_shapes());
}

#[test]
fn test_check_shape_invalid_with_errors() {
    let mut checker = BrepAlgoCheck::new();
    let err = BrepAlgoCheckError::new(
        BrepAlgoCheckErrorType::SelfInterferingShape,
        0,
        "self-intersecting solid".to_string(),
    );
    checker.check_shape(false, vec![err]);
    assert!(!checker.is_valid());
    assert_eq!(checker.nb_errors(), 1);
    assert!(checker.has_faulty_shapes());
}

#[test]
fn test_errors_slice_length_matches_nb_errors() {
    let mut checker = BrepAlgoCheck::new();
    let errors = vec![
        BrepAlgoCheckError::new(BrepAlgoCheckErrorType::BadType, 0, "bad type".to_string()),
        BrepAlgoCheckError::new(
            BrepAlgoCheckErrorType::IncompatibleEdge,
            1,
            "incompatible edge".to_string(),
        ),
    ];
    checker.check_shape(false, errors);
    assert_eq!(checker.nb_errors(), 2);
    assert_eq!(checker.errors().len(), 2);
}

#[test]
fn test_error_type_accessor() {
    let err = BrepAlgoCheckError::new(
        BrepAlgoCheckErrorType::NonRecoverableFace,
        3,
        "cannot recover face".to_string(),
    );
    assert_eq!(err.error_type(), BrepAlgoCheckErrorType::NonRecoverableFace);
}

#[test]
fn test_shape_index_accessor() {
    let err = BrepAlgoCheckError::new(
        BrepAlgoCheckErrorType::IncompatibleVertex,
        7,
        "vertex mismatch".to_string(),
    );
    assert_eq!(err.shape_index(), 7);
}

#[test]
fn test_message_accessor() {
    let err = BrepAlgoCheckError::new(
        BrepAlgoCheckErrorType::NotImplemented,
        0,
        "feature not implemented".to_string(),
    );
    assert_eq!(err.message(), "feature not implemented");
}

#[test]
fn test_no_error_variant() {
    let err = BrepAlgoCheckError::new(BrepAlgoCheckErrorType::NoError, 0, String::new());
    assert_eq!(err.error_type(), BrepAlgoCheckErrorType::NoError);
    assert_eq!(err.shape_index(), 0);
    assert_eq!(err.message(), "");
}

#[test]
fn test_bop_not_valid_variant() {
    let err = BrepAlgoCheckError::new(
        BrepAlgoCheckErrorType::BOPNotValid,
        2,
        "BOP not valid".to_string(),
    );
    assert_eq!(err.error_type(), BrepAlgoCheckErrorType::BOPNotValid);
    assert_eq!(err.shape_index(), 2);
}

#[test]
fn test_incompatible_face_variant() {
    let err = BrepAlgoCheckError::new(
        BrepAlgoCheckErrorType::IncompatibleFace,
        5,
        "face incompatible".to_string(),
    );
    assert_eq!(err.error_type(), BrepAlgoCheckErrorType::IncompatibleFace);
    assert_eq!(err.shape_index(), 5);
}

#[test]
fn test_has_faulty_shapes_false_when_no_errors() {
    let mut checker = BrepAlgoCheck::new();
    checker.check_shape(true, vec![]);
    assert!(!checker.has_faulty_shapes());
}

#[test]
fn test_has_faulty_shapes_true_with_errors() {
    let mut checker = BrepAlgoCheck::new();
    checker.check_shape(
        false,
        vec![BrepAlgoCheckError::new(
            BrepAlgoCheckErrorType::SelfInterferingShape,
            0,
            "self-interference".to_string(),
        )],
    );
    assert!(checker.has_faulty_shapes());
}

#[test]
fn test_errors_returns_correct_entries() {
    let mut checker = BrepAlgoCheck::new();
    let errors = vec![
        BrepAlgoCheckError::new(BrepAlgoCheckErrorType::BadType, 0, "bad type".to_string()),
        BrepAlgoCheckError::new(
            BrepAlgoCheckErrorType::NonRecoverableFace,
            1,
            "face issue".to_string(),
        ),
        BrepAlgoCheckError::new(
            BrepAlgoCheckErrorType::NotImplemented,
            2,
            "not implemented".to_string(),
        ),
    ];
    checker.check_shape(false, errors);
    let slice = checker.errors();
    assert_eq!(slice.len(), 3);
    assert_eq!(slice[0].error_type(), BrepAlgoCheckErrorType::BadType);
    assert_eq!(slice[1].error_type(), BrepAlgoCheckErrorType::NonRecoverableFace);
    assert_eq!(slice[2].error_type(), BrepAlgoCheckErrorType::NotImplemented);
    assert_eq!(slice[0].shape_index(), 0);
    assert_eq!(slice[1].shape_index(), 1);
    assert_eq!(slice[2].shape_index(), 2);
}

#[test]
fn test_check_shape_replaces_previous_state() {
    let mut checker = BrepAlgoCheck::new();
    checker.check_shape(
        false,
        vec![BrepAlgoCheckError::new(
            BrepAlgoCheckErrorType::BadType,
            0,
            "first check".to_string(),
        )],
    );
    assert!(!checker.is_valid());
    assert_eq!(checker.nb_errors(), 1);

    checker.check_shape(true, vec![]);
    assert!(checker.is_valid());
    assert_eq!(checker.nb_errors(), 0);
    assert!(!checker.has_faulty_shapes());
}

#[test]
fn test_default_equals_new() {
    let a = BrepAlgoCheck::new();
    let b = BrepAlgoCheck::default();
    assert_eq!(a.is_valid(), b.is_valid());
    assert_eq!(a.nb_errors(), b.nb_errors());
}

#[test]
fn test_error_type_variants_are_distinct() {
    assert_ne!(
        BrepAlgoCheckErrorType::NoError,
        BrepAlgoCheckErrorType::BOPNotValid
    );
    assert_ne!(
        BrepAlgoCheckErrorType::IncompatibleVertex,
        BrepAlgoCheckErrorType::IncompatibleEdge
    );
    assert_ne!(
        BrepAlgoCheckErrorType::IncompatibleFace,
        BrepAlgoCheckErrorType::SelfInterferingShape
    );
    assert_ne!(
        BrepAlgoCheckErrorType::BadType,
        BrepAlgoCheckErrorType::NonRecoverableFace
    );
    assert_ne!(
        BrepAlgoCheckErrorType::NonRecoverableFace,
        BrepAlgoCheckErrorType::NotImplemented
    );
}
