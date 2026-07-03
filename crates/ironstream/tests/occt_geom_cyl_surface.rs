//! Tests for `Geom_CylindricalSurface` — ported from OCCT test suite.
//!
//! Tolerance constants from `Precision`:
//!   CONFUSION = 1e-7  (linear coincidence)
//!   ANGULAR   = 1e-12 (angular)

use ironstream::geom_cyl_surface::GeomCylindricalSurface;
use ironstream::gp::{Ax1, Ax3, Pnt, Trsf};
use ironstream::precision::{ANGULAR, CONFUSION};
use std::f64::consts::PI;

// ── helpers ──────────────────────────────────────────────────────────────────

fn default_cylinder(radius: f64) -> GeomCylindricalSurface {
    GeomCylindricalSurface::new(Ax3::identity(), radius)
}

fn pnt_eq(a: Pnt, b: Pnt, tol: f64) -> bool {
    a.distance(b) < tol
}

fn vec_eq(a: Pnt, b: Pnt, tol: f64) -> bool {
    (a - b).norm() < tol
}

// ── test 1: constructor stores radius and position ────────────────────────────
#[test]
fn test_constructor_radius_and_position() {
    let cyl = default_cylinder(5.0);
    assert!((cyl.radius() - 5.0).abs() < CONFUSION);
    let pos = cyl.position();
    // Identity frame: location at origin, X=(1,0,0), Y=(0,1,0), Z=(0,0,1)
    assert!(pnt_eq(pos.location, Pnt::origin(), CONFUSION));
    assert!(vec_eq(pos.x_dir, Pnt::new(1.0, 0.0, 0.0), CONFUSION));
    assert!(vec_eq(pos.y_dir, Pnt::new(0.0, 1.0, 0.0), CONFUSION));
    assert!(vec_eq(pos.z_dir, Pnt::new(0.0, 0.0, 1.0), CONFUSION));
}

// ── test 2: Value / D0 at various (U, V) parameters ─────────────────────────
#[test]
fn test_value_d0() {
    let r = 3.0;
    let cyl = default_cylinder(r);

    // U=0, V=0 → (R, 0, 0)
    let p = cyl.value(0.0, 0.0);
    assert!(pnt_eq(p, Pnt::new(r, 0.0, 0.0), CONFUSION));

    // U=π/2, V=0 → (0, R, 0)
    let p = cyl.value(PI / 2.0, 0.0);
    assert!(pnt_eq(p, Pnt::new(0.0, r, 0.0), CONFUSION));

    // U=π, V=0 → (-R, 0, 0)
    let p = cyl.value(PI, 0.0);
    assert!(pnt_eq(p, Pnt::new(-r, 0.0, 0.0), CONFUSION));

    // U=3π/2, V=0 → (0, -R, 0)
    let p = cyl.value(3.0 * PI / 2.0, 0.0);
    assert!(pnt_eq(p, Pnt::new(0.0, -r, 0.0), CONFUSION));

    // U=0, V=7 → (R, 0, 7)
    let p = cyl.value(0.0, 7.0);
    assert!(pnt_eq(p, Pnt::new(r, 0.0, 7.0), CONFUSION));

    // D0 matches Value
    let p2 = cyl.d0(PI / 4.0, 2.0);
    let p3 = cyl.value(PI / 4.0, 2.0);
    assert!(pnt_eq(p2, p3, CONFUSION));
}

// ── test 3: D1 partial derivatives ──────────────────────────────────────────
#[test]
fn test_d1_derivatives() {
    let r = 4.0;
    let cyl = default_cylinder(r);

    // At U=0: DU = (0, R, 0), DV = (0, 0, 1)
    let (p, du, dv) = cyl.d1(0.0, 0.0);
    assert!(pnt_eq(p, Pnt::new(r, 0.0, 0.0), CONFUSION));
    assert!(vec_eq(du, Pnt::new(0.0, r, 0.0), CONFUSION));
    assert!(vec_eq(dv, Pnt::new(0.0, 0.0, 1.0), CONFUSION));

    // At U=π/2: DU = (-R, 0, 0), DV = (0, 0, 1)
    let (_, du2, dv2) = cyl.d1(PI / 2.0, 1.0);
    assert!(vec_eq(du2, Pnt::new(-r, 0.0, 0.0), CONFUSION));
    assert!(vec_eq(dv2, Pnt::new(0.0, 0.0, 1.0), CONFUSION));

    // |DU| = R always
    let (_, du3, _) = cyl.d1(1.23, 5.6);
    assert!((du3.norm() - r).abs() < CONFUSION);
}

// ── test 4: D2 second-order derivatives ─────────────────────────────────────
#[test]
fn test_d2_derivatives() {
    let r = 2.0;
    let cyl = default_cylinder(r);

    // At U=0: D2U = (-R, 0, 0) (centripetal), D2V = 0, D2UV = 0
    let (_, _, _, d2u, d2v, d2uv) = cyl.d2(0.0, 0.0);
    assert!(vec_eq(d2u, Pnt::new(-r, 0.0, 0.0), CONFUSION));
    assert!(vec_eq(d2v, Pnt::origin(), CONFUSION));
    assert!(vec_eq(d2uv, Pnt::origin(), CONFUSION));

    // D2U points toward the axis: D2U = -R*cos(U)*X - R*sin(U)*Y
    let u = 1.1;
    let (_, _, _, d2u2, _, _) = cyl.d2(u, 0.0);
    let expected_d2u = Pnt::new(-r * u.cos(), -r * u.sin(), 0.0);
    assert!(vec_eq(d2u2, expected_d2u, CONFUSION));
}

// ── test 5: DN higher-order derivatives ─────────────────────────────────────
#[test]
fn test_dn() {
    let r = 3.0;
    let cyl = default_cylinder(r);
    let u = 0.7;

    // DN(u, v, 1, 0) = DU = -R*sin(U)*X + R*cos(U)*Y
    let dn10 = cyl.dn(u, 0.0, 1, 0);
    let expected = Pnt::new(-r * u.sin(), r * u.cos(), 0.0);
    assert!(vec_eq(dn10, expected, CONFUSION));

    // DN(u, v, 0, 1) = DV = ZDir = (0,0,1)
    let dn01 = cyl.dn(u, 0.0, 0, 1);
    assert!(vec_eq(dn01, Pnt::new(0.0, 0.0, 1.0), CONFUSION));

    // DN(u, v, 0, 2) = 0 (V is linear)
    let dn02 = cyl.dn(u, 0.0, 0, 2);
    assert!(vec_eq(dn02, Pnt::origin(), CONFUSION));

    // DN(u, v, 2, 0) = D2U = -R*cos(U)*X - R*sin(U)*Y
    let dn20 = cyl.dn(u, 0.0, 2, 0);
    let expected_d2 = Pnt::new(-r * u.cos(), -r * u.sin(), 0.0);
    assert!(vec_eq(dn20, expected_d2, CONFUSION));

    // DN(u, v, 1, 1) = 0 (mixed derivative)
    let dn11 = cyl.dn(u, 0.0, 1, 1);
    assert!(vec_eq(dn11, Pnt::origin(), CONFUSION));

    // DN(u, v, 4, 0) = same as D0 direction scaled by R (4th derivative = original)
    let dn40 = cyl.dn(u, 0.0, 4, 0);
    let expected_d4 = Pnt::new(r * u.cos(), r * u.sin(), 0.0);
    assert!(vec_eq(dn40, expected_d4, CONFUSION));
}

// ── test 6: UIso and VIso ────────────────────────────────────────────────────
#[test]
fn test_u_iso_v_iso() {
    let r = 5.0;
    let cyl = default_cylinder(r);

    // UIso at U=0: line through (R,0,0) in direction Z
    let u_iso = cyl.u_iso(0.0);
    let p0 = u_iso.d0(0.0);
    assert!(pnt_eq(p0, Pnt::new(r, 0.0, 0.0), CONFUSION));
    let p1 = u_iso.d0(1.0);
    assert!(pnt_eq(p1, Pnt::new(r, 0.0, 1.0), CONFUSION));

    // UIso at U=π: line through (-R,0,0) in direction Z
    let u_iso2 = cyl.u_iso(PI);
    let q0 = u_iso2.d0(0.0);
    assert!(pnt_eq(q0, Pnt::new(-r, 0.0, 0.0), CONFUSION));

    // VIso at V=3: circle at height 3, same radius
    let v_iso = cyl.v_iso(3.0);
    assert!((v_iso.radius() - r).abs() < CONFUSION);
    // Center should be at (0, 0, 3)
    assert!(pnt_eq(v_iso.location(), Pnt::new(0.0, 0.0, 3.0), CONFUSION));
    // Point on VIso circle at U=0 should be (R, 0, 3)
    let cp = v_iso.d0(0.0);
    assert!(pnt_eq(cp, Pnt::new(r, 0.0, 3.0), CONFUSION));
}

// ── test 7: UReversedParameter, VReversedParameter ──────────────────────────
#[test]
fn test_reversed_parameters() {
    let cyl = default_cylinder(2.0);

    // UReversedParameter: 2*Pi - U
    assert!((cyl.u_reversed_parameter(0.0) - 2.0 * PI).abs() < CONFUSION);
    assert!((cyl.u_reversed_parameter(PI) - PI).abs() < CONFUSION);
    assert!((cyl.u_reversed_parameter(PI / 2.0) - 3.0 * PI / 2.0).abs() < CONFUSION);

    // VReversedParameter: -V
    assert!((cyl.v_reversed_parameter(5.0) - (-5.0)).abs() < CONFUSION);
    assert!((cyl.v_reversed_parameter(-3.0) - 3.0).abs() < CONFUSION);
    assert!((cyl.v_reversed_parameter(0.0)).abs() < CONFUSION);
}

// ── test 8: IsUClosed, IsVClosed, IsUPeriodic, IsVPeriodic, UPeriod ─────────
#[test]
fn test_topology_flags() {
    let cyl = default_cylinder(1.0);

    assert!(cyl.is_u_closed(), "cylinder must be closed in U");
    assert!(!cyl.is_v_closed(), "cylinder must NOT be closed in V");
    assert!(cyl.is_u_periodic(), "cylinder must be periodic in U");
    assert!(!cyl.is_v_periodic(), "cylinder must NOT be periodic in V");
    assert!((cyl.u_period() - 2.0 * PI).abs() < ANGULAR);
}

// ── test 9: Transform — translation ─────────────────────────────────────────
#[test]
fn test_transform_translation() {
    let r = 3.0;
    let mut cyl = default_cylinder(r);
    let t = Trsf::translation(Pnt::new(10.0, 20.0, 30.0));
    cyl.transform(&t);

    // Location should shift by (10, 20, 30)
    assert!(pnt_eq(cyl.location(), Pnt::new(10.0, 20.0, 30.0), CONFUSION));
    // Radius unchanged by translation
    assert!((cyl.radius() - r).abs() < CONFUSION);
    // Point on surface shifted
    let p = cyl.value(0.0, 0.0);
    assert!(pnt_eq(p, Pnt::new(10.0 + r, 20.0, 30.0), CONFUSION));
}

// ── test 10: Transform — uniform scale ───────────────────────────────────────
#[test]
fn test_transform_scale() {
    let r = 2.0;
    let scale = 3.0;
    let cyl = default_cylinder(r);
    let mut t = Trsf::identity();
    t.set_scale(Pnt::origin(), scale);
    let cyl2 = cyl.transformed(&t);

    // Radius should scale
    assert!((cyl2.radius() - r * scale).abs() < CONFUSION);
    // Location stays at origin (scaled from origin)
    assert!(pnt_eq(cyl2.location(), Pnt::origin(), CONFUSION));
    // Point check: value(U, V) uses the *parametric* V as-is against the axis
    // direction. After scaling the Ax3, the ZDir is still unit (normalized
    // inside Ax3::transformed), so value(0.0, 1.0) = (r*scale, 0, 1).
    let p = cyl2.value(0.0, 1.0);
    assert!(pnt_eq(p, Pnt::new(r * scale, 0.0, 1.0), CONFUSION));
    // Check that two surface points are correctly scaled in XY
    let p_at_v0 = cyl2.value(0.0, 0.0);
    assert!(pnt_eq(p_at_v0, Pnt::new(r * scale, 0.0, 0.0), CONFUSION));
}

// ── test 11: Transform — rotation ────────────────────────────────────────────
#[test]
fn test_transform_rotation() {
    let r = 2.0;
    let cyl = default_cylinder(r);
    // Rotate 90° about Z: X→Y, Y→-X, Z→Z
    let t = Trsf::rotation(Ax1::oz(), PI / 2.0);
    let cyl2 = cyl.transformed(&t);

    // Radius unchanged by rotation
    assert!((cyl2.radius() - r).abs() < CONFUSION);
    // X direction should now point in Y
    let pos = cyl2.position();
    assert!(vec_eq(pos.x_dir, Pnt::new(0.0, 1.0, 0.0), CONFUSION));
    assert!(vec_eq(pos.y_dir, Pnt::new(-1.0, 0.0, 0.0), CONFUSION));
    // Z axis unchanged (rotation about Z)
    assert!(vec_eq(pos.z_dir, Pnt::new(0.0, 0.0, 1.0), CONFUSION));
}

// ── test 12: axis() returns the correct Ax1 ──────────────────────────────────
#[test]
fn test_axis() {
    let loc = Pnt::new(1.0, 2.0, 3.0);
    let frame = Ax3 {
        location: loc,
        x_dir: Pnt::new(1.0, 0.0, 0.0),
        y_dir: Pnt::new(0.0, 1.0, 0.0),
        z_dir: Pnt::new(0.0, 0.0, 1.0),
    };
    let cyl = GeomCylindricalSurface::new(frame, 4.0);
    let ax = cyl.axis();
    assert!(pnt_eq(ax.location, loc, CONFUSION));
    assert!(vec_eq(ax.direction, Pnt::new(0.0, 0.0, 1.0), CONFUSION));
}

// ── test 13: non-identity placement ──────────────────────────────────────────
#[test]
fn test_non_identity_placement() {
    // Cylinder with axis along Y, centered at (1, 0, 0)
    let frame = Ax3 {
        location: Pnt::new(1.0, 0.0, 0.0),
        x_dir: Pnt::new(0.0, 0.0, 1.0), // X along world Z
        y_dir: Pnt::new(1.0, 0.0, 0.0), // Y along world X
        z_dir: Pnt::new(0.0, 1.0, 0.0), // Z (cylinder axis) along world Y
    };
    let r = 2.0;
    let cyl = GeomCylindricalSurface::new(frame, r);

    // At U=0, V=0: O + R*XDir = (1,0,0) + 2*(0,0,1) = (1, 0, 2)
    let p = cyl.value(0.0, 0.0);
    assert!(pnt_eq(p, Pnt::new(1.0, 0.0, 2.0), CONFUSION));

    // At U=PI/2, V=0: O + R*YDir = (1,0,0) + 2*(1,0,0) = (3, 0, 0)
    let p2 = cyl.value(PI / 2.0, 0.0);
    assert!(pnt_eq(p2, Pnt::new(3.0, 0.0, 0.0), CONFUSION));

    // At V=5: advances along ZDir = (0,1,0)
    let p3 = cyl.value(0.0, 5.0);
    assert!(pnt_eq(p3, Pnt::new(1.0, 5.0, 2.0), CONFUSION));
}

// ── test 14: copy() is independent ───────────────────────────────────────────
#[test]
fn test_copy_independence() {
    let mut cyl = default_cylinder(3.0);
    let cyl_copy = cyl.copy();
    cyl.set_radius(10.0);
    // The copy should still have radius 3.0
    assert!((cyl_copy.radius() - 3.0).abs() < CONFUSION);
    assert!((cyl.radius() - 10.0).abs() < CONFUSION);
}
