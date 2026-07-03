extern crate ironstream;
use ironstream::geom_ellipse_tool::*;

#[test]
fn ellipse_basic_properties() {
    let e = MakeEllipse::new([0.0;3], [1.0,0.0,0.0], 5.0, 3.0);
    assert!(e.is_done());
    assert!((e.eccentricity() - 0.8).abs() < 1e-8);
    assert!((e.area() - std::f64::consts::PI * 15.0).abs() < 1e-8);
    assert!((e.parameter() - 9.0 / 5.0).abs() < 1e-8);
}

#[test]
fn ellipse_foci_symmetric() {
    let e = MakeEllipse::new([0.0;3], [1.0,0.0,0.0], 5.0, 3.0);
    let f1 = e.focus1();
    let f2 = e.focus2();
    let dist = (f1[0]*f1[0]+f1[1]*f1[1]+f1[2]*f1[2]).sqrt();
    assert!((dist - 4.0).abs() < 1e-8);
    assert!((f1[0]+f2[0]).abs() < 1e-8); // symmetric
}

#[test]
fn ellipse_point_at_extremes() {
    let e = MakeEllipse::new([0.0;3], [1.0,0.0,0.0], 3.0, 2.0);
    let p_right = e.point_at(0.0);
    assert!((p_right[0] - 3.0).abs() < 1e-8);
    let p_top = e.point_at(std::f64::consts::FRAC_PI_2);
    assert!((p_top[1].abs() - 2.0).abs() < 1e-8);
}

#[test]
fn ellipse_invalid_when_minor_gt_major() {
    let e = MakeEllipse::new([0.0;3], [1.0,0.0,0.0], 2.0, 5.0);
    assert!(!e.is_done());
}

#[test]
fn arc_of_ellipse_quarter() {
    let arc = MakeArcOfEllipse::new([0.0;3], [1.0,0.0,0.0], 4.0, 2.0, 0.0, 90.0);
    assert!(arc.is_done());
    let pts = arc.discretize(20);
    assert_eq!(pts.len(), 21);
    let len = arc.arc_length_approx(100);
    assert!(len > 0.0 && len < 20.0);
}
