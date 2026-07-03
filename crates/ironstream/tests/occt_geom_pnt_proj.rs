extern crate ironstream;
use ironstream::geom_pnt_proj::*;

#[test]
fn test_geom_pnt_proj_basic() {
    // ProjectOnCurve
    let poc = ProjectOnCurve::new([1.0, 0.0, 0.0], "line_x");
    assert!(poc.is_done());
    assert!(poc.nb_points() > 0);
    let _pt = poc.nearest_point();
    let _t = poc.lower_distance_parameter();
    let d = poc.lower_distance();
    assert!(d >= 0.0);

    // empty curve name — done field may vary
    let poc2 = ProjectOnCurve::new([0.0, 0.0, 0.0], "");
    let _ = poc2.nb_points();

    // ProjectOnSurface
    let pos = ProjectOnSurface::new([1.0, 2.0, 3.0], "plane_xy");
    assert!(pos.is_done());
    assert!(pos.nb_points() > 0);
    let _pt2 = pos.nearest_point();
    let uv = pos.lower_distance_parameter();
    assert_eq!(uv.len(), 2);
    let d2 = pos.lower_distance();
    assert!(d2 >= 0.0);

    // free function
    let (t, pt3) = project_point_on_curve([0.5, 0.0, 0.0], "my_curve");
    let _ = t;
    let _ = pt3;
}
