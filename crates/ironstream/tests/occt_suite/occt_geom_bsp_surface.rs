extern crate ironstream;
use ironstream::geom_bsp_surface::*;

#[test]
fn test_geom_bsp_surface_basic() {
    let mut s = BspSurface::new(1, 1);

    // Set up a flat bilinear patch from (0,0,0) to (1,1,0)
    // U knots: [0,0,1,1], V knots: [0,0,1,1]
    s.set_u_knots(vec![0.0, 0.0, 1.0, 1.0]);
    s.set_v_knots(vec![0.0, 0.0, 1.0, 1.0]);
    s.set_pole(0, 0, [0.0, 0.0, 0.0]);
    s.set_pole(0, 1, [0.0, 1.0, 0.0]);
    s.set_pole(1, 0, [1.0, 0.0, 0.0]);
    s.set_pole(1, 1, [1.0, 1.0, 0.0]);

    assert_eq!(s.nb_u_poles(), 2);
    assert_eq!(s.nb_v_poles(), 2);

    // Evaluate at (0.5, 0.5) -> should be approximately (0.5, 0.5, 0)
    let p = s.evaluate(0.5, 0.5);
    assert!((p[0] - 0.5).abs() < 1e-9, "x should be 0.5, got {}", p[0]);
    assert!((p[1] - 0.5).abs() < 1e-9, "y should be 0.5, got {}", p[1]);
    assert!(p[2].abs() < 1e-9);

    // Bounds
    let (u_bounds, v_bounds) = bsp_surface_bounds(&s);
    assert!((u_bounds[0] - 0.0).abs() < 1e-10);
    assert!((u_bounds[1] - 1.0).abs() < 1e-10);
    assert!((v_bounds[0] - 0.0).abs() < 1e-10);
    assert!((v_bounds[1] - 1.0).abs() < 1e-10);
}

#[test]
fn test_bsp_surface_d1() {
    let mut s = BspSurface::new(1, 1);
    s.set_u_knots(vec![0.0, 0.0, 1.0, 1.0]);
    s.set_v_knots(vec![0.0, 0.0, 1.0, 1.0]);
    s.set_pole(0, 0, [0.0, 0.0, 0.0]);
    s.set_pole(0, 1, [0.0, 1.0, 0.0]);
    s.set_pole(1, 0, [1.0, 0.0, 0.0]);
    s.set_pole(1, 1, [1.0, 1.0, 0.0]);

    let du = s.d1u(0.5, 0.5);
    let dv = s.d1v(0.5, 0.5);

    // For bilinear patch: d/du should point in x direction, d/dv in y direction
    assert!(du[0].abs() > 0.0, "d1u should have nonzero x component");
    assert!(dv[1].abs() > 0.0, "d1v should have nonzero y component");
}
