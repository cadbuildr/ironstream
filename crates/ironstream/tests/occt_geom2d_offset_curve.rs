//! Port of OpenCascade `Geom2d_OffsetCurve` tests (TKG2d).
//!
//! All tolerances use:
//!   * `precision::CONFUSION` (1e-7) for linear distances
//!   * `precision::ANGULAR`  (1e-12) for angular / unit-vector checks

use ironstream::geom2d_circle::Geom2dCircle;
use ironstream::geom2d_ellipse::Geom2dEllipse;
use ironstream::geom2d_line::Geom2dLine;
use ironstream::geom2d_offset_curve::{BasisCurve2d, Geom2dOffsetCurve};
use ironstream::gp2d::{Ax2d, Pnt2d, Trsf2d};
use ironstream::precision;
use std::f64::consts::PI;

// ─────────────────────────────────────────────────────────────────────────────
// Helpers
// ─────────────────────────────────────────────────────────────────────────────

/// Linear tolerance used throughout.
const TOL: f64 = precision::CONFUSION;

/// Angular tolerance used throughout.
const ANG: f64 = precision::ANGULAR;

/// A circle of radius `r` centred at the origin, with direct orientation.
fn unit_circle(r: f64) -> Geom2dOffsetCurve {
    let c = Geom2dCircle::from_x_axis(Pnt2d::origin(), Pnt2d::new(1.0, 0.0), r, true);
    Geom2dOffsetCurve::new(BasisCurve2d::Circle(c), 0.0)
}

/// A horizontal line through the origin.
fn horiz_line() -> Geom2dOffsetCurve {
    let l = Geom2dLine::from_point_dir(Pnt2d::origin(), Pnt2d::new(1.0, 0.0));
    Geom2dOffsetCurve::new(BasisCurve2d::Line(l), 0.0)
}

// ─────────────────────────────────────────────────────────────────────────────
// 1. Constructor / accessors
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn constructor_stores_offset_and_basis() {
    let c = Geom2dCircle::from_x_axis(Pnt2d::origin(), Pnt2d::new(1.0, 0.0), 3.0, true);
    let oc = Geom2dOffsetCurve::new(BasisCurve2d::Circle(c), 2.0);
    assert!((oc.offset() - 2.0).abs() < ANG);
}

// ─────────────────────────────────────────────────────────────────────────────
// 2. Parameter range inherited from basis
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn first_last_parameter_from_circle() {
    let oc = unit_circle(5.0);
    assert!((oc.first_parameter() - 0.0).abs() < ANG);
    assert!((oc.last_parameter() - 2.0 * PI).abs() < 1e-14);
}

#[test]
fn first_last_parameter_from_line() {
    let oc = horiz_line();
    assert!(oc.first_parameter() <= -1.0e100);
    assert!(oc.last_parameter() >= 1.0e100);
}

// ─────────────────────────────────────────────────────────────────────────────
// 3. IsClosed / IsPeriodic
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn is_closed_and_periodic_for_offset_circle() {
    let c = Geom2dCircle::from_x_axis(Pnt2d::origin(), Pnt2d::new(1.0, 0.0), 5.0, true);
    // Offset by 1.0 — the offset of a circle is still a circle, so still closed.
    let oc = Geom2dOffsetCurve::new(BasisCurve2d::Circle(c), 1.0);
    assert!(oc.is_periodic(), "circle basis should be periodic");
    assert!(oc.is_closed(), "offset circle should be closed");
}

#[test]
fn is_not_closed_for_line() {
    let oc = horiz_line();
    assert!(!oc.is_closed(), "line offset should not be closed");
    assert!(!oc.is_periodic(), "line offset should not be periodic");
}

// ─────────────────────────────────────────────────────────────────────────────
// 4. Value (D0) — circle offset
// ─────────────────────────────────────────────────────────────────────────────

/// An offset of a circle of radius R by d gives a circle of radius R+d.
/// At u=0, C(0) = (R,0) and the normal to the circle at u=0 is +X,
/// so Q(0) = (R+d, 0).
#[test]
fn value_circle_offset_at_zero() {
    let r = 4.0;
    let d = 1.5;
    let c = Geom2dCircle::from_x_axis(Pnt2d::origin(), Pnt2d::new(1.0, 0.0), r, true);
    let oc = Geom2dOffsetCurve::new(BasisCurve2d::Circle(c), d);
    let q = oc.value(0.0);
    // At u=0: C(0)=(R,0), C'(0)=(0,R) → perp=(−R,0) → normalize=(−1,0)
    // Q(0) = (R,0) + d*(−1,0) = (R−d, 0)
    // Wait, let's reconsider: the formula is Q = C + d * normalize(perp(C'))
    // C'(0) = R*(−sin(0)*X + cos(0)*Y) = R*Y = (0, R)
    // perp(C'(0)) = perp(0, R) = (−R, 0)
    // normalize(perp(C'(0))) = (−1, 0)
    // Q(0) = (R, 0) + d*(−1, 0) = (R−d, 0)
    // For inward offset d>0: Q(0) = (R−d, 0)
    let expected_x = r - d;
    let expected_y = 0.0_f64;
    assert!(
        (q.x - expected_x).abs() < TOL,
        "expected x={expected_x}, got x={}",
        q.x
    );
    assert!(
        (q.y - expected_y).abs() < TOL,
        "expected y={expected_y}, got y={}",
        q.y
    );
}

/// For an offset circle, the distance from centre = R - d (inward normal).
#[test]
fn value_circle_offset_radius_consistency() {
    let r = 5.0;
    let d = 2.0;
    let c = Geom2dCircle::from_x_axis(Pnt2d::origin(), Pnt2d::new(1.0, 0.0), r, true);
    let oc = Geom2dOffsetCurve::new(BasisCurve2d::Circle(c), d);
    // Sample multiple points and verify they all lie on a circle of radius r-d.
    let expected_r = r - d;
    for k in 0..8 {
        let u = k as f64 * PI / 4.0;
        let q = oc.value(u);
        let dist = q.distance(Pnt2d::origin());
        assert!(
            (dist - expected_r).abs() < TOL,
            "u={u:.4}: expected radius {expected_r}, got {dist}"
        );
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 5. Value (D0) — line offset
// ─────────────────────────────────────────────────────────────────────────────

/// The offset of a horizontal line y=0 by d should give the line y=d.
#[test]
fn value_line_offset_perpendicular() {
    let d = 3.0;
    let l = Geom2dLine::from_point_dir(Pnt2d::origin(), Pnt2d::new(1.0, 0.0));
    let oc = Geom2dOffsetCurve::new(BasisCurve2d::Line(l), d);
    // C(u) = (u, 0), C'(u) = (1, 0), perp(C') = (0, 1), N = (0, 1)
    // Q(u) = (u, 0) + d*(0, 1) = (u, d)
    for u in [-5.0, 0.0, 3.0, 10.0] {
        let q = oc.value(u);
        assert!(
            (q.x - u).abs() < TOL && (q.y - d).abs() < TOL,
            "u={u}: expected ({u}, {d}), got ({}, {})",
            q.x,
            q.y
        );
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 6. D1 — line offset derivative
// ─────────────────────────────────────────────────────────────────────────────

/// For a line Q(u) = (u, d), Q'(u) = (1, 0).
#[test]
fn d1_line_offset_derivative() {
    let d = 2.0;
    let l = Geom2dLine::from_point_dir(Pnt2d::origin(), Pnt2d::new(1.0, 0.0));
    let oc = Geom2dOffsetCurve::new(BasisCurve2d::Line(l), d);
    let (q, dq) = oc.d1(1.0);
    // Q(1) = (1, d)
    assert!((q.x - 1.0).abs() < TOL && (q.y - d).abs() < TOL);
    // Q'(u) = C'(u) + d * N'(u)
    // C' = (1,0), N = (0,1) is constant, N' = (0,0)
    // Q' = (1, 0)
    assert!(
        (dq.x - 1.0).abs() < TOL && dq.y.abs() < TOL,
        "expected dq=(1,0), got ({}, {})",
        dq.x,
        dq.y
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// 7. D2 — ellipse offset second derivative (numerical consistency)
// ─────────────────────────────────────────────────────────────────────────────

/// Check that D2 is consistent with D1 via finite differences.
#[test]
fn d2_ellipse_consistent_with_d1() {
    let e = Geom2dEllipse::from_major_axis(
        Ax2d::new(Pnt2d::origin(), Pnt2d::new(1.0, 0.0)),
        5.0,
        3.0,
        true,
    );
    let oc = Geom2dOffsetCurve::new(BasisCurve2d::Ellipse(e), 0.5);

    let u = 0.8;
    let h = 1e-6;
    let (_, dq) = oc.d1(u);
    let (_, _, d2q) = oc.d2(u);

    // Forward difference approximation of Q'': (Q'(u+h) - Q'(u-h)) / (2h)
    let (_, dq_p) = oc.d1(u + h);
    let (_, dq_m) = oc.d1(u - h);
    let fd_d2x = (dq_p.x - dq_m.x) / (2.0 * h);
    let fd_d2y = (dq_p.y - dq_m.y) / (2.0 * h);

    let tol = 1e-4; // finite-difference accuracy
    assert!(
        (d2q.x - fd_d2x).abs() < tol,
        "D2.x analytical={} fd={fd_d2x}",
        d2q.x
    );
    assert!(
        (d2q.y - fd_d2y).abs() < tol,
        "D2.y analytical={} fd={fd_d2y}",
        d2q.y
    );
    // Also verify D1 point matches
    let (q, _) = oc.d1(u);
    let q0 = oc.value(u);
    assert!((q.x - q0.x).abs() < TOL && (q.y - q0.y).abs() < TOL);
    let _ = dq; // suppress unused warning
}

// ─────────────────────────────────────────────────────────────────────────────
// 8. Reverse
// ─────────────────────────────────────────────────────────────────────────────

/// Reversing a line offset negates the offset and reverses the line.
/// A horizontal line offset by d, reversed, is the same line with offset −d.
#[test]
fn reverse_negates_offset() {
    let d = 3.0;
    let l = Geom2dLine::from_point_dir(Pnt2d::origin(), Pnt2d::new(1.0, 0.0));
    let oc = Geom2dOffsetCurve::new(BasisCurve2d::Line(l), d);
    let rev = oc.reversed();
    assert!((rev.offset() + d).abs() < ANG, "reversed offset should be -{d}");
}

// ─────────────────────────────────────────────────────────────────────────────
// 9. Transform — translation
// ─────────────────────────────────────────────────────────────────────────────

/// Translating the offset curve moves every point by the translation vector,
/// and the offset distance is unchanged (pure translation has scale=1).
#[test]
fn transform_translation_moves_points() {
    let d = 2.0;
    let l = Geom2dLine::from_point_dir(Pnt2d::origin(), Pnt2d::new(1.0, 0.0));
    let oc = Geom2dOffsetCurve::new(BasisCurve2d::Line(l), d);

    let tx = 7.0;
    let ty = -3.0;
    let t = Trsf2d::translation(Pnt2d::new(tx, ty));
    let oc2 = oc.transformed(&t);

    // The offset should be unchanged (scale = 1 for a translation).
    assert!((oc2.offset() - d).abs() < ANG);

    // Q2(u) = Q(u) + (tx, ty)
    for u in [0.0, 1.0, -2.0] {
        let q_orig = oc.value(u);
        let q_trans = oc2.value(u);
        assert!(
            (q_trans.x - (q_orig.x + tx)).abs() < TOL
                && (q_trans.y - (q_orig.y + ty)).abs() < TOL,
            "u={u}: translation mismatch"
        );
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 10. Transform — uniform scale
// ─────────────────────────────────────────────────────────────────────────────

/// Scaling by factor s scales the offset by s too.
#[test]
fn transform_scale_scales_offset() {
    let d = 1.0;
    let l = Geom2dLine::from_point_dir(Pnt2d::origin(), Pnt2d::new(1.0, 0.0));
    let oc = Geom2dOffsetCurve::new(BasisCurve2d::Line(l), d);

    let s = 3.0;
    let t = Trsf2d {
        m: [[s, 0.0], [0.0, s]],
        t: [0.0, 0.0],
    };
    let oc2 = oc.transformed(&t);
    assert!(
        (oc2.offset() - d * s).abs() < 1e-10,
        "expected offset={}, got {}",
        d * s,
        oc2.offset()
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// 11. Zero offset → on the basis curve
// ─────────────────────────────────────────────────────────────────────────────

/// With offset=0 the value must equal the basis curve at every parameter.
#[test]
fn zero_offset_matches_basis_curve() {
    let c = Geom2dCircle::from_x_axis(Pnt2d::origin(), Pnt2d::new(1.0, 0.0), 7.0, true);
    let oc = Geom2dOffsetCurve::new(BasisCurve2d::Circle(c), 0.0);
    for k in 0..8 {
        let u = k as f64 * PI / 4.0;
        let q = oc.value(u);
        let p = c.eval_d0(u);
        assert!(
            q.distance(p) < TOL,
            "u={u:.4}: offset0 mismatch: oc={q:?}, basis={p:?}"
        );
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 12. D3 — line: all higher derivatives zero
// ─────────────────────────────────────────────────────────────────────────────

/// For a line Q(u) = (u, d), all derivatives of order ≥ 2 must be zero.
#[test]
fn d3_line_offset_higher_derivatives_zero() {
    let d = 1.5;
    let l = Geom2dLine::from_point_dir(Pnt2d::origin(), Pnt2d::new(1.0, 0.0));
    let oc = Geom2dOffsetCurve::new(BasisCurve2d::Line(l), d);
    let (_, _, d2q, d3q) = oc.d3(0.5);
    assert!(d2q.norm() < TOL, "D2 should be zero for line offset: {d2q:?}");
    assert!(d3q.norm() < TOL, "D3 should be zero for line offset: {d3q:?}");
}
