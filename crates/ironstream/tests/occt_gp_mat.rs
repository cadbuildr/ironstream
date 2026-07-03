//! Ported OpenCascade unit tests — `gp_Mat`.
//!
//! Faithful Rust port of OCCT's GoogleTest suite
//! `src/FoundationClasses/TKMath/GTests/gp_Mat_Test.cxx`
//! (Open-Cascade-SAS/OCCT). Same inputs, same expected values, same tolerances.
//! OCCT is the reference; IronStream's [`ironstream::gp_mat::Mat`] must
//! reproduce it.
//!
//! The upstream GTest file currently contains exactly one case,
//! `gp_MatTest.OCC22595_DefaultConstructor` (regression for bug OCC22595:
//! `gp_Mat`'s constructors incompletely initialise memory). That case is ported
//! verbatim below as [`occ22595_default_constructor`].
//!
//! Because the upstream suite exercises only the default constructor, the
//! remaining tests in this file verify the rest of the `gp_Mat` public API
//! against the exact semantics defined in `gp_Mat.hxx` (determinant, rotation,
//! inverse, products, transpose, …) using closed-form expected values and
//! `Precision::Confusion`/`Angular` tolerances — never a stub or a skipped
//! method.

use ironstream::gp::Pnt;
use ironstream::gp_mat::Mat;
use ironstream::precision::{ANGULAR, CONFUSION};
use std::f64::consts::FRAC_PI_2;

fn near(a: f64, b: f64, tol: f64) {
    assert!((a - b).abs() <= tol, "expected {a} ≈ {b} (tol {tol})");
}

fn mats_near(a: &Mat, b: &Mat, tol: f64) {
    for r in 1..=3 {
        for c in 1..=3 {
            near(a.value(r, c).unwrap(), b.value(r, c).unwrap(), tol);
        }
    }
}

// ============================ gp_Mat_Test.cxx ============================

/// `TEST(gp_MatTest, OCC22595_DefaultConstructor)` — ported verbatim.
///
/// Bug OCC22595: gp_Mat's constructors incompletely initialize memory.
/// Test that the default constructor properly initializes all matrix
/// elements to zero. (`EXPECT_DOUBLE_EQ` is an exact comparison; we keep that
/// strictness.)
#[test]
fn occ22595_default_constructor() {
    let m0 = Mat::new();

    assert_eq!(0.0, m0.value(1, 1).unwrap());
    assert_eq!(0.0, m0.value(1, 2).unwrap());
    assert_eq!(0.0, m0.value(1, 3).unwrap());

    assert_eq!(0.0, m0.value(2, 1).unwrap());
    assert_eq!(0.0, m0.value(2, 2).unwrap());
    assert_eq!(0.0, m0.value(2, 3).unwrap());

    assert_eq!(0.0, m0.value(3, 1).unwrap());
    assert_eq!(0.0, m0.value(3, 2).unwrap());
    assert_eq!(0.0, m0.value(3, 3).unwrap());
}

// The `Default` impl must behave identically to `Mat::new()` (OCC22595).
#[test]
fn occ22595_default_trait_constructor() {
    let m0 = Mat::default();
    for r in 1..=3 {
        for c in 1..=3 {
            assert_eq!(0.0, m0.value(r, c).unwrap());
        }
    }
}

// ===================== gp_Mat.hxx semantics coverage =====================

#[test]
fn coefficient_constructor_row_order() {
    // gp_Mat(a11..a33) fills in row order.
    let m = Mat::from_coeffs(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
    near(m.value(1, 1).unwrap(), 1.0, CONFUSION);
    near(m.value(1, 2).unwrap(), 2.0, CONFUSION);
    near(m.value(1, 3).unwrap(), 3.0, CONFUSION);
    near(m.value(2, 1).unwrap(), 4.0, CONFUSION);
    near(m.value(2, 2).unwrap(), 5.0, CONFUSION);
    near(m.value(2, 3).unwrap(), 6.0, CONFUSION);
    near(m.value(3, 1).unwrap(), 7.0, CONFUSION);
    near(m.value(3, 2).unwrap(), 8.0, CONFUSION);
    near(m.value(3, 3).unwrap(), 9.0, CONFUSION);
}

#[test]
fn set_identity_and_scale_and_diagonal() {
    let mut m = Mat::new();
    m.set_identity();
    near(m.determinant(), 1.0, CONFUSION);
    near(m.value(1, 1).unwrap(), 1.0, CONFUSION);
    near(m.value(2, 2).unwrap(), 1.0, CONFUSION);
    near(m.value(3, 3).unwrap(), 1.0, CONFUSION);
    near(m.value(1, 2).unwrap(), 0.0, CONFUSION);

    let mut s = Mat::new();
    s.set_scale(2.0);
    near(s.determinant(), 8.0, CONFUSION); // 2^3
    near(s.value(1, 1).unwrap(), 2.0, CONFUSION);

    let mut d = Mat::new();
    d.set_diagonal(3.0, 4.0, 5.0);
    near(d.value(1, 1).unwrap(), 3.0, CONFUSION);
    near(d.value(2, 2).unwrap(), 4.0, CONFUSION);
    near(d.value(3, 3).unwrap(), 5.0, CONFUSION);
    // Off-diagonal untouched (still zero from the null constructor).
    near(d.value(1, 2).unwrap(), 0.0, CONFUSION);
    near(d.diagonal().x, 3.0, CONFUSION);
    near(d.diagonal().y, 4.0, CONFUSION);
    near(d.diagonal().z, 5.0, CONFUSION);
}

#[test]
fn set_cols_rows_and_accessors() {
    let mut m = Mat::new();
    m.set_cols(
        Pnt::new(1.0, 2.0, 3.0),
        Pnt::new(4.0, 5.0, 6.0),
        Pnt::new(7.0, 8.0, 9.0),
    );
    let c1 = m.column(1).unwrap();
    near(c1.x, 1.0, CONFUSION);
    near(c1.y, 2.0, CONFUSION);
    near(c1.z, 3.0, CONFUSION);
    let r1 = m.row(1).unwrap();
    near(r1.x, 1.0, CONFUSION);
    near(r1.y, 4.0, CONFUSION);
    near(r1.z, 7.0, CONFUSION);

    let mut mr = Mat::new();
    mr.set_rows(
        Pnt::new(1.0, 2.0, 3.0),
        Pnt::new(4.0, 5.0, 6.0),
        Pnt::new(7.0, 8.0, 9.0),
    );
    let row2 = mr.row(2).unwrap();
    near(row2.x, 4.0, CONFUSION);
    near(row2.y, 5.0, CONFUSION);
    near(row2.z, 6.0, CONFUSION);

    // Single SetCol / SetRow.
    let mut m2 = Mat::identity();
    m2.set_col(2, Pnt::new(10.0, 11.0, 12.0)).unwrap();
    let c2 = m2.column(2).unwrap();
    near(c2.x, 10.0, CONFUSION);
    near(c2.y, 11.0, CONFUSION);
    near(c2.z, 12.0, CONFUSION);

    m2.set_row(3, Pnt::new(20.0, 21.0, 22.0)).unwrap();
    let r3 = m2.row(3).unwrap();
    near(r3.x, 20.0, CONFUSION);
    near(r3.y, 21.0, CONFUSION);
    near(r3.z, 22.0, CONFUSION);
}

#[test]
fn index_out_of_range_is_error() {
    let mut m = Mat::identity();
    assert!(m.value(0, 1).is_err());
    assert!(m.value(4, 1).is_err());
    assert!(m.value(1, 0).is_err());
    assert!(m.value(1, 4).is_err());
    assert!(m.set_value(4, 4, 1.0).is_err());
    assert!(m.set_col(0, Pnt::origin()).is_err());
    assert!(m.set_row(4, Pnt::origin()).is_err());
    assert!(m.column(4).is_err());
    assert!(m.row(0).is_err());
    assert!(m.change_value(0, 0).is_err());
}

#[test]
fn set_value_and_change_value() {
    let mut m = Mat::new();
    m.set_value(2, 3, 7.5).unwrap();
    near(m.value(2, 3).unwrap(), 7.5, CONFUSION);
    *m.change_value(1, 1).unwrap() = 9.0;
    near(m.value(1, 1).unwrap(), 9.0, CONFUSION);
}

#[test]
fn determinant_known_matrix() {
    // det( [[2,0,0],[0,3,0],[0,0,4]] ) = 24
    let m = Mat::from_coeffs(2.0, 0.0, 0.0, 0.0, 3.0, 0.0, 0.0, 0.0, 4.0);
    near(m.determinant(), 24.0, CONFUSION);

    // A general matrix: det( [[1,2,3],[4,5,6],[7,8,10]] ) = -3
    let g = Mat::from_coeffs(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 10.0);
    near(g.determinant(), -3.0, CONFUSION);
}

#[test]
fn is_singular_detects_rank_deficiency() {
    // Rows [1,2,3] and [2,4,6] are colinear -> det == 0 -> singular.
    let s = Mat::from_coeffs(1.0, 2.0, 3.0, 2.0, 4.0, 6.0, 7.0, 8.0, 10.0);
    assert!(s.is_singular());
    // Identity is non-singular.
    assert!(!Mat::identity().is_singular());
}

#[test]
fn set_cross_acts_as_cross_product() {
    // M built from ref must satisfy M * v == ref × v for any v.
    let r = Pnt::new(1.0, 2.0, 3.0);
    let mut m = Mat::new();
    m.set_cross(r);
    // The matrix is anti-symmetric.
    near(m.value(1, 1).unwrap(), 0.0, CONFUSION);
    near(m.value(2, 2).unwrap(), 0.0, CONFUSION);
    near(m.value(3, 3).unwrap(), 0.0, CONFUSION);

    let v = Pnt::new(4.0, -5.0, 6.0);
    // M * v computed column-major via the matrix.
    let mv = Pnt::new(
        m.value(1, 1).unwrap() * v.x + m.value(1, 2).unwrap() * v.y + m.value(1, 3).unwrap() * v.z,
        m.value(2, 1).unwrap() * v.x + m.value(2, 2).unwrap() * v.y + m.value(2, 3).unwrap() * v.z,
        m.value(3, 1).unwrap() * v.x + m.value(3, 2).unwrap() * v.y + m.value(3, 3).unwrap() * v.z,
    );
    let cross = r.cross(v);
    near(mv.x, cross.x, CONFUSION);
    near(mv.y, cross.y, CONFUSION);
    near(mv.z, cross.z, CONFUSION);
}

#[test]
fn set_dot_acts_as_outer_product() {
    // M * v == ref * (ref . v)   (ref ⊗ ref applied to v)
    let r = Pnt::new(1.0, 2.0, 3.0);
    let mut m = Mat::new();
    m.set_dot(r);
    // Symmetric.
    near(m.value(1, 2).unwrap(), m.value(2, 1).unwrap(), CONFUSION);
    near(m.value(1, 3).unwrap(), m.value(3, 1).unwrap(), CONFUSION);
    near(m.value(2, 3).unwrap(), m.value(3, 2).unwrap(), CONFUSION);

    let v = Pnt::new(4.0, -5.0, 6.0);
    let mv = Pnt::new(
        m.value(1, 1).unwrap() * v.x + m.value(1, 2).unwrap() * v.y + m.value(1, 3).unwrap() * v.z,
        m.value(2, 1).unwrap() * v.x + m.value(2, 2).unwrap() * v.y + m.value(2, 3).unwrap() * v.z,
        m.value(3, 1).unwrap() * v.x + m.value(3, 2).unwrap() * v.y + m.value(3, 3).unwrap() * v.z,
    );
    let expected = r * r.dot(v);
    near(mv.x, expected.x, CONFUSION);
    near(mv.y, expected.y, CONFUSION);
    near(mv.z, expected.z, CONFUSION);
}

#[test]
fn set_rotation_z_90_degrees() {
    // Rotation of +90deg about +Z maps X->Y, Y->-X.
    let mut m = Mat::new();
    m.set_rotation(Pnt::new(0.0, 0.0, 1.0), FRAC_PI_2).unwrap();
    // Expected matrix [[0,-1,0],[1,0,0],[0,0,1]].
    near(m.value(1, 1).unwrap(), 0.0, CONFUSION);
    near(m.value(1, 2).unwrap(), -1.0, CONFUSION);
    near(m.value(1, 3).unwrap(), 0.0, CONFUSION);
    near(m.value(2, 1).unwrap(), 1.0, CONFUSION);
    near(m.value(2, 2).unwrap(), 0.0, CONFUSION);
    near(m.value(2, 3).unwrap(), 0.0, CONFUSION);
    near(m.value(3, 1).unwrap(), 0.0, CONFUSION);
    near(m.value(3, 2).unwrap(), 0.0, CONFUSION);
    near(m.value(3, 3).unwrap(), 1.0, CONFUSION);

    // A rotation is orthonormal with determinant +1.
    near(m.determinant(), 1.0, CONFUSION);

    // Apply to X = (1,0,0): expect (0,1,0).
    let x = Pnt::new(1.0, 0.0, 0.0);
    let rx = Pnt::new(
        m.value(1, 1).unwrap() * x.x + m.value(1, 2).unwrap() * x.y + m.value(1, 3).unwrap() * x.z,
        m.value(2, 1).unwrap() * x.x + m.value(2, 2).unwrap() * x.y + m.value(2, 3).unwrap() * x.z,
        m.value(3, 1).unwrap() * x.x + m.value(3, 2).unwrap() * x.y + m.value(3, 3).unwrap() * x.z,
    );
    near(rx.x, 0.0, CONFUSION);
    near(rx.y, 1.0, CONFUSION);
    near(rx.z, 0.0, CONFUSION);
}

#[test]
fn set_rotation_null_axis_is_error() {
    let mut m = Mat::new();
    assert!(m.set_rotation(Pnt::origin(), FRAC_PI_2).is_err());
}

#[test]
fn set_rotation_normalizes_axis() {
    // A non-unit axis (0,0,5) must produce the same rotation as the unit axis.
    let mut a = Mat::new();
    a.set_rotation(Pnt::new(0.0, 0.0, 5.0), FRAC_PI_2).unwrap();
    let mut b = Mat::new();
    b.set_rotation(Pnt::new(0.0, 0.0, 1.0), FRAC_PI_2).unwrap();
    mats_near(&a, &b, CONFUSION);
}

#[test]
fn add_subtract() {
    let a = Mat::from_coeffs(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
    let b = Mat::from_coeffs(9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0);
    let sum = a.added(&b);
    for r in 1..=3 {
        for c in 1..=3 {
            near(sum.value(r, c).unwrap(), 10.0, CONFUSION);
        }
    }
    let diff = sum.subtracted(&b);
    mats_near(&diff, &a, CONFUSION);

    // In-place variants.
    let mut acc = a;
    acc.add(&b);
    mats_near(&acc, &sum, CONFUSION);
    acc.subtract(&b);
    mats_near(&acc, &a, CONFUSION);
}

#[test]
fn scalar_multiply_and_divide() {
    let a = Mat::from_coeffs(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
    let scaled = a.multiplied_scalar(2.0);
    near(scaled.value(1, 1).unwrap(), 2.0, CONFUSION);
    near(scaled.value(3, 3).unwrap(), 18.0, CONFUSION);

    let back = scaled.divided(2.0).unwrap();
    mats_near(&back, &a, CONFUSION);

    // Divide by ~0 is an error.
    assert!(a.divided(0.0).is_err());
}

#[test]
fn matrix_product_with_identity_and_general() {
    let a = Mat::from_coeffs(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 10.0);
    let id = Mat::identity();
    // A * I == A and I * A == A.
    mats_near(&a.multiplied(&id), &a, CONFUSION);
    let mut pre = a;
    pre.pre_multiply(&id);
    mats_near(&pre, &a, CONFUSION);

    // Concrete product: [[1,2],[3,4]] style extended to 3x3.
    let b = Mat::from_coeffs(2.0, 0.0, 0.0, 0.0, 2.0, 0.0, 0.0, 0.0, 2.0); // 2*I
    let ab = a.multiplied(&b);
    near(ab.value(1, 1).unwrap(), 2.0, CONFUSION);
    near(ab.value(2, 3).unwrap(), 12.0, CONFUSION);
    near(ab.value(3, 3).unwrap(), 20.0, CONFUSION);
}

#[test]
fn multiply_matches_hand_computed_product() {
    let a = Mat::from_coeffs(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
    let b = Mat::from_coeffs(10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0, 18.0);
    let ab = a.multiplied(&b);
    // Row 1 of A*B:
    // [1,2,3]·col1 = 1*10+2*13+3*16 = 84
    // [1,2,3]·col2 = 1*11+2*14+3*17 = 90
    // [1,2,3]·col3 = 1*12+2*15+3*18 = 96
    near(ab.value(1, 1).unwrap(), 84.0, CONFUSION);
    near(ab.value(1, 2).unwrap(), 90.0, CONFUSION);
    near(ab.value(1, 3).unwrap(), 96.0, CONFUSION);
    // Row 3: [7,8,9]
    // ·col1 = 7*10+8*13+9*16 = 318
    near(ab.value(3, 1).unwrap(), 318.0, CONFUSION);
}

#[test]
fn transpose_swaps_off_diagonal() {
    let a = Mat::from_coeffs(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
    let t = a.transposed();
    near(t.value(1, 2).unwrap(), 4.0, CONFUSION);
    near(t.value(2, 1).unwrap(), 2.0, CONFUSION);
    near(t.value(1, 3).unwrap(), 7.0, CONFUSION);
    near(t.value(3, 1).unwrap(), 3.0, CONFUSION);
    // Diagonal unchanged.
    near(t.value(2, 2).unwrap(), 5.0, CONFUSION);
    // Transposing twice is the identity transform.
    mats_near(&t.transposed(), &a, CONFUSION);

    // For a rotation, transpose == inverse.
    let mut rot = Mat::new();
    rot.set_rotation(Pnt::new(1.0, 1.0, 1.0), 0.7).unwrap();
    mats_near(&rot.transposed(), &rot.inverted().unwrap(), CONFUSION);
}

#[test]
fn inverted_gives_true_inverse() {
    let a = Mat::from_coeffs(1.0, 2.0, 3.0, 0.0, 1.0, 4.0, 5.0, 6.0, 0.0);
    let inv = a.inverted().unwrap();
    // A * A^-1 == I.
    let prod = a.multiplied(&inv);
    mats_near(&prod, &Mat::identity(), CONFUSION);
    // A^-1 * A == I.
    let prod2 = inv.multiplied(&a);
    mats_near(&prod2, &Mat::identity(), CONFUSION);
}

#[test]
fn invert_in_place_matches_inverted() {
    let a = Mat::from_coeffs(1.0, 2.0, 3.0, 0.0, 1.0, 4.0, 5.0, 6.0, 0.0);
    let mut b = a;
    b.invert().unwrap();
    mats_near(&b, &a.inverted().unwrap(), CONFUSION);
}

#[test]
fn inverted_singular_is_error() {
    // Colinear rows -> singular.
    let s = Mat::from_coeffs(1.0, 2.0, 3.0, 2.0, 4.0, 6.0, 7.0, 8.0, 10.0);
    assert!(s.inverted().is_err());
}

#[test]
fn powered_identity_and_inverse_and_squares() {
    let a = Mat::from_coeffs(1.0, 2.0, 3.0, 0.0, 1.0, 4.0, 5.0, 6.0, 0.0);

    // n == 0 -> identity.
    mats_near(&a.powered(0).unwrap(), &Mat::identity(), CONFUSION);
    // n == 1 -> itself.
    mats_near(&a.powered(1).unwrap(), &a, CONFUSION);
    // n == 2 -> A*A.
    mats_near(&a.powered(2).unwrap(), &a.multiplied(&a), CONFUSION);
    // n == 3 -> A*A*A.
    mats_near(
        &a.powered(3).unwrap(),
        &a.multiplied(&a).multiplied(&a),
        CONFUSION,
    );
    // n == -1 -> inverse.
    mats_near(&a.powered(-1).unwrap(), &a.inverted().unwrap(), CONFUSION);
    // n == -2 -> inverse squared; A^2 * A^-2 == I.
    let p2 = a.powered(2).unwrap();
    let pm2 = a.powered(-2).unwrap();
    mats_near(&p2.multiplied(&pm2), &Mat::identity(), CONFUSION);

    // In-place Power agrees with Powered.
    let mut b = a;
    b.power(3).unwrap();
    mats_near(&b, &a.powered(3).unwrap(), CONFUSION);
}

#[test]
fn rotation_is_orthonormal() {
    // Any SetRotation result has orthonormal rows (R * R^T == I).
    let mut r = Mat::new();
    r.set_rotation(Pnt::new(2.0, -1.0, 0.5), 1.234).unwrap();
    let rt = r.transposed();
    mats_near(&r.multiplied(&rt), &Mat::identity(), CONFUSION);
    near(r.determinant(), 1.0, CONFUSION);
    // Angle preserved: angle between basis images equals 90deg.
    let ex = r.column(1).unwrap();
    let ey = r.column(2).unwrap();
    near(ex.angle(ey), FRAC_PI_2, ANGULAR * 1.0e6);
}
