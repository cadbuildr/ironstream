extern crate ironstream;
use ironstream::convert_curve::*;

#[test]
fn test_convert_curve_basic() {
    // from_circle
    let c = ConicToBSpline::from_circle([0.0, 0.0, 0.0], 1.0);
    assert!(c.degree >= 2);
    assert!(!c.poles.is_empty());
    assert!(!c.knots.is_empty());
    assert!(!c.weights.is_empty());

    // from_ellipse
    let e = ConicToBSpline::from_ellipse([0.0, 0.0, 0.0], 2.0, 1.0);
    assert!(e.degree >= 2);
    assert!(!e.poles.is_empty());

    // circle_to_bspline wrapper
    let cb = circle_to_bspline([0.0, 0.0, 0.0], 5.0);
    assert!(!cb.poles.is_empty());

    // ellipse_to_bspline wrapper
    let eb = ellipse_to_bspline([0.0, 0.0, 0.0], 3.0, 1.5);
    assert!(!eb.poles.is_empty());

    // knot vector length check: nb_poles + degree + 1
    let nb = cb.poles.len();
    let d = cb.degree as usize;
    assert_eq!(cb.knots.len(), nb + d + 1);
}
