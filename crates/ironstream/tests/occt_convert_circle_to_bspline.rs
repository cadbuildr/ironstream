//! OCCT-faithful tests for `Convert_CircleToBSplineCurve`.
//!
//! Verifies that the rational quadratic NURBS produced by
//! `CircleToBSpline` exactly matches the expected properties from OCCT:
//! - quarter-circle middle weight = cos(π/4) = 1/√2
//! - all evaluated points lie on the original circle to `precision::CONFUSION`
//! - structural invariants (nb_poles, nb_knots, multiplicities, degree)
//! - 1-based accessor indexing
//! - various arc sizes and convert types

use ironstream::convert_circle_to_bspline::{CircleToBSpline, ConvertType};
use ironstream::gp::{Ax3, Pnt};
use ironstream::gp_prim::Circ;
use ironstream::precision;
use std::f64::consts::{FRAC_PI_2, PI};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Unit circle in the standard XY plane.
fn unit_circ() -> Circ {
    Circ::new(Ax3::identity(), 1.0)
}

/// Circle of radius `r` in the standard XY plane.
fn circ(r: f64) -> Circ {
    Circ::new(Ax3::identity(), r)
}

/// Euclidean distance from a point to the origin (i.e. its radius in XY).
fn xy_radius(p: Pnt) -> f64 {
    (p.x * p.x + p.y * p.y).sqrt()
}

// ---------------------------------------------------------------------------
// 1.  Quarter-circle: weight of middle pole = 1/√2 = cos(π/4)
// ---------------------------------------------------------------------------
#[test]
fn quarter_circle_middle_weight_equals_inv_sqrt2() {
    let c = CircleToBSpline::from_arc(&unit_circ(), 0.0, FRAC_PI_2, ConvertType::TgtThetaOver2);
    let expected = 1.0_f64 / 2.0_f64.sqrt(); // cos(π/4)
    let w_mid = c.weight(2);
    assert!(
        (w_mid - expected).abs() < precision::ANGULAR,
        "quarter-circle w_mid={w_mid:.15} expected {expected:.15}"
    );
}

// ---------------------------------------------------------------------------
// 2.  Quarter-circle: endpoint weights are exactly 1.0
// ---------------------------------------------------------------------------
#[test]
fn quarter_circle_endpoint_weights_are_one() {
    let c = CircleToBSpline::from_arc(&unit_circ(), 0.0, FRAC_PI_2, ConvertType::TgtThetaOver2);
    assert_eq!(c.weight(1), 1.0, "first pole weight");
    assert_eq!(c.weight(3), 1.0, "last pole weight");
}

// ---------------------------------------------------------------------------
// 3.  Quarter-circle structural invariants
// ---------------------------------------------------------------------------
#[test]
fn quarter_circle_structure() {
    let c = CircleToBSpline::from_arc(&unit_circ(), 0.0, FRAC_PI_2, ConvertType::TgtThetaOver2);
    assert_eq!(c.degree(), 2, "degree");
    assert_eq!(c.nb_poles(), 3, "nb_poles");
    assert_eq!(c.nb_knots(), 2, "nb_knots");
    assert_eq!(c.multiplicity(1), 3, "start multiplicity");
    assert_eq!(c.multiplicity(2), 3, "end multiplicity");
}

// ---------------------------------------------------------------------------
// 4.  Half-circle structural invariants
// ---------------------------------------------------------------------------
#[test]
fn half_circle_structure() {
    let c = CircleToBSpline::from_arc(&unit_circ(), 0.0, PI, ConvertType::TgtThetaOver2);
    // 2 Bézier spans → 5 poles, 3 distinct knots (mults 3, 2, 3)
    assert_eq!(c.nb_poles(), 5, "nb_poles");
    assert_eq!(c.nb_knots(), 3, "nb_knots");
    assert_eq!(c.multiplicity(1), 3, "start mult");
    assert_eq!(c.multiplicity(2), 2, "internal mult");
    assert_eq!(c.multiplicity(3), 3, "end mult");
}

// ---------------------------------------------------------------------------
// 5.  Full-circle structural invariants
// ---------------------------------------------------------------------------
#[test]
fn full_circle_structure() {
    let c = CircleToBSpline::from_circle(&unit_circ(), ConvertType::TgtThetaOver2);
    // 4 Bézier spans → 9 poles, 5 distinct knots
    assert_eq!(c.nb_poles(), 9, "nb_poles");
    assert_eq!(c.nb_knots(), 5, "nb_knots");
    assert_eq!(c.degree(), 2, "degree");
}

// ---------------------------------------------------------------------------
// 6.  NURBS evaluates on the circle — quarter arc, radius 1
// ---------------------------------------------------------------------------
#[test]
fn quarter_circle_evaluates_on_unit_circle() {
    let r = 1.0;
    let c = CircleToBSpline::from_arc(&circ(r), 0.0, FRAC_PI_2, ConvertType::TgtThetaOver2);
    for i in 0..=20 {
        let u = FRAC_PI_2 * i as f64 / 20.0;
        let p = c.value(u);
        let rad = xy_radius(p);
        assert!(
            (rad - r).abs() < precision::CONFUSION,
            "u={u:.4}: radius={rad:.10} expected {r}"
        );
    }
}

// ---------------------------------------------------------------------------
// 7.  NURBS evaluates on the circle — full circle, radius 5
// ---------------------------------------------------------------------------
#[test]
fn full_circle_evaluates_on_circle() {
    let r = 5.0;
    let c = CircleToBSpline::from_circle(&circ(r), ConvertType::TgtThetaOver2);
    for i in 0..=40 {
        let u = 2.0 * PI * i as f64 / 40.0;
        let p = c.value(u);
        let rad = xy_radius(p);
        assert!(
            (rad - r).abs() < precision::CONFUSION,
            "u={u:.4}: radius={rad:.10} expected {r}"
        );
    }
}

// ---------------------------------------------------------------------------
// 8.  NURBS evaluates on the circle — half circle, RationalC1
// ---------------------------------------------------------------------------
#[test]
fn half_circle_evaluates_on_circle() {
    let r = 3.0;
    let c = CircleToBSpline::from_arc(&circ(r), 0.0, PI, ConvertType::RationalC1);
    for i in 0..=20 {
        let u = PI * i as f64 / 20.0;
        let p = c.value(u);
        let rad = xy_radius(p);
        assert!(
            (rad - r).abs() < precision::CONFUSION,
            "u={u:.4}: radius={rad:.10} expected {r}"
        );
    }
}

// ---------------------------------------------------------------------------
// 9.  Endpoint poles interpolate the circle exactly
// ---------------------------------------------------------------------------
#[test]
fn endpoints_interpolate_circle_exactly() {
    let r = 2.0;
    let a1 = PI / 4.0;
    let a2 = a1 + FRAC_PI_2;
    let c = CircleToBSpline::from_arc(&circ(r), a1, a2, ConvertType::TgtThetaOver2);

    let expected_start = Pnt::new(r * a1.cos(), r * a1.sin(), 0.0);
    let expected_end = Pnt::new(r * a2.cos(), r * a2.sin(), 0.0);

    let p_start = c.pole(1);
    let p_end = c.pole(c.nb_poles());

    assert!(
        p_start.distance(expected_start) < precision::CONFUSION,
        "start pole: {p_start:?} vs {expected_start:?}"
    );
    assert!(
        p_end.distance(expected_end) < precision::CONFUSION,
        "end pole: {p_end:?} vs {expected_end:?}"
    );
}

// ---------------------------------------------------------------------------
// 10. Three-quarter-circle structure (QuasiAngular type)
// ---------------------------------------------------------------------------
#[test]
fn three_quarter_circle_structure() {
    let c = CircleToBSpline::from_arc(
        &unit_circ(),
        0.0,
        3.0 * FRAC_PI_2,
        ConvertType::QuasiAngular,
    );
    // 3 Bézier spans → 7 poles, 4 distinct knots
    assert_eq!(c.nb_poles(), 7, "nb_poles");
    assert_eq!(c.nb_knots(), 4, "nb_knots");
}

// ---------------------------------------------------------------------------
// 11. Three-quarter-circle evaluates on circle
// ---------------------------------------------------------------------------
#[test]
fn three_quarter_circle_evaluates_on_circle() {
    let r = 7.0;
    let arc = 3.0 * FRAC_PI_2;
    let c = CircleToBSpline::from_arc(&circ(r), 0.0, arc, ConvertType::QuasiAngular);
    for i in 0..=30 {
        let u = arc * i as f64 / 30.0;
        let p = c.value(u);
        let rad = xy_radius(p);
        assert!(
            (rad - r).abs() < precision::CONFUSION,
            "u={u:.4}: radius={rad:.10} expected {r}"
        );
    }
}

// ---------------------------------------------------------------------------
// 12. Knot range spans the arc angles
// ---------------------------------------------------------------------------
#[test]
fn knot_range_matches_arc_angles() {
    let a1 = PI / 3.0;
    let a2 = a1 + PI;
    let c = CircleToBSpline::from_arc(&unit_circ(), a1, a2, ConvertType::TgtThetaOver2);
    assert!(
        (c.knot(1) - a1).abs() < precision::CONFUSION,
        "first knot {} vs {}",
        c.knot(1),
        a1
    );
    assert!(
        (c.knot(c.nb_knots()) - a2).abs() < precision::CONFUSION,
        "last knot {} vs {}",
        c.knot(c.nb_knots()),
        a2
    );
}
