// FILE: tests/occt_geom_curve_tool.rs
extern crate ironstream;
use ironstream::geom_curve_tool::*;

use core::f64::consts::PI;

// ---- GeomCurveType -----------------------------------------------------------

#[test]
fn all_curve_type_variants_distinct() {
    let variants = [
        GeomCurveType::Line,
        GeomCurveType::Circle,
        GeomCurveType::Ellipse,
        GeomCurveType::Hyperbola,
        GeomCurveType::Parabola,
        GeomCurveType::BezierCurve,
        GeomCurveType::BSplineCurve,
        GeomCurveType::OffsetCurve,
        GeomCurveType::OtherCurve,
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
fn curve_type_is_copy_and_clone() {
    let original = GeomCurveType::BSplineCurve;
    let copied = original;
    let cloned = original.clone();
    assert_eq!(original, copied);
    assert_eq!(original, cloned);
}

#[test]
fn curve_type_debug_format() {
    assert_eq!(format!("{:?}", GeomCurveType::Line), "Line");
    assert_eq!(format!("{:?}", GeomCurveType::Circle), "Circle");
    assert_eq!(format!("{:?}", GeomCurveType::Ellipse), "Ellipse");
    assert_eq!(format!("{:?}", GeomCurveType::Hyperbola), "Hyperbola");
    assert_eq!(format!("{:?}", GeomCurveType::Parabola), "Parabola");
    assert_eq!(format!("{:?}", GeomCurveType::BezierCurve), "BezierCurve");
    assert_eq!(format!("{:?}", GeomCurveType::BSplineCurve), "BSplineCurve");
    assert_eq!(format!("{:?}", GeomCurveType::OffsetCurve), "OffsetCurve");
    assert_eq!(format!("{:?}", GeomCurveType::OtherCurve), "OtherCurve");
}

// ---- GeomCurveInfo: construction and accessors --------------------------------

#[test]
fn info_new_stores_type_first_last() {
    let info = GeomCurveInfo::new(GeomCurveType::Ellipse, -1.0, 4.5);
    assert_eq!(info.curve_type(), GeomCurveType::Ellipse);
    assert!((info.first() + 1.0).abs() < 1e-15);
    assert!((info.last() - 4.5).abs() < 1e-15);
}

#[test]
fn info_defaults_are_zero() {
    let info = GeomCurveInfo::new(GeomCurveType::BSplineCurve, 0.0, 1.0);
    assert_eq!(info.degree(), 0);
    assert_eq!(info.nb_poles(), 0);
    assert_eq!(info.continuity(), 0);
}

#[test]
fn info_set_degree() {
    let mut info = GeomCurveInfo::new(GeomCurveType::BezierCurve, 0.0, 1.0);
    info.set_degree(5);
    assert_eq!(info.degree(), 5);
}

#[test]
fn info_set_nb_poles() {
    let mut info = GeomCurveInfo::new(GeomCurveType::BezierCurve, 0.0, 1.0);
    info.set_nb_poles(6);
    assert_eq!(info.nb_poles(), 6);
}

#[test]
fn info_set_continuity() {
    let mut info = GeomCurveInfo::new(GeomCurveType::Line, 0.0, 10.0);
    info.set_continuity(3);
    assert_eq!(info.continuity(), 3);
}

#[test]
fn info_multiple_setters_independent() {
    let mut info = GeomCurveInfo::new(GeomCurveType::BSplineCurve, 0.0, 1.0);
    info.set_degree(3);
    info.set_nb_poles(7);
    info.set_continuity(2);
    assert_eq!(info.degree(), 3);
    assert_eq!(info.nb_poles(), 7);
    assert_eq!(info.continuity(), 2);
}

// ---- GeomCurveInfo: is_closed -------------------------------------------------

#[test]
fn circle_full_2pi_is_closed() {
    let info = GeomCurveInfo::new(GeomCurveType::Circle, 0.0, 2.0 * PI);
    assert!(info.is_closed(), "full circle must be closed");
}

#[test]
fn ellipse_full_2pi_is_closed() {
    let info = GeomCurveInfo::new(GeomCurveType::Ellipse, 0.0, 2.0 * PI);
    assert!(info.is_closed(), "full ellipse must be closed");
}

#[test]
fn circle_partial_arc_not_closed() {
    let info = GeomCurveInfo::new(GeomCurveType::Circle, 0.0, PI);
    assert!(!info.is_closed());
}

#[test]
fn ellipse_partial_arc_not_closed() {
    let info = GeomCurveInfo::new(GeomCurveType::Ellipse, 0.0, PI);
    assert!(!info.is_closed());
}

#[test]
fn line_spanning_2pi_not_closed() {
    let info = GeomCurveInfo::new(GeomCurveType::Line, 0.0, 2.0 * PI);
    assert!(!info.is_closed());
}

#[test]
fn hyperbola_not_closed() {
    let info = GeomCurveInfo::new(GeomCurveType::Hyperbola, -10.0, 10.0);
    assert!(!info.is_closed());
}

#[test]
fn parabola_not_closed() {
    let info = GeomCurveInfo::new(GeomCurveType::Parabola, -5.0, 5.0);
    assert!(!info.is_closed());
}

#[test]
fn bspline_spanning_2pi_not_closed() {
    let info = GeomCurveInfo::new(GeomCurveType::BSplineCurve, 0.0, 2.0 * PI);
    assert!(!info.is_closed());
}

// ---- GeomCurveInfo: is_periodic -----------------------------------------------

#[test]
fn circle_is_periodic() {
    let info = GeomCurveInfo::new(GeomCurveType::Circle, 0.0, 0.5);
    assert!(info.is_periodic());
}

#[test]
fn ellipse_is_periodic() {
    let info = GeomCurveInfo::new(GeomCurveType::Ellipse, 0.0, 2.0);
    assert!(info.is_periodic());
}

#[test]
fn line_not_periodic() {
    let info = GeomCurveInfo::new(GeomCurveType::Line, 0.0, 5.0);
    assert!(!info.is_periodic());
}

#[test]
fn hyperbola_not_periodic() {
    let info = GeomCurveInfo::new(GeomCurveType::Hyperbola, -3.0, 3.0);
    assert!(!info.is_periodic());
}

#[test]
fn parabola_not_periodic() {
    let info = GeomCurveInfo::new(GeomCurveType::Parabola, 0.0, 1.0);
    assert!(!info.is_periodic());
}

#[test]
fn bezier_not_periodic() {
    let info = GeomCurveInfo::new(GeomCurveType::BezierCurve, 0.0, 1.0);
    assert!(!info.is_periodic());
}

#[test]
fn bspline_not_periodic() {
    let info = GeomCurveInfo::new(GeomCurveType::BSplineCurve, 0.0, 1.0);
    assert!(!info.is_periodic());
}

#[test]
fn offset_not_periodic() {
    let info = GeomCurveInfo::new(GeomCurveType::OffsetCurve, 0.0, 1.0);
    assert!(!info.is_periodic());
}

#[test]
fn other_not_periodic() {
    let info = GeomCurveInfo::new(GeomCurveType::OtherCurve, 0.0, 1.0);
    assert!(!info.is_periodic());
}

// ---- GeomCurveEvaluator: value -----------------------------------------------

#[test]
fn evaluator_exposes_info() {
    let info = GeomCurveInfo::new(GeomCurveType::Line, 0.0, 5.0);
    let ev = GeomCurveEvaluator::new(info);
    assert_eq!(ev.info().curve_type(), GeomCurveType::Line);
    assert!((ev.info().first() - 0.0).abs() < 1e-15);
    assert!((ev.info().last() - 5.0).abs() < 1e-15);
}

#[test]
fn line_value_x_equals_t() {
    let info = GeomCurveInfo::new(GeomCurveType::Line, 0.0, 20.0);
    let ev = GeomCurveEvaluator::new(info);
    for t in [0.0f64, 1.0, 5.5, 20.0] {
        let p = ev.value(t);
        assert!((p[0] - t).abs() < 1e-15, "t={} x={}", t, p[0]);
        assert!(p[1].abs() < 1e-15, "y should be 0, got {}", p[1]);
        assert!(p[2].abs() < 1e-15, "z should be 0, got {}", p[2]);
    }
}

#[test]
fn circle_value_at_zero_is_one_zero_zero() {
    let info = GeomCurveInfo::new(GeomCurveType::Circle, 0.0, 2.0 * PI);
    let ev = GeomCurveEvaluator::new(info);
    let p = ev.value(0.0);
    assert!((p[0] - 1.0).abs() < 1e-15);
    assert!(p[1].abs() < 1e-15);
    assert!(p[2].abs() < 1e-15);
}

#[test]
fn circle_value_at_quarter_pi_is_cos_sin() {
    let info = GeomCurveInfo::new(GeomCurveType::Circle, 0.0, 2.0 * PI);
    let ev = GeomCurveEvaluator::new(info);
    let t = PI / 4.0;
    let p = ev.value(t);
    let expected_x = t.cos();
    let expected_y = t.sin();
    assert!((p[0] - expected_x).abs() < 1e-15);
    assert!((p[1] - expected_y).abs() < 1e-15);
    assert!(p[2].abs() < 1e-15);
}

#[test]
fn circle_value_at_pi_is_minus_one_zero() {
    let info = GeomCurveInfo::new(GeomCurveType::Circle, 0.0, 2.0 * PI);
    let ev = GeomCurveEvaluator::new(info);
    let p = ev.value(PI);
    assert!((p[0] + 1.0).abs() < 1e-15, "cos(pi)=-1, got {}", p[0]);
    assert!(p[1].abs() < 1e-14, "sin(pi)~0, got {}", p[1]);
}

#[test]
fn circle_value_on_unit_circle_always_norm_one() {
    let info = GeomCurveInfo::new(GeomCurveType::Circle, 0.0, 2.0 * PI);
    let ev = GeomCurveEvaluator::new(info);
    for i in 0..=8 {
        let t = (i as f64) * PI / 4.0;
        let p = ev.value(t);
        let r2 = p[0] * p[0] + p[1] * p[1];
        assert!((r2 - 1.0).abs() < 1e-14, "r^2={} at t={}", r2, t);
    }
}

#[test]
fn ellipse_value_is_stub_t_t_zero() {
    let info = GeomCurveInfo::new(GeomCurveType::Ellipse, 0.0, 1.0);
    let ev = GeomCurveEvaluator::new(info);
    let p = ev.value(0.4);
    assert!((p[0] - 0.4).abs() < 1e-15);
    assert!((p[1] - 0.4).abs() < 1e-15);
    assert!(p[2].abs() < 1e-15);
}

#[test]
fn hyperbola_value_is_stub_t_t_zero() {
    let info = GeomCurveInfo::new(GeomCurveType::Hyperbola, -5.0, 5.0);
    let ev = GeomCurveEvaluator::new(info);
    let p = ev.value(3.0);
    assert!((p[0] - 3.0).abs() < 1e-15);
    assert!((p[1] - 3.0).abs() < 1e-15);
}

#[test]
fn parabola_value_is_stub() {
    let info = GeomCurveInfo::new(GeomCurveType::Parabola, 0.0, 5.0);
    let ev = GeomCurveEvaluator::new(info);
    let p = ev.value(1.5);
    assert!((p[0] - 1.5).abs() < 1e-15);
    assert!((p[1] - 1.5).abs() < 1e-15);
}

#[test]
fn offset_curve_value_is_stub() {
    let info = GeomCurveInfo::new(GeomCurveType::OffsetCurve, 0.0, 2.0);
    let ev = GeomCurveEvaluator::new(info);
    let p = ev.value(1.0);
    assert!((p[0] - 1.0).abs() < 1e-15);
    assert!((p[1] - 1.0).abs() < 1e-15);
}

// ---- GeomCurveEvaluator: d1 --------------------------------------------------

#[test]
fn line_d1_point_matches_value() {
    let info = GeomCurveInfo::new(GeomCurveType::Line, 0.0, 10.0);
    let ev = GeomCurveEvaluator::new(info);
    let t = 4.5;
    let (p, _) = ev.d1(t);
    let v = ev.value(t);
    assert!((p[0] - v[0]).abs() < 1e-15);
    assert!((p[1] - v[1]).abs() < 1e-15);
    assert!((p[2] - v[2]).abs() < 1e-15);
}

#[test]
fn line_d1_tangent_is_one_zero_zero() {
    let info = GeomCurveInfo::new(GeomCurveType::Line, 0.0, 5.0);
    let ev = GeomCurveEvaluator::new(info);
    for t in [0.0f64, 1.0, 3.0] {
        let (_, d) = ev.d1(t);
        assert!((d[0] - 1.0).abs() < 1e-15, "dx={}", d[0]);
        assert!(d[1].abs() < 1e-15, "dy={}", d[1]);
        assert!(d[2].abs() < 1e-15, "dz={}", d[2]);
    }
}

#[test]
fn circle_d1_point_matches_value() {
    let info = GeomCurveInfo::new(GeomCurveType::Circle, 0.0, 2.0 * PI);
    let ev = GeomCurveEvaluator::new(info);
    let t = 1.3;
    let (p, _) = ev.d1(t);
    let v = ev.value(t);
    assert!((p[0] - v[0]).abs() < 1e-15);
    assert!((p[1] - v[1]).abs() < 1e-15);
}

#[test]
fn circle_d1_tangent_perpendicular_to_radius() {
    // For the unit circle p=[cos(t),sin(t)], d=[−sin(t),cos(t)].
    // p · d = 0 always.
    let info = GeomCurveInfo::new(GeomCurveType::Circle, 0.0, 2.0 * PI);
    let ev = GeomCurveEvaluator::new(info);
    for i in 0..=8 {
        let t = (i as f64) * PI / 4.0;
        let (p, d) = ev.d1(t);
        let dot = p[0] * d[0] + p[1] * d[1] + p[2] * d[2];
        assert!(dot.abs() < 1e-14, "dot={} at t={}", dot, t);
    }
}

#[test]
fn circle_d1_tangent_at_zero_is_zero_one_zero() {
    let info = GeomCurveInfo::new(GeomCurveType::Circle, 0.0, 2.0 * PI);
    let ev = GeomCurveEvaluator::new(info);
    let (_, d) = ev.d1(0.0);
    assert!(d[0].abs() < 1e-15, "d[0]={}", d[0]);
    assert!((d[1] - 1.0).abs() < 1e-15, "d[1]={}", d[1]);
    assert!(d[2].abs() < 1e-15);
}

#[test]
fn circle_d1_tangent_unit_speed() {
    // |tangent| = |-sin(t), cos(t), 0| = 1 always
    let info = GeomCurveInfo::new(GeomCurveType::Circle, 0.0, 2.0 * PI);
    let ev = GeomCurveEvaluator::new(info);
    for i in 0..=8 {
        let t = (i as f64) * PI / 4.0;
        let (_, d) = ev.d1(t);
        let speed = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
        assert!((speed - 1.0).abs() < 1e-14, "speed={} at t={}", speed, t);
    }
}

#[test]
fn circle_d1_finite_difference() {
    let info = GeomCurveInfo::new(GeomCurveType::Circle, 0.0, 2.0 * PI);
    let ev = GeomCurveEvaluator::new(info);
    let t = 2.1;
    let h = 1e-7;
    let (_, d) = ev.d1(t);
    let p_fwd = ev.value(t + h);
    let p_bwd = ev.value(t - h);
    let fd_x = (p_fwd[0] - p_bwd[0]) / (2.0 * h);
    let fd_y = (p_fwd[1] - p_bwd[1]) / (2.0 * h);
    assert!((d[0] - fd_x).abs() < 1e-8, "dx err={}", (d[0] - fd_x).abs());
    assert!((d[1] - fd_y).abs() < 1e-8, "dy err={}", (d[1] - fd_y).abs());
}

#[test]
fn other_curve_d1_stub_tangent_is_one_one_zero() {
    for ct in [
        GeomCurveType::Ellipse,
        GeomCurveType::Hyperbola,
        GeomCurveType::Parabola,
        GeomCurveType::BezierCurve,
        GeomCurveType::BSplineCurve,
        GeomCurveType::OffsetCurve,
        GeomCurveType::OtherCurve,
    ] {
        let info = GeomCurveInfo::new(ct, 0.0, 1.0);
        let ev = GeomCurveEvaluator::new(info);
        let (_, d) = ev.d1(0.5);
        assert!((d[0] - 1.0).abs() < 1e-15, "{:?} d[0]={}", ct, d[0]);
        assert!((d[1] - 1.0).abs() < 1e-15, "{:?} d[1]={}", ct, d[1]);
        assert!(d[2].abs() < 1e-15, "{:?} d[2]={}", ct, d[2]);
    }
}

// ---- GeomCurveEvaluator: length ----------------------------------------------

#[test]
fn line_length_positive_interval() {
    let info = GeomCurveInfo::new(GeomCurveType::Line, 0.0, 10.0);
    let ev = GeomCurveEvaluator::new(info);
    assert!((ev.length(0.0, 10.0) - 10.0).abs() < 1e-15);
    assert!((ev.length(3.0, 7.0) - 4.0).abs() < 1e-15);
}

#[test]
fn line_length_reversed_interval_is_absolute() {
    let info = GeomCurveInfo::new(GeomCurveType::Line, 0.0, 10.0);
    let ev = GeomCurveEvaluator::new(info);
    assert!((ev.length(7.0, 3.0) - 4.0).abs() < 1e-15);
}

#[test]
fn line_length_zero_interval() {
    let info = GeomCurveInfo::new(GeomCurveType::Line, 0.0, 5.0);
    let ev = GeomCurveEvaluator::new(info);
    assert!((ev.length(2.5, 2.5) - 0.0).abs() < 1e-15);
}

#[test]
fn circle_length_half_turn() {
    let info = GeomCurveInfo::new(GeomCurveType::Circle, 0.0, 2.0 * PI);
    let ev = GeomCurveEvaluator::new(info);
    let l = ev.length(0.0, PI);
    assert!((l - PI).abs() < 1e-15, "got {}", l);
}

#[test]
fn circle_length_full_turn() {
    let info = GeomCurveInfo::new(GeomCurveType::Circle, 0.0, 2.0 * PI);
    let ev = GeomCurveEvaluator::new(info);
    let l = ev.length(0.0, 2.0 * PI);
    assert!((l - 2.0 * PI).abs() < 1e-14, "got {}", l);
}

#[test]
fn bezier_length_is_span() {
    let info = GeomCurveInfo::new(GeomCurveType::BezierCurve, 0.0, 1.0);
    let ev = GeomCurveEvaluator::new(info);
    assert!((ev.length(0.0, 0.5) - 0.5).abs() < 1e-15);
    assert!((ev.length(0.1, 0.9) - 0.8).abs() < 1e-15);
}

// ---- GeomCurveEvaluator: curvature -------------------------------------------

#[test]
fn line_curvature_always_zero() {
    let info = GeomCurveInfo::new(GeomCurveType::Line, 0.0, 10.0);
    let ev = GeomCurveEvaluator::new(info);
    for t in [0.0f64, 1.0, 5.0, 10.0] {
        assert!((ev.curvature(t) - 0.0).abs() < 1e-15, "t={}", t);
    }
}

#[test]
fn circle_curvature_always_one() {
    let info = GeomCurveInfo::new(GeomCurveType::Circle, 0.0, 2.0 * PI);
    let ev = GeomCurveEvaluator::new(info);
    for t in [0.0f64, 0.3, 1.0, PI, 5.5] {
        assert!((ev.curvature(t) - 1.0).abs() < 1e-15, "t={}", t);
    }
}

#[test]
fn other_curves_curvature_is_zero() {
    for ct in [
        GeomCurveType::Ellipse,
        GeomCurveType::Hyperbola,
        GeomCurveType::Parabola,
        GeomCurveType::BezierCurve,
        GeomCurveType::BSplineCurve,
        GeomCurveType::OffsetCurve,
        GeomCurveType::OtherCurve,
    ] {
        let info = GeomCurveInfo::new(ct, 0.0, 1.0);
        let ev = GeomCurveEvaluator::new(info);
        assert!(
            ev.curvature(0.5).abs() < 1e-15,
            "{:?} curvature should be 0",
            ct
        );
    }
}

// ---- Copy / Clone / Debug ----------------------------------------------------

#[test]
fn info_copy_is_independent() {
    let mut info = GeomCurveInfo::new(GeomCurveType::Circle, 0.0, 2.0 * PI);
    info.set_degree(3);
    let info2 = info;
    assert_eq!(info2.degree(), 3);
    assert_eq!(info2.curve_type(), GeomCurveType::Circle);
}

#[test]
fn evaluator_copy_gives_same_results() {
    let info = GeomCurveInfo::new(GeomCurveType::Circle, 0.0, 2.0 * PI);
    let ev = GeomCurveEvaluator::new(info);
    let ev2 = ev;
    let p1 = ev.value(PI / 3.0);
    let p2 = ev2.value(PI / 3.0);
    assert!((p1[0] - p2[0]).abs() < 1e-15);
    assert!((p1[1] - p2[1]).abs() < 1e-15);
}

#[test]
fn info_debug_does_not_panic() {
    let info = GeomCurveInfo::new(GeomCurveType::Line, 0.0, 1.0);
    let _ = format!("{:?}", info);
}

#[test]
fn evaluator_debug_does_not_panic() {
    let info = GeomCurveInfo::new(GeomCurveType::Circle, 0.0, 2.0 * PI);
    let ev = GeomCurveEvaluator::new(info);
    let _ = format!("{:?}", ev);
}
