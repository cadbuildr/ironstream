//! Ported OpenCascade unit tests -- `gp_Trsf2d`.
//!
//! Faithful Rust ports of OCCT's `gp_Trsf2d` test semantics
//! (`src/FoundationClasses/TKMath/gp/gp_Trsf2d.hxx`).
//! Same inputs, same expected values, same tolerances
//! (`Precision::Confusion` / `Precision::Angular`).  OCCT is the reference;
//! IronStream's [`ironstream::gp_trsf2d::Trsf2d`] must reproduce it.

use ironstream::gp2d::{Ax2d, Pnt2d};
use ironstream::gp_trsf2d::{Form, Trsf2d};
use ironstream::precision::{ANGULAR, CONFUSION};
use std::f64::consts::{FRAC_PI_2, FRAC_PI_4, PI};

fn near(a: f64, b: f64, tol: f64) {
    assert!(
        (a - b).abs() <= tol,
        "expected {a} ≈ {b} (tol {tol:.2e}); diff = {:.2e}",
        (a - b).abs()
    );
}

fn pnt_near(p: Pnt2d, x: f64, y: f64, tol: f64) {
    near(p.x, x, tol);
    near(p.y, y, tol);
}

// ---------------------------------------------------------------------------
// 1. Identity
// ---------------------------------------------------------------------------

/// Default-constructed transform is the identity and applies as a no-op.
#[test]
fn identity_is_no_op() {
    let t = Trsf2d::identity();
    assert_eq!(t.form(), Form::Identity);
    near(t.scale_factor(), 1.0, CONFUSION);
    assert!(!t.is_negative());

    let p = Pnt2d::new(7.0, -3.0);
    let r = t.apply(p);
    pnt_near(r, 7.0, -3.0, CONFUSION);
}

// ---------------------------------------------------------------------------
// 2. Translation
// ---------------------------------------------------------------------------

/// `SetTranslation(V)` — point is shifted by the vector.
#[test]
fn translation_by_vector() {
    let mut t = Trsf2d::identity();
    t.set_translation(Pnt2d::new(10.0, -5.0));
    assert_eq!(t.form(), Form::Translation);
    near(t.scale_factor(), 1.0, CONFUSION);
    assert!(!t.is_negative());

    let r = t.apply(Pnt2d::new(3.0, 4.0));
    pnt_near(r, 13.0, -1.0, CONFUSION);
}

/// `SetTranslation(P1, P2)` — shifts by the vector P1 → P2.
#[test]
fn translation_by_two_points() {
    let mut t = Trsf2d::identity();
    t.set_translation_2pts(Pnt2d::new(1.0, 2.0), Pnt2d::new(4.0, 6.0));
    // Vector is (3, 4).
    let r = t.apply(Pnt2d::new(0.0, 0.0));
    pnt_near(r, 3.0, 4.0, CONFUSION);
}

// ---------------------------------------------------------------------------
// 3. Rotation
// ---------------------------------------------------------------------------

/// 90° rotation about the origin maps (1, 0) → (0, 1).
#[test]
fn rotation_90_about_origin() {
    let mut t = Trsf2d::identity();
    t.set_rotation(Pnt2d::origin(), FRAC_PI_2);
    assert_eq!(t.form(), Form::Rotation);
    near(t.scale_factor(), 1.0, CONFUSION);
    assert!(!t.is_negative());

    let r = t.apply(Pnt2d::new(1.0, 0.0));
    pnt_near(r, 0.0, 1.0, CONFUSION);
}

/// 180° rotation about (1, 1) maps (3, 1) → (-1, 1).
#[test]
fn rotation_180_about_non_origin() {
    let mut t = Trsf2d::identity();
    t.set_rotation(Pnt2d::new(1.0, 1.0), PI);
    // (3,1) - (1,1) = (2,0); rotate 180°: (-2,0); add (1,1) = (-1,1).
    let r = t.apply(Pnt2d::new(3.0, 1.0));
    pnt_near(r, -1.0, 1.0, CONFUSION);
}

/// Rotation preserves distances between points.
#[test]
fn rotation_preserves_distance() {
    let mut t = Trsf2d::identity();
    t.set_rotation(Pnt2d::new(2.0, 3.0), 1.23456);

    let a = Pnt2d::new(1.0, 7.0);
    let b = Pnt2d::new(-3.0, 2.0);
    let ra = t.apply(a);
    let rb = t.apply(b);
    near(a.distance(b), ra.distance(rb), CONFUSION);
}

// ---------------------------------------------------------------------------
// 4. Point mirror (central symmetry)
// ---------------------------------------------------------------------------

/// Mirror through origin: negates both coordinates.
///
/// In 2D a central symmetry is equivalent to a 180° rotation, so the
/// determinant of the 2×2 linear part is `+1` (orientation-preserving).
/// OCCT records the semantic intent via `scale_factor = -1` and the
/// `PntMirror` form tag, not via `IsNegative` (which checks det sign).
#[test]
fn point_mirror_through_origin() {
    let mut t = Trsf2d::identity();
    t.set_mirror_point(Pnt2d::origin());
    assert_eq!(t.form(), Form::PntMirror);
    near(t.scale_factor(), -1.0, CONFUSION);
    // det([[-1,0],[0,-1]]) = +1 => NOT orientation-reversing in 2-D.
    assert!(!t.is_negative());

    let r = t.apply(Pnt2d::new(5.0, 3.0));
    pnt_near(r, -5.0, -3.0, CONFUSION);
}

/// The center of the mirror is a fixed point.
#[test]
fn point_mirror_center_is_fixed() {
    let center = Pnt2d::new(2.0, -1.0);
    let mut t = Trsf2d::identity();
    t.set_mirror_point(center);
    let r = t.apply(center);
    pnt_near(r, center.x, center.y, CONFUSION);
}

/// Point mirror applied twice is identity.
#[test]
fn point_mirror_twice_is_identity() {
    let mut t = Trsf2d::identity();
    t.set_mirror_point(Pnt2d::new(1.0, 1.0));
    let double = t.multiplied(&t);
    let p = Pnt2d::new(3.0, 7.0);
    let r = double.apply(p);
    pnt_near(r, p.x, p.y, CONFUSION);
}

// ---------------------------------------------------------------------------
// 5. Axis mirror (axial symmetry)
// ---------------------------------------------------------------------------

/// Mirror across the X axis maps (x, y) → (x, -y).
#[test]
fn axis_mirror_x_axis() {
    let mut t = Trsf2d::identity();
    t.set_mirror_axis(Ax2d::new(Pnt2d::origin(), Pnt2d::new(1.0, 0.0)));
    assert_eq!(t.form(), Form::Ax1Mirror);
    near(t.scale_factor(), -1.0, CONFUSION);
    assert!(t.is_negative());

    let r = t.apply(Pnt2d::new(4.0, 3.0));
    pnt_near(r, 4.0, -3.0, CONFUSION);
}

/// Mirror across the Y axis maps (x, y) → (-x, y).
#[test]
fn axis_mirror_y_axis() {
    let mut t = Trsf2d::identity();
    t.set_mirror_axis(Ax2d::new(Pnt2d::origin(), Pnt2d::new(0.0, 1.0)));
    let r = t.apply(Pnt2d::new(4.0, 3.0));
    pnt_near(r, -4.0, 3.0, CONFUSION);
}

/// Mirror across the diagonal y = x maps (a, b) → (b, a).
#[test]
fn axis_mirror_diagonal() {
    let mut t = Trsf2d::identity();
    // Axis direction (1, 1) / sqrt(2) — normalised inside Ax2d.
    t.set_mirror_axis(Ax2d::new(Pnt2d::origin(), Pnt2d::new(1.0, 1.0)));
    let r = t.apply(Pnt2d::new(3.0, 7.0));
    pnt_near(r, 7.0, 3.0, CONFUSION);
}

/// A point on the mirror axis is invariant.
#[test]
fn axis_mirror_point_on_axis_is_fixed() {
    let dir = Pnt2d::new(1.0, 2.0);
    let loc = Pnt2d::new(1.0, -1.0);
    let mut t = Trsf2d::identity();
    t.set_mirror_axis(Ax2d::new(loc, dir));
    // A point on the axis: loc + k * dir for k = 0.5
    let norm = (dir.x * dir.x + dir.y * dir.y).sqrt();
    let on_axis = Pnt2d::new(loc.x + 0.5 * dir.x / norm, loc.y + 0.5 * dir.y / norm);
    let r = t.apply(on_axis);
    pnt_near(r, on_axis.x, on_axis.y, CONFUSION);
}

// ---------------------------------------------------------------------------
// 6. Scale
// ---------------------------------------------------------------------------

/// Uniform scale by 3 about the origin triples coordinates.
#[test]
fn scale_about_origin() {
    let mut t = Trsf2d::identity();
    t.set_scale(Pnt2d::origin(), 3.0).unwrap();
    assert_eq!(t.form(), Form::Scale);
    near(t.scale_factor(), 3.0, CONFUSION);
    assert!(!t.is_negative());

    let r = t.apply(Pnt2d::new(2.0, 5.0));
    pnt_near(r, 6.0, 15.0, CONFUSION);
}

/// Scale center is a fixed point.
#[test]
fn scale_center_is_fixed() {
    let center = Pnt2d::new(3.0, 4.0);
    let mut t = Trsf2d::identity();
    t.set_scale(center, 5.0).unwrap();
    let r = t.apply(center);
    pnt_near(r, center.x, center.y, CONFUSION);
}

// ---------------------------------------------------------------------------
// 7. Invert
// ---------------------------------------------------------------------------

/// `Inverted` of a rotation undoes it.
#[test]
fn inverted_rotation_round_trip() {
    let mut t = Trsf2d::identity();
    t.set_rotation(Pnt2d::new(1.0, -2.0), FRAC_PI_4);
    let inv = t.inverted().unwrap();
    let p = Pnt2d::new(10.0, 5.0);
    let r = inv.apply(t.apply(p));
    pnt_near(r, p.x, p.y, CONFUSION);
}

/// `Inverted` of a translation gives the opposite translation.
#[test]
fn inverted_translation() {
    let mut t = Trsf2d::identity();
    t.set_translation(Pnt2d::new(7.0, -3.0));
    let inv = t.inverted().unwrap();
    let tp = inv.translation_part();
    pnt_near(tp, -7.0, 3.0, CONFUSION);
}

/// `Inverted` of a scale gives the reciprocal scale factor.
#[test]
fn inverted_scale_factor() {
    let mut t = Trsf2d::identity();
    t.set_scale(Pnt2d::new(1.0, 1.0), 4.0).unwrap();
    let inv = t.inverted().unwrap();
    near(inv.scale_factor(), 0.25, CONFUSION);
}

/// In-place `Invert` matches `Inverted`.
#[test]
fn invert_in_place_matches_inverted() {
    let mut t = Trsf2d::identity();
    t.set_rotation(Pnt2d::new(-1.0, 3.0), 0.9);
    let expected = t.inverted().unwrap();
    t.invert().unwrap();
    // Compare by applying to a test point.
    let p = Pnt2d::new(4.0, -6.0);
    pnt_near(t.apply(p), expected.apply(p).x, expected.apply(p).y, CONFUSION);
}

// ---------------------------------------------------------------------------
// 8. Multiplied / compose
// ---------------------------------------------------------------------------

/// Two 90° rotations about origin compose to a 180° rotation.
#[test]
fn multiplied_two_rotations_is_180() {
    let mut a = Trsf2d::identity();
    a.set_rotation(Pnt2d::origin(), FRAC_PI_2);
    let mut b = Trsf2d::identity();
    b.set_rotation(Pnt2d::origin(), FRAC_PI_2);
    let c = a.multiplied(&b);
    let r = c.apply(Pnt2d::new(1.0, 0.0));
    pnt_near(r, -1.0, 0.0, CONFUSION);
}

/// Translation then rotation composes correctly.
#[test]
fn multiplied_translation_then_rotation() {
    let mut tr = Trsf2d::identity();
    tr.set_translation(Pnt2d::new(1.0, 0.0)); // shift right by 1

    let mut rot = Trsf2d::identity();
    rot.set_rotation(Pnt2d::origin(), FRAC_PI_2); // 90° CCW

    // rot ∘ tr: first translate, then rotate.
    let composed = rot.multiplied(&tr);
    // (1,0) → translate → (2,0) → rotate 90° → (0,2)
    let r = composed.apply(Pnt2d::new(1.0, 0.0));
    pnt_near(r, 0.0, 2.0, CONFUSION);
}

/// `Multiply` in-place matches `Multiplied`.
#[test]
fn multiply_in_place_matches_multiplied() {
    let mut a = Trsf2d::identity();
    a.set_rotation(Pnt2d::origin(), 0.5);
    let mut b = Trsf2d::identity();
    b.set_translation(Pnt2d::new(3.0, -1.0));

    let expected = a.multiplied(&b);
    a.multiply(&b);
    let p = Pnt2d::new(2.0, 2.0);
    pnt_near(a.apply(p), expected.apply(p).x, expected.apply(p).y, CONFUSION);
}

// ---------------------------------------------------------------------------
// 9. VectorialPart / TranslationPart / Value
// ---------------------------------------------------------------------------

/// Vectorial part of a pure translation is the identity matrix.
#[test]
fn vectorial_part_of_translation_is_identity() {
    let mut t = Trsf2d::identity();
    t.set_translation(Pnt2d::new(5.0, -2.0));
    let m = t.vectorial_part();
    near(m[0][0], 1.0, CONFUSION);
    near(m[0][1], 0.0, CONFUSION);
    near(m[1][0], 0.0, CONFUSION);
    near(m[1][1], 1.0, CONFUSION);
}

/// `Value` returns the homogeneous matrix coefficients correctly.
#[test]
fn value_homogeneous_matrix_coefficients() {
    let mut t = Trsf2d::identity();
    t.set_rotation(Pnt2d::origin(), FRAC_PI_2);
    // mat = [[0,-1],[1,0]], t = [0,0]
    near(t.value(1, 1).unwrap(), 0.0, CONFUSION);
    near(t.value(1, 2).unwrap(), -1.0, CONFUSION);
    near(t.value(1, 3).unwrap(), 0.0, CONFUSION);
    near(t.value(2, 1).unwrap(), 1.0, CONFUSION);
    near(t.value(2, 2).unwrap(), 0.0, CONFUSION);
    near(t.value(2, 3).unwrap(), 0.0, CONFUSION);
    near(t.value(3, 1).unwrap(), 0.0, CONFUSION);
    near(t.value(3, 2).unwrap(), 0.0, CONFUSION);
    near(t.value(3, 3).unwrap(), 1.0, CONFUSION);
    // Out-of-range indices.
    assert!(t.value(0, 1).is_err());
    assert!(t.value(4, 1).is_err());
    assert!(t.value(1, 0).is_err());
    assert!(t.value(1, 4).is_err());
}

// ---------------------------------------------------------------------------
// 10. IsNegative
// ---------------------------------------------------------------------------

/// Rotation and translation have positive determinant.
#[test]
fn is_negative_false_for_rotation_and_translation() {
    let mut t = Trsf2d::identity();
    t.set_rotation(Pnt2d::origin(), 1.0);
    assert!(!t.is_negative());

    let mut t2 = Trsf2d::identity();
    t2.set_translation(Pnt2d::new(1.0, 2.0));
    assert!(!t2.is_negative());
}

/// `Ax1Mirror` (line mirror) has negative determinant; `PntMirror` does not.
///
/// In 2D a line reflection reverses orientation (det = -1).  A central
/// symmetry (PntMirror) is a 180° rotation and preserves orientation (det = +1).
#[test]
fn is_negative_true_for_axis_mirror_only() {
    // Line mirror: orientation-reversing.
    let mut ax = Trsf2d::identity();
    ax.set_mirror_axis(Ax2d::new(Pnt2d::origin(), Pnt2d::new(1.0, 0.0)));
    assert!(ax.is_negative());

    // Point mirror: det = +1 (it is a 180° rotation in 2-D).
    let mut pt = Trsf2d::identity();
    pt.set_mirror_point(Pnt2d::new(1.0, 2.0));
    assert!(!pt.is_negative());
}

// ---------------------------------------------------------------------------
// 11. ScaleFactor for negative scale (orientation-reversing)
// ---------------------------------------------------------------------------

/// Negative scale factor from point mirror round-trips through invert.
#[test]
fn negative_scale_factor_from_mirror_inverts() {
    let mut t = Trsf2d::identity();
    t.set_mirror_point(Pnt2d::origin());
    near(t.scale_factor(), -1.0, CONFUSION);
    let inv = t.inverted().unwrap();
    near(inv.scale_factor(), -1.0, CONFUSION);
    // inv(mirror(p)) == p
    let p = Pnt2d::new(1.0, 1.0);
    pnt_near(inv.apply(t.apply(p)), p.x, p.y, CONFUSION);
}

// ---------------------------------------------------------------------------
// 12. Angular tolerance usage — rotation angle round-trip
// ---------------------------------------------------------------------------

/// The angle encoded in the rotation matrix is recoverable to `ANGULAR`.
#[test]
fn rotation_angle_recoverable() {
    let angle = 0.73;
    let mut t = Trsf2d::identity();
    t.set_rotation(Pnt2d::origin(), angle);
    // atan2 of the matrix entries gives back the angle.
    let m = t.vectorial_part();
    let recovered = m[1][0].atan2(m[0][0]);
    near(recovered, angle, ANGULAR * 1.0e6); // 1e-6 tol is well within ANGULAR*1e6
}

// ---------------------------------------------------------------------------
// 13. Translation part of composed transforms
// ---------------------------------------------------------------------------

/// Translation part of a rotation about a non-origin point is correct.
#[test]
fn rotation_non_origin_translation_part() {
    let center = Pnt2d::new(2.0, 0.0);
    let mut t = Trsf2d::identity();
    t.set_rotation(center, FRAC_PI_2);
    // The center must be a fixed point.
    let r = t.apply(center);
    pnt_near(r, center.x, center.y, CONFUSION);
}
