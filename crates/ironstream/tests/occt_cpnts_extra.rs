extern crate ironstream;
use ironstream::cpnts_extra::*;

#[test]
fn test_abscissa_point_parameter_from_u0() {
    let mut ap = AbscissaPoint::new(3.0, 1e-7);
    ap.perform(2.0);
    assert!(ap.is_done());
    assert!((ap.parameter() - 5.0).abs() < 1e-10);
}

#[test]
fn test_abscissa_point_rejects_negative() {
    let mut ap = AbscissaPoint::new(-1.0, 1e-7);
    ap.perform(0.0);
    assert!(!ap.is_done());
}

#[test]
fn test_uniform_deflection_produces_correct_count() {
    let mut ud = UniformDeflection::new(0.05);
    ud.perform(0.0, 1.0, 6);
    assert!(ud.is_done());
    assert_eq!(ud.nb_points(), 6);
}

#[test]
fn test_uniform_deflection_start_end_params() {
    let mut ud = UniformDeflection::new(0.1);
    ud.perform(2.0, 5.0, 4);
    let (_, u_start) = ud.point_at(0).unwrap();
    let (_, u_end) = ud.point_at(3).unwrap();
    assert!((u_start - 2.0).abs() < 1e-10);
    assert!((u_end - 5.0).abs() < 1e-10);
}

#[test]
fn test_uniform_abscissa_nb_points_matches_spacing() {
    let mut ua = UniformAbscissa::new(0.2);
    ua.perform(0.0, 1.0);
    assert!(ua.is_done());
    assert!(ua.nb_points() >= 6);
    assert!((ua.parameter(0).unwrap() - 0.0).abs() < 1e-10);
}

#[test]
fn test_root_function_solve() {
    let mut rf = MyRootFunction::new(2.0, 1e-7);
    rf.solve(1.0, 10.0);
    assert!(rf.is_done());
    assert!((rf.root().unwrap() - 3.0).abs() < 1e-10);
}
