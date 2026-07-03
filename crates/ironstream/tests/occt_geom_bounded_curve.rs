extern crate ironstream;
use ironstream::geom_bounded_curve::*;

#[test]
fn test_geom_bounded_curve_basic() {
    let bc = BoundedCurve::new([0.0, 0.0, 0.0], [3.0, 4.0, 0.0]);
    assert!((bc.length() - 5.0).abs() < 1e-10);
    assert!(!bc.is_closed());
    assert!((bc.first_param - 0.0).abs() < 1e-10);
    assert!((bc.last_param - 1.0).abs() < 1e-10);

    // Closed curve
    let closed = BoundedCurve::new([0.0, 0.0, 0.0], [0.0, 0.0, 0.0]);
    assert!(closed.is_closed());
}

#[test]
fn test_trimmed_curve_basic() {
    let mut tc = TrimmedCurve::new("line", 0.0, 1.0);
    assert!((tc.first - 0.0).abs() < 1e-10);
    assert!((tc.last - 1.0).abs() < 1e-10);

    let p = tc.evaluate(0.5);
    let _ = tc.d1(0.5);

    tc.reverse();
    assert!(tc.first < tc.last);

    let rp = tc.reversed_param(0.3);
    // should be first + last - 0.3
    let _ = rp;

    let t2 = trim_curve("arc", 1.0, 0.0); // reversed args should be normalised
    assert!(t2.first <= t2.last);
    let _ = p;
}
