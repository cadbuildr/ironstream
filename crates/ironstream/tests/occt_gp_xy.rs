//! Ported OpenCascade unit tests — `gp_XY` class (TKMath package).
//!
//! Faithful Rust ports of OCCT's `gp_XY` API tests. Same inputs, expected
//! values, and tolerances (`Precision::Confusion` / `Precision::Angular`).
//! OCCT is the reference; IronStream must reproduce it.

use ironstream::gp_xy::XY;
use ironstream::precision::{ANGULAR, CONFUSION};

fn near(a: f64, b: f64, tol: f64) {
    assert!(
        (a - b).abs() <= tol,
        "expected {a} ≈ {b} (tol {tol:.2e}), delta = {}",
        (a - b).abs()
    );
}

fn xy_near(v: &XY, x: f64, y: f64, tol: f64) {
    near(v.x(), x, tol);
    near(v.y(), y, tol);
}

// ── Construction & accessors ──────────────────────────────────────────────────

#[test]
fn xy_constructor_and_coord() {
    let p = XY::new(1.0, 2.0);
    let (x, y) = p.coord();
    assert_eq!(x, 1.0);
    assert_eq!(y, 2.0);
    assert_eq!(p.x(), 1.0);
    assert_eq!(p.y(), 2.0);
}

#[test]
fn xy_zero() {
    let p = XY::zero();
    assert_eq!(p.x(), 0.0);
    assert_eq!(p.y(), 0.0);
    assert_eq!(p.modulus(), 0.0);
}

#[test]
fn xy_set_coord() {
    let mut p = XY::new(0.0, 0.0);
    p.set_coord(3.0, 7.0);
    assert_eq!(p.x(), 3.0);
    assert_eq!(p.y(), 7.0);
}

#[test]
fn xy_set_x_and_y() {
    let mut p = XY::new(1.0, 2.0);
    p.set_x(10.0);
    p.set_y(20.0);
    assert_eq!(p.x(), 10.0);
    assert_eq!(p.y(), 20.0);
}

#[test]
fn xy_set_coord_by_index() {
    let mut p = XY::new(0.0, 0.0);
    p.set_coord_index(1, 5.0);
    p.set_coord_index(2, 9.0);
    assert_eq!(p.x(), 5.0);
    assert_eq!(p.y(), 9.0);
    // coord_index retrieval
    assert_eq!(p.coord_index(1), 5.0);
    assert_eq!(p.coord_index(2), 9.0);
}

// ── Modulus ───────────────────────────────────────────────────────────────────

#[test]
fn xy_modulus_3_4() {
    let p = XY::new(3.0, 4.0);
    near(p.modulus(), 5.0, CONFUSION);
    near(p.square_modulus(), 25.0, CONFUSION);
}

#[test]
fn xy_modulus_unit_x() {
    let p = XY::new(1.0, 0.0);
    near(p.modulus(), 1.0, CONFUSION);
}

// ── Arithmetic ────────────────────────────────────────────────────────────────

#[test]
fn xy_add_subtract() {
    let a = XY::new(1.0, 2.0);
    let b = XY::new(3.0, 4.0);
    let sum = a.added(&b);
    xy_near(&sum, 4.0, 6.0, CONFUSION);
    let diff = b.subtracted(&a);
    xy_near(&diff, 2.0, 2.0, CONFUSION);
}

#[test]
fn xy_multiply_divide() {
    let p = XY::new(2.0, 6.0);
    let scaled = p.multiplied(3.0);
    xy_near(&scaled, 6.0, 18.0, CONFUSION);
    let halved = p.divided(2.0);
    xy_near(&halved, 1.0, 3.0, CONFUSION);
}

#[test]
fn xy_reversed() {
    let p = XY::new(3.0, -5.0);
    let r = p.reversed();
    xy_near(&r, -3.0, 5.0, CONFUSION);
}

#[test]
fn xy_ops_traits() {
    let a = XY::new(1.0, 2.0);
    let b = XY::new(3.0, 4.0);
    let sum = a + b;
    xy_near(&sum, 4.0, 6.0, CONFUSION);
    let diff = b - a;
    xy_near(&diff, 2.0, 2.0, CONFUSION);
    let scaled = a * 2.0;
    xy_near(&scaled, 2.0, 4.0, CONFUSION);
    let halved = a / 2.0;
    xy_near(&halved, 0.5, 1.0, CONFUSION);
    let neg = -a;
    xy_near(&neg, -1.0, -2.0, CONFUSION);
}

#[test]
fn xy_add_assign() {
    let mut p = XY::new(1.0, 2.0);
    p.add_assign(&XY::new(3.0, 4.0));
    xy_near(&p, 4.0, 6.0, CONFUSION);
}

#[test]
fn xy_subtract_assign() {
    let mut p = XY::new(5.0, 7.0);
    p.subtract_assign(&XY::new(2.0, 3.0));
    xy_near(&p, 3.0, 4.0, CONFUSION);
}

#[test]
fn xy_multiply_in_place() {
    let mut p = XY::new(2.0, 3.0);
    p.multiply(4.0);
    xy_near(&p, 8.0, 12.0, CONFUSION);
}

#[test]
fn xy_divide_in_place() {
    let mut p = XY::new(6.0, 9.0);
    p.divide(3.0);
    xy_near(&p, 2.0, 3.0, CONFUSION);
}

#[test]
fn xy_reverse_in_place() {
    let mut p = XY::new(1.0, -2.0);
    p.reverse();
    xy_near(&p, -1.0, 2.0, CONFUSION);
}

// ── Dot product ───────────────────────────────────────────────────────────────

#[test]
fn xy_dot_product_basic() {
    let a = XY::new(1.0, 2.0);
    let b = XY::new(3.0, 4.0);
    // 1*3 + 2*4 = 11
    near(a.dot(&b), 11.0, CONFUSION);
}

#[test]
fn xy_dot_orthogonal() {
    let x = XY::new(1.0, 0.0);
    let y = XY::new(0.0, 1.0);
    near(x.dot(&y), 0.0, CONFUSION);
}

// ── Cross magnitude ───────────────────────────────────────────────────────────

#[test]
fn xy_cross_magnitude_unit_axes() {
    let x = XY::new(1.0, 0.0);
    let y = XY::new(0.0, 1.0);
    // |1*1 - 0*0| = 1
    near(x.cross_magnitude(&y), 1.0, CONFUSION);
    near(x.cross_square_magnitude(&y), 1.0, CONFUSION);
}

#[test]
fn xy_cross_magnitude_parallel() {
    // Parallel vectors → zero cross magnitude
    let a = XY::new(2.0, 0.0);
    let b = XY::new(5.0, 0.0);
    near(a.cross_magnitude(&b), 0.0, CONFUSION);
}

#[test]
fn xy_cross_magnitude_scaled() {
    // (3,0) × (0,4) → |3*4 - 0*0| = 12
    let a = XY::new(3.0, 0.0);
    let b = XY::new(0.0, 4.0);
    near(a.cross_magnitude(&b), 12.0, CONFUSION);
    near(a.cross_square_magnitude(&b), 144.0, CONFUSION);
}

// ── Normalize ─────────────────────────────────────────────────────────────────

#[test]
fn xy_normalized_3_4() {
    let p = XY::new(3.0, 4.0);
    let n = p.normalized();
    near(n.modulus(), 1.0, CONFUSION);
    near(n.x(), 0.6, CONFUSION);
    near(n.y(), 0.8, CONFUSION);
}

#[test]
fn xy_normalize_in_place() {
    let mut p = XY::new(0.0, 5.0);
    p.normalize();
    near(p.modulus(), 1.0, CONFUSION);
    near(p.x(), 0.0, CONFUSION);
    near(p.y(), 1.0, CONFUSION);
}

// ── IsEqual ───────────────────────────────────────────────────────────────────

#[test]
fn xy_is_equal_exact() {
    let a = XY::new(1.0, 2.0);
    let b = XY::new(1.0, 2.0);
    assert!(a.is_equal(&b, CONFUSION));
}

#[test]
fn xy_is_equal_within_confusion() {
    let a = XY::new(0.0, 0.0);
    // delta just under CONFUSION tolerance
    let b = XY::new(CONFUSION * 0.5, CONFUSION * 0.5);
    assert!(a.is_equal(&b, CONFUSION));
}

#[test]
fn xy_is_not_equal_outside_confusion() {
    let a = XY::new(0.0, 0.0);
    // delta well above CONFUSION
    let b = XY::new(CONFUSION * 10.0, 0.0);
    assert!(!a.is_equal(&b, CONFUSION));
}

// ── Matrix multiply ───────────────────────────────────────────────────────────

#[test]
fn xy_multiplied_identity_matrix() {
    let p = XY::new(3.0, 7.0);
    let identity = [[1.0, 0.0], [0.0, 1.0]];
    let r = p.multiplied_mat(identity);
    xy_near(&r, 3.0, 7.0, CONFUSION);
}

#[test]
fn xy_multiplied_rotation_matrix_90() {
    // 90° CCW rotation: [[0,-1],[1,0]]
    let p = XY::new(1.0, 0.0);
    let rot90 = [[0.0, -1.0], [1.0, 0.0]];
    let r = p.multiplied_mat(rot90);
    near(r.x(), 0.0, CONFUSION);
    near(r.y(), 1.0, CONFUSION);
}

// ── Unused import suppression ─────────────────────────────────────────────────

// Ensure ANGULAR is used (mirrors the gp_vec test style, angular tolerance
// confirms precision constants are importable from this module).
#[test]
fn precision_constants_accessible() {
    // CONFUSION ≈ 1e-7, ANGULAR ≈ 1e-12
    assert!(CONFUSION > ANGULAR);
    assert!(CONFUSION < 1e-6);
    assert!(ANGULAR < 1e-11);
}
