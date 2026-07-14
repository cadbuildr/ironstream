// FILE: tests/occt_gp_extra.rs
extern crate ironstream;
use ironstream::gp_extra::*;
use std::f64::consts::PI;

#[test]
fn test_gp_xy_basic_ops() {
    let a = GpXY::new(3.0, 4.0);
    assert!((a.modulus() - 5.0).abs() < 1e-12);
    let b = a.normalized().unwrap();
    assert!((b.modulus() - 1.0).abs() < 1e-12);
    let c = GpXY::new(1.0, 0.0);
    let d = GpXY::new(0.0, 1.0);
    assert!((c.dot(&d)).abs() < 1e-15);
    assert!((c.cross(&d) - 1.0).abs() < 1e-15);
}

#[test]
fn test_gp_xy_rotate() {
    let v = GpXY::new(1.0, 0.0);
    let r = v.rotated(PI / 2.0);
    assert!(r.x.abs() < 1e-12);
    assert!((r.y - 1.0).abs() < 1e-12);
}

#[test]
fn test_gp_mat2d_identity_and_inverse() {
    let id = GpMat2d::identity();
    assert!((id.determinant() - 1.0).abs() < 1e-14);
    let inv = id.inverted().unwrap();
    assert!(inv.is_identity(1e-12));
}

#[test]
fn test_gp_mat2d_rotation_transforms_vector() {
    let rot = GpMat2d::rotation(PI / 2.0);
    let v = GpXY::new(1.0, 0.0);
    let vr = rot.multiply_xy(&v);
    assert!(vr.x.abs() < 1e-12);
    assert!((vr.y - 1.0).abs() < 1e-12);
}

#[test]
fn test_gp_quaternion_identity_rotate() {
    let q = GpQuaternion::identity();
    assert!((q.norm() - 1.0).abs() < 1e-12);
    let v = q.rotate_vector([1.0, 0.0, 0.0]);
    assert!((v[0] - 1.0).abs() < 1e-12);
    assert!(v[1].abs() < 1e-12);
    assert!(v[2].abs() < 1e-12);
}

#[test]
fn test_gp_quaternion_axis_angle_and_slerp() {
    let q = GpQuaternion::from_axis_angle([0.0, 0.0, 1.0], PI / 2.0);
    let v = q.rotate_vector([1.0, 0.0, 0.0]);
    assert!(v[0].abs() < 1e-12);
    assert!((v[1] - 1.0).abs() < 1e-12);

    let q0 = GpQuaternion::identity();
    let q1 = GpQuaternion::from_axis_angle([0.0, 0.0, 1.0], PI);
    let qm = q0.slerp(&q1, 0.5);
    let (_, angle) = qm.get_axis_angle();
    assert!((angle - PI / 2.0).abs() < 1e-6);
}
