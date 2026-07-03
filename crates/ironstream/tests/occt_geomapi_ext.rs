// FILE: tests/occt_geomapi_ext.rs
extern crate ironstream;

use ironstream::geomapi_ext::{
    GeomApiExtremaCurveCurve, GeomApiExtremaCurveSurface, GeomApiPointsToBSpline,
};

#[test]
fn extrema_cc_initial_state() {
    let e = GeomApiExtremaCurveCurve::new();
    assert!(!e.is_done());
    assert_eq!(e.nb_extrema(), 0);
    assert_eq!(e.lower_distance(), None);
}

#[test]
fn extrema_cc_perform_is_done() {
    let mut e = GeomApiExtremaCurveCurve::new();
    e.perform("line", "line");
    assert!(e.is_done());
}

#[test]
fn extrema_cc_perform_nb_extrema() {
    let mut e = GeomApiExtremaCurveCurve::new();
    e.perform("circle", "line");
    assert_eq!(e.nb_extrema(), 1);
}

#[test]
fn extrema_cc_lower_distance_after_perform() {
    let mut e = GeomApiExtremaCurveCurve::new();
    e.perform("line", "circle");
    assert_eq!(e.lower_distance(), Some(0.0));
}

#[test]
fn extrema_cc_points_zero_index_none() {
    let mut e = GeomApiExtremaCurveCurve::new();
    e.perform("line", "line");
    assert!(e.points(0).is_none());
}

#[test]
fn extrema_cc_points_out_of_range_none() {
    let mut e = GeomApiExtremaCurveCurve::new();
    e.perform("line", "line");
    assert!(e.points(2).is_none());
}

#[test]
fn extrema_cc_points_valid_index() {
    let mut e = GeomApiExtremaCurveCurve::new();
    e.perform("line", "line");
    let pair = e.points(1).expect("pair at index 1");
    assert_eq!(pair.distance, 0.0);
    assert_eq!(pair.point1, [0.0, 0.0, 0.0]);
    assert_eq!(pair.point2, [0.0, 0.0, 0.0]);
}

#[test]
fn extrema_cs_initial_state() {
    let e = GeomApiExtremaCurveSurface::new();
    assert!(!e.is_done());
    assert_eq!(e.nb_extrema(), 0);
    assert_eq!(e.lower_distance(), None);
}

#[test]
fn extrema_cs_perform() {
    let mut e = GeomApiExtremaCurveSurface::new();
    e.perform();
    assert!(e.is_done());
    assert_eq!(e.nb_extrema(), 1);
    assert_eq!(e.lower_distance(), Some(0.0));
}

#[test]
fn extrema_cs_point_1based() {
    let mut e = GeomApiExtremaCurveSurface::new();
    e.perform();
    let r = e.point(1).expect("result at index 1");
    assert_eq!(r.curve_param, 0.0);
    assert_eq!(r.surface_u, 0.0);
    assert_eq!(r.surface_v, 0.0);
    assert_eq!(r.distance, 0.0);
    assert!(e.point(0).is_none());
}

#[test]
fn pts_bspline_no_points_not_done() {
    let mut b = GeomApiPointsToBSpline::new(2, 5);
    b.perform();
    assert!(!b.is_done());
}

#[test]
fn pts_bspline_one_point_not_done() {
    let mut b = GeomApiPointsToBSpline::new(2, 5);
    b.add_point(0.0, 0.0, 0.0);
    b.perform();
    assert!(!b.is_done());
}

#[test]
fn pts_bspline_two_points_done() {
    let mut b = GeomApiPointsToBSpline::new(3, 8);
    b.add_point(0.0, 0.0, 0.0);
    b.add_point(1.0, 1.0, 1.0);
    b.perform();
    assert!(b.is_done());
    assert_eq!(b.nb_poles(), 2);
    assert_eq!(b.degree(), 3);
}

#[test]
fn pts_bspline_multiple_points() {
    let mut b = GeomApiPointsToBSpline::new(2, 6);
    for i in 0..5 {
        b.add_point(i as f64, (i * i) as f64, 0.0);
    }
    b.perform();
    assert!(b.is_done());
    assert_eq!(b.nb_points(), 5);
    assert_eq!(b.nb_poles(), 5);
    assert_eq!(b.degree(), 2);
}

#[test]
fn pts_bspline_set_continuity_tolerance() {
    let mut b = GeomApiPointsToBSpline::new(1, 3);
    b.set_continuity(0);
    b.set_tolerance(1e-3);
    b.add_point(-1.0, 0.0, 0.0);
    b.add_point(1.0, 0.0, 0.0);
    b.perform();
    assert!(b.is_done());
    assert_eq!(b.degree(), 1);
    assert_eq!(b.nb_poles(), 2);
}
