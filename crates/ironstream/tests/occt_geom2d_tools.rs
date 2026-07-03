// FILE: tests/occt_geom2d_tools.rs
//! Integration tests for `geom2d_tools` — mirrors the OCCT
//! `Geom2dAdaptor` / `Geom2dConvert` helper surface.

extern crate ironstream;
use ironstream::geom2d_tools::*;

// ---------------------------------------------------------------------------
// Geom2dContinuity
// ---------------------------------------------------------------------------

#[test]
fn continuity_variants_are_distinct() {
    let variants = [
        Geom2dContinuity::C0,
        Geom2dContinuity::G1,
        Geom2dContinuity::C1,
        Geom2dContinuity::G2,
        Geom2dContinuity::C2,
        Geom2dContinuity::C3,
        Geom2dContinuity::CN,
    ];
    for i in 0..variants.len() {
        for j in 0..variants.len() {
            if i == j {
                assert_eq!(variants[i], variants[j]);
            } else {
                assert_ne!(variants[i], variants[j]);
            }
        }
    }
}

#[test]
fn continuity_ordering_strict_ascending() {
    assert!(Geom2dContinuity::C0 < Geom2dContinuity::G1);
    assert!(Geom2dContinuity::G1 < Geom2dContinuity::C1);
    assert!(Geom2dContinuity::C1 < Geom2dContinuity::G2);
    assert!(Geom2dContinuity::G2 < Geom2dContinuity::C2);
    assert!(Geom2dContinuity::C2 < Geom2dContinuity::C3);
    assert!(Geom2dContinuity::C3 < Geom2dContinuity::CN);
}

#[test]
fn continuity_copy_clone() {
    let a = Geom2dContinuity::C2;
    let b = a;
    let c = a.clone();
    assert_eq!(a, b);
    assert_eq!(a, c);
}

#[test]
fn continuity_debug_does_not_panic() {
    let _ = format!("{:?}", Geom2dContinuity::CN);
}

// ---------------------------------------------------------------------------
// Geom2dCurveInfo — construction and accessors
// ---------------------------------------------------------------------------

#[test]
fn curve_info_new_stores_type_and_range() {
    let info = Geom2dCurveInfo::new("Circle", 0.0, 6.283185307);
    assert_eq!(info.curve_type(), "Circle");
    assert!((info.first() - 0.0).abs() < 1.0e-12);
    assert!((info.last() - 6.283185307).abs() < 1.0e-12);
}

#[test]
fn curve_info_default_degree_and_poles_are_zero() {
    let info = Geom2dCurveInfo::new("Line", -10.0, 10.0);
    assert_eq!(info.degree(), 0);
    assert_eq!(info.nb_poles(), 0);
}

#[test]
fn curve_info_default_continuity_is_c0() {
    let info = Geom2dCurveInfo::new("Parabola", 0.0, 1.0);
    assert_eq!(info.continuity(), Geom2dContinuity::C0);
}

#[test]
fn curve_info_set_continuity_g1() {
    let mut info = Geom2dCurveInfo::new("BSpline", 0.0, 1.0);
    info.set_continuity(Geom2dContinuity::G1);
    assert_eq!(info.continuity(), Geom2dContinuity::G1);
}

#[test]
fn curve_info_set_continuity_cn() {
    let mut info = Geom2dCurveInfo::new("Ellipse", 0.0, 6.283185);
    info.set_continuity(Geom2dContinuity::CN);
    assert_eq!(info.continuity(), Geom2dContinuity::CN);
}

#[test]
fn curve_info_set_continuity_multiple_times() {
    let mut info = Geom2dCurveInfo::new("BSpline", 0.0, 1.0);
    info.set_continuity(Geom2dContinuity::C1);
    info.set_continuity(Geom2dContinuity::C3);
    assert_eq!(info.continuity(), Geom2dContinuity::C3);
}

// ---------------------------------------------------------------------------
// Geom2dCurveInfo — is_closed
// ---------------------------------------------------------------------------

#[test]
fn curve_info_is_closed_open_curve() {
    let info = Geom2dCurveInfo::new("Line", -5.0, 5.0);
    assert!(!info.is_closed());
}

#[test]
fn curve_info_is_closed_identical_endpoints() {
    let info = Geom2dCurveInfo::new("Circle", 0.0, 0.0);
    assert!(info.is_closed());
}

#[test]
fn curve_info_is_closed_within_tolerance() {
    // 5e-11 < 1e-10 → closed.
    let info = Geom2dCurveInfo::new("Circle", 0.0, 5.0e-11);
    assert!(info.is_closed());
}

#[test]
fn curve_info_is_not_closed_just_outside_tolerance() {
    // 1.1e-10 > 1e-10 → open.
    let info = Geom2dCurveInfo::new("Circle", 0.0, 1.1e-10);
    assert!(!info.is_closed());
}

#[test]
fn curve_info_is_closed_negative_first_greater_last() {
    // |first - last| = 5e-11 → closed regardless of sign order.
    let info = Geom2dCurveInfo::new("Curve", 5.0e-11, 0.0);
    assert!(info.is_closed());
}

// ---------------------------------------------------------------------------
// Geom2dTool::total_length
// ---------------------------------------------------------------------------

#[test]
fn total_length_zero_segments_gives_zero() {
    assert_eq!(Geom2dTool::total_length(0, 10.0), 0.0);
}

#[test]
fn total_length_one_segment() {
    assert!((Geom2dTool::total_length(1, 3.7) - 3.7).abs() < 1.0e-12);
}

#[test]
fn total_length_many_segments() {
    assert!((Geom2dTool::total_length(10, 0.5) - 5.0).abs() < 1.0e-12);
}

#[test]
fn total_length_zero_avg_length() {
    assert_eq!(Geom2dTool::total_length(100, 0.0), 0.0);
}

#[test]
fn total_length_fractional_segments() {
    let result = Geom2dTool::total_length(7, 1.0 / 7.0);
    assert!((result - 1.0).abs() < 1.0e-12);
}

// ---------------------------------------------------------------------------
// Geom2dTool::parameter_range
// ---------------------------------------------------------------------------

#[test]
fn parameter_range_empty_on_zero_points() {
    let v = Geom2dTool::parameter_range(0.0, 1.0, 0);
    assert!(v.is_empty());
}

#[test]
fn parameter_range_single_point_returns_first() {
    let v = Geom2dTool::parameter_range(3.0, 9.0, 1);
    assert_eq!(v.len(), 1);
    assert!((v[0] - 3.0).abs() < 1.0e-12);
}

#[test]
fn parameter_range_two_points_are_endpoints() {
    let v = Geom2dTool::parameter_range(-1.0, 1.0, 2);
    assert_eq!(v.len(), 2);
    assert!((v[0] - (-1.0)).abs() < 1.0e-12);
    assert!((v[1] - 1.0).abs() < 1.0e-12);
}

#[test]
fn parameter_range_three_points_uniform() {
    let v = Geom2dTool::parameter_range(0.0, 2.0, 3);
    assert_eq!(v.len(), 3);
    assert!((v[0] - 0.0).abs() < 1.0e-12);
    assert!((v[1] - 1.0).abs() < 1.0e-12);
    assert!((v[2] - 2.0).abs() < 1.0e-12);
}

#[test]
fn parameter_range_five_points_linspace() {
    let v = Geom2dTool::parameter_range(0.0, 1.0, 5);
    assert_eq!(v.len(), 5);
    let expected = [0.0, 0.25, 0.5, 0.75, 1.0];
    for (a, b) in v.iter().zip(expected.iter()) {
        assert!((a - b).abs() < 1.0e-12, "got {a} expected {b}");
    }
}

#[test]
fn parameter_range_first_and_last_exact() {
    let first = -7.3;
    let last = 4.9;
    let v = Geom2dTool::parameter_range(first, last, 50);
    assert_eq!(v.len(), 50);
    assert!((v[0] - first).abs() < 1.0e-12);
    assert!((v[49] - last).abs() < 1.0e-12);
}

#[test]
fn parameter_range_is_monotonically_increasing() {
    let v = Geom2dTool::parameter_range(0.0, 1.0, 100);
    for i in 1..v.len() {
        assert!(v[i] > v[i - 1], "not monotone at index {i}");
    }
}

// ---------------------------------------------------------------------------
// Geom2dTool::reverse_parameter
// ---------------------------------------------------------------------------

#[test]
fn reverse_parameter_maps_first_to_last() {
    let (f, l) = (0.0, 10.0);
    assert!((Geom2dTool::reverse_parameter(f, f, l) - l).abs() < 1.0e-12);
}

#[test]
fn reverse_parameter_maps_last_to_first() {
    let (f, l) = (0.0, 10.0);
    assert!((Geom2dTool::reverse_parameter(l, f, l) - f).abs() < 1.0e-12);
}

#[test]
fn reverse_parameter_midpoint_fixed() {
    let (f, l) = (2.0, 8.0);
    let mid = (f + l) * 0.5;
    assert!((Geom2dTool::reverse_parameter(mid, f, l) - mid).abs() < 1.0e-12);
}

#[test]
fn reverse_parameter_arbitrary_value() {
    // first=1, last=5, t=2 → 1+5-2 = 4
    assert!((Geom2dTool::reverse_parameter(2.0, 1.0, 5.0) - 4.0).abs() < 1.0e-12);
}

#[test]
fn reverse_parameter_is_involution() {
    let (f, l, t) = (-3.0, 7.0, 1.5);
    let r = Geom2dTool::reverse_parameter(Geom2dTool::reverse_parameter(t, f, l), f, l);
    assert!((r - t).abs() < 1.0e-12);
}

#[test]
fn reverse_parameter_negative_range() {
    let (f, l) = (-10.0, -2.0);
    let t = -4.0;
    // -10 + (-2) - (-4) = -12 + 4 = -8
    assert!((Geom2dTool::reverse_parameter(t, f, l) - (-8.0)).abs() < 1.0e-12);
}

// ---------------------------------------------------------------------------
// Geom2dTool::curve_info_from_degree
// ---------------------------------------------------------------------------

#[test]
fn curve_info_from_degree_type_name_is_bspline() {
    let info = Geom2dTool::curve_info_from_degree(3, 7);
    assert_eq!(info.curve_type(), "BSpline");
}

#[test]
fn curve_info_from_degree_stores_degree() {
    let info = Geom2dTool::curve_info_from_degree(5, 10);
    assert_eq!(info.degree(), 5);
}

#[test]
fn curve_info_from_degree_stores_nb_poles() {
    let info = Geom2dTool::curve_info_from_degree(2, 9);
    assert_eq!(info.nb_poles(), 9);
}

#[test]
fn curve_info_from_degree_default_range_zero_one() {
    let info = Geom2dTool::curve_info_from_degree(1, 2);
    assert_eq!(info.first(), 0.0);
    assert_eq!(info.last(), 1.0);
}

#[test]
fn curve_info_from_degree_is_not_closed() {
    let info = Geom2dTool::curve_info_from_degree(3, 6);
    assert!(!info.is_closed());
}

#[test]
fn curve_info_from_degree_default_continuity_c0() {
    let info = Geom2dTool::curve_info_from_degree(4, 8);
    assert_eq!(info.continuity(), Geom2dContinuity::C0);
}

#[test]
fn curve_info_from_degree_continuity_can_be_overridden() {
    let mut info = Geom2dTool::curve_info_from_degree(3, 5);
    info.set_continuity(Geom2dContinuity::C2);
    assert_eq!(info.continuity(), Geom2dContinuity::C2);
}
