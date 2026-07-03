extern crate ironstream;
use ironstream::geom_pnt_cloud::*;

#[test]
fn test_point_cloud_add_and_count() {
    let mut pc = PointCloud::new();
    pc.add_point([0.0, 0.0, 0.0]);
    pc.add_point([1.0, 0.0, 0.0]);
    pc.add_point([0.0, 1.0, 0.0]);
    assert_eq!(pc.nb_points(), 3);
    assert!(!pc.has_normals());
    assert!(!pc.has_colors());
}

#[test]
fn test_point_cloud_bounding_box() {
    let mut pc = PointCloud::new();
    pc.add_point([0.0, 0.0, 0.0]);
    pc.add_point([5.0, 3.0, -2.0]);
    let (min, max) = pc.bounding_box().unwrap();
    assert!((min[0]).abs() < 1e-10);
    assert!((max[0] - 5.0).abs() < 1e-10);
    assert!((min[2] + 2.0).abs() < 1e-10);
}

#[test]
fn test_point_cloud_centroid() {
    let mut pc = PointCloud::new();
    pc.add_point([0.0, 0.0, 0.0]);
    pc.add_point([4.0, 0.0, 0.0]);
    pc.add_point([2.0, 6.0, 0.0]);
    let c = pc.centroid().unwrap();
    assert!((c[0] - 2.0).abs() < 1e-10);
    assert!((c[1] - 2.0).abs() < 1e-10);
}

#[test]
fn test_point_cloud_k_nearest() {
    let mut pc = PointCloud::new();
    pc.add_point([0.0, 0.0, 0.0]);
    pc.add_point([1.0, 0.0, 0.0]);
    pc.add_point([100.0, 0.0, 0.0]);
    let nn = pc.k_nearest([0.1, 0.0, 0.0], 2);
    assert_eq!(nn.len(), 2);
    // Closest point is index 0
    assert_eq!(nn[0].0, 0);
    assert!(nn[0].1 < nn[1].1);
}

#[test]
fn test_point_cloud_voxel_downsample() {
    let mut pc = PointCloud::new();
    for i in 0..20 {
        pc.add_point([i as f64 * 0.01, 0.0, 0.0]);
    }
    let down = pc.voxel_downsample(0.1);
    assert!(down.nb_points() < pc.nb_points());
}
