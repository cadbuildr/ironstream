extern crate ironstream;
use ironstream::geom_curve_parameterize::*;

#[test]
fn test_geom_curve_parameterize_basic() {
    let pts = vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [2.0, 0.0, 0.0]];

    let cl = chord_length_params(&pts);
    assert_eq!(cl.len(), 3);
    assert!((cl[0] - 0.0).abs() < 1e-12);
    assert!((cl[2] - 1.0).abs() < 1e-12);

    let cp = centripetal_params(&pts);
    assert_eq!(cp.len(), 3);
    assert!((cp[0] - 0.0).abs() < 1e-12);
    assert!((cp[2] - 1.0).abs() < 1e-12);

    let up = uniform_params(5);
    assert_eq!(up.len(), 5);
    assert!((up[0] - 0.0).abs() < 1e-12);
    assert!((up[4] - 1.0).abs() < 1e-12);

    let mut param = CurveParameterizer::new(ParametrizationType::ChordLength);
    param.add_point([0.0, 0.0, 0.0]);
    param.add_point([3.0, 4.0, 0.0]);
    let params = param.parameters();
    assert_eq!(params.len(), 2);
    assert!((params[1] - 1.0).abs() < 1e-12);
}
