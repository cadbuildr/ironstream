extern crate ironstream;
use ironstream::geom_surface_approx::*;

#[test]
fn test_geom_surface_approx_basic() {
    // SurfaceApprox builder
    let mut sa = SurfaceApprox::new(3, 3, 1e-5);
    assert_eq!(sa.u_degree, 3);
    assert_eq!(sa.v_degree, 3);
    assert_eq!(sa.tolerance, 1e-5);
    assert!(!sa.is_done());

    // add a 4x4 flat grid in XY plane
    for i in 0..4_usize {
        let row: Vec<[f64; 3]> = (0..4_usize).map(|j| [i as f64, j as f64, 0.0]).collect();
        sa.add_row(row);
    }
    sa.compute();
    assert!(sa.is_done());
    assert!(sa.nb_u_poles() > 0);
    assert!(sa.nb_v_poles() > 0);
    let _p = sa.pole(0, 0);
    let err = sa.error();
    assert!(err >= 0.0);

    // fit_bspline_surface convenience wrapper
    let pts: Vec<Vec<[f64; 3]>> = (0..3_usize)
        .map(|i| (0..3_usize).map(|j| [i as f64, j as f64, 0.0]).collect())
        .collect();
    let result = fit_bspline_surface(&pts, 2, 2, 1e-4);
    assert!(result.is_done());
}
