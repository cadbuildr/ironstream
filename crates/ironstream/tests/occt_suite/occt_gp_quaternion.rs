//! Ported OpenCascade unit tests — `gp_Quaternion`.
//!
//! Faithful Rust port of OCCT's quaternion tests, supplemented with
//! closed-form expected values drawn from the OCCT API reference
//! (`gp_Quaternion.hxx`).  All tolerances use
//! `Precision::Confusion` / `Precision::Angular`.

use ironstream::gp_quaternion::{EulerSeq, Quaternion};
use ironstream::precision::{ANGULAR, CONFUSION};
use std::f64::consts::{FRAC_PI_2, FRAC_PI_4, PI};

// ── helpers ───────────────────────────────────────────────────────────────────

fn near(a: f64, b: f64, tol: f64) {
    assert!(
        (a - b).abs() <= tol,
        "expected {a} ≈ {b} (tol {tol}, diff {})",
        (a - b).abs()
    );
}

fn qnear(q: &Quaternion, x: f64, y: f64, z: f64, w: f64, tol: f64) {
    near(q.x(), x, tol);
    near(q.y(), y, tol);
    near(q.z(), z, tol);
    near(q.w(), w, tol);
}

// ── tests ─────────────────────────────────────────────────────────────────────

/// `SetIdent` resets to identity `(0, 0, 0, 1)`.
#[test]
fn set_ident_resets_to_identity() {
    let mut q = Quaternion::new(0.1, 0.2, 0.3, 0.9);
    q.set_ident();
    qnear(&q, 0.0, 0.0, 0.0, 1.0, CONFUSION);
    near(q.norm(), 1.0, CONFUSION);
}

/// `Set(x, y, z, w)` stores the four components verbatim.
#[test]
fn set_stores_components() {
    let mut q = Quaternion::identity();
    q.set(0.1, 0.2, 0.3, 0.927);
    near(q.x(), 0.1, CONFUSION);
    near(q.y(), 0.2, CONFUSION);
    near(q.z(), 0.3, CONFUSION);
    near(q.w(), 0.927, CONFUSION);
}

/// `SetVectorAndAngle` then `GetVectorAndAngle` round-trips correctly.
#[test]
fn vector_and_angle_round_trip() {
    // 90-degree rotation about Z axis.
    let mut q = Quaternion::identity();
    q.set_vector_and_angle(0.0, 0.0, 1.0, FRAC_PI_2).unwrap();

    let (ax, ay, az, angle) = q.get_vector_and_angle();
    near(ax, 0.0, CONFUSION);
    near(ay, 0.0, CONFUSION);
    near(az, 1.0, CONFUSION);
    near(angle, FRAC_PI_2, CONFUSION);
}

/// Rotation of 180° about X: `w = 0`, `x = 1`, `y = z = 0`.
#[test]
fn set_vector_and_angle_180_about_x() {
    let mut q = Quaternion::identity();
    q.set_vector_and_angle(1.0, 0.0, 0.0, PI).unwrap();

    near(q.w(), 0.0, CONFUSION);
    near(q.x(), 1.0, CONFUSION);
    near(q.y(), 0.0, CONFUSION);
    near(q.z(), 0.0, CONFUSION);
}

/// `Norm` of a unit quaternion is 1.
#[test]
fn norm_of_unit_quaternion() {
    let q = Quaternion::new(0.0, 0.0, (FRAC_PI_4).sin(), (FRAC_PI_4).cos());
    near(q.norm(), 1.0, CONFUSION);
}

/// `Normalize` rescales a non-unit quaternion to unit length.
#[test]
fn normalize_rescales_to_unit() {
    let mut q = Quaternion::new(2.0, 0.0, 0.0, 0.0);
    q.normalize();
    near(q.norm(), 1.0, CONFUSION);
    near(q.x(), 1.0, CONFUSION);
}

/// `Normalized` returns a unit copy without changing `self`.
#[test]
fn normalized_does_not_mutate_self() {
    let q = Quaternion::new(0.0, 3.0, 4.0, 0.0);
    let qn = q.normalized();
    // Original unchanged.
    near(q.x(), 0.0, CONFUSION);
    near(q.y(), 3.0, CONFUSION);
    near(q.z(), 4.0, CONFUSION);
    // Result is unit.
    near(qn.norm(), 1.0, CONFUSION);
}

/// `Inverted` of a unit quaternion equals its conjugate.
#[test]
fn inverted_unit_equals_conjugate() {
    let mut q = Quaternion::identity();
    q.set_vector_and_angle(0.0, 1.0, 0.0, FRAC_PI_2).unwrap();
    let qi = q.inverted();
    let qc = q.conjugated();
    near(qi.x(), qc.x(), CONFUSION);
    near(qi.y(), qc.y(), CONFUSION);
    near(qi.z(), qc.z(), CONFUSION);
    near(qi.w(), qc.w(), CONFUSION);
}

/// `Conjugated` negates the vector part and leaves `w` unchanged.
#[test]
fn conjugated_negates_vector_part() {
    let q = Quaternion::new(1.0, 2.0, 3.0, 4.0);
    let qc = q.conjugated();
    near(qc.x(), -1.0, CONFUSION);
    near(qc.y(), -2.0, CONFUSION);
    near(qc.z(), -3.0, CONFUSION);
    near(qc.w(), 4.0, CONFUSION);
}

/// `Negated` negates all four components.
#[test]
fn negated_flips_all_components() {
    let q = Quaternion::new(0.5, -0.5, 0.5, 0.5);
    let qn = q.negated();
    near(qn.x(), -0.5, CONFUSION);
    near(qn.y(), 0.5, CONFUSION);
    near(qn.z(), -0.5, CONFUSION);
    near(qn.w(), -0.5, CONFUSION);
}

/// `Multiplied` composes two rotations: apply 90° about Z twice gives 180° about Z.
#[test]
fn multiplied_composes_rotations() {
    let mut q90 = Quaternion::identity();
    q90.set_vector_and_angle(0.0, 0.0, 1.0, FRAC_PI_2).unwrap();
    let q180 = q90.multiplied(&q90);
    // The product should represent 180° about Z.
    let (ax, ay, az, angle) = q180.get_vector_and_angle();
    near(az.abs(), 1.0, CONFUSION); // axis ±Z
    near(ax, 0.0, CONFUSION);
    near(ay, 0.0, CONFUSION);
    near(angle, PI, CONFUSION);
}

/// `Dot` returns 1 for two identical unit quaternions.
#[test]
fn dot_product_self_equals_one() {
    let mut q = Quaternion::identity();
    q.set_vector_and_angle(1.0, 0.0, 0.0, PI / 3.0).unwrap();
    near(q.dot(&q), 1.0, CONFUSION);
}

/// `Add` and `Subtract` are component-wise.
#[test]
fn add_subtract_component_wise() {
    let a = Quaternion::new(1.0, 2.0, 3.0, 4.0);
    let b = Quaternion::new(0.5, 0.5, 0.5, 0.5);
    let sum = a.added(&b);
    near(sum.x(), 1.5, CONFUSION);
    near(sum.y(), 2.5, CONFUSION);
    near(sum.z(), 3.5, CONFUSION);
    near(sum.w(), 4.5, CONFUSION);
    let diff = a.subtracted(&b);
    near(diff.x(), 0.5, CONFUSION);
    near(diff.y(), 1.5, CONFUSION);
    near(diff.z(), 2.5, CONFUSION);
    near(diff.w(), 3.5, CONFUSION);
}

/// `IsEqual` is true for the same quaternion and for its negation.
#[test]
fn is_equal_same_and_negated() {
    let mut q = Quaternion::identity();
    q.set_vector_and_angle(0.0, 0.0, 1.0, FRAC_PI_4).unwrap();
    assert!(q.is_equal(&q, CONFUSION));
    let qn = q.negated();
    assert!(q.is_equal(&qn, CONFUSION));
    // Different rotation should not be equal.
    let mut q2 = Quaternion::identity();
    q2.set_vector_and_angle(0.0, 0.0, 1.0, PI / 3.0).unwrap();
    assert!(!q.is_equal(&q2, CONFUSION));
}

/// `GetMatrix` for a 90° rotation about Z gives the correct rotation matrix.
#[test]
fn get_matrix_90_about_z() {
    let mut q = Quaternion::identity();
    q.set_vector_and_angle(0.0, 0.0, 1.0, FRAC_PI_2).unwrap();
    let m = q.get_matrix();
    // R = [[0,-1,0],[1,0,0],[0,0,1]]
    near(m.m[0][0], 0.0, CONFUSION);
    near(m.m[0][1], -1.0, CONFUSION);
    near(m.m[0][2], 0.0, CONFUSION);
    near(m.m[1][0], 1.0, CONFUSION);
    near(m.m[1][1], 0.0, CONFUSION);
    near(m.m[1][2], 0.0, CONFUSION);
    near(m.m[2][0], 0.0, CONFUSION);
    near(m.m[2][1], 0.0, CONFUSION);
    near(m.m[2][2], 1.0, CONFUSION);
}

/// `Rotate` applies the rotation to a vector correctly.
/// 90° about Z: (1,0,0) → (0,1,0).
#[test]
fn rotate_vector_90_about_z() {
    let mut q = Quaternion::identity();
    q.set_vector_and_angle(0.0, 0.0, 1.0, FRAC_PI_2).unwrap();
    let (rx, ry, rz) = q.rotate(1.0, 0.0, 0.0);
    near(rx, 0.0, CONFUSION);
    near(ry, 1.0, CONFUSION);
    near(rz, 0.0, CONFUSION);
}

/// `Rotate` with identity quaternion leaves a vector unchanged.
#[test]
fn rotate_with_identity_is_no_op() {
    let q = Quaternion::identity();
    let (rx, ry, rz) = q.rotate(3.0, -1.0, 2.0);
    near(rx, 3.0, CONFUSION);
    near(ry, -1.0, CONFUSION);
    near(rz, 2.0, CONFUSION);
}

/// `SetEulerAngles` / `GetEulerAngles` round-trips for XYZ convention.
#[test]
fn euler_xyz_round_trip() {
    let alpha = 0.3;
    let beta = 0.5;
    let gamma = 0.7;
    let mut q = Quaternion::identity();
    q.set_euler_angles(EulerSeq::XYZ, alpha, beta, gamma);
    let (a2, b2, c2) = q.get_euler_angles(EulerSeq::XYZ);
    near(a2, alpha, 1.0e-9);
    near(b2, beta, 1.0e-9);
    near(c2, gamma, 1.0e-9);
}

/// `SetEulerAngles` / `GetEulerAngles` round-trips for ZYX convention.
#[test]
fn euler_zyx_round_trip() {
    let alpha = 0.4;
    let beta = 0.2;
    let gamma = 0.6;
    let mut q = Quaternion::identity();
    q.set_euler_angles(EulerSeq::ZYX, alpha, beta, gamma);
    let (a2, b2, c2) = q.get_euler_angles(EulerSeq::ZYX);
    near(a2, alpha, 1.0e-9);
    near(b2, beta, 1.0e-9);
    near(c2, gamma, 1.0e-9);
}

/// `Invert` in place: `q * q_inv == identity`.
#[test]
fn invert_in_place() {
    let mut q = Quaternion::identity();
    q.set_vector_and_angle(1.0, 1.0, 1.0, PI / 4.0).unwrap();
    let qi = q.inverted();
    // q * q^{-1} should be identity.
    let prod = q.multiplied(&qi);
    near(prod.x(), 0.0, CONFUSION);
    near(prod.y(), 0.0, CONFUSION);
    near(prod.z(), 0.0, CONFUSION);
    near(prod.w(), 1.0, CONFUSION);
}

/// 180° rotation about X: rotating (0,1,0) gives (0,−1,0).
#[test]
fn rotate_180_about_x() {
    let mut q = Quaternion::identity();
    q.set_vector_and_angle(1.0, 0.0, 0.0, PI).unwrap();
    let (rx, ry, rz) = q.rotate(0.0, 1.0, 0.0);
    near(rx, 0.0, CONFUSION);
    near(ry, -1.0, CONFUSION);
    near(rz, 0.0, CONFUSION);
}

/// `GetVectorAndAngle` for the identity quaternion returns angle 0.
#[test]
fn get_vector_and_angle_identity() {
    let q = Quaternion::identity();
    let (_ax, _ay, _az, angle) = q.get_vector_and_angle();
    near(angle, 0.0, ANGULAR);
}

/// Zero-axis `SetVectorAndAngle` returns an error.
#[test]
fn set_vector_and_angle_zero_axis_errors() {
    let mut q = Quaternion::identity();
    let result = q.set_vector_and_angle(0.0, 0.0, 0.0, 1.0);
    assert!(result.is_err());
}
