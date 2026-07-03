// FILE: rust/ironstream/crates/ironstream/tests/occt_conv_surface.rs
//! OCCT-faithful integration tests for `conv_surface::ConvBSplineSurface`.
//!
//! Mirrors the semantics described in OpenCascade's
//! `Convert_BSplineSurface` documentation and verifies:
//! - structural invariants (nb_poles, nb_knots, degrees, multiplicities)
//! - 1-based accessor indexing for poles, weights, knots, multiplicities
//! - `is_done()` lifecycle flag
//! - rational vs. non-rational weight handling
//! - raw-slice accessors for bulk data access
//! - panic guards for out-of-range indices

extern crate ironstream;
use ironstream::conv_surface::*;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Bilinear (degree 1×1) B-spline surface on [0,1]×[0,1], 2×2 poles.
/// OCCT equivalent: Convert_BSplineSurface with the simplest plane patch.
fn bilinear() -> ConvBSplineSurface {
    ConvBSplineSurface::new(
        vec![
            [0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 1.0, 0.0],
        ],
        vec![0.0, 1.0],
        vec![0.0, 1.0],
        vec![2, 2],
        vec![2, 2],
        1,
        1,
        false,
        false,
        vec![],
    )
}

/// Biquadratic (degree 2×2) B-spline surface on [0,1]×[0,1], 3×3 poles.
/// OCCT equivalent: a single Bézier patch stored as a B-spline surface.
fn biquadratic() -> ConvBSplineSurface {
    let mut poles = Vec::with_capacity(9);
    for i in 0..3usize {
        for j in 0..3usize {
            poles.push([i as f64 * 0.5, j as f64 * 0.5, (i + j) as f64 * 0.1]);
        }
    }
    ConvBSplineSurface::new(
        poles,
        vec![0.0, 1.0],
        vec![0.0, 1.0],
        vec![3, 3],
        vec![3, 3],
        2,
        2,
        false,
        false,
        vec![],
    )
}

/// Bicubic (degree 3×3) B-spline surface, 4×4 poles.
fn bicubic() -> ConvBSplineSurface {
    let mut poles = Vec::with_capacity(16);
    for i in 0..4usize {
        for j in 0..4usize {
            poles.push([i as f64, j as f64, 0.0]);
        }
    }
    ConvBSplineSurface::new(
        poles,
        vec![0.0, 1.0],
        vec![0.0, 1.0],
        vec![4, 4],
        vec![4, 4],
        3,
        3,
        false,
        false,
        vec![],
    )
}

/// Two-span quadratic in U, single span in V: 5×3 poles.
fn two_span_u_surface() -> ConvBSplineSurface {
    // u: mults [3,2,3], degree 2 → nb_u = 8-2-1 = 5
    // v: mults [3,3],   degree 2 → nb_v = 6-2-1 = 3
    let mut poles = Vec::with_capacity(15);
    for i in 0..5usize {
        for j in 0..3usize {
            poles.push([i as f64 * 0.25, j as f64 * 0.5, 0.0]);
        }
    }
    ConvBSplineSurface::new(
        poles,
        vec![0.0, 0.5, 1.0],
        vec![0.0, 1.0],
        vec![3, 2, 3],
        vec![3, 3],
        2,
        2,
        false,
        false,
        vec![],
    )
}

/// Rational bilinear surface with non-unit weights.
fn rational_bilinear() -> ConvBSplineSurface {
    ConvBSplineSurface::new(
        vec![
            [0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 1.0, 0.0],
        ],
        vec![0.0, 1.0],
        vec![0.0, 1.0],
        vec![2, 2],
        vec![2, 2],
        1,
        1,
        true,
        true,
        vec![1.0, 2.0, 3.0, 4.0],
    )
}

// ---------------------------------------------------------------------------
// 1.  IsDone lifecycle
// ---------------------------------------------------------------------------

#[test]
fn default_empty_is_not_done() {
    let s = ConvBSplineSurface::default_empty();
    assert!(!s.is_done(), "default-empty surface must not be done");
}

#[test]
fn bilinear_is_done() {
    assert!(bilinear().is_done());
}

#[test]
fn biquadratic_is_done() {
    assert!(biquadratic().is_done());
}

#[test]
fn bicubic_is_done() {
    assert!(bicubic().is_done());
}

// ---------------------------------------------------------------------------
// 2.  NbUPoles / NbVPoles
// ---------------------------------------------------------------------------

#[test]
fn bilinear_nb_poles_is_2x2() {
    let s = bilinear();
    assert_eq!(s.nb_u_poles(), 2);
    assert_eq!(s.nb_v_poles(), 2);
}

#[test]
fn biquadratic_nb_poles_is_3x3() {
    let s = biquadratic();
    assert_eq!(s.nb_u_poles(), 3);
    assert_eq!(s.nb_v_poles(), 3);
}

#[test]
fn bicubic_nb_poles_is_4x4() {
    let s = bicubic();
    assert_eq!(s.nb_u_poles(), 4);
    assert_eq!(s.nb_v_poles(), 4);
}

#[test]
fn two_span_u_nb_poles_is_5x3() {
    let s = two_span_u_surface();
    assert_eq!(s.nb_u_poles(), 5);
    assert_eq!(s.nb_v_poles(), 3);
}

// ---------------------------------------------------------------------------
// 3.  UDegree / VDegree
// ---------------------------------------------------------------------------

#[test]
fn bilinear_degree_is_1x1() {
    let s = bilinear();
    assert_eq!(s.u_degree(), 1);
    assert_eq!(s.v_degree(), 1);
}

#[test]
fn biquadratic_degree_is_2x2() {
    let s = biquadratic();
    assert_eq!(s.u_degree(), 2);
    assert_eq!(s.v_degree(), 2);
}

#[test]
fn bicubic_degree_is_3x3() {
    let s = bicubic();
    assert_eq!(s.u_degree(), 3);
    assert_eq!(s.v_degree(), 3);
}

// ---------------------------------------------------------------------------
// 4.  NbUKnots / NbVKnots
// ---------------------------------------------------------------------------

#[test]
fn bilinear_nb_knots() {
    let s = bilinear();
    assert_eq!(s.nb_u_knots(), 2);
    assert_eq!(s.nb_v_knots(), 2);
}

#[test]
fn two_span_u_nb_u_knots_is_3() {
    assert_eq!(two_span_u_surface().nb_u_knots(), 3);
}

#[test]
fn two_span_u_nb_v_knots_is_2() {
    assert_eq!(two_span_u_surface().nb_v_knots(), 2);
}

// ---------------------------------------------------------------------------
// 5.  UKnot / VKnot — 1-based values
// ---------------------------------------------------------------------------

#[test]
fn bilinear_u_knot_values() {
    let s = bilinear();
    assert_eq!(s.u_knot(1), 0.0);
    assert_eq!(s.u_knot(2), 1.0);
}

#[test]
fn bilinear_v_knot_values() {
    let s = bilinear();
    assert_eq!(s.v_knot(1), 0.0);
    assert_eq!(s.v_knot(2), 1.0);
}

#[test]
fn two_span_interior_u_knot_is_half() {
    let s = two_span_u_surface();
    assert!((s.u_knot(2) - 0.5).abs() < 1e-15);
}

// ---------------------------------------------------------------------------
// 6.  UMultiplicity / VMultiplicity — 1-based
// ---------------------------------------------------------------------------

#[test]
fn bilinear_endpoint_multiplicities() {
    let s = bilinear();
    // Clamped degree-1: end mults = degree + 1 = 2
    assert_eq!(s.u_multiplicity(1), 2);
    assert_eq!(s.u_multiplicity(2), 2);
    assert_eq!(s.v_multiplicity(1), 2);
    assert_eq!(s.v_multiplicity(2), 2);
}

#[test]
fn two_span_u_interior_mult_is_2() {
    let s = two_span_u_surface();
    // Interior knot at 0.5 has multiplicity 2 (C1 continuity for degree 2).
    assert_eq!(s.u_multiplicity(2), 2);
}

#[test]
fn two_span_u_endpoint_mults_are_3() {
    let s = two_span_u_surface();
    assert_eq!(s.u_multiplicity(1), 3);
    assert_eq!(s.u_multiplicity(3), 3);
}

#[test]
fn biquadratic_endpoint_mults_are_degree_plus_one() {
    let s = biquadratic();
    // Single span, degree 2: endpoint mults = 3.
    assert_eq!(s.u_multiplicity(1), 3);
    assert_eq!(s.u_multiplicity(2), 3);
    assert_eq!(s.v_multiplicity(1), 3);
    assert_eq!(s.v_multiplicity(2), 3);
}

// ---------------------------------------------------------------------------
// 7.  Pole — 1-based (UIndex, VIndex)
// ---------------------------------------------------------------------------

#[test]
fn bilinear_all_poles() {
    let s = bilinear();
    assert_eq!(s.pole(1, 1), [0.0, 0.0, 0.0]);
    assert_eq!(s.pole(1, 2), [0.0, 1.0, 0.0]);
    assert_eq!(s.pole(2, 1), [1.0, 0.0, 0.0]);
    assert_eq!(s.pole(2, 2), [1.0, 1.0, 0.0]);
}

#[test]
fn biquadratic_corner_poles() {
    let s = biquadratic();
    assert_eq!(s.pole(1, 1), [0.0, 0.0, 0.0]);
    // (i=2,j=2) → 0-based (1,1): [0.5, 0.5, 0.2]
    let p = s.pole(2, 2);
    assert!((p[0] - 0.5).abs() < 1e-15);
    assert!((p[1] - 0.5).abs() < 1e-15);
    assert!((p[2] - 0.2).abs() < 1e-15);
}

#[test]
fn biquadratic_last_pole() {
    let s = biquadratic();
    // Last pole: u=3, v=3 → 0-based (2,2): [1.0, 1.0, 0.4]
    let p = s.pole(3, 3);
    assert!((p[0] - 1.0).abs() < 1e-15);
    assert!((p[1] - 1.0).abs() < 1e-15);
    assert!((p[2] - 0.4).abs() < 1e-15);
}

// ---------------------------------------------------------------------------
// 8.  Weight — 1-based (UIndex, VIndex)
// ---------------------------------------------------------------------------

#[test]
fn non_rational_all_weights_are_one() {
    let s = bilinear();
    for ui in 1..=2 {
        for vi in 1..=2 {
            assert_eq!(s.weight(ui, vi), 1.0, "weight({ui},{vi})");
        }
    }
}

#[test]
fn rational_weights_correct() {
    let s = rational_bilinear();
    assert_eq!(s.weight(1, 1), 1.0);
    assert_eq!(s.weight(1, 2), 2.0);
    assert_eq!(s.weight(2, 1), 3.0);
    assert_eq!(s.weight(2, 2), 4.0);
}

// ---------------------------------------------------------------------------
// 9.  IsURational / IsVRational
// ---------------------------------------------------------------------------

#[test]
fn non_rational_flags_are_false() {
    let s = bilinear();
    assert!(!s.is_u_rational());
    assert!(!s.is_v_rational());
}

#[test]
fn rational_flags_are_true() {
    let s = rational_bilinear();
    assert!(s.is_u_rational());
    assert!(s.is_v_rational());
}

// ---------------------------------------------------------------------------
// 10. Raw slice accessors
// ---------------------------------------------------------------------------

#[test]
fn poles_raw_has_correct_length() {
    assert_eq!(bilinear().poles_raw().len(), 4);
    assert_eq!(biquadratic().poles_raw().len(), 9);
    assert_eq!(bicubic().poles_raw().len(), 16);
    assert_eq!(two_span_u_surface().poles_raw().len(), 15);
}

#[test]
fn weights_raw_length_matches_poles() {
    let s = rational_bilinear();
    assert_eq!(s.weights_raw().len(), s.poles_raw().len());
}

#[test]
fn non_rational_weights_raw_are_all_one() {
    let s = biquadratic();
    assert!(s.weights_raw().iter().all(|&w| (w - 1.0).abs() < 1e-15));
}

#[test]
fn u_knots_raw_matches_knot_count() {
    let s = two_span_u_surface();
    assert_eq!(s.u_knots_raw().len(), s.nb_u_knots());
}

#[test]
fn v_knots_raw_matches_knot_count() {
    let s = two_span_u_surface();
    assert_eq!(s.v_knots_raw().len(), s.nb_v_knots());
}

#[test]
fn u_mults_raw_matches_knot_count() {
    let s = two_span_u_surface();
    assert_eq!(s.u_mults_raw().len(), s.nb_u_knots());
}

#[test]
fn v_mults_raw_matches_knot_count() {
    let s = two_span_u_surface();
    assert_eq!(s.v_mults_raw().len(), s.nb_v_knots());
}

#[test]
fn bilinear_u_knots_raw_values() {
    assert_eq!(bilinear().u_knots_raw(), &[0.0f64, 1.0]);
}

#[test]
fn bilinear_u_mults_raw_values() {
    assert_eq!(bilinear().u_mults_raw(), &[2u32, 2]);
}

// ---------------------------------------------------------------------------
// 11. Clone produces independent equal data
// ---------------------------------------------------------------------------

#[test]
fn clone_is_equal_and_independent() {
    let s = biquadratic();
    let t = s.clone();
    assert_eq!(t.nb_u_poles(), s.nb_u_poles());
    assert_eq!(t.nb_v_poles(), s.nb_v_poles());
    assert_eq!(t.u_degree(), s.u_degree());
    assert_eq!(t.v_degree(), s.v_degree());
    assert_eq!(t.poles_raw(), s.poles_raw());
    assert_eq!(t.weights_raw(), s.weights_raw());
    assert_eq!(t.u_knots_raw(), s.u_knots_raw());
    assert_eq!(t.v_knots_raw(), s.v_knots_raw());
    assert_eq!(t.u_mults_raw(), s.u_mults_raw());
    assert_eq!(t.v_mults_raw(), s.v_mults_raw());
}

// ---------------------------------------------------------------------------
// 12. Panic guards — out-of-range 1-based indices
// ---------------------------------------------------------------------------

#[test]
#[should_panic]
fn pole_u_index_zero_panics() {
    let _ = bilinear().pole(0, 1);
}

#[test]
#[should_panic]
fn pole_u_index_overflow_panics() {
    let _ = bilinear().pole(3, 1);
}

#[test]
#[should_panic]
fn pole_v_index_zero_panics() {
    let _ = bilinear().pole(1, 0);
}

#[test]
#[should_panic]
fn pole_v_index_overflow_panics() {
    let _ = bilinear().pole(1, 3);
}

#[test]
#[should_panic]
fn weight_u_index_zero_panics() {
    let _ = bilinear().weight(0, 1);
}

#[test]
#[should_panic]
fn weight_v_index_overflow_panics() {
    let _ = bilinear().weight(1, 3);
}

#[test]
#[should_panic]
fn u_knot_index_zero_panics() {
    let _ = bilinear().u_knot(0);
}

#[test]
#[should_panic]
fn u_knot_index_overflow_panics() {
    let _ = bilinear().u_knot(3);
}

#[test]
#[should_panic]
fn v_knot_index_zero_panics() {
    let _ = bilinear().v_knot(0);
}

#[test]
#[should_panic]
fn v_knot_index_overflow_panics() {
    let _ = bilinear().v_knot(3);
}

#[test]
#[should_panic]
fn u_multiplicity_index_zero_panics() {
    let _ = bilinear().u_multiplicity(0);
}

#[test]
#[should_panic]
fn v_multiplicity_index_overflow_panics() {
    let _ = bilinear().v_multiplicity(3);
}

// ---------------------------------------------------------------------------
// 13. Mults sum consistency: sum(u_mults) = nb_u_poles + u_degree + 1
// ---------------------------------------------------------------------------

#[test]
fn bilinear_u_mults_sum_identity() {
    let s = bilinear();
    let sum: u32 = s.u_mults_raw().iter().sum();
    assert_eq!(sum as usize, s.nb_u_poles() + s.u_degree() as usize + 1);
}

#[test]
fn biquadratic_u_mults_sum_identity() {
    let s = biquadratic();
    let sum: u32 = s.u_mults_raw().iter().sum();
    assert_eq!(sum as usize, s.nb_u_poles() + s.u_degree() as usize + 1);
}

#[test]
fn two_span_u_mults_sum_identity() {
    let s = two_span_u_surface();
    let sum: u32 = s.u_mults_raw().iter().sum();
    assert_eq!(sum as usize, s.nb_u_poles() + s.u_degree() as usize + 1);
}

#[test]
fn bilinear_v_mults_sum_identity() {
    let s = bilinear();
    let sum: u32 = s.v_mults_raw().iter().sum();
    assert_eq!(sum as usize, s.nb_v_poles() + s.v_degree() as usize + 1);
}

// ---------------------------------------------------------------------------
// 14. Default-empty accessors return zero / empty
// ---------------------------------------------------------------------------

#[test]
fn default_empty_nb_poles_are_zero() {
    let s = ConvBSplineSurface::default_empty();
    assert_eq!(s.nb_u_poles(), 0);
    assert_eq!(s.nb_v_poles(), 0);
}

#[test]
fn default_empty_degrees_are_zero() {
    let s = ConvBSplineSurface::default_empty();
    assert_eq!(s.u_degree(), 0);
    assert_eq!(s.v_degree(), 0);
}

#[test]
fn default_empty_poles_raw_is_empty() {
    assert!(ConvBSplineSurface::default_empty().poles_raw().is_empty());
}

// ---------------------------------------------------------------------------
// ConvSurfaceToBSpline — Sphere
// ---------------------------------------------------------------------------

use ironstream::conv_surface::{
    ConvSurfaceContinuity, ConvSurfaceError, ConvSurfaceParams, ConvSurfaceToBSpline,
    ConvSurfaceType,
};

#[test]
fn sphere_perform_sets_done() {
    let params = ConvSurfaceParams::default();
    let mut conv = ConvSurfaceToBSpline::new(ConvSurfaceType::Sphere, params);
    assert!(!conv.is_done());
    conv.perform();
    assert!(conv.is_done());
}

#[test]
fn sphere_perform_poles_and_degree() {
    let params = ConvSurfaceParams::default();
    let mut conv = ConvSurfaceToBSpline::new(ConvSurfaceType::Sphere, params);
    conv.perform();
    assert_eq!(conv.nb_u_poles(), 4);
    assert_eq!(conv.nb_v_poles(), 4);
    assert_eq!(conv.u_degree(), 3);
    assert_eq!(conv.v_degree(), 3);
}

#[test]
fn sphere_is_u_rational_true() {
    let params = ConvSurfaceParams::default();
    let mut conv = ConvSurfaceToBSpline::new(ConvSurfaceType::Sphere, params);
    conv.perform();
    assert!(conv.is_u_rational());
}

#[test]
fn sphere_is_v_rational_false() {
    let params = ConvSurfaceParams::default();
    let mut conv = ConvSurfaceToBSpline::new(ConvSurfaceType::Sphere, params);
    conv.perform();
    assert!(!conv.is_v_rational());
}

#[test]
fn cylinder_u_rational_v_not() {
    let params = ConvSurfaceParams::default();
    let mut conv = ConvSurfaceToBSpline::new(ConvSurfaceType::Cylinder, params);
    conv.perform();
    assert!(conv.is_u_rational());
    assert!(!conv.is_v_rational());
}

#[test]
fn cone_u_rational_v_not() {
    let params = ConvSurfaceParams::default();
    let mut conv = ConvSurfaceToBSpline::new(ConvSurfaceType::Cone, params);
    conv.perform();
    assert!(conv.is_u_rational());
    assert!(!conv.is_v_rational());
}

#[test]
fn torus_u_rational_v_not() {
    let params = ConvSurfaceParams::default();
    let mut conv = ConvSurfaceToBSpline::new(ConvSurfaceType::Torus, params);
    conv.perform();
    assert!(conv.is_u_rational());
    assert!(!conv.is_v_rational());
}

#[test]
fn plane_neither_rational() {
    let params = ConvSurfaceParams::default();
    let mut conv = ConvSurfaceToBSpline::new(ConvSurfaceType::Plane, params);
    conv.perform();
    assert!(!conv.is_u_rational());
    assert!(!conv.is_v_rational());
}

#[test]
fn surface_of_extrusion_neither_rational() {
    let params = ConvSurfaceParams::default();
    let mut conv = ConvSurfaceToBSpline::new(ConvSurfaceType::SurfaceOfExtrusion, params);
    conv.perform();
    assert!(!conv.is_u_rational());
    assert!(!conv.is_v_rational());
}

#[test]
fn conv_surface_params_default_values() {
    let p = ConvSurfaceParams::default();
    assert_eq!(p.u_continuity, 2);
    assert_eq!(p.v_continuity, 2);
    assert!((p.tolerance - 1e-7).abs() < 1e-15);
    assert_eq!(p.max_degree, 14);
    assert_eq!(p.max_segments, 10000);
}

#[test]
fn continuity_u_degree_3() {
    assert_eq!(ConvSurfaceContinuity::u_continuity(3, 10), 2);
}

#[test]
fn continuity_v_degree_1() {
    assert_eq!(ConvSurfaceContinuity::v_continuity(1, 5), 0);
}

#[test]
fn continuity_degree_0_clamps() {
    assert_eq!(ConvSurfaceContinuity::u_continuity(0, 4), 0);
    assert_eq!(ConvSurfaceContinuity::v_continuity(0, 4), 0);
}

#[test]
fn max_degree_for_continuity_values() {
    assert_eq!(ConvSurfaceContinuity::max_degree_for_continuity(0), 1);
    assert_eq!(ConvSurfaceContinuity::max_degree_for_continuity(1), 2);
    assert_eq!(ConvSurfaceContinuity::max_degree_for_continuity(2), 3);
    assert_eq!(ConvSurfaceContinuity::max_degree_for_continuity(5), 8);
}

#[test]
fn error_enum_debug_and_eq() {
    let e = ConvSurfaceError::NotDone;
    assert_eq!(e, ConvSurfaceError::NotDone);
    assert_ne!(e, ConvSurfaceError::DegenerateSurface);
    let _ = format!("{:?}", ConvSurfaceError::SingularSurface);
    let _ = format!("{:?}", ConvSurfaceError::ExceededDegree);
}
