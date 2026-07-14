// Integration tests for gp_conv module
use ironstream::gp_conv::*;
use std::f64::consts::PI;

#[test]
fn gp_mat_identity_determinant_and_get() {
    let m = GpMat::identity();
    assert!((m.get(0, 0) - 1.0).abs() < 1e-10);
    assert!((m.get(0, 1)).abs() < 1e-10);
    assert!((m.get(1, 1) - 1.0).abs() < 1e-10);
    assert!((m.determinant() - 1.0).abs() < 1e-10);
}

#[test]
fn gp_mat_rotation_z_transforms_x_to_y() {
    let m = GpMat::rotation_z(PI / 2.0);
    let v = m.transform_dir([1.0, 0.0, 0.0]);
    assert!(v[0].abs() < 1e-10);
    assert!((v[1] - 1.0).abs() < 1e-10);
    assert!(v[2].abs() < 1e-10);
}

#[test]
fn gp_mat_transposed_and_multiply() {
    let m = GpMat::rotation_z(PI / 4.0);
    let mt = m.transposed();
    // R * R^T = I
    let product = m.multiply(&mt);
    for i in 0..3 {
        for j in 0..3 {
            let expected = if i == j { 1.0 } else { 0.0 };
            assert!((product.get(i, j) - expected).abs() < 1e-10,
                "product[{i}][{j}] = {} != {expected}", product.get(i, j));
        }
    }
}

#[test]
fn gp_mat2d_rotation_and_inverse() {
    let m = GpMat2d::rotation(PI / 2.0);
    let v = m.transform_vec([1.0, 0.0]);
    assert!(v[0].abs() < 1e-10);
    assert!((v[1] - 1.0).abs() < 1e-10);

    let inv = m.inverse().unwrap();
    // M * M^-1 should give identity
    let product = m.multiply(&inv);
    assert!((product.get(0, 0) - 1.0).abs() < 1e-10);
    assert!((product.get(1, 1) - 1.0).abs() < 1e-10);
    assert!(product.get(0, 1).abs() < 1e-10);
}

#[test]
fn gp_gtrsf_translation_and_rotation() {
    let mut g = GpGTrsf::new();
    assert_eq!(g.form(), TrsfForm::Identity);

    g.set_translation([1.0, 2.0, 3.0]);
    assert_eq!(g.form(), TrsfForm::Translation);
    let p = g.transform_point([0.0, 0.0, 0.0]);
    assert!((p[0] - 1.0).abs() < 1e-10);
    assert!((p[1] - 2.0).abs() < 1e-10);
    assert!((p[2] - 3.0).abs() < 1e-10);

    let mut g2 = GpGTrsf::new();
    g2.set_rotation([0.0, 0.0, 1.0], PI / 2.0);
    assert_eq!(g2.form(), TrsfForm::Rotation);
    let q = g2.transform_point([1.0, 0.0, 0.0]);
    assert!(q[0].abs() < 1e-10);
    assert!((q[1] - 1.0).abs() < 1e-10);
}

#[test]
fn gp_gtrsf2d_rotation_transforms_correctly() {
    let mut g = GpGTrsf2d::new();
    g.set_rotation(PI / 2.0);
    let p = g.transform_point([1.0, 0.0]);
    assert!(p[0].abs() < 1e-10);
    assert!((p[1] - 1.0).abs() < 1e-10);

    // With translation added
    let mut g2 = GpGTrsf2d::new();
    g2.set_translation([5.0, 5.0]);
    let q = g2.transform_point([1.0, 1.0]);
    assert!((q[0] - 6.0).abs() < 1e-10);
    assert!((q[1] - 6.0).abs() < 1e-10);
}
