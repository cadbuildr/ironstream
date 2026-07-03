// FILE: tests/occt_v3d_window.rs
extern crate ironstream;

use ironstream::v3d_window::{ProjectionType, V3dCamera, V3dViewer, V3dWindow, ViewOrientation};

#[test]
fn v3d_camera_direction_and_distance() {
    let mut cam = V3dCamera::new();
    cam.set_eye([0.0, 0.0, 5.0]);
    cam.set_center([0.0, 0.0, 0.0]);
    let dir = cam.direction();
    assert!((dir[2] - (-1.0)).abs() < 1e-10, "direction should point in -Z");
    assert!((cam.distance() - 5.0).abs() < 1e-10);
}

#[test]
fn v3d_window_aspect_ratio_and_resize() {
    let mut win = V3dWindow::new(1024, 768);
    assert!((win.aspect_ratio() - 1024.0 / 768.0).abs() < 1e-10);
    win.resize(800, 600);
    assert_eq!(win.size(), (800, 600));
    assert!((win.aspect_ratio() - 800.0 / 600.0).abs() < 1e-10);
}

#[test]
fn v3d_camera_set_projection_orthographic() {
    let mut cam = V3dCamera::new();
    assert!(!cam.is_orthographic());
    cam.set_projection(ProjectionType::Orthographic);
    assert!(cam.is_orthographic());
    assert_eq!(cam.projection, ProjectionType::Orthographic);
}

#[test]
fn v3d_viewer_create_view_and_nb_views() {
    let mut viewer = V3dViewer::new();
    let id0 = viewer.create_view(V3dWindow::new(800, 600));
    let id1 = viewer.create_view(V3dWindow::new(400, 300));
    assert_eq!(viewer.nb_views(), 2);
    assert_ne!(id0, id1);
    assert!(viewer.view(id0).is_some());
    assert!(viewer.view(id1).is_some());
    assert!(viewer.view(99).is_none());
}

#[test]
fn v3d_view_set_orientation() {
    let mut viewer = V3dViewer::new();
    let id = viewer.create_view(V3dWindow::new(800, 600));
    let view = viewer.view_mut(id).unwrap();
    view.set_orientation(ViewOrientation::Top);
    assert_eq!(view.orientation, ViewOrientation::Top);
    assert!((view.camera.eye[1] - 10.0).abs() < 1e-10, "Top view: eye.y should be 10");

    view.set_orientation(ViewOrientation::Front);
    assert_eq!(view.orientation, ViewOrientation::Front);
    assert!((view.camera.eye[2] - 10.0).abs() < 1e-10, "Front view: eye.z should be 10");
}
