extern crate ironstream;
use ironstream::extrema_points::*;

#[test]
fn test_p_on_surf_parameter() {
    let p = POnSurf::new([1.0, 2.0, 3.0], 0.3, 0.7);
    assert_eq!(p.parameter(), (0.3, 0.7));
    assert!((p.point[2] - 3.0).abs() < 1e-10);
}

#[test]
fn test_ext_pc_exact_hit_zero_distance() {
    let mut ext = ExtPC::new([2.0, 0.0, 0.0], 0.0, 4.0, 1e-7);
    ext.perform([2.0, 0.0, 0.0], 2.0);
    assert!(ext.is_done());
    assert!(ext.square_distance(0) < 1e-14);
    assert!(ext.is_min(0));
}

#[test]
fn test_ext_pc_linear_scan() {
    let mut ext = ExtPC::new([0.5, 0.0, 0.0], 0.0, 1.0, 1e-7);
    ext.perform_linear(21);
    assert!(ext.is_done());
    assert_eq!(ext.nb_ext(), 1);
    let poc = ext.point(0).unwrap();
    assert!((poc.parameter - 0.5).abs() < 0.06); // step is 0.05
}

#[test]
fn test_ext_ss_midpoint_solution() {
    let mut ext = ExtSS::new(
        0, 1,
        (0.0, 1.0), (0.0, 1.0),
        (3.0, 4.0), (0.0, 1.0),
        1e-7,
    );
    ext.perform();
    assert!(ext.is_done());
    let d2 = ext.square_distance(0);
    assert!(d2 > 0.0);
    let p1 = ext.points_on_s1(0).unwrap();
    assert!((p1.u - 0.5).abs() < 1e-10);
}

#[test]
fn test_ext_cs_has_solution() {
    let mut ext = ExtCS::new(0, 1, (0.0, 1.0), [0.0, 0.0], [1.0, 1.0], 1e-7);
    ext.perform();
    assert!(ext.is_done());
    assert_eq!(ext.nb_ext(), 1);
}
