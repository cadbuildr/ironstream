//! Ported OpenCascade unit tests — `gp_Vec` class (TKMath package).
//!
//! Faithful Rust ports of OCCT's `gp_Vec_Test.cxx` tests
//! (Open-Cascade-SAS/OCCT, `src/FoundationClasses/TKMath/GTests/`).
//! Same inputs, expected values, and tolerances
//! (`Precision::Confusion` / `Precision::Angular`). OCCT is the reference;
//! IronStream must reproduce it.

use ironstream::gp::{Ax1, Ax3, Pnt, Trsf};
use ironstream::gp_vec::Vec as GpVec;
use ironstream::precision::{ANGULAR, CONFUSION};
use std::f64::consts::{FRAC_PI_2, PI};

fn near(a: f64, b: f64, tol: f64) {
    assert!((a - b).abs() <= tol, "expected {a} ≈ {b} (tol {tol:.2e})");
}

fn vec_near(v: &GpVec, x: f64, y: f64, z: f64, tol: f64) {
    near(v.x(), x, tol);
    near(v.y(), y, tol);
    near(v.z(), z, tol);
}

// ── Construction & accessors ──────────────────────────────────────────────────

#[test]
fn vec_constructor_and_coord() {
    let v = GpVec::new(1.0, 2.0, 3.0);
    let (x, y, z) = v.coord();
    assert_eq!(x, 1.0);
    assert_eq!(y, 2.0);
    assert_eq!(z, 3.0);
    assert_eq!(v.x(), 1.0);
    assert_eq!(v.y(), 2.0);
    assert_eq!(v.z(), 3.0);
}

#[test]
fn vec_zero() {
    let v = GpVec::zero();
    assert_eq!(v.x(), 0.0);
    assert_eq!(v.y(), 0.0);
    assert_eq!(v.z(), 0.0);
    assert_eq!(v.magnitude(), 0.0);
}

// ── Magnitude ────────────────────────────────────────────────────────────────

#[test]
fn vec_magnitude() {
    let v = GpVec::new(3.0, 4.0, 0.0);
    near(v.magnitude(), 5.0, CONFUSION);
    near(v.square_magnitude(), 25.0, CONFUSION);
}

#[test]
fn vec_magnitude_3d() {
    // sqrt(1+4+9) = sqrt(14)
    let v = GpVec::new(1.0, 2.0, 3.0);
    near(v.magnitude(), 14_f64.sqrt(), CONFUSION);
    near(v.square_magnitude(), 14.0, CONFUSION);
}

// ── Arithmetic ────────────────────────────────────────────────────────────────

#[test]
fn vec_add_subtract() {
    let a = GpVec::new(1.0, 2.0, 3.0);
    let b = GpVec::new(4.0, 5.0, 6.0);
    let sum = a.add(&b);
    vec_near(&sum, 5.0, 7.0, 9.0, CONFUSION);
    let diff = b.subtract(&a);
    vec_near(&diff, 3.0, 3.0, 3.0, CONFUSION);
}

#[test]
fn vec_multiply_divide() {
    let v = GpVec::new(2.0, 4.0, 6.0);
    let scaled = v.multiplied(3.0);
    vec_near(&scaled, 6.0, 12.0, 18.0, CONFUSION);
    let halved = v.divided(2.0);
    vec_near(&halved, 1.0, 2.0, 3.0, CONFUSION);
}

#[test]
fn vec_reversed() {
    let v = GpVec::new(1.0, -2.0, 3.0);
    let r = v.reversed();
    vec_near(&r, -1.0, 2.0, -3.0, CONFUSION);
}

#[test]
fn vec_ops_traits() {
    let a = GpVec::new(1.0, 2.0, 3.0);
    let b = GpVec::new(1.0, 0.0, 0.0);
    let sum = a + b;
    vec_near(&sum, 2.0, 2.0, 3.0, CONFUSION);
    let diff = a - b;
    vec_near(&diff, 0.0, 2.0, 3.0, CONFUSION);
    let scaled = a * 2.0;
    vec_near(&scaled, 2.0, 4.0, 6.0, CONFUSION);
    let halved = a / 2.0;
    vec_near(&halved, 0.5, 1.0, 1.5, CONFUSION);
    let neg = -a;
    vec_near(&neg, -1.0, -2.0, -3.0, CONFUSION);
}

// ── Cross & dot products ──────────────────────────────────────────────────────

#[test]
fn vec_crossed_xy() {
    let x = GpVec::new(1.0, 0.0, 0.0);
    let y = GpVec::new(0.0, 1.0, 0.0);
    let z = x.crossed(&y);
    vec_near(&z, 0.0, 0.0, 1.0, CONFUSION);
}

#[test]
fn vec_cross_product_operator() {
    let a = GpVec::new(1.0, 0.0, 0.0);
    let b = GpVec::new(0.0, 1.0, 0.0);
    let c = a ^ b; // operator^ = cross
    vec_near(&c, 0.0, 0.0, 1.0, CONFUSION);
}

#[test]
fn vec_cross_magnitude() {
    let a = GpVec::new(3.0, 0.0, 0.0);
    let b = GpVec::new(0.0, 4.0, 0.0);
    near(a.cross_magnitude(&b), 12.0, CONFUSION);
    near(a.cross_square_magnitude(&b), 144.0, CONFUSION);
}

#[test]
fn vec_dot_product() {
    let a = GpVec::new(1.0, 2.0, 3.0);
    let b = GpVec::new(4.0, 5.0, 6.0);
    // 4 + 10 + 18 = 32
    near(a.dot(&b), 32.0, CONFUSION);
}

#[test]
fn vec_dot_cross_triple_product() {
    // a · (b × c) = scalar triple product
    let a = GpVec::new(1.0, 0.0, 0.0);
    let b = GpVec::new(0.0, 1.0, 0.0);
    let c = GpVec::new(0.0, 0.0, 1.0);
    // (1,0,0) · ((0,1,0) × (0,0,1)) = (1,0,0)·(1,0,0) = 1
    near(a.dot_cross(&b, &c), 1.0, CONFUSION);
}

// ── Angles ────────────────────────────────────────────────────────────────────

#[test]
fn vec_angle_orthogonal() {
    let x = GpVec::new(1.0, 0.0, 0.0);
    let y = GpVec::new(0.0, 1.0, 0.0);
    near(x.angle(&y), FRAC_PI_2, ANGULAR);
}

#[test]
fn vec_angle_parallel() {
    let a = GpVec::new(2.0, 0.0, 0.0);
    let b = GpVec::new(5.0, 0.0, 0.0);
    near(a.angle(&b), 0.0, ANGULAR);
}

#[test]
fn vec_angle_antiparallel() {
    let a = GpVec::new(1.0, 0.0, 0.0);
    let b = GpVec::new(-1.0, 0.0, 0.0);
    near(a.angle(&b), PI, ANGULAR);
}

#[test]
fn vec_angle_general() {
    // (1,1,0) and (1,0,0) → 45°
    let a = GpVec::new(1.0, 1.0, 0.0);
    let b = GpVec::new(1.0, 0.0, 0.0);
    near(a.angle(&b), PI / 4.0, ANGULAR);
}

#[test]
fn vec_angle_with_ref_sign() {
    let x = GpVec::new(1.0, 0.0, 0.0);
    let y = GpVec::new(0.0, 1.0, 0.0);
    let z_ref = GpVec::new(0.0, 0.0, 1.0);
    // x × y = z → same direction as z_ref → positive angle
    near(x.angle_with_ref(&y, &z_ref), FRAC_PI_2, ANGULAR);
    // Reversed ref → negative angle
    let neg_z = GpVec::new(0.0, 0.0, -1.0);
    near(x.angle_with_ref(&y, &neg_z), -FRAC_PI_2, ANGULAR);
}

// ── Predicates ────────────────────────────────────────────────────────────────

#[test]
fn vec_is_equal() {
    let a = GpVec::new(1.0, 2.0, 3.0);
    let b = GpVec::new(1.0, 2.0, 3.0);
    assert!(a.is_equal(&b, CONFUSION, ANGULAR));
    let c = GpVec::new(1.0, 2.0, 4.0);
    assert!(!a.is_equal(&c, CONFUSION, ANGULAR));
}

#[test]
fn vec_is_normal() {
    let x = GpVec::new(1.0, 0.0, 0.0);
    let y = GpVec::new(0.0, 1.0, 0.0);
    assert!(x.is_normal(&y, ANGULAR));
    assert!(!x.is_normal(&x, ANGULAR));
}

#[test]
fn vec_is_opposite() {
    let a = GpVec::new(1.0, 0.0, 0.0);
    let b = GpVec::new(-1.0, 0.0, 0.0);
    assert!(a.is_opposite(&b, ANGULAR));
    assert!(!a.is_opposite(&a, ANGULAR));
}

#[test]
fn vec_is_parallel() {
    let a = GpVec::new(2.0, 0.0, 0.0);
    let b = GpVec::new(-5.0, 0.0, 0.0);
    assert!(a.is_parallel(&b, ANGULAR));
    let c = GpVec::new(3.0, 0.0, 0.0);
    assert!(a.is_parallel(&c, ANGULAR));
    let d = GpVec::new(0.0, 1.0, 0.0);
    assert!(!a.is_parallel(&d, ANGULAR));
}

// ── Normalized ───────────────────────────────────────────────────────────────

#[test]
fn vec_normalized() {
    let v = GpVec::new(3.0, 4.0, 0.0);
    let n = v.normalized();
    near(n.magnitude(), 1.0, CONFUSION);
    near(n.x(), 0.6, CONFUSION);
    near(n.y(), 0.8, CONFUSION);
    near(n.z(), 0.0, CONFUSION);
}

#[test]
fn vec_normalize_in_place() {
    let mut v = GpVec::new(0.0, 0.0, 5.0);
    v.normalize();
    near(v.magnitude(), 1.0, CONFUSION);
    near(v.z(), 1.0, CONFUSION);
}

// ── Rotation ─────────────────────────────────────────────────────────────────

#[test]
fn vec_rotated_90_about_z() {
    let v = GpVec::new(1.0, 0.0, 0.0);
    let axis = Ax1::new(Pnt::origin(), Pnt::new(0.0, 0.0, 1.0));
    let r = v.rotated(axis, FRAC_PI_2);
    near(r.x(), 0.0, CONFUSION);
    near(r.y(), 1.0, CONFUSION);
    near(r.z(), 0.0, CONFUSION);
}

#[test]
fn vec_rotated_180_about_y() {
    // Rotating (1, 0, 0) by π about Y → (-1, 0, 0)
    let v = GpVec::new(1.0, 0.0, 0.0);
    let axis = Ax1::new(Pnt::origin(), Pnt::new(0.0, 1.0, 0.0));
    let r = v.rotated(axis, PI);
    near(r.x(), -1.0, CONFUSION);
    near(r.y(), 0.0, CONFUSION);
    near(r.z(), 0.0, CONFUSION);
}

// ── Mirror ────────────────────────────────────────────────────────────────────

#[test]
fn vec_mirrored_point() {
    // Mirroring a free vector through a point negates the vector.
    let v = GpVec::new(1.0, 2.0, 3.0);
    let r = v.mirrored_point(Pnt::origin());
    vec_near(&r, -1.0, -2.0, -3.0, CONFUSION);
}

#[test]
fn vec_mirrored_axis() {
    // Mirror (0, 1, 0) through X-axis: reflection across X → (0, -1, 0)
    let v = GpVec::new(0.0, 1.0, 0.0);
    let axis = Ax1::new(Pnt::origin(), Pnt::new(1.0, 0.0, 0.0));
    let r = v.mirrored_axis(axis);
    // v' = 2(v·d)d - v, d=(1,0,0), v·d=0 → v'=-v
    near(r.x(), 0.0, CONFUSION);
    near(r.y(), -1.0, CONFUSION);
    near(r.z(), 0.0, CONFUSION);
}

#[test]
fn vec_mirrored_axis_along() {
    // Mirror (1, 0, 0) through X-axis → (1, 0, 0) (component along axis unchanged)
    let v = GpVec::new(1.0, 0.0, 0.0);
    let axis = Ax1::new(Pnt::origin(), Pnt::new(1.0, 0.0, 0.0));
    let r = v.mirrored_axis(axis);
    near(r.x(), 1.0, CONFUSION);
    near(r.y(), 0.0, CONFUSION);
    near(r.z(), 0.0, CONFUSION);
}

#[test]
fn vec_mirrored_plane_xy() {
    // Mirror (0, 0, 1) through XY plane (normal = Z) → (0, 0, -1)
    let v = GpVec::new(0.0, 0.0, 1.0);
    let ax2 = Ax3::from_origin_normal(
        Pnt::origin(),
        Pnt::new(0.0, 0.0, 1.0),
        Pnt::new(1.0, 0.0, 0.0),
    );
    let r = v.mirrored_plane(ax2);
    near(r.x(), 0.0, CONFUSION);
    near(r.y(), 0.0, CONFUSION);
    near(r.z(), -1.0, CONFUSION);
}

// ── Scale & transform ─────────────────────────────────────────────────────────

#[test]
fn vec_scaled() {
    let v = GpVec::new(1.0, 2.0, 3.0);
    let s = v.scaled(5.0);
    vec_near(&s, 5.0, 10.0, 15.0, CONFUSION);
}

#[test]
fn vec_transformed_translation_ignored() {
    // A pure translation Trsf should NOT affect a free vector.
    let v = GpVec::new(1.0, 0.0, 0.0);
    let mut t = Trsf::identity();
    t.set_translation(Pnt::new(100.0, 200.0, 300.0));
    let r = v.transformed(&t);
    near(r.x(), 1.0, CONFUSION);
    near(r.y(), 0.0, CONFUSION);
    near(r.z(), 0.0, CONFUSION);
}

#[test]
fn vec_transformed_rotation() {
    // 90° rotation about Z: (1,0,0) → (0,1,0)
    let v = GpVec::new(1.0, 0.0, 0.0);
    let axis = Ax1::new(Pnt::origin(), Pnt::new(0.0, 0.0, 1.0));
    let t = Trsf::rotation(axis, FRAC_PI_2);
    let r = v.transformed(&t);
    near(r.x(), 0.0, CONFUSION);
    near(r.y(), 1.0, CONFUSION);
    near(r.z(), 0.0, CONFUSION);
}

// ── Conversion ────────────────────────────────────────────────────────────────

#[test]
fn vec_from_pnt_and_to_pnt() {
    let p = Pnt::new(3.0, 4.0, 5.0);
    let v = GpVec::from_pnt(p);
    assert_eq!(v.x(), 3.0);
    assert_eq!(v.y(), 4.0);
    assert_eq!(v.z(), 5.0);
    let q = v.to_pnt();
    assert_eq!(q.x, 3.0);
    assert_eq!(q.y, 4.0);
    assert_eq!(q.z, 5.0);
}

#[test]
fn vec_from_dir_is_unit() {
    let v = GpVec::from_dir(Pnt::new(3.0, 4.0, 0.0));
    near(v.magnitude(), 1.0, CONFUSION);
}
