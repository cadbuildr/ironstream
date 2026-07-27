extern crate ironstream;
use ironstream::brep_sweep_ext::*;

#[test]
fn test_brep_sweep_ext_basic() {
    let trsf = SweepTrsf::new("profile", 4);
    let loc = trsf.location_at(0);
    assert_eq!(loc[3][3], 1.0);

    let between = trsf.between(0, 2);
    assert_eq!(between[3][3], 1.0);

    let mut iter = SweepIterator::new("shell");
    assert!(!iter.more());
    iter.shapes.push("face_1".to_string());
    iter.shapes.push("face_2".to_string());
    iter.index = 0;
    assert!(iter.more());
    let s = iter.current_shape();
    assert_eq!(s, Some(&"face_1".to_string()));

    let result = SweepParams::new("spine", "profile")
        .as_solid(true)
        .build();
    assert!(!result.is_empty());
}
