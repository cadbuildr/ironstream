extern crate ironstream;
use ironstream::law_ext::*;

#[test]
fn test_law_ext_basic() {
    let knots = vec![0.0, 1.0, 2.0, 3.0];
    let mults = vec![2, 1, 1, 2];
    let mut ks = KnotSplitting::new(knots, mults);
    ks.compute_splits(1);
    assert!(ks.num_intervals() >= 1);

    let law = LawS::new(2.0);
    let v = law.evaluate(0.5);
    assert!(v > 0.0);
    let d = law.derivative(0.5);
    assert!(d >= 0.0);

    let f = vec![1.0, 2.0, 3.0];
    let g = vec![2.0, 3.0, 4.0];
    let composed = law_compose(&f, &g);
    assert_eq!(composed.len(), 3);
    assert!((composed[0] - 2.0).abs() < 1e-12);

    let scaled = law_scale(&f, 0.5);
    assert!((scaled[0] - 0.5).abs() < 1e-12);
}
