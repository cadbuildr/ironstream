use ironstream::voxel::*;

#[test]
fn bool_ds_set_get_nb_set() {
    let b = VoxelBounds::default();
    let mut ds = VoxelBoolDs::new(b, 4, 4, 4);
    assert_eq!(ds.nb_set(), 0);
    ds.set(1, 2, 3, true);
    assert_eq!(ds.get(1, 2, 3), Some(true));
    assert_eq!(ds.nb_set(), 1);
    ds.set(1, 2, 3, false);
    assert_eq!(ds.nb_set(), 0);
}

#[test]
fn bool_ds_out_of_bounds() {
    let b = VoxelBounds::default();
    let mut ds = VoxelBoolDs::new(b, 3, 3, 3);
    assert!(!ds.set(10, 0, 0, true));
    assert!(ds.get(10, 0, 0).is_none());
    assert_eq!(ds.nb_voxels(), 27);
}

#[test]
fn bool_ds_voxel_size() {
    let b = VoxelBounds::new(0.0, 0.0, 0.0, 10.0, 20.0, 30.0);
    let ds = VoxelBoolDs::new(b, 10, 20, 30);
    let sz = ds.voxel_size();
    assert!((sz[0] - 1.0).abs() < 1e-10);
    assert!((sz[1] - 1.0).abs() < 1e-10);
    assert!((sz[2] - 1.0).abs() < 1e-10);
}

#[test]
fn float_ds_min_max() {
    let b = VoxelBounds::default();
    let mut ds = VoxelFloatDs::new(b, 2, 2, 2);
    ds.set(0, 0, 0, -5.0);
    ds.set(1, 1, 1, 10.0);
    assert!((ds.min_value() - (-5.0)).abs() < 1e-6);
    assert!((ds.max_value() - 10.0).abs() < 1e-6);
}

#[test]
fn collision_detection_detect() {
    let b = VoxelBounds::default();
    let mut a = VoxelBoolDs::new(b, 3, 3, 3);
    let mut bds = VoxelBoolDs::new(b, 3, 3, 3);
    a.set(1, 1, 1, true);
    bds.set(1, 1, 1, true);
    bds.set(0, 0, 0, true);
    let mut cd = VoxelCollisionDetection::new(0.0);
    cd.detect(&a, &bds);
    assert_eq!(cd.nb_collisions(), 1);
    assert!(cd.has_collision());
}

#[test]
fn no_collision_when_disjoint() {
    let b = VoxelBounds::default();
    let mut a = VoxelBoolDs::new(b, 3, 3, 3);
    let bds = VoxelBoolDs::new(b, 3, 3, 3);
    a.set(0, 0, 0, true);
    let mut cd = VoxelCollisionDetection::new(0.0);
    cd.detect(&a, &bds);
    assert!(!cd.has_collision());
    assert_eq!(cd.nb_collisions(), 0);
}
