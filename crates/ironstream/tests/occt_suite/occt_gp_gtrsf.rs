//! Ported OpenCascade unit tests — `gp_GTrsf`.
//!
//! Faithful Rust ports of OCCT's `gp_GTrsf` semantics.
//! Same inputs, same expected values, tolerances `Precision::Confusion`
//! / `Precision::Angular`.  OCCT is the reference;
//! IronStream's [`ironstream::gp_gtrsf::GTrsf`] must reproduce it.

use ironstream::gp::{Ax1, Ax3, Pnt, Trsf as GpTrsf};
use ironstream::gp_gtrsf::{GTrsf, GTrsfError, GTrsfForm};
use ironstream::precision::{ANGULAR, CONFUSION};
use std::f64::consts::{FRAC_PI_2, FRAC_PI_4};

// ─────────────────────────────────────────────────────────────────────────────
// Helpers
// ─────────────────────────────────────────────────────────────────────────────

fn near(a: f64, b: f64, tol: f64) {
    assert!(
        (a - b).abs() <= tol,
        "expected {a} ≈ {b} (tol {tol:.2e}); diff = {:.2e}",
        (a - b).abs()
    );
}

fn pnt_near(p: Pnt, x: f64, y: f64, z: f64, tol: f64) {
    near(p.x, x, tol);
    near(p.y, y, tol);
    near(p.z, z, tol);
}

fn mat_near(a: &[[f64; 3]; 3], b: &[[f64; 3]; 3], tol: f64) {
    for i in 0..3 {
        for j in 0..3 {
            near(a[i][j], b[i][j], tol);
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 1. Default (identity) constructor
// ─────────────────────────────────────────────────────────────────────────────

/// Default-constructed `GTrsf` is the identity: form = Identity, det = 1.
#[test]
fn default_is_identity() {
    let g = GTrsf::identity();
    assert_eq!(g.form(), GTrsfForm::Identity);
    assert!(!g.is_negative());

    // VectorialPart = I
    let vp = g.vectorial_part();
    mat_near(
        &vp,
        &[[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
        ANGULAR,
    );

    // TranslationPart = 0
    let tp = g.translation_part();
    pnt_near(tp, 0.0, 0.0, 0.0, CONFUSION);

    // Applies as no-op
    let p = Pnt::new(3.0, -1.0, 7.0);
    let r = g.transform_pnt(p);
    pnt_near(r, 3.0, -1.0, 7.0, CONFUSION);
}

// ─────────────────────────────────────────────────────────────────────────────
// 2. SetValue / Value (1-based indexing)
// ─────────────────────────────────────────────────────────────────────────────

/// `SetValue` and `Value` round-trip all 12 augmented-matrix coefficients.
#[test]
fn set_value_and_value_roundtrip() {
    let mut g = GTrsf::identity();
    // Fill with recognizable values.
    let mut expected = [[0.0f64; 3]; 3];
    for row in 1..=3usize {
        for col in 1..=4usize {
            let v = (row * 10 + col) as f64;
            g.set_value(row, col, v).unwrap();
            if col <= 3 {
                expected[row - 1][col - 1] = v;
            }
        }
    }
    for row in 1..=3usize {
        for col in 1..=4usize {
            let expected_v = (row * 10 + col) as f64;
            near(g.value(row, col).unwrap(), expected_v, ANGULAR);
        }
    }
    // Translation part via TranslationPart
    let tp = g.translation_part();
    near(tp.x, 14.0, CONFUSION);
    near(tp.y, 24.0, CONFUSION);
    near(tp.z, 34.0, CONFUSION);
    // VectorialPart via VectorialPart
    let vp = g.vectorial_part();
    mat_near(&vp, &expected, CONFUSION);
}

/// Out-of-range indices return `Err(OutOfRange)`.
#[test]
fn set_value_out_of_range() {
    let mut g = GTrsf::identity();
    assert_eq!(
        g.set_value(0, 1, 0.0),
        Err(GTrsfError::OutOfRange)
    );
    assert_eq!(
        g.set_value(4, 1, 0.0),
        Err(GTrsfError::OutOfRange)
    );
    assert_eq!(
        g.set_value(1, 5, 0.0),
        Err(GTrsfError::OutOfRange)
    );
    assert_eq!(
        g.value(3, 5),
        Err(GTrsfError::OutOfRange)
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// 3. SetTranslationPart / TranslationPart
// ─────────────────────────────────────────────────────────────────────────────

/// Setting the translation part changes form to Translation (when M = I).
#[test]
fn set_translation_part() {
    let mut g = GTrsf::identity();
    g.set_translation_part(Pnt::new(5.0, -3.0, 2.0));
    assert_eq!(g.form(), GTrsfForm::Translation);
    let tp = g.translation_part();
    pnt_near(tp, 5.0, -3.0, 2.0, CONFUSION);

    // Transform: pure translation
    let p = Pnt::new(1.0, 2.0, 3.0);
    let r = g.transform_pnt(p);
    pnt_near(r, 6.0, -1.0, 5.0, CONFUSION);
}

// ─────────────────────────────────────────────────────────────────────────────
// 4. SetVectorialPart / VectorialPart
// ─────────────────────────────────────────────────────────────────────────────

/// `SetVectorialPart` sets the 3×3 matrix; `VectorialPart` returns it.
#[test]
fn set_vectorial_part() {
    let m = [[2.0, 0.0, 0.0], [0.0, 3.0, 0.0], [0.0, 0.0, 4.0]];
    let mut g = GTrsf::identity();
    g.set_vectorial_part(m);
    mat_near(&g.vectorial_part(), &m, ANGULAR);
    // Non-uniform scale → Other form
    assert_eq!(g.form(), GTrsfForm::Other);
    // IsNegative: det = 24 > 0 → false
    assert!(!g.is_negative());
}

// ─────────────────────────────────────────────────────────────────────────────
// 5. IsNegative
// ─────────────────────────────────────────────────────────────────────────────

/// Negative determinant → `IsNegative` is true.
#[test]
fn is_negative_det() {
    let mut g = GTrsf::identity();
    // A mirror through the XY-plane: negate Z-component.
    g.set_vectorial_part([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, -1.0]]);
    assert!(g.is_negative());
    // det = -1 < 0
    let det = {
        let m = g.vectorial_part();
        m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1])
            - m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0])
            + m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0])
    };
    near(det, -1.0, CONFUSION);
}

// ─────────────────────────────────────────────────────────────────────────────
// 6. SetAffinity along Ax1 (line affinity)
// ─────────────────────────────────────────────────────────────────────────────

/// Axial affinity along Z with ratio 2: Z component doubles, X/Y unchanged.
#[test]
fn affinity_ax1_z_ratio2() {
    let axis = Ax1::new(Pnt::origin(), Pnt::new(0.0, 0.0, 1.0));
    let mut g = GTrsf::identity();
    g.set_affinity_ax1(axis, 2.0);

    // A point on the axis should be fixed.
    let on_axis = Pnt::new(0.0, 0.0, 5.0);
    let r = g.transform_pnt(on_axis);
    pnt_near(r, 0.0, 0.0, 5.0, CONFUSION);

    // A point off the axis: perpendicular distance should be scaled by ratio.
    // Point (3, 4, 5): distance to Z-axis = 5; after affinity dist = 5 * 2 = 10.
    // With Z-axis affinity (ratio 2): perp components scaled by 2, Z unchanged.
    // So (3,4,5) → (6, 8, 5).
    let p = Pnt::new(3.0, 4.0, 5.0);
    let rp = g.transform_pnt(p);
    pnt_near(rp, 6.0, 8.0, 5.0, CONFUSION);
}

/// Axial affinity with ratio 1.0 is the identity.
#[test]
fn affinity_ax1_ratio_one_is_identity() {
    let axis = Ax1::new(Pnt::new(1.0, 2.0, 3.0), Pnt::new(1.0, 0.0, 0.0));
    let mut g = GTrsf::identity();
    g.set_affinity_ax1(axis, 1.0);

    let p = Pnt::new(7.0, -2.0, 5.0);
    let r = g.transform_pnt(p);
    pnt_near(r, 7.0, -2.0, 5.0, CONFUSION);
}

// ─────────────────────────────────────────────────────────────────────────────
// 7. SetAffinity along Ax2 (plane affinity)
// ─────────────────────────────────────────────────────────────────────────────

/// Planar affinity: XY-plane (normal = Z), ratio 2.
/// Points in the plane unchanged; Z component scaled by ratio.
#[test]
fn affinity_ax2_xy_plane_ratio2() {
    // XY-plane = Ax3 with z_dir = (0,0,1), location at origin.
    let ax2 = Ax3 {
        location: Pnt::origin(),
        x_dir: Pnt::new(1.0, 0.0, 0.0),
        y_dir: Pnt::new(0.0, 1.0, 0.0),
        z_dir: Pnt::new(0.0, 0.0, 1.0),
    };
    let mut g = GTrsf::identity();
    g.set_affinity_ax2(ax2, 2.0);

    // Point in XY plane → unchanged.
    let in_plane = Pnt::new(3.0, 4.0, 0.0);
    let r = g.transform_pnt(in_plane);
    pnt_near(r, 3.0, 4.0, 0.0, CONFUSION);

    // Point above plane: (1, 2, 5) → (1, 2, 10).
    let p = Pnt::new(1.0, 2.0, 5.0);
    let rp = g.transform_pnt(p);
    pnt_near(rp, 1.0, 2.0, 10.0, CONFUSION);
}

/// Planar affinity with ratio 0 collapses to projection onto the plane.
#[test]
fn affinity_ax2_ratio_zero_projects() {
    let ax2 = Ax3 {
        location: Pnt::origin(),
        x_dir: Pnt::new(1.0, 0.0, 0.0),
        y_dir: Pnt::new(0.0, 1.0, 0.0),
        z_dir: Pnt::new(0.0, 0.0, 1.0),
    };
    let mut g = GTrsf::identity();
    g.set_affinity_ax2(ax2, 0.0);

    // Any point should project onto Z=0 plane.
    let p = Pnt::new(3.0, 4.0, 7.0);
    let r = g.transform_pnt(p);
    pnt_near(r, 3.0, 4.0, 0.0, CONFUSION);
}

// ─────────────────────────────────────────────────────────────────────────────
// 8. Inverted
// ─────────────────────────────────────────────────────────────────────────────

/// `Inverted()` of an identity is identity.
#[test]
fn inverted_identity() {
    let g = GTrsf::identity();
    let inv = g.inverted().unwrap();
    assert_eq!(inv.form(), GTrsfForm::Identity);
    let p = Pnt::new(5.0, -3.0, 2.0);
    let r = inv.transform_pnt(p);
    pnt_near(r, 5.0, -3.0, 2.0, CONFUSION);
}

/// `g * g.inverted() ≈ identity` for a non-trivial transform.
#[test]
fn inverted_compose_is_identity() {
    // Build a non-trivial GTrsf: uniform scale 3 + translation.
    let mut g = GTrsf::identity();
    g.set_vectorial_part([[3.0, 0.0, 0.0], [0.0, 3.0, 0.0], [0.0, 0.0, 3.0]]);
    g.set_translation_part(Pnt::new(1.0, -2.0, 5.0));

    let inv = g.inverted().unwrap();
    let composed = g.multiplied(&inv);

    // composed should be identity: mat ≈ I, loc ≈ 0.
    mat_near(
        &composed.vectorial_part(),
        &[[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
        CONFUSION,
    );
    pnt_near(composed.translation_part(), 0.0, 0.0, 0.0, CONFUSION);
}

/// Singular matrix returns error.
#[test]
fn inverted_singular_error() {
    let mut g = GTrsf::identity();
    // Rank-deficient matrix (all rows identical → det = 0).
    g.set_vectorial_part([[1.0, 2.0, 3.0], [1.0, 2.0, 3.0], [1.0, 2.0, 3.0]]);
    assert_eq!(g.inverted(), Err(GTrsfError::Singular));
}

// ─────────────────────────────────────────────────────────────────────────────
// 9. Multiplied / Multiply / PreMultiply
// ─────────────────────────────────────────────────────────────────────────────

/// `Multiplied`: compose two translations adds the vectors.
#[test]
fn multiplied_translations_add() {
    let mut a = GTrsf::identity();
    a.set_translation_part(Pnt::new(1.0, 2.0, 3.0));

    let mut b = GTrsf::identity();
    b.set_translation_part(Pnt::new(10.0, 20.0, 30.0));

    let c = a.multiplied(&b);
    let p = c.transform_pnt(Pnt::origin());
    pnt_near(p, 11.0, 22.0, 33.0, CONFUSION);
    assert_eq!(c.form(), GTrsfForm::Translation);
}

/// `PreMultiply(a)` is equivalent to `other = a * self`.
#[test]
fn pre_multiply() {
    let mut g = GTrsf::identity();
    g.set_translation_part(Pnt::new(1.0, 0.0, 0.0));

    let mut scale = GTrsf::identity();
    scale.set_vectorial_part([[2.0, 0.0, 0.0], [0.0, 2.0, 0.0], [0.0, 0.0, 2.0]]);

    // g.pre_multiply(scale)  ⟹  g = scale * g
    g.pre_multiply(&scale);

    // scale * t(1,0,0) = scale * [identity * p + (1,0,0)] * 2
    // scale maps p→2p, then adds 2*(1,0,0)=(2,0,0).
    let p = g.transform_pnt(Pnt::origin());
    pnt_near(p, 2.0, 0.0, 0.0, CONFUSION);
}

// ─────────────────────────────────────────────────────────────────────────────
// 10. Power
// ─────────────────────────────────────────────────────────────────────────────

/// `Power(0)` → identity.
#[test]
fn power_zero_is_identity() {
    let mut g = GTrsf::identity();
    g.set_translation_part(Pnt::new(1.0, 2.0, 3.0));
    let g0 = g.power(0).unwrap();
    assert_eq!(g0.form(), GTrsfForm::Identity);
    let r = g0.transform_pnt(Pnt::new(5.0, 5.0, 5.0));
    pnt_near(r, 5.0, 5.0, 5.0, CONFUSION);
}

/// `Power(n)` of a translation repeats n times.
#[test]
fn power_translation() {
    let mut g = GTrsf::identity();
    g.set_translation_part(Pnt::new(1.0, 0.0, 0.0));

    let g3 = g.power(3).unwrap();
    let r = g3.transform_pnt(Pnt::origin());
    pnt_near(r, 3.0, 0.0, 0.0, CONFUSION);

    let gm2 = g.power(-2).unwrap();
    let r2 = gm2.transform_pnt(Pnt::origin());
    pnt_near(r2, -2.0, 0.0, 0.0, CONFUSION);
}

/// `Power(n) * Power(-n) = identity`.
#[test]
fn power_inverse_compose() {
    let mut g = GTrsf::identity();
    g.set_vectorial_part([[2.0, 0.0, 0.0], [0.0, 3.0, 0.0], [0.0, 0.0, 4.0]]);
    g.set_translation_part(Pnt::new(1.0, -1.0, 2.0));

    let g2 = g.power(2).unwrap();
    let gm2 = g.power(-2).unwrap();
    let id = g2.multiplied(&gm2);

    mat_near(
        &id.vectorial_part(),
        &[[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
        CONFUSION,
    );
    pnt_near(id.translation_part(), 0.0, 0.0, 0.0, CONFUSION);
}

// ─────────────────────────────────────────────────────────────────────────────
// 11. to_gp_trsf — cast
// ─────────────────────────────────────────────────────────────────────────────

/// A GTrsf built from identity casts successfully to gp::Trsf.
#[test]
fn to_gp_trsf_identity() {
    let g = GTrsf::identity();
    let t = g.to_gp_trsf().unwrap();
    // gp::Trsf identity: m = I, t = 0.
    let p = t.apply_point(Pnt::new(3.0, -1.0, 7.0));
    pnt_near(p, 3.0, -1.0, 7.0, CONFUSION);
}

/// A GTrsf built from a rotation (via gp::Trsf) casts back correctly.
#[test]
fn to_gp_trsf_rotation() {
    // Build a rotation 90° about Z via gp::Trsf.
    let axis = Ax1::new(Pnt::origin(), Pnt::new(0.0, 0.0, 1.0));
    let gp = GpTrsf::rotation(axis, FRAC_PI_2);

    // Wrap in GTrsf and cast back.
    let g = GTrsf::from_gp_trsf(&gp);
    let t = g.to_gp_trsf().unwrap();

    // (1, 0, 0) rotated 90° about Z → (0, 1, 0).
    let p = t.apply_point(Pnt::new(1.0, 0.0, 0.0));
    pnt_near(p, 0.0, 1.0, 0.0, CONFUSION);
}

/// A shear GTrsf (non-orthogonal) cannot be cast to gp::Trsf.
#[test]
fn to_gp_trsf_shear_fails() {
    let mut g = GTrsf::identity();
    // Shear: x' = x + 0.5*y, y' = y, z' = z.
    g.set_vectorial_part([[1.0, 0.5, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]);
    match g.to_gp_trsf() {
        Err(GTrsfError::NotOrthogonal) => {} // expected
        other => panic!("expected NotOrthogonal, got {other:?}"),
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 12. Transforms (in-place)
// ─────────────────────────────────────────────────────────────────────────────

/// `Transforms` in-place matches `transform_pnt`.
#[test]
fn transforms_matches_transform_pnt() {
    let mut g = GTrsf::identity();
    g.set_vectorial_part([[1.0, 2.0, 0.0], [0.0, 1.0, 3.0], [0.0, 0.0, 1.0]]);
    g.set_translation_part(Pnt::new(10.0, 20.0, 30.0));

    let p = Pnt::new(1.0, 1.0, 1.0);

    let via_fn = g.transform_pnt(p);

    let mut x = p.x;
    let mut y = p.y;
    let mut z = p.z;
    g.transforms(&mut x, &mut y, &mut z);

    near(x, via_fn.x, CONFUSION);
    near(y, via_fn.y, CONFUSION);
    near(z, via_fn.z, CONFUSION);
}

// ─────────────────────────────────────────────────────────────────────────────
// 13. Form detection
// ─────────────────────────────────────────────────────────────────────────────

/// Form detection: rotation matrix without translation → Rotation.
#[test]
fn form_rotation() {
    let axis = Ax1::new(Pnt::origin(), Pnt::new(0.0, 0.0, 1.0));
    let gp = GpTrsf::rotation(axis, FRAC_PI_4);
    let g = GTrsf::from_gp_trsf(&gp);
    assert_eq!(g.form(), GTrsfForm::Rotation);
    assert!(!g.is_negative());
}

/// Form detection: pure scale → Scale.
#[test]
fn form_scale() {
    let mut g = GTrsf::identity();
    g.set_vectorial_part([[5.0, 0.0, 0.0], [0.0, 5.0, 0.0], [0.0, 0.0, 5.0]]);
    assert_eq!(g.form(), GTrsfForm::Scale);
    assert!(!g.is_negative());
}

/// Form detection: general affine (non-uniform scale) → Other.
#[test]
fn form_general_affine() {
    let mut g = GTrsf::identity();
    g.set_vectorial_part([[1.0, 0.5, 0.0], [0.5, 1.0, 0.0], [0.0, 0.0, 1.0]]);
    assert_eq!(g.form(), GTrsfForm::Other);
}
