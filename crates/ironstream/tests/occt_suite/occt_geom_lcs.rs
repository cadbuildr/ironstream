extern crate ironstream;

use ironstream::geom_lcs::{Datum3D, LocalCS};

#[test]
fn identity_round_trip() {
    let lcs = LocalCS::identity();
    let p = [1.0, 2.0, 3.0];
    let local = lcs.world_to_local(p);
    let world = lcs.local_to_world(local);
    for i in 0..3 {
        assert!((world[i] - p[i]).abs() < 1e-10, "axis {i} mismatch");
    }
}

#[test]
fn lcs_z_dir_identity() {
    let lcs = LocalCS::identity();
    let z = lcs.z_dir();
    assert!((z[0]).abs() < 1e-10);
    assert!((z[1]).abs() < 1e-10);
    assert!((z[2] - 1.0).abs() < 1e-10);
}

#[test]
fn lcs_world_to_local_offset_origin() {
    let lcs = LocalCS::new([5.0,0.0,0.0], [1.0,0.0,0.0], [0.0,1.0,0.0]);
    let local = lcs.world_to_local([6.0, 2.0, 0.0]);
    assert!((local[0] - 1.0).abs() < 1e-10);
    assert!((local[1] - 2.0).abs() < 1e-10);
}

#[test]
fn lcs_translate() {
    let lcs = LocalCS::identity();
    let moved = lcs.translated([0.0, 3.0, 0.0]);
    assert!((moved.origin[1] - 3.0).abs() < 1e-10);
    // axes unchanged
    assert_eq!(moved.x_dir, [1.0, 0.0, 0.0]);
}

#[test]
fn datum3d_identity_check() {
    let d = Datum3D::new(LocalCS::identity());
    assert!(d.is_identity());
    let shifted = Datum3D::new(LocalCS::identity().translated([1.0, 0.0, 0.0]));
    assert!(!shifted.is_identity());
}
