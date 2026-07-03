//! Ported OpenCascade unit tests -- `Geom_Plane`.
//!
//! Faithful Rust port of OCCT's `Geom_Plane` tests
//! (Open-Cascade-SAS/OCCT). Same numeric inputs, same expected values, and the
//! same tolerances as OCCT:
//! - `Precision::Confusion()` → [`ironstream::precision::CONFUSION`] = 1e-7
//! - `Precision::Angular()`  → [`ironstream::precision::ANGULAR`]    = 1e-12
//!
//! OCCT helper-type mapping used in these tests:
//! - `gp_Pnt`                 → [`ironstream::gp::Pnt`]
//! - `gp_Dir`                 → [`ironstream::gp::Pnt`] (unit, via `Pnt::dir`)
//! - `gp_Vec`                 → [`ironstream::gp::Vec3`]
//! - `gp_Ax2` / `gp_Ax3`     → [`ironstream::gp::Ax3`]
//! - `gp_Trsf`                → [`ironstream::gp::Trsf`]
//! - `gp_Pln`                 → [`ironstream::gp_prim::Pln`]
//! - `Handle(Geom_Plane)`     → value [`ironstream::geom_plane::GeomPlane`]
//! - `Handle(Geom_Line)`      → value [`ironstream::geom_line::GeomLine`]

use ironstream::geom_plane::GeomPlane;
use ironstream::gp::{Ax1, Ax3, Pnt, Trsf};
use ironstream::gp_prim::Pln;
use ironstream::precision::{ANGULAR, CONFUSION};

// ─────────────────────────── helpers ────────────────────────────────────────

fn near(a: f64, b: f64, tol: f64) {
    assert!(
        (a - b).abs() <= tol,
        "expected {a} ≈ {b} (tol {tol}), diff = {}",
        (a - b).abs()
    );
}

fn pnt_near(p: Pnt, q: Pnt, tol: f64) {
    assert!(
        p.distance(q) <= tol,
        "expected point {p:?} ≈ {q:?} (tol {tol}), dist = {}",
        p.distance(q)
    );
}

/// Mirror of OCCT's `gp_Ax2(loc, dir)` constructor: picks an X direction
/// perpendicular to `dir` in the same canonical way OCCT does.
fn ax2(loc: Pnt, n: Pnt) -> Ax3 {
    let v = n.normalized();
    let (a, b, c) = (v.x, v.y, v.z);
    let (aa, ba, ca) = (a.abs(), b.abs(), c.abs());
    let xdir = if ba <= aa && ba <= ca {
        if aa > ca {
            Pnt::new(-c, 0.0, a)
        } else {
            Pnt::new(c, 0.0, -a)
        }
    } else if aa <= ba && aa <= ca {
        if ba > ca {
            Pnt::new(0.0, -c, b)
        } else {
            Pnt::new(0.0, c, -b)
        }
    } else if aa > ba {
        Pnt::new(-b, a, 0.0)
    } else {
        Pnt::new(b, -a, 0.0)
    };
    Ax3::from_origin_normal(loc, v, xdir)
}

// ====================== Tests ======================

// TEST: Constructor from gp_Pln
#[test]
fn constructor_from_pln() {
    let origin = Pnt::new(0.0, 0.0, 0.0);
    let normal = Pnt::dir(0.0, 0.0, 1.0);
    let pln = Pln::new_pd(origin, normal);
    let plane = GeomPlane::new(pln);

    // The location should match the origin.
    pnt_near(plane.location(), origin, CONFUSION);

    // The normal direction should match the Z-axis.
    let n = plane.normal_direction();
    near(n.x, 0.0, CONFUSION);
    near(n.y, 0.0, CONFUSION);
    near(n.z, 1.0, CONFUSION);
}

// TEST: Constructor from point and normal
#[test]
fn constructor_from_point_normal() {
    let origin = Pnt::new(1.0, 2.0, 3.0);
    let normal = Pnt::dir(0.0, 1.0, 0.0);
    let plane = GeomPlane::from_point_normal(origin, normal);

    pnt_near(plane.location(), origin, CONFUSION);

    let n = plane.normal_direction();
    near(n.x, 0.0, CONFUSION);
    near(n.y, 1.0, CONFUSION);
    near(n.z, 0.0, CONFUSION);
}

// TEST: Constructor from implicit coefficients (the XY plane: z = 0, i.e. 0x+0y+1z+0=0)
#[test]
fn constructor_from_coefficients_xy_plane() {
    // Plane equation: z = 0  →  0·x + 0·y + 1·z + 0 = 0
    let plane = GeomPlane::from_coefficients(0.0, 0.0, 1.0, 0.0);

    // Location should be on z=0 (specifically the foot of the normal from origin).
    let loc = plane.location();
    near(loc.z, 0.0, CONFUSION);

    // Normal should point in +Z.
    let n = plane.normal_direction();
    near(n.z.abs(), 1.0, CONFUSION);
}

// TEST: Plane at distance d from origin (z = d)
#[test]
fn constructor_from_coefficients_offset_plane() {
    // Plane equation: z - 5 = 0  →  0·x + 0·y + 1·z + (−5) = 0
    let plane = GeomPlane::from_coefficients(0.0, 0.0, 1.0, -5.0);
    let loc = plane.location();
    // The location should have z = 5.
    near(loc.z, 5.0, CONFUSION);
}

// TEST: Value/D0 evaluation
#[test]
fn value_evaluation() {
    let a3 = ax2(Pnt::new(0.0, 0.0, 0.0), Pnt::dir(0.0, 0.0, 1.0));
    let plane = GeomPlane::from_ax3(a3);

    // P(0, 0) should be the origin.
    pnt_near(plane.value(0.0, 0.0), Pnt::new(0.0, 0.0, 0.0), CONFUSION);

    // P(1, 0) should be along X.
    let p = plane.value(1.0, 0.0);
    near(p.x, 1.0, CONFUSION);
    near(p.y, 0.0, CONFUSION);
    near(p.z, 0.0, CONFUSION);

    // P(0, 1) should be along Y.
    let p = plane.value(0.0, 1.0);
    near(p.x, 0.0, CONFUSION);
    near(p.y, 1.0, CONFUSION);
    near(p.z, 0.0, CONFUSION);

    // P(3, -4) arbitrary check.
    let p = plane.value(3.0, -4.0);
    near(p.x, 3.0, CONFUSION);
    near(p.y, -4.0, CONFUSION);
    near(p.z, 0.0, CONFUSION);
}

// TEST: D1 partial derivatives
#[test]
fn d1_partial_derivatives() {
    let a3 = ax2(Pnt::new(0.0, 0.0, 0.0), Pnt::dir(0.0, 0.0, 1.0));
    let plane = GeomPlane::from_ax3(a3);

    let (p, du, dv) = plane.d1(2.0, 3.0);

    // Point should match value(2, 3).
    pnt_near(p, plane.value(2.0, 3.0), CONFUSION);

    // DU = XDir  = (1, 0, 0) for this plane.
    near(du.x, 1.0, CONFUSION);
    near(du.y, 0.0, CONFUSION);
    near(du.z, 0.0, CONFUSION);

    // DV = YDir = (0, 1, 0) for this plane.
    near(dv.x, 0.0, CONFUSION);
    near(dv.y, 1.0, CONFUSION);
    near(dv.z, 0.0, CONFUSION);
}

// TEST: D2 second-order derivatives are all zero
#[test]
fn d2_second_derivatives_zero() {
    let a3 = ax2(Pnt::new(1.0, 2.0, 3.0), Pnt::dir(0.0, 0.0, 1.0));
    let plane = GeomPlane::from_ax3(a3);

    let (_p, _du, _dv, d2u, d2v, d2uv) = plane.d2(0.5, 0.5);

    near(d2u.norm(), 0.0, CONFUSION);
    near(d2v.norm(), 0.0, CONFUSION);
    near(d2uv.norm(), 0.0, CONFUSION);
}

// TEST: DN — N-th order derivatives
#[test]
fn dn_derivatives() {
    let a3 = ax2(Pnt::new(0.0, 0.0, 0.0), Pnt::dir(0.0, 0.0, 1.0));
    let plane = GeomPlane::from_ax3(a3);

    // Order (1, 0) → XDir.
    let d = plane.dn(0.0, 0.0, 1, 0);
    near(d.x, 1.0, CONFUSION);
    near(d.y, 0.0, CONFUSION);
    near(d.z, 0.0, CONFUSION);

    // Order (0, 1) → YDir.
    let d = plane.dn(0.0, 0.0, 0, 1);
    near(d.x, 0.0, CONFUSION);
    near(d.y, 1.0, CONFUSION);
    near(d.z, 0.0, CONFUSION);

    // Order (2, 0) → zero.
    let d = plane.dn(0.0, 0.0, 2, 0);
    near(d.norm(), 0.0, CONFUSION);

    // Order (1, 1) → zero.
    let d = plane.dn(0.0, 0.0, 1, 1);
    near(d.norm(), 0.0, CONFUSION);
}

// TEST: Normal
#[test]
fn normal_evaluation() {
    let a3 = ax2(Pnt::new(0.0, 0.0, 0.0), Pnt::dir(0.0, 0.0, 1.0));
    let plane = GeomPlane::from_ax3(a3);

    // Normal is constant regardless of (u, v).
    let (p0, n0) = plane.normal(0.0, 0.0);
    let (p1, n1) = plane.normal(5.0, -3.0);

    pnt_near(p0, Pnt::new(0.0, 0.0, 0.0), CONFUSION);
    pnt_near(p1, plane.value(5.0, -3.0), CONFUSION);

    // Both normals should equal ZDir = (0, 0, 1).
    near(n0.x, 0.0, CONFUSION);
    near(n0.y, 0.0, CONFUSION);
    near(n0.z, 1.0, CONFUSION);

    // Normals must be equal for a plane.
    pnt_near(n0, n1, ANGULAR);

    // Normal must be a unit vector.
    near(n0.norm(), 1.0, CONFUSION);
}

// TEST: UIso and VIso lines
#[test]
fn iso_curves() {
    let a3 = ax2(Pnt::new(0.0, 0.0, 0.0), Pnt::dir(0.0, 0.0, 1.0));
    let plane = GeomPlane::from_ax3(a3);

    // UIso(2) should be a line through (2, 0, 0) along YDir = (0, 1, 0).
    let u_line = plane.u_iso(2.0);
    let origin_u = u_line.position().location;
    near(origin_u.x, 2.0, CONFUSION);
    near(origin_u.y, 0.0, CONFUSION);
    near(origin_u.z, 0.0, CONFUSION);

    let dir_u = u_line.position().direction;
    near(dir_u.x, 0.0, CONFUSION);
    near(dir_u.y, 1.0, CONFUSION);
    near(dir_u.z, 0.0, CONFUSION);

    // VIso(3) should be a line through (0, 3, 0) along XDir = (1, 0, 0).
    let v_line = plane.v_iso(3.0);
    let origin_v = v_line.position().location;
    near(origin_v.x, 0.0, CONFUSION);
    near(origin_v.y, 3.0, CONFUSION);
    near(origin_v.z, 0.0, CONFUSION);

    let dir_v = v_line.position().direction;
    near(dir_v.x, 1.0, CONFUSION);
    near(dir_v.y, 0.0, CONFUSION);
    near(dir_v.z, 0.0, CONFUSION);
}

// TEST: Points on UIso/VIso lie on the plane
#[test]
fn iso_points_lie_on_plane() {
    let origin = Pnt::new(1.0, 2.0, 0.0);
    let pln = Pln::new_pd(origin, Pnt::dir(0.0, 0.0, 1.0));
    let plane = GeomPlane::new(pln);

    // Evaluate UIso(u=5) at several V parameters and confirm all points are on
    // the plane (z = 0 for the XY plane).
    let u_line = plane.u_iso(5.0);
    for t in [-3.0, 0.0, 1.5, 10.0] {
        let pt = u_line.d0(t);
        near(pt.z, 0.0, CONFUSION);
        assert!(pln.contains(pt, CONFUSION), "UIso point not on plane: {pt:?}");
    }

    // Similarly for VIso(v=−2).
    let v_line = plane.v_iso(-2.0);
    for t in [-3.0, 0.0, 1.5, 10.0] {
        let pt = v_line.d0(t);
        near(pt.z, 0.0, CONFUSION);
        assert!(pln.contains(pt, CONFUSION), "VIso point not on plane: {pt:?}");
    }
}

// TEST: Transform — translation
#[test]
fn transform_translation() {
    let a3 = ax2(Pnt::new(0.0, 0.0, 0.0), Pnt::dir(0.0, 0.0, 1.0));
    let mut plane = GeomPlane::from_ax3(a3);

    let trsf = Trsf::translation(Pnt::new(3.0, 4.0, 5.0));
    plane.transform(&trsf);

    pnt_near(plane.location(), Pnt::new(3.0, 4.0, 5.0), CONFUSION);
    // Normal should be unchanged.
    let n = plane.normal_direction();
    near(n.z.abs(), 1.0, CONFUSION);
}

// TEST: Transform — rotation
#[test]
fn transform_rotation() {
    let a3 = ax2(Pnt::new(0.0, 0.0, 0.0), Pnt::dir(0.0, 0.0, 1.0));
    let mut plane = GeomPlane::from_ax3(a3);

    // Rotate 90° about X → normal should point in -Y (from +Z).
    let trsf = Trsf::rotation(
        Ax1::new(Pnt::origin(), Pnt::new(1.0, 0.0, 0.0)),
        std::f64::consts::FRAC_PI_2,
    );
    plane.transform(&trsf);

    let n = plane.normal_direction();
    // Rx(π/2) * (0,0,1) = (0, -1, 0): +Z becomes -Y under 90° rotation about X.
    near(n.x.abs(), 0.0, CONFUSION);
    near(n.y, -1.0, CONFUSION);
    near(n.z.abs(), 0.0, CONFUSION);
}

// TEST: Transformed (immutable) leaves original unchanged
#[test]
fn transformed_immutable() {
    let a3 = ax2(Pnt::new(1.0, 1.0, 0.0), Pnt::dir(0.0, 0.0, 1.0));
    let plane = GeomPlane::from_ax3(a3);

    let trsf = Trsf::translation(Pnt::new(10.0, 0.0, 0.0));
    let moved = plane.transformed(&trsf);

    // Original unchanged.
    pnt_near(plane.location(), Pnt::new(1.0, 1.0, 0.0), CONFUSION);
    // New plane moved.
    pnt_near(moved.location(), Pnt::new(11.0, 1.0, 0.0), CONFUSION);
}

// TEST: Copy
#[test]
fn copy_plane() {
    let a3 = ax2(Pnt::new(2.0, 3.0, 4.0), Pnt::dir(0.0, 0.0, 1.0));
    let plane = GeomPlane::from_ax3(a3);
    let c = plane.copy();

    pnt_near(c.location(), Pnt::new(2.0, 3.0, 4.0), CONFUSION);
    pnt_near(c.normal_direction(), Pnt::new(0.0, 0.0, 1.0), CONFUSION);
}

// TEST: Reversed parameters
#[test]
fn reversed_parameters() {
    let plane = GeomPlane::from_point_normal(Pnt::origin(), Pnt::dir(0.0, 0.0, 1.0));

    near(plane.u_reversed_parameter(3.5), -3.5, CONFUSION);
    near(plane.v_reversed_parameter(-7.2), 7.2, CONFUSION);
    near(plane.u_reversed_parameter(0.0), 0.0, CONFUSION);
}

// TEST: Closure / periodicity flags
#[test]
fn closure_periodicity_flags() {
    let plane = GeomPlane::from_point_normal(Pnt::origin(), Pnt::dir(1.0, 0.0, 0.0));

    assert!(!plane.is_u_closed());
    assert!(!plane.is_v_closed());
    assert!(!plane.is_u_periodic());
    assert!(!plane.is_v_periodic());
}

// TEST: D0 and Value give identical results
#[test]
fn d0_equals_value() {
    let a3 = ax2(Pnt::new(0.0, 0.0, 0.0), Pnt::dir(1.0, 0.0, 0.0));
    let plane = GeomPlane::from_ax3(a3);

    for (u, v) in [(0.0, 0.0), (1.5, -2.3), (-10.0, 7.0)] {
        pnt_near(plane.d0(u, v), plane.value(u, v), CONFUSION);
    }
}

// TEST: Normal is always perpendicular to DU and DV
#[test]
fn normal_perpendicular_to_tangents() {
    let a3 = ax2(Pnt::new(5.0, -3.0, 2.0), Pnt::dir(1.0, 1.0, 1.0));
    let plane = GeomPlane::from_ax3(a3);

    let (_p, du, dv) = plane.d1(0.0, 0.0);
    let (_q, n) = plane.normal(0.0, 0.0);

    // Normal must be unit length.
    near(n.norm(), 1.0, CONFUSION);
    // DU and DV must be unit length (plane frame is orthonormal).
    near(du.norm(), 1.0, CONFUSION);
    near(dv.norm(), 1.0, CONFUSION);

    // n · du = 0 and n · dv = 0.
    near(n.dot(du).abs(), 0.0, CONFUSION);
    near(n.dot(dv).abs(), 0.0, CONFUSION);

    // du · dv = 0 (orthonormal frame).
    near(du.dot(dv).abs(), 0.0, CONFUSION);
}

// TEST: SetPln replaces the underlying gp_Pln
#[test]
fn set_pln() {
    let a3 = ax2(Pnt::new(0.0, 0.0, 0.0), Pnt::dir(0.0, 0.0, 1.0));
    let mut plane = GeomPlane::from_ax3(a3);

    let new_pln = Pln::new_pd(Pnt::new(1.0, 2.0, 3.0), Pnt::dir(1.0, 0.0, 0.0));
    plane.set_pln(new_pln);

    pnt_near(plane.location(), Pnt::new(1.0, 2.0, 3.0), CONFUSION);
    let n = plane.normal_direction();
    near(n.x, 1.0, CONFUSION);
    near(n.y, 0.0, CONFUSION);
    near(n.z, 0.0, CONFUSION);
}
