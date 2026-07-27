//! Ported OpenCascade unit tests -- `Geom_Direction`.
//!
//! Source: OpenCascade Technology, `TKG3d` package, `Geom_Direction` class
//! tests (ported by hand).  Each `#[test]` corresponds to one logical GTest or
//! scenario from the OCCT test suite for `Geom_Direction`.
//!
//! Tolerances used:
//! * linear:  `precision::CONFUSION` = 1e-7
//! * angular: `precision::ANGULAR`   = 1e-12

use ironstream::geom_direction::GeomDirection;
use ironstream::gp::{Ax1, Pnt, Trsf, Vec3};
use ironstream::precision::{ANGULAR, CONFUSION};
use std::f64::consts::{FRAC_PI_2, FRAC_PI_4, FRAC_PI_6, PI};

/// `EXPECT_NEAR` helper — panics if |a - b| > tol.
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
    // gp_Dir d(3, 0, 4); (magnitude 5 → unit (0.6, 0.0, 0.8))
    let d = GeomDirection::new(3.0, 0.0, 4.0);
    near(d.x(), 0.6, CONFUSION);
    near(d.y(), 0.0, CONFUSION);
    near(d.z(), 0.8, CONFUSION);
    let mag = (d.x() * d.x() + d.y() * d.y() + d.z() * d.z()).sqrt();
    near(mag, 1.0, CONFUSION);
}

// ── 2. Constructor: unit vector is preserved ──────────────────────────────────

#[test]
fn construct_unit_x() {
    let d = GeomDirection::new(1.0, 0.0, 0.0);
    near(d.x(), 1.0, CONFUSION);
    near(d.y(), 0.0, CONFUSION);
    near(d.z(), 0.0, CONFUSION);
}

// ── 3. from_dir round-trips a Vec3 ────────────────────────────────────────────

#[test]
fn from_dir_round_trip() {
    let v = Vec3::new(0.0, 1.0, 0.0);
    let d = GeomDirection::from_dir(v);
    near(d.x(), 0.0, CONFUSION);
    near(d.y(), 1.0, CONFUSION);
    near(d.z(), 0.0, CONFUSION);
    let q = d.dir();
    near(q.x, 0.0, CONFUSION);
    near(q.y, 1.0, CONFUSION);
    near(q.z, 0.0, CONFUSION);
}

// ── 4. SetCoord replaces and normalizes ───────────────────────────────────────

#[test]
fn set_coord_normalizes() {
    let mut d = GeomDirection::new(1.0, 0.0, 0.0);
    d.set_coord(0.0, 0.0, 7.0);
    near(d.x(), 0.0, CONFUSION);
    near(d.y(), 0.0, CONFUSION);
    near(d.z(), 1.0, CONFUSION);
}

// ── 5. coord() tuple matches x(), y(), z() ────────────────────────────────────

#[test]
fn coord_tuple_matches_accessors() {
    let d = GeomDirection::new(1.0, 2.0, 2.0);
    let (x, y, z) = d.coord();
    near(x, d.x(), ANGULAR);
    near(y, d.y(), ANGULAR);
    near(z, d.z(), ANGULAR);
}

// ── 6. Crossed: perpendicular directions give |cross| = sin(90°) = 1 ─────────

#[test]
fn crossed_perpendicular_is_one() {
    let dx = GeomDirection::new(1.0, 0.0, 0.0);
    let dy = GeomDirection::new(0.0, 1.0, 0.0);
    let c = dx.crossed(&dy);
    near(c.norm(), 1.0, CONFUSION);
    // cross(X,Y) = Z
    near(c.x, 0.0, CONFUSION);
    near(c.y, 0.0, CONFUSION);
    near(c.z, 1.0, CONFUSION);
}

// ── 7. Crossed: parallel directions give zero ─────────────────────────────────

#[test]
fn crossed_parallel_is_zero() {
    let dx = GeomDirection::new(1.0, 0.0, 0.0);
    let dx2 = GeomDirection::new(2.0, 0.0, 0.0); // normalizes to (1,0,0)
    let c = dx.crossed(&dx2);
    near(c.norm(), 0.0, CONFUSION);
}

// ── 8. Crossed is antisymmetric ───────────────────────────────────────────────

#[test]
fn crossed_antisymmetric() {
    let a = GeomDirection::new(1.0, 0.0, 0.0);
    let b = GeomDirection::new(0.0, 1.0, 0.0);
    let ab = a.crossed(&b);
    let ba = b.crossed(&a);
    near(ab.x, -ba.x, ANGULAR);
    near(ab.y, -ba.y, ANGULAR);
    near(ab.z, -ba.z, ANGULAR);
}

// ── 9. Dot: same direction gives cos(0°) = 1 ─────────────────────────────────

#[test]
fn dot_same_direction_is_one() {
    let d = GeomDirection::new(1.0, 0.0, 0.0);
    near(d.dot(&d), 1.0, CONFUSION);
}

// ── 10. Dot: perpendicular gives cos(90°) = 0 ────────────────────────────────

#[test]
fn dot_perpendicular_is_zero() {
    let dx = GeomDirection::new(1.0, 0.0, 0.0);
    let dy = GeomDirection::new(0.0, 1.0, 0.0);
    near(dx.dot(&dy), 0.0, CONFUSION);
}

// ── 11. Dot: opposite direction gives -1 ─────────────────────────────────────

#[test]
fn dot_opposite_is_minus_one() {
    let d = GeomDirection::new(1.0, 0.0, 0.0);
    let neg = GeomDirection::new(-1.0, 0.0, 0.0);
    near(d.dot(&neg), -1.0, CONFUSION);
}

// ── 12. IsOpposite: antiparallel within tol ──────────────────────────────────

#[test]
fn is_opposite_antiparallel() {
    let d = GeomDirection::new(1.0, 0.0, 0.0);
    let neg = GeomDirection::new(-1.0, 0.0, 0.0);
    assert!(d.is_opposite(&neg, 1e-7));
}

// ── 13. IsOpposite: co-directional is not opposite ────────────────────────────

#[test]
fn is_opposite_same_is_false() {
    let d = GeomDirection::new(1.0, 0.0, 0.0);
    assert!(!d.is_opposite(&d, 1e-7));
}

// ── 14. IsParallel: same direction ───────────────────────────────────────────

#[test]
fn is_parallel_same() {
    let d = GeomDirection::new(1.0, 0.0, 0.0);
    assert!(d.is_parallel(&d, 1e-7));
}

// ── 15. IsParallel: antiparallel is also parallel ─────────────────────────────

#[test]
fn is_parallel_antiparallel() {
    let d = GeomDirection::new(1.0, 0.0, 0.0);
    let neg = GeomDirection::new(-1.0, 0.0, 0.0);
    assert!(d.is_parallel(&neg, 1e-7));
}

// ── 16. IsParallel: perpendicular is NOT parallel ─────────────────────────────

#[test]
fn is_parallel_perpendicular_is_false() {
    let dx = GeomDirection::new(1.0, 0.0, 0.0);
    let dy = GeomDirection::new(0.0, 1.0, 0.0);
    assert!(!dx.is_parallel(&dy, 1e-7));
}

// ── 17. Angle: 90° between X and Y axes ──────────────────────────────────────

#[test]
fn angle_90_degrees() {
    let dx = GeomDirection::new(1.0, 0.0, 0.0);
    let dy = GeomDirection::new(0.0, 1.0, 0.0);
    near(dx.angle(&dy), FRAC_PI_2, ANGULAR);
}

// ── 18. Angle: 45° ────────────────────────────────────────────────────────────

#[test]
fn angle_45_degrees() {
    let dx = GeomDirection::new(1.0, 0.0, 0.0);
    let d45 = GeomDirection::new(1.0, 1.0, 0.0);
    near(dx.angle(&d45), FRAC_PI_4, CONFUSION);
}

// ── 19. Angle: 30° ────────────────────────────────────────────────────────────

#[test]
fn angle_30_degrees() {
    let dx = GeomDirection::new(1.0, 0.0, 0.0);
    let d30 = GeomDirection::new(3_f64.sqrt(), 1.0, 0.0);
    near(dx.angle(&d30), FRAC_PI_6, CONFUSION);
}

// ── 20. Angle: antiparallel is π ─────────────────────────────────────────────

#[test]
fn angle_antiparallel_is_pi() {
    let dx = GeomDirection::new(1.0, 0.0, 0.0);
    let neg = GeomDirection::new(-1.0, 0.0, 0.0);
    near(dx.angle(&neg), PI, CONFUSION);
}

// ── 21. Angle: self→self is 0 ────────────────────────────────────────────────

#[test]
fn angle_to_self_is_zero() {
    let d = GeomDirection::new(1.0, 1.0, 1.0);
    near(d.angle(&d), 0.0, ANGULAR);
}

// ── 22. Angle: symmetric (unsigned) ──────────────────────────────────────────

#[test]
fn angle_is_symmetric() {
    let a = GeomDirection::new(1.0, 0.0, 0.0);
    let b = GeomDirection::new(0.0, 1.0, 0.0);
    near(a.angle(&b), b.angle(&a), ANGULAR);
}

// ── 23. Reverse ───────────────────────────────────────────────────────────────

#[test]
fn reverse_in_place() {
    let mut d = GeomDirection::new(1.0, 0.0, 0.0);
    d.reverse();
    near(d.x(), -1.0, CONFUSION);
    near(d.y(), 0.0, CONFUSION);
    near(d.z(), 0.0, CONFUSION);
}

// ── 24. Reversed (non-mutating) ───────────────────────────────────────────────

#[test]
fn reversed_non_mutating() {
    let original = GeomDirection::new(1.0, 0.0, 0.0);
    let r = original.reversed();
    near(r.x(), -1.0, CONFUSION);
    near(r.y(), 0.0, CONFUSION);
    near(r.z(), 0.0, CONFUSION);
    // original unchanged
    near(original.x(), 1.0, CONFUSION);
}

// ── 25. Reverse twice returns to original ─────────────────────────────────────

#[test]
fn double_reverse_identity() {
    let original = GeomDirection::new(3.0, 0.0, 4.0);
    let mut d = original;
    d.reverse();
    d.reverse();
    near(d.x(), original.x(), ANGULAR);
    near(d.y(), original.y(), ANGULAR);
    near(d.z(), original.z(), ANGULAR);
}

// ── 26. Transform: 90° rotation about Z ──────────────────────────────────────

#[test]
fn transform_rotation_90_about_z() {
    let mut d = GeomDirection::new(1.0, 0.0, 0.0);
    let axis = Ax1::new(Pnt::origin(), Vec3::new(0.0, 0.0, 1.0));
    let t = Trsf::rotation(axis, FRAC_PI_2);
    d.transform(&t);
    near(d.x(), 0.0, CONFUSION);
    near(d.y(), 1.0, CONFUSION);
    near(d.z(), 0.0, CONFUSION);
}

// ── 27. Transformed (non-mutating) ───────────────────────────────────────────

#[test]
fn transformed_non_mutating() {
    let d = GeomDirection::new(1.0, 0.0, 0.0);
    let axis = Ax1::new(Pnt::origin(), Vec3::new(0.0, 0.0, 1.0));
    let t = Trsf::rotation(axis, FRAC_PI_2);
    let t2 = d.transformed(&t);
    // original unchanged
    near(d.x(), 1.0, CONFUSION);
    near(d.y(), 0.0, CONFUSION);
    near(d.z(), 0.0, CONFUSION);
    // transformed
    near(t2.x(), 0.0, CONFUSION);
    near(t2.y(), 1.0, CONFUSION);
    near(t2.z(), 0.0, CONFUSION);
}

// ── 28. Transform: 180° rotation gives reversed direction ────────────────────

#[test]
fn transform_rotation_180() {
    let mut d = GeomDirection::new(1.0, 0.0, 0.0);
    let axis = Ax1::new(Pnt::origin(), Vec3::new(0.0, 0.0, 1.0));
    let t = Trsf::rotation(axis, PI);
    d.transform(&t);
    near(d.x(), -1.0, CONFUSION);
    near(d.y(), 0.0, CONFUSION);
    near(d.z(), 0.0, CONFUSION);
}

// ── 29. Copy returns independent value ───────────────────────────────────────

#[test]
fn copy_is_independent() {
    let original = GeomDirection::new(1.0, 0.0, 0.0);
    let mut c = original.copy();
    c.set_coord(0.0, 1.0, 0.0);
    // original must be unchanged
    near(original.x(), 1.0, CONFUSION);
    near(original.y(), 0.0, CONFUSION);
    near(original.z(), 0.0, CONFUSION);
}

// ── 30. SetDir replaces direction ────────────────────────────────────────────

#[test]
fn set_dir_replaces() {
    let mut d = GeomDirection::new(1.0, 0.0, 0.0);
    d.set_dir(Vec3::new(0.0, 0.0, 3.0));
    near(d.x(), 0.0, CONFUSION);
    near(d.y(), 0.0, CONFUSION);
    near(d.z(), 1.0, CONFUSION);
}

// ── 31. 3D diagonal direction is unit ────────────────────────────────────────

#[test]
fn diagonal_direction_is_unit() {
    // Direction (1,1,1) normalized = (1/√3, 1/√3, 1/√3)
    let d = GeomDirection::new(1.0, 1.0, 1.0);
    let expected = 1.0_f64 / 3.0_f64.sqrt();
    near(d.x(), expected, CONFUSION);
    near(d.y(), expected, CONFUSION);
    near(d.z(), expected, CONFUSION);
    let mag = (d.x() * d.x() + d.y() * d.y() + d.z() * d.z()).sqrt();
    near(mag, 1.0, CONFUSION);
}

// ── 32. Angle between Z and diagonal ─────────────────────────────────────────

#[test]
fn angle_z_and_diagonal() {
    let dz = GeomDirection::new(0.0, 0.0, 1.0);
    // (1,1,1) normalized; angle with Z = acos(1/√3)
    let diag = GeomDirection::new(1.0, 1.0, 1.0);
    let expected = (1.0_f64 / 3.0_f64.sqrt()).acos();
    near(dz.angle(&diag), expected, CONFUSION);
}
