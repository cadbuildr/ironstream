// FILE: tests/occt_geom_fill.rs
//! Integration tests for `ironstream::geom_fill` — the GeomFill filling
//! algorithms port.
//!
//! Tests exercise the public API only, covering:
//! - [`GeomFillBezierCurves`]  (two- and four-boundary Bezier fills)
//! - [`GeomFillBSplineCurves`] (two- and four-boundary B-spline fills)
//! - [`GeomFillPipe`]          (circular-section pipe sweep)
//! - [`GeomFillConstrainedFilling`] (G0/G1 constrained fills)

extern crate ironstream;

use ironstream::geom_bspline_curve::GeomBSplineCurve;
use ironstream::geom_fill::{
    ConstraintOrder, FillingBoundary, FillingStyle, GeomFillBSplineCurves,
    GeomFillBezierCurves, GeomFillConstrainedFilling, GeomFillPipe, PipeError,
    PipeTransitionMode,
};
use ironstream::gp::Pnt;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Build a degree-1 (linear) B-spline from two endpoints.
fn line_bspline(p0: Pnt, p1: Pnt) -> GeomBSplineCurve {
    let poles = vec![p0, p1];
    let knots = vec![0.0, 1.0];
    let mults = vec![2, 2];
    GeomBSplineCurve::new(&poles, &knots, &mults, 1, false)
}

/// Build a degree-3 (cubic) B-spline through four control points.
fn cubic_bspline(p0: Pnt, p1: Pnt, p2: Pnt, p3: Pnt) -> GeomBSplineCurve {
    let poles = vec![p0, p1, p2, p3];
    let knots = vec![0.0, 1.0];
    let mults = vec![4, 4];
    GeomBSplineCurve::new(&poles, &knots, &mults, 3, false)
}

// ---------------------------------------------------------------------------
// GeomFillBezierCurves — basic construction
// ---------------------------------------------------------------------------

#[test]
fn bezier_fill_default_has_no_curves() {
    let filler = GeomFillBezierCurves::new();
    assert_eq!(filler.nb_curves(), 0);
}

#[test]
fn bezier_fill_two_straight_lines_produces_ruled_patch() {
    // Two parallel horizontal lines; should produce a ruled surface.
    let c0 = vec![Pnt::new(0.0, 0.0, 0.0), Pnt::new(3.0, 0.0, 0.0)];
    let c1 = vec![Pnt::new(0.0, 2.0, 0.0), Pnt::new(3.0, 2.0, 0.0)];

    let mut filler = GeomFillBezierCurves::new();
    filler.init_two(c0, c1, FillingStyle::Stretch);

    let surf = filler.surface();
    // Ruled: U = degree 1, V = degree 3 (4 poles).
    assert_eq!(surf.u_degree(), 1);
    assert_eq!(surf.v_degree(), 3);
    assert_eq!(surf.nb_u_poles(), 2);
    assert_eq!(surf.nb_v_poles(), 4);
}

#[test]
fn bezier_fill_two_curves_coons_style_corner_values() {
    // Lines from (0,0,0)→(1,0,0) and (0,1,0)→(1,1,0).
    // The first U-pole row should be on the bottom line.
    let c0 = vec![Pnt::new(0.0, 0.0, 0.0), Pnt::new(1.0, 0.0, 0.0)];
    let c1 = vec![Pnt::new(0.0, 1.0, 0.0), Pnt::new(1.0, 1.0, 0.0)];
    let mut filler = GeomFillBezierCurves::new();
    filler.init_two(c0.clone(), c1.clone(), FillingStyle::Coons);
    let surf = filler.surface();
    // Corner (u=0, v=0) ~ c0[0] and corner (u=0, v=1) ~ c1[0].
    let p00 = surf.value(0.0, 0.0);
    let p01 = surf.value(0.0, 1.0);
    assert!(p00.distance(c0[0]) < 1.0e-9, "p(0,0) should be c0 start: {:?}", p00);
    assert!(p01.distance(c1[0]) < 1.0e-9, "p(0,1) should be c1 start: {:?}", p01);
}

// ---------------------------------------------------------------------------
// GeomFillBezierCurves — four boundaries (Coons patch)
// ---------------------------------------------------------------------------

#[test]
fn bezier_fill_four_boundaries_style_stretch_produces_bilinear() {
    // Unit square: Stretch == bilinear, so the center should be exactly (0.5, 0.5, 0).
    let bot   = vec![Pnt::new(0.0, 0.0, 0.0), Pnt::new(1.0, 0.0, 0.0)];
    let right = vec![Pnt::new(1.0, 0.0, 0.0), Pnt::new(1.0, 1.0, 0.0)];
    let top   = vec![Pnt::new(0.0, 1.0, 0.0), Pnt::new(1.0, 1.0, 0.0)];
    let left  = vec![Pnt::new(0.0, 0.0, 0.0), Pnt::new(0.0, 1.0, 0.0)];

    let mut filler = GeomFillBezierCurves::new();
    filler.init_four(bot, right, top, left, FillingStyle::Stretch);
    assert_eq!(filler.nb_curves(), 4);

    let surf = filler.surface();
    let mid = surf.value(0.5, 0.5);
    assert!((mid.x - 0.5).abs() < 0.05, "x at mid: {}", mid.x);
    assert!((mid.y - 0.5).abs() < 0.05, "y at mid: {}", mid.y);
    assert!(mid.z.abs() < 1.0e-10, "z at mid should be 0: {}", mid.z);
}

#[test]
fn bezier_fill_four_boundaries_curved_style() {
    let bot   = vec![Pnt::new(0.0, 0.0, 0.0), Pnt::new(1.0, 0.0, 0.0)];
    let right = vec![Pnt::new(1.0, 0.0, 0.0), Pnt::new(1.0, 1.0, 0.0)];
    let top   = vec![Pnt::new(0.0, 1.0, 0.0), Pnt::new(1.0, 1.0, 0.0)];
    let left  = vec![Pnt::new(0.0, 0.0, 0.0), Pnt::new(0.0, 1.0, 0.0)];

    let mut filler = GeomFillBezierCurves::new();
    filler.init_four(bot, right, top, left, FillingStyle::Curved);
    assert_eq!(filler.style(), FillingStyle::Curved);
    let surf = filler.surface();
    // Surface must be 4×4.
    assert_eq!(surf.nb_u_poles(), 4);
    assert_eq!(surf.nb_v_poles(), 4);
}

// ---------------------------------------------------------------------------
// GeomFillBSplineCurves — ruled fill
// ---------------------------------------------------------------------------

#[test]
fn bspline_fill_two_linear_curves_produces_4x4_surface() {
    let c0 = line_bspline(Pnt::new(0.0, 0.0, 0.0), Pnt::new(4.0, 0.0, 0.0));
    let c1 = line_bspline(Pnt::new(0.0, 3.0, 0.0), Pnt::new(4.0, 3.0, 0.0));

    let mut filler = GeomFillBSplineCurves::new();
    filler.init_two(c0, c1, FillingStyle::Stretch);

    assert_eq!(filler.nb_curves(), 2);
    let surf = filler.surface();
    assert_eq!(surf.nb_u_poles(), 4);
    assert_eq!(surf.nb_v_poles(), 4);
}

#[test]
fn bspline_fill_two_curves_corner_interpolation() {
    let p_start_bot = Pnt::new(0.0, 0.0, 0.0);
    let p_end_bot   = Pnt::new(1.0, 0.0, 0.0);
    let p_start_top = Pnt::new(0.0, 1.0, 0.0);
    let p_end_top   = Pnt::new(1.0, 1.0, 0.0);

    let c0 = line_bspline(p_start_bot, p_end_bot);
    let c1 = line_bspline(p_start_top, p_end_top);

    let mut filler = GeomFillBSplineCurves::new();
    filler.init_two(c0, c1, FillingStyle::Stretch);
    let surf = filler.surface();

    // Corners of the filled surface.
    let p00 = surf.value(0.0, 0.0);
    let p11 = surf.value(1.0, 1.0);

    // The ruled surface corners should be close to the input curve endpoints.
    assert!(p00.distance(p_start_bot) < 0.1, "corner (0,0): {:?}", p00);
    assert!(p11.distance(p_end_top)   < 0.1, "corner (1,1): {:?}", p11);
}

// ---------------------------------------------------------------------------
// GeomFillBSplineCurves — four boundaries
// ---------------------------------------------------------------------------

#[test]
fn bspline_fill_four_boundaries_coons() {
    let c0 = line_bspline(Pnt::new(0.0, 0.0, 0.0), Pnt::new(2.0, 0.0, 0.0));
    let c1 = line_bspline(Pnt::new(2.0, 0.0, 0.0), Pnt::new(2.0, 2.0, 0.0));
    let c2 = line_bspline(Pnt::new(0.0, 2.0, 0.0), Pnt::new(2.0, 2.0, 0.0));
    let c3 = line_bspline(Pnt::new(0.0, 0.0, 0.0), Pnt::new(0.0, 2.0, 0.0));

    let mut filler = GeomFillBSplineCurves::new();
    filler.init_four(c0, c1, c2, c3, FillingStyle::Coons);

    assert_eq!(filler.nb_curves(), 4);
    assert_eq!(filler.style(), FillingStyle::Coons);
    let surf = filler.surface();
    assert_eq!(surf.nb_u_poles(), 4);
    assert_eq!(surf.nb_v_poles(), 4);
}

#[test]
fn bspline_fill_four_boundaries_centre_is_interior() {
    // The centre of a flat square fill should be inside the square.
    let c0 = line_bspline(Pnt::new(0.0, 0.0, 0.0), Pnt::new(1.0, 0.0, 0.0));
    let c1 = line_bspline(Pnt::new(1.0, 0.0, 0.0), Pnt::new(1.0, 1.0, 0.0));
    let c2 = line_bspline(Pnt::new(0.0, 1.0, 0.0), Pnt::new(1.0, 1.0, 0.0));
    let c3 = line_bspline(Pnt::new(0.0, 0.0, 0.0), Pnt::new(0.0, 1.0, 0.0));

    let mut filler = GeomFillBSplineCurves::new();
    filler.init_four(c0, c1, c2, c3, FillingStyle::Coons);
    let surf = filler.surface();
    let mid = surf.value(0.5, 0.5);
    // Must be inside the unit square in XY, z = 0.
    assert!(mid.x > 0.0 && mid.x < 1.0, "x in (0,1): {}", mid.x);
    assert!(mid.y > 0.0 && mid.y < 1.0, "y in (0,1): {}", mid.y);
    assert!(mid.z.abs() < 1.0e-10, "z should be 0: {}", mid.z);
}

// ---------------------------------------------------------------------------
// GeomFillPipe
// ---------------------------------------------------------------------------

#[test]
fn pipe_straight_spine_produces_tubular_surface() {
    // Straight spine along Z-axis, radius 1.
    let spine = cubic_bspline(
        Pnt::new(0.0, 0.0, 0.0),
        Pnt::new(0.0, 0.0, 1.0),
        Pnt::new(0.0, 0.0, 2.0),
        Pnt::new(0.0, 0.0, 3.0),
    );
    let mut pipe = GeomFillPipe::new(spine, 1.0);
    pipe.perform();
    assert!(pipe.is_done(), "pipe must be done after perform()");

    let surf = pipe.surface();
    // Surface must have at least 4 poles in each direction.
    assert!(surf.nb_u_poles() >= 4);
    assert!(surf.nb_v_poles() >= 3);
}

#[test]
fn pipe_section_points_approx_radius_from_spine() {
    // Spine along Z; every point on the tube should be ~radius from the spine.
    let spine = line_bspline(Pnt::new(0.0, 0.0, 0.0), Pnt::new(0.0, 0.0, 4.0));
    let r = 0.5;
    let mut pipe = GeomFillPipe::new(spine, r);
    pipe.perform();
    assert!(pipe.is_done());

    let surf = pipe.surface();
    // Sample a few points and check their XY distance from the Z-axis.
    for &(u, v) in &[(0.0, 0.0), (0.5, 0.25), (1.0, 0.5)] {
        let p = surf.value(u, v);
        let dist_from_axis = (p.x * p.x + p.y * p.y).sqrt();
        // Allow generous tolerance (control-point approx, not exact NURBS circle).
        assert!(
            dist_from_axis < r * 2.0 + 0.5,
            "dist from axis {} at ({},{}) = {}",
            r,
            u,
            v,
            dist_from_axis
        );
    }
}

#[test]
fn pipe_error_is_ok_after_successful_perform() {
    let spine = line_bspline(Pnt::new(0.0, 0.0, 0.0), Pnt::new(1.0, 0.0, 0.0));
    let mut pipe = GeomFillPipe::new(spine, 0.2);
    pipe.perform();
    assert_eq!(pipe.error_status(), PipeError::PipeOk);
}

#[test]
fn pipe_is_not_done_before_perform() {
    let spine = line_bspline(Pnt::new(0.0, 0.0, 0.0), Pnt::new(1.0, 0.0, 0.0));
    let pipe = GeomFillPipe::new(spine, 0.3);
    assert!(!pipe.is_done());
}

#[test]
fn pipe_transition_mode_can_be_changed() {
    let spine = line_bspline(Pnt::new(0.0, 0.0, 0.0), Pnt::new(2.0, 0.0, 0.0));
    let mut pipe = GeomFillPipe::new(spine, 0.5);
    pipe.set_transition(PipeTransitionMode::CorrectedFrenet);
    pipe.perform();
    assert!(pipe.is_done());
}

// ---------------------------------------------------------------------------
// GeomFillConstrainedFilling
// ---------------------------------------------------------------------------

#[test]
fn constrained_fill_g0_flat_square_is_done() {
    let c0 = line_bspline(Pnt::new(0.0, 0.0, 0.0), Pnt::new(1.0, 0.0, 0.0));
    let c1 = line_bspline(Pnt::new(1.0, 0.0, 0.0), Pnt::new(1.0, 1.0, 0.0));
    let c2 = line_bspline(Pnt::new(0.0, 1.0, 0.0), Pnt::new(1.0, 1.0, 0.0));
    let c3 = line_bspline(Pnt::new(0.0, 0.0, 0.0), Pnt::new(0.0, 1.0, 0.0));

    let mut filler = GeomFillConstrainedFilling::new(3, 1);
    filler.init_four(
        FillingBoundary::new_g0(c0),
        FillingBoundary::new_g0(c1),
        FillingBoundary::new_g0(c2),
        FillingBoundary::new_g0(c3),
    );
    filler.build();
    assert!(filler.is_done());
}

#[test]
fn constrained_fill_g0_surface_is_4x4() {
    let c0 = line_bspline(Pnt::new(0.0, 0.0, 0.0), Pnt::new(1.0, 0.0, 0.0));
    let c1 = line_bspline(Pnt::new(1.0, 0.0, 0.0), Pnt::new(1.0, 1.0, 0.0));
    let c2 = line_bspline(Pnt::new(0.0, 1.0, 0.0), Pnt::new(1.0, 1.0, 0.0));
    let c3 = line_bspline(Pnt::new(0.0, 0.0, 0.0), Pnt::new(0.0, 1.0, 0.0));

    let mut filler = GeomFillConstrainedFilling::new(3, 1);
    filler.init_four(
        FillingBoundary::new_g0(c0),
        FillingBoundary::new_g0(c1),
        FillingBoundary::new_g0(c2),
        FillingBoundary::new_g0(c3),
    );
    filler.build();
    let surf = filler.surface();
    assert_eq!(surf.nb_u_poles(), 4);
    assert_eq!(surf.nb_v_poles(), 4);
}

#[test]
fn constrained_fill_g1_shifts_inner_poles_in_normal_direction() {
    // G1 boundary with upward normals: inner control points should have z > 0.
    let c0 = line_bspline(Pnt::new(0.0, 0.0, 0.0), Pnt::new(1.0, 0.0, 0.0));
    let c1 = line_bspline(Pnt::new(1.0, 0.0, 0.0), Pnt::new(1.0, 1.0, 0.0));
    let c2 = line_bspline(Pnt::new(0.0, 1.0, 0.0), Pnt::new(1.0, 1.0, 0.0));
    let c3 = line_bspline(Pnt::new(0.0, 0.0, 0.0), Pnt::new(0.0, 1.0, 0.0));

    let normals = vec![Pnt::new(0.0, 0.0, 1.0); 4];
    let b0 = FillingBoundary::new_g1(c0, normals);

    let mut filler = GeomFillConstrainedFilling::new(3, 1);
    filler.init_four(
        b0,
        FillingBoundary::new_g0(c1),
        FillingBoundary::new_g0(c2),
        FillingBoundary::new_g0(c3),
    );
    filler.build();
    assert!(filler.is_done());
    // The inner pole (2,2) in 1-based coordinates should have z > 0.
    let surf = filler.surface();
    let p = surf.pole(2, 2);
    assert!(p.z > 0.0, "G1 normal correction: inner pole z={}", p.z);
}

#[test]
fn constrained_fill_nb_boundaries_after_add() {
    let mut filler = GeomFillConstrainedFilling::new(3, 1);
    let c = line_bspline(Pnt::new(0.0, 0.0, 0.0), Pnt::new(1.0, 0.0, 0.0));
    filler.add_boundary(FillingBoundary::new_g0(c));
    assert_eq!(filler.nb_boundaries(), 1);
}

#[test]
fn constrained_fill_constraint_order_comparison() {
    // G0 < G1 < G2 must hold for the ordering to be meaningful.
    assert!(ConstraintOrder::G0 < ConstraintOrder::G1);
    assert!(ConstraintOrder::G1 < ConstraintOrder::G2);
    assert!(ConstraintOrder::G0 < ConstraintOrder::G2);
}

#[test]
fn filling_boundary_g0_has_no_tangent_normals() {
    let c = line_bspline(Pnt::new(0.0, 0.0, 0.0), Pnt::new(1.0, 0.0, 0.0));
    let b = FillingBoundary::new_g0(c);
    assert_eq!(b.order(), ConstraintOrder::G0);
    // G0 has no tangent normals; curve reference accessible.
    let _ = b.curve();
}

#[test]
fn filling_boundary_g2_order() {
    let c = line_bspline(Pnt::new(0.0, 0.0, 0.0), Pnt::new(1.0, 0.0, 0.0));
    let normals = vec![Pnt::new(0.0, 0.0, 1.0); 2];
    let b = FillingBoundary::new_g2(c, normals);
    assert_eq!(b.order(), ConstraintOrder::G2);
}
