// FILE: tests/occt_v3d_view_ext.rs
extern crate ironstream;

use ironstream::v3d_view_ext::{ProjectionType, Graphic3dCamera, V3dViewExt};

#[test]
fn camera_default_state() {
    let cam = Graphic3dCamera::new();
    assert_eq!(cam.projection, ProjectionType::Perspective);
    assert!((cam.fov_y() - 45.0).abs() < 1e-9);
    assert!(cam.is_zup);
}

#[test]
fn camera_direction_normalized() {
    let cam = Graphic3dCamera::new(); // eye=[0,0,100], center=[0,0,0]
    let dir = cam.direction();
    // direction should be (0,0,-1)
    assert!((dir[0] - 0.0).abs() < 1e-9);
    assert!((dir[1] - 0.0).abs() < 1e-9);
    assert!((dir[2] - (-1.0)).abs() < 1e-9);
}

#[test]
fn camera_fov_clamping() {
    let mut cam = Graphic3dCamera::new();
    cam.set_fov_y(60.0);
    assert!((cam.fov_y() - 60.0).abs() < 1e-9);
    cam.set_fov_y(0.0); // clamps to 1.0
    assert!((cam.fov_y() - 1.0).abs() < 1e-9);
    cam.set_fov_y(200.0); // clamps to 179.0
    assert!((cam.fov_y() - 179.0).abs() < 1e-9);
}

#[test]
fn view_ext_aspect_ratio() {
    let v = V3dViewExt::new(1920, 1080);
    let ratio = v.aspect_ratio();
    assert!((ratio - 1920.0 / 1080.0).abs() < 1e-9);
    assert!(v.is_antialiased());
}

#[test]
fn view_ext_background_and_gradient() {
    let mut v = V3dViewExt::new(800, 600);
    v.set_gradient_background([0.0, 0.0, 0.2], [0.5, 0.5, 1.0]);
    assert!(v.gradient_bg.is_some());
    v.set_background_color(0.1, 0.2, 0.3);
    assert_eq!(v.background_color, [0.1, 0.2, 0.3]);
    assert!(v.gradient_bg.is_none()); // cleared by set_background_color
}

#[test]
fn camera_view_volume_orthographic() {
    let mut cam = Graphic3dCamera::new();
    cam.projection = ProjectionType::Orthographic;
    cam.scale = 5.0;
    cam.aspect = 2.0;
    let vol = cam.view_volume(); // [left, right, bottom, top, near, far]
    assert!((vol[0] - (-10.0)).abs() < 1e-9); // left = -scale*aspect
    assert!((vol[1] - 10.0).abs() < 1e-9);   // right = scale*aspect
    assert!((vol[2] - (-5.0)).abs() < 1e-9); // bottom = -scale
    assert!((vol[3] - 5.0).abs() < 1e-9);   // top = scale
}
