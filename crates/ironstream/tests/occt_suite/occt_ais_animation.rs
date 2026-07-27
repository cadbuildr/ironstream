extern crate ironstream;

use ironstream::ais_animation::{Animation, AnimationCamera, AnimationObject, AnimationTimer};

#[test]
fn animation_end_time() {
    let mut a = Animation::new("seq");
    a.set_start_time(1.0);
    a.set_duration(3.0);
    assert!((a.end_time() - 4.0).abs() < 1e-10);
}

#[test]
fn animation_update_clamping() {
    let mut a = Animation::new("clamp");
    a.set_duration(1.0);
    assert!((a.update(-1.0) - 0.0).abs() < 1e-10);
    assert!((a.update(2.0) - 1.0).abs() < 1e-10);
    assert!((a.update(0.5) - 0.5).abs() < 1e-10);
}

#[test]
fn camera_animation_interpolation() {
    let mut cam = AnimationCamera::new(
        "cam",
        [0.0, 0.0, 0.0], [10.0, 0.0, 0.0],
        [0.0, 0.0, -1.0], [10.0, 0.0, -1.0],
        [0.0, 1.0, 0.0], [0.0, 1.0, 0.0],
    );
    cam.base.set_duration(2.0);
    let eye = cam.eye_at(1.0);
    assert!((eye[0] - 5.0).abs() < 1e-10);
    let center = cam.center_at(1.0);
    assert!((center[0] - 5.0).abs() < 1e-10);
}

#[test]
fn animation_object_keyframe_interpolation() {
    let mut ao = AnimationObject::new("box");
    ao.add_keyframe(0.0, [0.0, 0.0, 0.0]);
    ao.add_keyframe(2.0, [20.0, 0.0, 0.0]);
    let t = ao.translation_at(1.0);
    assert!((t[0] - 10.0).abs() < 1e-10);
}

#[test]
fn animation_timer_speed() {
    let mut timer = AnimationTimer::new();
    timer.speed = 2.0;
    timer.play();
    timer.advance(1.0);
    assert!((timer.elapsed - 2.0).abs() < 1e-14);
    timer.stop();
    assert!((timer.elapsed).abs() < 1e-14);
    assert!(!timer.is_playing);
}
