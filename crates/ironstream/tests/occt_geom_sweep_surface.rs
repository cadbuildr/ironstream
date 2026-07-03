// FILE: tests/occt_geom_sweep_surface.rs
//! Integration tests for `geom_sweep_surface` — the `Geom_SweptSurface` module.
//!
//! Tests exercise the unified `GeomSweptSurface` enum, the two concrete swept
//! surface types, and the convenience constructors.
//!
//! Tolerance constants:
//!   CONFUSION = 1e-7 (linear coincidence)
//!   ANGULAR   = 1e-12 (angular coincidence)

extern crate ironstream;

use ironstream::geom_circle::GeomCircle;
use ironstream::geom_extrusion_surface::{GeomCurve, GeomSurfaceOfLinearExtrusion};
use ironstream::geom_revol_surface::{BasisCurve, GeomSurfaceOfRevolution};
use ironstream::geom_sweep_surface::{
    make_circle_extrusion, make_cylinder_revolution, make_torus_revolution, GeomSweptSurface,
    SweptSurfaceKind,
};
use ironstream::gp::{Ax1, Ax3, Pnt, Trsf};
use ironstream::precision::CONFUSION;
use std::f64::consts::PI;

// ── helpers ───────────────────────────────────────────────────────────────────

fn pnt_close(a: Pnt, b: Pnt, tol: f64) -> bool {
    a.distance(b) < tol
}

fn vec_close(a: Pnt, b: Pnt, tol: f64) -> bool {
    (a - b).norm() < tol
}

/// Circle extrusion surface: circle of radius `r` in XY plane, swept along +Z.
fn circle_ext(r: f64) -> GeomSweptSurface {
    let circle = GeomCircle::from_ax3(Ax3::identity(), r);
    GeomSweptSurface::new_extrusion(GeomCurve::Circle(circle), Pnt::new(0.0, 0.0, 1.0))
}

/// Cylinder revolution surface: line at x=r, parallel to Z, revolved around Z.
fn cyl_rev(r: f64) -> GeomSweptSurface {
    let basis = BasisCurve::line(Pnt::new(r, 0.0, 0.0), Pnt::new(0.0, 0.0, 1.0));
    let axis = Ax1::new(Pnt::origin(), Pnt::new(0.0, 0.0, 1.0));
    GeomSweptSurface::new_revolution(basis, axis)
}

// ── test 1: kind() tag and predicate methods ─────────────────────────────────
#[test]
fn test_kind_and_predicates() {
    let ext = circle_ext(1.0);
    assert_eq!(ext.kind(), SweptSurfaceKind::LinearExtrusion);
    assert!(ext.is_extrusion());
    assert!(!ext.is_revolution());

    let rev = cyl_rev(1.0);
    assert_eq!(rev.kind(), SweptSurfaceKind::Revolution);
    assert!(rev.is_revolution());
    assert!(!rev.is_extrusion());
}

// ── test 2: extrusion — value at canonical points ───────────────────────────
#[test]
fn test_extrusion_canonical_points() {
    let r = 4.0;
    let surf = circle_ext(r);

    // Circle at U=0: P = (R, 0, 0); no sweep offset.
    assert!(pnt_close(surf.value(0.0, 0.0), Pnt::new(r, 0.0, 0.0), CONFUSION));
    // Circle at U=π/2: P = (0, R, 0).
    assert!(pnt_close(surf.value(PI / 2.0, 0.0), Pnt::new(0.0, r, 0.0), CONFUSION));
    // Circle at U=π: P = (-R, 0, 0).
    assert!(pnt_close(surf.value(PI, 0.0), Pnt::new(-r, 0.0, 0.0), CONFUSION));
    // Sweep offset: V=7 translates along Z.
    assert!(pnt_close(surf.value(0.0, 7.0), Pnt::new(r, 0.0, 7.0), CONFUSION));
    // Negative V.
    assert!(pnt_close(surf.value(0.0, -3.0), Pnt::new(r, 0.0, -3.0), CONFUSION));
}

// ── test 3: revolution — cylinder radius is constant ────────────────────────
#[test]
fn test_revolution_cylinder_radius_constant() {
    let r = 6.0;
    let surf = cyl_rev(r);

    // At all (u, v): distance from Z axis = r.
    for &(u, v) in &[
        (0.0f64, 0.0f64),
        (PI / 3.0, 1.0),
        (PI, -2.0),
        (3.0 * PI / 2.0, 5.0),
        (2.0 * PI - 0.1, -0.5),
    ] {
        let p = surf.value(u, v);
        let xy_r = (p.x * p.x + p.y * p.y).sqrt();
        assert!(
            (xy_r - r).abs() < CONFUSION,
            "u={u:.3} v={v:.3}: xy_r={xy_r:.9}"
        );
        // V is the Z coordinate for a line-at-x=r basis.
        assert!((p.z - v).abs() < CONFUSION, "u={u:.3} v={v:.3}: z={}", p.z);
    }
}

// ── test 4: D0 equals value ──────────────────────────────────────────────────
#[test]
fn test_d0_equals_value() {
    let ext = circle_ext(3.0);
    let rev = cyl_rev(3.0);
    for &(u, v) in &[(0.0f64, 0.0f64), (1.0, 2.0), (PI, -1.5)] {
        assert!(pnt_close(ext.d0(u, v), ext.value(u, v), CONFUSION));
        assert!(pnt_close(rev.d0(u, v), rev.value(u, v), CONFUSION));
    }
}

// ── test 5: D1 — extrusion DV is always extrusion direction ─────────────────
#[test]
fn test_extrusion_d1_dv_constant() {
    let surf = circle_ext(5.0);
    let expected_dv = Pnt::new(0.0, 0.0, 1.0);

    for &(u, v) in &[(0.0f64, 0.0f64), (0.5, 1.0), (PI / 2.0, -2.0), (3.1, 7.7)] {
        let (_, _, dv) = surf.d1(u, v);
        assert!(vec_close(dv, expected_dv, CONFUSION), "u={u} v={v}: dv={dv:?}");
    }
}

// ── test 6: D1 — revolution at U=0 for cylinder ─────────────────────────────
#[test]
fn test_revolution_d1_at_u0() {
    let r = 4.0;
    let surf = cyl_rev(r);

    // At U=0, V=0: P=(r,0,0), DU = Z×(r,0,0) = (0,r,0), DV = (0,0,1)
    let (p, du, dv) = surf.d1(0.0, 0.0);
    assert!(pnt_close(p, Pnt::new(r, 0.0, 0.0), CONFUSION));
    assert!(vec_close(du, Pnt::new(0.0, r, 0.0), CONFUSION));
    assert!(vec_close(dv, Pnt::new(0.0, 0.0, 1.0), CONFUSION));
}

// ── test 7: surface normal is unit and orthogonal to partial derivatives ──────
#[test]
fn test_normal_is_unit_and_orthogonal() {
    let ext = circle_ext(3.0);
    let rev = cyl_rev(4.0);

    for &(u, v) in &[(0.3f64, 1.1f64), (PI / 4.0, 0.0), (PI, -2.0)] {
        // Extrusion normal
        let (_, n_ext) = ext.normal(u, v);
        let (_, du, dv) = ext.d1(u, v);
        assert!((n_ext.norm() - 1.0).abs() < 1e-9, "ext normal not unit at u={u} v={v}");
        assert!(n_ext.dot(du).abs() < 1e-9, "ext normal not perp to DU");
        assert!(n_ext.dot(dv).abs() < 1e-9, "ext normal not perp to DV");

        // Revolution normal
        let (_, n_rev) = rev.normal(u, v);
        let (_, dur, dvr) = rev.d1(u, v);
        assert!((n_rev.norm() - 1.0).abs() < 1e-9, "rev normal not unit at u={u} v={v}");
        assert!(n_rev.dot(dur).abs() < 1e-9, "rev normal not perp to DU");
        assert!(n_rev.dot(dvr).abs() < 1e-9, "rev normal not perp to DV");
    }
}

// ── test 8: topology flags — extrusion from circle ───────────────────────────
#[test]
fn test_topology_flags_extrusion() {
    let surf = circle_ext(2.0);

    assert!(surf.is_u_closed(), "circle extrusion: U should be closed");
    assert!(!surf.is_v_closed(), "extrusion: V should NOT be closed");
    assert!(surf.is_u_periodic(), "circle extrusion: U should be periodic");
    assert!(!surf.is_v_periodic(), "extrusion: V should NOT be periodic");

    // U range = [0, 2π]
    assert!((surf.u_first_parameter() - 0.0).abs() < CONFUSION);
    assert!((surf.u_last_parameter() - 2.0 * PI).abs() < CONFUSION);

    // V range is ±INFINITE
    assert!(surf.v_first_parameter() < -1e100);
    assert!(surf.v_last_parameter() > 1e100);
}

// ── test 9: topology flags — revolution ──────────────────────────────────────
#[test]
fn test_topology_flags_revolution() {
    let surf = cyl_rev(3.0);

    assert!(surf.is_u_closed(), "revolution: U should be closed");
    assert!(surf.is_u_periodic(), "revolution: U should be periodic");

    // U range = [0, 2π]
    assert!((surf.u_first_parameter() - 0.0).abs() < CONFUSION);
    assert!((surf.u_last_parameter() - 2.0 * PI).abs() < CONFUSION);
}

// ── test 10: transform — translation moves surface points ─────────────────────
#[test]
fn test_transform_translation() {
    let r = 2.0;
    let surf = circle_ext(r);
    let t = Trsf::translation(Pnt::new(10.0, 20.0, 30.0));
    let surf2 = surf.transformed(&t);

    // P(0, 0) was (r, 0, 0); after translation by (10,20,30) = (r+10, 20, 30)
    let p = surf2.value(0.0, 0.0);
    assert!(pnt_close(p, Pnt::new(r + 10.0, 20.0, 30.0), CONFUSION));

    // P(0, 5) was (r, 0, 5); after translation = (r+10, 20, 35)
    let p2 = surf2.value(0.0, 5.0);
    assert!(pnt_close(p2, Pnt::new(r + 10.0, 20.0, 35.0), CONFUSION));
}

// ── test 11: make_cylinder_revolution convenience function ────────────────────
#[test]
fn test_make_cylinder_revolution() {
    let r = 7.0;
    let surf = make_cylinder_revolution(r);

    // Radius in XY plane is constant r for all (u, v).
    for &(u, v) in &[(0.0f64, 0.0f64), (PI / 3.0, 4.0), (PI, -1.0)] {
        let p = surf.value(u, v);
        let xy_r = (p.x * p.x + p.y * p.y).sqrt();
        assert!((xy_r - r).abs() < CONFUSION, "xy_r={xy_r}");
    }

    // u_iso circle at v=0 has radius r.
    let c = surf.u_iso(0.0);
    let center = c.location();
    // Center of u-iso circle should be on the Z axis.
    assert!((center.x).abs() < CONFUSION);
    assert!((center.y).abs() < CONFUSION);
}

// ── test 12: make_circle_extrusion convenience function ───────────────────────
#[test]
fn test_make_circle_extrusion_convenience() {
    let r = 5.0;
    let surf = make_circle_extrusion(r, Pnt::new(0.0, 0.0, 1.0));

    // Verify radius: XY distance from origin is r for all u.
    let p0 = surf.value(0.0, 0.0);
    let p90 = surf.value(PI / 2.0, 0.0);
    assert!((p0.x * p0.x + p0.y * p0.y).sqrt() - r < CONFUSION);
    assert!((p90.x * p90.x + p90.y * p90.y).sqrt() - r < CONFUSION);

    // V offset works correctly.
    let p_v5 = surf.value(0.0, 5.0);
    assert!(pnt_close(p_v5, Pnt::new(r, 0.0, 5.0), CONFUSION));
}

// ── test 13: make_torus_revolution — outermost and innermost radii ────────────
#[test]
fn test_make_torus_revolution_radii() {
    let major_r = 5.0;
    let minor_r = 2.0;
    let surf = make_torus_revolution(major_r, minor_r);

    // At v=0, the circle point is (major_r + minor_r, 0, 0) (outermost).
    let p_outer = surf.value(0.0, 0.0);
    let outer_xy = (p_outer.x * p_outer.x + p_outer.y * p_outer.y).sqrt();
    assert!(
        (outer_xy - (major_r + minor_r)).abs() < CONFUSION,
        "outer_xy={outer_xy}"
    );

    // At v=π, the circle point is (major_r - minor_r, 0, 0) (innermost).
    let p_inner = surf.value(0.0, PI);
    let inner_xy = (p_inner.x * p_inner.x + p_inner.y * p_inner.y).sqrt();
    assert!(
        (inner_xy - (major_r - minor_r)).abs() < CONFUSION,
        "inner_xy={inner_xy}"
    );

    // Revolving at u=π/2 rotates into the Y direction.
    let p_rev = surf.value(PI / 2.0, 0.0);
    // x should be near 0, y should be near (major_r + minor_r).
    assert!(p_rev.x.abs() < CONFUSION, "x={}", p_rev.x);
    assert!(
        (p_rev.y - (major_r + minor_r)).abs() < CONFUSION,
        "y={}",
        p_rev.y
    );
}

// ── test 14: as_extrusion / as_revolution down-cast helpers ───────────────────
#[test]
fn test_downcast_helpers() {
    let ext = circle_ext(3.0);
    let inner: &GeomSurfaceOfLinearExtrusion = ext.as_extrusion().unwrap();
    // Verify we can call methods on the inner type.
    let d = inner.direction();
    assert!(vec_close(d, Pnt::new(0.0, 0.0, 1.0), CONFUSION));
    assert!(ext.as_revolution().is_none());

    let rev = cyl_rev(3.0);
    let inner_rev: &GeomSurfaceOfRevolution = rev.as_revolution().unwrap();
    // The revolution axis should be Z.
    let ax = inner_rev.axis();
    assert!(vec_close(ax.direction, Pnt::new(0.0, 0.0, 1.0), CONFUSION));
    assert!(rev.as_extrusion().is_none());
}

// ── test 15: u_iso_circle present for revolution, absent for extrusion ─────────
#[test]
fn test_u_iso_circle_dispatch() {
    let rev = cyl_rev(5.0);
    let circ = rev.u_iso_circle(0.0);
    assert!(circ.is_some(), "revolution should provide u_iso_circle");

    let ext = circle_ext(5.0);
    let none = ext.u_iso_circle(0.0);
    assert!(none.is_none(), "extrusion should not provide u_iso_circle");
}

// ── test 16: copy is independent of original ──────────────────────────────────
#[test]
fn test_copy_independence() {
    let surf = circle_ext(3.0);
    let surf_copy = surf.copy();

    // Both evaluate to the same value initially.
    let p1 = surf.value(0.0, 0.0);
    let p2 = surf_copy.value(0.0, 0.0);
    assert!(pnt_close(p1, p2, CONFUSION));

    // Transforming original must not affect copy.
    let t = Trsf::translation(Pnt::new(200.0, 0.0, 0.0));
    let surf_moved = surf.transformed(&t);
    let p_moved = surf_moved.value(0.0, 0.0);
    let p_copy_still = surf_copy.value(0.0, 0.0);
    assert!(
        !pnt_close(p_moved, p_copy_still, 1.0),
        "copy should be independent"
    );
}

// ── test 17: revolution u_period is 2π ───────────────────────────────────────
#[test]
fn test_revolution_u_period() {
    let surf = cyl_rev(3.0);
    // At (u, v) and (u + 2π, v): same point.
    let u = 1.23;
    let v = 4.5;
    let p1 = surf.value(u, v);
    let p2 = surf.value(u + 2.0 * PI, v);
    assert!(
        pnt_close(p1, p2, CONFUSION),
        "surface not periodic: p1={p1:?} p2={p2:?}"
    );
}
