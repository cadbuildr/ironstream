extern crate ironstream;
use ironstream::geom_api_ext::*;

#[test]
fn test_geom_api_ext_basic() {
    // ExtremaCurveCurveApi
    let ex = ExtremaCurveCurveApi::new("curve_a", "curve_b");
    assert_eq!(ex.c1, "curve_a");
    assert_eq!(ex.c2, "curve_b");
    let nb = ex.nb_extrema();
    let _ = nb;

    // lower_distance returns INFINITY when no solutions
    if ex.solutions.is_empty() {
        assert!(ex.lower_distance().is_infinite());
    } else {
        assert!(ex.lower_distance() >= 0.0);
    }

    // out-of-range returns NAN
    let d = ex.distance(9999);
    assert!(d.is_nan());
    let (t1, t2) = ex.parameters(9999);
    assert!(t1.is_nan() && t2.is_nan());

    // ExtremaCurveSurfaceApi
    let exs = ExtremaCurveSurfaceApi::new("my_curve", "my_surface");
    assert_eq!(exs.curve, "my_curve");
    assert_eq!(exs.surface, "my_surface");
    let _nb2 = exs.nb_extrema();
    let _ld = exs.lower_distance();

    // free function
    let d2 = extrema_distance("c1", "c2");
    let _ = d2;
}
