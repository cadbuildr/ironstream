use ironstream::bvh_prim::*;

#[test]
fn bvh_box_contains() {
    let b = BvhBox::new([0.0, 0.0, 0.0], [1.0, 1.0, 1.0]);
    assert!(b.is_valid());
    assert!(b.contains([0.5, 0.5, 0.5]));
    assert!(!b.contains([2.0, 0.0, 0.0]));
}

#[test]
fn bvh_box_extend() {
    let mut b = BvhBox::default();
    b.extend([1.0, 2.0, 3.0]);
    b.extend([-1.0, 0.0, 1.0]);
    assert!((b.min[0] - (-1.0)).abs() < 1e-6);
    assert!((b.max[2] - 3.0).abs() < 1e-6);
}

#[test]
fn bvh_box_combine() {
    let b1 = BvhBox::new([0.0; 3], [1.0; 3]);
    let b2 = BvhBox::new([-2.0, -2.0, -2.0], [0.5, 0.5, 0.5]);
    let combined = b1.combine(&b2);
    assert!((combined.min[0] - (-2.0)).abs() < 1e-6);
    assert!((combined.max[0] - 1.0).abs() < 1e-6);
}

#[test]
fn bvh_primitive_set_closest() {
    let mut s = BvhPrimitiveSet3d::new();
    s.add(BvhPrimitive::from_point(1, [0.0, 0.0, 0.0]));
    s.add(BvhPrimitive::from_point(2, [10.0, 0.0, 0.0]));
    let (id, dist) = s.closest([1.0, 0.0, 0.0]).unwrap();
    assert_eq!(id, 1);
    assert!((dist - 1.0).abs() < 1e-5);
}

#[test]
fn bvh_distance_field_set_get() {
    let b = BvhBox::new([0.0; 3], [1.0; 3]);
    let mut df = BvhDistanceField::new(b, 4, 4, 4);
    assert!(df.set(1, 2, 3, 0.5));
    assert_eq!(df.get(1, 2, 3), Some(0.5));
    assert!(df.get(10, 0, 0).is_none());
}

#[test]
fn bvh_primitive_set_total_bounds() {
    let mut s = BvhPrimitiveSet3d::new();
    let b1 = BvhBox::new([0.0; 3], [1.0; 3]);
    let b2 = BvhBox::new([-2.0, -2.0, -2.0], [0.5, 0.5, 0.5]);
    s.add(BvhPrimitive::from_box(1, b1));
    s.add(BvhPrimitive::from_box(2, b2));
    let total = s.total_bounds();
    assert!(total.min[0] <= -2.0);
    assert!(total.max[0] >= 1.0);
    assert_eq!(s.nb_primitives(), 2);
}
