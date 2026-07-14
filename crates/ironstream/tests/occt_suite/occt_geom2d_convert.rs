// FILE: tests/occt_geom2d_convert.rs
//! Integration tests for `geom2d_convert` — mirrors the OCCT
//! `Geom2dConvert` package test suite.

extern crate ironstream;

use ironstream::geom2d_bspline::Geom2dBSplineCurve;
use ironstream::geom2d_convert::{
    Geom2dConvert, Geom2dConvert_BSplineCurveToBezierCurve,
    Geom2dConvert_CompCurveToBSplineCurve,
};
use ironstream::gp2d::Pnt2d;

// ---------------------------------------------------------------------------
// Test helpers
// ---------------------------------------------------------------------------

/// Cubic (degree-3) single-span B-spline in 2D.
fn cubic_2d() -> Geom2dBSplineCurve {
    let poles = [
        Pnt2d::new(0.0, 0.0),
        Pnt2d::new(1.0, 3.0),
        Pnt2d::new(2.0, 3.0),
        Pnt2d::new(3.0, 0.0),
    ];
    let knots = [0.0f64, 1.0];
    let mults = [4usize, 4];
    Geom2dBSplineCurve::new(&poles, &knots, &mults, 3, false)
}

/// Quadratic (degree-2) two-span B-spline in 2D.
/// knots {0,0.5,1} mults {3,2,3}: n_poles = 8 - 2 - 1 = 5.
fn quad_two_span_2d() -> Geom2dBSplineCurve {
    let poles = [
        Pnt2d::new(0.0, 0.0),
        Pnt2d::new(0.75, 2.0),
        Pnt2d::new(1.5, 2.5),
        Pnt2d::new(2.25, 2.0),
        Pnt2d::new(3.0, 0.0),
    ];
    let knots = [0.0f64, 0.5, 1.0];
    let mults = [3usize, 2, 3];
    Geom2dBSplineCurve::new(&poles, &knots, &mults, 2, false)
}

/// Linear (degree-1) B-spline between two points.
fn line_2d(p1: Pnt2d, p2: Pnt2d) -> Geom2dBSplineCurve {
    Geom2dBSplineCurve::new(&[p1, p2], &[0.0f64, 1.0], &[2usize, 2], 1, false)
}

/// Three-span linear B-spline along x-axis: (0,0)→(1,0)→(2,0)→(3,0).
fn three_span_2d() -> [Geom2dBSplineCurve; 3] {
    [
        line_2d(Pnt2d::new(0.0, 0.0), Pnt2d::new(1.0, 0.0)),
        line_2d(Pnt2d::new(1.0, 0.0), Pnt2d::new(2.0, 0.0)),
        line_2d(Pnt2d::new(2.0, 0.0), Pnt2d::new(3.0, 0.0)),
    ]
}

// ---------------------------------------------------------------------------
// Geom2dConvert static helpers
// ---------------------------------------------------------------------------

#[test]
fn test_curve_to_bspline_is_identity_clone() {
    let c = cubic_2d();
    let c2 = Geom2dConvert::curve_to_bspline(&c);
    assert_eq!(c.degree(), c2.degree());
    assert_eq!(c.nb_poles(), c2.nb_poles());
    // Check geometry at several parameter values.
    for &u in &[0.0, 0.2, 0.5, 0.8, 1.0] {
        let a = c.value(u);
        let b = c2.value(u);
        assert!(
            a.distance(b) < 1.0e-12,
            "geometry differs at u={u}: a={a:?} b={b:?}"
        );
    }
}

#[test]
fn test_point_to_point_builds_linear_segment() {
    let p1 = Pnt2d::new(-1.0, 3.0);
    let p2 = Pnt2d::new(5.0, 7.0);
    let seg = Geom2dConvert::point_to_point(p1, p2);
    assert_eq!(seg.degree(), 1, "point_to_point must be degree 1");
    assert_eq!(seg.nb_poles(), 2);
    assert!(seg.start_point().distance(p1) < 1.0e-12);
    assert!(seg.end_point().distance(p2) < 1.0e-12);
    // Mid-point should be the average.
    let mid = seg.value(0.5);
    let expected = Pnt2d::new((p1.x + p2.x) * 0.5, (p1.y + p2.y) * 0.5);
    assert!(mid.distance(expected) < 1.0e-10);
}

#[test]
fn test_concat_g1_single_returns_clone() {
    let c = cubic_2d();
    let result = Geom2dConvert::concat_g1(&[c.clone()]).unwrap();
    assert_eq!(result.degree(), c.degree());
    assert_eq!(result.nb_poles(), c.nb_poles());
}

#[test]
fn test_concat_g1_two_linear_segments() {
    let segs = three_span_2d();
    let two = Geom2dConvert::concat_g1(&segs[..2]);
    assert!(two.is_some(), "concat_g1 of two connected segs must succeed");
}

// ---------------------------------------------------------------------------
// Geom2dConvert_BSplineCurveToBezierCurve
// ---------------------------------------------------------------------------

#[test]
fn test_to_bezier_cubic_single_arc() {
    let c = cubic_2d();
    let conv = Geom2dConvert_BSplineCurveToBezierCurve::new(&c);
    assert_eq!(conv.nb_arcs(), 1, "cubic single-span -> exactly 1 arc");
}

#[test]
fn test_to_bezier_quadratic_two_arcs() {
    let c = quad_two_span_2d();
    let conv = Geom2dConvert_BSplineCurveToBezierCurve::new(&c);
    assert_eq!(conv.nb_arcs(), 2, "two-span quadratic -> exactly 2 arcs");
}

#[test]
fn test_to_bezier_knots_length() {
    let c = quad_two_span_2d();
    let conv = Geom2dConvert_BSplineCurveToBezierCurve::new(&c);
    assert_eq!(
        conv.knots().len(),
        conv.nb_arcs() + 1,
        "knots array length must be NbArcs + 1"
    );
}

#[test]
fn test_to_bezier_arc_degree_preserved() {
    let c = cubic_2d();
    let conv = Geom2dConvert_BSplineCurveToBezierCurve::new(&c);
    let arc = conv.arc(1);
    assert_eq!(
        arc.degree(),
        c.degree(),
        "Bezier arc degree must match B-spline degree"
    );
}

#[test]
fn test_to_bezier_arc_endpoints_match_bspline() {
    let c = cubic_2d();
    let conv = Geom2dConvert_BSplineCurveToBezierCurve::new(&c);
    let arc = conv.arc(1);
    assert!(
        arc.start_point().distance(c.start_point()) < 1.0e-8,
        "arc start should match B-spline start"
    );
    assert!(
        arc.end_point().distance(c.end_point()) < 1.0e-8,
        "arc end should match B-spline end"
    );
}

#[test]
fn test_to_bezier_sub_range_single_arc() {
    let c = quad_two_span_2d();
    // Request only the first span [0, 0.5].
    let conv =
        Geom2dConvert_BSplineCurveToBezierCurve::new_with_params(&c, 0.0, 0.5, 1.0e-7);
    assert_eq!(
        conv.nb_arcs(),
        1,
        "sub-range [0,0.5] of two-span should yield 1 arc"
    );
    let arc = conv.arc(1);
    assert!(
        arc.start_point().distance(c.start_point()) < 1.0e-8,
        "arc start should match B-spline start"
    );
}

// ---------------------------------------------------------------------------
// Geom2dConvert_CompCurveToBSplineCurve
// ---------------------------------------------------------------------------

#[test]
fn test_comp_curve_single_segment_roundtrip() {
    let c = cubic_2d();
    let builder = Geom2dConvert_CompCurveToBSplineCurve::new(&c);
    let result = builder.bspline_curve().expect("single segment must produce a curve");
    assert_eq!(result.degree(), c.degree());
    assert_eq!(result.nb_poles(), c.nb_poles());
    // Geometry preserved.
    for &u in &[0.0, 0.25, 0.5, 0.75, 1.0] {
        let a = c.value(u);
        let b = result.value(u);
        assert!(a.distance(b) < 1.0e-9, "geometry mismatch at u={u}");
    }
}

#[test]
fn test_comp_curve_add_disjoint_returns_false() {
    let c = cubic_2d();
    let disjoint = Geom2dBSplineCurve::new(
        &[
            Pnt2d::new(50.0, 50.0),
            Pnt2d::new(51.0, 50.0),
            Pnt2d::new(52.0, 50.0),
            Pnt2d::new(53.0, 50.0),
        ],
        &[0.0f64, 1.0],
        &[4usize, 4],
        3,
        false,
    );
    let mut builder = Geom2dConvert_CompCurveToBSplineCurve::new(&c);
    assert!(
        !builder.add(&disjoint, 1.0e-7),
        "add of a disjoint curve must return false"
    );
}

#[test]
fn test_comp_curve_two_linear_segments_produces_three_poles() {
    let seg1 = line_2d(Pnt2d::new(0.0, 0.0), Pnt2d::new(1.0, 0.0));
    let seg2 = line_2d(Pnt2d::new(1.0, 0.0), Pnt2d::new(2.0, 0.0));
    let mut builder = Geom2dConvert_CompCurveToBSplineCurve::new(&seg1);
    let ok = builder.add(&seg2, 1.0e-7);
    assert!(ok, "connected segments should produce true");
    // 3 distinct poles: (0,0), (1,0), (2,0).
    assert_eq!(builder.nb_poles(), 3);
    assert_eq!(builder.degree(), 1);
}
