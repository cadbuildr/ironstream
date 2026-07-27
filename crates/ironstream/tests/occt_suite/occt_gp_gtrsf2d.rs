//! Ported OpenCascade unit tests -- `gp_GTrsf2d`.
//!
//! Tests for the general 2D transformation class `gp_GTrsf2d`, ported from
//! OpenCascade's reference semantics.  Each test is an independent `#[test]`
//! function.  Tolerances follow [`ironstream::precision`]:
//! * Linear: `CONFUSION = 1e-7`
//! * Angular: `ANGULAR = 1e-12`

use ironstream::gp2d::{Ax2d, Pnt2d};
use ironstream::gp_gtrsf2d::{GTrsf2d, TrsfForm};
use ironstream::precision::{ANGULAR, CONFUSION};

// ─────────────────────── helpers ───────────────────────

fn near(a: f64, b: f64, tol: f64) {
    assert!(
        (a - b).abs() <= tol,
        "expected {a} ≈ {b}  (tol = {tol},  diff = {})",
        (a - b).abs()
    );
}

fn pnt_near(a: Pnt2d, b: Pnt2d, tol: f64) {
    near(a.x, b.x, tol);
    near(a.y, b.y, tol);
}

fn gtrsf_near(a: &GTrsf2d, b: &GTrsf2d, tol: f64) {
    for row in 1..=2usize {
        for col in 1..=3usize {
            near(a.value(row, col), b.value(row, col), tol);
        }
    }
}

// ─────────────────────── tests ────────────────────────

/// Default constructor produces an identity transform.
#[test]
fn identity_form() {
    let t = GTrsf2d::identity();
    assert_eq!(t.form(), TrsfForm::Identity);
    // Diagonal must be 1, off-diagonal 0, translation 0.
    near(t.value(1, 1), 1.0, ANGULAR);
    near(t.value(1, 2), 0.0, ANGULAR);
    near(t.value(2, 1), 0.0, ANGULAR);
    near(t.value(2, 2), 1.0, ANGULAR);
    near(t.value(1, 3), 0.0, ANGULAR);
    near(t.value(2, 3), 0.0, ANGULAR);
}

/// Identity does not move any point.
#[test]
fn identity_transforms_point() {
    let t = GTrsf2d::identity();
    let p = Pnt2d::new(3.0, -5.0);
    let q = t.transform_pnt(p);
    pnt_near(q, p, CONFUSION);
}

/// SetValue / Value round-trip for all 6 entries (2 rows × 3 cols).
#[test]
fn set_value_round_trip() {
    let mut t = GTrsf2d::identity();
    // Row 1: mat col 1, mat col 2, translation
    t.set_value(1, 1, 2.0);
    t.set_value(1, 2, 3.0);
    t.set_value(1, 3, 7.0);
    // Row 2
    t.set_value(2, 1, -1.0);
    t.set_value(2, 2, 4.0);
    t.set_value(2, 3, -2.0);

    near(t.value(1, 1), 2.0, ANGULAR);
    near(t.value(1, 2), 3.0, ANGULAR);
    near(t.value(1, 3), 7.0, ANGULAR);
    near(t.value(2, 1), -1.0, ANGULAR);
    near(t.value(2, 2), 4.0, ANGULAR);
    near(t.value(2, 3), -2.0, ANGULAR);
}

/// TranslationPart returns the translation column.
#[test]
fn translation_part() {
    let mut t = GTrsf2d::identity();
    t.set_value(1, 3, 5.0);
    t.set_value(2, 3, -3.0);
    let tr = t.translation_part();
    near(tr.x, 5.0, ANGULAR);
    near(tr.y, -3.0, ANGULAR);
}

/// VectorialPart returns the 2×2 matrix.
#[test]
fn vectorial_part() {
    let mut t = GTrsf2d::identity();
    t.set_value(1, 1, 2.0);
    t.set_value(1, 2, 3.0);
    t.set_value(2, 1, -1.0);
    t.set_value(2, 2, 4.0);

    let m = t.vectorial_part();
    near(m[0][0], 2.0, ANGULAR);
    near(m[0][1], 3.0, ANGULAR);
    near(m[1][0], -1.0, ANGULAR);
    near(m[1][1], 4.0, ANGULAR);
}

/// IsNegative: det < 0 when the sign of the determinant is negative.
#[test]
fn is_negative() {
    // Build a reflection: M = diag(-1, 1)  → det = -1
    let mut t = GTrsf2d::identity();
    t.set_value(1, 1, -1.0);
    assert!(t.is_negative(), "det should be negative for a reflection");

    // Identity has det = +1, not negative.
    let id = GTrsf2d::identity();
    assert!(!id.is_negative());
}

/// Transforms applies the affine map correctly.
#[test]
fn transforms_apply() {
    // M = [[2, 1], [0, 3]]   t = [4, -1]
    // For point (1, 2):  x' = 2*1 + 1*2 + 4 = 8,  y' = 0*1 + 3*2 - 1 = 5
    let mut t = GTrsf2d::identity();
    t.set_value(1, 1, 2.0);
    t.set_value(1, 2, 1.0);
    t.set_value(2, 1, 0.0);
    t.set_value(2, 2, 3.0);
    t.set_value(1, 3, 4.0);
    t.set_value(2, 3, -1.0);

    let mut x = 1.0_f64;
    let mut y = 2.0_f64;
    t.transforms(&mut x, &mut y);
    near(x, 8.0, CONFUSION);
    near(y, 5.0, CONFUSION);
}

/// Inverted: compose with inverse gives identity.
#[test]
fn inverted_composed_is_identity() {
    let mut t = GTrsf2d::identity();
    t.set_value(1, 1, 2.0);
    t.set_value(1, 2, 1.0);
    t.set_value(2, 1, -1.0);
    t.set_value(2, 2, 3.0);
    t.set_value(1, 3, 5.0);
    t.set_value(2, 3, -2.0);

    let inv = t.inverted();
    let composed = t.multiplied(&inv);
    let id = GTrsf2d::identity();
    gtrsf_near(&composed, &id, CONFUSION);
}

/// Multiplied: composition is associative and matches manual calculation.
#[test]
fn multiplied_composition() {
    // A: scale x2
    let mut a = GTrsf2d::identity();
    a.set_value(1, 1, 2.0);
    a.set_value(2, 2, 2.0);

    // B: translate (3, 4)
    let mut b = GTrsf2d::identity();
    b.set_value(1, 3, 3.0);
    b.set_value(2, 3, 4.0);

    // A * B applied to (1, 1):
    // B maps (1,1) → (4,5);  A maps (4,5) → (8,10).
    let ab = a.multiplied(&b);
    let p = ab.transform_pnt(Pnt2d::new(1.0, 1.0));
    pnt_near(p, Pnt2d::new(8.0, 10.0), CONFUSION);
}

/// Power(0) gives identity; Power(1) is self; Power(-1) is Inverted.
#[test]
fn power_zero_and_negative() {
    let mut t = GTrsf2d::identity();
    t.set_value(1, 1, 2.0);
    t.set_value(1, 2, 1.0);
    t.set_value(2, 1, 0.0);
    t.set_value(2, 2, 3.0);
    t.set_value(1, 3, 1.0);
    t.set_value(2, 3, -1.0);

    // n = 0 → identity
    let p0 = t.power(0);
    gtrsf_near(&p0, &GTrsf2d::identity(), CONFUSION);

    // n = 1 → self
    let p1 = t.power(1);
    gtrsf_near(&p1, &t, CONFUSION);

    // n = -1 → inverted
    let p_neg1 = t.power(-1);
    let inv = t.inverted();
    gtrsf_near(&p_neg1, &inv, CONFUSION);
}

/// Power(2) equals Multiplied(self).
#[test]
fn power_two_equals_self_composed() {
    let mut t = GTrsf2d::identity();
    t.set_value(1, 1, 1.5);
    t.set_value(1, 2, 0.5);
    t.set_value(2, 1, -0.5);
    t.set_value(2, 2, 1.5);

    let p2 = t.power(2);
    let m2 = t.multiplied(&t);
    gtrsf_near(&p2, &m2, CONFUSION);
}

/// SetAffinity along the X axis with ratio 0 projects points onto the axis.
#[test]
fn set_affinity_x_axis_ratio_zero() {
    // X-axis: location = origin, direction = (1, 0).
    let axis = Ax2d::new(Pnt2d::origin(), Pnt2d::new(1.0, 0.0));
    let mut t = GTrsf2d::identity();
    t.set_affinity(axis, 0.0);

    // Any point (x, y) should map to (x, 0): projection onto X axis.
    let p = t.transform_pnt(Pnt2d::new(3.0, 7.0));
    pnt_near(p, Pnt2d::new(3.0, 0.0), CONFUSION);
}

/// SetAffinity with ratio 1 is the identity transform.
#[test]
fn set_affinity_ratio_one_is_identity() {
    let axis = Ax2d::new(Pnt2d::new(2.0, 3.0), Pnt2d::new(0.0, 1.0));
    let mut t = GTrsf2d::identity();
    t.set_affinity(axis, 1.0);

    // ratio=1: perpendicular component is unchanged → identity.
    let p = Pnt2d::new(5.0, -1.0);
    let q = t.transform_pnt(p);
    pnt_near(q, p, CONFUSION);
}

/// SetAffinity with ratio -1 reflects across the axis.
#[test]
fn set_affinity_ratio_neg1_reflection() {
    // Reflect across the X axis (y → -y).
    let axis = Ax2d::new(Pnt2d::origin(), Pnt2d::new(1.0, 0.0));
    let mut t = GTrsf2d::identity();
    t.set_affinity(axis, -1.0);

    let p = Pnt2d::new(4.0, 3.0);
    let q = t.transform_pnt(p);
    pnt_near(q, Pnt2d::new(4.0, -3.0), CONFUSION);

    // Reflected transform has det < 0.
    assert!(t.is_negative());
}

/// Form detection: a pure rotation matrix → TrsfForm::Rotation.
#[test]
fn form_rotation_detected() {
    use std::f64::consts::FRAC_PI_4;
    let (s, c) = FRAC_PI_4.sin_cos();
    let mut t = GTrsf2d::identity();
    t.set_value(1, 1, c);
    t.set_value(1, 2, -s);
    t.set_value(2, 1, s);
    t.set_value(2, 2, c);
    // No translation → Rotation form.
    assert_eq!(t.form(), TrsfForm::Rotation);
    assert!(!t.is_negative());
}

/// Form detection: scale matrix → TrsfForm::Scale.
#[test]
fn form_scale_detected() {
    let mut t = GTrsf2d::identity();
    t.set_value(1, 1, 3.0);
    t.set_value(2, 2, 3.0);
    assert_eq!(t.form(), TrsfForm::Scale);
    assert!(!t.is_negative());
}
