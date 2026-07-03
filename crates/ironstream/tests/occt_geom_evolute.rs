extern crate ironstream;
use ironstream::geom_evolute::*;

#[test]
fn test_geom_evolute_basic() {
    let ev = Evolute::from_curve("circle", 4);
    assert_eq!(ev.sample_count(), 4);
    // Evolute of a unit circle should be at the centre
    for pt in &ev.samples {
        assert!(pt[0].abs() < 1e-9);
        assert!(pt[1].abs() < 1e-9);
    }

    let inv = Involute::new("circle", 0.0);
    let pt = inv.at(0.0);
    // At start param, involute collapses to curve point
    assert!((pt[0] - 1.0).abs() < 1e-9);

    let off = CurveOffset::new("circle", 0.5);
    let q = off.evaluate(0.0);
    let r = (q[0] * q[0] + q[1] * q[1]).sqrt();
    assert!((r - 0.5).abs() < 1e-9);

    let pts = offset_curve_points("ellipse", 0.1, 8);
    assert_eq!(pts.len(), 8);
}
