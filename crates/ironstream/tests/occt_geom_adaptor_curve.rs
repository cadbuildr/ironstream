// FILE: tests/occt_geom_adaptor_curve.rs
extern crate ironstream;
use ironstream::geom_adaptor_curve::GeomAdaptorCurve;
use ironstream::geom_circle::GeomCircle;
use ironstream::geom_ellipse::GeomEllipse;
use ironstream::geom_line::GeomLine;
use ironstream::gp::{Ax1, Ax3, Pnt};
use ironstream::geom_abs::CurveType as GeomAbs_CurveType;
use core::f64::consts::PI;

fn xy_circle(radius: f64) -> GeomCircle {
    let ax2 = Ax3::from_origin_normal(Pnt::new(0.0, 0.0, 0.0), Pnt::new(0.0, 0.0, 1.0), Pnt::new(1.0, 0.0, 0.0));
    GeomCircle::from_ax3(ax2, radius)
}

fn xy_ellipse(a: f64, b: f64) -> GeomEllipse {
    let ax2 = Ax3::from_origin_normal(Pnt::new(0.0, 0.0, 0.0), Pnt::new(0.0, 0.0, 1.0), Pnt::new(1.0, 0.0, 0.0));
    GeomEllipse::from_ax3(ax2, a, b)
}

fn line_along_x() -> GeomLine {
    GeomLine::new(Ax1::new(Pnt::new(0.0, 0.0, 0.0), Pnt::new(1.0, 0.0, 0.0)))
}

// ---- circle adapt and sample at pi -------------------------------------------

#[test]
fn adapt_circle_curve_type_is_circle() {
    let adaptor = GeomAdaptorCurve::from_circle(xy_circle(1.0), 0.0, 2.0 * PI);
    assert!(matches!(adaptor.curve_type(), GeomAbs_CurveType::Circle));
}

#[test]
fn adapt_circle_value_at_pi() {
    let r = 7.0;
    let adaptor = GeomAdaptorCurve::from_circle(xy_circle(r), 0.0, 2.0 * PI);
    let p = adaptor.value(PI);
    assert!((p[0] + r).abs() < 1e-11, "x should be -{} but got {}", r, p[0]);
    assert!(p[1].abs() < 1e-11, "y should be 0 but got {}", p[1]);
    assert!(p[2].abs() < 1e-11, "z should be 0 but got {}", p[2]);
}

#[test]
fn adapt_circle_value_at_quarter_turn() {
    let r = 4.0;
    let adaptor = GeomAdaptorCurve::from_circle(xy_circle(r), 0.0, 2.0 * PI);
    let p = adaptor.value(PI / 2.0);
    assert!(p[0].abs() < 1e-11, "x={}", p[0]);
    assert!((p[1] - r).abs() < 1e-11, "y={}", p[1]);
}

#[test]
fn adapt_circle_first_last_parameter() {
    let adaptor = GeomAdaptorCurve::from_circle(xy_circle(1.0), 0.25, 3.0);
    assert!((adaptor.first_parameter() - 0.25).abs() < 1e-15);
    assert!((adaptor.last_parameter() - 3.0).abs() < 1e-15);
}

#[test]
fn adapt_circle_d1_at_pi() {
    let r = 3.0;
    let adaptor = GeomAdaptorCurve::from_circle(xy_circle(r), 0.0, 2.0 * PI);
    let (p, d1) = adaptor.d1(PI);
    // P(PI) = (-r, 0, 0)
    assert!((p[0] + r).abs() < 1e-11);
    // P'(PI) = (0, -r, 0)  because -sin(PI)=0 cos(PI)=-1
    assert!(d1[0].abs() < 1e-11, "d1x={}", d1[0]);
    assert!((d1[1] + r).abs() < 1e-11, "d1y={}", d1[1]);
}

#[test]
fn adapt_circle_d2_at_pi() {
    let r = 3.0;
    let adaptor = GeomAdaptorCurve::from_circle(xy_circle(r), 0.0, 2.0 * PI);
    let (_, _, d2) = adaptor.d2(PI);
    // P''(PI) = (r, 0, 0)  because -cos(PI)=1
    assert!((d2[0] - r).abs() < 1e-11, "d2x={}", d2[0]);
    assert!(d2[1].abs() < 1e-11, "d2y={}", d2[1]);
}

#[test]
fn adapt_circle_is_periodic() {
    let adaptor = GeomAdaptorCurve::from_circle(xy_circle(1.0), 0.0, 2.0 * PI);
    assert!(adaptor.is_periodic());
    assert!((adaptor.period() - 2.0 * PI).abs() < 1e-12);
}

#[test]
fn adapt_circle_is_closed() {
    let adaptor = GeomAdaptorCurve::from_circle(xy_circle(1.0), 0.0, 2.0 * PI);
    assert!(adaptor.is_closed());
}

#[test]
fn adapt_circle_returns_circ_with_correct_radius() {
    let r = 11.0;
    let adaptor = GeomAdaptorCurve::from_circle(xy_circle(r), 0.0, 2.0 * PI);
    let circ = adaptor.circle().unwrap();
    assert!((circ.radius() - r).abs() < 1e-12);
}

// ---- ellipse adapt -----------------------------------------------------------

#[test]
fn adapt_ellipse_curve_type_is_ellipse() {
    let adaptor = GeomAdaptorCurve::from_ellipse(xy_ellipse(5.0, 3.0), 0.0, 2.0 * PI);
    assert!(matches!(adaptor.curve_type(), GeomAbs_CurveType::Ellipse));
}

#[test]
fn adapt_ellipse_value_at_pi() {
    let a = 5.0;
    let b = 3.0;
    let adaptor = GeomAdaptorCurve::from_ellipse(xy_ellipse(a, b), 0.0, 2.0 * PI);
    let p = adaptor.value(PI);
    // At PI: (-a, 0, 0)
    assert!((p[0] + a).abs() < 1e-11, "x={}", p[0]);
    assert!(p[1].abs() < 1e-11, "y={}", p[1]);
}

#[test]
fn adapt_ellipse_d1_finite_difference_check() {
    let adaptor = GeomAdaptorCurve::from_ellipse(xy_ellipse(4.0, 2.0), 0.0, 2.0 * PI);
    let u = 0.7;
    let h = 1e-6;
    let (_, d1) = adaptor.d1(u);
    let p_fwd = adaptor.value(u + h);
    let p_bwd = adaptor.value(u - h);
    let fd_x = (p_fwd[0] - p_bwd[0]) / (2.0 * h);
    let fd_y = (p_fwd[1] - p_bwd[1]) / (2.0 * h);
    assert!((d1[0] - fd_x).abs() < 1e-9, "dx err={}", (d1[0]-fd_x).abs());
    assert!((d1[1] - fd_y).abs() < 1e-9, "dy err={}", (d1[1]-fd_y).abs());
}

#[test]
fn adapt_ellipse_accessor_radii() {
    let a = 8.0;
    let b = 3.0;
    let adaptor = GeomAdaptorCurve::from_ellipse(xy_ellipse(a, b), 0.0, 2.0 * PI);
    let ell = adaptor.ellipse().unwrap();
    assert!((ell.major_radius() - a).abs() < 1e-12);
    assert!((ell.minor_radius() - b).abs() < 1e-12);
}

// ---- line adapt --------------------------------------------------------------

#[test]
fn adapt_line_curve_type_is_line() {
    let adaptor = GeomAdaptorCurve::from_line(line_along_x(), 0.0, 10.0);
    assert!(matches!(adaptor.curve_type(), GeomAbs_CurveType::Line));
}

#[test]
fn adapt_line_value_at_various_params() {
    let adaptor = GeomAdaptorCurve::from_line(line_along_x(), 0.0, 10.0);
    for t in [0.0f64, 1.0, 3.7, 10.0] {
        let p = adaptor.value(t);
        assert!((p[0] - t).abs() < 1e-12, "t={} x={}", t, p[0]);
        assert!(p[1].abs() < 1e-12);
        assert!(p[2].abs() < 1e-12);
    }
}

#[test]
fn adapt_line_not_periodic_not_closed() {
    let adaptor = GeomAdaptorCurve::from_line(line_along_x(), 0.0, 10.0);
    assert!(!adaptor.is_periodic());
    assert!(!adaptor.is_closed());
}

// ---- trimmed adapt -----------------------------------------------------------

#[test]
fn adapt_trimmed_circle_curve_type() {
    let c = GeomAdaptorCurve::from_circle(xy_circle(2.0), 0.0, 2.0 * PI);
    let trimmed = GeomAdaptorCurve::from_trimmed(c, PI / 4.0, 3.0 * PI / 4.0);
    assert!(matches!(trimmed.curve_type(), GeomAbs_CurveType::Circle));
}

#[test]
fn adapt_trimmed_parameter_range() {
    let c = GeomAdaptorCurve::from_circle(xy_circle(2.0), 0.0, 2.0 * PI);
    let trimmed = GeomAdaptorCurve::from_trimmed(c, 0.5, 2.5);
    assert!((trimmed.first_parameter() - 0.5).abs() < 1e-15);
    assert!((trimmed.last_parameter() - 2.5).abs() < 1e-15);
}

#[test]
fn adapt_trimmed_value_matches_base() {
    let r = 5.0;
    let c1 = GeomAdaptorCurve::from_circle(xy_circle(r), 0.0, 2.0 * PI);
    let c2 = GeomAdaptorCurve::from_circle(xy_circle(r), 0.0, 2.0 * PI);
    let trimmed = GeomAdaptorCurve::from_trimmed(c1, 0.0, PI);
    let u = PI / 3.0;
    let p_trim = trimmed.value(u);
    let p_base = c2.value(u);
    assert!((p_trim[0] - p_base[0]).abs() < 1e-12);
    assert!((p_trim[1] - p_base[1]).abs() < 1e-12);
    assert!((p_trim[2] - p_base[2]).abs() < 1e-12);
}

#[test]
fn adapt_trimmed_not_periodic() {
    let c = GeomAdaptorCurve::from_circle(xy_circle(1.0), 0.0, 2.0 * PI);
    let trimmed = GeomAdaptorCurve::from_trimmed(c, 0.0, PI);
    assert!(!trimmed.is_periodic());
}

#[test]
fn adapt_trimmed_circle_accessor_works() {
    let r = 3.0;
    let c = GeomAdaptorCurve::from_circle(xy_circle(r), 0.0, 2.0 * PI);
    let trimmed = GeomAdaptorCurve::from_trimmed(c, 0.0, PI);
    let circ = trimmed.circle().unwrap();
    assert!((circ.radius() - r).abs() < 1e-12);
}

// ---- wrong accessor returns None --------------------------------------------

#[test]
fn circle_adaptor_returns_none_for_wrong_accessors() {
    let adaptor = GeomAdaptorCurve::from_circle(xy_circle(1.0), 0.0, 2.0 * PI);
    assert!(adaptor.line().is_none());
    assert!(adaptor.ellipse().is_none());
    assert!(adaptor.bspline().is_none());
}

#[test]
fn line_adaptor_returns_none_for_wrong_accessors() {
    let adaptor = GeomAdaptorCurve::from_line(line_along_x(), 0.0, 1.0);
    assert!(adaptor.circle().is_none());
    assert!(adaptor.ellipse().is_none());
    assert!(adaptor.bspline().is_none());
}

// ---- offset circle correctness -----------------------------------------------

#[test]
fn offset_circle_value_at_zero_and_pi() {
    // Circle centred at (2, 3, 0) radius 1
    let ax2 = Ax3::from_origin_normal(Pnt::new(2.0, 3.0, 0.0), Pnt::new(0.0, 0.0, 1.0), Pnt::new(1.0, 0.0, 0.0));
    let c = GeomCircle::from_ax3(ax2, 1.0);
    let adaptor = GeomAdaptorCurve::from_circle(c, 0.0, 2.0 * PI);
    let p0 = adaptor.value(0.0);
    let p_pi = adaptor.value(PI);
    assert!((p0[0] - 3.0).abs() < 1e-11);
    assert!((p0[1] - 3.0).abs() < 1e-11);
    assert!((p_pi[0] - 1.0).abs() < 1e-11);
    assert!((p_pi[1] - 3.0).abs() < 1e-11);
}

// ---- arc length proxy via finite-difference integration ----------------------

#[test]
fn circle_arc_length_half_circle() {
    let r = 2.0;
    let adaptor = GeomAdaptorCurve::from_circle(xy_circle(r), 0.0, PI);
    // Numerically integrate |P'(u)| over [0, PI] using midpoint rule with 1000 steps
    let n = 1000usize;
    let h = PI / n as f64;
    let mut length = 0.0f64;
    for i in 0..n {
        let u = (i as f64 + 0.5) * h;
        let (_, d1) = adaptor.d1(u);
        let speed = (d1[0]*d1[0] + d1[1]*d1[1] + d1[2]*d1[2]).sqrt();
        length += speed * h;
    }
    let expected = PI * r; // half circumference
    assert!((length - expected).abs() < 1e-6, "arc length={} expected={}", length, expected);
}
