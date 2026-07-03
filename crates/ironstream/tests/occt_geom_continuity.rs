extern crate ironstream;
use ironstream::geom_continuity::*;

#[test]
fn test_geom_continuity_basic() {
    // Order queries.
    assert_eq!(GeomContinuity::C0.order(), 0);
    assert_eq!(GeomContinuity::CN.order(), u32::MAX);
    assert_eq!(GeomContinuity::C2.order(), 2);

    // Geometric flag.
    assert!(GeomContinuity::G1.is_geometric());
    assert!(GeomContinuity::G2.is_geometric());
    assert!(!GeomContinuity::C1.is_geometric());

    // Ordering.
    assert!(GeomContinuity::C0 < GeomContinuity::CN);
    assert!(GeomContinuity::C1 < GeomContinuity::C2);

    // OsculatingSurface.
    let s = OsculatingSurface::new("bspline_surface", 1e-6);
    assert_eq!(s.surface, "bspline_surface");
    assert!(s.is_ok(0.0, 0.0));
    assert!(!s.is_ok(f64::NAN, 0.0));
    let p1 = s.lrp1(1.0, 2.0);
    let p2 = s.lrp2(1.0, 2.0);
    assert_eq!(p1, [1.0, 2.0, 0.0]);
    assert_eq!(p2, [1.0, 2.0, 1.0]);

    // Continuity helpers.
    assert_eq!(check_continuity_curve("bspline_curve", 1e-6), GeomContinuity::CN);
    assert_eq!(check_continuity_curve("bezier_seg", 1e-6), GeomContinuity::C2);
    assert_eq!(check_continuity_curve("mystery", 1e-6), GeomContinuity::C0);
    assert_eq!(check_continuity_surface("sphere_face", 1e-6), GeomContinuity::CN);
    assert_eq!(check_continuity_surface("unknown", 1e-6), GeomContinuity::C0);
}
