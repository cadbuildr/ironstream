//! Ported OpenCascade unit tests -- `Geom_ConicalSurface`.
//!
//! Faithful Rust port of OCCT's `Geom_ConicalSurface` tests. Same numeric
//! inputs, same expected values, and the same tolerances as OCCT:
//! - `Precision::Confusion()` → [`ironstream::precision::CONFUSION`] = 1e-7
//! - `Precision::Angular()`  → [`ironstream::precision::ANGULAR`]    = 1e-12
//!
//! OCCT helper-type mapping used in these tests:
//! - `gp_Pnt`               → [`ironstream::gp::Pnt`]
//! - `gp_Dir`               → [`ironstream::gp::Pnt`] (unit, via `Pnt::dir`)
//! - `gp_Vec`               → [`ironstream::gp::Vec3`]
//! - `gp_Ax2` / `gp_Ax3`   → [`ironstream::gp::Ax3`]
//! - `gp_Ax1`               → [`ironstream::gp::Ax1`]
//! - `gp_Trsf`              → [`ironstream::gp::Trsf`]
//! - `Geom_ConicalSurface`  → value [`ironstream::geom_cone_surface::GeomConicalSurface`]

use ironstream::geom_cone_surface::GeomConicalSurface;
use ironstream::gp::{Ax1, Ax3, Pnt, Trsf};
use ironstream::precision::{ANGULAR, CONFUSION};
use std::f64::consts::PI;

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

fn vec_near(v: Pnt, w: Pnt, tol: f64) {
    assert!(
        (v - w).norm() <= tol,
        "expected vec {v:?} ≈ {w:?} (tol {tol}), diff = {}",
        (v - w).norm()
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

// ========================== Tests ==========================================

/// TEST: Constructor and basic property getters (semi_angle, ref_radius).
#[test]
fn constructor_and_getters() {
    let a3 = ax2(Pnt::new(0.0, 0.0, 0.0), Pnt::dir(0.0, 0.0, 1.0));
    let semi_angle = PI / 6.0; // 30 degrees
    let radius = 2.0;
    let cone = GeomConicalSurface::new(a3, semi_angle, radius);

    near(cone.semi_angle(), semi_angle, ANGULAR);
    near(cone.ref_radius(), radius, CONFUSION);
    pnt_near(cone.location(), Pnt::new(0.0, 0.0, 0.0), CONFUSION);
}

/// TEST: Value (D0) at specific parameter values.
///
/// For U=0, V=0: P = O + R*(cos(0)*X + sin(0)*Y) + 0*Z = O + R*X.
/// For origin along Z, a3 is the identity frame, so P(0,0) = (R, 0, 0).
#[test]
fn value_at_u0_v0() {
    let a3 = ax2(Pnt::new(0.0, 0.0, 0.0), Pnt::dir(0.0, 0.0, 1.0));
    let radius = 3.0;
    let semi_angle = PI / 4.0; // 45 degrees, tan = 1
    let cone = GeomConicalSurface::new(a3, semi_angle, radius);

    // P(0, 0) = (3, 0, 0) in world coords
    let p = cone.value(0.0, 0.0);
    pnt_near(p, Pnt::new(3.0, 0.0, 0.0), CONFUSION);

    // P(π/2, 0) = (0, 3, 0)
    let p2 = cone.value(PI / 2.0, 0.0);
    pnt_near(p2, Pnt::new(0.0, 3.0, 0.0), CONFUSION);
}

/// TEST: Value at V ≠ 0: radius grows as R + V*tan(α).
///
/// With α = π/4 (tan=1), R=1, V=2: r(2) = 1 + 2 = 3.
/// P(0, 2) = (3, 0, 2).
#[test]
fn value_radius_grows_with_v() {
    let a3 = ax2(Pnt::new(0.0, 0.0, 0.0), Pnt::dir(0.0, 0.0, 1.0));
    let semi_angle = PI / 4.0; // tan(α) = 1
    let radius = 1.0;
    let cone = GeomConicalSurface::new(a3, semi_angle, radius);

    let p = cone.value(0.0, 2.0);
    pnt_near(p, Pnt::new(3.0, 0.0, 2.0), CONFUSION);

    // At angle π: P(π, 2) = (-3, 0, 2)
    let p2 = cone.value(PI, 2.0);
    pnt_near(p2, Pnt::new(-3.0, 0.0, 2.0), CONFUSION);
}

/// TEST: Apex location.
///
/// With R=3, α=π/4 (tan=1): V_apex = -R/tan(α) = -3.
/// So apex = O + (-3)*ZDir = (0,0,-3) for the identity frame.
#[test]
fn apex_location() {
    let a3 = ax2(Pnt::new(0.0, 0.0, 0.0), Pnt::dir(0.0, 0.0, 1.0));
    let semi_angle = PI / 4.0;
    let radius = 3.0;
    let cone = GeomConicalSurface::new(a3, semi_angle, radius);

    let apex = cone.apex();
    // V_apex = -3/1 = -3, so apex = (0,0,-3)
    pnt_near(apex, Pnt::new(0.0, 0.0, -3.0), CONFUSION);
}

/// TEST: Axis direction is ZDir of the local coordinate system.
#[test]
fn axis_direction() {
    let loc = Pnt::new(1.0, 2.0, 3.0);
    let a3 = ax2(loc, Pnt::dir(0.0, 0.0, 1.0));
    let cone = GeomConicalSurface::new(a3, PI / 6.0, 1.0);

    let ax = cone.axis();
    pnt_near(ax.location, loc, CONFUSION);
    // Direction should be (0, 0, 1) (unit Z).
    near(ax.direction.x, 0.0, CONFUSION);
    near(ax.direction.y, 0.0, CONFUSION);
    near(ax.direction.z, 1.0, CONFUSION);
}

/// TEST: D1 partial derivatives.
///
/// At (U=0, V=0) with α=π/6 (tan(α)=1/√3), R=2, identity frame:
/// - P(0,0)  = (2, 0, 0)
/// - DU(0,0) = (R+0*tan)*(-sin(0)*X + cos(0)*Y) = 2*(0, 1, 0) = (0, 2, 0)
/// - DV(0,0) = tan(α)*(cos(0)*X + sin(0)*Y) + Z
///           = (1/√3, 0, 0) + (0, 0, 1) = (1/√3, 0, 1)
#[test]
fn d1_derivatives() {
    let a3 = ax2(Pnt::new(0.0, 0.0, 0.0), Pnt::dir(0.0, 0.0, 1.0));
    let alpha = PI / 6.0; // 30°, tan = 1/sqrt(3)
    let radius = 2.0;
    let cone = GeomConicalSurface::new(a3, alpha, radius);

    let (p, du, dv) = cone.d1(0.0, 0.0);

    pnt_near(p, Pnt::new(2.0, 0.0, 0.0), CONFUSION);
    // DU = R * (-sin(0)*X + cos(0)*Y) = (0, 2, 0)
    vec_near(du, Pnt::new(0.0, 2.0, 0.0), CONFUSION);
    // DV = tan(π/6)*(X) + Z = (1/√3, 0, 1)
    let tan_a = alpha.tan();
    vec_near(dv, Pnt::new(tan_a, 0.0, 1.0), CONFUSION);
}

/// TEST: DN for various orders.
///
/// The 2nd angular derivative at (0, 0): r*(cos(π)*X + sin(π)*Y) = -r*X.
#[test]
fn dn_second_angular_derivative() {
    let a3 = ax2(Pnt::new(0.0, 0.0, 0.0), Pnt::dir(0.0, 0.0, 1.0));
    let alpha = PI / 4.0;
    let radius = 5.0;
    let cone = GeomConicalSurface::new(a3, alpha, radius);

    // DN(0, 0; nu=2, nv=0) = r*(cos(π)*X + sin(π)*Y) = (-5, 0, 0)
    let d2u = cone.dn(0.0, 0.0, 2, 0);
    vec_near(d2u, Pnt::new(-5.0, 0.0, 0.0), CONFUSION);

    // DN(0, 0; nu=0, nv=1) = DV(0,0) = tan(α)*X + Z = (1, 0, 1)
    let d0v1 = cone.dn(0.0, 0.0, 0, 1);
    vec_near(d0v1, Pnt::new(1.0, 0.0, 1.0), CONFUSION);

    // DN(0, 0; nu=0, nv=2) = zero (cone is linear in V)
    let d0v2 = cone.dn(0.0, 0.0, 0, 2);
    vec_near(d0v2, Pnt::new(0.0, 0.0, 0.0), CONFUSION);
}

/// TEST: UIso at U=0 is a generatrix line.
///
/// For the identity frame with R=2, α=π/4 (tan=1):
/// Origin of UIso(0): (2, 0, 0)
/// Direction of UIso(0): tan(α)*(1,0,0) + (0,0,1) = (1, 0, 1) (then normalized).
/// D0 at parameter V=1 on UIso(0) should equal Value(0, 1) = (3, 0, 1).
#[test]
fn u_iso_line() {
    let a3 = ax2(Pnt::new(0.0, 0.0, 0.0), Pnt::dir(0.0, 0.0, 1.0));
    let alpha = PI / 4.0; // tan = 1
    let radius = 2.0;
    let cone = GeomConicalSurface::new(a3, alpha, radius);

    let line = cone.u_iso(0.0);
    // Origin of the line: (2, 0, 0)
    pnt_near(
        line.position().location,
        Pnt::new(2.0, 0.0, 0.0),
        CONFUSION,
    );

    // The direction should be normalized (1, 0, 1)/sqrt(2).
    let dir_len = line.position().direction.norm();
    near(dir_len, 1.0, CONFUSION);

    // Point at parameter t = sqrt(2) should reach (3, 0, 1):
    // The direction vector (normalized) of UIso(0) is (1/√2, 0, 1/√2).
    // So P(t) = (2, 0, 0) + t*(1/√2, 0, 1/√2).
    // For t = sqrt(2): P = (2+1, 0, 1) = (3, 0, 1) = Value(0, 1).
    let sqrt2 = 2.0_f64.sqrt();
    let pt = line.d0(sqrt2);
    pnt_near(pt, Pnt::new(3.0, 0.0, 1.0), CONFUSION);

    // Cross-check with Value(0, 1)
    pnt_near(cone.value(0.0, 1.0), Pnt::new(3.0, 0.0, 1.0), CONFUSION);
}

/// TEST: VIso at V=2 is a circle of radius R + 2*tan(α).
///
/// For R=1, α=π/4 (tan=1): r(2) = 3.
/// Center at (0, 0, 2) for identity frame.
#[test]
fn v_iso_circle() {
    let a3 = ax2(Pnt::new(0.0, 0.0, 0.0), Pnt::dir(0.0, 0.0, 1.0));
    let alpha = PI / 4.0; // tan = 1
    let radius = 1.0;
    let cone = GeomConicalSurface::new(a3, alpha, radius);

    let circle = cone.v_iso(2.0);
    near(circle.radius(), 3.0, CONFUSION);
    pnt_near(circle.location(), Pnt::new(0.0, 0.0, 2.0), CONFUSION);

    // A point on the circle at parameter U=0 should be (3, 0, 2).
    pnt_near(circle.d0(0.0), Pnt::new(3.0, 0.0, 2.0), CONFUSION);

    // Cross-check with Value(0, 2)
    pnt_near(cone.value(0.0, 2.0), Pnt::new(3.0, 0.0, 2.0), CONFUSION);
}

/// TEST: Normal vector at a point on the cone.
///
/// At (U=0, V=0), α=π/4, identity frame:
/// N = cos(α)*(cos(0)*X + sin(0)*Y) - sin(α)*Z
///   = (1/√2, 0, -1/√2)
#[test]
fn normal_at_origin_angle_45() {
    let a3 = ax2(Pnt::new(0.0, 0.0, 0.0), Pnt::dir(0.0, 0.0, 1.0));
    let alpha = PI / 4.0;
    let radius = 2.0;
    let cone = GeomConicalSurface::new(a3, alpha, radius);

    let (_p, n) = cone.normal(0.0, 0.0);
    let inv_sqrt2 = 1.0_f64 / 2.0_f64.sqrt();
    near(n.x, inv_sqrt2, CONFUSION);
    near(n.y, 0.0, CONFUSION);
    near(n.z, -inv_sqrt2, CONFUSION);

    // Also verify normal is unit length.
    near(n.norm(), 1.0, CONFUSION);
}

/// TEST: Transform — translation shifts the surface location.
#[test]
fn transform_translation() {
    let a3 = ax2(Pnt::new(0.0, 0.0, 0.0), Pnt::dir(0.0, 0.0, 1.0));
    let semi_angle = PI / 6.0;
    let radius = 2.0;
    let mut cone = GeomConicalSurface::new(a3, semi_angle, radius);

    let t = Trsf::translation(Pnt::new(5.0, 0.0, 0.0));
    cone.transform(&t);

    pnt_near(cone.location(), Pnt::new(5.0, 0.0, 0.0), CONFUSION);
    // Angle and radius are unchanged by pure translation.
    near(cone.semi_angle(), semi_angle, ANGULAR);
    near(cone.ref_radius(), radius, CONFUSION);

    // P(0, 0) should now be at (5 + 2, 0, 0) = (7, 0, 0).
    pnt_near(cone.value(0.0, 0.0), Pnt::new(7.0, 0.0, 0.0), CONFUSION);
}

/// TEST: Transform — uniform scaling scales the radius.
#[test]
fn transform_scale() {
    let a3 = ax2(Pnt::new(0.0, 0.0, 0.0), Pnt::dir(0.0, 0.0, 1.0));
    let semi_angle = PI / 6.0;
    let radius = 2.0;
    let mut cone = GeomConicalSurface::new(a3, semi_angle, radius);

    let mut t = Trsf::identity();
    t.set_scale(Pnt::new(0.0, 0.0, 0.0), 3.0);
    cone.transform(&t);

    // Radius is scaled by 3.
    near(cone.ref_radius(), 6.0, CONFUSION);
    // Semi-angle is preserved.
    near(cone.semi_angle(), semi_angle, ANGULAR);
}

/// TEST: is_u_periodic / is_u_closed / is_v_closed.
#[test]
fn topology_flags() {
    let a3 = ax2(Pnt::new(0.0, 0.0, 0.0), Pnt::dir(0.0, 0.0, 1.0));
    let cone = GeomConicalSurface::new(a3, PI / 4.0, 1.0);

    assert!(cone.is_u_closed(), "cone must be closed in U");
    assert!(cone.is_u_periodic(), "cone must be periodic in U");
    assert!(!cone.is_v_closed(), "cone must be open in V");
    assert!(!cone.is_v_periodic(), "cone must be non-periodic in V");
    near(cone.u_period(), 2.0 * PI, ANGULAR);
}

/// TEST: Offset origin — cone at non-zero location.
#[test]
fn offset_location() {
    let origin = Pnt::new(1.0, 2.0, 3.0);
    let a3 = ax2(origin, Pnt::dir(0.0, 0.0, 1.0));
    let alpha = PI / 4.0; // tan = 1
    let radius = 0.0; // apex is at origin
    let cone = GeomConicalSurface::new(a3, alpha, radius);

    // Apex should be at origin (R=0, tan(α)=1, so V_apex = 0).
    pnt_near(cone.apex(), origin, CONFUSION);

    // P(0, 1): r(1) = 0 + 1 = 1. P = origin + 1*X + 1*Z.
    // With Z = (0,0,1), X = (1,0,0): P = (1+1, 2+0, 3+1) = (2, 2, 4).
    let p = cone.value(0.0, 1.0);
    pnt_near(p, Pnt::new(2.0, 2.0, 4.0), CONFUSION);
}

/// TEST: Rotation transform — axis direction changes.
#[test]
fn transform_rotation() {
    let a3 = ax2(Pnt::new(0.0, 0.0, 0.0), Pnt::dir(0.0, 0.0, 1.0));
    let semi_angle = PI / 6.0;
    let radius = 1.0;
    let mut cone = GeomConicalSurface::new(a3, semi_angle, radius);

    // Rotate 90° about X axis: Z → Y, Y → -Z.
    let t = Trsf::rotation(
        Ax1::new(Pnt::new(0.0, 0.0, 0.0), Pnt::new(1.0, 0.0, 0.0)),
        PI / 2.0,
    );
    cone.transform(&t);

    // Axis direction should now be Y (approximately).
    let ax = cone.axis();
    near(ax.direction.x, 0.0, CONFUSION);
    near(ax.direction.y.abs(), 1.0, CONFUSION);
    near(ax.direction.z, 0.0, CONFUSION);

    // Radius and semi-angle unchanged.
    near(cone.ref_radius(), radius, CONFUSION);
    near(cone.semi_angle(), semi_angle, ANGULAR);
}

/// TEST: DN mixed derivative (nu=1, nv=1).
///
/// At (U=0, V=0), α=π/4 (tan=1), R=2, identity frame:
/// DN(0,0; 1,1) = tan(α)*(cos(U+π/2)*X + sin(U+π/2)*Y)
///              = 1*(cos(π/2)*X + sin(π/2)*Y)
///              = (0, 1, 0)
#[test]
fn dn_mixed_derivative() {
    let a3 = ax2(Pnt::new(0.0, 0.0, 0.0), Pnt::dir(0.0, 0.0, 1.0));
    let alpha = PI / 4.0; // tan = 1
    let radius = 2.0;
    let cone = GeomConicalSurface::new(a3, alpha, radius);

    let d11 = cone.dn(0.0, 0.0, 1, 1);
    // tan(α) * (cos(π/2)*X + sin(π/2)*Y) = 1 * (0*X + 1*Y) = (0, 1, 0)
    vec_near(d11, Pnt::new(0.0, 1.0, 0.0), CONFUSION);
}
