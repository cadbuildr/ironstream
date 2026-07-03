//! Ported OpenCascade unit tests — `gp` analytic primitives.
//!
//! Faithful Rust ports of OCCT's `gp_Lin_Test.cxx`, `gp_Circ_Test.cxx` and
//! `gp_Pln_Test.cxx` (Open-Cascade-SAS/OCCT, `src/FoundationClasses/TKMath/
//! GTests/`). Same inputs, expected values and tolerances.

use ironstream::gp::{Ax1, Ax3, Pnt, Trsf};
use ironstream::gp_prim::{Circ, Lin, Pln};
use ironstream::precision::{ANGULAR, CONFUSION};
use std::f64::consts::{FRAC_PI_2, PI};

fn near(a: f64, b: f64, tol: f64) {
    assert!((a - b).abs() <= tol, "expected {a} ≈ {b} (tol {tol})");
}
fn ax2(loc: Pnt, n: Pnt) -> Ax3 {
    Ax3::from_origin_normal(loc, n, n.any_perpendicular())
}

// ====================== gp_Lin_Test.cxx ======================

#[test]
fn lin_constructor_from_point_and_dir() {
    let l = Lin::new_pd(Pnt::origin(), Pnt::dir(0.0, 0.0, 1.0));
    assert!(l.location().is_equal(Pnt::new(0.0, 0.0, 0.0), CONFUSION));
    assert!(l.direction().is_equal_dir(Pnt::dir(0.0, 0.0, 1.0), ANGULAR));
}

#[test]
fn lin_distance_to_point() {
    let l = Lin::new_pd(Pnt::origin(), Pnt::dir(1.0, 0.0, 0.0));
    near(l.distance_point(Pnt::new(5.0, 3.0, 4.0)), 5.0, CONFUSION);
}

#[test]
fn lin_distance_to_line_parallel() {
    let l1 = Lin::new_pd(Pnt::origin(), Pnt::dir(1.0, 0.0, 0.0));
    let l2 = Lin::new_pd(Pnt::new(0.0, 3.0, 4.0), Pnt::dir(1.0, 0.0, 0.0));
    near(l1.distance_line(&l2), 5.0, CONFUSION);
}

#[test]
fn lin_distance_to_line_intersecting() {
    let l1 = Lin::new_pd(Pnt::origin(), Pnt::dir(1.0, 0.0, 0.0));
    let l2 = Lin::new_pd(Pnt::origin(), Pnt::dir(0.0, 1.0, 0.0));
    near(l1.distance_line(&l2), 0.0, CONFUSION);
}

#[test]
fn lin_contains() {
    let l = Lin::new_pd(Pnt::origin(), Pnt::dir(1.0, 0.0, 0.0));
    assert!(l.contains(Pnt::new(5.0, 0.0, 0.0), CONFUSION));
    assert!(!l.contains(Pnt::new(5.0, 1.0, 0.0), CONFUSION));
}

#[test]
fn lin_angle() {
    let l1 = Lin::new_pd(Pnt::origin(), Pnt::dir(1.0, 0.0, 0.0));
    let l2 = Lin::new_pd(Pnt::origin(), Pnt::dir(0.0, 1.0, 0.0));
    near(l1.angle(&l2), FRAC_PI_2, ANGULAR);
}

#[test]
fn lin_translate() {
    let l = Lin::new_pd(Pnt::origin(), Pnt::dir(1.0, 0.0, 0.0));
    let t = l.translated(Pnt::new(0.0, 5.0, 0.0));
    near(t.location().y, 5.0, CONFUSION);
}

#[test]
fn lin_rotate() {
    let l = Lin::new_pd(Pnt::new(1.0, 0.0, 0.0), Pnt::dir(1.0, 0.0, 0.0));
    let axis = Ax1::new(Pnt::origin(), Pnt::dir(0.0, 0.0, 1.0));
    let r = l.rotated(axis, FRAC_PI_2);
    near(r.location().x, 0.0, CONFUSION);
    near(r.location().y, 1.0, CONFUSION);
    assert!(r.direction().is_equal_dir(Pnt::dir(0.0, 1.0, 0.0), ANGULAR));
}

#[test]
fn lin_mirror() {
    let l = Lin::new_pd(Pnt::new(1.0, 1.0, 0.0), Pnt::dir(1.0, 0.0, 0.0));
    let axis = Ax1::new(Pnt::origin(), Pnt::dir(1.0, 0.0, 0.0));
    let m = l.mirrored_axis(axis);
    near(m.location().y, -1.0, CONFUSION);
}

#[test]
fn lin_transform() {
    let l = Lin::new_pd(Pnt::origin(), Pnt::dir(1.0, 0.0, 0.0));
    let mut t = Trsf::identity();
    t.set_translation(Pnt::new(0.0, 0.0, 10.0));
    near(l.transformed(&t).location().z, 10.0, CONFUSION);
}

// ====================== gp_Circ_Test.cxx ======================

#[test]
fn circ_constructor_radius() {
    let c = Circ::new(ax2(Pnt::origin(), Pnt::dir(0.0, 0.0, 1.0)), 5.0);
    near(c.radius(), 5.0, CONFUSION);
}

#[test]
fn circ_area() {
    let c = Circ::new(ax2(Pnt::origin(), Pnt::dir(0.0, 0.0, 1.0)), 1.0);
    near(c.area(), PI, CONFUSION);
}

#[test]
fn circ_length() {
    let c = Circ::new(ax2(Pnt::origin(), Pnt::dir(0.0, 0.0, 1.0)), 1.0);
    near(c.length(), 2.0 * PI, CONFUSION);
}

#[test]
fn circ_center() {
    let center = Pnt::new(1.0, 2.0, 3.0);
    let c = Circ::new(ax2(center, Pnt::dir(0.0, 0.0, 1.0)), 5.0);
    assert!(c.location().is_equal(center, CONFUSION));
}

#[test]
fn circ_translate() {
    let c = Circ::new(ax2(Pnt::origin(), Pnt::dir(0.0, 0.0, 1.0)), 5.0);
    let t = c.translated(Pnt::new(10.0, 20.0, 30.0));
    near(t.location().x, 10.0, CONFUSION);
    near(t.location().y, 20.0, CONFUSION);
    near(t.location().z, 30.0, CONFUSION);
    near(t.radius(), 5.0, CONFUSION);
}

#[test]
fn circ_rotate() {
    let c = Circ::new(ax2(Pnt::new(1.0, 0.0, 0.0), Pnt::dir(0.0, 0.0, 1.0)), 2.0);
    let r = c.rotated(Ax1::new(Pnt::origin(), Pnt::dir(0.0, 0.0, 1.0)), FRAC_PI_2);
    near(r.location().x, 0.0, CONFUSION);
    near(r.location().y, 1.0, CONFUSION);
    near(r.radius(), 2.0, CONFUSION);
}

#[test]
fn circ_mirror() {
    let c = Circ::new(ax2(Pnt::new(1.0, 2.0, 3.0), Pnt::dir(0.0, 0.0, 1.0)), 5.0);
    let m = c.mirrored_point(Pnt::origin());
    near(m.location().x, -1.0, CONFUSION);
    near(m.location().y, -2.0, CONFUSION);
    near(m.location().z, -3.0, CONFUSION);
}

#[test]
fn circ_transform_scales_radius() {
    let c = Circ::new(ax2(Pnt::origin(), Pnt::dir(0.0, 0.0, 1.0)), 3.0);
    let mut t = Trsf::identity();
    t.set_scale(Pnt::origin(), 2.0);
    near(c.transformed(&t).radius(), 6.0, CONFUSION);
}

// ====================== gp_Pln_Test.cxx ======================

#[test]
fn pln_constructor_from_point_and_dir() {
    let p = Pln::new_pd(Pnt::origin(), Pnt::dir(0.0, 0.0, 1.0));
    assert!(p.location().is_equal(Pnt::origin(), CONFUSION));
    assert!(p
        .axis_direction()
        .is_equal_dir(Pnt::dir(0.0, 0.0, 1.0), ANGULAR));
}

#[test]
fn pln_constructor_from_coefficients() {
    // Z = 5  =>  0x + 0y + 1z - 5 = 0
    let p = Pln::new_coeffs(0.0, 0.0, 1.0, -5.0);
    let (a, b, c, d) = p.coefficients();
    near(a, 0.0, CONFUSION);
    near(b, 0.0, CONFUSION);
    near(c, 1.0, CONFUSION);
    near(d, -5.0, CONFUSION);
}

#[test]
fn pln_distance_to_point() {
    let p = Pln::new_pd(Pnt::origin(), Pnt::dir(0.0, 0.0, 1.0));
    near(p.distance_point(Pnt::new(3.0, 4.0, 5.0)), 5.0, CONFUSION);
}

#[test]
fn pln_contains() {
    let p = Pln::new_pd(Pnt::origin(), Pnt::dir(0.0, 0.0, 1.0));
    assert!(p.contains(Pnt::new(3.0, 4.0, 0.0), CONFUSION));
    assert!(!p.contains(Pnt::new(3.0, 4.0, 1.0), CONFUSION));
}

#[test]
fn pln_mirror() {
    let p = Pln::new_pd(Pnt::new(0.0, 0.0, 5.0), Pnt::dir(0.0, 0.0, 1.0));
    let mirror = ax2(Pnt::origin(), Pnt::dir(0.0, 0.0, 1.0));
    near(p.mirrored_plane(mirror).location().z, -5.0, CONFUSION);
}

#[test]
fn pln_rotate() {
    let p = Pln::new_pd(Pnt::new(1.0, 0.0, 0.0), Pnt::dir(0.0, 0.0, 1.0));
    let r = p.rotated(Ax1::new(Pnt::origin(), Pnt::dir(0.0, 0.0, 1.0)), FRAC_PI_2);
    near(r.location().x, 0.0, CONFUSION);
    near(r.location().y, 1.0, CONFUSION);
}

#[test]
fn pln_scale() {
    let p = Pln::new_pd(Pnt::new(1.0, 0.0, 0.0), Pnt::dir(0.0, 0.0, 1.0));
    near(p.scaled(Pnt::origin(), 3.0).location().x, 3.0, CONFUSION);
}

#[test]
fn pln_translate() {
    let p = Pln::new_pd(Pnt::origin(), Pnt::dir(0.0, 0.0, 1.0));
    let t = p.translated(Pnt::new(1.0, 2.0, 3.0));
    near(t.location().x, 1.0, CONFUSION);
    near(t.location().y, 2.0, CONFUSION);
    near(t.location().z, 3.0, CONFUSION);
}

#[test]
fn pln_transform() {
    let p = Pln::new_pd(Pnt::origin(), Pnt::dir(0.0, 0.0, 1.0));
    let mut t = Trsf::identity();
    t.set_translation(Pnt::new(0.0, 0.0, 10.0));
    near(p.transformed(&t).location().z, 10.0, CONFUSION);
}
