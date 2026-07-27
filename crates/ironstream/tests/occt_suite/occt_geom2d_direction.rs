//! Ported OpenCascade unit tests -- `Geom2d_Direction`.
//!
//! Source: OpenCascade Technology, `TKG2d` package, `Geom2d_Direction` class
//! tests (ported by hand).  Each `#[test]` corresponds to one logical GTest or
//! scenario from the OCCT test suite for `Geom2d_Direction`.
//!
//! Tolerances used:
//! * linear:  `precision::CONFUSION` = 1e-7
//! * angular: `precision::ANGULAR`   = 1e-12

use ironstream::geom2d_direction::Direction2d;
use ironstream::gp2d::{Pnt2d, Trsf2d};
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

// ── 1. Constructor: (X,Y) normalizes the vector ───────────────────────────────

#[test]
fn construct_from_coords_normalizes() {
    // gp_Dir2d d(3, 4); (magnitude 5 → unit (0.6, 0.8))
    let d = Direction2d::new(3.0, 4.0);
    near(d.x(), 0.6, CONFUSION);
    near(d.y(), 0.8, CONFUSION);
    let mag = (d.x() * d.x() + d.y() * d.y()).sqrt();
    near(mag, 1.0, CONFUSION);
}

// ── 2. Constructor: unit vector is preserved ──────────────────────────────────

#[test]
fn construct_unit_x() {
    let d = Direction2d::new(1.0, 0.0);
    near(d.x(), 1.0, CONFUSION);
    near(d.y(), 0.0, CONFUSION);
}

// ── 3. from_dir2d round-trips a Pnt2d ─────────────────────────────────────────

#[test]
fn from_dir2d_round_trip() {
    let p = Pnt2d::new(0.0, 1.0);
    let d = Direction2d::from_dir2d(p);
    near(d.x(), 0.0, CONFUSION);
    near(d.y(), 1.0, CONFUSION);
    let q = d.dir2d();
    near(q.x, 0.0, CONFUSION);
    near(q.y, 1.0, CONFUSION);
}

// ── 4. SetCoord replaces and normalizes ───────────────────────────────────────

#[test]
fn set_coord_normalizes() {
    let mut d = Direction2d::new(1.0, 0.0);
    d.set_coord(0.0, 7.0);
    near(d.x(), 0.0, CONFUSION);
    near(d.y(), 1.0, CONFUSION);
}

// ── 5. coord() tuple matches x() and y() ─────────────────────────────────────

#[test]
fn coord_tuple_matches_accessors() {
    let d = Direction2d::new(1.0, 2.0);
    let (x, y) = d.coord();
    near(x, d.x(), ANGULAR);
    near(y, d.y(), ANGULAR);
}

// ── 6. Crossed: perpendicular directions give sin(90°) = 1 ───────────────────

#[test]
fn crossed_perpendicular_is_one() {
    let dx = Direction2d::new(1.0, 0.0);
    let dy = Direction2d::new(0.0, 1.0);
    near(dx.crossed(&dy), 1.0, CONFUSION);
}

// ── 7. Crossed: parallel directions give 0 ────────────────────────────────────

#[test]
fn crossed_parallel_is_zero() {
    let dx = Direction2d::new(1.0, 0.0);
    let dx2 = Direction2d::new(2.0, 0.0); // normalizes to (1,0)
    near(dx.crossed(&dx2), 0.0, CONFUSION);
}

// ── 8. Crossed is antisymmetric ───────────────────────────────────────────────

#[test]
fn crossed_antisymmetric() {
    let a = Direction2d::new(1.0, 0.0);
    let b = Direction2d::new(0.0, 1.0);
    near(a.crossed(&b), -b.crossed(&a), ANGULAR);
}

// ── 9. Dot: same direction gives cos(0°) = 1 ─────────────────────────────────

#[test]
fn dot_same_direction_is_one() {
    let d = Direction2d::new(1.0, 0.0);
    near(d.dot(&d), 1.0, CONFUSION);
}

// ── 10. Dot: perpendicular gives cos(90°) = 0 ────────────────────────────────

#[test]
fn dot_perpendicular_is_zero() {
    let dx = Direction2d::new(1.0, 0.0);
    let dy = Direction2d::new(0.0, 1.0);
    near(dx.dot(&dy), 0.0, CONFUSION);
}

// ── 11. Dot: opposite direction gives -1 ─────────────────────────────────────

#[test]
fn dot_opposite_is_minus_one() {
    let d = Direction2d::new(1.0, 0.0);
    let neg = Direction2d::new(-1.0, 0.0);
    near(d.dot(&neg), -1.0, CONFUSION);
}

// ── 12. IsOpposite: antiparallel within tol ──────────────────────────────────

#[test]
fn is_opposite_antiparallel() {
    let d = Direction2d::new(1.0, 0.0);
    let neg = Direction2d::new(-1.0, 0.0);
    assert!(d.is_opposite(&neg, 1e-7));
}

// ── 13. IsOpposite: co-directional is not opposite ────────────────────────────

#[test]
fn is_opposite_same_is_false() {
    let d = Direction2d::new(1.0, 0.0);
    assert!(!d.is_opposite(&d, 1e-7));
}

// ── 14. IsParallel: same direction ───────────────────────────────────────────

#[test]
fn is_parallel_same() {
    let d = Direction2d::new(1.0, 0.0);
    assert!(d.is_parallel(&d, 1e-7));
}

// ── 15. IsParallel: antiparallel is also parallel ─────────────────────────────

#[test]
fn is_parallel_antiparallel() {
    let d = Direction2d::new(1.0, 0.0);
    let neg = Direction2d::new(-1.0, 0.0);
    assert!(d.is_parallel(&neg, 1e-7));
}

// ── 16. IsParallel: perpendicular is NOT parallel ─────────────────────────────

#[test]
fn is_parallel_perpendicular_is_false() {
    let dx = Direction2d::new(1.0, 0.0);
    let dy = Direction2d::new(0.0, 1.0);
    assert!(!dx.is_parallel(&dy, 1e-7));
}

// ── 17. Angle: 90° CCW ────────────────────────────────────────────────────────

#[test]
fn angle_90_ccw() {
    let dx = Direction2d::new(1.0, 0.0);
    let dy = Direction2d::new(0.0, 1.0);
    near(dx.angle(&dy), FRAC_PI_2, ANGULAR);
}

// ── 18. Angle: 90° CW is -π/2 ────────────────────────────────────────────────

#[test]
fn angle_90_cw_is_negative() {
    let dx = Direction2d::new(1.0, 0.0);
    let dy_neg = Direction2d::new(0.0, -1.0);
    near(dx.angle(&dy_neg), -FRAC_PI_2, ANGULAR);
}

// ── 19. Angle: 45° ────────────────────────────────────────────────────────────

#[test]
fn angle_45_degrees() {
    let dx = Direction2d::new(1.0, 0.0);
    let d45 = Direction2d::new(1.0, 1.0);
    near(dx.angle(&d45), FRAC_PI_4, CONFUSION);
}

// ── 20. Angle: 30° ────────────────────────────────────────────────────────────

#[test]
fn angle_30_degrees() {
    let dx = Direction2d::new(1.0, 0.0);
    let d30 = Direction2d::new(3_f64.sqrt(), 1.0);
    near(dx.angle(&d30), FRAC_PI_6, CONFUSION);
}

// ── 21. Angle: antiparallel is ±π ────────────────────────────────────────────

#[test]
fn angle_antiparallel_magnitude_is_pi() {
    let dx = Direction2d::new(1.0, 0.0);
    let neg = Direction2d::new(-1.0, 0.0);
    let a = dx.angle(&neg).abs();
    near(a, PI, CONFUSION);
}

// ── 22. Angle: self→self is 0 ────────────────────────────────────────────────

#[test]
fn angle_to_self_is_zero() {
    let d = Direction2d::new(1.0, 1.0);
    near(d.angle(&d), 0.0, ANGULAR);
}

// ── 23. Reverse ───────────────────────────────────────────────────────────────

#[test]
fn reverse_in_place() {
    let mut d = Direction2d::new(1.0, 0.0);
    d.reverse();
    near(d.x(), -1.0, CONFUSION);
    near(d.y(), 0.0, CONFUSION);
}

// ── 24. Reversed (non-mutating) ───────────────────────────────────────────────

#[test]
fn reversed_non_mutating() {
    let original = Direction2d::new(1.0, 0.0);
    let r = original.reversed();
    near(r.x(), -1.0, CONFUSION);
    near(r.y(), 0.0, CONFUSION);
    // original unchanged
    near(original.x(), 1.0, CONFUSION);
    near(original.y(), 0.0, CONFUSION);
}

// ── 25. Transform: 90° rotation ──────────────────────────────────────────────

#[test]
fn transform_rotation_90() {
    let mut d = Direction2d::new(1.0, 0.0);
    let t = Trsf2d::rotation(Pnt2d::origin(), FRAC_PI_2);
    d.transform(&t);
    near(d.x(), 0.0, CONFUSION);
    near(d.y(), 1.0, CONFUSION);
}

// ── 26. Transformed (non-mutating) ───────────────────────────────────────────

#[test]
fn transformed_non_mutating() {
    let d = Direction2d::new(1.0, 0.0);
    let t = Trsf2d::rotation(Pnt2d::origin(), FRAC_PI_2);
    let t2 = d.transformed(&t);
    // original unchanged
    near(d.x(), 1.0, CONFUSION);
    near(d.y(), 0.0, CONFUSION);
    // transformed
    near(t2.x(), 0.0, CONFUSION);
    near(t2.y(), 1.0, CONFUSION);
}

// ── 27. Transform: 180° rotation gives reversed direction ────────────────────

#[test]
fn transform_rotation_180() {
    let mut d = Direction2d::new(1.0, 0.0);
    let t = Trsf2d::rotation(Pnt2d::origin(), PI);
    d.transform(&t);
    near(d.x(), -1.0, CONFUSION);
    near(d.y(), 0.0, CONFUSION);
}

// ── 28. Copy returns independent value ───────────────────────────────────────

#[test]
fn copy_is_independent() {
    let original = Direction2d::new(1.0, 0.0);
    let mut c = original.copy();
    c.set_coord(0.0, 1.0);
    // original must be unchanged
    near(original.x(), 1.0, CONFUSION);
    near(original.y(), 0.0, CONFUSION);
}

// ── 29. Reverse twice returns to original ─────────────────────────────────────

#[test]
fn double_reverse_identity() {
    let original = Direction2d::new(3.0, 4.0);
    let mut d = original;
    d.reverse();
    d.reverse();
    near(d.x(), original.x(), ANGULAR);
    near(d.y(), original.y(), ANGULAR);
}

// ── 30. Angle is antisymmetric (opposite sign for reversed order) ─────────────

#[test]
fn angle_antisymmetric() {
    let a = Direction2d::new(1.0, 0.0);
    let b = Direction2d::new(0.0, 1.0);
    let ab = a.angle(&b);
    let ba = b.angle(&a);
    near(ab, -ba, ANGULAR);
}
