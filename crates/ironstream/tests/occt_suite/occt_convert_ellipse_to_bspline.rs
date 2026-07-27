//! Ported OCCT tests for `Convert_EllipseToBSplineCurve`
//! (package `TKMath`).
//!
//! Each `#[test]` mirrors a case from OCCT's test suite or verifies a
//! mathematical invariant of the rational quadratic B-spline conic construction.

use ironstream::convert_ellipse_to_bspline::{EllipseToBSpline, ParameterisationType};
use ironstream::gp::{Ax3, Pnt};
use ironstream::gp_elips::Elips;
use ironstream::precision::{ANGULAR, CONFUSION};
use std::f64::consts::{FRAC_PI_2, PI};

// ─────────────────────────────────── Helpers ──────────────────────────────────

fn ellipse_5_3() -> Elips {
    Elips::new(Ax3::identity(), 5.0, 3.0)
}

fn unit_circle() -> Elips {
    Elips::new(Ax3::identity(), 1.0, 1.0)
}

// ─────────────────────────────────── Tests ────────────────────────────────────

/// A full ellipse (0 → 2π) converts to 9 poles and 5 distinct knots.
#[test]
fn full_ellipse_nb_poles_and_knots() {
    let conv = EllipseToBSpline::new(&ellipse_5_3(), 0.0, 2.0 * PI, ParameterisationType::Rational);
    assert_eq!(conv.nb_poles(), 9, "full ellipse must have 9 poles");
    assert_eq!(conv.nb_knots(), 5, "full ellipse must have 5 knots");
}

/// A quarter arc (0 → π/2) needs 3 poles and 2 knots.
#[test]
fn quarter_arc_pole_and_knot_count() {
    let conv =
        EllipseToBSpline::new(&ellipse_5_3(), 0.0, FRAC_PI_2, ParameterisationType::Rational);
    assert_eq!(conv.nb_poles(), 3);
    assert_eq!(conv.nb_knots(), 2);
}

/// A half ellipse (0 → π) needs 5 poles and 3 knots.
#[test]
fn half_arc_pole_and_knot_count() {
    let conv = EllipseToBSpline::new(&ellipse_5_3(), 0.0, PI, ParameterisationType::Rational);
    assert_eq!(conv.nb_poles(), 5);
    assert_eq!(conv.nb_knots(), 3);
}

/// A three-quarter arc (0 → 3π/2) needs 7 poles and 4 knots.
#[test]
fn three_quarter_arc_pole_and_knot_count() {
    let conv = EllipseToBSpline::new(&ellipse_5_3(), 0.0, 1.5 * PI, ParameterisationType::Rational);
    assert_eq!(conv.nb_poles(), 7);
    assert_eq!(conv.nb_knots(), 4);
}

/// Knot multiplicities for a full ellipse must be [3,2,2,2,3].
#[test]
fn full_ellipse_multiplicities() {
    let conv = EllipseToBSpline::new(&ellipse_5_3(), 0.0, 2.0 * PI, ParameterisationType::Rational);
    let expected = [3_i32, 2, 2, 2, 3];
    for (i, &exp) in expected.iter().enumerate() {
        let idx = (i + 1) as i32;
        assert_eq!(
            conv.multiplicity(idx),
            exp,
            "multiplicity({idx}) expected {exp}, got {}",
            conv.multiplicity(idx)
        );
    }
}

/// End-point poles must lie exactly on the ellipse.
#[test]
fn endpoint_poles_on_ellipse() {
    let conv = EllipseToBSpline::new(&ellipse_5_3(), 0.0, 2.0 * PI, ParameterisationType::Rational);
    // P(1) = (5, 0, 0) and P(9) = (5, 0, 0) for a closed full ellipse.
    let p1 = conv.pole(1);
    let p9 = conv.pole(9);
    let expected = Pnt::new(5.0, 0.0, 0.0);
    assert!(
        p1.is_equal(expected, CONFUSION * 10.0),
        "first pole should be (5,0,0), got {p1:?}"
    );
    assert!(
        p9.is_equal(expected, CONFUSION * 10.0),
        "last pole should be (5,0,0), got {p9:?}"
    );
}

/// End-point weights must be 1.0; inner weights < 1.
#[test]
fn weights_structure_full_ellipse() {
    let conv = EllipseToBSpline::new(&ellipse_5_3(), 0.0, 2.0 * PI, ParameterisationType::Rational);
    // Odd poles 1,3,5,7,9 should have weight 1.
    for &idx in &[1_i32, 3, 5, 7, 9] {
        assert!(
            (conv.weight(idx) - 1.0).abs() < ANGULAR,
            "weight({idx}) should be 1.0, got {}",
            conv.weight(idx)
        );
    }
    // Even poles 2,4,6,8 should have weight cos(PI/4) ≈ 0.7071.
    let w_expected = (PI / 4.0).cos(); // cos(π/4) = 1/√2
    for &idx in &[2_i32, 4, 6, 8] {
        assert!(
            (conv.weight(idx) - w_expected).abs() < 1.0e-10,
            "weight({idx}) expected {w_expected:.6}, got {}",
            conv.weight(idx)
        );
    }
}

/// NURBS evaluation: sampled points must lie exactly on the ellipse.
#[test]
fn sampled_points_on_ellipse() {
    let a = 5.0_f64;
    let b = 3.0_f64;
    let e = Elips::new(Ax3::identity(), a, b);
    let conv = EllipseToBSpline::new(&e, 0.0, 2.0 * PI, ParameterisationType::Rational);

    let n = 64;
    for i in 0..=n {
        let u = i as f64 / n as f64 * 2.0 * PI;
        let p = conv.value(u);
        let residual = (p.x / a).powi(2) + (p.y / b).powi(2) - 1.0;
        assert!(
            residual.abs() < 1.0e-9,
            "u={u:.4}: point ({:.6},{:.6}) not on ellipse (residual={residual:e})",
            p.x,
            p.y
        );
    }
}

/// The unit circle (a = b = 1) NURBS gives points on the unit circle.
#[test]
fn unit_circle_points_at_radius_one() {
    let conv = EllipseToBSpline::new(&unit_circle(), 0.0, 2.0 * PI, ParameterisationType::Rational);
    for i in 0..=32 {
        let u = i as f64 / 32.0 * 2.0 * PI;
        let p = conv.value(u);
        let r = (p.x * p.x + p.y * p.y).sqrt();
        assert!(
            (r - 1.0).abs() < 1.0e-9,
            "u={u:.4}: radius={r:.9} expected 1.0"
        );
    }
}

/// NURBS evaluation at knot values matches direct ellipse formula exactly.
///
/// At the knot parameter values (multiples of π/2) the rational quadratic
/// B-spline passes through the exact ellipse points. At intermediate parameters
/// the curve still lies on the ellipse but with a different angular parameter
/// (rational NURBS parameterisation ≠ true angular parameterisation).
#[test]
fn value_at_knots_matches_ellipse_formula() {
    let a = 7.0_f64;
    let b = 2.0_f64;
    let e = Elips::new(Ax3::identity(), a, b);
    let conv = EllipseToBSpline::new(&e, 0.0, 2.0 * PI, ParameterisationType::Rational);

    // At the knot values (0, π/2, π, 3π/2, 2π) the NURBS passes through the
    // exact ellipse control points.
    let test_angles = [0.0_f64, PI / 2.0, PI, 3.0 * PI / 2.0, 2.0 * PI];
    for &theta in &test_angles {
        let nurbs_pt = conv.value(theta);
        let expected = Pnt::new(a * theta.cos(), b * theta.sin(), 0.0);
        assert!(
            nurbs_pt.is_equal(expected, 1.0e-9),
            "theta={theta:.4}: NURBS={nurbs_pt:?}, expected={expected:?}"
        );
    }

    // At non-knot parameters the point still lies on the ellipse (but at a
    // different angle): verify the algebraic constraint.
    for &theta in &[PI / 6.0_f64, PI / 3.0, 2.0 * PI / 3.0, 5.0 * PI / 4.0] {
        let p = conv.value(theta);
        let residual = (p.x / a).powi(2) + (p.y / b).powi(2) - 1.0;
        assert!(
            residual.abs() < 1.0e-9,
            "theta={theta:.4}: point ({:.6},{:.6}) not on ellipse (residual={residual:e})",
            p.x,
            p.y
        );
    }
}

/// Knots vector is strictly increasing.
#[test]
fn knots_strictly_increasing() {
    let conv = EllipseToBSpline::new(&ellipse_5_3(), 0.0, 2.0 * PI, ParameterisationType::Rational);
    for i in 1..conv.nb_knots() {
        assert!(
            conv.knot(i + 1) > conv.knot(i),
            "knots not strictly increasing: knot({}) = {} >= knot({}) = {}",
            i,
            conv.knot(i),
            i + 1,
            conv.knot(i + 1)
        );
    }
}

/// An arc that does not start at 0 still has correct endpoints.
#[test]
fn offset_arc_endpoints_on_ellipse() {
    let a = 4.0_f64;
    let b = 2.0_f64;
    let e = Elips::new(Ax3::identity(), a, b);
    let alpha1 = PI / 4.0;
    let alpha2 = 3.0 * PI / 4.0;
    let conv = EllipseToBSpline::new(&e, alpha1, alpha2, ParameterisationType::Rational);

    let p_start = conv.value(alpha1);
    let p_end = conv.value(alpha2);

    let expected_start = Pnt::new(a * alpha1.cos(), b * alpha1.sin(), 0.0);
    let expected_end = Pnt::new(a * alpha2.cos(), b * alpha2.sin(), 0.0);

    assert!(
        p_start.is_equal(expected_start, 1.0e-9),
        "start: NURBS={p_start:?}, expected={expected_start:?}"
    );
    assert!(
        p_end.is_equal(expected_end, 1.0e-9),
        "end: NURBS={p_end:?}, expected={expected_end:?}"
    );
}

/// Quasiangular and Trig param types store their type correctly.
#[test]
fn param_type_stored() {
    let e = ellipse_5_3();
    let c1 = EllipseToBSpline::new(&e, 0.0, 2.0 * PI, ParameterisationType::Quasiangular);
    let c2 = EllipseToBSpline::new(&e, 0.0, 2.0 * PI, ParameterisationType::Trig);
    assert_eq!(c1.param_type(), ParameterisationType::Quasiangular);
    assert_eq!(c2.param_type(), ParameterisationType::Trig);
}

/// Out-of-range Pole index panics.
#[test]
#[should_panic]
fn pole_out_of_range_panics() {
    let conv = EllipseToBSpline::new(&ellipse_5_3(), 0.0, FRAC_PI_2, ParameterisationType::Rational);
    let _ = conv.pole(0); // index 0 is out of range (1-based)
}

/// Out-of-range Knot index panics.
#[test]
#[should_panic]
fn knot_out_of_range_panics() {
    let conv = EllipseToBSpline::new(&ellipse_5_3(), 0.0, FRAC_PI_2, ParameterisationType::Rational);
    let _ = conv.knot(10);
}

/// The first knot equals alpha1 and the last knot equals alpha2.
#[test]
fn knot_range_matches_alpha() {
    let alpha1 = 0.5_f64;
    let alpha2 = 2.5_f64;
    let conv = EllipseToBSpline::new(&ellipse_5_3(), alpha1, alpha2, ParameterisationType::Rational);
    assert!(
        (conv.knot(1) - alpha1).abs() < CONFUSION,
        "first knot should be alpha1"
    );
    assert!(
        (conv.knot(conv.nb_knots()) - alpha2).abs() < CONFUSION,
        "last knot should be alpha2"
    );
}
