//! Ported OpenCascade unit tests — `gp_Dir`.
//!
//! Source: OpenCascade Technology, `TKMath` package, `gp_Dir` class tests
//! (ported by hand). Each `#[test]` corresponds to one logical scenario from
//! the OCCT test suite for `gp_Dir`.
//!
//! Tolerances used:
//! * linear:  `precision::CONFUSION` = 1e-7
//! * angular: `precision::ANGULAR`   = 1e-12

use ironstream::gp::{Ax1, Ax3, Pnt, Trsf};
use ironstream::gp_dir::Dir;
use ironstream::precision::{ANGULAR, CONFUSION};
use std::f64::consts::{FRAC_PI_2, FRAC_PI_4, FRAC_PI_6, PI};

/// `EXPECT_NEAR` helper — panics if `|a - b| > tol`.
fn near(a: f64, b: f64, tol: f64) {
    assert!(
        (a - b).abs() <= tol,
        "expected {a} ~= {b} within {tol} (diff {})",
        (a - b).abs()
    );
}

// ── 1. Constructor: (X,Y,Z) normalizes the vector ────────────────────────────

#[test]
fn construct_from_coords_normalizes() {
    // gp_Dir d(3, 0, 4); magnitude 5 → unit (0.6, 0.0, 0.8)
    let d = Dir::new(3.0, 0.0, 4.0);
    near(d.x(), 0.6, CONFUSION);
    near(d.y(), 0.0, CONFUSION);
    near(d.z(), 0.8, CONFUSION);
    let mag = (d.x() * d.x() + d.y() * d.y() + d.z() * d.z()).sqrt();
    near(mag, 1.0, CONFUSION);
}

// ── 2. Constructor: a unit vector is preserved unchanged ─────────────────────

#[test]
fn construct_unit_x_preserved() {
    let d = Dir::new(1.0, 0.0, 0.0);
    near(d.x(), 1.0, ANGULAR);
    near(d.y(), 0.0, ANGULAR);
    near(d.z(), 0.0, ANGULAR);
}

// ── 3. from_vec round-trip ────────────────────────────────────────────────────

#[test]
fn from_vec_round_trip() {
    let v = Pnt::new(0.0, 3.0, 4.0);
    let d = Dir::from_vec(v);
    near(d.x(), 0.0, CONFUSION);
    near(d.y(), 0.6, CONFUSION);
    near(d.z(), 0.8, CONFUSION);
    let back = d.as_vec();
    near(back.x, d.x(), ANGULAR);
    near(back.y, d.y(), ANGULAR);
    near(back.z, d.z(), ANGULAR);
}

// ── 4. SetCoord replaces and normalizes ──────────────────────────────────────

#[test]
fn set_coord_replaces_and_normalizes() {
    let mut d = Dir::new(1.0, 0.0, 0.0);
    d.set_coord(0.0, 0.0, 7.0);
    near(d.x(), 0.0, CONFUSION);
    near(d.y(), 0.0, CONFUSION);
    near(d.z(), 1.0, CONFUSION);
}

// ── 5. coord() tuple matches individual accessors ─────────────────────────────

#[test]
fn coord_tuple_matches_accessors() {
    let d = Dir::new(1.0, 2.0, 2.0);
    let (x, y, z) = d.coord();
    near(x, d.x(), ANGULAR);
    near(y, d.y(), ANGULAR);
    near(z, d.z(), ANGULAR);
}

// ── 6. Dot: same direction → cos(0°) = 1 ────────────────────────────────────

#[test]
fn dot_same_direction_is_one() {
    let d = Dir::new(1.0, 0.0, 0.0);
    near(d.dot(&d), 1.0, CONFUSION);
}

// ── 7. Dot: perpendicular → cos(90°) = 0 ────────────────────────────────────

#[test]
fn dot_perpendicular_is_zero() {
    let dx = Dir::new(1.0, 0.0, 0.0);
    let dy = Dir::new(0.0, 1.0, 0.0);
    near(dx.dot(&dy), 0.0, CONFUSION);
}

// ── 8. Dot: opposite direction → -1 ─────────────────────────────────────────

#[test]
fn dot_opposite_is_minus_one() {
    let d = Dir::new(1.0, 0.0, 0.0);
    let neg = Dir::new(-1.0, 0.0, 0.0);
    near(d.dot(&neg), -1.0, CONFUSION);
}

// ── 9. Crossed: perpendicular axes give a unit Z vector ──────────────────────

#[test]
fn crossed_x_y_gives_z() {
    let dx = Dir::new(1.0, 0.0, 0.0);
    let dy = Dir::new(0.0, 1.0, 0.0);
    let c = dx.crossed(&dy);
    near(c.norm(), 1.0, CONFUSION);
    near(c.x, 0.0, CONFUSION);
    near(c.y, 0.0, CONFUSION);
    near(c.z, 1.0, CONFUSION);
}

// ── 10. Crossed: parallel directions give zero vector ────────────────────────

#[test]
fn crossed_parallel_is_zero() {
    let dx = Dir::new(1.0, 0.0, 0.0);
    let dx2 = Dir::new(2.0, 0.0, 0.0); // normalizes to (1,0,0)
    let c = dx.crossed(&dx2);
    near(c.norm(), 0.0, CONFUSION);
}

// ── 11. Crossed is antisymmetric ─────────────────────────────────────────────

#[test]
fn crossed_antisymmetric() {
    let a = Dir::new(1.0, 0.0, 0.0);
    let b = Dir::new(0.0, 1.0, 0.0);
    let ab = a.crossed(&b);
    let ba = b.crossed(&a);
    near(ab.x, -ba.x, ANGULAR);
    near(ab.y, -ba.y, ANGULAR);
    near(ab.z, -ba.z, ANGULAR);
}

// ── 12. CrossMagnitude: perpendicular → sin(90°) = 1 ────────────────────────

#[test]
fn cross_magnitude_perpendicular() {
    let dx = Dir::new(1.0, 0.0, 0.0);
    let dy = Dir::new(0.0, 1.0, 0.0);
    near(dx.cross_magnitude(&dy), 1.0, CONFUSION);
}

// ── 13. CrossSquareMagnitude: perpendicular → 1 ──────────────────────────────

#[test]
fn cross_square_magnitude_perpendicular() {
    let dx = Dir::new(1.0, 0.0, 0.0);
    let dy = Dir::new(0.0, 1.0, 0.0);
    near(dx.cross_square_magnitude(&dy), 1.0, CONFUSION);
}

// ── 14. CrossMagnitude + CrossSquareMagnitude at 45° ─────────────────────────

#[test]
fn cross_magnitude_45_degrees() {
    let dx = Dir::new(1.0, 0.0, 0.0);
    let d45 = Dir::new(1.0, 1.0, 0.0); // normalizes to (1/√2, 1/√2, 0)
    let expected_sin = FRAC_PI_4.sin(); // sin(45°) = 1/√2
    near(dx.cross_magnitude(&d45), expected_sin, CONFUSION);
    near(dx.cross_square_magnitude(&d45), expected_sin * expected_sin, CONFUSION);
}

// ── 15. Angle: 90° between X and Y ──────────────────────────────────────────

#[test]
fn angle_90_degrees() {
    let dx = Dir::new(1.0, 0.0, 0.0);
    let dy = Dir::new(0.0, 1.0, 0.0);
    near(dx.angle(&dy), FRAC_PI_2, ANGULAR);
}

// ── 16. Angle: 45° ───────────────────────────────────────────────────────────

#[test]
fn angle_45_degrees() {
    let dx = Dir::new(1.0, 0.0, 0.0);
    let d45 = Dir::new(1.0, 1.0, 0.0);
    near(dx.angle(&d45), FRAC_PI_4, CONFUSION);
}

// ── 17. Angle: 30° ───────────────────────────────────────────────────────────

#[test]
fn angle_30_degrees() {
    let dx = Dir::new(1.0, 0.0, 0.0);
    let d30 = Dir::new(3_f64.sqrt(), 1.0, 0.0);
    near(dx.angle(&d30), FRAC_PI_6, CONFUSION);
}

// ── 18. Angle: antiparallel → π ──────────────────────────────────────────────

#[test]
fn angle_antiparallel_is_pi() {
    let dx = Dir::new(1.0, 0.0, 0.0);
    let neg = Dir::new(-1.0, 0.0, 0.0);
    near(dx.angle(&neg), PI, CONFUSION);
}

// ── 19. Angle: self → self = 0 ───────────────────────────────────────────────

#[test]
fn angle_to_self_is_zero() {
    let d = Dir::new(1.0, 1.0, 1.0);
    near(d.angle(&d), 0.0, ANGULAR);
}

// ── 20. Angle is symmetric (unsigned) ────────────────────────────────────────

#[test]
fn angle_is_symmetric() {
    let a = Dir::new(1.0, 0.0, 0.0);
    let b = Dir::new(0.0, 1.0, 0.0);
    near(a.angle(&b), b.angle(&a), ANGULAR);
}

// ── 21. IsEqual ──────────────────────────────────────────────────────────────

#[test]
fn is_equal_same_direction() {
    let d = Dir::new(1.0, 0.0, 0.0);
    assert!(d.is_equal(&d, 1e-9));
    let dy = Dir::new(0.0, 1.0, 0.0);
    assert!(!d.is_equal(&dy, 1e-9));
}

// ── 22. IsNormal ─────────────────────────────────────────────────────────────

#[test]
fn is_normal_perpendicular() {
    let dx = Dir::new(1.0, 0.0, 0.0);
    let dy = Dir::new(0.0, 1.0, 0.0);
    assert!(dx.is_normal(&dy, 1e-9));
    assert!(!dx.is_normal(&dx, 1e-9));
}

// ── 23. IsOpposite ───────────────────────────────────────────────────────────

#[test]
fn is_opposite_antiparallel() {
    let d = Dir::new(1.0, 0.0, 0.0);
    let neg = Dir::new(-1.0, 0.0, 0.0);
    assert!(d.is_opposite(&neg, 1e-9));
    assert!(!d.is_opposite(&d, 1e-9));
}

// ── 24. IsParallel: same and antiparallel are parallel ───────────────────────

#[test]
fn is_parallel_same_and_antiparallel() {
    let d = Dir::new(1.0, 0.0, 0.0);
    let neg = Dir::new(-1.0, 0.0, 0.0);
    assert!(d.is_parallel(&d, 1e-9));
    assert!(d.is_parallel(&neg, 1e-9));
    let dy = Dir::new(0.0, 1.0, 0.0);
    assert!(!d.is_parallel(&dy, 1e-9));
}

// ── 25. AngleWithRef: signed angle X→Y about Z is +π/2 ──────────────────────

#[test]
fn angle_with_ref_positive() {
    let dx = Dir::new(1.0, 0.0, 0.0);
    let dy = Dir::new(0.0, 1.0, 0.0);
    let dz = Dir::new(0.0, 0.0, 1.0);
    let a = dx.angle_with_ref(&dy, &dz);
    near(a, FRAC_PI_2, CONFUSION);
}

// ── 26. AngleWithRef: signed angle Y→X about Z is -π/2 ──────────────────────

#[test]
fn angle_with_ref_negative() {
    let dx = Dir::new(1.0, 0.0, 0.0);
    let dy = Dir::new(0.0, 1.0, 0.0);
    let dz = Dir::new(0.0, 0.0, 1.0);
    let a = dy.angle_with_ref(&dx, &dz);
    near(a, -FRAC_PI_2, CONFUSION);
}

// ── 27. Reversed (non-mutating) ──────────────────────────────────────────────

#[test]
fn reversed_non_mutating() {
    let original = Dir::new(1.0, 0.0, 0.0);
    let r = original.reversed();
    near(r.x(), -1.0, CONFUSION);
    near(r.y(), 0.0, CONFUSION);
    near(r.z(), 0.0, CONFUSION);
    // original unchanged
    near(original.x(), 1.0, CONFUSION);
}

// ── 28. Reverse (in-place) ────────────────────────────────────────────────────

#[test]
fn reverse_in_place() {
    let mut d = Dir::new(0.0, 0.0, 1.0);
    d.reverse();
    near(d.x(), 0.0, CONFUSION);
    near(d.y(), 0.0, CONFUSION);
    near(d.z(), -1.0, CONFUSION);
}

// ── 29. Double reverse returns to original ───────────────────────────────────

#[test]
fn double_reverse_identity() {
    let original = Dir::new(3.0, 0.0, 4.0);
    let mut d = original;
    d.reverse();
    d.reverse();
    near(d.x(), original.x(), ANGULAR);
    near(d.y(), original.y(), ANGULAR);
    near(d.z(), original.z(), ANGULAR);
}

// ── 30. Rotated: 90° about Z maps X → Y ─────────────────────────────────────

#[test]
fn rotated_90_about_z() {
    let d = Dir::new(1.0, 0.0, 0.0);
    let axis = Ax1::new(Pnt::origin(), Pnt::new(0.0, 0.0, 1.0));
    let r = d.rotated(axis, FRAC_PI_2);
    near(r.x(), 0.0, CONFUSION);
    near(r.y(), 1.0, CONFUSION);
    near(r.z(), 0.0, CONFUSION);
}

// ── 31. Rotated: 180° about Z reverses X ────────────────────────────────────

#[test]
fn rotated_180_about_z_reverses_x() {
    let d = Dir::new(1.0, 0.0, 0.0);
    let axis = Ax1::new(Pnt::origin(), Pnt::new(0.0, 0.0, 1.0));
    let r = d.rotated(axis, PI);
    near(r.x(), -1.0, CONFUSION);
    near(r.y(), 0.0, CONFUSION);
    near(r.z(), 0.0, CONFUSION);
}

// ── 32. Transformed: 90° rotation about Z ───────────────────────────────────

#[test]
fn transformed_rotation_90_about_z() {
    let d = Dir::new(1.0, 0.0, 0.0);
    let axis = Ax1::new(Pnt::origin(), Pnt::new(0.0, 0.0, 1.0));
    let t = Trsf::rotation(axis, FRAC_PI_2);
    let r = d.transformed(&t);
    near(r.x(), 0.0, CONFUSION);
    near(r.y(), 1.0, CONFUSION);
    near(r.z(), 0.0, CONFUSION);
}

// ── 33. Mirrored about a point (equivalent to reverse) ───────────────────────

#[test]
fn mirrored_point_reverses() {
    let d = Dir::new(1.0, 2.0, 3.0);
    let m = d.mirrored_point(Pnt::new(5.0, 5.0, 5.0));
    let rev = d.reversed();
    near(m.x(), rev.x(), CONFUSION);
    near(m.y(), rev.y(), CONFUSION);
    near(m.z(), rev.z(), CONFUSION);
}

// ── 34. Mirrored across an axis ──────────────────────────────────────────────

#[test]
fn mirrored_axis_reflects_perpendicular_component() {
    // Reflect (1,1,0) across the X axis: Y component flips
    let d = Dir::new(1.0, 1.0, 0.0);
    let ax = Ax1::new(Pnt::origin(), Pnt::new(1.0, 0.0, 0.0));
    let m = d.mirrored_axis(ax);
    let expected = Dir::new(1.0, -1.0, 0.0);
    near(m.x(), expected.x(), CONFUSION);
    near(m.y(), expected.y(), CONFUSION);
    near(m.z(), expected.z(), CONFUSION);
}

// ── 35. Mirrored across Z-axis: only Z flips ─────────────────────────────────

#[test]
fn mirrored_axis_z_flips_xy() {
    // Reflect (1,0,1) across the Z axis: X component flips
    let d = Dir::new(1.0, 0.0, 1.0);
    let ax = Ax1::new(Pnt::origin(), Pnt::new(0.0, 0.0, 1.0));
    let m = d.mirrored_axis(ax);
    let expected = Dir::new(-1.0, 0.0, 1.0);
    near(m.x(), expected.x(), CONFUSION);
    near(m.y(), expected.y(), CONFUSION);
    near(m.z(), expected.z(), CONFUSION);
}

// ── 36. Mirrored across XY plane (Ax3 with Z normal) ────────────────────────

#[test]
fn mirrored_plane_xy_flips_z() {
    // Reflect (0,0,1) across XY plane (normal = Z): gives (0,0,-1)
    let d = Dir::new(0.0, 0.0, 1.0);
    let ax2 = Ax3::identity();
    let m = d.mirrored_plane(ax2);
    near(m.x(), 0.0, CONFUSION);
    near(m.y(), 0.0, CONFUSION);
    near(m.z(), -1.0, CONFUSION);
}

// ── 37. Mirrored plane: tangential component is unchanged ────────────────────

#[test]
fn mirrored_plane_tangential_unchanged() {
    // Reflect (1,0,0) across XY plane (normal Z): tangential, unchanged
    let d = Dir::new(1.0, 0.0, 0.0);
    let ax2 = Ax3::identity();
    let m = d.mirrored_plane(ax2);
    near(m.x(), 1.0, CONFUSION);
    near(m.y(), 0.0, CONFUSION);
    near(m.z(), 0.0, CONFUSION);
}

// ── 38. SetX / SetY / SetZ re-normalize ──────────────────────────────────────

#[test]
fn set_x_y_z_normalizes() {
    let mut d = Dir::new(1.0, 0.0, 0.0);
    d.set_y(1.0);
    // Now (1,1,0) normalized = (1/√2, 1/√2, 0)
    let expected = 1.0_f64 / 2.0_f64.sqrt();
    near(d.x(), expected, CONFUSION);
    near(d.y(), expected, CONFUSION);
    near(d.z(), 0.0, CONFUSION);

    let mut d2 = Dir::new(1.0, 0.0, 0.0);
    d2.set_z(1.0);
    near(d2.x(), expected, CONFUSION);
    near(d2.y(), 0.0, CONFUSION);
    near(d2.z(), expected, CONFUSION);
}

// ── 39. Diagonal direction is unit ───────────────────────────────────────────

#[test]
fn diagonal_direction_is_unit() {
    let d = Dir::new(1.0, 1.0, 1.0);
    let expected = 1.0_f64 / 3.0_f64.sqrt();
    near(d.x(), expected, CONFUSION);
    near(d.y(), expected, CONFUSION);
    near(d.z(), expected, CONFUSION);
    let mag = (d.x() * d.x() + d.y() * d.y() + d.z() * d.z()).sqrt();
    near(mag, 1.0, CONFUSION);
}

// ── 40. CrossMagnitude is zero for parallel directions ───────────────────────

#[test]
fn cross_magnitude_parallel_is_zero() {
    let d = Dir::new(0.0, 0.0, 1.0);
    let same = Dir::new(0.0, 0.0, 3.0); // normalizes to Z
    near(d.cross_magnitude(&same), 0.0, CONFUSION);
    near(d.cross_square_magnitude(&same), 0.0, CONFUSION);
}
