use ironstream::extrema_curve::*;

#[test]
fn ext_cc3d_status_default_not_done() {
    let s = ExtCC3dStatus::default();
    assert_eq!(s, ExtCC3dStatus::NotDone);
    assert!(!s.is_done());
}

#[test]
fn curve_pair_distance_calculation() {
    // Two points 5 units apart (3-4-0 triangle)
    let pair = CurvePair::new(0.0, [0.0, 0.0, 0.0], 1.0, [3.0, 4.0, 0.0]);
    assert!((pair.distance() - 5.0).abs() < 1e-10);
    assert!((pair.square_distance - 25.0).abs() < 1e-10);
}

#[test]
fn ext_cc3d_parallel_flag() {
    let mut ext = ExtCCurve3d::new(10, 20);
    assert!(!ext.is_parallel());
    ext.parallel = true;
    assert!(ext.is_parallel());
}

#[test]
fn ext_cc3d_lower_distance_multiple_solutions() {
    let mut ext = ExtCCurve3d::new(1, 2);
    ext.add_solution(CurvePair::new(0.0, [0.0, 0.0, 0.0], 1.0, [10.0, 0.0, 0.0]));
    ext.add_solution(CurvePair::new(0.5, [0.0, 0.0, 0.0], 0.5, [3.0, 4.0, 0.0]));
    ext.perform();
    assert_eq!(ext.nb_ext(), 2);
    assert!((ext.lower_distance() - 5.0).abs() < 1e-10);
    assert_eq!(ext.lower_distance_index(), Some(1));
}

#[test]
fn ext_pc3d_set_range() {
    let mut ext = ExtPCurve3d::new(5, [1.0, 2.0, 3.0]);
    ext.set_range(0.25, 0.75);
    assert_eq!(ext.t_range, [0.25, 0.75]);
    ext.perform();
    assert!(ext.is_done());
    assert_eq!(ext.nb_ext(), 0);
}

#[test]
fn ext_curve_parameters_roundtrip() {
    let p = ExtCurveParameters::new(99, 3.14, [10.0, 20.0, 30.0]);
    assert_eq!(p.curve_id, 99);
    assert!((p.parameter() - 3.14).abs() < 1e-10);
    assert_eq!(p.value(), [10.0, 20.0, 30.0]);
}
