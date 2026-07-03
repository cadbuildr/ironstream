use ironstream::extrema_::*;

#[test]
fn extr_p_on_curv_basic() {
    let p = ExtrPOnCurv::new(0.5, [1.0, 2.0, 3.0]);
    assert!((p.parameter() - 0.5).abs() < 1e-12);
    assert_eq!(p.value(), [1.0, 2.0, 3.0]);
}

#[test]
fn extrema_ext_pc_basic() {
    let e = ExtremaExtPC::new([0.0,1.0,0.0], 1, 0.0, 1.0);
    assert!(e.is_done());
    assert_eq!(e.nb_ext(), 1);
    assert!(e.square_distance(1).is_some());
    assert!(e.square_distance(0).is_none());
}

#[test]
fn extrema_ext_ps_project() {
    let e = ExtremaExtPS::new([0.3, 0.4, 5.0], 1, 0.0, 1.0, 0.0, 1.0);
    assert!(e.is_done());
    let p = e.point(1).unwrap();
    assert!((p.u_parameter() - 0.3).abs() < 1e-10);
    assert!((p.v_parameter() - 0.4).abs() < 1e-10);
}

#[test]
fn extrema_ext_cc() {
    let e = ExtremaExtCC::new(1, 0.0, 1.0, 2, 0.0, 1.0);
    assert!(e.is_done());
    assert!(e.points(1).is_some());
    assert!(e.points(0).is_none());
}

#[test]
fn extrema_ext_ss_same() {
    let e = ExtremaExtSS::new(1, 1);
    assert!(e.is_done());
    assert!(e.is_same_surface());
}
