extern crate ironstream;

use ironstream::bvh_tree::{BvhBox, BvhTree, LinearBuilder};

#[test]
fn bvh_box_merge() {
    let a = BvhBox { min: [0.0,0.0,0.0], max: [1.0,1.0,1.0] };
    let b = BvhBox { min: [2.0,2.0,2.0], max: [3.0,3.0,3.0] };
    let m = a.merge(&b);
    assert_eq!(m.min, [0.0,0.0,0.0]);
    assert_eq!(m.max, [3.0,3.0,3.0]);
}

#[test]
fn bvh_box_surface_area() {
    let b = BvhBox { min: [0.0,0.0,0.0], max: [2.0,3.0,4.0] };
    // SA = 2*(2*3 + 3*4 + 2*4) = 2*(6+12+8) = 52
    assert!((b.surface_area() - 52.0).abs() < 1e-10);
}

#[test]
fn bvh_tree_root_bounds() {
    let boxes: Vec<BvhBox> = (0..4).map(|i| BvhBox::from_point([i as f64, 0.0, 0.0])).collect();
    let tree = BvhTree::build(boxes);
    let root = tree.root_box().unwrap();
    assert!((root.min[0] - 0.0).abs() < 1e-10);
    assert!((root.max[0] - 3.0).abs() < 1e-10);
}

#[test]
fn bvh_tree_empty() {
    let tree = BvhTree::build(vec![]);
    assert!(tree.is_empty());
    assert!(tree.root_box().is_none());
}

#[test]
fn linear_builder_build() {
    let builder = LinearBuilder::new(2);
    let boxes: Vec<BvhBox> = (0..3).map(|i| BvhBox::from_point([i as f64, i as f64, 0.0])).collect();
    let tree = builder.build(boxes);
    assert!(!tree.is_empty());
}
