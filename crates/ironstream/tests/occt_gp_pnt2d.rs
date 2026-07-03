//! OCCT-ported tests for `gp_Pnt2d` (IronStream: `gp_pnt2d::GpPnt2d`).
//!
//! Each test corresponds directly to behaviour specified in the OCCT
//! documentation or the OCCT test suite for `gp_Pnt2d`.

use ironstream::gp2d::{Ax2d, Pnt2d};
use ironstream::gp_pnt2d::GpPnt2d;
use ironstream::gp_trsf2d::Trsf2d;
use ironstream::precision::{ANGULAR, CONFUSION};
use std::f64::consts::{FRAC_PI_2, FRAC_PI_4, PI};

// ── helpers ──────────────────────────────────────────────────────────────────

fn near(a: f64, b: f64) {
    assert!(
        (a - b).abs() <= CONFUSION * 10.0,
        "expected {b}, got {a} (diff {})",
        (a - b).abs()
    );
}

fn near_strict(a: f64, b: f64) {
    assert!(
        (a - b).abs() <= CONFUSION,
        "expected {b}, got {a} (diff {})",
        (a - b).abs()
    );
}

// ── tests ─────────────────────────────────────────────────────────────────────

/// Constructor, X(), Y(), Coord() accessors.
#[test]
fn test_constructor_and_accessors() {
    let p = GpPnt2d::new(1.5, -2.5);
    near_strict(p.x(), 1.5);
    near_strict(p.y(), -2.5);
    let (x, y) = p.coord();
    near_strict(x, 1.5);
    near_strict(y, -2.5);
    near_strict(p.coord_index(1), 1.5);
    near_strict(p.coord_index(2), -2.5);
    let _ = ANGULAR; // suppress unused-constant warning
}

/// SetCoord, SetX, SetY, SetCoord(index, val).
#[test]
fn test_setters() {
    let mut p = GpPnt2d::origin();
    p.set_coord(3.0, 7.0);
    near_strict(p.x(), 3.0);
    near_strict(p.y(), 7.0);

    p.set_x(-1.0);
    near_strict(p.x(), -1.0);

    p.set_y(99.0);
    near_strict(p.y(), 99.0);

    p.set_coord_index(1, 10.0);
    near_strict(p.x(), 10.0);

    p.set_coord_index(2, 20.0);
    near_strict(p.y(), 20.0);
}

/// Distance — 3-4-5 right triangle.
#[test]
fn test_distance() {
    let a = GpPnt2d::new(0.0, 0.0);
    let b = GpPnt2d::new(3.0, 4.0);
    near_strict(a.distance(b), 5.0);
    near_strict(b.distance(a), 5.0); // symmetry
}

/// SquareDistance.
#[test]
fn test_square_distance() {
    let a = GpPnt2d::new(1.0, 1.0);
    let b = GpPnt2d::new(4.0, 5.0);
    // dist = 5, sq = 25
    near_strict(a.square_distance(b), 25.0);
    near_strict(b.square_distance(a), 25.0);
}

/// IsEqual with tolerance.
#[test]
fn test_is_equal() {
    let a = GpPnt2d::new(0.0, 0.0);
    // Slightly inside tolerance
    let b = GpPnt2d::new(CONFUSION * 0.5, 0.0);
    assert!(a.is_equal(b, CONFUSION));
    // Slightly outside tolerance
    let c = GpPnt2d::new(CONFUSION * 2.0, 0.0);
    assert!(!a.is_equal(c, CONFUSION));
    // Identical point
    assert!(a.is_equal(a, 0.0));
}

/// Translated(Vec) and Translate(Vec) (in-place) and two-point forms.
#[test]
fn test_translate() {
    let p = GpPnt2d::new(2.0, 3.0);
    let v = Pnt2d::new(10.0, -5.0);

    // value form
    let q = p.translated(v);
    near_strict(q.x(), 12.0);
    near_strict(q.y(), -2.0);

    // in-place form
    let mut r = p;
    r.translate(v);
    near_strict(r.x(), 12.0);
    near_strict(r.y(), -2.0);

    // two-point form: translate by P1→P2 = (1,1)→(4,6) = (3,5)
    let p1 = GpPnt2d::new(1.0, 1.0);
    let p2 = GpPnt2d::new(4.0, 6.0);
    let s = p.translated_2pts(p1, p2);
    near_strict(s.x(), 5.0);
    near_strict(s.y(), 8.0);

    let mut u = p;
    u.translate_2pts(p1, p2);
    near_strict(u.x(), 5.0);
    near_strict(u.y(), 8.0);
}

/// Rotated / Rotate — CCW about origin.
#[test]
fn test_rotate_about_origin() {
    let p = GpPnt2d::new(1.0, 0.0);
    let origin = GpPnt2d::origin();

    // 90°
    let q = p.rotated(origin, FRAC_PI_2);
    near(q.x(), 0.0);
    near(q.y(), 1.0);

    // 180°
    let r = p.rotated(origin, PI);
    near(r.x(), -1.0);
    near(r.y(), 0.0);

    // 360° → back to start
    let s = p.rotated(origin, 2.0 * PI);
    near(s.x(), 1.0);
    near(s.y(), 0.0);

    // in-place
    let mut t = p;
    t.rotate(origin, FRAC_PI_2);
    near(t.x(), 0.0);
    near(t.y(), 1.0);
}

/// Rotated / Rotate — CCW about a non-origin center.
#[test]
fn test_rotate_about_nonorigin() {
    // Rotate (3, 1) by 90° about (1, 1) → (1, 3).
    let p = GpPnt2d::new(3.0, 1.0);
    let c = GpPnt2d::new(1.0, 1.0);
    let q = p.rotated(c, FRAC_PI_2);
    near(q.x(), 1.0);
    near(q.y(), 3.0);
}

/// Mirrored(Point) — central symmetry.
#[test]
fn test_mirror_point() {
    let p = GpPnt2d::new(4.0, 5.0);
    let center = GpPnt2d::new(1.0, 2.0);
    let q = p.mirrored_point(center);
    // q = 2*center - p = (-2, -1)
    near_strict(q.x(), -2.0);
    near_strict(q.y(), -1.0);

    // Mirroring twice should return the original.
    let q2 = q.mirrored_point(center);
    near_strict(q2.x(), p.x());
    near_strict(q2.y(), p.y());

    // in-place
    let mut r = p;
    r.mirror_point(center);
    near_strict(r.x(), -2.0);
    near_strict(r.y(), -1.0);
}

/// Mirrored(Axis) — reflection across lines aligned with X, Y, and a diagonal.
#[test]
fn test_mirror_axis() {
    let p = GpPnt2d::new(3.0, 4.0);

    // X axis: (3, 4) → (3, -4)
    let ax = Ax2d::new(Pnt2d::new(0.0, 0.0), Pnt2d::new(1.0, 0.0));
    let q = p.mirrored_axis(ax);
    near_strict(q.x(), 3.0);
    near_strict(q.y(), -4.0);

    // Y axis: (3, 4) → (-3, 4)
    let ay = Ax2d::new(Pnt2d::new(0.0, 0.0), Pnt2d::new(0.0, 1.0));
    let r = p.mirrored_axis(ay);
    near_strict(r.x(), -3.0);
    near_strict(r.y(), 4.0);

    // 45° diagonal (y=x): (3,4) → (4,3)
    let ad = Ax2d::new(Pnt2d::new(0.0, 0.0), Pnt2d::new(1.0, 1.0));
    let s = p.mirrored_axis(ad);
    near(s.x(), 4.0);
    near(s.y(), 3.0);

    // in-place
    let mut t = p;
    t.mirror_axis(ax);
    near_strict(t.x(), 3.0);
    near_strict(t.y(), -4.0);
}

/// Scaled / Scale — uniform scaling about a center.
#[test]
fn test_scale() {
    let p = GpPnt2d::new(4.0, 6.0);
    let center = GpPnt2d::new(2.0, 2.0);

    // Factor 2: rel=(2,4) → scaled rel=(4,8) → abs=(6,10)
    let q = p.scaled(center, 2.0);
    near_strict(q.x(), 6.0);
    near_strict(q.y(), 10.0);

    // Factor 0 → collapses to center
    let z = p.scaled(center, 0.0);
    near_strict(z.x(), center.x());
    near_strict(z.y(), center.y());

    // Negative factor: inversion relative to center
    let n = p.scaled(center, -1.0);
    near_strict(n.x(), 0.0);
    near_strict(n.y(), -2.0);

    // in-place
    let mut r = p;
    r.scale(center, 0.5);
    near_strict(r.x(), 3.0);
    near_strict(r.y(), 4.0);
}

/// Transform / Transformed — apply gp_Trsf2d.
#[test]
fn test_transform() {
    // Translation
    let mut t_tr = Trsf2d::identity();
    t_tr.set_translation(Pnt2d::new(5.0, -3.0));
    let p = GpPnt2d::new(1.0, 2.0);
    let q = p.transformed(&t_tr);
    near_strict(q.x(), 6.0);
    near_strict(q.y(), -1.0);

    // Rotation 90° about origin
    let mut t_rot = Trsf2d::identity();
    t_rot.set_rotation(Pnt2d::origin(), FRAC_PI_2);
    let r = GpPnt2d::new(1.0, 0.0).transformed(&t_rot);
    near(r.x(), 0.0);
    near(r.y(), 1.0);

    // Scale factor 3 about origin
    let mut t_sc = Trsf2d::identity();
    t_sc.set_scale(Pnt2d::origin(), 3.0).unwrap();
    let s = GpPnt2d::new(2.0, -1.0).transformed(&t_sc);
    near_strict(s.x(), 6.0);
    near_strict(s.y(), -3.0);

    // in-place
    let mut u = GpPnt2d::new(1.0, 0.0);
    u.transform(&t_rot);
    near(u.x(), 0.0);
    near(u.y(), 1.0);
}

/// Mirror across an axis not passing through the origin.
#[test]
fn test_mirror_axis_offset() {
    // Mirror (0, 0) across the horizontal line y = 2  (axis location (0,2), dir (1,0)).
    let p = GpPnt2d::origin();
    let ax = Ax2d::new(Pnt2d::new(0.0, 2.0), Pnt2d::new(1.0, 0.0));
    let q = p.mirrored_axis(ax);
    near(q.x(), 0.0);
    near(q.y(), 4.0);
}

/// Rotate with a 45° angle and check both components.
#[test]
fn test_rotate_45() {
    let p = GpPnt2d::new(1.0, 0.0);
    let origin = GpPnt2d::origin();
    let q = p.rotated(origin, FRAC_PI_4);
    let expected = 1.0_f64 / 2.0_f64.sqrt();
    near(q.x(), expected);
    near(q.y(), expected);
}

/// from_pnt2d / as_pnt2d round-trip and that origin() is (0,0).
#[test]
fn test_origin_and_pnt2d_roundtrip() {
    let o = GpPnt2d::origin();
    near_strict(o.x(), 0.0);
    near_strict(o.y(), 0.0);

    let raw = Pnt2d::new(-5.0, 8.0);
    let wrapped = GpPnt2d::from_pnt2d(raw);
    assert_eq!(wrapped.as_pnt2d(), raw);
    near_strict(wrapped.x(), -5.0);
    near_strict(wrapped.y(), 8.0);
}
