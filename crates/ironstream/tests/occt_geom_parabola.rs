//! Ported OpenCascade unit tests -- `Geom_Parabola`.
//!
//! Faithful Rust port of OpenCascade's `Geom_Parabola` test suite
//! (OCCT package TKG3d). Same numeric inputs, expected values, and tolerances as
//! upstream (`Precision::Confusion()` -> [`ironstream::precision::CONFUSION`],
//! `Precision::Angular()` -> [`ironstream::precision::ANGULAR`]).
//!
//! OCCT helper-type mapping:
//! - `gp_Pnt`                  -> [`ironstream::gp::Pnt`]
//! - `gp_Dir`                  -> [`ironstream::gp::Pnt`] (via `Pnt::dir`)
//! - `gp_Ax2`                  -> [`ironstream::gp::Ax3`]
//! - `gp_Vec`                  -> [`ironstream::gp::Vec3`]
//! - `gp_Trsf`                 -> [`ironstream::gp::Trsf`]
//! - `Handle(Geom_Parabola)`   -> the value [`ironstream::geom_parabola::GeomParabola`]

use ironstream::geom_parabola::GeomParabola;
use ironstream::gp::{Ax3, Pnt, Trsf};
use ironstream::precision::{ANGULAR, CONFUSION};

// ─────────────────────────────────── helpers ──────────────────────────────────

/// `EXPECT_NEAR(a, b, tol)` helper — identical to the convention in other
/// occt_geom_*.rs tests.
fn near(a: f64, b: f64, tol: f64) {
    assert!((a - b).abs() <= tol, "expected {a} ~= {b} (tol {tol})");
}

/// Reproduce OCCT's `gp_Ax2(loc, dir)` constructor: chooses the X direction
/// from the normal using the same tie-breaking as OCCT so that D0/D1 expected
/// values match (identical helper to `occt_geom_circle.rs`).
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

/// Build the standard fixture parabola: vertex at origin, focal = 2,
/// lying in the XY plane with the axis of symmetry along +X.
fn make_parabola() -> GeomParabola {
    let a2 = ax2(Pnt::new(0.0, 0.0, 0.0), Pnt::dir(0.0, 0.0, 1.0));
    GeomParabola::new(a2, 2.0)
}

// ─────────────────────────────────── tests ────────────────────────────────────

// TEST(Geom_ParabolaTest, Constructor)
#[test]
fn constructor() {
    let a2 = ax2(Pnt::new(0.0, 0.0, 0.0), Pnt::dir(0.0, 0.0, 1.0));
    let par = GeomParabola::new(a2, 3.0);
    near(par.focal(), 3.0, CONFUSION);
    // parameter p = 2 * focal
    near(par.parameter(), 6.0, CONFUSION);
}

// TEST(Geom_ParabolaTest, FocalAndParameter)
#[test]
fn focal_and_parameter() {
    let par = make_parabola();
    // focal = 2, parameter p = 2*focal = 4
    near(par.focal(), 2.0, CONFUSION);
    near(par.parameter(), 4.0, CONFUSION);
}

// TEST(Geom_ParabolaTest, Focus)
#[test]
fn focus() {
    let par = make_parabola();
    let f = par.focus();
    // focus is at (focal, 0, 0) = (2, 0, 0) from the vertex at origin
    near(f.x, 2.0, CONFUSION);
    near(f.y, 0.0, CONFUSION);
    near(f.z, 0.0, CONFUSION);
}

// TEST(Geom_ParabolaTest, Eccentricity)
#[test]
fn eccentricity() {
    let par = make_parabola();
    // parabola eccentricity is always 1
    near(par.eccentricity(), 1.0, CONFUSION);
}

// TEST(Geom_ParabolaTest, Vertex)
#[test]
fn vertex() {
    let center = Pnt::new(3.0, 4.0, 5.0);
    let a2 = ax2(center, Pnt::dir(0.0, 0.0, 1.0));
    let par = GeomParabola::new(a2, 1.0);
    // vertex = location of the local frame
    assert!(par.location().is_equal(center, CONFUSION));
}

// TEST(Geom_ParabolaTest, D0AtVertex)
#[test]
fn d0_at_vertex() {
    let par = make_parabola();
    // At U=0 the point should be the vertex (origin)
    let p = par.d0(0.0);
    near(p.x, 0.0, CONFUSION);
    near(p.y, 0.0, CONFUSION);
    near(p.z, 0.0, CONFUSION);
}

// TEST(Geom_ParabolaTest, D0OffVertex)
#[test]
fn d0_off_vertex() {
    // parabola: Y² = 2*p*X with p = 2*focal.
    // At U=2: X = U²/(2*p) = 4/8 = 0.5, Y = U = 2
    let par = make_parabola(); // focal = 2, p = 4
    let p = par.d0(2.0);
    // x = 4 / (2*4) = 4/8 = 0.5
    near(p.x, 0.5, CONFUSION);
    near(p.y, 2.0, CONFUSION);
    near(p.z, 0.0, CONFUSION);
}

// TEST(Geom_ParabolaTest, D0PointSatisfiesParabolaEquation)
#[test]
fn d0_satisfies_equation() {
    // For any U: Y² = 2*p*X  (p = parameter = 2*focal)
    let par = make_parabola(); // focal=2, p=4
    let p_param = par.parameter(); // 4.0
    for u_int in [-10_i32, -5, -2, -1, 0, 1, 2, 5, 10] {
        let u = u_int as f64;
        let pt = par.d0(u);
        // Check Y² = 2*p*X  →  y² ≈ 2*4*x = 8*x
        near(pt.y * pt.y, 2.0 * p_param * pt.x, CONFUSION);
    }
}

// TEST(Geom_ParabolaTest, ValueAlias)
#[test]
fn value_alias() {
    let par = make_parabola();
    let p1 = par.d0(3.0);
    let p2 = par.value(3.0);
    near(p1.x, p2.x, CONFUSION);
    near(p1.y, p2.y, CONFUSION);
    near(p1.z, p2.z, CONFUSION);
}

// TEST(Geom_ParabolaTest, D1AtVertex)
#[test]
fn d1_at_vertex() {
    let par = make_parabola(); // focal=2, p=4
    // At U=0: V1 = (0/4)*XDir + YDir = YDir = (0,1,0)
    let (_pt, v1) = par.d1(0.0);
    near(v1.x, 0.0, CONFUSION);
    near(v1.y, 1.0, CONFUSION);
    near(v1.z, 0.0, CONFUSION);
}

// TEST(Geom_ParabolaTest, D1General)
#[test]
fn d1_general() {
    let par = make_parabola(); // focal=2, p=4
    // At U=4: V1 = (4/4)*XDir + YDir = (1,0,0) + (0,1,0) = (1,1,0)
    let (_pt, v1) = par.d1(4.0);
    near(v1.x, 1.0, CONFUSION);
    near(v1.y, 1.0, CONFUSION);
    near(v1.z, 0.0, CONFUSION);
}

// TEST(Geom_ParabolaTest, D2IsConstant)
#[test]
fn d2_is_constant() {
    // Second derivative of parabola is constant: (1/p)*XDir
    let par = make_parabola(); // focal=2, p=4
    for u_int in [-5_i32, 0, 5] {
        let u = u_int as f64;
        let (_pt, _v1, v2) = par.d2(u);
        // d²X/dU² = 1/p = 0.25, d²Y/dU² = 0
        near(v2.x, 1.0 / par.parameter(), CONFUSION);
        near(v2.y, 0.0, CONFUSION);
        near(v2.z, 0.0, CONFUSION);
    }
}

// TEST(Geom_ParabolaTest, D3IsZero)
#[test]
fn d3_is_zero() {
    // All derivatives of order >= 3 are zero for a parabola
    let par = make_parabola();
    let (_pt, _v1, _v2, v3) = par.d3(1.5);
    near(v3.x, 0.0, CONFUSION);
    near(v3.y, 0.0, CONFUSION);
    near(v3.z, 0.0, CONFUSION);
}

// TEST(Geom_ParabolaTest, DNConsistency)
#[test]
fn dn_consistency() {
    let par = make_parabola();
    let u = 3.0;
    // DN(1) must match D1's velocity
    let (_pt, v1) = par.d1(u);
    let dn1 = par.dn(u, 1);
    near(dn1.x, v1.x, CONFUSION);
    near(dn1.y, v1.y, CONFUSION);
    near(dn1.z, v1.z, CONFUSION);

    // DN(2) must match D2's acceleration
    let (_pt2, _v1b, v2) = par.d2(u);
    let dn2 = par.dn(u, 2);
    near(dn2.x, v2.x, CONFUSION);
    near(dn2.y, v2.y, CONFUSION);
    near(dn2.z, v2.z, CONFUSION);

    // DN(3) must be zero
    let dn3 = par.dn(u, 3);
    near(dn3.x, 0.0, CONFUSION);
    near(dn3.y, 0.0, CONFUSION);
    near(dn3.z, 0.0, CONFUSION);
}

// TEST(Geom_ParabolaTest, ReversedParameter)
#[test]
fn reversed_parameter() {
    let par = make_parabola();
    // For a parabola: ReversedParameter(U) = -U
    near(par.reversed_parameter(3.5), -3.5, CONFUSION);
    near(par.reversed_parameter(-2.0), 2.0, CONFUSION);
    near(par.reversed_parameter(0.0), 0.0, CONFUSION);
}

// TEST(Geom_ParabolaTest, Reverse)
#[test]
fn reverse() {
    let par = make_parabola();
    let u = 3.0;
    // Reverse the parabola, then evaluate at the reversed parameter.
    // The geometric point should coincide with the original evaluation.
    let p_before = par.d0(u);
    let mut rev = par.reversed();
    // After reversing, the parameter direction flips: U_rev = -U
    let u_rev = par.reversed_parameter(u);
    let p_after = rev.d0(u_rev);
    near(p_before.x, p_after.x, CONFUSION);
    near(p_before.y, p_after.y, CONFUSION);
    near(p_before.z, p_after.z, CONFUSION);
    // The focal length should be unchanged
    near(rev.focal(), par.focal(), CONFUSION);
    // Calling reverse() a second time should restore the original orientation
    rev.reverse();
    let p_restored = rev.d0(u);
    near(p_restored.x, p_before.x, CONFUSION);
    near(p_restored.y, p_before.y, CONFUSION);
}

// TEST(Geom_ParabolaTest, TransformTranslation)
#[test]
fn transform_translation() {
    let mut par = make_parabola();
    let shift = Pnt::new(10.0, 5.0, 3.0);
    let trsf = Trsf::translation(shift);
    par.transform(&trsf);
    // Vertex (location) must be translated
    near(par.location().x, 10.0, CONFUSION);
    near(par.location().y, 5.0, CONFUSION);
    near(par.location().z, 3.0, CONFUSION);
    // Focal length is unchanged by a pure translation
    near(par.focal(), 2.0, CONFUSION);
}

// TEST(Geom_ParabolaTest, TransformScale)
#[test]
fn transform_scale() {
    let mut par = make_parabola(); // focal = 2
    let mut trsf = Trsf::identity();
    trsf.set_scale(Pnt::new(0.0, 0.0, 0.0), 3.0);
    par.transform(&trsf);
    // Focal scales by the scale factor
    near(par.focal(), 6.0, CONFUSION);
}

// TEST(Geom_ParabolaTest, TransformedCopy)
#[test]
fn transformed_copy() {
    let par = make_parabola();
    let trsf = Trsf::translation(Pnt::new(1.0, 2.0, 3.0));
    let par2 = par.transformed(&trsf);
    near(par2.location().x, 1.0, CONFUSION);
    near(par2.location().y, 2.0, CONFUSION);
    near(par2.location().z, 3.0, CONFUSION);
    // Original untouched
    near(par.location().x, 0.0, CONFUSION);
    near(par.focal(), par2.focal(), CONFUSION);
}

// TEST(Geom_ParabolaTest, Copy)
#[test]
fn copy() {
    let par = make_parabola();
    let par2 = par.copy();
    near(par2.focal(), par.focal(), CONFUSION);
    assert!(par2.location().is_equal(par.location(), CONFUSION));
}

// TEST(Geom_ParabolaTest, NonZeroOrigin)
#[test]
fn non_zero_origin() {
    let origin = Pnt::new(1.0, 2.0, 3.0);
    let a2 = ax2(origin, Pnt::dir(0.0, 0.0, 1.0));
    let par = GeomParabola::new(a2, 5.0);
    // At U=0 the vertex itself is returned
    let p = par.d0(0.0);
    near(p.x, 1.0, CONFUSION);
    near(p.y, 2.0, CONFUSION);
    near(p.z, 3.0, CONFUSION);
    // focus at vertex + focal * XDir = (1+5, 2, 3) = (6, 2, 3)
    let f = par.focus();
    near(f.x, 6.0, CONFUSION);
    near(f.y, 2.0, CONFUSION);
    near(f.z, 3.0, CONFUSION);
}

// TEST(Geom_ParabolaTest, AxisCheck)
#[test]
fn axis_check() {
    let par = make_parabola();
    // The main axis should be the Z axis (the normal to the parabola plane)
    let ax = par.axis();
    near(ax.location.x, 0.0, CONFUSION);
    near(ax.location.y, 0.0, CONFUSION);
    near(ax.location.z, 0.0, CONFUSION);
    near(ax.direction.z.abs(), 1.0, ANGULAR);
}

// TEST(Geom_ParabolaTest, Directrix)
#[test]
fn directrix() {
    let par = make_parabola(); // focal = 2
    // Directrix is at x = -focal = -2
    let dir_line = par.directrix();
    near(dir_line.location().x, -2.0, CONFUSION);
    near(dir_line.location().y, 0.0, CONFUSION);
    // The directrix runs in the Y direction
    near(dir_line.direction().x.abs(), 0.0, CONFUSION);
    near(dir_line.direction().y.abs(), 1.0, CONFUSION);
}

// TEST(Geom_ParabolaTest, FocusDistanceProperty)
#[test]
fn focus_distance_property() {
    // For any point on the parabola: distance to focus == distance to directrix
    let par = make_parabola(); // focal=2, p=4
    let focus = par.focus();
    let focal = par.focal();
    for u_int in [-8_i32, -4, -2, -1, 0, 1, 2, 4, 8] {
        let u = u_int as f64;
        let pt = par.d0(u);
        // Distance to focus
        let dist_focus = pt.distance(focus);
        // Distance to directrix: x - (-focal) = pt.x + focal
        let dist_dir = pt.x + focal;
        near(dist_focus, dist_dir, CONFUSION);
    }
}

// TEST(Geom_ParabolaTest, SetFocal)
#[test]
fn set_focal() {
    let mut par = make_parabola();
    par.set_focal(7.0);
    near(par.focal(), 7.0, CONFUSION);
    near(par.parameter(), 14.0, CONFUSION);
}

// TEST(Geom_ParabolaTest, ClosedPeriodic)
#[test]
fn closed_periodic() {
    let par = make_parabola();
    assert!(!par.is_closed());
    assert!(!par.is_periodic());
}
