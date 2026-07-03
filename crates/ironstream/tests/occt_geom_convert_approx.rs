// FILE: rust/ironstream/crates/ironstream/tests/occt_geom_convert_approx.rs
//! Tests for the `geom_convert_approx` module.

extern crate ironstream;
use ironstream::geom_convert_approx::*;

// ---------------------------------------------------------------------------
// GeomConvertContinuity
// ---------------------------------------------------------------------------

#[test]
fn continuity_ordering() {
    // C0 < G1 < C1 < G2 < C2 < C3 < CN per OCCT convention.
    assert!(GeomConvertContinuity::C0 < GeomConvertContinuity::G1);
    assert!(GeomConvertContinuity::G1 < GeomConvertContinuity::C1);
    assert!(GeomConvertContinuity::C1 < GeomConvertContinuity::G2);
    assert!(GeomConvertContinuity::G2 < GeomConvertContinuity::C2);
    assert!(GeomConvertContinuity::C2 < GeomConvertContinuity::C3);
    assert!(GeomConvertContinuity::C3 < GeomConvertContinuity::CN);
}

#[test]
fn continuity_min_multiplicity_c0() {
    // For C0 the required knot multiplicity equals the degree.
    assert_eq!(GeomConvertContinuity::C0.min_multiplicity_for_split(3), 3);
    assert_eq!(GeomConvertContinuity::C0.min_multiplicity_for_split(1), 1);
    assert_eq!(GeomConvertContinuity::C0.min_multiplicity_for_split(5), 5);
}

#[test]
fn continuity_min_multiplicity_c1() {
    assert_eq!(GeomConvertContinuity::C1.min_multiplicity_for_split(3), 2);
    assert_eq!(GeomConvertContinuity::C1.min_multiplicity_for_split(1), 0);
}

#[test]
fn continuity_min_multiplicity_g1_same_as_c1() {
    for degree in 0u32..=5 {
        assert_eq!(
            GeomConvertContinuity::G1.min_multiplicity_for_split(degree),
            GeomConvertContinuity::C1.min_multiplicity_for_split(degree),
        );
    }
}

#[test]
fn continuity_min_multiplicity_c2() {
    assert_eq!(GeomConvertContinuity::C2.min_multiplicity_for_split(3), 1);
    assert_eq!(GeomConvertContinuity::C2.min_multiplicity_for_split(2), 0);
}

#[test]
fn continuity_min_multiplicity_g2_same_as_c2() {
    for degree in 0u32..=5 {
        assert_eq!(
            GeomConvertContinuity::G2.min_multiplicity_for_split(degree),
            GeomConvertContinuity::C2.min_multiplicity_for_split(degree),
        );
    }
}

#[test]
fn continuity_min_multiplicity_c3() {
    assert_eq!(GeomConvertContinuity::C3.min_multiplicity_for_split(4), 1);
    assert_eq!(GeomConvertContinuity::C3.min_multiplicity_for_split(3), 0);
}

#[test]
fn continuity_min_multiplicity_cn_is_zero() {
    for degree in 0u32..=6 {
        assert_eq!(GeomConvertContinuity::CN.min_multiplicity_for_split(degree), 0);
    }
}

// ---------------------------------------------------------------------------
// GeomConvertBSplineCurveKnotSplitting
// ---------------------------------------------------------------------------

#[test]
fn curve_split_new_not_done() {
    let s = GeomConvertBSplineCurveKnotSplitting::new();
    assert!(!s.is_done());
    assert_eq!(s.nb_splits(), 0);
}

#[test]
fn curve_split_default_not_done() {
    let s = GeomConvertBSplineCurveKnotSplitting::default();
    assert!(!s.is_done());
}

#[test]
fn curve_split_zero_poles_is_done_no_splits() {
    let mut s = GeomConvertBSplineCurveKnotSplitting::new();
    s.compute(0, 3, GeomConvertContinuity::C0);
    assert!(s.is_done());
    assert_eq!(s.nb_splits(), 0);
}

#[test]
fn curve_split_zero_degree_is_done() {
    let mut s = GeomConvertBSplineCurveKnotSplitting::new();
    s.compute(5, 0, GeomConvertContinuity::C0);
    assert!(s.is_done());
    assert_eq!(s.nb_splits(), 0);
}

#[test]
fn curve_split_cn_no_splits() {
    let mut s = GeomConvertBSplineCurveKnotSplitting::new();
    s.compute(10, 3, GeomConvertContinuity::CN);
    assert!(s.is_done());
    assert_eq!(s.nb_splits(), 0);
}

#[test]
fn curve_split_c0_produces_interior_indices() {
    let mut s = GeomConvertBSplineCurveKnotSplitting::new();
    // nb_poles=8, degree=3 => flat_len=12; interior range [4..8)
    s.compute(8, 3, GeomConvertContinuity::C0);
    assert!(s.is_done());
    // All returned indices must be in the interior range (degree+1 .. nb_poles).
    for &idx in &s.split_knots {
        assert!(idx >= 4, "index {idx} below interior start");
        assert!(idx < 8, "index {idx} at or above nb_poles");
    }
}

#[test]
fn curve_split_c0_split_knot_accessor() {
    let mut s = GeomConvertBSplineCurveKnotSplitting::new();
    s.compute(6, 3, GeomConvertContinuity::C0);
    assert!(s.is_done());
    let n = s.nb_splits();
    for i in 0..n {
        let _ = s.split_knot(i); // must not panic
    }
}

#[test]
fn curve_split_c0_small_curve_no_interior() {
    // nb_poles=4, degree=3: flat_len=8; interior range [4..4) — empty.
    let mut s = GeomConvertBSplineCurveKnotSplitting::new();
    s.compute(4, 3, GeomConvertContinuity::C0);
    assert!(s.is_done());
    assert_eq!(s.nb_splits(), 0);
}

#[test]
fn curve_split_repeated_compute_resets() {
    let mut s = GeomConvertBSplineCurveKnotSplitting::new();
    s.compute(10, 3, GeomConvertContinuity::C0);
    let first_n = s.nb_splits();
    // Compute again with fewer poles — result must be independent.
    s.compute(4, 3, GeomConvertContinuity::C0);
    assert_eq!(s.nb_splits(), 0);
    // Restore
    s.compute(10, 3, GeomConvertContinuity::C0);
    assert_eq!(s.nb_splits(), first_n);
}

// ---------------------------------------------------------------------------
// GeomConvertBSplineSurfaceKnotSplitting
// ---------------------------------------------------------------------------

#[test]
fn surface_split_new_not_done() {
    let s = GeomConvertBSplineSurfaceKnotSplitting::new();
    assert!(!s.is_done());
    assert_eq!(s.nb_u_splits(), 0);
    assert_eq!(s.nb_v_splits(), 0);
}

#[test]
fn surface_split_default_not_done() {
    let s = GeomConvertBSplineSurfaceKnotSplitting::default();
    assert!(!s.is_done());
}

#[test]
fn surface_split_is_done_after_compute() {
    let mut s = GeomConvertBSplineSurfaceKnotSplitting::new();
    s.compute(6, 6, 3);
    assert!(s.is_done());
}

#[test]
fn surface_split_no_interior_u() {
    // U: nb_u=4, degree=3 => no interior knots.
    let mut s = GeomConvertBSplineSurfaceKnotSplitting::new();
    s.compute(4, 8, 3);
    assert!(s.is_done());
    assert_eq!(s.nb_u_splits(), 0);
    // V should have interior knots.
    assert!(s.nb_v_splits() > 0);
}

#[test]
fn surface_split_no_interior_v() {
    // V: nb_v=4, degree=3 => no interior knots.
    let mut s = GeomConvertBSplineSurfaceKnotSplitting::new();
    s.compute(8, 4, 3);
    assert!(s.is_done());
    assert_eq!(s.nb_v_splits(), 0);
    assert!(s.nb_u_splits() > 0);
}

#[test]
fn surface_split_symmetric_uv() {
    // Same nb poles in U and V should yield the same count.
    let mut s = GeomConvertBSplineSurfaceKnotSplitting::new();
    s.compute(8, 8, 3);
    assert_eq!(s.nb_u_splits(), s.nb_v_splits());
}

#[test]
fn surface_split_u_indices_in_range() {
    let nb_u: usize = 8;
    let degree: u32 = 3;
    let mut s = GeomConvertBSplineSurfaceKnotSplitting::new();
    s.compute(nb_u, 8, degree);
    for &idx in &s.u_split_knots {
        assert!(idx >= degree as usize + 1);
        assert!(idx < nb_u);
    }
}

#[test]
fn surface_split_v_indices_in_range() {
    let nb_v: usize = 10;
    let degree: u32 = 3;
    let mut s = GeomConvertBSplineSurfaceKnotSplitting::new();
    s.compute(8, nb_v, degree);
    for &idx in &s.v_split_knots {
        assert!(idx >= degree as usize + 1);
        assert!(idx < nb_v);
    }
}

#[test]
fn surface_split_repeated_compute_resets() {
    let mut s = GeomConvertBSplineSurfaceKnotSplitting::new();
    s.compute(10, 10, 3);
    let u_first = s.nb_u_splits();
    s.compute(4, 4, 3);
    assert_eq!(s.nb_u_splits(), 0);
    assert_eq!(s.nb_v_splits(), 0);
    s.compute(10, 10, 3);
    assert_eq!(s.nb_u_splits(), u_first);
}

// ---------------------------------------------------------------------------
// GeomConvertSampling
// ---------------------------------------------------------------------------

#[test]
fn sampling_accessors() {
    let s = GeomConvertSampling::new(1e-3, 10);
    assert!((s.tolerance() - 1e-3).abs() < 1e-15);
    assert_eq!(s.nb_points(), 10);
}

#[test]
fn sampling_zero_points_empty() {
    let s = GeomConvertSampling::new(1e-3, 0);
    let params = s.parameters(0.0, 1.0);
    assert!(params.is_empty());
}

#[test]
fn sampling_one_point_is_midpoint() {
    let s = GeomConvertSampling::new(1e-3, 1);
    let params = s.parameters(2.0, 4.0);
    assert_eq!(params.len(), 1);
    assert!((params[0] - 3.0).abs() < 1e-12);
}

#[test]
fn sampling_two_points_are_endpoints() {
    let s = GeomConvertSampling::new(1e-3, 2);
    let params = s.parameters(1.0, 5.0);
    assert_eq!(params.len(), 2);
    assert!((params[0] - 1.0).abs() < 1e-12);
    assert!((params[1] - 5.0).abs() < 1e-12);
}

#[test]
fn sampling_five_points_evenly_spaced() {
    let s = GeomConvertSampling::new(1e-6, 5);
    let params = s.parameters(0.0, 1.0);
    assert_eq!(params.len(), 5);
    let expected = [0.0, 0.25, 0.5, 0.75, 1.0];
    for (p, e) in params.iter().zip(expected.iter()) {
        assert!((p - e).abs() < 1e-12, "got {p}, expected {e}");
    }
}

#[test]
fn sampling_start_equals_end_all_same() {
    let s = GeomConvertSampling::new(1e-3, 4);
    let params = s.parameters(3.0, 3.0);
    assert_eq!(params.len(), 4);
    for p in &params {
        assert!((p - 3.0).abs() < 1e-12);
    }
}

#[test]
fn sampling_negative_interval() {
    // u_start > u_end is allowed; produces a descending sequence.
    let s = GeomConvertSampling::new(1e-3, 3);
    let params = s.parameters(1.0, -1.0);
    assert_eq!(params.len(), 3);
    assert!((params[0] - 1.0).abs() < 1e-12);
    assert!((params[1] - 0.0).abs() < 1e-12);
    assert!((params[2] - (-1.0)).abs() < 1e-12);
}

#[test]
fn sampling_large_point_count() {
    let n: u32 = 1000;
    let s = GeomConvertSampling::new(1e-6, n);
    let params = s.parameters(0.0, 1.0);
    assert_eq!(params.len(), n as usize);
    assert!((params[0] - 0.0).abs() < 1e-12);
    assert!((params[n as usize - 1] - 1.0).abs() < 1e-12);
    // Monotonically increasing.
    for w in params.windows(2) {
        assert!(w[1] > w[0], "not monotone at {:?}", w);
    }
}
