extern crate ironstream;
use ironstream::geom_curve_extrema::*;

#[test]
fn test_geom_curve_extrema_basic() {
    let mut epc = ExtremaPtCurve::new([0.0, 0.0, 0.0], "line_1", 1e-7);
    assert!(!epc.is_done());
    epc.perform();
    assert!(epc.is_done());
    assert!(epc.nb_ext() > 0);

    let param = epc.parameter(0);
    assert!(param.is_some());

    let dist = epc.distance(0);
    assert!(dist.is_some());

    let foot = epc.point(0);
    assert!(foot.is_some());

    assert!(epc.parameter(999).is_none());
    assert!(epc.distance(999).is_none());

    let mut ecc = ExtremaCurveCurve::new("line_1", "line_2", 1e-7);
    assert!(!ecc.is_done());
    ecc.perform();
    assert!(ecc.is_done());
    assert!(ecc.nb_ext() > 0);

    let (t, foot_pt) = nearest_point_on_curve([1.0, 0.0, 0.0], "curve_1");
    assert!(t.is_finite());
    let _ = foot_pt;
}
