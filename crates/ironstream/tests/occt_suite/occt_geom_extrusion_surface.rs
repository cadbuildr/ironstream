//! Tests for `Geom_SurfaceOfLinearExtrusion` — ported from OCCT test suite.
//!
//! Tolerance constants from `Precision`:
//!   CONFUSION = 1e-7  (linear coincidence)
//!   ANGULAR   = 1e-12 (angular)

use ironstream::geom_circle::GeomCircle;
use ironstream::geom_ellipse::GeomEllipse;
use ironstream::geom_extrusion_surface::{GeomCurve, GeomSurfaceOfLinearExtrusion};
use ironstream::geom_line::GeomLine;
use ironstream::gp::{Ax1, Ax3, Pnt, Trsf};
use ironstream::precision::{ANGULAR, CONFUSION};
use std::f64::consts::PI;

// ── helpers ──────────────────────────────────────────────────────────────────

fn pnt_eq(a: Pnt, b: Pnt, tol: f64) -> bool {
    a.distance(b) < tol
}

fn vec_eq(a: Pnt, b: Pnt, tol: f64) -> bool {
    (a - b).norm() < tol
}

/// Build a default circle-based extrusion surface swept along +Z.
///
/// The circle has radius `r` in the XY plane (identity frame);
/// the direction is `(0, 0, 1)`.
fn circle_extrusion(r: f64) -> GeomSurfaceOfLinearExtrusion {
    let circle = GeomCircle::from_ax3(Ax3::identity(), r);
    let dir = Pnt::new(0.0, 0.0, 1.0);
    GeomSurfaceOfLinearExtrusion::new(GeomCurve::Circle(circle), dir)
}

/// Build a line-based extrusion surface: basis is the +X axis (line through
/// origin in direction X), swept along +Z.
fn line_extrusion() -> GeomSurfaceOfLinearExtrusion {
    let line = GeomLine::from_point_dir(Pnt::origin(), Pnt::new(1.0, 0.0, 0.0));
    let dir = Pnt::new(0.0, 0.0, 1.0);
    GeomSurfaceOfLinearExtrusion::new(GeomCurve::Line(line), dir)
}

// ── test 1: constructor stores direction and basis curve ──────────────────────
#[test]
fn test_constructor_direction_and_basis_curve() {
    let surf = circle_extrusion(3.0);

    // Direction should be unit +Z.
    let d = surf.direction();
    assert!(vec_eq(d, Pnt::new(0.0, 0.0, 1.0), CONFUSION));

    // U range inherited from circle: [0, 2π]
    assert!((surf.u_first_parameter() - 0.0).abs() < CONFUSION);
    assert!((surf.u_last_parameter() - 2.0 * PI).abs() < CONFUSION);
}

// ── test 2: Value / D0 on circle extrusion ────────────────────────────────────
#[test]
fn test_value_circle_extrusion() {
    let r = 5.0;
    let surf = circle_extrusion(r);

    // At (U=0, V=0): circle at U=0 is (R,0,0), V=0 no offset → (R, 0, 0).
    let p = surf.value(0.0, 0.0);
    assert!(pnt_eq(p, Pnt::new(r, 0.0, 0.0), CONFUSION));

    // At (U=π/2, V=0): circle at U=π/2 is (0,R,0).
    let p = surf.value(PI / 2.0, 0.0);
    assert!(pnt_eq(p, Pnt::new(0.0, r, 0.0), CONFUSION));

    // At (U=0, V=3): circle at U=0 is (R,0,0), translate by 3 along Z.
    let p = surf.value(0.0, 3.0);
    assert!(pnt_eq(p, Pnt::new(r, 0.0, 3.0), CONFUSION));

    // At (U=π, V=-2): circle at U=π is (-R,0,0), translate by -2 along Z.
    let p = surf.value(PI, -2.0);
    assert!(pnt_eq(p, Pnt::new(-r, 0.0, -2.0), CONFUSION));

    // D0 matches value.
    let p1 = surf.d0(1.0, 2.0);
    let p2 = surf.value(1.0, 2.0);
    assert!(pnt_eq(p1, p2, CONFUSION));
}

// ── test 3: D1 partial derivatives on circle extrusion ───────────────────────
#[test]
fn test_d1_circle_extrusion() {
    let r = 4.0;
    let surf = circle_extrusion(r);

    // At U=0:
    // DU = C'(0) = -R*sin(0)*X + R*cos(0)*Y = (0, R, 0)
    // DV = Direction = (0, 0, 1)
    let (p, du, dv) = surf.d1(0.0, 0.0);
    assert!(pnt_eq(p, Pnt::new(r, 0.0, 0.0), CONFUSION));
    assert!(vec_eq(du, Pnt::new(0.0, r, 0.0), CONFUSION));
    assert!(vec_eq(dv, Pnt::new(0.0, 0.0, 1.0), CONFUSION));

    // At U=π/2:
    // DU = C'(π/2) = -R*sin(π/2)*X + R*cos(π/2)*Y = (-R, 0, 0)
    // DV = Direction = (0, 0, 1)
    let (_, du2, dv2) = surf.d1(PI / 2.0, 5.0);
    assert!(vec_eq(du2, Pnt::new(-r, 0.0, 0.0), CONFUSION));
    assert!(vec_eq(dv2, Pnt::new(0.0, 0.0, 1.0), CONFUSION));

    // |DU| = R always (for circle basis).
    let (_, du3, _) = surf.d1(1.23, 4.56);
    assert!((du3.norm() - r).abs() < CONFUSION);

    // DV is always the unit extrusion direction.
    assert!((dv.norm() - 1.0).abs() < CONFUSION);
}

// ── test 4: D2 second-order derivatives on circle extrusion ──────────────────
#[test]
fn test_d2_circle_extrusion() {
    let r = 3.0;
    let surf = circle_extrusion(r);

    // At U=0:
    // D2U = C''(0) = -R*cos(0)*X - R*sin(0)*Y = (-R, 0, 0)
    // D2V = 0 (linear in V)
    // D2UV = 0 (U and V decouple)
    let (_, _, _, d2u, d2v, d2uv) = surf.d2(0.0, 0.0);
    assert!(vec_eq(d2u, Pnt::new(-r, 0.0, 0.0), CONFUSION));
    assert!(vec_eq(d2v, Pnt::origin(), CONFUSION));
    assert!(vec_eq(d2uv, Pnt::origin(), CONFUSION));

    // At arbitrary U: D2U = C''(U) = -R*cos(U)*X - R*sin(U)*Y
    let u = 1.1;
    let (_, _, _, d2u2, _, _) = surf.d2(u, 2.0);
    let expected = Pnt::new(-r * u.cos(), -r * u.sin(), 0.0);
    assert!(vec_eq(d2u2, expected, CONFUSION));
}

// ── test 5: DN higher-order derivatives on circle extrusion ──────────────────
#[test]
fn test_dn_circle_extrusion() {
    let r = 2.0;
    let surf = circle_extrusion(r);
    let u = 0.7;
    let v = 1.5;

    // DN(nu=1, nv=0) = DU = C'(U) = -R*sin(U)*X + R*cos(U)*Y
    let dn10 = surf.dn(u, v, 1, 0);
    let expected = Pnt::new(-r * u.sin(), r * u.cos(), 0.0);
    assert!(vec_eq(dn10, expected, CONFUSION));

    // DN(nu=0, nv=1) = Direction = (0, 0, 1)
    let dn01 = surf.dn(u, v, 0, 1);
    assert!(vec_eq(dn01, Pnt::new(0.0, 0.0, 1.0), CONFUSION));

    // DN(nu=0, nv=2) = 0 (V is linear)
    let dn02 = surf.dn(u, v, 0, 2);
    assert!(vec_eq(dn02, Pnt::origin(), CONFUSION));

    // DN(nu=2, nv=0) = D2U = -R*cos(U)*X - R*sin(U)*Y
    let dn20 = surf.dn(u, v, 2, 0);
    let expected_d2 = Pnt::new(-r * u.cos(), -r * u.sin(), 0.0);
    assert!(vec_eq(dn20, expected_d2, CONFUSION));

    // DN(nu=1, nv=1) = 0 (mixed derivative is zero)
    let dn11 = surf.dn(u, v, 1, 1);
    assert!(vec_eq(dn11, Pnt::origin(), CONFUSION));

    // DN(nu=4, nv=0) = 4th U-derivative of circle = R*cos(U)*X + R*sin(U)*Y
    let dn40 = surf.dn(u, v, 4, 0);
    let expected_d4 = Pnt::new(r * u.cos(), r * u.sin(), 0.0);
    assert!(vec_eq(dn40, expected_d4, CONFUSION));
}

// ── test 6: UIso is a line through C(U) along Direction ──────────────────────
#[test]
fn test_u_iso() {
    let r = 5.0;
    let surf = circle_extrusion(r);

    // UIso at U=0: line through (R,0,0) in direction Z.
    let iso = surf.u_iso(0.0);
    // D0(0) on the iso line = origin of the line = (R,0,0).
    let p0 = iso.d0(0.0);
    assert!(pnt_eq(p0, Pnt::new(r, 0.0, 0.0), CONFUSION));
    // D0(3) on the iso line = (R,0,3).
    let p3 = iso.d0(3.0);
    assert!(pnt_eq(p3, Pnt::new(r, 0.0, 3.0), CONFUSION));

    // UIso at U=π/2: line through (0,R,0) in direction Z.
    let iso2 = surf.u_iso(PI / 2.0);
    let q0 = iso2.d0(0.0);
    assert!(pnt_eq(q0, Pnt::new(0.0, r, 0.0), CONFUSION));

    // The direction of any UIso is the extrusion direction.
    let (_, dir) = iso.d1(0.0);
    assert!(vec_eq(dir, Pnt::new(0.0, 0.0, 1.0), CONFUSION));
}

// ── test 7: VIso is a translated copy of the basis curve ─────────────────────
#[test]
fn test_v_iso() {
    let r = 4.0;
    let surf = circle_extrusion(r);

    // VIso at V=0: same as basis curve (no translation).
    let iso_v0 = surf.v_iso(0.0);
    let p_v0 = iso_v0.value(0.0);
    assert!(pnt_eq(p_v0, Pnt::new(r, 0.0, 0.0), CONFUSION));

    // VIso at V=7: circle translated by 7 along Z.
    let iso_v7 = surf.v_iso(7.0);
    // At U=0 on this iso-curve: (R, 0, 7)
    let p_v7 = iso_v7.value(0.0);
    assert!(pnt_eq(p_v7, Pnt::new(r, 0.0, 7.0), CONFUSION));

    // At U=π/2 on VIso(3): (0, R, 3)
    let iso_v3 = surf.v_iso(3.0);
    let p_pi2 = iso_v3.value(PI / 2.0);
    assert!(pnt_eq(p_pi2, Pnt::new(0.0, r, 3.0), CONFUSION));
}

// ── test 8: is_u_closed / is_v_closed / is_u_periodic / is_v_periodic ────────
#[test]
fn test_topology_flags() {
    // Circle basis — closed and periodic in U.
    let surf_circ = circle_extrusion(2.0);
    assert!(surf_circ.is_u_closed(), "circle extrusion must be closed in U");
    assert!(
        !surf_circ.is_v_closed(),
        "extrusion must NOT be closed in V"
    );
    assert!(
        surf_circ.is_u_periodic(),
        "circle extrusion must be periodic in U"
    );
    assert!(
        !surf_circ.is_v_periodic(),
        "extrusion must NOT be periodic in V"
    );

    // Line basis — not closed, not periodic.
    let surf_line = line_extrusion();
    assert!(
        !surf_line.is_u_closed(),
        "line extrusion must NOT be closed in U"
    );
    assert!(
        !surf_line.is_u_periodic(),
        "line extrusion must NOT be periodic in U"
    );
}

// ── test 9: Transform — translation ──────────────────────────────────────────
#[test]
fn test_transform_translation() {
    let r = 3.0;
    let surf = circle_extrusion(r);
    let t = Trsf::translation(Pnt::new(10.0, 20.0, 30.0));
    let surf2 = surf.transformed(&t);

    // Direction unchanged (translation doesn't rotate).
    let d = surf2.direction();
    assert!(vec_eq(d, Pnt::new(0.0, 0.0, 1.0), CONFUSION));

    // Value at (U=0, V=0): was (R,0,0), now (R+10, 20, 30).
    let p = surf2.value(0.0, 0.0);
    assert!(pnt_eq(p, Pnt::new(r + 10.0, 20.0, 30.0), CONFUSION));

    // Value at (U=0, V=5): was (R,0,5), now (R+10, 20, 35).
    let p2 = surf2.value(0.0, 5.0);
    assert!(pnt_eq(p2, Pnt::new(r + 10.0, 20.0, 35.0), CONFUSION));
}

// ── test 10: Transform — rotation ────────────────────────────────────────────
#[test]
fn test_transform_rotation() {
    let r = 2.0;
    let surf = circle_extrusion(r);

    // Rotate 90° about Z axis: X→Y, Y→−X, Z→Z.
    let t = Trsf::rotation(Ax1::oz(), PI / 2.0);
    let surf2 = surf.transformed(&t);

    // Direction (was Z) stays Z after rotation about Z.
    let d = surf2.direction();
    assert!(vec_eq(d, Pnt::new(0.0, 0.0, 1.0), CONFUSION));

    // The basis circle's X direction was +X; after 90° rotation it becomes +Y.
    // So value(0, 0) which was (R,0,0) becomes (0,R,0).
    let p = surf2.value(0.0, 0.0);
    assert!(pnt_eq(p, Pnt::new(0.0, r, 0.0), CONFUSION));
}

// ── test 11: Transform — direction is transformed ────────────────────────────
#[test]
fn test_transform_direction_rotated() {
    // Start with extrusion along +X.
    let line = GeomLine::from_point_dir(Pnt::origin(), Pnt::new(0.0, 1.0, 0.0));
    let dir_x = Pnt::new(1.0, 0.0, 0.0);
    let surf = GeomSurfaceOfLinearExtrusion::new(GeomCurve::Line(line), dir_x);

    // Rotate 90° about Z: X→Y, so the extrusion direction becomes +Y.
    let t = Trsf::rotation(Ax1::oz(), PI / 2.0);
    let mut surf2 = surf.clone();
    surf2.transform(&t);

    let d = surf2.direction();
    assert!(vec_eq(d, Pnt::new(0.0, 1.0, 0.0), CONFUSION));
}

// ── test 12: Ellipse basis curve extrusion ───────────────────────────────────
#[test]
fn test_ellipse_basis_extrusion() {
    let major = 5.0;
    let minor = 3.0;
    let ellipse = GeomEllipse::from_ax3(Ax3::identity(), major, minor);
    let dir = Pnt::new(0.0, 0.0, 1.0);
    let surf =
        GeomSurfaceOfLinearExtrusion::new(GeomCurve::Ellipse(ellipse), dir);

    // P(0, 0) = ellipse at U=0 = (major, 0, 0)
    let p = surf.value(0.0, 0.0);
    assert!(pnt_eq(p, Pnt::new(major, 0.0, 0.0), CONFUSION));

    // P(π/2, 0) = (0, minor, 0)
    let p2 = surf.value(PI / 2.0, 0.0);
    assert!(pnt_eq(p2, Pnt::new(0.0, minor, 0.0), CONFUSION));

    // P(0, 4) = (major, 0, 4)
    let p3 = surf.value(0.0, 4.0);
    assert!(pnt_eq(p3, Pnt::new(major, 0.0, 4.0), CONFUSION));

    // The ellipse basis is closed and periodic.
    assert!(surf.is_u_closed());
    assert!(surf.is_u_periodic());
    assert!(!surf.is_v_periodic());
}

// ── test 13: set_direction and set_basis_curve ────────────────────────────────
#[test]
fn test_setters() {
    let mut surf = circle_extrusion(2.0);

    // Change direction to +Y.
    surf.set_direction(Pnt::new(0.0, 1.0, 0.0));
    let d = surf.direction();
    assert!(vec_eq(d, Pnt::new(0.0, 1.0, 0.0), CONFUSION));

    // The direction is normalized: supplying a non-unit vector.
    surf.set_direction(Pnt::new(0.0, 2.0, 0.0)); // magnitude 2, should normalize to (0,1,0)
    let d2 = surf.direction();
    assert!((d2.norm() - 1.0).abs() < CONFUSION);
    assert!(vec_eq(d2, Pnt::new(0.0, 1.0, 0.0), CONFUSION));

    // Replace basis curve with a line.
    let line = GeomLine::from_point_dir(Pnt::new(1.0, 0.0, 0.0), Pnt::new(0.0, 1.0, 0.0));
    surf.set_basis_curve(GeomCurve::Line(line));
    // V range should still be infinite; U range now inherits from line.
    let u_lo = surf.u_first_parameter();
    let u_hi = surf.u_last_parameter();
    assert!(u_lo < -1e100);
    assert!(u_hi > 1e100);
}

// ── test 14: copy independence ────────────────────────────────────────────────
#[test]
fn test_copy_independence() {
    let surf = circle_extrusion(3.0);
    let surf_copy = surf.copy();

    // Value on original.
    let p_orig = surf.value(0.0, 0.0);
    let p_copy = surf_copy.value(0.0, 0.0);
    assert!(pnt_eq(p_orig, p_copy, CONFUSION));

    // Transform original only.
    let t = Trsf::translation(Pnt::new(100.0, 0.0, 0.0));
    let surf_moved = surf.transformed(&t);

    let p_moved = surf_moved.value(0.0, 0.0);
    let p_copy_still = surf_copy.value(0.0, 0.0);

    // Copy should be unaffected.
    assert!(!pnt_eq(p_moved, p_copy_still, 1.0));
}

// ── test 15: V reversed parameter ────────────────────────────────────────────
#[test]
fn test_reversed_parameters() {
    let surf = circle_extrusion(1.0);

    // VReversedParameter: -V
    assert!((surf.v_reversed_parameter(5.0) - (-5.0)).abs() < CONFUSION);
    assert!((surf.v_reversed_parameter(-3.0) - 3.0).abs() < CONFUSION);
    assert!((surf.v_reversed_parameter(0.0)).abs() < CONFUSION);

    // UReversedParameter: -U
    assert!((surf.u_reversed_parameter(1.0) - (-1.0)).abs() < CONFUSION);
    assert!((surf.u_reversed_parameter(0.0)).abs() < CONFUSION);
}

// ── test 16: line basis — value and D1 ───────────────────────────────────────
#[test]
fn test_line_basis_value_and_d1() {
    let surf = line_extrusion();

    // Basis line: P_line(U) = (U, 0, 0). Direction: Z.
    // Surface: P(U, V) = (U, 0, V).
    let p = surf.value(3.0, 4.0);
    assert!(pnt_eq(p, Pnt::new(3.0, 0.0, 4.0), CONFUSION));

    let (pt, du, dv) = surf.d1(2.0, 1.0);
    assert!(pnt_eq(pt, Pnt::new(2.0, 0.0, 1.0), CONFUSION));
    // DU = C'(U) for a line along X = (1, 0, 0)
    assert!(vec_eq(du, Pnt::new(1.0, 0.0, 0.0), CONFUSION));
    // DV = Z = (0, 0, 1)
    assert!(vec_eq(dv, Pnt::new(0.0, 0.0, 1.0), CONFUSION));
}

// ── test 17: u_period for periodic circle basis ───────────────────────────────
#[test]
fn test_u_period_circle() {
    let surf = circle_extrusion(1.0);

    // Circle: first=0, last=2π, period should be 2π.
    let period = surf.u_period();
    assert!((period - 2.0 * PI).abs() < ANGULAR.sqrt()); // generous angular tol here
}

// ── test 18: non-identity placement — oblique extrusion ──────────────────────
#[test]
fn test_oblique_extrusion() {
    // Circle in the XZ plane (normal = Y), swept along +Y.
    let frame = Ax3 {
        location: Pnt::new(0.0, 0.0, 0.0),
        x_dir: Pnt::new(1.0, 0.0, 0.0),
        y_dir: Pnt::new(0.0, 0.0, 1.0), // XZ plane
        z_dir: Pnt::new(0.0, 1.0, 0.0), // normal = Y
    };
    let r = 2.0;
    let circle = GeomCircle::from_ax3(frame, r);
    let dir_y = Pnt::new(0.0, 1.0, 0.0);
    let surf = GeomSurfaceOfLinearExtrusion::new(GeomCurve::Circle(circle), dir_y);

    // C(0) = origin + R*XDir + 0*YDir = (R, 0, 0).
    // P(0, 0) = (R, 0, 0).
    let p = surf.value(0.0, 0.0);
    assert!(pnt_eq(p, Pnt::new(r, 0.0, 0.0), CONFUSION));

    // C(π/2) = origin + 0*XDir + R*YDir = (0, 0, R) (because YDir = +Z).
    // P(π/2, 0) = (0, 0, R).
    let p2 = surf.value(PI / 2.0, 0.0);
    assert!(pnt_eq(p2, Pnt::new(0.0, 0.0, r), CONFUSION));

    // P(0, 3) = (R, 0, 0) + 3*(0,1,0) = (R, 3, 0).
    let p3 = surf.value(0.0, 3.0);
    assert!(pnt_eq(p3, Pnt::new(r, 3.0, 0.0), CONFUSION));

    // Direction should be +Y.
    let d = surf.direction();
    assert!(vec_eq(d, Pnt::new(0.0, 1.0, 0.0), CONFUSION));
}
