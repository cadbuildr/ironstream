//! Ported OpenCascade unit tests — `Geom_TrimmedCurve`.
//!
//! Faithful Rust port of OCCT's `Geom_TrimmedCurve` test suite (package TKG3d).
//! Same numeric inputs, expected values, and tolerances as upstream
//! (`Precision::Confusion()` -> [`ironstream::precision::CONFUSION`],
//! `Precision::Angular()` -> [`ironstream::precision::ANGULAR`]).
//!
//! OCCT helper-type mapping:
//! - `gp_Pnt`                   -> [`ironstream::gp::Pnt`]
//! - `gp_Dir`                   -> [`ironstream::gp::Pnt`] (via `Pnt::dir`)
//! - `gp_Ax2`                   -> [`ironstream::gp::Ax3`]
//! - `gp_Vec`                   -> [`ironstream::gp::Vec3`]
//! - `gp_Trsf`                  -> [`ironstream::gp::Trsf`]
//! - `Handle(Geom_Line)`        -> [`ironstream::geom_line::GeomLine`]
//! - `Handle(Geom_Circle)`      -> [`ironstream::geom_circle::GeomCircle`]
//! - `Handle(Geom_Ellipse)`     -> [`ironstream::geom_ellipse::GeomEllipse`]
//! - `Handle(Geom_TrimmedCurve)` -> [`ironstream::geom_trimmed_curve::GeomTrimmedCurve`]

use ironstream::geom_circle::GeomCircle;
use ironstream::geom_ellipse::GeomEllipse;
use ironstream::geom_line::GeomLine;
use ironstream::geom_trimmed_curve::{BasisCurve, GeomTrimmedCurve};
use ironstream::gp::{Ax1, Ax3, Pnt, Trsf};
use ironstream::precision::{ANGULAR, CONFUSION};
use std::f64::consts::PI;

// ─────────────────────────────── helpers ──────────────────────────────────────

fn near(a: f64, b: f64, tol: f64) {
    assert!((a - b).abs() <= tol, "expected {a} ~= {b} (tol {tol})");
}

fn pnt_near(a: Pnt, b: Pnt, tol: f64) {
    assert!(
        a.distance(b) <= tol,
        "expected {a:?} ~= {b:?} (dist {}, tol {tol})",
        a.distance(b)
    );
}

/// Build a `gp_Ax2(loc, dir)` the same way OCCT's constructor does.
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

// ─────────────────────────────── tests ────────────────────────────────────────

// TEST(Geom_TrimmedCurveTest, Constructor_Line)
/// Verify that a trimmed line stores the correct parameter bounds.
#[test]
fn constructor_line() {
    let line = GeomLine::new(Ax1::new(Pnt::origin(), Pnt::new(1.0, 0.0, 0.0)));
    let tc = GeomTrimmedCurve::new(BasisCurve::Line(line), 0.0, 5.0);
    near(tc.first_parameter(), 0.0, CONFUSION);
    near(tc.last_parameter(), 5.0, CONFUSION);
}

// TEST(Geom_TrimmedCurveTest, Value_Line)
/// `Value(t)` on a trimmed line should return the point on the basis line.
#[test]
fn value_line() {
    let origin = Pnt::new(1.0, 2.0, 3.0);
    let dir = Pnt::new(1.0, 0.0, 0.0);
    let line = GeomLine::new(Ax1::new(origin, dir));
    let tc = GeomTrimmedCurve::new(BasisCurve::Line(line), 0.0, 10.0);

    // At t=0 we get the origin.
    pnt_near(tc.value(0.0), origin, CONFUSION);
    // At t=3 we advance 3 units along X.
    pnt_near(tc.value(3.0), Pnt::new(4.0, 2.0, 3.0), CONFUSION);
    // At t=10 we reach the end of the trim.
    pnt_near(tc.value(10.0), Pnt::new(11.0, 2.0, 3.0), CONFUSION);
}

// TEST(Geom_TrimmedCurveTest, FirstLastParameter)
/// After constructing with (u2, u1) order (u2 < u1) the interval is still [u2,u1].
#[test]
fn first_last_parameter_swapped_input() {
    let line = GeomLine::new(Ax1::new(Pnt::origin(), Pnt::new(0.0, 1.0, 0.0)));
    // Pass (10.0, 2.0) — the constructor must order them.
    let tc = GeomTrimmedCurve::new(BasisCurve::Line(line), 10.0, 2.0);
    near(tc.first_parameter(), 2.0, CONFUSION);
    near(tc.last_parameter(), 10.0, CONFUSION);
}

// TEST(Geom_TrimmedCurveTest, BasisCurve)
/// `BasisCurve()` returns the curve used at construction.
#[test]
fn basis_curve_is_line() {
    let line = GeomLine::new(Ax1::new(Pnt::origin(), Pnt::new(1.0, 0.0, 0.0)));
    let tc = GeomTrimmedCurve::new(BasisCurve::Line(line), 0.0, 1.0);
    assert!(matches!(tc.basis_curve(), BasisCurve::Line(_)));
}

// TEST(Geom_TrimmedCurveTest, IsClosed_Circle_Full)
/// A circle trimmed to a full period is closed.
#[test]
fn is_closed_circle_full_period() {
    let axis = ax2(Pnt::origin(), Pnt::dir(0.0, 0.0, 1.0));
    let circle = GeomCircle::from_ax3(axis, 5.0);
    // Trim from 0 to 2π — exactly one full revolution.
    let tc = GeomTrimmedCurve::new(BasisCurve::Circle(circle), 0.0, 2.0 * PI);
    assert!(tc.is_closed(), "full-period circle trim must be closed");
}

// TEST(Geom_TrimmedCurveTest, IsClosed_Line)
/// A line is never closed (endpoints are different).
#[test]
fn is_closed_line() {
    let line = GeomLine::new(Ax1::new(Pnt::origin(), Pnt::new(1.0, 0.0, 0.0)));
    let tc = GeomTrimmedCurve::new(BasisCurve::Line(line), 0.0, 5.0);
    assert!(!tc.is_closed());
}

// TEST(Geom_TrimmedCurveTest, IsPeriodic_Circle)
/// `IsPeriodic` on a trimmed circle is `true` (delegates to basis circle).
#[test]
fn is_periodic_circle() {
    let axis = ax2(Pnt::origin(), Pnt::dir(0.0, 0.0, 1.0));
    let circle = GeomCircle::from_ax3(axis, 3.0);
    let tc = GeomTrimmedCurve::new(BasisCurve::Circle(circle), 0.0, PI);
    assert!(tc.is_periodic());
}

// TEST(Geom_TrimmedCurveTest, IsPeriodic_Line)
/// `IsPeriodic` on a trimmed line is `false`.
#[test]
fn is_periodic_line() {
    let line = GeomLine::new(Ax1::new(Pnt::origin(), Pnt::new(1.0, 0.0, 0.0)));
    let tc = GeomTrimmedCurve::new(BasisCurve::Line(line), 0.0, 1.0);
    assert!(!tc.is_periodic());
}

// TEST(Geom_TrimmedCurveTest, D1_Line)
/// D1 on a trimmed line returns the line direction and the point.
#[test]
fn d1_line() {
    let dir = Pnt::new(1.0, 0.0, 0.0);
    let line = GeomLine::new(Ax1::new(Pnt::origin(), dir));
    let tc = GeomTrimmedCurve::new(BasisCurve::Line(line), 0.0, 4.0);

    let (p, v1) = tc.d1(2.0);
    pnt_near(p, Pnt::new(2.0, 0.0, 0.0), CONFUSION);
    // First derivative of a line is its unit direction.
    near(v1.norm(), 1.0, CONFUSION);
    near(v1.x, 1.0, CONFUSION);
}

// TEST(Geom_TrimmedCurveTest, D2_Circle)
/// D2 on a trimmed circle arc: verifies point, tangent, and curvature vector.
#[test]
fn d2_circle() {
    let r = 2.0_f64;
    let axis = ax2(Pnt::origin(), Pnt::dir(0.0, 0.0, 1.0));
    let circle = GeomCircle::from_ax3(axis, r);
    let tc = GeomTrimmedCurve::new(BasisCurve::Circle(circle), 0.0, PI);

    // At U = PI/2 (top of circle):
    // P = (0, r, 0)
    // V1 = (-r, 0, 0)          — tangent pointing left
    // V2 = (0, -r, 0)          — centripetal acceleration
    let u = PI / 2.0;
    let (p, v1, v2) = tc.d2(u);
    pnt_near(p, Pnt::new(0.0, r, 0.0), CONFUSION);
    near(v1.x, -r, CONFUSION);
    near(v1.y, 0.0, CONFUSION);
    near(v2.x, 0.0, CONFUSION);
    near(v2.y, -r, CONFUSION);
}

// TEST(Geom_TrimmedCurveTest, D3_Circle)
/// D3 on a trimmed circle at U = 0 checks the third-derivative vector.
#[test]
fn d3_circle() {
    let r = 1.0_f64;
    let axis = ax2(Pnt::origin(), Pnt::dir(0.0, 0.0, 1.0));
    let circle = GeomCircle::from_ax3(axis, r);
    let tc = GeomTrimmedCurve::new(BasisCurve::Circle(circle), 0.0, PI);

    // At U=0: P=(1,0,0), V1=(0,1,0), V2=(-1,0,0), V3=(0,-1,0).
    let (p, v1, v2, v3) = tc.d3(0.0);
    pnt_near(p, Pnt::new(r, 0.0, 0.0), CONFUSION);
    near(v1.y, r, CONFUSION);
    near(v2.x, -r, CONFUSION);
    near(v3.y, -r, CONFUSION);
}

// TEST(Geom_TrimmedCurveTest, DN_Ellipse)
/// DN on a trimmed ellipse equals direct D1 at the same parameter.
#[test]
fn dn_ellipse() {
    let a2 = ax2(Pnt::origin(), Pnt::dir(0.0, 0.0, 1.0));
    let ellipse = GeomEllipse::from_ax3(a2, 10.0, 5.0);
    let tc = GeomTrimmedCurve::new(BasisCurve::Ellipse(ellipse.clone()), 0.0, PI);

    let u = PI / 4.0;
    let dn1 = tc.dn(u, 1);
    let (_, d1_vec) = tc.d1(u);
    // DN(u, 1) must equal the first derivative from D1.
    near(dn1.x, d1_vec.x, CONFUSION);
    near(dn1.y, d1_vec.y, CONFUSION);
    near(dn1.z, d1_vec.z, CONFUSION);
}

// TEST(Geom_TrimmedCurveTest, SetTrim)
/// `SetTrim` changes the trim interval while keeping the basis curve unchanged.
#[test]
fn set_trim() {
    let line = GeomLine::new(Ax1::new(Pnt::origin(), Pnt::new(1.0, 0.0, 0.0)));
    let mut tc = GeomTrimmedCurve::new(BasisCurve::Line(line), 0.0, 5.0);

    near(tc.first_parameter(), 0.0, CONFUSION);
    near(tc.last_parameter(), 5.0, CONFUSION);

    tc.set_trim(1.0, 3.0, true, false);
    near(tc.first_parameter(), 1.0, CONFUSION);
    near(tc.last_parameter(), 3.0, CONFUSION);

    // The basis curve is still a line.
    assert!(matches!(tc.basis_curve(), BasisCurve::Line(_)));
}

// TEST(Geom_TrimmedCurveTest, Reverse_Line)
/// After `Reverse()` the parametric direction is flipped for a line.
#[test]
fn reverse_line() {
    let origin = Pnt::origin();
    let dir = Pnt::new(1.0, 0.0, 0.0);
    let line = GeomLine::new(Ax1::new(origin, dir));
    let mut tc = GeomTrimmedCurve::new(BasisCurve::Line(line), 0.0, 4.0);

    // Before reverse: value(0) = (0,0,0), value(4) = (4,0,0).
    pnt_near(tc.value(tc.first_parameter()), Pnt::new(0.0, 0.0, 0.0), CONFUSION);
    pnt_near(tc.value(tc.last_parameter()), Pnt::new(4.0, 0.0, 0.0), CONFUSION);

    tc.reverse();

    // After reverse: first_parameter maps to the old last geometric point.
    // The basis line direction flips, so value(first_param) == (4, 0, 0).
    let fp = tc.first_parameter();
    let lp = tc.last_parameter();
    pnt_near(tc.value(fp), Pnt::new(4.0, 0.0, 0.0), CONFUSION);
    pnt_near(tc.value(lp), Pnt::new(0.0, 0.0, 0.0), CONFUSION);
}

// TEST(Geom_TrimmedCurveTest, Transform_Scale)
/// `Transform` with a uniform scale changes the trim parameters proportionally.
#[test]
fn transform_scale() {
    let line = GeomLine::new(Ax1::new(Pnt::origin(), Pnt::new(1.0, 0.0, 0.0)));
    let mut tc = GeomTrimmedCurve::new(BasisCurve::Line(line), 1.0, 3.0);

    // Scale by factor 2 about the origin.
    let mut t = Trsf::identity();
    t.set_scale(Pnt::origin(), 2.0);
    tc.transform(&t);

    // Trim parameters scaled: [1, 3] → [2, 6].
    near(tc.first_parameter(), 2.0, CONFUSION);
    near(tc.last_parameter(), 6.0, CONFUSION);
}

// TEST(Geom_TrimmedCurveTest, Circle_Arc_Value)
/// A half-circle trim: value at mid-parameter is at the top of the circle.
#[test]
fn circle_arc_value() {
    let r = 3.0_f64;
    let axis = ax2(Pnt::origin(), Pnt::dir(0.0, 0.0, 1.0));
    let circle = GeomCircle::from_ax3(axis, r);
    // Trim from 0 to π — a half-circle going from (r,0,0) to (-r,0,0).
    let tc = GeomTrimmedCurve::new(BasisCurve::Circle(circle), 0.0, PI);

    // At t=0: start (r, 0, 0).
    pnt_near(tc.value(0.0), Pnt::new(r, 0.0, 0.0), CONFUSION);
    // At t=π/2: top (0, r, 0).
    pnt_near(tc.value(PI / 2.0), Pnt::new(0.0, r, 0.0), CONFUSION);
    // At t=π: end (-r, 0, 0).
    pnt_near(tc.value(PI), Pnt::new(-r, 0.0, 0.0), CONFUSION);
}

// TEST(Geom_TrimmedCurveTest, ReversedParameter_Circle)
/// `ReversedParameter` maps a parameter U to 2π − U for a circle.
#[test]
fn reversed_parameter_circle() {
    let axis = ax2(Pnt::origin(), Pnt::dir(0.0, 0.0, 1.0));
    let circle = GeomCircle::from_ax3(axis, 5.0);
    let tc = GeomTrimmedCurve::new(BasisCurve::Circle(circle), 0.0, PI);
    // Circle reversed_parameter: 2π − U.
    near(tc.reversed_parameter(PI / 3.0), 2.0 * PI - PI / 3.0, ANGULAR);
}

// TEST(Geom_TrimmedCurveTest, Sense_False)
/// When `sense = false` the basis curve is reversed so D1 points in the
/// opposite direction compared to the original un-reversed curve.
#[test]
fn sense_false_reverses_curve() {
    // A line going in the +X direction, trimmed [0, 5] with sense=false.
    let origin = Pnt::origin();
    let line = GeomLine::new(Ax1::new(origin, Pnt::new(1.0, 0.0, 0.0)));
    let tc = GeomTrimmedCurve::with_sense(BasisCurve::Line(line), 0.0, 5.0, false, false);

    // The D1 tangent should now point in the −X direction.
    let (_, v1) = tc.d1(tc.first_parameter());
    assert!(v1.x < 0.0, "with sense=false the direction must flip; got {v1:?}");
}

// TEST(Geom_TrimmedCurveTest, Ellipse_Trim_D1)
/// D1 at the start of a trimmed ellipse arc.
#[test]
fn ellipse_trim_d1() {
    let a2 = ax2(Pnt::origin(), Pnt::dir(0.0, 0.0, 1.0));
    // Major radius 4, minor 2 — standard ellipse in XY plane.
    let ellipse = GeomEllipse::from_ax3(a2, 4.0, 2.0);
    let tc = GeomTrimmedCurve::new(BasisCurve::Ellipse(ellipse.clone()), 0.0, PI / 2.0);

    // At U=0: P=(4,0,0); V1=(0, minor_r, 0) = (0, 2, 0).
    let (p, v1) = tc.d1(0.0);
    pnt_near(p, Pnt::new(4.0, 0.0, 0.0), CONFUSION);
    near(v1.x, 0.0, CONFUSION);
    near(v1.y, 2.0, CONFUSION);
}
