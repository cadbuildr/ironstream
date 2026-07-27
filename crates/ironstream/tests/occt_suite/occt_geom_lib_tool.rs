use ironstream::geom_lib_tool::*;

#[test]
fn check_bspline_perform_valid() {
    let mut c = GeomLibCheckBSplineCurve::new(5);
    assert!(!c.is_done());
    c.perform();
    assert!(c.is_done());
    assert!(c.result().is_valid());
    assert_eq!(c.result(), GeomLibCheckResult::Valid);
}

#[test]
fn check_bspline_null_curve() {
    let mut c = GeomLibCheckBSplineCurve::new(0);
    c.perform();
    assert!(c.is_done());
    assert_eq!(c.result(), GeomLibCheckResult::NullLength);
    assert!(!c.result().is_valid());
}

#[test]
fn adjust_extremity_tangent_length() {
    let mut a = GeomLibAdjustExtremity::new();
    a.set_first([3.0, 4.0, 0.0], [0.0; 3]);
    assert!((a.first_tangent_length() - 5.0).abs() < 1e-10);
    a.set_last([0.0, 0.0, 1.0], [0.0; 3]);
    assert!((a.last_tangent_length() - 1.0).abs() < 1e-10);
}

#[test]
fn geom_lib_tool_point_on_surface() {
    let t = GeomLibTool::new(1);
    let p = t.point_on_surface(2.0, 3.0);
    assert_eq!(p, [2.0, 3.0, 0.0]);
}

#[test]
fn geom_lib_tool_same_parameter() {
    let t = GeomLibTool::new(1);
    assert!(t.same_parameter(1.0, 1.0 + 1e-10));
    assert!(!t.same_parameter(1.0, 1.0 + 1e-6));
}

#[test]
fn denominator_multiplier_value() {
    let m = GeomLibDenominatorMultiplier::new(3, vec![0.0, 0.5, 1.0], vec![4, 1, 4]);
    assert_eq!(m.nb_knots(), 3);
    assert!((m.value(0.25) - 1.0).abs() < 1e-12);
}
