//! Ported OpenCascade unit tests — `gp_Trsf`.
//!
//! Faithful Rust ports of OCCT's `gp_Trsf` test semantics.
//! Same inputs, same expected values, same tolerances
//! (`Precision::Confusion` / `Precision::Angular`).  OCCT is the reference;
//! IronStream's [`ironstream::gp_trsf::Trsf`] must reproduce it.

use ironstream::gp::{Ax1, Ax3, Pnt};
use ironstream::gp_trsf::{Form, Trsf, TrsfError};
use ironstream::precision::CONFUSION;
use std::f64::consts::{FRAC_PI_2, FRAC_PI_4, PI};

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

// ─────────────────────────────────────────────────────────────────────────────
// 1. Identity
// ─────────────────────────────────────────────────────────────────────────────

/// Default-constructed transform is the identity and applies as a no-op.
#[test]
fn identity_is_no_op() {
    let t = Trsf::identity();
    assert_eq!(t.form(), Form::Identity);
    near(t.scale_factor(), 1.0, CONFUSION);
    assert!(!t.is_negative());

    let p = Pnt::new(7.0, -3.0, 5.0);
    let r = t.apply(p);
    pnt_near(r, 7.0, -3.0, 5.0, CONFUSION);
}

// ─────────────────────────────────────────────────────────────────────────────
// 2. Translation
// ─────────────────────────────────────────────────────────────────────────────

/// `SetTranslation(Vec)` — point is shifted by the vector.
#[test]
fn translation_by_vector() {
    let mut t = Trsf::identity();
    t.set_translation(Pnt::new(10.0, -5.0, 3.0));
    assert_eq!(t.form(), Form::Translation);
    near(t.scale_factor(), 1.0, CONFUSION);
    assert!(!t.is_negative());

    let r = t.apply(Pnt::new(3.0, 4.0, 0.0));
    pnt_near(r, 13.0, -1.0, 3.0, CONFUSION);

    let tp = t.translation_part();
    pnt_near(tp, 10.0, -5.0, 3.0, CONFUSION);
}

/// `SetTranslation(P1, P2)` — translation from one point to another.
#[test]
fn translation_two_points() {
    let mut t = Trsf::identity();
    let p1 = Pnt::new(1.0, 2.0, 3.0);
    let p2 = Pnt::new(4.0, 6.0, 8.0);
    t.set_translation_2pts(p1, p2);
    // Should translate by (3, 4, 5)
    let r = t.apply(Pnt::new(0.0, 0.0, 0.0));
    pnt_near(r, 3.0, 4.0, 5.0, CONFUSION);
}

// ─────────────────────────────────────────────────────────────────────────────
// 3. Rotation
// ─────────────────────────────────────────────────────────────────────────────

/// `SetRotation(Ax1, angle)` — rotation 90° about Z through origin.
#[test]
fn rotation_90_about_z() {
    let mut t = Trsf::identity();
    let oz = Ax1::new(Pnt::origin(), Pnt::new(0.0, 0.0, 1.0));
    t.set_rotation(oz, FRAC_PI_2);
    assert_eq!(t.form(), Form::Rotation);
    near(t.scale_factor(), 1.0, CONFUSION);
    assert!(!t.is_negative());

    // +X maps to +Y
    let p = t.apply(Pnt::new(1.0, 0.0, 0.0));
    pnt_near(p, 0.0, 1.0, 0.0, CONFUSION);

    // +Y maps to -X
    let q = t.apply(Pnt::new(0.0, 1.0, 0.0));
    pnt_near(q, -1.0, 0.0, 0.0, CONFUSION);
}

/// Rotation about an off-origin axis: points on the axis are fixed.
#[test]
fn rotation_fixed_point_on_axis() {
    let mut t = Trsf::identity();
    let axis_pt = Pnt::new(1.0, 0.0, 0.0);
    let ax = Ax1::new(axis_pt, Pnt::new(0.0, 0.0, 1.0));
    t.set_rotation(ax, FRAC_PI_2);
    // The point on the axis itself should be fixed
    let fixed = t.apply(axis_pt);
    pnt_near(fixed, 1.0, 0.0, 0.0, CONFUSION);
}

// ─────────────────────────────────────────────────────────────────────────────
// 4. Mirror — point
// ─────────────────────────────────────────────────────────────────────────────

/// `SetMirror(Point)` — central symmetry negates displacement.
#[test]
fn mirror_point_central_symmetry() {
    let mut t = Trsf::identity();
    t.set_mirror_point(Pnt::origin());
    assert_eq!(t.form(), Form::PntMirror);
    near(t.scale_factor(), -1.0, CONFUSION);
    // Point mirror through origin has det = det(-I) = -1 ... but -1 in 3D:
    // det(-I_3x3) = (-1)^3 = -1, so is_negative = true.
    assert!(t.is_negative());

    let r = t.apply(Pnt::new(3.0, 4.0, 5.0));
    pnt_near(r, -3.0, -4.0, -5.0, CONFUSION);

    // Mirror about a non-origin point
    let mut t2 = Trsf::identity();
    t2.set_mirror_point(Pnt::new(1.0, 1.0, 1.0));
    let r2 = t2.apply(Pnt::new(3.0, 4.0, 5.0));
    pnt_near(r2, -1.0, -2.0, -3.0, CONFUSION);
}

// ─────────────────────────────────────────────────────────────────────────────
// 5. Mirror — axis (Ax1)
// ─────────────────────────────────────────────────────────────────────────────

/// `SetMirror(Ax1)` — reflection across the Z axis through origin.
/// Points on the axis are fixed; others are reflected.
#[test]
fn mirror_ax1_z_axis() {
    let mut t = Trsf::identity();
    let oz = Ax1::new(Pnt::origin(), Pnt::new(0.0, 0.0, 1.0));
    t.set_mirror_axis(oz);
    assert_eq!(t.form(), Form::Ax1Mirror);
    near(t.scale_factor(), -1.0, CONFUSION);

    // Point on the axis: fixed
    let on_axis = t.apply(Pnt::new(0.0, 0.0, 5.0));
    pnt_near(on_axis, 0.0, 0.0, 5.0, CONFUSION);

    // Point off the axis: x and y are negated, z preserved
    let off = t.apply(Pnt::new(2.0, 3.0, 4.0));
    pnt_near(off, -2.0, -3.0, 4.0, CONFUSION);
}

// ─────────────────────────────────────────────────────────────────────────────
// 6. Mirror — plane (Ax2)
// ─────────────────────────────────────────────────────────────────────────────

/// `SetMirror(Ax2)` — reflection across the XY plane (normal = +Z).
#[test]
fn mirror_ax2_xy_plane() {
    let mut t = Trsf::identity();
    let xy_plane = Ax3::identity();
    t.set_mirror_plane(xy_plane);
    assert_eq!(t.form(), Form::Ax2Mirror);
    near(t.scale_factor(), -1.0, CONFUSION);
    assert!(t.is_negative());

    // Points in XY plane are fixed
    let in_plane = t.apply(Pnt::new(3.0, 4.0, 0.0));
    pnt_near(in_plane, 3.0, 4.0, 0.0, CONFUSION);

    // Z component is negated
    let above = t.apply(Pnt::new(1.0, 2.0, 5.0));
    pnt_near(above, 1.0, 2.0, -5.0, CONFUSION);
}

// ─────────────────────────────────────────────────────────────────────────────
// 7. Scale
// ─────────────────────────────────────────────────────────────────────────────

/// `SetScale(center, factor)` — uniform scaling.
#[test]
fn scale_about_center() {
    let mut t = Trsf::identity();
    let center = Pnt::new(1.0, 1.0, 1.0);
    t.set_scale(center, 3.0).unwrap();
    assert_eq!(t.form(), Form::Scale);
    near(t.scale_factor(), 3.0, CONFUSION);
    assert!(!t.is_negative());

    // Center maps to itself
    let c = t.apply(center);
    pnt_near(c, 1.0, 1.0, 1.0, CONFUSION);

    // Another point
    let p = t.apply(Pnt::new(2.0, 2.0, 2.0));
    pnt_near(p, 4.0, 4.0, 4.0, CONFUSION);

    // Zero scale is an error
    let mut t2 = Trsf::identity();
    assert_eq!(t2.set_scale(Pnt::origin(), 0.0), Err(TrsfError::ZeroScale));
}

// ─────────────────────────────────────────────────────────────────────────────
// 8. Value (1-based index access)
// ─────────────────────────────────────────────────────────────────────────────

/// `Value(row, col)` — augmented 4×4 homogeneous matrix access.
#[test]
fn value_indexing() {
    let mut t = Trsf::identity();
    let oz = Ax1::new(Pnt::origin(), Pnt::new(0.0, 0.0, 1.0));
    t.set_rotation(oz, FRAC_PI_2);
    // mat = [[0, -1, 0], [1, 0, 0], [0, 0, 1]]  (approx)
    near(t.value(1, 1).unwrap(), 0.0, CONFUSION);
    near(t.value(1, 2).unwrap(), -1.0, CONFUSION);
    near(t.value(2, 1).unwrap(), 1.0, CONFUSION);
    near(t.value(2, 2).unwrap(), 0.0, CONFUSION);
    near(t.value(3, 3).unwrap(), 1.0, CONFUSION);
    // Translation column
    near(t.value(1, 4).unwrap(), 0.0, CONFUSION);
    near(t.value(2, 4).unwrap(), 0.0, CONFUSION);
    near(t.value(3, 4).unwrap(), 0.0, CONFUSION);
    // Homogeneous row
    near(t.value(4, 1).unwrap(), 0.0, CONFUSION);
    near(t.value(4, 4).unwrap(), 1.0, CONFUSION);
    // Out-of-range
    assert!(t.value(0, 1).is_err());
    assert!(t.value(5, 1).is_err());
    assert!(t.value(1, 5).is_err());
}

// ─────────────────────────────────────────────────────────────────────────────
// 9. Invert / Inverted
// ─────────────────────────────────────────────────────────────────────────────

/// Invert a rotation: round-trip gives identity.
#[test]
fn invert_rotation_roundtrip() {
    let mut t = Trsf::identity();
    let ax = Ax1::new(Pnt::new(1.0, 2.0, 3.0), Pnt::new(1.0, 1.0, 1.0));
    t.set_rotation(ax, PI / 6.0);

    let inv = t.inverted().unwrap();
    let p = Pnt::new(5.0, -2.0, 7.0);
    let roundtrip = inv.apply(t.apply(p));
    pnt_near(roundtrip, p.x, p.y, p.z, CONFUSION);
}

/// Invert a scale.
#[test]
fn invert_scale() {
    let mut t = Trsf::identity();
    t.set_scale(Pnt::origin(), 4.0).unwrap();
    let inv = t.inverted().unwrap();
    near(inv.scale_factor(), 0.25, CONFUSION);
    let p = Pnt::new(1.0, 2.0, 3.0);
    let rr = inv.apply(t.apply(p));
    pnt_near(rr, p.x, p.y, p.z, CONFUSION);
}

/// Invert a translation.
#[test]
fn invert_translation() {
    let mut t = Trsf::identity();
    t.set_translation(Pnt::new(3.0, -1.0, 7.0));
    let inv = t.inverted().unwrap();
    let p = Pnt::new(0.0, 0.0, 0.0);
    let rr = inv.apply(t.apply(p));
    pnt_near(rr, 0.0, 0.0, 0.0, CONFUSION);
}

// ─────────────────────────────────────────────────────────────────────────────
// 10. Multiplied / Multiply / PreMultiply
// ─────────────────────────────────────────────────────────────────────────────

/// Composing two 90° rotations about Z gives 180°.
#[test]
fn multiply_two_rotations() {
    let oz = Ax1::new(Pnt::origin(), Pnt::new(0.0, 0.0, 1.0));
    let mut a = Trsf::identity();
    a.set_rotation(oz, FRAC_PI_2);
    let mut b = Trsf::identity();
    b.set_rotation(oz, FRAC_PI_2);
    let c = a.multiplied(&b); // 90° then 90° = 180°
    let r = c.apply(Pnt::new(1.0, 0.0, 0.0));
    pnt_near(r, -1.0, 0.0, 0.0, CONFUSION);
}

/// `Multiply` is in-place `self = self ∘ other`.
#[test]
fn multiply_in_place() {
    let oz = Ax1::new(Pnt::origin(), Pnt::new(0.0, 0.0, 1.0));
    let mut t = Trsf::identity();
    t.set_rotation(oz, FRAC_PI_2);
    let t_orig = t;
    let mut t2 = Trsf::identity();
    t2.set_rotation(oz, FRAC_PI_2);
    t.multiply(&t2);
    // Equivalent to multiplied
    let expected = t_orig.multiplied(&t2);
    let p = Pnt::new(1.0, 0.0, 0.0);
    pnt_near(t.apply(p), expected.apply(p).x, expected.apply(p).y, expected.apply(p).z, CONFUSION);
}

/// `PreMultiply(other)` — in-place `self = other ∘ self`.
#[test]
fn pre_multiply() {
    let oz = Ax1::new(Pnt::origin(), Pnt::new(0.0, 0.0, 1.0));
    let mut rot90 = Trsf::identity();
    rot90.set_rotation(oz, FRAC_PI_2);
    let mut trans = Trsf::identity();
    trans.set_translation(Pnt::new(1.0, 0.0, 0.0));

    // trans.pre_multiply(rot90) => rot90 ∘ trans
    let mut t = trans;
    t.pre_multiply(&rot90);

    // Expected: rot90 ∘ trans (first translate then rotate)
    let expected = rot90.multiplied(&trans);
    let p = Pnt::new(0.0, 0.0, 0.0);
    let r1 = t.apply(p);
    let r2 = expected.apply(p);
    pnt_near(r1, r2.x, r2.y, r2.z, CONFUSION);
}

// ─────────────────────────────────────────────────────────────────────────────
// 11. Power
// ─────────────────────────────────────────────────────────────────────────────

/// `Power(0)` gives identity; `Power(n)` repeats.
#[test]
fn power_rotation() {
    let oz = Ax1::new(Pnt::origin(), Pnt::new(0.0, 0.0, 1.0));
    let mut t = Trsf::identity();
    t.set_rotation(oz, FRAC_PI_4); // 45°

    // Power(0) = identity
    let p0 = t.powered(0).unwrap();
    let p = Pnt::new(1.0, 0.0, 0.0);
    pnt_near(p0.apply(p), 1.0, 0.0, 0.0, CONFUSION);

    // Power(4) = 4 × 45° = 180° → x maps to -x
    let p4 = t.powered(4).unwrap();
    pnt_near(p4.apply(p), -1.0, 0.0, 0.0, CONFUSION);

    // Power(-1) = inverse: maps back
    let pm1 = t.powered(-1).unwrap();
    let q = Pnt::new(0.0, 1.0, 0.0);
    let rr = pm1.apply(t.apply(q));
    pnt_near(rr, 0.0, 1.0, 0.0, CONFUSION);
}

// ─────────────────────────────────────────────────────────────────────────────
// 12. GetRotation (quaternion extraction)
// ─────────────────────────────────────────────────────────────────────────────

/// `GetRotation` — identity gives `(w=1, x=0, y=0, z=0)`.
#[test]
fn get_rotation_identity() {
    let t = Trsf::identity();
    let q = t.get_rotation().unwrap();
    near(q[0].abs(), 1.0, CONFUSION); // w = ±1
    near(q[1].abs(), 0.0, CONFUSION);
    near(q[2].abs(), 0.0, CONFUSION);
    near(q[3].abs(), 0.0, CONFUSION);
}

/// `GetRotation` from a 90° rotation about Z gives expected quaternion.
#[test]
fn get_rotation_90_z() {
    let mut t = Trsf::identity();
    let oz = Ax1::new(Pnt::origin(), Pnt::new(0.0, 0.0, 1.0));
    t.set_rotation(oz, FRAC_PI_2);
    let q = t.get_rotation().unwrap();
    // Expected: w = cos(45°) = 1/√2, x=0, y=0, z = sin(45°) = 1/√2
    let expected_w = FRAC_PI_4.cos();
    let expected_z = FRAC_PI_4.sin();
    near(q[0].abs(), expected_w, CONFUSION);
    near(q[3].abs(), expected_z, CONFUSION);
}

// ─────────────────────────────────────────────────────────────────────────────
// 13. IsNegative
// ─────────────────────────────────────────────────────────────────────────────

/// Orientation-reversing vs. orientation-preserving checks.
#[test]
fn is_negative_checks() {
    // Identity: positive
    assert!(!Trsf::identity().is_negative());

    // Rotation: positive
    let mut rot = Trsf::identity();
    rot.set_rotation(
        Ax1::new(Pnt::origin(), Pnt::new(0.0, 0.0, 1.0)),
        PI / 3.0,
    );
    assert!(!rot.is_negative());

    // Plane mirror: negative
    let mut pm = Trsf::identity();
    pm.set_mirror_plane(Ax3::identity());
    assert!(pm.is_negative());

    // Axis mirror: NOT orientation-reversing.
    // The matrix is 2*d*d^T - I.  For d = (0,0,1): diag(-1,-1,1), det = +1.
    // (An Ax1Mirror is geometrically a rotation by 180° about the axis.)
    let mut am = Trsf::identity();
    am.set_mirror_axis(Ax1::new(Pnt::origin(), Pnt::new(0.0, 0.0, 1.0)));
    assert!(!am.is_negative());

    // Point mirror: negative in 3-D (det(-I) = -1)
    let mut cm = Trsf::identity();
    cm.set_mirror_point(Pnt::origin());
    assert!(cm.is_negative());
}

// ─────────────────────────────────────────────────────────────────────────────
// 14. VectorialPart
// ─────────────────────────────────────────────────────────────────────────────

/// `VectorialPart` returns the 3×3 linear matrix.
#[test]
fn vectorial_part_translation_is_identity_mat() {
    let mut t = Trsf::identity();
    t.set_translation(Pnt::new(5.0, 3.0, -1.0));
    let vp = t.vectorial_part();
    // Linear part of pure translation is identity
    for (i, row) in vp.iter().enumerate() {
        for (j, &val) in row.iter().enumerate() {
            let expected = if i == j { 1.0 } else { 0.0 };
            near(val, expected, CONFUSION);
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 15. Transforms (in-place triple)
// ─────────────────────────────────────────────────────────────────────────────

/// `Transforms(x, y, z)` modifies coordinates in-place.
#[test]
fn transforms_in_place() {
    let mut t = Trsf::identity();
    t.set_translation(Pnt::new(1.0, 2.0, 3.0));
    let mut x = 0.0_f64;
    let mut y = 0.0_f64;
    let mut z = 0.0_f64;
    t.transforms(&mut x, &mut y, &mut z);
    near(x, 1.0, CONFUSION);
    near(y, 2.0, CONFUSION);
    near(z, 3.0, CONFUSION);
}

// ─────────────────────────────────────────────────────────────────────────────
// 16. SetDisplacementOfCS
// ─────────────────────────────────────────────────────────────────────────────

/// Displacement transform maps points from one frame to another.
#[test]
fn displacement_of_cs() {
    // fromCS = identity (world frame)
    let from_cs = Ax3::identity();
    // toCS = translated along X by 5
    let to_cs = Ax3 {
        location: Pnt::new(5.0, 0.0, 0.0),
        x_dir: Pnt::new(1.0, 0.0, 0.0),
        y_dir: Pnt::new(0.0, 1.0, 0.0),
        z_dir: Pnt::new(0.0, 0.0, 1.0),
    };
    let mut t = Trsf::identity();
    t.set_displacement_of_cs(from_cs, to_cs).unwrap();
    // A point at origin in world space should map to... let's verify the transform
    // is self-consistent: applying T to (0,0,0) gives... the location shift.
    let p = t.apply(Pnt::new(0.0, 0.0, 0.0));
    // The displacement transform maps fromCS origin to toCS origin
    near(p.x, 5.0, CONFUSION);
    near(p.y, 0.0, CONFUSION);
    near(p.z, 0.0, CONFUSION);
}

// ─────────────────────────────────────────────────────────────────────────────
// 17. Composition: mirror ∘ mirror = rotation
// ─────────────────────────────────────────────────────────────────────────────

/// Two reflections across planes intersecting at an angle θ produce a rotation
/// of 2θ about their intersection line.
#[test]
fn two_plane_mirrors_give_rotation() {
    // XY-plane (normal +Z)
    let xy = Ax3::identity();
    let mut m1 = Trsf::identity();
    m1.set_mirror_plane(xy);

    // YZ-plane (normal +X, x_dir=+Y, y_dir=+Z in OCCT convention — but we
    // just need the plane, which is: origin at (0,0,0), normal=(1,0,0))
    let yz = Ax3 {
        location: Pnt::origin(),
        x_dir: Pnt::new(0.0, 1.0, 0.0),
        y_dir: Pnt::new(0.0, 0.0, 1.0),
        z_dir: Pnt::new(1.0, 0.0, 0.0),
    };
    let mut m2 = Trsf::identity();
    m2.set_mirror_plane(yz);

    // Compose: m2 ∘ m1 = rotation by 2*90° = 180° about Z∩X intersection line (=Y axis)
    let composed = m2.multiplied(&m1);
    // Applying to (1, 0, 0): XY mirror → (1,0,0), YZ mirror → (-1,0,0).
    let r = composed.apply(Pnt::new(1.0, 0.0, 0.0));
    pnt_near(r, -1.0, 0.0, 0.0, CONFUSION);
    // The composed transform should be orientation-preserving (product of two reflections)
    assert!(!composed.is_negative());
}
