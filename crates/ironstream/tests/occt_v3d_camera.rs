use ironstream::v3d_camera::*;

#[test]
fn camera_defaults() {
    let cam = V3dCamera::new(1);
    assert!(cam.is_perspective());
    assert!(!cam.is_orthographic());
    assert!((cam.fov_y() - 45.0).abs() < 1e-10);
    // default eye=(0,0,100), center=(0,0,0) -> distance=100
    assert!((cam.distance() - 100.0).abs() < 1e-10);
}

#[test]
fn camera_direction() {
    // eye at (0,0,100), center at origin -> direction should be (0,0,-1)
    let cam = V3dCamera::new(1);
    let d = cam.direction();
    assert!(d[0].abs() < 1e-10);
    assert!(d[1].abs() < 1e-10);
    assert!((d[2] + 1.0).abs() < 1e-10);
    // Verify it's a unit vector
    let len = (d[0]*d[0] + d[1]*d[1] + d[2]*d[2]).sqrt();
    assert!((len - 1.0).abs() < 1e-10);
}

#[test]
fn camera_fit_all() {
    let mut cam = V3dCamera::new(1);
    cam.fit_all([-10.0, -10.0, -10.0], [10.0, 10.0, 10.0]);
    // center should be at bbox center (0,0,0)
    let c = cam.center();
    assert!(c[0].abs() < 1e-10);
    assert!(c[1].abs() < 1e-10);
    assert!(c[2].abs() < 1e-10);
    // z_far must be greater than z_near
    assert!(cam.z_far() > cam.z_near());
    // camera must have moved to some positive distance
    assert!(cam.distance() > 0.0);
}

#[test]
fn ambient_light_type_enabled_set_enabled() {
    let mut l = V3dLight::ambient(1, [1.0, 1.0, 1.0], 0.5);
    assert_eq!(l.light_type(), LightType::Ambient);
    assert!(l.is_enabled());
    l.set_enabled(false);
    assert!(!l.is_enabled());
    l.set_enabled(true);
    assert!(l.is_enabled());
}

#[test]
fn directional_light_type_direction_normalized() {
    // Provide a non-unit direction; constructor should normalize it
    let l = V3dLight::directional(2, [1.0, 0.8, 0.6], 1.0, [0.0, 0.0, -3.0]);
    assert_eq!(l.light_type(), LightType::Directional);
    let d = l.direction;
    let len = (d[0]*d[0] + d[1]*d[1] + d[2]*d[2]).sqrt();
    assert!((len - 1.0).abs() < 1e-10);
    // direction should be (0, 0, -1) after normalization
    assert!(d[0].abs() < 1e-10);
    assert!(d[1].abs() < 1e-10);
    assert!((d[2] + 1.0).abs() < 1e-10);
}

#[test]
fn trihedron_is_visible_set_visible_set_axis_color() {
    let mut t = V3dTrihedron::new();
    assert!(t.is_visible());
    t.set_visible(false);
    assert!(!t.is_visible());
    t.set_visible(true);
    assert!(t.is_visible());
    // X axis (index 0) color change
    t.set_axis_color(0, [0.5, 0.5, 0.5]);
    assert_eq!(t.color_x, [0.5, 0.5, 0.5]);
    // Y axis (index 1) color change
    t.set_axis_color(1, [0.2, 0.8, 0.0]);
    assert_eq!(t.color_y, [0.2, 0.8, 0.0]);
    // Z axis (index 2) color change
    t.set_axis_color(2, [0.0, 0.3, 1.0]);
    assert_eq!(t.color_z, [0.0, 0.3, 1.0]);
}
