//! Ported OpenCascade unit tests -- `Geom2d_Parabola`.
//!
//! Faithful Rust port of OpenCascade's `Geom2d_Parabola` test suite
//! (OCCT package TKG2d). Same numeric inputs, expected values, and tolerances as
//! upstream (`Precision::Confusion()` -> [`ironstream::precision::CONFUSION`],
//! `Precision::Angular()` -> [`ironstream::precision::ANGULAR`]).
//!
//! OCCT helper-type mapping:
//! - `gp_Pnt2d`                  -> [`ironstream::gp2d::Pnt2d`]
//! - `gp_Dir2d`                  -> [`ironstream::gp2d::Pnt2d`] (unit vector)
//! - `gp_Ax2d`                   -> [`ironstream::gp2d::Ax2d`]
//! - `gp_Vec2d`                  -> [`ironstream::gp2d::Vec2d`]
//! - `gp_Trsf2d`                 -> [`ironstream::gp2d::Trsf2d`]
//! - `Handle(Geom2d_Parabola)`   -> the value [`ironstream::geom2d_parabola::Geom2dParabola`]

use ironstream::geom2d_parabola::{Geom2dParabola, Parab2d};
use ironstream::geom2d_circle::Ax22d2;
use ironstream::gp2d::{Ax2d, Pnt2d, Trsf2d};
use ironstream::precision::{ANGULAR, CONFUSION};

// ─────────────────────────────────── helpers ──────────────────────────────────

/// `EXPECT_NEAR(a, b, tol)` helper.
fn near(a: f64, b: f64, tol: f64) {
    assert!((a - b).abs() <= tol, "expected {a} ~= {b} (tol {tol})");
}

/// Build the standard fixture parabola: vertex at origin, axis along +X,
/// direct frame, focal = 2.
fn make_parabola() -> Geom2dParabola {
    Geom2dParabola::new(
        Ax2d::new(Pnt2d::origin(), Pnt2d::new(1.0, 0.0)),
        2.0,
        true,
    )
}

// ─────────────────────────────────── tests ────────────────────────────────────

// TEST(Geom2d_ParabolaTest, Constructor)
#[test]
fn constructor() {
    let par = Geom2dParabola::new(
        Ax2d::new(Pnt2d::origin(), Pnt2d::new(1.0, 0.0)),
        3.0,
        true,
    );
    near(par.focal(), 3.0, CONFUSION);
    // parameter p = 2 * focal
    near(par.parameter(), 6.0, CONFUSION);
}

// TEST(Geom2d_ParabolaTest, FocalAndParameter)
#[test]
fn focal_and_parameter() {
    let par = make_parabola();
    // focal = 2, parameter p = 2*focal = 4
    near(par.focal(), 2.0, CONFUSION);
    near(par.parameter(), 4.0, CONFUSION);
}

// TEST(Geom2d_ParabolaTest, Focus)
#[test]
fn focus() {
    let par = make_parabola();
    let f = par.focus();
    // focus is at (focal, 0) = (2, 0) from the vertex at origin
    near(f.x, 2.0, CONFUSION);
    near(f.y, 0.0, CONFUSION);
}

// TEST(Geom2d_ParabolaTest, Eccentricity)
#[test]
fn eccentricity() {
    let par = make_parabola();
    // parabola eccentricity is always 1
    near(par.eccentricity(), 1.0, CONFUSION);
}

// TEST(Geom2d_ParabolaTest, Vertex)
#[test]
fn vertex() {
    let origin = Pnt2d::new(3.0, 4.0);
    let par = Geom2dParabola::new(
        Ax2d::new(origin, Pnt2d::new(1.0, 0.0)),
        1.0,
        true,
    );
    // vertex = location of the local frame
    near(par.location().x, 3.0, CONFUSION);
    near(par.location().y, 4.0, CONFUSION);
}

// TEST(Geom2d_ParabolaTest, D0AtVertex)
#[test]
fn d0_at_vertex() {
    let par = make_parabola();
    // At U=0 the point should be the vertex (origin)
    let p = par.d0(0.0);
    near(p.x, 0.0, CONFUSION);
    near(p.y, 0.0, CONFUSION);
}

// TEST(Geom2d_ParabolaTest, D0OffVertex)
#[test]
fn d0_off_vertex() {
    // parabola: Y² = 2*p*X with p = 2*focal.
    // focal=2, p=4; at U=2: X = 4/(2*4)=0.5, Y = 2
    let par = make_parabola();
    let pt = par.d0(2.0);
    near(pt.x, 0.5, CONFUSION);
    near(pt.y, 2.0, CONFUSION);
}

// TEST(Geom2d_ParabolaTest, D0PointSatisfiesParabolaEquation)
#[test]
fn d0_satisfies_parabola_equation() {
    // For any U: Y² = 2*p*X  (p = parameter = 2*focal)
    let par = make_parabola(); // focal=2, p=4
    let p_param = par.parameter(); // 4.0
    for u_int in [-10_i32, -5, -2, -1, 0, 1, 2, 5, 10] {
        let u = u_int as f64;
        let pt = par.d0(u);
        // Y² = 2*p*X
        near(pt.y * pt.y, 2.0 * p_param * pt.x, CONFUSION);
    }
}

// TEST(Geom2d_ParabolaTest, ValueAlias)
#[test]
fn value_alias() {
    let par = make_parabola();
    let p1 = par.d0(3.0);
    let p2 = par.value(3.0);
    near(p1.x, p2.x, CONFUSION);
    near(p1.y, p2.y, CONFUSION);
}

// TEST(Geom2d_ParabolaTest, D1AtVertex)
#[test]
fn d1_at_vertex() {
    let par = make_parabola(); // focal=2, p=4
    // At U=0: V1 = (0/4)*XDir + YDir = (0,1)
    let (_pt, v1) = par.d1(0.0);
    near(v1.x, 0.0, CONFUSION);
    near(v1.y, 1.0, CONFUSION);
}

// TEST(Geom2d_ParabolaTest, D1General)
#[test]
fn d1_general() {
    let par = make_parabola(); // focal=2, p=4
    // At U=4: V1 = (4/4)*XDir + YDir = (1,0) + (0,1) = (1,1)
    let (_pt, v1) = par.d1(4.0);
    near(v1.x, 1.0, CONFUSION);
    near(v1.y, 1.0, CONFUSION);
}

// TEST(Geom2d_ParabolaTest, D2IsConstant)
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
    }
}

// TEST(Geom2d_ParabolaTest, D3IsZero)
#[test]
fn d3_is_zero() {
    // All derivatives of order >= 3 are zero for a parabola
    let par = make_parabola();
    let (_pt, _v1, _v2, v3) = par.d3(1.5);
    near(v3.x, 0.0, CONFUSION);
    near(v3.y, 0.0, CONFUSION);
}

// TEST(Geom2d_ParabolaTest, DNConsistency)
#[test]
fn dn_consistency() {
    let par = make_parabola();
    let u = 3.0;
    // DN(1) must match D1's velocity
    let (_pt, v1) = par.d1(u);
    let dn1 = par.dn(u, 1);
    near(dn1.x, v1.x, CONFUSION);
    near(dn1.y, v1.y, CONFUSION);

    // DN(2) must match D2's acceleration
    let (_pt2, _v1b, v2) = par.d2(u);
    let dn2 = par.dn(u, 2);
    near(dn2.x, v2.x, CONFUSION);
    near(dn2.y, v2.y, CONFUSION);

    // DN(3) must be zero
    let dn3 = par.dn(u, 3);
    near(dn3.x, 0.0, CONFUSION);
    near(dn3.y, 0.0, CONFUSION);
}

// TEST(Geom2d_ParabolaTest, ReversedParameter)
#[test]
fn reversed_parameter() {
    let par = make_parabola();
    // For a parabola: ReversedParameter(U) = -U
    near(par.reversed_parameter(3.5), -3.5, CONFUSION);
    near(par.reversed_parameter(-2.0), 2.0, CONFUSION);
    near(par.reversed_parameter(0.0), 0.0, CONFUSION);
}

// TEST(Geom2d_ParabolaTest, Reverse)
#[test]
fn reverse() {
    let par = make_parabola();
    let u = 3.0;
    // Reverse the parabola; evaluate at the reversed parameter.
    // The geometric point should coincide with the original evaluation.
    let p_before = par.d0(u);
    let mut rev = par.reversed();
    let u_rev = par.reversed_parameter(u);
    let p_after = rev.d0(u_rev);
    near(p_before.x, p_after.x, CONFUSION);
    near(p_before.y, p_after.y, CONFUSION);
    // The focal length should be unchanged
    near(rev.focal(), par.focal(), CONFUSION);
    // Calling reverse() a second time should restore the original orientation
    rev.reverse();
    let p_restored = rev.d0(u);
    near(p_restored.x, p_before.x, CONFUSION);
    near(p_restored.y, p_before.y, CONFUSION);
}

// TEST(Geom2d_ParabolaTest, IsClosed)
#[test]
fn is_closed_and_periodic() {
    let par = make_parabola();
    assert!(!par.is_closed());
    assert!(!par.is_periodic());
}

// TEST(Geom2d_ParabolaTest, TransformTranslation)
#[test]
fn transform_translation() {
    let mut par = make_parabola();
    let trsf = Trsf2d::translation(Pnt2d::new(10.0, 5.0));
    par.transform(&trsf);
    // Vertex (location) must be translated
    near(par.location().x, 10.0, CONFUSION);
    near(par.location().y, 5.0, CONFUSION);
    // Focal length is unchanged by a pure translation
    near(par.focal(), 2.0, CONFUSION);
}

// TEST(Geom2d_ParabolaTest, TransformRotation)
#[test]
fn transform_rotation() {
    let mut par = make_parabola(); // focal=2, vertex at origin, axis along +X
    // Rotate 90 degrees CCW about the origin
    let trsf = Trsf2d::rotation(Pnt2d::origin(), std::f64::consts::FRAC_PI_2);
    par.transform(&trsf);
    // Focal unchanged by rotation
    near(par.focal(), 2.0, CONFUSION);
    // Vertex still at origin
    near(par.location().x, 0.0, CONFUSION);
    near(par.location().y, 0.0, CONFUSION);
    // Axis of symmetry now along +Y (X direction should be (0,1))
    let xd = par.x_direction();
    near(xd.x, 0.0, ANGULAR);
    near(xd.y, 1.0, ANGULAR);
    // Focus now at (0, focal) = (0, 2)
    let f = par.focus();
    near(f.x, 0.0, CONFUSION);
    near(f.y, 2.0, CONFUSION);
}

// TEST(Geom2d_ParabolaTest, TransformedCopy)
#[test]
fn transformed_copy() {
    let par = make_parabola();
    let trsf = Trsf2d::translation(Pnt2d::new(1.0, 2.0));
    let par2 = par.transformed(&trsf);
    near(par2.location().x, 1.0, CONFUSION);
    near(par2.location().y, 2.0, CONFUSION);
    // Original untouched
    near(par.location().x, 0.0, CONFUSION);
    near(par.focal(), par2.focal(), CONFUSION);
}

// TEST(Geom2d_ParabolaTest, Copy)
#[test]
fn copy() {
    let par = make_parabola();
    let par2 = par.copy();
    near(par2.focal(), par.focal(), CONFUSION);
    near(par2.location().x, par.location().x, CONFUSION);
    near(par2.location().y, par.location().y, CONFUSION);
}

// TEST(Geom2d_ParabolaTest, NonZeroOrigin)
#[test]
fn non_zero_origin() {
    let origin = Pnt2d::new(1.0, 2.0);
    let par = Geom2dParabola::new(
        Ax2d::new(origin, Pnt2d::new(1.0, 0.0)),
        5.0,
        true,
    );
    // At U=0 the vertex itself is returned
    let p = par.d0(0.0);
    near(p.x, 1.0, CONFUSION);
    near(p.y, 2.0, CONFUSION);
    // focus at vertex + focal * XDir = (1+5, 2) = (6, 2)
    let f = par.focus();
    near(f.x, 6.0, CONFUSION);
    near(f.y, 2.0, CONFUSION);
}

// TEST(Geom2d_ParabolaTest, Directrix)
#[test]
fn directrix() {
    let par = make_parabola(); // focal = 2
    // Directrix is at x = -focal = -2, in the Y direction
    let dir_ax = par.directrix();
    near(dir_ax.location.x, -2.0, CONFUSION);
    near(dir_ax.location.y, 0.0, CONFUSION);
    // The directrix direction is the Y axis direction = (0, 1)
    near(dir_ax.direction.x.abs(), 0.0, CONFUSION);
    near(dir_ax.direction.y.abs(), 1.0, CONFUSION);
}

// TEST(Geom2d_ParabolaTest, FocusDistanceProperty)
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

// TEST(Geom2d_ParabolaTest, SetFocal)
#[test]
fn set_focal() {
    let mut par = make_parabola();
    par.set_focal(7.0);
    near(par.focal(), 7.0, CONFUSION);
    near(par.parameter(), 14.0, CONFUSION);
}

// TEST(Geom2d_ParabolaTest, FromParab2d)
#[test]
fn from_parab2d() {
    let ax = Ax22d2::from_x_axis(Pnt2d::new(1.0, 2.0), Pnt2d::new(1.0, 0.0), true);
    let prb = Parab2d::from_axis(ax, 3.0);
    let par = Geom2dParabola::from_parab2d(prb);
    near(par.focal(), 3.0, CONFUSION);
    near(par.location().x, 1.0, CONFUSION);
    near(par.location().y, 2.0, CONFUSION);
}
