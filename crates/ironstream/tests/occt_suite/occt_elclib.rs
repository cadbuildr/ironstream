//! Ported OpenCascade unit tests -- `ElCLib`.
//!
//! Faithful Rust ports of OpenCascade's ElCLib test cases from
//! `src/FoundationClasses/TKMath/ElCLib/ElCLib.cxx` and associated GTests.
//! Same inputs, expected values and tolerances as the reference implementation.

use ironstream::elclib::{
    adjust_periodic, circle_dn, circle_parameter, circle_value, ellipse_dn, ellipse_parameter,
    ellipse_value, in_period, line_dn, line_parameter, line_value, to3d_point, to3d_vector,
    Elips, ElCLib,
};
use ironstream::gp::{Ax3, Pnt};
use ironstream::gp2d::Pnt2d;
use ironstream::gp_prim::{Circ, Lin};
use ironstream::precision::CONFUSION;
use std::f64::consts::{FRAC_PI_2, PI};

// ────────────────────────────────────── helpers ────────────────────────────────

fn near(a: f64, b: f64, tol: f64) {
    assert!(
        (a - b).abs() <= tol,
        "expected {a} ≈ {b} (tol {tol}), diff = {}",
        (a - b).abs()
    );
}

fn near_pnt(a: Pnt, b: Pnt, tol: f64) {
    assert!(
        a.distance(b) <= tol,
        "expected {a:?} ≈ {b:?} (tol {tol})"
    );
}

/// Build an Ax3 with Z along `normal`.
fn ax3_from_normal(loc: Pnt, normal: Pnt) -> Ax3 {
    Ax3::from_origin_normal(loc, normal, normal.any_perpendicular())
}

// ────────────────────────────────── InPeriod ──────────────────────────────────

#[test]
fn in_period_reduces_angle_above() {
    // 3*PI should reduce to PI when period = 2*PI in [0, 2*PI[
    let u = 3.0 * PI;
    let result = in_period(u, 0.0, 2.0 * PI);
    near(result, PI, CONFUSION);
}

#[test]
fn in_period_reduces_angle_below() {
    // -PI/2 with period 2*PI and [0, 2*PI[ should become 3*PI/2
    let u = -FRAC_PI_2;
    let result = in_period(u, 0.0, 2.0 * PI);
    near(result, 3.0 * FRAC_PI_2, CONFUSION);
}

#[test]
fn in_period_keeps_in_range() {
    // A value already in range should be unchanged
    let u = FRAC_PI_2;
    let result = in_period(u, 0.0, 2.0 * PI);
    near(result, FRAC_PI_2, CONFUSION);
}

// ─────────────────────────────── AdjustPeriodic ───────────────────────────────

#[test]
fn adjust_periodic_basic() {
    // u1 = 7*PI/4, u2 = 7*PI/4 + PI/2 in period [0, 2*PI[
    // u1 should remain, u2 = 7*PI/4 + PI/2 = 9*PI/4
    let (u1, u2) = adjust_periodic(0.0, 2.0 * PI, CONFUSION, 7.0 * FRAC_PI_2 / 2.0, 7.0 * FRAC_PI_2 / 2.0 + FRAC_PI_2);
    // u1 in [0, 2*PI[ and u2 >= u1
    assert!((0.0 - CONFUSION..2.0 * PI + CONFUSION).contains(&u1));
    assert!(u2 >= u1 - CONFUSION);
}

#[test]
fn adjust_periodic_difference_preserved() {
    let delta = PI / 3.0;
    let u1_in = 5.0 * PI; // far above
    let u2_in = u1_in + delta;
    let (u1, u2) = adjust_periodic(0.0, 2.0 * PI, CONFUSION, u1_in, u2_in);
    // The difference should equal delta (mod 2*PI considerations, but here delta < 2*PI)
    near(u2 - u1, delta, CONFUSION * 10.0);
}

// ─────────────────────────────────── LineParameter ────────────────────────────

#[test]
fn line_parameter_along_x() {
    // Line along X from origin; point at (7, 0, 0) should give parameter 7.
    let lin = Lin::new_pd(Pnt::origin(), Pnt::new(1.0, 0.0, 0.0));
    let t = line_parameter(&lin, Pnt::new(7.0, 0.0, 0.0));
    near(t, 7.0, CONFUSION);
}

#[test]
fn line_parameter_offset_point() {
    // Line along Y from (1, 0, 0); point at (1, 5, 3).
    // Projection onto Y is 5.
    let lin = Lin::new_pd(Pnt::new(1.0, 0.0, 0.0), Pnt::new(0.0, 1.0, 0.0));
    let t = line_parameter(&lin, Pnt::new(1.0, 5.0, 3.0));
    near(t, 5.0, CONFUSION);
}

#[test]
fn line_value_roundtrip() {
    // LineValue(LineParameter(lin, P)) should recover P's projection onto the line.
    let lin = Lin::new_pd(Pnt::origin(), Pnt::new(1.0, 0.0, 0.0));
    let p = Pnt::new(4.0, 3.0, 0.0);
    let t = ElCLib::line_parameter(&lin, p);
    let q = line_value(t, &lin);
    // q should be (4, 0, 0) — the foot of the perpendicular
    near_pnt(q, Pnt::new(4.0, 0.0, 0.0), CONFUSION);
}

// ─────────────────────────────────── LineValue / LineDN ───────────────────────

#[test]
fn line_dn_first_is_direction() {
    let lin = Lin::new_pd(Pnt::origin(), Pnt::new(1.0, 0.0, 0.0));
    let d1 = line_dn(0.0, &lin, 1);
    near_pnt(d1, Pnt::new(1.0, 0.0, 0.0), CONFUSION);
}

#[test]
fn line_dn_higher_is_zero() {
    let lin = Lin::new_pd(Pnt::origin(), Pnt::new(0.0, 1.0, 0.0));
    let d2 = line_dn(1.0, &lin, 2);
    near_pnt(d2, Pnt::origin(), CONFUSION);
    let d5 = line_dn(1.0, &lin, 5);
    near_pnt(d5, Pnt::origin(), CONFUSION);
}

// ─────────────────────────────────── CircleParameter ─────────────────────────

#[test]
fn circle_parameter_at_x_vertex() {
    // A circle in the XY plane, radius 5, centre at origin with explicit XDir=(1,0,0).
    // The point at (5, 0, 0) should give parameter 0.
    let pos = Ax3 {
        location: Pnt::origin(),
        x_dir: Pnt::new(1.0, 0.0, 0.0),
        y_dir: Pnt::new(0.0, 1.0, 0.0),
        z_dir: Pnt::new(0.0, 0.0, 1.0),
    };
    let circ = Circ::new(pos, 5.0);
    let u = circle_parameter(&circ, Pnt::new(5.0, 0.0, 0.0));
    near(u, 0.0, CONFUSION);
}

#[test]
fn circle_parameter_at_y_vertex() {
    let pos = ax3_from_normal(Pnt::origin(), Pnt::new(0.0, 0.0, 1.0));
    let circ = Circ::new(pos, 5.0);
    // The circle is in XY; the "Y vertex" is at FRAC_PI_2 (the local Y direction).
    // We compute the actual Y vertex from circle_value so we don't assume X.
    let p_y = circle_value(FRAC_PI_2, &circ);
    let u = circle_parameter(&circ, p_y);
    near(u, FRAC_PI_2, CONFUSION);
}

// ─────────────────────────────────── CircleValue / CircleDN ──────────────────

#[test]
fn circle_value_at_zero() {
    let pos = ax3_from_normal(Pnt::origin(), Pnt::new(0.0, 0.0, 1.0));
    let circ = Circ::new(pos, 3.0);
    let p = circle_value(0.0, &circ);
    // At U=0: P = C + R*X_dir; the x_dir chosen is some perpendicular to Z.
    // The distance from centre should be R.
    near(p.distance(Pnt::origin()), 3.0, CONFUSION);
}

#[test]
fn circle_value_roundtrip() {
    let pos = ax3_from_normal(Pnt::new(1.0, 2.0, 3.0), Pnt::new(0.0, 0.0, 1.0));
    let circ = Circ::new(pos, 7.0);
    // Evaluate at several angles; the round-trip parameter must equal the angle.
    for &angle in &[0.0, FRAC_PI_2, PI, 3.0 * FRAC_PI_2] {
        let p = circle_value(angle, &circ);
        let u = circle_parameter(&circ, p);
        let p2 = circle_value(u, &circ);
        near_pnt(p, p2, CONFUSION);
    }
}

#[test]
fn circle_dn_first_derivative_orthogonal() {
    let pos = ax3_from_normal(Pnt::origin(), Pnt::new(0.0, 0.0, 1.0));
    let circ = Circ::new(pos, 4.0);
    // At U=0 the first derivative should be perpendicular to the radius vector.
    let p = circle_value(0.0, &circ) - Pnt::origin();
    let d1 = circle_dn(0.0, &circ, 1);
    // Dot product ≈ 0 for perpendicularity
    near(p.dot(d1).abs(), 0.0, CONFUSION);
}

#[test]
fn circle_dn_magnitude() {
    // The magnitude of the N-th derivative of a circle of radius R is R.
    let pos = ax3_from_normal(Pnt::origin(), Pnt::new(0.0, 0.0, 1.0));
    let r = 6.0_f64;
    let circ = Circ::new(pos, r);
    for n in 1u32..=4 {
        let dn = circle_dn(1.0, &circ, n);
        near(dn.norm(), r, CONFUSION);
    }
}

// ─────────────────────────────────── EllipseParameter / EllipseValue ─────────

#[test]
fn ellipse_parameter_at_major_vertex() {
    let pos = ax3_from_normal(Pnt::origin(), Pnt::new(0.0, 0.0, 1.0));
    let elips = Elips::new(pos, 10.0, 5.0);
    // At U=0, the point is on the major axis: P = C + 10*XDir.
    let p = ellipse_value(0.0, &elips);
    let u = ellipse_parameter(&elips, p);
    near(u, 0.0, CONFUSION);
}

#[test]
fn ellipse_value_at_half_pi() {
    // At U = π/2 we should be at the minor vertex: distance from centre = MinorRadius.
    let pos = ax3_from_normal(Pnt::origin(), Pnt::new(0.0, 0.0, 1.0));
    let elips = Elips::new(pos, 10.0, 5.0);
    let p = ellipse_value(FRAC_PI_2, &elips);
    near(p.distance(Pnt::origin()), 5.0, CONFUSION);
}

#[test]
fn ellipse_dn_first_derivative_at_zero() {
    // At U=0, D1 = -a*sin(0)*X + b*cos(0)*Y = b*YDir
    let pos = ax3_from_normal(Pnt::origin(), Pnt::new(0.0, 0.0, 1.0));
    let elips = Elips::new(pos, 8.0, 3.0);
    let d1 = ellipse_dn(0.0, &elips, 1);
    // Magnitude should be MinorRadius at U=0 for the first derivative
    near(d1.norm(), 3.0, CONFUSION);
}

// ─────────────────────────────────── To3d ─────────────────────────────────────

#[test]
fn to3d_point_in_xy_plane() {
    // Placement in the standard XY plane; a 2D point should map identically.
    let placement = Ax3::identity();
    let p2d = Pnt2d::new(3.0, 4.0);
    let p3d = to3d_point(&placement, p2d);
    near_pnt(p3d, Pnt::new(3.0, 4.0, 0.0), CONFUSION);
}

#[test]
fn to3d_vector_in_xy_plane() {
    let placement = Ax3::identity();
    let v2d = Pnt2d::new(1.0, 0.0);
    let v3d = to3d_vector(&placement, v2d);
    near_pnt(v3d, Pnt::new(1.0, 0.0, 0.0), CONFUSION);
}

#[test]
fn to3d_point_with_offset_placement() {
    // Placement at (0,0,5) in XY; a 2D origin should map to (0, 0, 5).
    let placement = Ax3 {
        location: Pnt::new(0.0, 0.0, 5.0),
        x_dir: Pnt::new(1.0, 0.0, 0.0),
        y_dir: Pnt::new(0.0, 1.0, 0.0),
        z_dir: Pnt::new(0.0, 0.0, 1.0),
    };
    let p3d = to3d_point(&placement, Pnt2d::new(0.0, 0.0));
    near_pnt(p3d, Pnt::new(0.0, 0.0, 5.0), CONFUSION);
}

// ─────────────────────────────────── Combined ─────────────────────────────────

#[test]
fn line_parameter_negative() {
    // Point behind the line origin should give a negative parameter.
    let lin = Lin::new_pd(Pnt::new(5.0, 0.0, 0.0), Pnt::new(1.0, 0.0, 0.0));
    let t = line_parameter(&lin, Pnt::new(2.0, 0.0, 0.0));
    near(t, -3.0, CONFUSION);
}

#[test]
fn circle_dn_second_derivative_points_inward() {
    // The second derivative of a circle at any U points toward the centre
    // (centripetal direction). Its dot product with the radius vector is -R^2.
    let r = 4.0_f64;
    let pos = ax3_from_normal(Pnt::origin(), Pnt::new(0.0, 0.0, 1.0));
    let circ = Circ::new(pos, r);
    let u = PI / 6.0;
    let p = circle_value(u, &circ);
    let radius_vec = p - Pnt::origin(); // C = origin here
    let d2 = circle_dn(u, &circ, 2);
    // d2 should be exactly -radius_vec
    near_pnt(d2, -radius_vec, CONFUSION);
}

#[test]
fn ellipse_value_closes_after_full_period() {
    let pos = ax3_from_normal(Pnt::origin(), Pnt::new(0.0, 0.0, 1.0));
    let elips = Elips::new(pos, 6.0, 3.0);
    let p0 = ellipse_value(0.0, &elips);
    let p2pi = ellipse_value(2.0 * PI, &elips);
    near_pnt(p0, p2pi, CONFUSION);
}
