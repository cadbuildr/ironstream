extern crate ironstream;
use ironstream::geom_nurbs_convert::*;

#[test]
fn test_geom_nurbs_convert_basic() {
    let mut curve = NurbsCurve::new(3);
    curve.add_pole([0.0, 0.0, 0.0], 1.0);
    curve.add_pole([1.0, 1.0, 0.0], 1.0);
    curve.add_pole([2.0, 0.0, 0.0], 1.0);
    curve.add_pole([3.0, 0.0, 0.0], 1.0);
    let pt = curve.evaluate(0.5);
    assert!(pt[0].is_finite());

    let nc = curve_to_nurbs("circle");
    assert!(nc.degree >= 1);

    let ns = surface_to_nurbs("plane");
    assert!(ns.u_degree >= 1);
    assert!(ns.v_degree >= 1);
    let spt = ns.evaluate(0.5, 0.5);
    assert!(spt[0].is_finite());
}
