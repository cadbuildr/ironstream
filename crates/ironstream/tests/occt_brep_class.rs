extern crate ironstream;
use ironstream::brep_class::*;

#[test]
fn test_brep_class_basic() {
    // SolidClassifier
    let mut sc = SolidClassifier::new("MySolid");
    assert_eq!(sc.solid, "MySolid");
    // tolerance is recorded by perform(), not preset at construction
    assert_eq!(sc.tolerance, 0.0);
    let state = sc.perform([0.0, 0.0, 0.0], 1e-7);
    assert!((sc.tolerance - 1e-7).abs() < 1e-12);
    // stub returns Unknown
    assert_eq!(state, TopState::Unknown);
    assert_eq!(sc.state(), TopState::Unknown);
    assert!(!sc.is_on_face());

    // FaceClassifier
    let mut fc = FaceClassifier::new();
    let fs = fc.perform("MyFace", [0.5, 0.5], 1e-7);
    assert_eq!(fs, TopState::Unknown);
    assert_eq!(fc.state(), TopState::Unknown);

    // Free functions
    let s = point_in_solid([1.0, 2.0, 3.0], "Box", 1e-6);
    assert_eq!(s, TopState::Unknown);
    let f = point_in_face([0.3, 0.7], "Face1", 1e-6);
    assert_eq!(f, TopState::Unknown);

    // TopState variants
    assert_ne!(TopState::In, TopState::Out);
    assert_ne!(TopState::On, TopState::Unknown);
}
