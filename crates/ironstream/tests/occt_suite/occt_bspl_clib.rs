// Integration tests for bspl_clib
use ironstream::bspl_clib::*;

#[test]
fn build_knots_from_mults_basic() {
    let kv = vec![0.0, 0.5, 1.0];
    let mv = vec![3u32, 1, 3];
    let flat = BSplCLib::build_knots_from_mults(&kv, &mv);
    assert_eq!(flat, vec![0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0]);
}

#[test]
fn check_knots_invalid_decreasing() {
    let knots = vec![0.0, 0.5, 0.3, 1.0]; // decreasing: invalid
    assert!(!BSplCLib::check_knots(1, &knots, 2));
}

#[test]
fn reparametrize_endpoints() {
    let t0 = BSplCLib::reparametrize(0.0, 0.0, 1.0, 2.0, 5.0);
    let t1 = BSplCLib::reparametrize(1.0, 0.0, 1.0, 2.0, 5.0);
    assert!((t0 - 2.0).abs() < 1e-12);
    assert!((t1 - 5.0).abs() < 1e-12);
}

#[test]
fn find_span_at_last_knot() {
    // degree=1, n=2 → knots=[0,0,0.5,1,1]
    let knots = vec![0.0, 0.0, 0.5, 1.0, 1.0];
    // t >= knots[n+1] = knots[3] = 1.0 → should clamp to n=2
    let span = BSplCLib::find_span(2, 1, 1.0, &knots);
    assert_eq!(span, 2);
}

#[test]
fn surface_point_bilinear() {
    // Bilinear patch: 2x2 poles, degree 1 in both directions
    let knots_u = vec![0.0, 0.0, 1.0, 1.0];
    let knots_v = vec![0.0, 0.0, 1.0, 1.0];
    let poles = vec![
        vec![[0.0, 0.0, 0.0], [0.0, 1.0, 0.0]],
        vec![[1.0, 0.0, 0.0], [1.0, 1.0, 0.0]],
    ];
    let pt = BSplSLib::surface_point(0.5, 0.5, 1, 1, &knots_u, &knots_v, &poles).unwrap();
    assert!((pt[0] - 0.5).abs() < 1e-10);
    assert!((pt[1] - 0.5).abs() < 1e-10);
    assert!(pt[2].abs() < 1e-10);
}

#[test]
fn knot_multiplicities_uniform() {
    // Uniform open knot vector for degree=2, 4 poles: [0,0,0,1,1,1]
    let knots = vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0];
    let mults = BSplCLib::knot_multiplicities(&knots);
    assert_eq!(mults.len(), 2);
    assert_eq!(mults[0], (0.0, 3));
    assert_eq!(mults[1], (1.0, 3));
}
