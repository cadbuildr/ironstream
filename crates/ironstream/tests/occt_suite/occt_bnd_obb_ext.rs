use ironstream::bnd_obb_ext::*;

#[test]
fn obb_volume_2x2x2_box_equals_48() {
    // half extents 1,2,3 => volume = 8*1*2*3 = 48
    let o = BndObbExt::new([0.0; 3], 1.0, 2.0, 3.0);
    assert!((o.volume() - 48.0).abs() < 1e-10);
    assert!(!o.is_void());
}

#[test]
fn obb_intersects_overlapping_true_separated_false() {
    let a = BndObbExt::new([0.0; 3], 1.0, 1.0, 1.0);
    let b = BndObbExt::new([1.5, 0.0, 0.0], 1.0, 1.0, 1.0);
    assert!(a.intersects_obb(&b)); // overlapping
    let c = BndObbExt::new([5.0, 0.0, 0.0], 1.0, 1.0, 1.0);
    assert!(!a.intersects_obb(&c)); // separated
}

#[test]
fn sphere_contains_point() {
    let s = BndSphereExt::new([0.0; 3], 2.0);
    assert!(s.contains_point([1.0, 1.0, 0.0])); // inside
    assert!(!s.contains_point([2.0, 2.0, 0.0])); // outside (distance = sqrt(8) > 2)
}

#[test]
fn sphere_distance_to_point_outside_and_inside() {
    let s = BndSphereExt::new([0.0; 3], 1.0);
    // point at (3,0,0): distance to surface = 3-1 = 2
    assert!((s.distance_to_point([3.0, 0.0, 0.0]) - 2.0).abs() < 1e-10);
    // point inside: distance = 0
    assert!((s.distance_to_point([0.5, 0.0, 0.0])).abs() < 1e-10);
}

#[test]
fn bnd_range_intersect_and_contains() {
    let mut r = BndRange::new(1.0, 5.0);
    assert!((r.delta() - 4.0).abs() < 1e-10);
    assert!(r.contains(3.0));
    assert!(!r.contains(6.0));
    r.intersect(BndRange::new(3.0, 10.0));
    assert!((r.first() - 3.0).abs() < 1e-10);
    assert!((r.last() - 5.0).abs() < 1e-10);
}
