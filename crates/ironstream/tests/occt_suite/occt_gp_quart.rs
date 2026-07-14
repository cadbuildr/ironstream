extern crate ironstream;
use ironstream::gp_quart::*;

#[test]
fn test_gp_quart_basic() {
    let id = Quaternion::identity();
    assert_eq!(id.w, 1.0);
    assert_eq!(id.x, 0.0);

    // 90-degree rotation about Z
    let q = Quaternion::from_axis_angle([0.0, 0.0, 1.0], std::f64::consts::FRAC_PI_2);
    assert!((q.norm() - 1.0).abs() < 1e-10);

    // Rotating X by 90 around Z should give Y
    let v = q.rotate_vec([1.0, 0.0, 0.0]);
    assert!((v[0] - 0.0).abs() < 1e-9);
    assert!((v[1] - 1.0).abs() < 1e-9);
    assert!((v[2] - 0.0).abs() < 1e-9);

    let conj = q.conjugate();
    assert!((conj.x + q.x).abs() < 1e-10);
    assert!((conj.w - q.w).abs() < 1e-10);

    // q * q^-1 == identity
    let prod = q.mul(&conj);
    let norm_prod = prod.normalize();
    assert!((norm_prod.w - 1.0).abs() < 1e-9);

    // round-trip through matrix
    let mat = q.to_matrix();
    let q2 = Quaternion::from_matrix(mat);
    let v2 = q2.rotate_vec([1.0, 0.0, 0.0]);
    assert!((v2[0] - v[0]).abs() < 1e-9);
    assert!((v2[1] - v[1]).abs() < 1e-9);
}
