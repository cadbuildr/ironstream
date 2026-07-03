extern crate ironstream;
use ironstream::topo_modify::*;

#[test]
fn occt_trsf_translate() {
    let t = ShapeModification::from_translation([10.0, 0.0, 0.0]);
    let p = t.apply_to_point([1.0, 2.0, 3.0]);
    assert!((p[0] - 11.0).abs() < 1e-10);
}

#[test]
fn occt_shape_modifier() {
    let m = ShapeModification::from_scale(3.0);
    let mut modifier = TopoModifier::new("box_shape")
        .with_modification(m);
    let result = modifier.perform();
    assert!(modifier.is_done());
    assert!(result.contains("modified:"));
}
