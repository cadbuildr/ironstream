extern crate ironstream;
use ironstream::geom_interp::*;

#[test]
fn test_geom_interp_basic() {
    // Interpolate3d
    let pts = vec![[0.0, 0.0, 0.0], [1.0, 1.0, 0.0], [2.0, 0.0, 0.0]];
    let mut interp = Interpolate3d::new(pts.clone(), false);
    interp.load();
    assert!(interp.is_done());
    assert!(interp.nb_poles() >= 3);

    // convenience function
    let interp2 = interpolate_3d(&pts, false);
    assert!(interp2.is_done());

    // Interpolate2d
    let pts2 = vec![[0.0_f64, 0.0_f64], [1.0, 2.0], [2.0, 0.0]];
    let mut interp2d = Interpolate2d::new(pts2, false);
    interp2d.load();
    assert!(interp2d.is_done());

    // evaluate at t=0 should give first point
    let p = interp2d.evaluate(0.0);
    assert!((p[0]).abs() < 1e-9);
    assert!((p[1]).abs() < 1e-9);
}
