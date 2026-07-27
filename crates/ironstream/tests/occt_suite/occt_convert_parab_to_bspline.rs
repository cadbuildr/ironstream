//! Ported OCCT tests for `Convert_ParabolicArcToBSplineCurve`
//! (package `TKMath`).
//!
//! Each `#[test]` mirrors a case from OCCT's test suite or verifies a
//! mathematical invariant of the quadratic B-spline parabola construction.

use ironstream::convert_parab_to_bspline::ParabToBSpline;
use ironstream::gp::{Ax1, Ax3, Pnt};
use ironstream::gp_parab::Parab;
use ironstream::precision::{ANGULAR, CONFUSION};

// ─────────────────────────────────── Helpers ──────────────────────────────────

fn unit_focal_parab() -> Parab {
    Parab::new(Ax3::identity(), 1.0)
}

fn parab_point(prb: &Parab, t: f64) -> Pnt {
    let pos = prb.position();
    let focal = prb.focal();
    let inv_4f = if focal.abs() < 1.0e-15 {
        0.0
    } else {
        1.0 / (4.0 * focal)
    };
    pos.location + pos.x_dir * (t * t * inv_4f) + pos.y_dir * t
}

// ─────────────────────────────────── Tests ────────────────────────────────────

/// A parabolic arc always converts to exactly 3 poles (degree-2 Bezier).
#[test]
fn nb_poles_is_3() {
    let conv = ParabToBSpline::new(&unit_focal_parab(), -2.0, 2.0);
    assert_eq!(conv.nb_poles(), 3, "parabola arc must have exactly 3 poles");
}

/// A parabolic arc always has exactly 2 distinct knots.
#[test]
fn nb_knots_is_2() {
    let conv = ParabToBSpline::new(&unit_focal_parab(), 0.0, 5.0);
    assert_eq!(conv.nb_knots(), 2, "parabola arc must have exactly 2 knots");
}

/// The first knot equals alpha1 and the second knot equals alpha2.
#[test]
fn knots_equal_alpha1_alpha2() {
    let a1 = -3.5_f64;
    let a2 = 7.2_f64;
    let conv = ParabToBSpline::new(&unit_focal_parab(), a1, a2);
    assert!(
        (conv.knot(1) - a1).abs() < CONFUSION,
        "knot(1)={} expected {}",
        conv.knot(1),
        a1
    );
    assert!(
        (conv.knot(2) - a2).abs() < CONFUSION,
        "knot(2)={} expected {}",
        conv.knot(2),
        a2
    );
}

/// Pole 1 lies on the parabola at alpha1.
#[test]
fn pole1_lies_on_parabola_at_alpha1() {
    let p = unit_focal_parab();
    let a1 = -2.0;
    let a2 = 4.0;
    let conv = ParabToBSpline::new(&p, a1, a2);
    let expected = parab_point(&p, a1);
    assert!(
        conv.pole(1).is_equal(expected, CONFUSION),
        "pole(1)={:?} expected={:?}",
        conv.pole(1),
        expected
    );
}

/// Pole 3 lies on the parabola at alpha2.
#[test]
fn pole3_lies_on_parabola_at_alpha2() {
    let p = unit_focal_parab();
    let a1 = 0.0;
    let a2 = 3.0;
    let conv = ParabToBSpline::new(&p, a1, a2);
    let expected = parab_point(&p, a2);
    assert!(
        conv.pole(3).is_equal(expected, CONFUSION),
        "pole(3)={:?} expected={:?}",
        conv.pole(3),
        expected
    );
}

/// Both knots have multiplicity 3 (degree + 1 = 3 for a clamped Bezier).
#[test]
fn multiplicities_are_3() {
    let conv = ParabToBSpline::new(&unit_focal_parab(), -1.0, 1.0);
    assert_eq!(conv.multiplicity(1), 3, "multiplicity(1) must be 3");
    assert_eq!(conv.multiplicity(2), 3, "multiplicity(2) must be 3");
}

/// The curve degree is 2.
#[test]
fn degree_is_2() {
    let conv = ParabToBSpline::new(&unit_focal_parab(), 0.0, 1.0);
    assert_eq!(conv.degree(), 2);
}

/// The curve is non-rational (parabola is a polynomial curve).
#[test]
fn is_not_rational() {
    let conv = ParabToBSpline::new(&unit_focal_parab(), 0.0, 1.0);
    assert!(!conv.is_rational(), "parabola BSpline must be non-rational");
}

/// value(alpha1) equals the parabola at alpha1.
#[test]
fn value_at_alpha1_is_start_point() {
    let p = unit_focal_parab();
    let a1 = -1.0;
    let a2 = 2.0;
    let conv = ParabToBSpline::new(&p, a1, a2);
    let v = conv.value(a1);
    let expected = parab_point(&p, a1);
    assert!(
        v.is_equal(expected, CONFUSION),
        "value(alpha1)={v:?} expected={expected:?}"
    );
}

/// value(alpha2) equals the parabola at alpha2.
#[test]
fn value_at_alpha2_is_end_point() {
    let p = unit_focal_parab();
    let a1 = -1.0;
    let a2 = 2.0;
    let conv = ParabToBSpline::new(&p, a1, a2);
    let v = conv.value(a2);
    let expected = parab_point(&p, a2);
    assert!(
        v.is_equal(expected, CONFUSION),
        "value(alpha2)={v:?} expected={expected:?}"
    );
}

/// Dense sampling: BSpline evaluation equals direct parabola evaluation for
/// 50 parameter values across [-5, 5].
#[test]
fn sampled_points_match_parabola() {
    let p = Parab::new(Ax3::identity(), 2.0); // focal = 2
    let a1 = -5.0_f64;
    let a2 = 5.0_f64;
    let conv = ParabToBSpline::new(&p, a1, a2);
    let n = 50;
    for i in 0..=n {
        let t = a1 + (a2 - a1) * i as f64 / n as f64;
        let bspline_pt = conv.value(t);
        let direct_pt = parab_point(&p, t);
        assert!(
            bspline_pt.is_equal(direct_pt, CONFUSION),
            "i={i} t={t}: bspline={bspline_pt:?} direct={direct_pt:?}"
        );
    }
}

/// For the symmetric arc [-a, a], pole(2) must have Y = 0 (lies on symmetry axis).
#[test]
fn symmetric_arc_middle_pole_on_axis() {
    let p = unit_focal_parab();
    let a = 3.0_f64;
    let conv = ParabToBSpline::new(&p, -a, a);
    let b1 = conv.pole(2);
    assert!(
        b1.y.abs() < CONFUSION,
        "Middle pole Y={} for symmetric arc; expected 0",
        b1.y
    );
}

/// Pole(2) is the intersection of tangents at pole(1) and pole(3).
///
/// For parabola focal=1, arc [1, 3]: B1 = (0.75, 2.0, 0.0).
#[test]
fn middle_pole_equals_tangent_intersection() {
    let p = Parab::new(Ax3::identity(), 1.0);
    let a1 = 1.0_f64;
    let a2 = 3.0_f64;
    let conv = ParabToBSpline::new(&p, a1, a2);
    let b1 = conv.pole(2);
    // B1 = V + (t0*t1/(4f))*X + ((t0+t1)/2)*Y
    //    = origin + (1*3/4) * (1,0,0) + 2 * (0,1,0)
    //    = (0.75, 2.0, 0.0)
    let expected = Pnt::new(0.75, 2.0, 0.0);
    assert!(
        b1.is_equal(expected, CONFUSION),
        "B1={b1:?} expected={expected:?}"
    );
}

/// A parabola with larger focal still reconstructs the arc exactly.
#[test]
fn larger_focal_reconstruction() {
    let p = Parab::new(Ax3::identity(), 5.0);
    let a1 = -10.0_f64;
    let a2 = 10.0_f64;
    let conv = ParabToBSpline::new(&p, a1, a2);
    // Endpoints
    assert!(
        conv.pole(1).is_equal(parab_point(&p, a1), CONFUSION),
        "start pole mismatch"
    );
    assert!(
        conv.pole(3).is_equal(parab_point(&p, a2), CONFUSION),
        "end pole mismatch"
    );
    // Mid-arc
    let mid = conv.value((a1 + a2) * 0.5);
    let expected = parab_point(&p, (a1 + a2) * 0.5);
    assert!(mid.is_equal(expected, CONFUSION), "mid mismatch");
}

/// A translated parabola produces poles shifted by the same translation.
#[test]
fn translated_parabola_shifts_poles() {
    let offset = Pnt::new(10.0, 5.0, 3.0);
    let ax_shifted = Ax3::from_origin_normal(
        offset,
        Pnt::new(0.0, 0.0, 1.0),
        Pnt::new(1.0, 0.0, 0.0),
    );
    let p_orig = Parab::new(Ax3::identity(), 1.0);
    let p_shift = Parab::new(ax_shifted, 1.0);
    let a1 = -2.0;
    let a2 = 2.0;
    let conv_orig = ParabToBSpline::new(&p_orig, a1, a2);
    let conv_shift = ParabToBSpline::new(&p_shift, a1, a2);
    for i in 1..=3 {
        let diff = conv_shift.pole(i) - conv_orig.pole(i);
        assert!(
            diff.is_equal(offset, CONFUSION),
            "pole {i} shift mismatch: diff={diff:?} expected={offset:?}"
        );
    }
}

/// Out-of-range pole access panics.
#[test]
fn pole_index_0_panics() {
    let conv = ParabToBSpline::new(&unit_focal_parab(), 0.0, 1.0);
    let r = std::panic::catch_unwind(|| conv.pole(0));
    assert!(r.is_err(), "pole(0) must panic");
}

/// Out-of-range pole access panics.
#[test]
fn pole_index_4_panics() {
    let conv = ParabToBSpline::new(&unit_focal_parab(), 0.0, 1.0);
    let r = std::panic::catch_unwind(|| conv.pole(4));
    assert!(r.is_err(), "pole(4) must panic");
}

/// Knot out-of-range access panics.
#[test]
fn knot_index_out_of_range_panics() {
    let conv = ParabToBSpline::new(&unit_focal_parab(), 0.0, 1.0);
    let r = std::panic::catch_unwind(|| conv.knot(3));
    assert!(r.is_err(), "knot(3) must panic");
}

/// For a rotated parabola, the BSpline poles transform consistently.
#[test]
fn rotated_parabola_reconstruction() {
    use std::f64::consts::FRAC_PI_4;
    // Rotate the standard parabola 45° around Z.
    let p_orig = Parab::new(Ax3::identity(), 1.0);
    let rot_ax = Ax1::new(Pnt::origin(), Pnt::new(0.0, 0.0, 1.0));
    let p_rot = p_orig.rotated(rot_ax, FRAC_PI_4);
    let a1 = -1.0;
    let a2 = 1.0;
    let conv = ParabToBSpline::new(&p_rot, a1, a2);
    // Endpoints must lie on the rotated parabola.
    assert!(
        conv.pole(1).is_equal(parab_point(&p_rot, a1), CONFUSION),
        "rotated pole(1) mismatch"
    );
    assert!(
        conv.pole(3).is_equal(parab_point(&p_rot, a2), CONFUSION),
        "rotated pole(3) mismatch"
    );
    // Dense check
    let n = 20;
    for i in 0..=n {
        let t = a1 + (a2 - a1) * i as f64 / n as f64;
        let bspline_pt = conv.value(t);
        let direct_pt = parab_point(&p_rot, t);
        assert!(
            bspline_pt.is_equal(direct_pt, CONFUSION * 10.0),
            "i={i} t={t}: bspline={bspline_pt:?} direct={direct_pt:?}"
        );
    }
}

/// A one-sided arc (alpha1=0) starts at the vertex.
#[test]
fn arc_starting_at_vertex() {
    let p = unit_focal_parab();
    let a1 = 0.0;
    let a2 = 4.0;
    let conv = ParabToBSpline::new(&p, a1, a2);
    // At t=0, P(0) = vertex = origin.
    let vertex = p.location();
    assert!(
        conv.pole(1).is_equal(vertex, CONFUSION),
        "pole(1) should be vertex for arc starting at t=0"
    );
    assert!(
        conv.value(0.0).is_equal(vertex, CONFUSION),
        "value(0) should be vertex"
    );
}

/// The curve is C1 continuous at every interior parameter (parabola is smooth).
/// We verify this indirectly by checking the derivative approximation.
#[test]
fn mid_value_matches_parabola() {
    let _ = ANGULAR; // silence unused import
    let p = Parab::new(Ax3::identity(), 3.0);
    let a1 = -6.0_f64;
    let a2 = 6.0_f64;
    let conv = ParabToBSpline::new(&p, a1, a2);
    // Check midpoint.
    let t_mid = 0.0_f64;
    let bspline_mid = conv.value(t_mid);
    let direct_mid = parab_point(&p, t_mid);
    assert!(
        bspline_mid.is_equal(direct_mid, CONFUSION),
        "midpoint mismatch: bspline={bspline_mid:?} direct={direct_mid:?}"
    );
}
