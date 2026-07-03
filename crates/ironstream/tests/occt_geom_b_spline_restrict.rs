extern crate ironstream;
use ironstream::geom_b_spline_restrict::*;

#[test]
fn insert_knot_interior() {
    let knots = vec![0.0, 1.0];
    let mult = vec![1u32, 1];
    let (k, m) = insert_knot(&knots, &mult, 0.5, 3);
    assert!(k.contains(&0.5));
    assert_eq!(k.len(), m.len());
    assert_eq!(k.len(), 3);
}

#[test]
fn insert_knot_increases_existing_multiplicity() {
    let knots = vec![0.0, 0.5, 1.0];
    let mult = vec![1u32, 1, 1];
    let (k, m) = insert_knot(&knots, &mult, 0.5, 3);
    let idx = k.iter().position(|&x| (x - 0.5).abs() < 1e-14).unwrap();
    assert_eq!(m[idx], 2);
    assert_eq!(k.len(), 3); // no new knot added
}

#[test]
fn remove_knot_decrements_multiplicity() {
    let knots = vec![0.0, 0.5, 1.0];
    let mult = vec![1u32, 2, 1];
    let (k, m) = remove_knot(&knots, &mult, 1, 1e-6).unwrap();
    assert_eq!(m[1], 1);
    assert_eq!(k.len(), 3);
}

#[test]
fn concat_bsplines_merges_shared_endpoint() {
    let mut c = ConcatBSplines::new(1e-6);
    c.add(BSplineSegment { poles: vec![[0.0,0.0,0.0],[0.5,1.0,0.0],[1.0,0.0,0.0]], t_start: 0.0, t_end: 1.0 });
    c.add(BSplineSegment { poles: vec![[1.0,0.0,0.0],[1.5,-1.0,0.0],[2.0,0.0,0.0]], t_start: 0.0, t_end: 1.0 });
    c.build();
    assert!(c.is_done());
    assert_eq!(c.nb_poles(), 5);
}

#[test]
fn rational_to_poly_normalizes() {
    let poles = vec![[2.0,0.0,0.0],[4.0,0.0,0.0]];
    let weights = vec![2.0, 4.0];
    let r = RationalToPoly::new(poles, weights);
    assert!(r.is_done());
    assert_eq!(r.nb_poles(), 2);
    assert!((r.result_poles[0][0] - 1.0).abs() < 1e-10);
    assert!((r.result_poles[1][0] - 1.0).abs() < 1e-10);
}
