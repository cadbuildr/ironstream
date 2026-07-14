// FILE: tests/occt_aspect_background.rs
extern crate ironstream;

use ironstream::aspect_background::{AnyBackground, Background, GradientBackground, GradientFillMethod};

#[test]
fn background_brightness_dark() {
    let b = Background::new(0.0, 0.0, 0.0);
    assert!(b.is_dark(), "black should be dark");
    assert!((b.brightness()).abs() < 1e-6);
}

#[test]
fn background_brightness_light() {
    let w = Background::WHITE;
    assert!(!w.is_dark(), "white should not be dark");
    assert!((w.brightness() - 1.0).abs() < 1e-5);
}

#[test]
fn gradient_background_color_at_midpoint() {
    let g = GradientBackground::vertical([0.0, 0.0, 0.0], [1.0, 1.0, 1.0]);
    let mid = g.color_at(0.5);
    assert!((mid[0] - 0.5).abs() < 1e-6);
    assert!((mid[1] - 0.5).abs() < 1e-6);
    assert!((mid[2] - 0.5).abs() < 1e-6);
}

#[test]
fn gradient_background_endpoints_clamped() {
    let g = GradientBackground::vertical([1.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
    let start = g.color_at(0.0);
    let end = g.color_at(1.0);
    assert!((start[0] - 1.0).abs() < 1e-6, "start red should be 1");
    assert!((end[2] - 1.0).abs() < 1e-6, "end blue should be 1");
    // clamp below 0 gives same as 0
    let below = g.color_at(-1.0);
    assert!((below[0] - start[0]).abs() < 1e-6);
}

#[test]
fn any_background_is_gradient() {
    let solid = AnyBackground::Solid(Background::new(0.3, 0.3, 0.3));
    assert!(!solid.is_gradient());

    let grad = AnyBackground::Gradient(GradientBackground::new(
        [0.0, 0.0, 0.0],
        [1.0, 1.0, 1.0],
        GradientFillMethod::Horizontal,
    ));
    assert!(grad.is_gradient());
}
