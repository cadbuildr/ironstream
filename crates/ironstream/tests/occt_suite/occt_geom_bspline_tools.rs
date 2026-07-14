extern crate ironstream;
use ironstream::geom_bspline_tools::*;

#[test]
fn test_geom_bspline_tools_basic() {
    // Build a clamped cubic B-spline for a straight line [0,0,0] -> [1,0,0].
    let knots = vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0];
    let poles = vec![
        [0.0, 0.0, 0.0],
        [1.0 / 3.0, 0.0, 0.0],
        [2.0 / 3.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
    ];

    // Endpoint interpolation.
    let p0 = bspline_eval(&knots, &poles, 3, 0.0);
    let p1 = bspline_eval(&knots, &poles, 3, 1.0);
    assert!((p0[0]).abs() < 1e-12);
    assert!((p1[0] - 1.0).abs() < 1e-12);

    // Partition of unity.
    let sum: f64 = (0..4).map(|i| bspline_basis(&knots, 3, i, 0.5)).sum();
    assert!((sum - 1.0).abs() < 1e-12);

    // Knot insertion preserves curve.
    let (nk, np) = knot_insert(&knots, &poles, 3, 0.5);
    let orig = bspline_eval(&knots, &poles, 3, 0.5);
    let new = bspline_eval(&nk, &np, 3, 0.5);
    assert!((orig[0] - new[0]).abs() < 1e-10);

    // Knot refinement preserves curve.
    let (rk, rp) = knot_refine(&knots, &poles, 3, &[0.25, 0.75]);
    let orig2 = bspline_eval(&knots, &poles, 3, 0.25);
    let new2 = bspline_eval(&rk, &rp, 3, 0.25);
    assert!((orig2[0] - new2[0]).abs() < 1e-10);

    // BSplineToolkit builder.
    let mut tk = BSplineToolkit::new(3);
    for _ in 0..4 { tk.add_knot(0.0); }
    for _ in 0..4 { tk.add_knot(1.0); }
    tk.add_pole([0.0, 0.0, 0.0]);
    tk.add_pole([1.0 / 3.0, 0.0, 0.0]);
    tk.add_pole([2.0 / 3.0, 0.0, 0.0]);
    tk.add_pole([1.0, 0.0, 0.0]);
    let mid = tk.evaluate(0.5);
    assert!((mid[0] - 0.5).abs() < 1e-12);
    assert!(!tk.is_periodic());

    // Derivative order 0 matches eval.
    let d0 = bspline_deriv(&knots, &poles, 3, 0.5, 0);
    let ev = bspline_eval(&knots, &poles, 3, 0.5);
    assert!((d0[0] - ev[0]).abs() < 1e-12);
}
