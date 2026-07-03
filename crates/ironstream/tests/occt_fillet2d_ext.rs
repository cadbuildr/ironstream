extern crate ironstream;
use ironstream::fillet2d_ext::*;

#[test]
fn test_fillet_arc2d_arc_length() {
    let arc = FilletArc2d::new([0.0, 0.0], 2.0, 0.0, std::f64::consts::FRAC_PI_2);
    let expected = 2.0 * std::f64::consts::FRAC_PI_2;
    assert!((arc.arc_length() - expected).abs() < 1e-10);
}

#[test]
fn test_fillet_arc2d_start_end_points() {
    let arc = FilletArc2d::new([0.0, 0.0], 1.0, 0.0, std::f64::consts::FRAC_PI_2);
    let sp = arc.start_point();
    let ep = arc.end_point();
    assert!((sp[0] - 1.0).abs() < 1e-10); // cos(0)=1
    assert!((ep[1] - 1.0).abs() < 1e-10); // sin(π/2)=1
}

#[test]
fn test_fillet2d_perform_success() {
    let mut f = Fillet2d::new(0, 1, 3.0);
    f.perform();
    assert!(f.is_done());
    assert_eq!(f.nb_solutions(), 1);
    assert!((f.curve().unwrap().radius - 3.0).abs() < 1e-10);
}

#[test]
fn test_fillet2d_bad_radius_status() {
    let mut f = Fillet2d::new(0, 1, 0.0);
    f.perform();
    assert!(!f.is_done());
    assert_eq!(f.status, FilletStatus::BadRadius);
}

#[test]
fn test_fillet_constructor_multiple_fillets() {
    let mut fc = FilletConstructor::new(10);
    fc.add_fillet(0, 1, 1.0);
    fc.add_fillet(1, 2, 2.0);
    fc.add_fillet(2, 3, 0.5);
    fc.perform();
    assert!(fc.is_done());
    assert_eq!(fc.nb_fillets(), 3);
    assert_eq!(fc.nb_solutions(), 3);
}

#[test]
fn test_fillet_surf_builder_perform() {
    let mut b = FilletSurfBuilder::new(0, 1, 5.0);
    b.perform();
    assert!(b.is_done());
    assert_eq!(b.nb_surfaces(), 1);
    assert!((b.surface_first_param() - 0.0).abs() < 1e-10);
    assert!((b.surface_last_param() - 1.0).abs() < 1e-10);
}
