// FILE: tests/occt_int_ana_2d.rs
extern crate ironstream;
use ironstream::int_ana_2d::*;

fn near(a: f64, b: f64, tol: f64) {
    assert!(
        (a - b).abs() <= tol,
        "expected {a} ~= {b} within {tol} (diff {})",
        (a - b).abs()
    );
}

#[test]
fn line_line_x_axis_and_y_axis_intersect_at_origin() {
    let inter = IntAna2dAnaIntersection::perform_line_line(
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
    );
    assert!(inter.is_done());
    assert!(!inter.is_empty());
    assert!(!inter.is_parallel());
    assert_eq!(inter.nb_points(), 1);
    let p = inter.point(0).unwrap();
    near(p.x, 0.0, 1e-12);
    near(p.y, 0.0, 1e-12);
}

#[test]
fn line_line_diagonal_intersection() {
    // y=x: (0,0)+(1,1) and y=2-x: (2,0)+(-1,1) intersect at (1,1)
    let inter = IntAna2dAnaIntersection::perform_line_line(
        0.0, 0.0, 1.0, 1.0,
        2.0, 0.0, -1.0, 1.0,
    );
    assert!(inter.is_done());
    assert!(!inter.is_empty());
    let p = inter.point(0).unwrap();
    near(p.x, 1.0, 1e-12);
    near(p.y, 1.0, 1e-12);
}

#[test]
fn line_line_parallel_is_empty_and_flagged() {
    let inter = IntAna2dAnaIntersection::perform_line_line(
        0.0, 0.0, 1.0, 0.0,
        0.0, 3.0, 1.0, 0.0,
    );
    assert!(inter.is_done());
    assert!(inter.is_empty());
    assert!(inter.is_parallel());
    assert_eq!(inter.nb_points(), 0);
}

#[test]
fn line_circle_y0_through_unit_circle_gives_two_points() {
    // line y=0 (x-axis) through unit circle centred at origin
    let inter = IntAna2dAnaIntersection::perform_line_circle(
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 1.0,
    );
    assert!(inter.is_done());
    assert!(!inter.is_empty());
    assert_eq!(inter.nb_points(), 2);
    let mut xs: Vec<f64> = (0..2).map(|i| inter.point(i).unwrap().x).collect();
    xs.sort_by(|a, b| a.partial_cmp(b).unwrap());
    near(xs[0], -1.0, 1e-10);
    near(xs[1], 1.0, 1e-10);
}

#[test]
fn line_circle_y_axis_through_unit_circle_gives_two_points() {
    // line x=0 (y-axis) through unit circle centred at origin
    let inter = IntAna2dAnaIntersection::perform_line_circle(
        0.0, 0.0, 0.0, 1.0,
        0.0, 0.0, 1.0,
    );
    assert!(inter.is_done());
    assert_eq!(inter.nb_points(), 2);
    let mut ys: Vec<f64> = (0..2).map(|i| inter.point(i).unwrap().y).collect();
    ys.sort_by(|a, b| a.partial_cmp(b).unwrap());
    near(ys[0], -1.0, 1e-10);
    near(ys[1], 1.0, 1e-10);
}

#[test]
fn line_circle_tangent_at_top_gives_one_point() {
    // y=1 is tangent to unit circle at (0,1)
    let inter = IntAna2dAnaIntersection::perform_line_circle(
        0.0, 1.0, 1.0, 0.0,
        0.0, 0.0, 1.0,
    );
    assert!(inter.is_done());
    assert!(!inter.is_empty());
    assert_eq!(inter.nb_points(), 1);
    let p = inter.point(0).unwrap();
    near(p.x, 0.0, 1e-6);
    near(p.y, 1.0, 1e-6);
}

#[test]
fn line_circle_tangent_at_bottom_gives_one_point() {
    // y=-1 tangent to unit circle at (0,-1)
    let inter = IntAna2dAnaIntersection::perform_line_circle(
        0.0, -1.0, 1.0, 0.0,
        0.0, 0.0, 1.0,
    );
    assert!(inter.is_done());
    assert_eq!(inter.nb_points(), 1);
    let p = inter.point(0).unwrap();
    near(p.x, 0.0, 1e-6);
    near(p.y, -1.0, 1e-6);
}

#[test]
fn line_circle_no_intersection() {
    // y=2 misses unit circle entirely
    let inter = IntAna2dAnaIntersection::perform_line_circle(
        0.0, 2.0, 1.0, 0.0,
        0.0, 0.0, 1.0,
    );
    assert!(inter.is_done());
    assert!(inter.is_empty());
    assert_eq!(inter.nb_points(), 0);
}

#[test]
fn line_circle_offset_centre_two_points() {
    // Line y=3 through circle centred at (0,3) radius 2 → points at (±2, 3)
    let inter = IntAna2dAnaIntersection::perform_line_circle(
        0.0, 3.0, 1.0, 0.0,
        0.0, 3.0, 2.0,
    );
    assert!(inter.is_done());
    assert_eq!(inter.nb_points(), 2);
    let mut xs: Vec<f64> = (0..2).map(|i| inter.point(i).unwrap().x).collect();
    xs.sort_by(|a, b| a.partial_cmp(b).unwrap());
    near(xs[0], -2.0, 1e-10);
    near(xs[1], 2.0, 1e-10);
}

#[test]
fn conic_from_line_coefficients() {
    // Horizontal line through origin: dx=1, dy=0
    // D=dy=0, E=-dx=-1, F=-dy*0+dx*0=0
    let c = IntAna2dConic::from_line(0.0, 0.0, 1.0, 0.0);
    near(c.a, 0.0, 1e-15);
    near(c.b, 0.0, 1e-15);
    near(c.c, 0.0, 1e-15);
    near(c.d, 0.0, 1e-15);
    near(c.e, -1.0, 1e-15);
    near(c.f, 0.0, 1e-15);
}

#[test]
fn conic_from_circle_coefficients() {
    // Circle centred at (1,2) radius 1: A=1,B=0,C=1,D=-2,E=-4,F=1+4-1=4
    let c = IntAna2dConic::from_circle(1.0, 2.0, 1.0);
    near(c.a, 1.0, 1e-15);
    near(c.b, 0.0, 1e-15);
    near(c.c, 1.0, 1e-15);
    near(c.d, -2.0, 1e-15);
    near(c.e, -4.0, 1e-15);
    near(c.f, 4.0, 1e-15);
}

#[test]
fn intersection_point_fields_and_params() {
    let p = IntAna2dPoint::new(3.0, 4.0, 1.5, 2.5);
    near(p.x, 3.0, 1e-15);
    near(p.y, 4.0, 1e-15);
    near(p.param1, 1.5, 1e-15);
    near(p.param2, 2.5, 1e-15);
}
