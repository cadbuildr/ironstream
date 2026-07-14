// FILE: tests/occt_geom_convert.rs
//! Integration tests for `GeomConvert` / `GeomConvert_BSplineCurveToBezierCurve` /
//! `GeomConvert_BSplineSurfaceToBezierSurface` / `GeomConvert_CompCurveToBSplineCurve`.
//!
//! These tests mirror the OCCT behaviour described in the `GeomConvert` package
//! documentation and test suite.  Each test is self-contained and exercises the
//! public API from outside the crate.

extern crate ironstream;

use ironstream::geom_bspline_curve::GeomBSplineCurve;
use ironstream::geom_bspline_surface::GeomBSplineSurface;
use ironstream::geom_convert::{
    GeomConvert, GeomConvert_BSplineCurveToBezierCurve, GeomConvert_BSplineSurfaceToBezierSurface,
    GeomConvert_CompCurveToBSplineCurve,
};
use ironstream::gp::Pnt;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Single-span cubic Bezier B-spline, knots {0,1}, mults {4,4}, 4 poles.
fn cubic_bezier_bspline() -> GeomBSplineCurve {
    let poles = [
        Pnt::new(0.0, 0.0, 0.0),
        Pnt::new(1.0, 2.0, 0.0),
        Pnt::new(2.0, 2.0, 0.0),
        Pnt::new(3.0, 0.0, 0.0),
    ];
    GeomBSplineCurve::new(&poles, &[0.0, 1.0], &[4, 4], 3, false)
}

/// Two-span quadratic B-spline, knots {0, 0.5, 1}, mults {3, 2, 3}, 5 poles.
fn two_span_quadratic_bspline() -> GeomBSplineCurve {
    let poles = [
        Pnt::new(0.0, 0.0, 0.0),
        Pnt::new(0.75, 2.0, 0.0),
        Pnt::new(1.5, 1.0, 0.0),
        Pnt::new(2.25, 2.0, 0.0),
        Pnt::new(3.0, 0.0, 0.0),
    ];
    GeomBSplineCurve::new(&poles, &[0.0, 0.5, 1.0], &[3, 2, 3], 2, false)
}

/// Bilinear (degree 1 x 1) B-spline surface over [0,1]^2, 2x2 poles.
fn unit_bilinear_surface() -> GeomBSplineSurface {
    let poles = vec![
        vec![Pnt::new(0.0, 0.0, 0.0), Pnt::new(0.0, 1.0, 0.0)],
        vec![Pnt::new(1.0, 0.0, 0.0), Pnt::new(1.0, 1.0, 0.0)],
    ];
    GeomBSplineSurface::new(
        poles,
        vec![0.0, 1.0],
        vec![0.0, 1.0],
        vec![2, 2],
        vec![2, 2],
        1,
        1,
        false,
        false,
    )
}

/// Quadratic biquadratic surface, degree 2x2, 3x3 poles, single span.
fn biquadratic_surface() -> GeomBSplineSurface {
    let poles = vec![
        vec![Pnt::new(0.0, 0.0, 0.0), Pnt::new(0.0, 1.0, 0.5), Pnt::new(0.0, 2.0, 0.0)],
        vec![Pnt::new(1.0, 0.0, 0.5), Pnt::new(1.0, 1.0, 1.0), Pnt::new(1.0, 2.0, 0.5)],
        vec![Pnt::new(2.0, 0.0, 0.0), Pnt::new(2.0, 1.0, 0.5), Pnt::new(2.0, 2.0, 0.0)],
    ];
    GeomBSplineSurface::new(
        poles,
        vec![0.0, 1.0],
        vec![0.0, 1.0],
        vec![3, 3],
        vec![3, 3],
        2,
        2,
        false,
        false,
    )
}

// ---------------------------------------------------------------------------
// Tests: GeomConvert static utilities
// ---------------------------------------------------------------------------

/// `GeomConvert::CurveToBSplineCurve` is identity for a B-spline — it returns a
/// clone with the same degree, pole count, and evaluation.
#[test]
fn curve_to_bspline_preserves_degree_and_poles() {
    let c = cubic_bezier_bspline();
    let c2 = GeomConvert::curve_to_bspline(&c);
    assert_eq!(c.degree(), c2.degree());
    assert_eq!(c.nb_poles(), c2.nb_poles());
}

/// The cloned curve evaluates to the same values at multiple parameter samples.
#[test]
fn curve_to_bspline_evaluation_matches() {
    let c = cubic_bezier_bspline();
    let c2 = GeomConvert::curve_to_bspline(&c);
    for &u in &[0.0, 0.1, 0.25, 0.5, 0.75, 0.9, 1.0] {
        let a = c.value(u);
        let b = c2.value(u);
        assert!(
            a.distance(b) < 1.0e-10,
            "curve_to_bspline: values differ at u={u}: {a:?} vs {b:?}"
        );
    }
}

/// `GeomConvert::SurfaceToBSplineSurface` produces the same degree surface.
#[test]
fn surface_to_bspline_preserves_degrees() {
    let s = unit_bilinear_surface();
    let s2 = GeomConvert::surface_to_bspline(&s);
    assert_eq!(s.u_degree(), s2.u_degree());
    assert_eq!(s.v_degree(), s2.v_degree());
}

// ---------------------------------------------------------------------------
// Tests: GeomConvert_BSplineCurveToBezierCurve
// ---------------------------------------------------------------------------

/// A single-span Bezier-form B-spline decomposes into exactly one arc.
#[test]
fn single_span_cubic_decomposes_to_one_arc() {
    let c = cubic_bezier_bspline();
    let conv = GeomConvert_BSplineCurveToBezierCurve::new(&c);
    assert_eq!(conv.nb_arcs(), 1, "single-span curve should give 1 arc");
}

/// The arc recovered from a single-span B-spline has the same degree.
#[test]
fn arc_degree_equals_bspline_degree() {
    let c = cubic_bezier_bspline();
    let conv = GeomConvert_BSplineCurveToBezierCurve::new(&c);
    let arc = conv.arc(1);
    assert_eq!(arc.degree(), c.degree());
}

/// The start point of the first arc equals the start point of the original curve.
#[test]
fn first_arc_start_matches_curve_start() {
    let c = cubic_bezier_bspline();
    let conv = GeomConvert_BSplineCurveToBezierCurve::new(&c);
    let arc = conv.arc(1);
    assert!(
        arc.start_point().distance(c.start_point()) < 1.0e-8,
        "first arc start != curve start"
    );
}

/// The end point of the last arc equals the end point of the original curve.
#[test]
fn last_arc_end_matches_curve_end() {
    let c = two_span_quadratic_bspline();
    let conv = GeomConvert_BSplineCurveToBezierCurve::new(&c);
    let last = conv.arc(conv.nb_arcs());
    assert!(
        last.end_point().distance(c.end_point()) < 1.0e-8,
        "last arc end != curve end"
    );
}

/// A two-span quadratic B-spline decomposes into exactly two arcs.
#[test]
fn two_span_quadratic_decomposes_to_two_arcs() {
    let c = two_span_quadratic_bspline();
    let conv = GeomConvert_BSplineCurveToBezierCurve::new(&c);
    assert_eq!(conv.nb_arcs(), 2);
}

/// `Knots()` returns a slice of length `NbArcs + 1`.
#[test]
fn knots_length_is_nb_arcs_plus_one() {
    let c = two_span_quadratic_bspline();
    let conv = GeomConvert_BSplineCurveToBezierCurve::new(&c);
    assert_eq!(conv.knots().len(), (conv.nb_arcs() + 1) as usize);
}

/// Consecutive arcs share an endpoint within tolerance (G0 continuity).
#[test]
fn consecutive_arcs_share_endpoint() {
    let c = two_span_quadratic_bspline();
    let conv = GeomConvert_BSplineCurveToBezierCurve::new(&c);
    let nb = conv.nb_arcs();
    for i in 1..nb {
        let left = conv.arc(i);
        let right = conv.arc(i + 1);
        let dist = left.end_point().distance(right.start_point());
        assert!(dist < 1.0e-6, "arc {i}–{} junction gap = {dist}", i + 1);
    }
}

// ---------------------------------------------------------------------------
// Tests: GeomConvert_BSplineSurfaceToBezierSurface
// ---------------------------------------------------------------------------

/// A single-span bilinear surface decomposes into exactly one patch.
#[test]
fn bilinear_surface_gives_one_patch() {
    let s = unit_bilinear_surface();
    let conv = GeomConvert_BSplineSurfaceToBezierSurface::new(&s);
    assert_eq!(conv.nb_u_patches(), 1);
    assert_eq!(conv.nb_v_patches(), 1);
}

/// The recovered patch from a biquadratic surface has degree 2 in both directions.
#[test]
fn biquadratic_patch_degree_matches() {
    let s = biquadratic_surface();
    let conv = GeomConvert_BSplineSurfaceToBezierSurface::new(&s);
    let p = conv.patch(1, 1);
    assert_eq!(p.u_degree(), 2);
    assert_eq!(p.v_degree(), 2);
}

/// `UKnots` and `VKnots` lengths equal `NbUPatches+1` and `NbVPatches+1`.
#[test]
fn surface_knot_arrays_have_correct_length() {
    let s = unit_bilinear_surface();
    let conv = GeomConvert_BSplineSurfaceToBezierSurface::new(&s);
    assert_eq!(conv.u_knots().len(), (conv.nb_u_patches() + 1) as usize);
    assert_eq!(conv.v_knots().len(), (conv.nb_v_patches() + 1) as usize);
}

// ---------------------------------------------------------------------------
// Tests: GeomConvert_CompCurveToBSplineCurve
// ---------------------------------------------------------------------------

/// Building a composite curve from a single segment returns the original curve.
#[test]
fn comp_curve_single_segment_roundtrip() {
    let c = cubic_bezier_bspline();
    let builder = GeomConvert_CompCurveToBSplineCurve::new(&c);
    let result = builder.bspline_curve().expect("should return a B-spline");
    assert_eq!(result.degree(), c.degree());
    assert_eq!(result.nb_poles(), c.nb_poles());
    // Verify evaluation at several parameter values.
    for &u in &[0.0, 0.3, 0.6, 1.0] {
        let a = c.value(u);
        let b = result.value(u);
        assert!(a.distance(b) < 1.0e-9, "comp single segment: mismatch at u={u}");
    }
}

/// `Add` with a gap (non-matching endpoints) returns `false`.
#[test]
fn comp_curve_add_disjoint_returns_false() {
    let c = cubic_bezier_bspline();
    let disjoint = GeomBSplineCurve::new(
        &[
            Pnt::new(100.0, 100.0, 0.0),
            Pnt::new(101.0, 101.0, 0.0),
            Pnt::new(102.0, 101.0, 0.0),
            Pnt::new(103.0, 100.0, 0.0),
        ],
        &[0.0, 1.0],
        &[4, 4],
        3,
        false,
    );
    let mut builder = GeomConvert_CompCurveToBSplineCurve::new(&c);
    assert!(!builder.add(&disjoint, 1.0e-6), "expected false for disjoint curves");
}

/// After a successful `Add`, the degree of the composite equals the segment degree.
#[test]
fn comp_curve_degree_preserved_after_add() {
    let c = cubic_bezier_bspline();
    // Create a second segment that starts at (3,0,0), the end of `c`.
    let c2 = GeomBSplineCurve::new(
        &[
            Pnt::new(3.0, 0.0, 0.0),
            Pnt::new(4.0, -1.0, 0.0),
            Pnt::new(5.0, -1.0, 0.0),
            Pnt::new(6.0, 0.0, 0.0),
        ],
        &[0.0, 1.0],
        &[4, 4],
        3,
        false,
    );
    let mut builder = GeomConvert_CompCurveToBSplineCurve::new(&c);
    let ok = builder.add(&c2, 1.0e-7);
    assert!(ok, "add should succeed for connected curves");
    assert_eq!(builder.degree(), 3);
}
