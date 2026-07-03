//! Integration tests for `Geom2d_TrimmedCurve` (OCCT TKG2d).
//!
//! All imports are from `ironstream::` only. Tolerances follow OCCT:
//!   - linear: `precision::CONFUSION = 1e-7`
//!   - angular: `precision::ANGULAR  = 1e-12`

use ironstream::geom2d_bspline::Geom2dBSplineCurve;
use ironstream::geom2d_circle::Geom2dCircle;
use ironstream::geom2d_ellipse::Geom2dEllipse;
use ironstream::geom2d_hyperbola::Geom2dHyperbola;
use ironstream::geom2d_line::Geom2dLine;
use ironstream::geom2d_parabola::Geom2dParabola;
use ironstream::geom2d_trimmed_curve::{BasisCurve2d, Geom2dTrimmedCurve};
use ironstream::gp2d::{Ax2d, Pnt2d, Trsf2d};
use ironstream::precision::{ANGULAR, CONFUSION};
use std::f64::consts::PI;

// ─────────────────────────────────────────────────────────────────────────────
// Helpers
// ─────────────────────────────────────────────────────────────────────────────

fn x_line() -> Geom2dLine {
    Geom2dLine::from_point_dir(Pnt2d::origin(), Pnt2d::new(1.0, 0.0))
}

fn unit_circle() -> Geom2dCircle {
    Geom2dCircle::from_x_axis(Pnt2d::origin(), Pnt2d::new(1.0, 0.0), 1.0, true)
}

fn make_ellipse(major: f64, minor: f64) -> Geom2dEllipse {
    Geom2dEllipse::from_major_axis(
        Ax2d::new(Pnt2d::origin(), Pnt2d::new(1.0, 0.0)),
        major,
        minor,
        true,
    )
}

fn make_hyperbola(a: f64, b: f64) -> Geom2dHyperbola {
    Geom2dHyperbola::new(
        Ax2d::new(Pnt2d::origin(), Pnt2d::new(1.0, 0.0)),
        a,
        b,
        true,
    )
}

fn make_parabola(focal: f64) -> Geom2dParabola {
    Geom2dParabola::new(
        Ax2d::new(Pnt2d::origin(), Pnt2d::new(1.0, 0.0)),
        focal,
        true,
    )
}

fn make_bspline_line() -> Geom2dBSplineCurve {
    // Degree-1 BSpline through (0,0) and (1,1): effectively a straight line.
    let poles = [Pnt2d::new(0.0, 0.0), Pnt2d::new(1.0, 1.0)];
    let knots = [0.0, 1.0];
    let mults = [2usize, 2];
    Geom2dBSplineCurve::new(&poles, &knots, &mults, 1, false)
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 1 — constructor stores ordered trim interval
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn constructor_and_parameter_range() {
    // Reversed arguments → should be stored in ascending order.
    let tc = Geom2dTrimmedCurve::new(BasisCurve2d::Line(x_line()), 8.0, 2.0);
    assert!(
        (tc.first_parameter() - 2.0).abs() < CONFUSION,
        "first={}", tc.first_parameter()
    );
    assert!(
        (tc.last_parameter() - 8.0).abs() < CONFUSION,
        "last={}", tc.last_parameter()
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 2 — Value on a line: P(u) = u * (1, 0)
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn value_line_endpoints() {
    let tc = Geom2dTrimmedCurve::new(BasisCurve2d::Line(x_line()), 3.0, 9.0);
    let p3 = tc.value(3.0);
    let p9 = tc.value(9.0);
    assert!((p3.x - 3.0).abs() < CONFUSION && p3.y.abs() < CONFUSION, "{p3:?}");
    assert!((p9.x - 9.0).abs() < CONFUSION && p9.y.abs() < CONFUSION, "{p9:?}");
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 3 — D1 on a circle: tangent ⊥ radius
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn d1_circle_tangent_perpendicular() {
    let tc = Geom2dTrimmedCurve::new(BasisCurve2d::Circle(unit_circle()), 0.0, PI);
    let u = PI / 6.0;
    let (p, v1) = tc.d1(u);
    // radius vector = p - center = p (center is origin)
    let dot = p.x * v1.x + p.y * v1.y;
    assert!(dot.abs() < CONFUSION, "dot={dot}");
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 4 — D2 on a circle: second derivative = centripetal = -(P - center)
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn d2_circle_centripetal() {
    let tc = Geom2dTrimmedCurve::new(BasisCurve2d::Circle(unit_circle()), 0.0, 2.0 * PI);
    let u = PI / 4.0;
    let (p, _v1, v2) = tc.d2(u);
    // V2 = -P for unit circle at origin.
    assert!((v2.x + p.x).abs() < CONFUSION, "v2.x={} p.x={}", v2.x, p.x);
    assert!((v2.y + p.y).abs() < CONFUSION, "v2.y={} p.y={}", v2.y, p.y);
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 5 — D3 on a line: all higher derivatives are zero
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn d3_line_is_zero() {
    let line = Geom2dLine::from_point_dir(Pnt2d::new(0.0, 0.0), Pnt2d::new(0.0, 1.0));
    let tc = Geom2dTrimmedCurve::new(BasisCurve2d::Line(line), 1.0, 10.0);
    let (_p, _v1, v2, v3) = tc.d3(5.0);
    assert!(v2.x.abs() < CONFUSION && v2.y.abs() < CONFUSION, "v2={v2:?}");
    assert!(v3.x.abs() < CONFUSION && v3.y.abs() < CONFUSION, "v3={v3:?}");
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 6 — DN: N=1 matches D1 tangent
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn dn_n1_matches_d1() {
    let tc = Geom2dTrimmedCurve::new(BasisCurve2d::Circle(unit_circle()), 0.0, PI);
    let u = 0.8;
    let (_p, v1) = tc.d1(u);
    let dn1 = tc.dn(u, 1);
    assert!((dn1.x - v1.x).abs() < ANGULAR, "dn1={dn1:?} v1={v1:?}");
    assert!((dn1.y - v1.y).abs() < ANGULAR, "dn1={dn1:?} v1={v1:?}");
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 7 — IsClosed: full circle is closed; half-arc is not
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn is_closed_full_vs_arc() {
    let full = Geom2dTrimmedCurve::new(BasisCurve2d::Circle(unit_circle()), 0.0, 2.0 * PI);
    let arc = Geom2dTrimmedCurve::new(BasisCurve2d::Circle(unit_circle()), 0.0, PI);
    assert!(full.is_closed(), "full circle must be closed");
    assert!(!arc.is_closed(), "half arc must not be closed");
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 8 — IsPeriodic: always false
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn is_periodic_always_false() {
    let circle_tc = Geom2dTrimmedCurve::new(BasisCurve2d::Circle(unit_circle()), 0.0, 2.0 * PI);
    let line_tc = Geom2dTrimmedCurve::new(BasisCurve2d::Line(x_line()), 0.0, 5.0);
    assert!(!circle_tc.is_periodic());
    assert!(!line_tc.is_periodic());
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 9 — BasisCurve accessor
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn basis_curve_accessor() {
    let tc = Geom2dTrimmedCurve::new(BasisCurve2d::Line(x_line()), 0.0, 1.0);
    assert!(
        matches!(tc.basis_curve(), BasisCurve2d::Line(_)),
        "expected Line"
    );

    let tc2 = Geom2dTrimmedCurve::new(BasisCurve2d::Circle(unit_circle()), 0.0, PI);
    assert!(
        matches!(tc2.basis_curve(), BasisCurve2d::Circle(_)),
        "expected Circle"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 10 — SetTrim updates the interval
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn set_trim_updates_interval() {
    let mut tc = Geom2dTrimmedCurve::new(BasisCurve2d::Line(x_line()), 0.0, 10.0);
    tc.set_trim(2.0, 6.0);
    assert!((tc.first_parameter() - 2.0).abs() < CONFUSION);
    assert!((tc.last_parameter() - 6.0).abs() < CONFUSION);
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 11 — SetTrim with reversed arguments (should reorder)
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn set_trim_reversed_args() {
    let mut tc = Geom2dTrimmedCurve::new(BasisCurve2d::Line(x_line()), 0.0, 10.0);
    tc.set_trim(9.0, 3.0);
    assert!((tc.first_parameter() - 3.0).abs() < CONFUSION);
    assert!((tc.last_parameter() - 9.0).abs() < CONFUSION);
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 12 — Reverse on a line: swaps the trim interval via ReversedParameter
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn reverse_line_interval() {
    // Line reversed_parameter(u) = -u.
    // Trim [2, 7]: reversed maps 7 → -7, 2 → -2 → new interval [-7, -2].
    let tc = Geom2dTrimmedCurve::new(BasisCurve2d::Line(x_line()), 2.0, 7.0);
    let rev = tc.reversed();
    assert!(
        (rev.first_parameter() - (-7.0)).abs() < CONFUSION,
        "u1={}", rev.first_parameter()
    );
    assert!(
        (rev.last_parameter() - (-2.0)).abs() < CONFUSION,
        "u2={}", rev.last_parameter()
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 13 — Reverse geometric consistency: start ↔ end
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn reverse_geometric_start_end_swap() {
    // Offset-origin line for interest: origin (5, 0), dir (1, 0), trim [0, 4].
    let line = Geom2dLine::from_point_dir(Pnt2d::new(5.0, 0.0), Pnt2d::new(1.0, 0.0));
    let tc = Geom2dTrimmedCurve::new(BasisCurve2d::Line(line), 0.0, 4.0);
    let p_start = tc.value(tc.first_parameter()); // (5, 0)
    let p_end = tc.value(tc.last_parameter());    // (9, 0)
    let rev = tc.reversed();
    let p_rev_start = rev.value(rev.first_parameter());
    let p_rev_end = rev.value(rev.last_parameter());
    // After reverse: the start of the reversed curve should be the end of the original.
    assert!(
        p_end.distance(p_rev_start) < CONFUSION,
        "end {p_end:?} ≠ rev_start {p_rev_start:?}"
    );
    assert!(
        p_start.distance(p_rev_end) < CONFUSION,
        "start {p_start:?} ≠ rev_end {p_rev_end:?}"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 14 — Transform (translation) shifts the start point
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn transform_translation_shifts_point() {
    let tc = Geom2dTrimmedCurve::new(BasisCurve2d::Line(x_line()), 2.0, 5.0);
    let t = Trsf2d::translation(Pnt2d::new(10.0, 20.0));
    let tc2 = tc.transformed(&t);
    // Line origin shifts by (10, 20), dir unchanged. Point at u=2: (12, 20).
    let p = tc2.value(tc2.first_parameter());
    assert!((p.x - 12.0).abs() < CONFUSION, "p.x={}", p.x);
    assert!((p.y - 20.0).abs() < CONFUSION, "p.y={}", p.y);
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 15 — Transform on a circle leaves trim parameters unchanged (scale=1)
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn transform_rotation_circle_params_unchanged() {
    let tc = Geom2dTrimmedCurve::new(BasisCurve2d::Circle(unit_circle()), 0.0, PI / 2.0);
    let t = Trsf2d::rotation(Pnt2d::origin(), PI / 4.0);
    let tc2 = tc.transformed(&t);
    assert!(
        (tc2.first_parameter() - 0.0).abs() < CONFUSION,
        "u1={}", tc2.first_parameter()
    );
    assert!(
        (tc2.last_parameter() - PI / 2.0).abs() < CONFUSION,
        "u2={}", tc2.last_parameter()
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 16 — Ellipse trimmed curve: value at endpoints
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn ellipse_trim_endpoints() {
    // Ellipse a=4, b=2. Trim [0, PI/2].
    let ell = make_ellipse(4.0, 2.0);
    let tc = Geom2dTrimmedCurve::new(BasisCurve2d::Ellipse(ell), 0.0, PI / 2.0);
    // At u=0: (a, 0) = (4, 0).
    let p0 = tc.value(0.0);
    assert!((p0.x - 4.0).abs() < CONFUSION && p0.y.abs() < CONFUSION, "{p0:?}");
    // At u=PI/2: (0, b) = (0, 2).
    let p1 = tc.value(PI / 2.0);
    assert!(p1.x.abs() < CONFUSION && (p1.y - 2.0).abs() < CONFUSION, "{p1:?}");
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 17 — Hyperbola trimmed: value at u=0 is vertex (a, 0)
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn hyperbola_trim_vertex() {
    // a=3, b=4. Vertex at (3, 0).
    let hyp = make_hyperbola(3.0, 4.0);
    let tc = Geom2dTrimmedCurve::new(BasisCurve2d::Hyperbola(hyp), -1.0, 1.0);
    let p = tc.value(0.0);
    assert!((p.x - 3.0).abs() < CONFUSION, "p.x={}", p.x);
    assert!(p.y.abs() < CONFUSION, "p.y={}", p.y);
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 18 — Parabola trimmed: vertex at u=0
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn parabola_trim_vertex_at_zero() {
    let par = make_parabola(1.0);
    let tc = Geom2dTrimmedCurve::new(BasisCurve2d::Parabola(par), -3.0, 3.0);
    let p = tc.value(0.0);
    assert!(p.x.abs() < CONFUSION, "p.x={}", p.x);
    assert!(p.y.abs() < CONFUSION, "p.y={}", p.y);
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 19 — BSpline trimmed: value at endpoints
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn bspline_trim_endpoints() {
    let bsp = make_bspline_line();
    let tc = Geom2dTrimmedCurve::new(BasisCurve2d::BSpline(bsp), 0.0, 1.0);
    let p0 = tc.value(0.0);
    let p1 = tc.value(1.0);
    assert!(p0.distance(Pnt2d::new(0.0, 0.0)) < CONFUSION, "{p0:?}");
    assert!(p1.distance(Pnt2d::new(1.0, 1.0)) < CONFUSION, "{p1:?}");
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 20 — Periodic trim: span > period is clamped to period
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn periodic_trim_clamped_to_period() {
    // Circle period = 2*PI. Request [0, 3*PI] → should clamp span to 2*PI.
    let tc = Geom2dTrimmedCurve::new(BasisCurve2d::Circle(unit_circle()), 0.0, 3.0 * PI);
    let span = tc.last_parameter() - tc.first_parameter();
    assert!(
        span <= 2.0 * PI + CONFUSION,
        "span={span} should be <= 2*PI"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 21 — Copy returns independent clone
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn copy_is_independent() {
    let tc = Geom2dTrimmedCurve::new(BasisCurve2d::Line(x_line()), 1.0, 4.0);
    let mut tc2 = tc.copy();
    tc2.set_trim(0.0, 10.0);
    // Original is unchanged.
    assert!((tc.first_parameter() - 1.0).abs() < CONFUSION);
    assert!((tc.last_parameter() - 4.0).abs() < CONFUSION);
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 22 — DN N=2 on circle
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn dn_n2_circle_equals_minus_point() {
    // For unit circle at origin: D2 at u = -(P - center) = -P.
    let tc = Geom2dTrimmedCurve::new(BasisCurve2d::Circle(unit_circle()), 0.0, 2.0 * PI);
    let u = PI / 3.0;
    let p = tc.value(u);
    let dn2 = tc.dn(u, 2);
    assert!((dn2.x + p.x).abs() < CONFUSION, "dn2.x={} p.x={}", dn2.x, p.x);
    assert!((dn2.y + p.y).abs() < CONFUSION, "dn2.y={} p.y={}", dn2.y, p.y);
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 23 — Transform (scale) on a line scales trim parameters
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn transform_scale_line_params() {
    // Scale by factor 2 (rotation 0, scale 2 via a scale matrix).
    // We simulate with a scale transform: m = [[2, 0], [0, 2]], t = [0, 0].
    use ironstream::gp2d::Trsf2d;
    let t = Trsf2d {
        m: [[2.0, 0.0], [0.0, 2.0]],
        t: [0.0, 0.0],
    };
    let tc = Geom2dTrimmedCurve::new(BasisCurve2d::Line(x_line()), 1.0, 3.0);
    let tc2 = tc.transformed(&t);
    // Parametric scale = |scale| = 2.
    assert!((tc2.first_parameter() - 2.0).abs() < CONFUSION, "u1={}", tc2.first_parameter());
    assert!((tc2.last_parameter() - 6.0).abs() < CONFUSION, "u2={}", tc2.last_parameter());
}
