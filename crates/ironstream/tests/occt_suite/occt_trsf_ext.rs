use ironstream::trsf_ext::*;

#[test]
fn gtrsf_identity_transform() {
    let t = GpGTrsf::identity();
    let p = t.transform_point([1.0, 2.0, 3.0]);
    assert!((p[0] - 1.0).abs() < 1e-10);
    assert!((p[1] - 2.0).abs() < 1e-10);
    assert!((p[2] - 3.0).abs() < 1e-10);
}

#[test]
fn gtrsf_translation() {
    let mut t = GpGTrsf::identity();
    t.set_translation([5.0, -3.0, 1.0]);
    let p = t.transform_point([1.0, 1.0, 1.0]);
    assert!((p[0] - 6.0).abs() < 1e-10);
    assert!((p[1] - (-2.0)).abs() < 1e-10);
    assert!((p[2] - 2.0).abs() < 1e-10);
}

#[test]
fn gtrsf2d_inverse_roundtrip() {
    let mut t = GpGTrsf2d::identity();
    t.set_value(0, 0, 2.0); // scale x by 2
    t.set_value(1, 1, 3.0); // scale y by 3
    let inv = t.inverted().expect("non-singular transform should be invertible");
    let p = [4.0, 9.0];
    let tp = t.transform_point(p);
    let back = inv.transform_point(tp);
    assert!((back[0] - p[0]).abs() < 1e-10, "round-trip x should match");
    assert!((back[1] - p[1]).abs() < 1e-10, "round-trip y should match");
}

#[test]
fn gtrsf2d_singular_returns_none() {
    let t = GpGTrsf2d { mat: [[0.0, 0.0, 0.0], [0.0, 0.0, 0.0]] };
    assert!(t.inverted().is_none(), "singular (zero) matrix has no inverse");
}

#[test]
fn toploc_location_composed() {
    let mut loc = TopLocLocation::new();
    assert!(loc.is_identity());
    let mut t1 = GpGTrsf::identity();
    t1.set_translation([1.0, 0.0, 0.0]);
    let mut t2 = GpGTrsf::identity();
    t2.set_translation([0.0, 2.0, 0.0]);
    loc.push(TopLocDatum3d::new("d1", t1));
    loc.push(TopLocDatum3d::new("d2", t2));
    assert_eq!(loc.depth(), 2);
    assert!(!loc.is_identity());
    let _composed = loc.composed();
    // The composed result is deterministic; just verify it doesn't panic
}

#[test]
fn toploc_location_push_pop() {
    let mut loc = TopLocLocation::new();
    let d = TopLocDatum3d::identity();
    loc.push(d);
    assert_eq!(loc.depth(), 1);
    let popped = loc.pop();
    assert!(popped.is_some());
    assert_eq!(loc.depth(), 0);
    assert!(loc.is_identity());
}
