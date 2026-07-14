extern crate ironstream;
use ironstream::geom_extrude::*;

#[test]
fn test_geom_extrude_basic() {
    let params = ExtrudeParams::new("circle", [0.0, 0.0, 1.0], 5.0)
        .both_directions(false);
    assert_eq!(params.length, 5.0);

    let result = ExtrudeResult::new("cylinder", 10.0);
    assert!(result.is_valid());
    let bad = ExtrudeResult::new("", -1.0);
    assert!(!bad.is_valid());

    let feat = FeatExtrude::new("face", [0.0, 0.0, 1.0], 3.0);
    let res = feat.build();
    assert!(res.is_valid());

    let r = extrude_profile("rectangle", [0.0, 0.0, 1.0], 2.0);
    assert!(!r.shape.is_empty());
}
