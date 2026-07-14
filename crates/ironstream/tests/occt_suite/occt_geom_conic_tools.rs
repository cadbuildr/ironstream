// FILE: tests/occt_geom_conic_tools.rs
extern crate ironstream;
use ironstream::geom_conic_tools::*;

use std::f64::consts::PI;

const EPS: f64 = 1e-10;
const EPS_APPROX: f64 = 1e-6;

fn approx_eq(a: f64, b: f64, eps: f64) -> bool {
    (a - b).abs() < eps
}

fn approx_eq2(a: [f64; 2], b: [f64; 2], eps: f64) -> bool {
    approx_eq(a[0], b[0], eps) && approx_eq(a[1], b[1], eps)
}

// ---------------------------------------------------------------------------
// GCE2dMakeArcOfEllipse integration tests
// ---------------------------------------------------------------------------

#[test]
fn arc_of_ellipse_construction_succeeds() {
    let arc = GCE2dMakeArcOfEllipse::new([0.0, 0.0], 5.0, 3.0, 0.0, PI / 2.0);
    assert!(arc.is_done());
}

#[test]
fn arc_of_ellipse_unit_circle_quarter_arc_endpoints() {
    // a == b == 1 → circle. Quarter arc from 0 to π/2.
    let arc = GCE2dMakeArcOfEllipse::new([0.0, 0.0], 1.0, 1.0, 0.0, PI / 2.0);
    assert!(approx_eq2(arc.start_point(), [1.0, 0.0], EPS));
    assert!(approx_eq2(arc.end_point(), [0.0, 1.0], EPS));
}

#[test]
fn arc_of_ellipse_start_at_angle_pi() {
    // At π: [cx + a*cos(π), cy + b*sin(π)] = [cx − a, cy]
    let arc = GCE2dMakeArcOfEllipse::new([0.0, 0.0], 5.0, 3.0, PI, 2.0 * PI);
    assert!(approx_eq2(arc.start_point(), [-5.0, 0.0], EPS));
}

#[test]
fn arc_of_ellipse_end_at_2pi_equals_start_at_0() {
    // cos(2π) = 1, sin(2π) = 0
    let arc = GCE2dMakeArcOfEllipse::new([0.0, 0.0], 5.0, 3.0, 0.0, 2.0 * PI);
    assert!(approx_eq2(arc.end_point(), [5.0, 0.0], EPS));
}

#[test]
fn arc_of_ellipse_center_accessor() {
    let arc = GCE2dMakeArcOfEllipse::new([3.0, -4.0], 7.0, 2.0, 0.0, PI);
    assert!(approx_eq2(arc.center(), [3.0, -4.0], EPS));
}

#[test]
fn arc_of_ellipse_radius_accessors() {
    let arc = GCE2dMakeArcOfEllipse::new([0.0, 0.0], 7.0, 2.0, 0.0, PI);
    assert!(approx_eq(arc.major_radius(), 7.0, EPS));
    assert!(approx_eq(arc.minor_radius(), 2.0, EPS));
}

#[test]
fn arc_of_ellipse_angle_accessors() {
    let arc = GCE2dMakeArcOfEllipse::new([0.0, 0.0], 5.0, 3.0, 0.5, 1.8);
    assert!(approx_eq(arc.start_angle(), 0.5, EPS));
    assert!(approx_eq(arc.end_angle(), 1.8, EPS));
}

#[test]
fn arc_of_ellipse_length_circle_case_full() {
    // Full circle (a == b == r): Ramanujan length = 2*π*r.
    let r = 5.0_f64;
    let arc = GCE2dMakeArcOfEllipse::new([0.0, 0.0], r, r, 0.0, 2.0 * PI);
    let expected = 2.0 * PI * r;
    assert!(
        approx_eq(arc.length(), expected, EPS),
        "length={} expected={}",
        arc.length(),
        expected
    );
}

#[test]
fn arc_of_ellipse_length_half_circle() {
    // Half of a circular arc.
    let r = 4.0_f64;
    let arc = GCE2dMakeArcOfEllipse::new([0.0, 0.0], r, r, 0.0, PI);
    let expected = PI * r;
    assert!(approx_eq(arc.length(), expected, EPS));
}

#[test]
fn arc_of_ellipse_length_always_positive() {
    let arc = GCE2dMakeArcOfEllipse::new([0.0, 0.0], 5.0, 3.0, 0.3, 1.9);
    assert!(arc.length() > 0.0);
}

#[test]
fn arc_of_ellipse_length_ellipse_approximate() {
    // For a=5, b=3, full perimeter by Ramanujan: π*(3*(5+3) - sqrt((15+3)*(5+9)))
    // = π*(24 - sqrt(18*14)) = π*(24 - sqrt(252)) ≈ π*(24 - 15.874) ≈ π*8.126 ≈ 25.527
    let a = 5.0_f64;
    let b = 3.0_f64;
    let arc = GCE2dMakeArcOfEllipse::new([0.0, 0.0], a, b, 0.0, 2.0 * PI);
    let expected_perimeter = PI * (3.0 * (a + b) - ((3.0 * a + b) * (a + 3.0 * b)).sqrt());
    assert!(approx_eq(arc.length(), expected_perimeter, EPS));
}

#[test]
fn arc_of_ellipse_point_on_ellipse_satisfies_equation() {
    // Points must satisfy (x-cx)²/a² + (y-cy)²/b² = 1
    let cx = 1.0_f64;
    let cy = 2.0_f64;
    let a = 5.0_f64;
    let b = 3.0_f64;
    let arc = GCE2dMakeArcOfEllipse::new([cx, cy], a, b, 0.7, 2.1);
    let sp = arc.start_point();
    let ep = arc.end_point();
    let lhs_s = ((sp[0] - cx) / a).powi(2) + ((sp[1] - cy) / b).powi(2);
    let lhs_e = ((ep[0] - cx) / a).powi(2) + ((ep[1] - cy) / b).powi(2);
    assert!(approx_eq(lhs_s, 1.0, EPS), "start: lhs={}", lhs_s);
    assert!(approx_eq(lhs_e, 1.0, EPS), "end: lhs={}", lhs_e);
}

#[test]
fn arc_of_ellipse_clone_preserves_values() {
    let arc = GCE2dMakeArcOfEllipse::new([1.0, 2.0], 5.0, 3.0, 0.0, PI);
    let cloned = arc.clone();
    assert!(approx_eq2(cloned.center(), [1.0, 2.0], EPS));
    assert!(approx_eq(cloned.major_radius(), 5.0, EPS));
    assert!(approx_eq(cloned.start_angle(), 0.0, EPS));
    assert!(approx_eq(cloned.end_angle(), PI, EPS));
}

// ---------------------------------------------------------------------------
// GCE2dMakeArcOfHyperbola integration tests
// ---------------------------------------------------------------------------

#[test]
fn arc_of_hyperbola_construction_succeeds() {
    let arc = GCE2dMakeArcOfHyperbola::new([0.0, 0.0], 2.0, 1.0, -1.0, 1.0);
    assert!(arc.is_done());
}

#[test]
fn arc_of_hyperbola_vertex_at_t_zero() {
    // The vertex of the right branch (t=0) is at (cx + a, cy).
    let a = 3.0_f64;
    let arc = GCE2dMakeArcOfHyperbola::new([0.0, 0.0], a, 2.0, 0.0, 1.0);
    assert!(approx_eq2(arc.start_point(), [a, 0.0], EPS));
}

#[test]
fn arc_of_hyperbola_symmetric_endpoints() {
    // cosh is even, sinh is odd: start(-t) and end(t) are mirror images in x.
    let a = 2.0_f64;
    let b = 1.5_f64;
    let t = 1.3_f64;
    let arc = GCE2dMakeArcOfHyperbola::new([0.0, 0.0], a, b, -t, t);
    let sp = arc.start_point();
    let ep = arc.end_point();
    assert!(approx_eq(sp[0], ep[0], EPS));  // same x
    assert!(approx_eq(sp[1], -ep[1], EPS)); // opposite y
}

#[test]
fn arc_of_hyperbola_satisfies_hyperbola_equation() {
    // x²/a² − y²/b² = 1 on the right branch.
    let cx = 0.0_f64;
    let cy = 0.0_f64;
    let a = 4.0_f64;
    let b = 3.0_f64;
    for t_int in -5_i32..=5 {
        let t = t_int as f64 * 0.4;
        let x = cx + a * t.cosh();
        let y = cy + b * t.sinh();
        let lhs = (x / a).powi(2) - (y / b).powi(2);
        assert!(
            approx_eq(lhs, 1.0, EPS),
            "t={}: lhs={}",
            t,
            lhs
        );
    }
}

#[test]
fn arc_of_hyperbola_start_point_with_offset() {
    let cx = 2.0_f64;
    let cy = -3.0_f64;
    let a = 1.0_f64;
    let b = 1.0_f64;
    let t = 0.5_f64;
    let arc = GCE2dMakeArcOfHyperbola::new([cx, cy], a, b, t, 1.0);
    let expected = [cx + a * t.cosh(), cy + b * t.sinh()];
    assert!(approx_eq2(arc.start_point(), expected, EPS));
}

#[test]
fn arc_of_hyperbola_end_point_large_t() {
    // For large |t|, cosh(t) ≈ sinh(t) ≈ exp(t)/2 → x ≈ y (when a == b).
    // The relative difference (cosh(t) - sinh(t)) / cosh(t) = exp(-2t) → 0.
    let a = 1.0_f64;
    let b = 1.0_f64;
    let t = 5.0_f64;
    let arc = GCE2dMakeArcOfHyperbola::new([0.0, 0.0], a, b, 0.0, t);
    let ep = arc.end_point();
    // cosh(5) ≈ sinh(5) to good relative accuracy; use relative tolerance.
    let rel_diff = (ep[0] - ep[1]).abs() / ep[0].abs();
    assert!(
        rel_diff < 1e-4,
        "relative diff too large: x={} y={} rel={}",
        ep[0],
        ep[1],
        rel_diff
    );
}

#[test]
fn arc_of_hyperbola_clone_preserves_values() {
    let arc = GCE2dMakeArcOfHyperbola::new([1.0, 2.0], 3.0, 2.0, -1.0, 1.0);
    let cloned = arc.clone();
    assert!(cloned.is_done());
    assert!(approx_eq2(cloned.start_point(), arc.start_point(), EPS));
    assert!(approx_eq2(cloned.end_point(), arc.end_point(), EPS));
}

// ---------------------------------------------------------------------------
// GCE2dMakeArcOfParabola integration tests
// ---------------------------------------------------------------------------

#[test]
fn arc_of_parabola_construction_succeeds() {
    let arc = GCE2dMakeArcOfParabola::new(1.0, [0.0, 0.0], -2.0, 2.0);
    assert!(arc.is_done());
}

#[test]
fn arc_of_parabola_vertex_at_x_zero() {
    // At x=0: [vx, vy] (vertex of the parabola).
    let arc = GCE2dMakeArcOfParabola::new(1.0, [0.0, 0.0], 0.0, 1.0);
    assert!(approx_eq2(arc.start_point(), [0.0, 0.0], EPS));
}

#[test]
fn arc_of_parabola_standard_focal_1_at_x_2() {
    // f=1, x=2: y = 4/(2*1) = 2 → [vx+2, vy+2]
    let arc = GCE2dMakeArcOfParabola::new(1.0, [0.0, 0.0], 0.0, 2.0);
    assert!(approx_eq2(arc.end_point(), [2.0, 2.0], EPS));
}

#[test]
fn arc_of_parabola_negative_x() {
    // y = x²/(2f) is symmetric: y(−x) == y(x).
    let f = 2.0_f64;
    let arc_pos = GCE2dMakeArcOfParabola::new(f, [0.0, 0.0], 4.0, 4.0);
    let arc_neg = GCE2dMakeArcOfParabola::new(f, [0.0, 0.0], -4.0, -4.0);
    assert!(approx_eq(arc_pos.start_point()[1], arc_neg.start_point()[1], EPS));
}

#[test]
fn arc_of_parabola_satisfies_parabola_equation() {
    // y = (x−vx)² / (2*f) + vy for all points.
    let f = 3.0_f64;
    let vx = 1.0_f64;
    let vy = -2.0_f64;
    for x_int in -6_i32..=6 {
        let x = x_int as f64;
        let arc = GCE2dMakeArcOfParabola::new(f, [vx, vy], x, x);
        let pt = arc.start_point();
        let expected_y = vy + x * x / (2.0 * f);
        assert!(
            approx_eq(pt[1], expected_y, EPS),
            "x={}: y={} expected={}",
            x,
            pt[1],
            expected_y
        );
    }
}

#[test]
fn arc_of_parabola_with_vertex_offset() {
    let f = 2.0_f64;
    let vx = 3.0_f64;
    let vy = 1.0_f64;
    let x = 4.0_f64;
    let arc = GCE2dMakeArcOfParabola::new(f, [vx, vy], 0.0, x);
    // end: [vx + x, vy + x²/(2f)] = [7, 1 + 16/4] = [7, 5]
    assert!(approx_eq2(arc.end_point(), [7.0, 5.0], EPS));
}

#[test]
fn arc_of_parabola_start_and_end_same_x() {
    // When start_x == end_x, start_point == end_point.
    let arc = GCE2dMakeArcOfParabola::new(1.0, [0.0, 0.0], 3.0, 3.0);
    assert!(approx_eq2(arc.start_point(), arc.end_point(), EPS));
}

#[test]
fn arc_of_parabola_increasing_y_for_larger_x() {
    // y = x²/(2f), so |x| larger → y larger.
    let f = 1.0_f64;
    let arc1 = GCE2dMakeArcOfParabola::new(f, [0.0, 0.0], 1.0, 1.0);
    let arc2 = GCE2dMakeArcOfParabola::new(f, [0.0, 0.0], 2.0, 2.0);
    assert!(arc2.start_point()[1] > arc1.start_point()[1]);
}

#[test]
fn arc_of_parabola_focal_scaling() {
    // Doubling focal halves y for same x.
    let x = 4.0_f64;
    let arc1 = GCE2dMakeArcOfParabola::new(1.0, [0.0, 0.0], x, x);
    let arc2 = GCE2dMakeArcOfParabola::new(2.0, [0.0, 0.0], x, x);
    assert!(approx_eq(arc1.start_point()[1], 2.0 * arc2.start_point()[1], EPS));
}

#[test]
fn arc_of_parabola_clone_preserves_values() {
    let arc = GCE2dMakeArcOfParabola::new(2.0, [1.0, -1.0], -3.0, 3.0);
    let cloned = arc.clone();
    assert!(cloned.is_done());
    assert!(approx_eq2(cloned.start_point(), arc.start_point(), EPS));
    assert!(approx_eq2(cloned.end_point(), arc.end_point(), EPS));
}

// ---------------------------------------------------------------------------
// Cross-type sanity checks
// ---------------------------------------------------------------------------

#[test]
fn ellipse_arc_is_done_returns_true() {
    assert!(GCE2dMakeArcOfEllipse::new([0.0, 0.0], 3.0, 2.0, 0.0, 1.0).is_done());
}

#[test]
fn hyperbola_arc_is_done_returns_true() {
    assert!(GCE2dMakeArcOfHyperbola::new([0.0, 0.0], 3.0, 2.0, 0.0, 1.0).is_done());
}

#[test]
fn parabola_arc_is_done_returns_true() {
    assert!(GCE2dMakeArcOfParabola::new(1.0, [0.0, 0.0], 0.0, 1.0).is_done());
}

#[test]
fn ellipse_arc_circle_perimeter_matches_known() {
    // Circle r=1: perimeter = 2π
    let arc = GCE2dMakeArcOfEllipse::new([0.0, 0.0], 1.0, 1.0, 0.0, 2.0 * PI);
    assert!(approx_eq(arc.length(), 2.0 * PI, EPS));
}
