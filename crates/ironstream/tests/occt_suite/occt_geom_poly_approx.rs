extern crate ironstream;
use ironstream::geom_poly_approx::*;

#[test]
fn test_geom_poly_approx_basic() {
    // PolyApprox builder
    let mut pa = PolyApprox::new(3, 1e-6);
    assert_eq!(pa.degree, 3);
    assert_eq!(pa.tolerance, 1e-6);
    assert!(!pa.is_done());

    // add points along x-axis
    for i in 0..10 {
        pa.add_point([i as f64, 0.0, 0.0]);
    }
    pa.compute();
    assert!(pa.is_done());
    assert!(pa.nb_poles() > 0);
    let _p0 = pa.pole(0);
    let _k0 = pa.knot(0);
    let err = pa.error();
    assert!(err >= 0.0);

    // fit_bspline convenience wrapper
    let pts: Vec<[f64; 3]> = (0..8).map(|i| [i as f64, (i as f64).sin(), 0.0]).collect();
    let result = fit_bspline(&pts, 3, 1e-4);
    assert!(result.is_done());

    // rdp_simplify
    let polyline: Vec<[f64; 3]> = (0..20).map(|i| [i as f64, 0.0, 0.0]).collect();
    let simplified = rdp_simplify(&polyline, 0.1);
    assert!(simplified.len() <= polyline.len());
    assert!(simplified.len() >= 2);

    // zero epsilon => unchanged
    let unchanged = rdp_simplify(&polyline, 0.0);
    assert_eq!(unchanged.len(), polyline.len());
}
