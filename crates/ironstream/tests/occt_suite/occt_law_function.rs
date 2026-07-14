// FILE: tests/occt_law_function.rs
//! Integration tests for `ironstream::law_function` — Law_Function and
//! evolution laws.
//!
//! Mappings:
//! - `Law_Function`  → [`ironstream::law_function::LawFunction`] (trait)
//! - `Law_Linear`    → [`ironstream::law_function::LawLinear`]
//! - `Law_Interpol`  → [`ironstream::law_function::LawInterpol`]
//! - `Law_BSpline`   → [`ironstream::law_function::LawBSpline`]
//! - `Law_Composite` → [`ironstream::law_function::LawComposite`]

extern crate ironstream;

use ironstream::law_function::{
    LawBSpline, LawComposite, LawFunction, LawInterpol, LawLinear,
};

// ─────────────────────────────────────────────────────────────────────────────
// Helpers
// ─────────────────────────────────────────────────────────────────────────────

fn near(a: f64, b: f64, tol: f64, msg: &str) {
    assert!(
        (a - b).abs() <= tol,
        "{msg}: |{a} - {b}| = {} > tol {tol}",
        (a - b).abs()
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// LawLinear integration tests
// ─────────────────────────────────────────────────────────────────────────────

/// Law_Linear: a unit ramp must have slope 1 and pass through start/end exactly.
#[test]
fn law_linear_unit_ramp() {
    let law = LawLinear::new(0.0, 0.0, 1.0, 1.0);
    near(law.value(0.0), 0.0, 1e-12, "v(0)");
    near(law.value(0.25), 0.25, 1e-12, "v(0.25)");
    near(law.value(0.75), 0.75, 1e-12, "v(0.75)");
    near(law.value(1.0), 1.0, 1e-12, "v(1)");
    near(law.d1(0.5), 1.0, 1e-12, "slope = 1");
}

/// Law_Linear: a descending ramp must have negative slope.
#[test]
fn law_linear_descending() {
    let law = LawLinear::new(0.0, 5.0, 2.0, 1.0);
    near(law.value(0.0), 5.0, 1e-12, "start = 5");
    near(law.value(2.0), 1.0, 1e-12, "end = 1");
    near(law.d1(1.0), -2.0, 1e-12, "slope = -2");
    // Midpoint
    near(law.value(1.0), 3.0, 1e-12, "midpoint = 3");
}

/// Law_Linear: parameter bounds are correctly reported.
#[test]
fn law_linear_parameter_bounds() {
    let law = LawLinear::new(-3.0, 1.0, 7.0, 4.0);
    assert_eq!(law.first_parameter(), -3.0);
    assert_eq!(law.last_parameter(), 7.0);
    let (first, last) = law.bounds();
    assert_eq!(first, -3.0);
    assert_eq!(last, 7.0);
}

/// Law_Linear: value_and_d1 must agree with separate value / d1 calls.
#[test]
fn law_linear_value_and_d1_agree() {
    let law = LawLinear::new(1.0, 2.0, 5.0, 8.0);
    for &t in &[1.0, 2.5, 3.7, 5.0] {
        let (v, d) = law.value_and_d1(t);
        near(v, law.value(t), 1e-14, "value agree");
        near(d, law.d1(t), 1e-14, "d1 agree");
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// LawInterpol integration tests
// ─────────────────────────────────────────────────────────────────────────────

/// Law_Interpol: the spline passes exactly through all control points.
#[test]
fn law_interpol_passes_through_knots() {
    let params = [0.0, 0.5, 1.0, 1.5, 2.0];
    let values = [0.0, 1.0, 0.5, 1.5, 1.0];
    let law = LawInterpol::new(&params, &values, false).unwrap();
    for i in 0..params.len() {
        near(
            law.value(params[i]),
            values[i],
            1e-9,
            &format!("knot {i}"),
        );
    }
}

/// Law_Interpol: for linearly-spaced data along a line, the spline is linear.
#[test]
fn law_interpol_linear_data() {
    let params = [0.0, 1.0, 2.0, 3.0];
    let values = [1.0, 3.0, 5.0, 7.0]; // v = 1 + 2t
    let law = LawInterpol::new(&params, &values, false).unwrap();
    // Interior evaluation should be nearly linear.
    near(law.value(0.5), 2.0, 1e-9, "v(0.5)=2");
    near(law.value(1.5), 4.0, 1e-9, "v(1.5)=4");
    near(law.value(2.5), 6.0, 1e-9, "v(2.5)=6");
}

/// Law_Interpol: derivative is consistent with finite differences at interior
/// points that are NOT on a knot boundary (natural cubic spline derivative
/// is continuous within each segment; at knots the one-sided derivatives can
/// differ for the finite-difference stencil when h straddles a knot).
#[test]
fn law_interpol_derivative_consistency() {
    let params = [0.0, 1.0, 2.0, 3.0, 4.0];
    let values = [0.0, 1.0, 0.0, 1.0, 0.0];
    let law = LawInterpol::new(&params, &values, false).unwrap();
    let h = 1e-5f64;
    // Only test at strictly interior non-knot parameter values.
    for &t in &[0.5, 1.5, 2.5, 3.5] {
        let fd = (law.value(t + h) - law.value(t - h)) / (2.0 * h);
        near(law.d1(t), fd, 1e-3, &format!("fd at {t}"));
    }
}

/// Law_Interpol: metadata (knot count, accessor) round-trips correctly.
#[test]
fn law_interpol_metadata() {
    let params = [0.0, 1.0, 2.0];
    let values = [3.0, 5.0, 2.0];
    let law = LawInterpol::new(&params, &values, false).unwrap();
    assert_eq!(law.nb_knots(), 3);
    near(law.knot(0), 0.0, 1e-14, "knot[0]");
    near(law.knot(2), 2.0, 1e-14, "knot[2]");
    near(law.knot_value(1), 5.0, 1e-14, "value[1]");
}

// ─────────────────────────────────────────────────────────────────────────────
// LawBSpline integration tests
// ─────────────────────────────────────────────────────────────────────────────

/// Law_BSpline: a degree-1 spline with 2 poles is a linear ramp.
#[test]
fn law_bspline_degree1_linear_ramp() {
    let poles = [2.0, 8.0];
    let knots = [0.0, 1.0];
    let mults = [2usize, 2usize];
    let law = LawBSpline::new(&poles, &knots, &mults, 1);
    near(law.value(0.0), 2.0, 1e-12, "v(0)=2");
    near(law.value(1.0), 8.0, 1e-12, "v(1)=8");
    near(law.value(0.5), 5.0, 1e-12, "v(0.5)=5");
}

/// Law_BSpline: a degree-2 Bezier with symmetric poles must peak at the midpoint.
#[test]
fn law_bspline_quadratic_bezier_peak() {
    let poles = [0.0, 2.0, 0.0];
    let knots = [0.0, 1.0];
    let mults = [3usize, 3usize];
    let law = LawBSpline::new(&poles, &knots, &mults, 2);
    // Symmetric: value at 0 and 1 both zero; peak at 0.5.
    near(law.value(0.0), 0.0, 1e-12, "v(0)=0");
    near(law.value(1.0), 0.0, 1e-12, "v(1)=0");
    // Quadratic Bezier: B(0.5) = (1-0.5)^2*0 + 2*(1-0.5)*0.5*2 + 0.5^2*0 = 1.
    near(law.value(0.5), 1.0, 1e-10, "peak = 1");
}

/// Law_BSpline: nb_poles and pole accessor.
#[test]
fn law_bspline_nb_poles_and_accessor() {
    let poles = [1.0, 2.0, 3.0, 4.0];
    let knots = [0.0, 1.0];
    let mults = [4usize, 4usize];
    let law = LawBSpline::new(&poles, &knots, &mults, 3);
    assert_eq!(law.nb_poles(), 4);
    near(law.pole(0), 1.0, 1e-14, "pole[0]");
    near(law.pole(3), 4.0, 1e-14, "pole[3]");
}

// ─────────────────────────────────────────────────────────────────────────────
// LawComposite integration tests
// ─────────────────────────────────────────────────────────────────────────────

/// Law_Composite: a composite of three linear segments stitches correctly.
#[test]
fn law_composite_three_segments() {
    let mut comp = LawComposite::new();
    // Segment 1: [0,1], v: 0->1
    comp.add_segment(0.0, 1.0, Box::new(LawLinear::new(0.0, 0.0, 1.0, 1.0)));
    // Segment 2: [1,2], v: 1->1 (constant)
    comp.add_segment(1.0, 2.0, Box::new(LawLinear::new(1.0, 1.0, 2.0, 1.0)));
    // Segment 3: [2,3], v: 1->2
    comp.add_segment(2.0, 3.0, Box::new(LawLinear::new(2.0, 1.0, 3.0, 2.0)));

    near(comp.value(0.0), 0.0, 1e-12, "start");
    near(comp.value(0.5), 0.5, 1e-12, "mid seg 1");
    near(comp.value(1.5), 1.0, 1e-12, "mid seg 2 (constant)");
    near(comp.value(2.5), 1.5, 1e-12, "mid seg 3");
    near(comp.value(3.0), 2.0, 1e-12, "end");
}

/// Law_Composite: mixing a linear and an interpolation law.
#[test]
fn law_composite_mixed_laws() {
    let linear_part = LawLinear::new(0.0, 0.0, 1.0, 1.0);
    let interp_part =
        LawInterpol::new(&[1.0, 1.5, 2.0], &[1.0, 2.0, 1.0], false).unwrap();

    let mut comp = LawComposite::new();
    comp.add_segment(0.0, 1.0, Box::new(linear_part));
    comp.add_segment(1.0, 2.0, Box::new(interp_part));

    near(comp.value(0.0), 0.0, 1e-12, "start of linear");
    near(comp.value(1.0), 1.0, 1e-9, "junction");
    near(comp.value(2.0), 1.0, 1e-9, "end of interpol");
    // At parameter 1.5 the interpolation spline passes through (1.5, 2.0).
    near(comp.value(1.5), 2.0, 1e-9, "interpol knot at 1.5");
}

/// Law_Composite: nb_intervals and break_points metadata.
#[test]
fn law_composite_metadata() {
    let mut comp = LawComposite::new();
    comp.add_segment(0.0, 2.0, Box::new(LawLinear::new(0.0, 1.0, 2.0, 3.0)));
    comp.add_segment(2.0, 5.0, Box::new(LawLinear::new(2.0, 3.0, 5.0, 9.0)));

    assert_eq!(comp.nb_intervals(), 2);
    assert_eq!(comp.break_points(), &[0.0, 2.0, 5.0]);
    assert_eq!(comp.first_parameter(), 0.0);
    assert_eq!(comp.last_parameter(), 5.0);
}
