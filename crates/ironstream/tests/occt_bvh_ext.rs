use ironstream::bvh_ext::*;

#[test]
fn bvh_box_add_point_expands_bounds() {
    let mut b = BvhBox3d::default();
    assert!(b.is_void());
    b.add_point([1.0, 2.0, 3.0]);
    b.add_point([-1.0, 0.0, 5.0]);
    assert!(!b.is_void());
    assert!((b.min[0] + 1.0).abs() < 1e-6);
    assert!((b.max[2] - 5.0).abs() < 1e-6);
}

#[test]
fn bvh_box_intersects_and_non_intersects() {
    let a = BvhBox3d::new([0.0; 3], [1.0; 3]);
    let b = BvhBox3d::new([0.5, 0.5, 0.5], [2.0, 2.0, 2.0]);
    let c = BvhBox3d::new([2.0, 2.0, 2.0], [3.0, 3.0, 3.0]);
    assert!(a.intersects(&b), "overlapping boxes should intersect");
    assert!(!a.intersects(&c), "disjoint boxes should not intersect");
}

#[test]
fn bvh_box_longest_axis() {
    let b = BvhBox3d::new([0.0; 3], [3.0, 1.0, 2.0]);
    assert_eq!(b.longest_axis(), 0, "X extent=3 is longest");
    let b2 = BvhBox3d::new([0.0; 3], [1.0, 5.0, 2.0]);
    assert_eq!(b2.longest_axis(), 1, "Y extent=5 is longest");
}

#[test]
fn bvh_ray_hit() {
    let ray = BvhRay3d::new([0.0, 0.0, -5.0], [0.0, 0.0, 1.0]);
    let b = BvhBox3d::new([-1.0; 3], [1.0; 3]);
    let hit = ray.intersect_box(&b);
    assert!(hit.is_some(), "ray pointing at box should hit");
    let (t0, t1) = hit.unwrap();
    assert!(t0 < t1);
    assert!(t0 > 0.0, "intersection should be in front of origin");
}

#[test]
fn bvh_ray_miss() {
    let ray = BvhRay3d::new([5.0, 5.0, -5.0], [0.0, 0.0, 1.0]);
    let b = BvhBox3d::new([-1.0; 3], [1.0; 3]);
    assert!(ray.intersect_box(&b).is_none(), "ray offset away from box should miss");
}

#[test]
fn bvh_tree_build() {
    let boxes = vec![
        BvhBox3d::new([0.0; 3], [1.0; 3]),
        BvhBox3d::new([2.0; 3], [3.0; 3]),
    ];
    let mut tree = BvhTree3d::new();
    assert!(!tree.is_built());
    tree.build_for(&boxes);
    assert!(tree.is_built());
    assert_eq!(tree.nb_primitives(), 2);
    assert_eq!(tree.depth(), 1);
}
