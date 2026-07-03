extern crate ironstream;
use ironstream::geom_curve_network::*;

#[test]
fn test_geom_curve_network_basic() {
    // CurveNetwork
    let mut net = CurveNetwork::new(1.0e-6);
    net.add_curve(vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]);
    net.add_curve(vec![[0.0, 1.0, 0.0], [1.0, 1.0, 0.0]]);
    assert_eq!(net.num_curves(), 2);
    let out = net.build();
    assert_eq!(out.len(), 4); // no coincident points

    // NetworkConstraint
    let nc = NetworkConstraint::new(0).with_weight(2.0).with_tangent(true);
    assert_eq!(nc.curve_idx, 0);
    assert!((nc.weight - 2.0).abs() < f64::EPSILON);
    assert!(nc.tangent);

    // interpolate_network
    let curves = vec![
        vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]],
        vec![[1.0, 0.0, 0.0], [1.0, 1.0, 0.0]],
    ];
    let result = interpolate_network(&curves, 1.0e-6);
    // shared point [1,0,0] merges: 3 unique
    assert_eq!(result.len(), 3);

    // curve_network_bounds
    let (lo, hi) = curve_network_bounds(&net);
    assert!((lo[0] - 0.0).abs() < 1e-12);
    assert!((hi[0] - 1.0).abs() < 1e-12);
}
