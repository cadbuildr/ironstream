use ironstream::math_matrix::*;

#[test]
fn math_status_ok_variant() {
    let s = MathStatus::default();
    assert!(s.is_ok());
    assert!(!MathStatus::SingularMatrix.is_ok());
}

#[test]
fn math_vector_min_max() {
    let v = MathVector::from_slice(1, &[3.0, -1.0, 7.0, 2.0]);
    assert!((v.max_value() - 7.0).abs() < 1e-10);
    assert!((v.min_value() - (-1.0)).abs() < 1e-10);
}

#[test]
fn math_vector_sub_and_norm() {
    let a = MathVector::from_slice(1, &[4.0, 0.0, 0.0]);
    let b = MathVector::from_slice(1, &[1.0, 0.0, 0.0]);
    let diff = a.sub(&b).unwrap();
    assert!((diff.norm() - 3.0).abs() < 1e-10);
}

#[test]
fn math_integer_vector_with_init() {
    let v = MathIntegerVector::with_init(2, 5, -7);
    assert_eq!(v.get(2), Some(-7));
    assert_eq!(v.get(5), Some(-7));
    assert_eq!(v.len(), 4);
    assert_eq!(v.max_value(), -7);
    assert_eq!(v.min_value(), -7);
}

#[test]
fn math_matrix_add() {
    let m1 = MathMatrix::with_init(1, 2, 1, 2, 1.0);
    let m2 = MathMatrix::with_init(1, 2, 1, 2, 2.0);
    let sum = m1.add(&m2).unwrap();
    assert!((sum.get(1, 1).unwrap() - 3.0).abs() < 1e-10);
    assert!((sum.get(2, 2).unwrap() - 3.0).abs() < 1e-10);
}

#[test]
fn math_matrix_multiply_square() {
    // 2x2 matrix times itself
    let mut m = MathMatrix::new(1, 2, 1, 2);
    m.set(1, 1, 1.0); m.set(1, 2, 2.0);
    m.set(2, 1, 3.0); m.set(2, 2, 4.0);
    let m2 = m.multiply(&m).unwrap();
    // [1,2;3,4]*[1,2;3,4] = [7,10;15,22]
    assert!((m2.get(1, 1).unwrap() - 7.0).abs() < 1e-10);
    assert!((m2.get(2, 2).unwrap() - 22.0).abs() < 1e-10);
}
