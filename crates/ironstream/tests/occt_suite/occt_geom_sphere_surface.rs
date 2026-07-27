//! Tests for `Geom_SphericalSurface` — ported from OCCT test suite.
//!
//! Tolerance constants from `Precision`:
//!   CONFUSION = 1e-7  (linear coincidence)
//!   ANGULAR   = 1e-12 (angular)

use ironstream::geom_sphere_surface::GeomSphericalSurface;
use ironstream::gp::{Ax1, Ax3, Pnt, Trsf};
use ironstream::precision::{ANGULAR, CONFUSION};
use std::f64::consts::PI;

// ── helpers ──────────────────────────────────────────────────────────────────

fn default_sphere(radius: f64) -> GeomSphericalSurface {
    GeomSphericalSurface::new(Ax3::identity(), radius)
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
    let s = default_sphere(7.5);
    assert!((s.radius() - 7.5).abs() < CONFUSION);
    let pos = s.position();
    assert!(pnt_eq(pos.location, Pnt::origin(), CONFUSION));
    assert!(vec_eq(pos.x_dir, Pnt::new(1.0, 0.0, 0.0), CONFUSION));
    assert!(vec_eq(pos.y_dir, Pnt::new(0.0, 1.0, 0.0), CONFUSION));
    assert!(vec_eq(pos.z_dir, Pnt::new(0.0, 0.0, 1.0), CONFUSION));
}

// ── test 2: Value / D0 at key parameter values ───────────────────────────────
#[test]
fn test_value_d0() {
    let r = 5.0;
    let s = default_sphere(r);

    // (U=0, V=0) → on the equator along +X: (R, 0, 0)
    assert!(pnt_eq(s.value(0.0, 0.0), Pnt::new(r, 0.0, 0.0), CONFUSION));

    // (U=π/2, V=0) → (0, R, 0)
    assert!(pnt_eq(
        s.value(PI / 2.0, 0.0),
        Pnt::new(0.0, r, 0.0),
        CONFUSION
    ));

    // (U=π, V=0) → (-R, 0, 0)
    assert!(pnt_eq(
        s.value(PI, 0.0),
        Pnt::new(-r, 0.0, 0.0),
        CONFUSION
    ));

    // (U=0, V=π/2) → north pole: (0, 0, R)
    assert!(pnt_eq(
        s.value(0.0, PI / 2.0),
        Pnt::new(0.0, 0.0, r),
        CONFUSION
    ));

    // (U=0, V=-π/2) → south pole: (0, 0, -R)
    assert!(pnt_eq(
        s.value(0.0, -PI / 2.0),
        Pnt::new(0.0, 0.0, -r),
        CONFUSION
    ));

    // D0 matches Value
    let u = 1.1;
    let v = 0.4;
    assert!(pnt_eq(s.d0(u, v), s.value(u, v), CONFUSION));

    // All points are at distance R from center
    for &uu in &[0.0, PI / 4.0, PI / 2.0, PI, 3.0 * PI / 2.0] {
        for &vv in &[-PI / 2.0, -PI / 4.0, 0.0, PI / 4.0, PI / 2.0] {
            let p = s.value(uu, vv);
            assert!((p.distance(Pnt::origin()) - r).abs() < CONFUSION);
        }
    }
}

// ── test 3: D1 partial derivatives ──────────────────────────────────────────
#[test]
fn test_d1_derivatives() {
    let r = 3.0;
    let s = default_sphere(r);

    // At (U=0, V=0): DU = R*(0·X + 1·Y)*cos(0) = (0, R, 0), DV = (-R*sin(0)*cos(0), 0, R*cos(0)) = (0, 0, R)
    let (p, du, dv) = s.d1(0.0, 0.0);
    assert!(pnt_eq(p, Pnt::new(r, 0.0, 0.0), CONFUSION));
    // DU = R·cos(V)·(-sin(U)·X + cos(U)·Y) = R·1·(0·X + 1·Y) = (0, R, 0)
    assert!(vec_eq(du, Pnt::new(0.0, r, 0.0), CONFUSION));
    // DV = R·(-sin(V)·cos(U)·X - sin(V)·sin(U)·Y + cos(V)·Z) = R·(0 + 0 + Z) = (0,0,R)
    assert!(vec_eq(dv, Pnt::new(0.0, 0.0, r), CONFUSION));

    // DU · P_from_center = 0 (tangent perpendicular to radius) at any point
    let u = 0.7;
    let v = 0.3;
    let (p2, du2, dv2) = s.d1(u, v);
    let radial = p2 - Pnt::origin();
    // DU is tangent: DU · radial = 0
    assert!(du2.dot(radial).abs() < CONFUSION * r);
    // DV is tangent: DV · radial = 0
    assert!(dv2.dot(radial).abs() < CONFUSION * r);

    // |DU| = R·|cos(V)|
    let (_, du3, _) = s.d1(u, v);
    let expected_du_norm = r * v.cos();
    assert!((du3.norm() - expected_du_norm).abs() < CONFUSION);
}

// ── test 4: D2 second-order derivatives ─────────────────────────────────────
#[test]
fn test_d2_derivatives() {
    let r = 4.0;
    let s = default_sphere(r);

    // At U=0, V=0:
    // P = (R, 0, 0)
    // DU = (0, R, 0)
    // DV = (0, 0, R)
    // D2U = R·cos(V)·(-cos(U)·X - sin(U)·Y) = R·(-1·X - 0·Y) = (-R, 0, 0)
    // D2V = R·(-cos(V)·cos(U)·X - cos(V)·sin(U)·Y - sin(V)·Z) = R·(-X) = (-R, 0, 0)
    // D2UV = R·sin(V)·(sin(U)·X - cos(U)·Y) = R·0·(0·X - 1·Y) = 0
    let (p, _du, _dv, d2u, d2v, d2uv) = s.d2(0.0, 0.0);
    assert!(pnt_eq(p, Pnt::new(r, 0.0, 0.0), CONFUSION));
    assert!(vec_eq(d2u, Pnt::new(-r, 0.0, 0.0), CONFUSION));
    assert!(vec_eq(d2v, Pnt::new(-r, 0.0, 0.0), CONFUSION));
    assert!(vec_eq(d2uv, Pnt::origin(), CONFUSION));

    // At U=π/2, V=0:
    // P = (0, R, 0)
    // D2U = R·cos(0)·(-cos(π/2)·X - sin(π/2)·Y) = R·(0 - Y) = (0, -R, 0)
    let (_, _, _, d2u2, _, _) = s.d2(PI / 2.0, 0.0);
    assert!(vec_eq(d2u2, Pnt::new(0.0, -r, 0.0), CONFUSION));
}

// ── test 5: DN higher-order derivatives ─────────────────────────────────────
#[test]
fn test_dn() {
    let r = 3.0;
    let s = default_sphere(r);
    let u = 0.7;
    let v = 0.4;

    // DN(u, v, 1, 0) should match D1.du
    let (_, du_ref, _) = s.d1(u, v);
    let dn10 = s.dn(u, v, 1, 0);
    assert!(vec_eq(dn10, du_ref, CONFUSION));

    // DN(u, v, 0, 1) should match D1.dv
    let (_, _, dv_ref) = s.d1(u, v);
    let dn01 = s.dn(u, v, 0, 1);
    assert!(vec_eq(dn01, dv_ref, CONFUSION));

    // DN(u, v, 2, 0) should match D2.d2u
    let (_, _, _, d2u_ref, _, _) = s.d2(u, v);
    let dn20 = s.dn(u, v, 2, 0);
    assert!(vec_eq(dn20, d2u_ref, CONFUSION));

    // DN(u, v, 0, 2) should match D2.d2v
    let (_, _, _, _, d2v_ref, _) = s.d2(u, v);
    let dn02 = s.dn(u, v, 0, 2);
    assert!(vec_eq(dn02, d2v_ref, CONFUSION));

    // DN(u, v, 1, 1) should match D2.d2uv
    let (_, _, _, _, _, d2uv_ref) = s.d2(u, v);
    let dn11 = s.dn(u, v, 1, 1);
    assert!(vec_eq(dn11, d2uv_ref, CONFUSION));

    // DN(u, v, 4, 0): 4th U-derivative = cos(U + 4·π/2) = cos(U), so
    // = R·cos(V)·(cos(U)·X + sin(U)·Y) (same as original XY part of P)
    let dn40 = s.dn(u, v, 4, 0);
    let expected_d4u = Pnt::new(r * v.cos() * u.cos(), r * v.cos() * u.sin(), 0.0);
    assert!(vec_eq(dn40, expected_d4u, CONFUSION));

    // DN(u, v, 0, 4): 4th V-derivative:
    // XY: R·cos(V + 4·π/2)·(cos(U)·X + sin(U)·Y) = R·cos(V)·(cos(U)·X + sin(U)·Y)
    // Z:  R·sin(V + 4·π/2) = R·sin(V)
    // = P - O (vector from center to surface point)
    let dn04 = s.dn(u, v, 0, 4);
    let p_from_center = s.value(u, v) - Pnt::origin();
    assert!(vec_eq(dn04, p_from_center, CONFUSION));
}

// ── test 6: UIso and VIso ────────────────────────────────────────────────────
#[test]
fn test_u_iso_v_iso() {
    let r = 5.0;
    let s = default_sphere(r);

    // UIso at U=0: great circle (meridian) in the XZ plane.
    // At parameter v_param = 0, we should land on the equator at (R,0,0).
    let u_iso = s.u_iso(0.0);
    let p_eq = u_iso.d0(0.0);
    assert!(pnt_eq(p_eq, Pnt::new(r, 0.0, 0.0), CONFUSION));

    // At parameter v_param = π/2 on the UIso circle, we should reach the north pole.
    let p_np = u_iso.d0(PI / 2.0);
    assert!(pnt_eq(p_np, Pnt::new(0.0, 0.0, r), CONFUSION));

    // UIso circle is centered at the sphere center and has radius R.
    assert!(pnt_eq(u_iso.location(), Pnt::origin(), CONFUSION));
    assert!((u_iso.radius() - r).abs() < CONFUSION);

    // VIso at V=0: the equator circle of radius R.
    let v_iso_equator = s.v_iso(0.0);
    assert!((v_iso_equator.radius() - r).abs() < CONFUSION);
    assert!(pnt_eq(v_iso_equator.location(), Pnt::origin(), CONFUSION));
    // Point at U=0 on equator: (R, 0, 0)
    assert!(pnt_eq(v_iso_equator.d0(0.0), Pnt::new(r, 0.0, 0.0), CONFUSION));

    // VIso at V=π/6: circle at height R·sin(π/6)=R/2, radius R·cos(π/6)=R·√3/2
    let v6 = PI / 6.0;
    let v_iso_30 = s.v_iso(v6);
    let expected_r = r * v6.cos();
    let expected_z = r * v6.sin();
    assert!((v_iso_30.radius() - expected_r).abs() < CONFUSION);
    assert!(pnt_eq(
        v_iso_30.location(),
        Pnt::new(0.0, 0.0, expected_z),
        CONFUSION
    ));

    // VIso at V=π/2 (north pole): circle of radius 0.
    let v_iso_np = s.v_iso(PI / 2.0);
    assert!(v_iso_np.radius() < CONFUSION);
}

// ── test 7: IsUClosed, IsVClosed, IsUPeriodic, IsVPeriodic, UPeriod ─────────
#[test]
fn test_topology_flags() {
    let s = default_sphere(1.0);

    assert!(s.is_u_closed(), "sphere must be closed in U");
    assert!(s.is_v_closed(), "sphere must be closed in V");
    assert!(s.is_u_periodic(), "sphere must be periodic in U");
    assert!(!s.is_v_periodic(), "sphere must NOT be periodic in V");
    assert!((s.u_period() - 2.0 * PI).abs() < ANGULAR);

    // Parameter bounds
    assert!((s.u_first_parameter() - 0.0).abs() < ANGULAR);
    assert!((s.u_last_parameter() - 2.0 * PI).abs() < ANGULAR);
    assert!((s.v_first_parameter() - (-PI / 2.0)).abs() < ANGULAR);
    assert!((s.v_last_parameter() - PI / 2.0).abs() < ANGULAR);
}

// ── test 8: Transform — translation ─────────────────────────────────────────
#[test]
fn test_transform_translation() {
    let r = 3.0;
    let mut s = default_sphere(r);
    let t = Trsf::translation(Pnt::new(10.0, 20.0, 30.0));
    s.transform(&t);

    // Center should shift by (10, 20, 30)
    assert!(pnt_eq(s.location(), Pnt::new(10.0, 20.0, 30.0), CONFUSION));
    // Radius unchanged by translation
    assert!((s.radius() - r).abs() < CONFUSION);
    // Point on sphere at (U=0, V=0) should shift accordingly
    let p = s.value(0.0, 0.0);
    assert!(pnt_eq(p, Pnt::new(10.0 + r, 20.0, 30.0), CONFUSION));
    // Surface point still at distance R from new center
    assert!((p.distance(s.location()) - r).abs() < CONFUSION);
}

// ── test 9: Transform — uniform scale ───────────────────────────────────────
#[test]
fn test_transform_scale() {
    let r = 2.0;
    let scale = 3.0;
    let s = default_sphere(r);
    let mut t = Trsf::identity();
    t.set_scale(Pnt::origin(), scale);
    let s2 = s.transformed(&t);

    // Radius should scale
    assert!((s2.radius() - r * scale).abs() < CONFUSION);
    // Center stays at origin
    assert!(pnt_eq(s2.location(), Pnt::origin(), CONFUSION));
    // Area should scale by square of scale factor
    let expected_area = 4.0 * PI * (r * scale) * (r * scale);
    assert!((s2.area() - expected_area).abs() < CONFUSION);
}

// ── test 10: Transform — rotation ────────────────────────────────────────────
#[test]
fn test_transform_rotation() {
    let r = 2.0;
    let s = default_sphere(r);
    // Rotate 90° about Z: X→Y, Y→-X, Z→Z
    let t = Trsf::rotation(Ax1::oz(), PI / 2.0);
    let s2 = s.transformed(&t);

    // Radius unchanged by rotation
    assert!((s2.radius() - r).abs() < CONFUSION);
    let pos = s2.position();
    // X direction should now point in Y
    assert!(vec_eq(pos.x_dir, Pnt::new(0.0, 1.0, 0.0), CONFUSION));
    // Y direction should now point in -X
    assert!(vec_eq(pos.y_dir, Pnt::new(-1.0, 0.0, 0.0), CONFUSION));
    // Z direction unchanged
    assert!(vec_eq(pos.z_dir, Pnt::new(0.0, 0.0, 1.0), CONFUSION));
}

// ── test 11: area() ──────────────────────────────────────────────────────────
#[test]
fn test_area() {
    // area = 4·π·R²
    for &r in &[1.0_f64, 2.0, 5.0, 10.0, 0.5] {
        let s = default_sphere(r);
        let expected = 4.0 * PI * r * r;
        assert!((s.area() - expected).abs() < CONFUSION * r * r);
    }
}

// ── test 12: UReversedParameter, VReversedParameter ─────────────────────────
#[test]
fn test_reversed_parameters() {
    let s = default_sphere(2.0);

    // UReversedParameter: 2·π - U
    assert!((s.u_reversed_parameter(0.0) - 2.0 * PI).abs() < CONFUSION);
    assert!((s.u_reversed_parameter(PI) - PI).abs() < CONFUSION);
    assert!((s.u_reversed_parameter(PI / 2.0) - 3.0 * PI / 2.0).abs() < CONFUSION);

    // VReversedParameter: -V
    assert!((s.v_reversed_parameter(PI / 4.0) - (-PI / 4.0)).abs() < CONFUSION);
    assert!((s.v_reversed_parameter(-PI / 3.0) - PI / 3.0).abs() < CONFUSION);
    assert!(s.v_reversed_parameter(0.0).abs() < CONFUSION);
}

// ── test 13: axis() returns correct Ax1 ──────────────────────────────────────
#[test]
fn test_axis() {
    let loc = Pnt::new(1.0, 2.0, 3.0);
    let frame = Ax3 {
        location: loc,
        x_dir: Pnt::new(1.0, 0.0, 0.0),
        y_dir: Pnt::new(0.0, 1.0, 0.0),
        z_dir: Pnt::new(0.0, 0.0, 1.0),
    };
    let s = GeomSphericalSurface::new(frame, 4.0);
    let ax = s.axis();
    assert!(pnt_eq(ax.location, loc, CONFUSION));
    assert!(vec_eq(ax.direction, Pnt::new(0.0, 0.0, 1.0), CONFUSION));
}

// ── test 14: copy() is independent ───────────────────────────────────────────
#[test]
fn test_copy_independence() {
    let mut s = default_sphere(3.0);
    let s_copy = s.copy();
    s.set_radius(10.0);
    assert!((s_copy.radius() - 3.0).abs() < CONFUSION);
    assert!((s.radius() - 10.0).abs() < CONFUSION);
}

// ── test 15: normal() is the outward unit radial vector ──────────────────────
#[test]
fn test_normal() {
    let r = 6.0;
    let s = default_sphere(r);
    for &u in &[0.0, PI / 4.0, PI / 2.0, PI, 5.0 * PI / 4.0] {
        for &v in &[-PI / 2.0, -PI / 6.0, 0.0, PI / 6.0, PI / 2.0] {
            let (p, n) = s.normal(u, v);
            // Normal is unit
            assert!((n.norm() - 1.0).abs() < CONFUSION);
            // Normal points away from center: p - O = R·n
            let radial = p - Pnt::origin();
            let expected_n = radial * (1.0 / r);
            assert!(vec_eq(n, expected_n, CONFUSION));
        }
    }
}

// ── test 16: non-identity placement ──────────────────────────────────────────
#[test]
fn test_non_identity_placement() {
    // Sphere centered at (5, 0, 0), Z axis along world X.
    let frame = Ax3 {
        location: Pnt::new(5.0, 0.0, 0.0),
        x_dir: Pnt::new(0.0, 1.0, 0.0), // local X along world Y
        y_dir: Pnt::new(0.0, 0.0, 1.0), // local Y along world Z
        z_dir: Pnt::new(1.0, 0.0, 0.0), // local Z (pole axis) along world X
    };
    let r = 3.0;
    let s = GeomSphericalSurface::new(frame, r);

    // (U=0, V=0): O + R·XDir = (5,0,0) + 3·(0,1,0) = (5, 3, 0)
    assert!(pnt_eq(s.value(0.0, 0.0), Pnt::new(5.0, 3.0, 0.0), CONFUSION));

    // (U=0, V=π/2): north pole = O + R·ZDir = (5,0,0) + 3·(1,0,0) = (8, 0, 0)
    assert!(pnt_eq(
        s.value(0.0, PI / 2.0),
        Pnt::new(8.0, 0.0, 0.0),
        CONFUSION
    ));

    // All points are at distance R from center
    for &u in &[0.0, PI / 3.0, PI, 5.0 * PI / 3.0] {
        for &v in &[-PI / 2.0, 0.0, PI / 4.0, PI / 2.0] {
            let p = s.value(u, v);
            let dist = p.distance(Pnt::new(5.0, 0.0, 0.0));
            assert!((dist - r).abs() < CONFUSION, "dist={dist}, r={r}, u={u}, v={v}");
        }
    }
}
