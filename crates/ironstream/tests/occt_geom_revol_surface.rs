//! Tests for `Geom_SurfaceOfRevolution` — ported from OCCT test suite.
//!
//! Tolerance constants from `Precision`:
//!   CONFUSION = 1e-7  (linear coincidence)
//!   ANGULAR   = 1e-12 (angular)
//!
//! All tests revolve a line parallel to the Z axis at radius R around the
//! Z axis, which produces a cylinder of that radius. This is the canonical
//! OCCT test setup.

use ironstream::geom_revol_surface::{BasisCurve, GeomSurfaceOfRevolution};
use ironstream::gp::{Ax1, Pnt, Trsf};
use ironstream::precision::{ANGULAR, CONFUSION};
use std::f64::consts::PI;

// ── helpers ───────────────────────────────────────────────────────────────────

/// Cylinder of radius `r` about the Z axis: line at x=r parallel to Z,
/// revolved around Z.
fn cylinder_surface(r: f64) -> GeomSurfaceOfRevolution {
    let line = BasisCurve::line(Pnt::new(r, 0.0, 0.0), Pnt::new(0.0, 0.0, 1.0));
    let axis = Ax1::new(Pnt::origin(), Pnt::new(0.0, 0.0, 1.0));
    GeomSurfaceOfRevolution::new(line, axis)
}

fn pnt_close(a: Pnt, b: Pnt, tol: f64) -> bool {
    a.distance(b) < tol
}

fn vec_close(a: Pnt, b: Pnt, tol: f64) -> bool {
    (a - b).norm() < tol
}

// ── test 1: constructor, axis and basis curve stored correctly ────────────────
#[test]
fn test_constructor_stores_axis_and_basis() {
    let r = 5.0;
    let surf = cylinder_surface(r);

    // Axis should be Z at origin.
    let axis = surf.axis();
    assert!(pnt_close(axis.location, Pnt::origin(), CONFUSION));
    assert!(vec_close(
        axis.direction,
        Pnt::new(0.0, 0.0, 1.0),
        CONFUSION
    ));

    // Location convenience getter.
    assert!(pnt_close(surf.location(), Pnt::origin(), CONFUSION));

    // Direction convenience getter.
    assert!(vec_close(surf.direction(), Pnt::new(0.0, 0.0, 1.0), CONFUSION));
}

// ── test 2: Value / D0 matches known cylinder points ─────────────────────────
#[test]
fn test_value_matches_cylinder() {
    let r = 3.0;
    let surf = cylinder_surface(r);

    // (u=0, v=0) → (R, 0, 0)
    let p = surf.value(0.0, 0.0);
    assert!(pnt_close(p, Pnt::new(r, 0.0, 0.0), CONFUSION), "{p:?}");

    // (u=π/2, v=0) → (0, R, 0)
    let p = surf.value(PI / 2.0, 0.0);
    assert!(pnt_close(p, Pnt::new(0.0, r, 0.0), CONFUSION), "{p:?}");

    // (u=π, v=0) → (-R, 0, 0)
    let p = surf.value(PI, 0.0);
    assert!(pnt_close(p, Pnt::new(-r, 0.0, 0.0), CONFUSION), "{p:?}");

    // (u=3π/2, v=0) → (0, -R, 0)
    let p = surf.value(3.0 * PI / 2.0, 0.0);
    assert!(pnt_close(p, Pnt::new(0.0, -r, 0.0), CONFUSION), "{p:?}");

    // (u=0, v=7) → (R, 0, 7) — V translates along Z for line basis.
    let p = surf.value(0.0, 7.0);
    assert!(pnt_close(p, Pnt::new(r, 0.0, 7.0), CONFUSION), "{p:?}");

    // D0 matches Value.
    let p2 = surf.d0(PI / 4.0, 2.0);
    let p3 = surf.value(PI / 4.0, 2.0);
    assert!(pnt_close(p2, p3, CONFUSION));
}

// ── test 3: radius in XY plane is constant at R for all (u, v) ───────────────
#[test]
fn test_xy_radius_is_constant() {
    let r = 4.0;
    let surf = cylinder_surface(r);

    for &u in &[0.0, 0.5, 1.0, PI / 3.0, PI, 5.0] {
        for &v in &[-5.0, 0.0, 3.0, 10.0] {
            let p = surf.value(u, v);
            let xy_r = (p.x * p.x + p.y * p.y).sqrt();
            assert!(
                (xy_r - r).abs() < CONFUSION,
                "u={u}, v={v}: xy_radius={xy_r}, expected {r}"
            );
            // Z coordinate equals V.
            assert!((p.z - v).abs() < CONFUSION, "u={u}, v={v}: z={}", p.z);
        }
    }
}

// ── test 4: D1 partial derivatives match cylinder formulae ───────────────────
#[test]
fn test_d1_derivatives() {
    let r = 4.0;
    let surf = cylinder_surface(r);

    // At u=0, v=0:
    // DU = axis × (P - O) = (0,0,1) × (R,0,0) = (0,R,0)
    // DV = Rot(0)·(0,0,1) = (0,0,1)
    let (p, du, dv) = surf.d1(0.0, 0.0);
    assert!(pnt_close(p, Pnt::new(r, 0.0, 0.0), CONFUSION));
    assert!(vec_close(du, Pnt::new(0.0, r, 0.0), CONFUSION), "du={du:?}");
    assert!(vec_close(dv, Pnt::new(0.0, 0.0, 1.0), CONFUSION), "dv={dv:?}");

    // At u=π/2, v=0:
    // P = (0, R, 0), rel = (0, R, 0)
    // DU = (0,0,1) × (0,R,0) = (-R, 0, 0)
    // DV = Rot(π/2)·(0,0,1) = (0,0,1) (Z is along axis, unchanged by rotation)
    let (_, du2, dv2) = surf.d1(PI / 2.0, 0.0);
    assert!(
        vec_close(du2, Pnt::new(-r, 0.0, 0.0), CONFUSION),
        "du2={du2:?}"
    );
    assert!(
        vec_close(dv2, Pnt::new(0.0, 0.0, 1.0), CONFUSION),
        "dv2={dv2:?}"
    );

    // |DU| = R (for cylinder, DU = axis × rel, |rel|_xy = R).
    let (_, du3, _) = surf.d1(1.23, 5.0);
    assert!((du3.norm() - r).abs() < CONFUSION, "|DU|={}", du3.norm());

    // DV is always (0, 0, 1) for a Z-axis line basis revolved around Z.
    let (_, _, dv3) = surf.d1(2.7, -3.0);
    assert!(
        vec_close(dv3, Pnt::new(0.0, 0.0, 1.0), CONFUSION),
        "dv3={dv3:?}"
    );
}

// ── test 5: UIso (constant V) is a circle of radius R ────────────────────────
#[test]
fn test_u_iso_is_circle() {
    let r = 3.0;
    let surf = cylinder_surface(r);

    // UIso(v=0): circle at height 0, radius R, center at origin.
    let circle = surf.u_iso(0.0);
    assert!((circle.radius() - r).abs() < CONFUSION, "r={}", circle.radius());
    let center = circle.location();
    assert!(pnt_close(center, Pnt::origin(), CONFUSION), "center={center:?}");

    // UIso(v=5): circle at height 5, radius R, center at (0,0,5).
    let circle5 = surf.u_iso(5.0);
    assert!((circle5.radius() - r).abs() < CONFUSION);
    let center5 = circle5.location();
    assert!(
        pnt_close(center5, Pnt::new(0.0, 0.0, 5.0), CONFUSION),
        "center5={center5:?}"
    );

    // Points on UIso(v=0) should lie on the surface.
    for &u in &[0.0, PI / 4.0, PI / 2.0, PI, 3.0 * PI / 2.0] {
        let p_circle = circle.d0(u);
        let p_surf = surf.value(u, 0.0);
        assert!(
            pnt_close(p_circle, p_surf, CONFUSION),
            "u={u}: circle={p_circle:?}, surf={p_surf:?}"
        );
    }
}

// ── test 6: VIso (constant U) is a line parallel to Z axis ───────────────────
#[test]
fn test_v_iso_is_line() {
    let r = 3.0;
    let surf = cylinder_surface(r);

    // VIso(u=0): line through (R, 0, 0) parallel to Z.
    let line0 = surf.v_iso(0.0);
    let origin0 = line0.position().location;
    let dir0 = line0.position().direction;
    assert!(
        pnt_close(origin0, Pnt::new(r, 0.0, 0.0), CONFUSION),
        "origin0={origin0:?}"
    );
    assert!(
        vec_close(dir0, Pnt::new(0.0, 0.0, 1.0), CONFUSION),
        "dir0={dir0:?}"
    );

    // VIso(u=π/2): line through (0, R, 0) parallel to Z.
    let line_pi2 = surf.v_iso(PI / 2.0);
    let origin_pi2 = line_pi2.position().location;
    let dir_pi2 = line_pi2.position().direction;
    assert!(
        pnt_close(origin_pi2, Pnt::new(0.0, r, 0.0), CONFUSION),
        "origin_pi2={origin_pi2:?}"
    );
    assert!(
        vec_close(dir_pi2, Pnt::new(0.0, 0.0, 1.0), CONFUSION),
        "dir_pi2={dir_pi2:?}"
    );

    // VIso(u=π): line through (-R, 0, 0) parallel to Z.
    let line_pi = surf.v_iso(PI);
    let origin_pi = line_pi.position().location;
    assert!(
        pnt_close(origin_pi, Pnt::new(-r, 0.0, 0.0), CONFUSION),
        "origin_pi={origin_pi:?}"
    );
}

// ── test 7: DN higher-order derivatives ──────────────────────────────────────
#[test]
fn test_dn_derivatives() {
    let r = 2.0;
    let surf = cylinder_surface(r);

    // At u=0, v=0:
    // P = (R, 0, 0), rel = (R, 0, 0)
    // DN(1,0) = DU = d × rel = (0,0,1)×(R,0,0) = (0,R,0)
    let d10 = surf.dn(0.0, 0.0, 1, 0);
    assert!(
        vec_close(d10, Pnt::new(0.0, r, 0.0), CONFUSION),
        "d10={d10:?}"
    );

    // DN(0,1) = DV = Rot(0)·(0,0,1) = (0,0,1)
    let d01 = surf.dn(0.0, 0.0, 0, 1);
    assert!(
        vec_close(d01, Pnt::new(0.0, 0.0, 1.0), CONFUSION),
        "d01={d01:?}"
    );

    // DN(2,0) = d × DU = (0,0,1)×(0,R,0) = (-R, 0, 0)
    // (second U-derivative: centripetal direction)
    let d20 = surf.dn(0.0, 0.0, 2, 0);
    assert!(
        vec_close(d20, Pnt::new(-r, 0.0, 0.0), CONFUSION),
        "d20={d20:?}"
    );

    // DN(3,0) = d × D2U = (0,0,1)×(-R,0,0) = (0,-R,0)
    let d30 = surf.dn(0.0, 0.0, 3, 0);
    assert!(
        vec_close(d30, Pnt::new(0.0, -r, 0.0), CONFUSION),
        "d30={d30:?}"
    );

    // DN(4,0) = d × D3U = (0,0,1)×(0,-R,0) = (R, 0, 0)
    let d40 = surf.dn(0.0, 0.0, 4, 0);
    assert!(
        vec_close(d40, Pnt::new(r, 0.0, 0.0), CONFUSION),
        "d40={d40:?}"
    );

    // DN(0,2) = Rot(0)·C''(0) = 0 for line basis (C'' = 0).
    let d02 = surf.dn(0.0, 0.0, 0, 2);
    assert!(
        vec_close(d02, Pnt::origin(), CONFUSION),
        "d02={d02:?}"
    );

    // DN(1,1) = d × DV = (0,0,1) × (0,0,1) = (0,0,0)
    let d11 = surf.dn(0.0, 0.0, 1, 1);
    assert!(
        vec_close(d11, Pnt::origin(), CONFUSION),
        "d11={d11:?}"
    );
}

// ── test 8: Transform — scale and translate axis ──────────────────────────────
#[test]
fn test_transform_scale() {
    let r = 2.0;
    let mut surf = cylinder_surface(r);

    // Scale by factor 3.
    let mut t = Trsf::default();
    t.set_scale(Pnt::origin(), 3.0);
    surf.transform(&t);

    // After scaling, radius should be 3*r = 6.
    // Check at u=0, v=0: should be at (6, 0, 0).
    let p = surf.value(0.0, 0.0);
    assert!(
        pnt_close(p, Pnt::new(3.0 * r, 0.0, 0.0), CONFUSION),
        "scaled p={p:?}"
    );

    // At u=π/2: (0, 6, 0).
    let p2 = surf.value(PI / 2.0, 0.0);
    assert!(
        pnt_close(p2, Pnt::new(0.0, 3.0 * r, 0.0), CONFUSION),
        "scaled p2={p2:?}"
    );
}

// ── test 9: Transform — translation of axis ──────────────────────────────────
#[test]
fn test_transform_translation() {
    let r = 2.0;
    let mut surf = cylinder_surface(r);

    // Translate by (1, 2, 3).
    let t = Trsf::translation(Pnt::new(1.0, 2.0, 3.0));
    surf.transform(&t);

    // New axis location should be (1, 2, 3).
    let axis = surf.axis();
    assert!(
        pnt_close(axis.location, Pnt::new(1.0, 2.0, 3.0), CONFUSION),
        "axis loc={:?}",
        axis.location
    );

    // At u=0, v=0: line origin was (2,0,0), now (3,2,3)
    // Then rotated by 0 around new axis → same point (3, 2, 3).
    let p = surf.value(0.0, 0.0);
    assert!(
        pnt_close(p, Pnt::new(r + 1.0, 2.0, 3.0), CONFUSION),
        "translated p={p:?}"
    );
}

// ── test 10: Topology flags ────────────────────────────────────────────────────
#[test]
fn test_topology_flags() {
    let surf = cylinder_surface(1.0);

    assert!(surf.is_u_closed(), "should be closed in U");
    assert!(surf.is_u_periodic(), "should be periodic in U");
    assert!(!surf.is_v_closed(), "line basis: not closed in V");
    assert!(!surf.is_v_periodic(), "line basis: not periodic in V");
    assert!((surf.u_period() - 2.0 * PI).abs() < ANGULAR);
    assert!((surf.u_first_parameter() - 0.0).abs() < ANGULAR);
    assert!((surf.u_last_parameter() - 2.0 * PI).abs() < ANGULAR);
}

// ── test 11: UIso circle normal is the axis direction ──────────────────────────
#[test]
fn test_u_iso_normal_is_axis() {
    let surf = cylinder_surface(5.0);

    for &v in &[0.0, 1.0, -3.0, 7.5] {
        let circle = surf.u_iso(v);
        let normal = circle.axis().direction;
        assert!(
            vec_close(normal, Pnt::new(0.0, 0.0, 1.0), CONFUSION),
            "v={v}: circle normal={normal:?}"
        );
    }
}

// ── test 12: Copy produces identical surface ──────────────────────────────────
#[test]
fn test_copy() {
    let surf = cylinder_surface(4.0);
    let copy = surf.copy();

    for &u in &[0.0, PI / 3.0, PI] {
        for &v in &[0.0, 2.5, -1.0] {
            let p1 = surf.value(u, v);
            let p2 = copy.value(u, v);
            assert!(pnt_close(p1, p2, CONFUSION), "u={u}, v={v}: {p1:?} vs {p2:?}");
        }
    }
}

// ── test 13: U-reversed parameter ─────────────────────────────────────────────
#[test]
fn test_u_reversed_parameter() {
    let surf = cylinder_surface(1.0);
    let u = 1.2;
    let rev = surf.u_reversed_parameter(u);
    assert!(
        (rev - (2.0 * PI - u)).abs() < ANGULAR,
        "rev={rev}, expected {}",
        2.0 * PI - u
    );
}

// ── test 14: Normal at surface points ─────────────────────────────────────────
#[test]
fn test_normal_at_cylinder() {
    // For a cylinder radius R revolved around Z:
    // Normal at (u, v) should be (cos u, sin u, 0) — radial outward.
    let r = 3.0;
    let surf = cylinder_surface(r);

    for &u in &[0.0, PI / 4.0, PI / 2.0, PI, 3.0 * PI / 2.0] {
        let (p, n) = surf.normal(u, 0.0);
        let expected_n = Pnt::new(u.cos(), u.sin(), 0.0);
        assert!(
            vec_close(n, expected_n, CONFUSION),
            "u={u}: normal={n:?}, expected={expected_n:?}"
        );
        // Verify P is on the cylinder.
        let xy_r = (p.x * p.x + p.y * p.y).sqrt();
        assert!((xy_r - r).abs() < CONFUSION);
    }
}

// ── test 15: Non-zero axis location (off-center axis) ─────────────────────────
#[test]
fn test_off_center_axis() {
    // Axis through (1, 0, 0) pointing Z, line basis at x=4 (3 units from axis).
    let axis = Ax1::new(Pnt::new(1.0, 0.0, 0.0), Pnt::new(0.0, 0.0, 1.0));
    let line = BasisCurve::line(Pnt::new(4.0, 0.0, 0.0), Pnt::new(0.0, 0.0, 1.0));
    let surf = GeomSurfaceOfRevolution::new(line, axis);

    // At u=0, v=0: (4, 0, 0) — no rotation.
    let p = surf.value(0.0, 0.0);
    assert!(pnt_close(p, Pnt::new(4.0, 0.0, 0.0), CONFUSION), "{p:?}");

    // At u=π, v=0: should be at (1 + (1-4), 0, 0) = (-2, 0, 0).
    // The point (4,0,0) rotated 180° around x=1: 2*1 - 4 = -2.
    let p2 = surf.value(PI, 0.0);
    assert!(pnt_close(p2, Pnt::new(-2.0, 0.0, 0.0), CONFUSION), "{p2:?}");

    // Distance from axis (x=1 line) should always be 3.
    let p3 = surf.value(PI / 2.0, 0.0);
    let dx = p3.x - 1.0;
    let dy = p3.y;
    let dist = (dx * dx + dy * dy).sqrt();
    assert!((dist - 3.0).abs() < CONFUSION, "dist={dist}");
}
