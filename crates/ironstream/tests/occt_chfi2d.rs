extern crate ironstream;
use ironstream::chfi2d::*;

#[test]
fn test_edge2d_line_length_and_direction() {
    let e = Edge2d::line(0, [0.0, 0.0], [4.0, 3.0]);
    assert!((e.length() - 5.0).abs() < 1e-10);
    let d = e.direction();
    assert!((d[0] - 0.8).abs() < 1e-10);
    assert!((d[1] - 0.6).abs() < 1e-10);
}

#[test]
fn test_fillet_algo2d_success() {
    let mut fa = FilletAlgo2d::new();
    fa.add_edge(Edge2d::line(0, [0.0, 0.0], [5.0, 0.0]));
    fa.add_edge(Edge2d::line(1, [5.0, 0.0], [5.0, 5.0]));
    let ok = fa.perform(0, 1.5);
    assert!(ok);
    assert!(fa.is_done);
    assert_eq!(fa.nb_results(), 1);
    assert!((fa.result(0).unwrap().radius - 1.5).abs() < 1e-10);
}

#[test]
fn test_fillet_algo2d_rejects_negative_radius() {
    let mut fa = FilletAlgo2d::new();
    assert!(!fa.perform(0, 0.0));
    assert_eq!(fa.error, ChFi2dError::BadParameters);
}

#[test]
fn test_ana_fillet_algo2d() {
    let mut afa = AnaFilletAlgo2d::new();
    afa.init(
        Edge2d::line(0, [0.0, 0.0], [10.0, 0.0]),
        Edge2d::line(1, [10.0, 0.0], [10.0, 8.0]),
    );
    assert!(afa.perform(0, 3.0));
    assert!(afa.is_done());
    let r = afa.result().unwrap();
    assert!((r.radius - 3.0).abs() < 1e-10);
}

#[test]
fn test_chamfer_api2d_symmetric() {
    let mut ca = ChamferAPI2d::new();
    ca.add_edge(Edge2d::line(0, [0.0, 0.0], [10.0, 0.0]));
    ca.add_edge(Edge2d::line(1, [10.0, 0.0], [10.0, 10.0]));
    assert!(ca.perform_equal(0, 2.5));
    let r = ca.result(0).unwrap();
    assert!((r.dist1 - 2.5).abs() < 1e-10);
    assert!((r.dist2 - 2.5).abs() < 1e-10);
}

#[test]
fn test_chfi2d_error_is_ok() {
    assert!(ChFi2dError::Ok.is_ok());
    assert!(!ChFi2dError::FilletRadiusTooLarge.is_ok());
}
