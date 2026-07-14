extern crate ironstream;
use ironstream::geom_surface_extensions::*;

#[test]
fn test_geom_surface_extensions_basic() {
    // TrimmedSurface
    let ts = TrimmedSurface::new("plane", 0.0, 1.0, 0.0, 2.0);
    assert_eq!(ts.basis, "plane");
    assert!((ts.u1 - 0.0).abs() < 1e-12);
    assert!((ts.u2 - 1.0).abs() < 1e-12);
    // evaluate clamps to bounds
    let p = ts.evaluate(0.5, 1.0);
    assert!((p[0] - 0.5).abs() < 1e-12);
    assert!((p[1] - 1.0).abs() < 1e-12);

    // bound normalization: reversed u
    let ts2 = TrimmedSurface::new("cyl", 2.0, 0.0, 1.0, 3.0);
    assert!(ts2.u1 <= ts2.u2);

    // SweptSurface
    let sw = SweptSurface::new("profile", "spine");
    assert_eq!(sw.profile, "profile");
    assert_eq!(sw.spine, "spine");
    let sp = sw.evaluate(0.5, 1.0);
    assert!((sp[0] - 0.5).abs() < 1e-12);

    // ExtrusionSurface
    let mut es = ExtrusionSurface::new([0.0, 0.0, 1.0]);
    es.add_profile_pt([0.0, 0.0, 0.0]);
    es.add_profile_pt([1.0, 0.0, 0.0]);
    let ep = es.evaluate(0.0, 0.0);
    assert!((ep[0] - 0.0).abs() < 1e-10);
    let ep2 = es.evaluate(1.0, 0.0);
    assert!((ep2[0] - 1.0).abs() < 1e-10);
}
