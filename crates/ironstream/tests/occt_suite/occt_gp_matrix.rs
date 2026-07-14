use ironstream::gp_matrix::*;

#[test]
fn mat_identity_det_equals_1_not_singular() {
    let m = GpMat::identity();
    assert!((m.determinant() - 1.0).abs() < 1e-12);
    assert!(!m.is_singular(1e-10));
}

#[test]
fn mat_multiply_identity_preserves_values() {
    let id = GpMat::identity();
    let mut b = GpMat::default();
    b.set_rows([2.0, 0.0, 0.0], [0.0, 3.0, 0.0], [0.0, 0.0, 4.0]);
    let c = id.multiply(&b);
    assert!((c.value(0, 0) - 2.0).abs() < 1e-10);
    assert!((c.value(1, 1) - 3.0).abs() < 1e-10);
    assert!((c.value(2, 2) - 4.0).abs() < 1e-10);
}

#[test]
fn mat_transposed_element_swapped() {
    let mut m = GpMat::default();
    m.set_value(0, 1, 5.0);
    m.set_value(1, 0, 3.0);
    let t = m.transposed();
    assert!((t.value(1, 0) - 5.0).abs() < 1e-10);
    assert!((t.value(0, 1) - 3.0).abs() < 1e-10);
}

#[test]
fn mat2d_det_and_inverse() {
    let m = GpMat2d::new(2.0, 1.0, 1.0, 2.0);
    assert!((m.determinant() - 3.0).abs() < 1e-12);
    let inv = m.inverted(1e-10).unwrap();
    let id = m.multiply(&inv);
    assert!((id.a - 1.0).abs() < 1e-10);
    assert!(id.b.abs() < 1e-10);
    assert!(id.c.abs() < 1e-10);
    assert!((id.d - 1.0).abs() < 1e-10);
}

#[test]
fn mat2d_singular_no_inverse() {
    let m = GpMat2d::new(1.0, 2.0, 2.0, 4.0);
    assert!(m.is_singular(1e-10));
    assert!(m.inverted(1e-10).is_none());
}
