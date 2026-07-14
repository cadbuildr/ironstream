//! Ported OpenCascade unit tests — `Geom_ToroidalSurface`.
//!
//! Faithful Rust port of OCCT's `Geom_ToroidalSurface` tests. Same numeric
//! inputs, same expected values, and the same tolerances as OCCT:
//! - `Precision::Confusion()` → [`ironstream::precision::CONFUSION`] = 1e-7
//! - `Precision::Angular()`  → [`ironstream::precision::ANGULAR`]    = 1e-12
//!
//! OCCT helper-type mapping used in these tests:
//! - `gp_Pnt`                  → [`ironstream::gp::Pnt`]
//! - `gp_Vec`                  → [`ironstream::gp::Vec3`]
//! - `gp_Ax2` / `gp_Ax3`      → [`ironstream::gp::Ax3`]
//! - `gp_Ax1`                  → [`ironstream::gp::Ax1`]
//! - `gp_Trsf`                 → [`ironstream::gp::Trsf`]
//! - `Geom_ToroidalSurface`    → [`ironstream::geom_torus_surface::GeomToroidalSurface`]

use ironstream::geom_torus_surface::GeomToroidalSurface;
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

fn default_torus(major: f64, minor: f64) -> GeomToroidalSurface {
    GeomToroidalSurface::new(Ax3::identity(), major, minor)
}

// ========================== Tests ==========================================

/// TEST 1: Constructor stores major/minor radii and position correctly.
#[test]
fn test_constructor_and_getters() {
    let major = 5.0;
    let minor = 2.0;
    let torus = default_torus(major, minor);

    near(torus.major_radius(), major, CONFUSION);
    near(torus.minor_radius(), minor, CONFUSION);

    let pos = torus.position();
    pnt_near(pos.location, Pnt::origin(), CONFUSION);
    vec_near(pos.x_dir, Pnt::new(1.0, 0.0, 0.0), CONFUSION);
    vec_near(pos.y_dir, Pnt::new(0.0, 1.0, 0.0), CONFUSION);
    vec_near(pos.z_dir, Pnt::new(0.0, 0.0, 1.0), CONFUSION);

    pnt_near(torus.location(), Pnt::origin(), CONFUSION);
}

/// TEST 2: Value (D0) at key parameter pairs.
///
/// With R=5, r=2, identity frame:
///  P(0, 0)  = O + (R+r)·X + 0·Z = (7, 0, 0)
///  P(π/2,0) = O + (R+r)·Y       = (0, 7, 0)
///  P(0, π)  = O + (R-r)·X       = (3, 0, 0)   [cos(π)=-1 → R-r]
///  P(0, π/2)= O + R·X + r·Z     = (5, 0, 2)
#[test]
fn test_value_d0_key_points() {
    let r_maj = 5.0;
    let r_min = 2.0;
    let torus = default_torus(r_maj, r_min);

    // U=0, V=0: outermost equatorial point
    pnt_near(torus.value(0.0, 0.0), Pnt::new(7.0, 0.0, 0.0), CONFUSION);

    // U=π/2, V=0
    pnt_near(torus.value(PI / 2.0, 0.0), Pnt::new(0.0, 7.0, 0.0), CONFUSION);

    // U=π, V=0
    pnt_near(torus.value(PI, 0.0), Pnt::new(-7.0, 0.0, 0.0), CONFUSION);

    // U=0, V=π: innermost equatorial point
    pnt_near(torus.value(0.0, PI), Pnt::new(3.0, 0.0, 0.0), CONFUSION);

    // U=0, V=π/2: top of tube
    pnt_near(torus.value(0.0, PI / 2.0), Pnt::new(5.0, 0.0, 2.0), CONFUSION);

    // D0 agrees with value
    let u = 1.1;
    let v = 0.7;
    pnt_near(torus.d0(u, v), torus.value(u, v), CONFUSION);
}

/// TEST 3: D1 partial derivatives at (0, 0).
///
/// R=5, r=2, identity frame, (U=0, V=0):
///   P    = (7, 0, 0)
///   DU   = ρ·(-sin(0)·X + cos(0)·Y) = 7·(0,1,0) = (0, 7, 0)
///   DV   = -r·sin(0)·(X) + r·cos(0)·Z = r·Z = (0, 0, 2)
#[test]
fn test_d1_at_origin_params() {
    let r_maj = 5.0;
    let r_min = 2.0;
    let torus = default_torus(r_maj, r_min);

    let (p, du, dv) = torus.d1(0.0, 0.0);

    pnt_near(p, Pnt::new(7.0, 0.0, 0.0), CONFUSION);
    // DU = ρ·(-sin(0)·X + cos(0)·Y) = 7·Y
    vec_near(du, Pnt::new(0.0, 7.0, 0.0), CONFUSION);
    // DV = r·cos(0)·Z = 2·Z
    vec_near(dv, Pnt::new(0.0, 0.0, 2.0), CONFUSION);
}

/// TEST 4: D1 partial derivatives at (π/2, π/4).
///
/// R=4, r=1, identity frame, (U=π/2, V=π/4):
///   ρ = R + r·cos(π/4) = 4 + 1/√2
///   cos(U)=0, sin(U)=1
///   DU = ρ·(-1·X + 0·Y) = -ρ·X
///   DV = -r·sin(π/4)·(0·X + 1·Y) + r·cos(π/4)·Z
///       = -1/√2·Y + 1/√2·Z
#[test]
fn test_d1_at_varied_params() {
    let r_maj = 4.0;
    let r_min = 1.0;
    let torus = default_torus(r_maj, r_min);

    let u = PI / 2.0;
    let v = PI / 4.0;
    let (_, du, dv) = torus.d1(u, v);

    let inv_sqrt2 = 1.0_f64 / 2.0_f64.sqrt();
    let rho = r_maj + r_min * v.cos(); // 4 + 1/√2

    // DU = ρ·(-sin(π/2)·X + cos(π/2)·Y) = ρ·(-1·X + 0·Y) = (-ρ, 0, 0)
    vec_near(du, Pnt::new(-rho, 0.0, 0.0), CONFUSION);

    // DV = -r·sin(π/4)·(cos(π/2)·X + sin(π/2)·Y) + r·cos(π/4)·Z
    //     = -1·(1/√2)·(0·X + 1·Y) + 1·(1/√2)·Z
    //     = (0, -1/√2, 1/√2)
    vec_near(
        dv,
        Pnt::new(0.0, -r_min * inv_sqrt2, r_min * inv_sqrt2),
        CONFUSION,
    );
}

/// TEST 5: DN — first-order pure U and V derivatives.
///
/// DN(u, v, 1, 0) must equal the DU from D1.
/// DN(u, v, 0, 1) must equal the DV from D1.
#[test]
fn test_dn_first_order() {
    let torus = default_torus(5.0, 2.0);
    let u = 0.8;
    let v = 1.2;

    let (_, du, dv) = torus.d1(u, v);

    let dn10 = torus.dn(u, v, 1, 0);
    let dn01 = torus.dn(u, v, 0, 1);

    vec_near(dn10, du, CONFUSION);
    vec_near(dn01, dv, CONFUSION);
}

/// TEST 6: DN — second-order pure U derivative.
///
/// At (U=0, V=0): d²P/dU² = -ρ·(cos(0)·X + sin(0)·Y) = -(R+r)·X
/// With R=5, r=2: = -7·X = (-7, 0, 0)
#[test]
fn test_dn_second_order_u() {
    let r_maj = 5.0;
    let r_min = 2.0;
    let torus = default_torus(r_maj, r_min);

    // DN(0, 0; nu=2, nv=0) = (R+r)*(cos(0+π)*X + sin(0+π)*Y) = -7*X
    let d2u = torus.dn(0.0, 0.0, 2, 0);
    vec_near(d2u, Pnt::new(-(r_maj + r_min), 0.0, 0.0), CONFUSION);

    // DN(0, 0; nu=4, nv=0) = (R+r)*(cos(0+2π)*X + sin(0+2π)*Y) = (R+r)*X
    let d4u = torus.dn(0.0, 0.0, 4, 0);
    vec_near(d4u, Pnt::new(r_maj + r_min, 0.0, 0.0), CONFUSION);
}

/// TEST 7: DN — second-order pure V derivative.
///
/// d²P/dV² = r·cos(V + π)·e_r + r·sin(V + π)·Z
///          = -r·cos(V)·e_r - r·sin(V)·Z
///
/// At (U=0, V=0): = -r·X = (-2, 0, 0)
#[test]
fn test_dn_second_order_v() {
    let r_maj = 5.0;
    let r_min = 2.0;
    let torus = default_torus(r_maj, r_min);

    // DN(0, 0; nu=0, nv=2) = r*cos(0+π)*e_r(0) + r*sin(0+π)*Z
    //                       = r*(-1)*(1,0,0) + r*(0)*(0,0,1)
    //                       = (-2, 0, 0)
    let d2v = torus.dn(0.0, 0.0, 0, 2);
    vec_near(d2v, Pnt::new(-r_min, 0.0, 0.0), CONFUSION);
}

/// TEST 8: DN — mixed partial derivative (nu=1, nv=1).
///
/// d²P / dU dV = r·cos(V + π/2)·(cos(U + π/2)·X + sin(U + π/2)·Y)
///
/// At (U=0, V=0): = r·cos(π/2)·(cos(π/2)·X + sin(π/2)·Y) = r·0·(0·X + 1·Y) = (0, 0, 0)
///
/// At (U=0, V=π/2): cos(V + π/2) = cos(π) = -1
///   = r·(-1)·(cos(π/2)·X + sin(π/2)·Y) = -r·(0, 1, 0) = (0, -r, 0)
#[test]
fn test_dn_mixed_derivative() {
    let r_maj = 5.0;
    let r_min = 2.0;
    let torus = default_torus(r_maj, r_min);

    // At (U=0, V=0): should be zero.
    let d11_at_00 = torus.dn(0.0, 0.0, 1, 1);
    vec_near(d11_at_00, Pnt::origin(), CONFUSION);

    // At (U=0, V=π/2):
    // r·cos(V+π/2)·(cos(U+π/2)·X + sin(U+π/2)·Y)
    // = 2·cos(π)·(cos(π/2)·X + sin(π/2)·Y)
    // = 2·(-1)·(0·X + 1·Y) = (0, -2, 0)
    let d11_at_0_pi2 = torus.dn(0.0, PI / 2.0, 1, 1);
    vec_near(d11_at_0_pi2, Pnt::new(0.0, -r_min, 0.0), CONFUSION);
}

/// TEST 9: UIso returns a circle at the correct center with the correct radius.
///
/// UIso(U=0) for R=5, r=2:
///   center = O + R·X = (5, 0, 0)
///   radius = r = 2
///   The circle lies in the (X, Z) plane; at V=0 should equal Value(0, 0)=(7,0,0).
///   At V=π/2 should equal Value(0, π/2)=(5, 0, 2).
#[test]
fn test_u_iso() {
    let r_maj = 5.0;
    let r_min = 2.0;
    let torus = default_torus(r_maj, r_min);

    let circ = torus.u_iso(0.0);
    near(circ.radius(), r_min, CONFUSION);
    pnt_near(circ.location(), Pnt::new(r_maj, 0.0, 0.0), CONFUSION);

    // Point at V=0 on UIso: center + r·x_dir = (5,0,0) + 2·(1,0,0) = (7,0,0)
    // which matches Value(0, 0).
    let pt0 = circ.d0(0.0);
    pnt_near(pt0, torus.value(0.0, 0.0), CONFUSION);

    // Point at V=π/2 on UIso: center + r·y_dir.
    // The UIso frame has y_dir = z_dir of torus = (0,0,1).
    // So circ.d0(π/2) = center + r·sin(π/2)·y_dir = (5,0,0) + 2·(0,0,1) = (5,0,2).
    let pt_pi2 = circ.d0(PI / 2.0);
    pnt_near(pt_pi2, torus.value(0.0, PI / 2.0), CONFUSION);
}

/// TEST 10: VIso returns a circle at the correct center with the correct radius.
///
/// VIso(V=0) for R=5, r=2:
///   radius = R + r·cos(0) = 7
///   center = O + r·sin(0)·Z = O = (0, 0, 0)
///
/// VIso(V=π/2):
///   radius = R + r·cos(π/2) = 5
///   center = O + r·sin(π/2)·Z = (0, 0, 2)
#[test]
fn test_v_iso() {
    let r_maj = 5.0;
    let r_min = 2.0;
    let torus = default_torus(r_maj, r_min);

    let circ0 = torus.v_iso(0.0);
    near(circ0.radius(), r_maj + r_min, CONFUSION);
    pnt_near(circ0.location(), Pnt::origin(), CONFUSION);

    // Point at U=0 on VIso(0): center + radius·X = (7, 0, 0) = Value(0, 0).
    pnt_near(circ0.d0(0.0), torus.value(0.0, 0.0), CONFUSION);

    let circ_pi2 = torus.v_iso(PI / 2.0);
    near(circ_pi2.radius(), r_maj, CONFUSION); // R + r*cos(π/2) = 5
    pnt_near(circ_pi2.location(), Pnt::new(0.0, 0.0, r_min), CONFUSION);

    // Point at U=0 on VIso(π/2): center + radius·X = (5, 0, 2) = Value(0, π/2).
    pnt_near(circ_pi2.d0(0.0), torus.value(0.0, PI / 2.0), CONFUSION);
}

/// TEST 11: Normal at key points is unit and points outward.
///
/// At (U=0, V=0): n = cos(0)·e_r(0) + sin(0)·Z = X = (1, 0, 0)
/// At (U=0, V=π/2): n = cos(π/2)·X + sin(π/2)·Z = Z = (0, 0, 1)
/// At (U=π/2, V=0): n = e_r(π/2) = Y = (0, 1, 0)
#[test]
fn test_normal_unit_and_direction() {
    let torus = default_torus(5.0, 2.0);

    let (_, n00) = torus.normal(0.0, 0.0);
    near(n00.norm(), 1.0, CONFUSION);
    vec_near(n00, Pnt::new(1.0, 0.0, 0.0), CONFUSION);

    let (_, n0_pi2) = torus.normal(0.0, PI / 2.0);
    near(n0_pi2.norm(), 1.0, CONFUSION);
    vec_near(n0_pi2, Pnt::new(0.0, 0.0, 1.0), CONFUSION);

    let (_, n_pi2_0) = torus.normal(PI / 2.0, 0.0);
    near(n_pi2_0.norm(), 1.0, CONFUSION);
    vec_near(n_pi2_0, Pnt::new(0.0, 1.0, 0.0), CONFUSION);

    // At (π, π): n = cos(π)·e_r(π) + sin(π)·Z = (-1)·(-1,0,0) = (1,0,0)
    let (_, n_pi_pi) = torus.normal(PI, PI);
    near(n_pi_pi.norm(), 1.0, CONFUSION);
    vec_near(n_pi_pi, Pnt::new(1.0, 0.0, 0.0), CONFUSION);
}

/// TEST 12: Area and Volume.
///
/// Area   = 4·π²·R·r
/// Volume = 2·π²·R·r²
#[test]
fn test_area_and_volume() {
    let r_maj = 5.0;
    let r_min = 2.0;
    let torus = default_torus(r_maj, r_min);

    let expected_area = 4.0 * PI * PI * r_maj * r_min;
    let expected_volume = 2.0 * PI * PI * r_maj * r_min * r_min;

    near(torus.area(), expected_area, expected_area * 1e-12);
    near(torus.volume(), expected_volume, expected_volume * 1e-12);
}

/// TEST 13: Transform — translation shifts location; radii unchanged.
#[test]
fn test_transform_translation() {
    let r_maj = 5.0;
    let r_min = 2.0;
    let mut torus = default_torus(r_maj, r_min);
    let shift = Pnt::new(10.0, 20.0, 30.0);
    let t = Trsf::translation(shift);
    torus.transform(&t);

    pnt_near(torus.location(), shift, CONFUSION);
    near(torus.major_radius(), r_maj, CONFUSION);
    near(torus.minor_radius(), r_min, CONFUSION);

    // Value(0, 0) should be at (10+7, 20, 30)
    pnt_near(torus.value(0.0, 0.0), Pnt::new(17.0, 20.0, 30.0), CONFUSION);
}

/// TEST 14: Transform — uniform scale scales both radii.
#[test]
fn test_transform_scale() {
    let r_maj = 4.0;
    let r_min = 1.0;
    let scale = 3.0;
    let mut torus = default_torus(r_maj, r_min);

    let mut t = Trsf::identity();
    t.set_scale(Pnt::origin(), scale);
    torus.transform(&t);

    near(torus.major_radius(), r_maj * scale, CONFUSION);
    near(torus.minor_radius(), r_min * scale, CONFUSION);
    pnt_near(torus.location(), Pnt::origin(), CONFUSION);

    // Area and volume should scale accordingly.
    let expected_area = 4.0 * PI * PI * (r_maj * scale) * (r_min * scale);
    near(torus.area(), expected_area, expected_area * 1e-12);
}

/// TEST 15: Transform — rotation changes X/Y/Z axes; radii unchanged.
#[test]
fn test_transform_rotation() {
    let r_maj = 5.0;
    let r_min = 2.0;
    let mut torus = default_torus(r_maj, r_min);

    // Rotate 90° about Z: X→Y, Y→-X, Z→Z.
    let t = Trsf::rotation(Ax1::oz(), PI / 2.0);
    torus.transform(&t);

    near(torus.major_radius(), r_maj, CONFUSION);
    near(torus.minor_radius(), r_min, CONFUSION);

    let pos = torus.position();
    vec_near(pos.x_dir, Pnt::new(0.0, 1.0, 0.0), CONFUSION);
    vec_near(pos.y_dir, Pnt::new(-1.0, 0.0, 0.0), CONFUSION);
    vec_near(pos.z_dir, Pnt::new(0.0, 0.0, 1.0), CONFUSION);

    // Value(0, 0): P = O + (R+r)·new_X = (R+r)·(0,1,0) = (0, 7, 0)
    pnt_near(torus.value(0.0, 0.0), Pnt::new(0.0, 7.0, 0.0), CONFUSION);
}

/// TEST 16: Topology flags — both U and V are closed and periodic.
#[test]
fn test_topology_flags() {
    let torus = default_torus(3.0, 1.0);

    assert!(torus.is_u_closed(), "torus must be closed in U");
    assert!(torus.is_v_closed(), "torus must be closed in V");
    assert!(torus.is_u_periodic(), "torus must be periodic in U");
    assert!(torus.is_v_periodic(), "torus must be periodic in V");
    near(torus.u_period(), 2.0 * PI, ANGULAR);
    near(torus.v_period(), 2.0 * PI, ANGULAR);
    near(torus.u_first_parameter(), 0.0, ANGULAR);
    near(torus.u_last_parameter(), 2.0 * PI, ANGULAR);
    near(torus.v_first_parameter(), 0.0, ANGULAR);
    near(torus.v_last_parameter(), 2.0 * PI, ANGULAR);
}

/// TEST 17: Reversed parameters.
///
/// UReversedParameter(U) = 2π - U.
/// VReversedParameter(V) = 2π - V.
#[test]
fn test_reversed_parameters() {
    let torus = default_torus(5.0, 2.0);

    near(torus.u_reversed_parameter(0.0), 2.0 * PI, CONFUSION);
    near(torus.u_reversed_parameter(PI / 2.0), 3.0 * PI / 2.0, CONFUSION);
    near(torus.u_reversed_parameter(PI), PI, CONFUSION);

    near(torus.v_reversed_parameter(0.0), 2.0 * PI, CONFUSION);
    near(torus.v_reversed_parameter(PI / 4.0), 7.0 * PI / 4.0, CONFUSION);
    near(torus.v_reversed_parameter(2.0 * PI), 0.0, CONFUSION);
}

/// TEST 18: Copy is independent from original.
#[test]
fn test_copy_independence() {
    let mut torus = default_torus(5.0, 2.0);
    let copy = torus.copy();
    torus.set_major_radius(10.0);
    torus.set_minor_radius(4.0);

    // The copy retains original values.
    near(copy.major_radius(), 5.0, CONFUSION);
    near(copy.minor_radius(), 2.0, CONFUSION);
    near(torus.major_radius(), 10.0, CONFUSION);
    near(torus.minor_radius(), 4.0, CONFUSION);
}

/// TEST 19: Non-identity placement — torus axis along X, centered away from
/// origin.
///
/// Frame: origin=(1,2,3), Z=X_world, X=Y_world, Y=Z_world (right-handed).
/// With R=4, r=1:
///   Value(U=0, V=0) = O + (R+r)·frame_X = (1,2,3) + 5·(0,1,0) = (1,7,3)
#[test]
fn test_non_identity_placement() {
    let frame = Ax3 {
        location: Pnt::new(1.0, 2.0, 3.0),
        x_dir: Pnt::new(0.0, 1.0, 0.0),  // X → world Y
        y_dir: Pnt::new(0.0, 0.0, 1.0),  // Y → world Z
        z_dir: Pnt::new(1.0, 0.0, 0.0),  // Z → world X (axis)
    };
    let r_maj = 4.0;
    let r_min = 1.0;
    let torus = GeomToroidalSurface::new(frame, r_maj, r_min);

    // P(0, 0) = (1,2,3) + 5·(0,1,0) = (1, 7, 3)
    pnt_near(torus.value(0.0, 0.0), Pnt::new(1.0, 7.0, 3.0), CONFUSION);

    // P(π/2, 0) = O + (R+r)·Y_world_dir = (1,2,3) + 5·(0,0,1) = (1, 2, 8)
    pnt_near(
        torus.value(PI / 2.0, 0.0),
        Pnt::new(1.0, 2.0, 8.0),
        CONFUSION,
    );

    // P(0, π/2) = O + R·frame_X + r·frame_Z = (1,2,3) + 4·(0,1,0) + 1·(1,0,0)
    //           = (2, 6, 3)
    pnt_near(torus.value(0.0, PI / 2.0), Pnt::new(2.0, 6.0, 3.0), CONFUSION);
}
