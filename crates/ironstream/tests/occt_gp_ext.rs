extern crate ironstream;
use ironstream::gp_ext::*;

#[test]
fn ax3_identity_z_dir() {
    let ax = Ax3::identity();
    let z = ax.z_dir();
    assert!((z[0]).abs() < 1e-10);
    assert!((z[1]).abs() < 1e-10);
    assert!((z[2] - 1.0).abs() < 1e-10);
    assert!(ax.is_direct());
}

#[test]
fn ax3_mirror_point() {
    let ax = Ax3::identity();
    let mirrored = ax.mirror_point([1.0, 2.0, 3.0]);
    assert!((mirrored[0] - 1.0).abs() < 1e-10);
    assert!((mirrored[1] - 2.0).abs() < 1e-10);
    assert!((mirrored[2] + 3.0).abs() < 1e-10);
}

#[test]
fn euler_angles_identity_matrix() {
    let e = EulerAngles::from_degrees(0.0, 0.0, 0.0, EulerConvention::TaitBryanZYX);
    let m = e.to_matrix_zyx();
    assert!((m[0][0] - 1.0).abs() < 1e-10);
    assert!((m[1][1] - 1.0).abs() < 1e-10);
    assert!((m[2][2] - 1.0).abs() < 1e-10);
}

#[test]
fn nlerp_translation_midpoint() {
    let lerp = TrsfNLerp::new(
        [0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 1.0],
        [2.0, 4.0, 6.0], [0.0, 0.0, 0.0, 1.0],
    );
    let (t, _r) = lerp.interpolate(0.5);
    assert!((t[0] - 1.0).abs() < 1e-10);
    assert!((t[1] - 2.0).abs() < 1e-10);
    assert!((t[2] - 3.0).abs() < 1e-10);
}

#[test]
fn nlerp_clamp_bounds() {
    let lerp = TrsfNLerp::new(
        [0.0; 3], [0.0, 0.0, 0.0, 1.0],
        [1.0, 0.0, 0.0], [0.0, 0.0, 0.0, 1.0],
    );
    let (t0, _) = lerp.interpolate(0.0);
    let (t1, _) = lerp.interpolate(1.0);
    assert!((t0[0]).abs() < 1e-10);
    assert!((t1[0] - 1.0).abs() < 1e-10);
}
