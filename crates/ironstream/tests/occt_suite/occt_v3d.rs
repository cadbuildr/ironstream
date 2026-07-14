// FILE: tests/occt_v3d.rs
extern crate ironstream;
use ironstream::v3d::*;

#[test]
fn test_create_viewer_and_add_views() {
    let mut viewer = V3dViewer::new();
    let v1 = V3dViewer::create_view();
    let v2 = V3dViewer::create_view();
    viewer.add_view(v1);
    viewer.add_view(v2);
    assert_eq!(viewer.view_count(), 2);
}

#[test]
fn test_view_projection_roundtrip() {
    let mut view = V3dView::new();
    assert_eq!(view.projection(), V3dTypeOfProjection::Orthographic);
    view.set_projection(V3dTypeOfProjection::Perspective);
    assert_eq!(view.projection(), V3dTypeOfProjection::Perspective);
    view.set_projection(V3dTypeOfProjection::Orthographic);
    assert_eq!(view.projection(), V3dTypeOfProjection::Orthographic);
}

#[test]
fn test_view_size_set_and_get() {
    let mut view = V3dView::new();
    view.set_size(1280, 720);
    assert_eq!(view.size(), (1280, 720));
    view.set_size(3840, 2160);
    assert_eq!(view.size(), (3840, 2160));
}

#[test]
fn test_view_eye_position() {
    let mut view = V3dView::new();
    view.set_eye(10.0, 20.0, 30.0);
    let eye = view.eye();
    assert_eq!(eye[0], 10.0);
    assert_eq!(eye[1], 20.0);
    assert_eq!(eye[2], 30.0);
}

#[test]
fn test_view_at_position() {
    let mut view = V3dView::new();
    view.set_at(-1.0, 2.5, 0.0);
    let at = view.at();
    assert_eq!(at[0], -1.0);
    assert_eq!(at[1], 2.5);
    assert_eq!(at[2], 0.0);
}

#[test]
fn test_view_scale() {
    let mut view = V3dView::new();
    assert_eq!(view.scale(), 1.0);
    view.set_scale(0.25);
    assert_eq!(view.scale(), 0.25);
    view.set_scale(100.0);
    assert_eq!(view.scale(), 100.0);
}

#[test]
fn test_viewer_background_color() {
    let mut viewer = V3dViewer::new();
    viewer.set_background_color(0.1, 0.2, 0.3, 0.9);
    let bg = viewer.background_color();
    assert!((bg[0] - 0.1f32).abs() < 1e-6);
    assert!((bg[1] - 0.2f32).abs() < 1e-6);
    assert!((bg[2] - 0.3f32).abs() < 1e-6);
    assert!((bg[3] - 0.9f32).abs() < 1e-6);
}

#[test]
fn test_viewer_update_does_not_panic() {
    let viewer = V3dViewer::new();
    viewer.update();
}

#[test]
fn test_view_fit_all_does_not_panic() {
    let mut view = V3dView::new();
    view.set_size(640, 480);
    view.set_eye(5.0, 5.0, 5.0);
    view.fit_all();
    assert_eq!(view.size(), (640, 480));
}

#[test]
fn test_orientation_thirteen_variants() {
    let all = [
        V3dTypeOfOrientation::XposYpos,
        V3dTypeOfOrientation::XposYneg,
        V3dTypeOfOrientation::XnegYpos,
        V3dTypeOfOrientation::XnegYneg,
        V3dTypeOfOrientation::XposZpos,
        V3dTypeOfOrientation::XposZneg,
        V3dTypeOfOrientation::XnegZpos,
        V3dTypeOfOrientation::XnegZneg,
        V3dTypeOfOrientation::YposZpos,
        V3dTypeOfOrientation::YposZneg,
        V3dTypeOfOrientation::YnegZpos,
        V3dTypeOfOrientation::YnegZneg,
        V3dTypeOfOrientation::XposYposZpos,
    ];
    assert_eq!(all.len(), 13);
    for i in 0..all.len() {
        for j in (i + 1)..all.len() {
            assert_ne!(all[i], all[j], "variants {} and {} must differ", i, j);
        }
    }
}

#[test]
fn test_view_full_camera_setup() {
    let mut view = V3dView::new();
    view.set_projection(V3dTypeOfProjection::Perspective);
    view.set_size(1024, 768);
    view.set_eye(0.0, -10.0, 5.0);
    view.set_at(0.0, 0.0, 0.0);
    view.set_scale(1.5);
    view.fit_all();
    view.update();

    assert_eq!(view.projection(), V3dTypeOfProjection::Perspective);
    assert_eq!(view.size(), (1024, 768));
    assert_eq!(view.eye(), [0.0, -10.0, 5.0]);
    assert_eq!(view.at(), [0.0, 0.0, 0.0]);
    assert_eq!(view.scale(), 1.5);
}

#[test]
fn test_viewer_empty_on_construction() {
    let viewer = V3dViewer::new();
    assert_eq!(viewer.view_count(), 0);
    let bg = viewer.background_color();
    assert_eq!(bg[3], 1.0f32);
}

#[test]
fn test_enums_copy_and_clone() {
    let o = V3dTypeOfOrientation::YposZneg;
    let o2 = o;
    assert_eq!(o, o2);

    let p = V3dTypeOfProjection::Orthographic;
    let p_clone = p;
    assert_eq!(p, p_clone);
}
