// FILE: rust/ironstream/crates/ironstream/tests/occt_aspect_window.rs
extern crate ironstream;
use ironstream::aspect_window::*;

#[test]
fn params_new_sets_width_and_height() {
    let p = AspectWindowParams::new(1280, 720);
    assert_eq!(p.width, 1280);
    assert_eq!(p.height, 720);
}

#[test]
fn params_new_defaults_position_zero() {
    let p = AspectWindowParams::new(100, 100);
    assert_eq!(p.position(), (0, 0));
}

#[test]
fn params_new_defaults_title_empty() {
    let p = AspectWindowParams::new(100, 100);
    assert_eq!(p.title(), "");
}

#[test]
fn params_new_defaults_background_color_mid_grey() {
    let p = AspectWindowParams::new(100, 100);
    let bg = p.background_color();
    assert!((bg[0] - 0.5).abs() < f32::EPSILON, "bg[0] should be 0.5, got {}", bg[0]);
    assert!((bg[1] - 0.5).abs() < f32::EPSILON, "bg[1] should be 0.5, got {}", bg[1]);
    assert!((bg[2] - 0.5).abs() < f32::EPSILON, "bg[2] should be 0.5, got {}", bg[2]);
}

#[test]
fn params_set_title_roundtrip() {
    let mut p = AspectWindowParams::new(800, 600);
    p.set_title("OpenCascade Viewer");
    assert_eq!(p.title(), "OpenCascade Viewer");
}

#[test]
fn params_set_title_empty_string() {
    let mut p = AspectWindowParams::new(800, 600);
    p.set_title("Initial");
    p.set_title("");
    assert_eq!(p.title(), "");
}

#[test]
fn params_set_background_color_roundtrip() {
    let mut p = AspectWindowParams::new(100, 100);
    p.set_background_color([0.0, 0.25, 1.0]);
    let bg = p.background_color();
    assert!((bg[0] - 0.0).abs() < f32::EPSILON);
    assert!((bg[1] - 0.25).abs() < f32::EPSILON);
    assert!((bg[2] - 1.0).abs() < f32::EPSILON);
}

#[test]
fn params_set_position_roundtrip() {
    let mut p = AspectWindowParams::new(200, 200);
    p.set_position(100, 200);
    assert_eq!(p.position(), (100, 200));
}

#[test]
fn params_set_position_negative_coords() {
    let mut p = AspectWindowParams::new(200, 200);
    p.set_position(-50, -100);
    assert_eq!(p.position(), (-50, -100));
}

#[test]
fn params_set_position_twice_last_wins() {
    let mut p = AspectWindowParams::new(200, 200);
    p.set_position(10, 20);
    p.set_position(30, 40);
    assert_eq!(p.position(), (30, 40));
}

#[test]
fn params_clone_independent() {
    let mut p = AspectWindowParams::new(640, 480);
    p.set_title("original");
    p.set_position(5, 5);
    let mut p2 = p.clone();
    p2.set_title("clone");
    p2.set_position(99, 99);
    // Original unchanged
    assert_eq!(p.title(), "original");
    assert_eq!(p.position(), (5, 5));
    // Clone has new values
    assert_eq!(p2.title(), "clone");
    assert_eq!(p2.position(), (99, 99));
}

#[test]
fn window_new_starts_unmapped() {
    let p = AspectWindowParams::new(800, 600);
    let w = AspectWindow::new(p, AspectWindowType::NativeWindow);
    assert!(!w.is_mapped());
}

#[test]
fn window_map_sets_is_mapped_true() {
    let p = AspectWindowParams::new(800, 600);
    let mut w = AspectWindow::new(p, AspectWindowType::NativeWindow);
    w.map();
    assert!(w.is_mapped());
}

#[test]
fn window_unmap_sets_is_mapped_false() {
    let p = AspectWindowParams::new(800, 600);
    let mut w = AspectWindow::new(p, AspectWindowType::NativeWindow);
    w.map();
    w.unmap();
    assert!(!w.is_mapped());
}

#[test]
fn window_map_idempotent() {
    let p = AspectWindowParams::new(100, 100);
    let mut w = AspectWindow::new(p, AspectWindowType::VirtualWindow);
    w.map();
    w.map();
    assert!(w.is_mapped());
}

#[test]
fn window_unmap_idempotent_when_already_unmapped() {
    let p = AspectWindowParams::new(100, 100);
    let mut w = AspectWindow::new(p, AspectWindowType::VirtualWindow);
    w.unmap();
    assert!(!w.is_mapped());
}

#[test]
fn window_width_matches_params() {
    let p = AspectWindowParams::new(1920, 1080);
    let w = AspectWindow::new(p, AspectWindowType::NativeWindow);
    assert_eq!(w.width(), 1920);
}

#[test]
fn window_height_matches_params() {
    let p = AspectWindowParams::new(1920, 1080);
    let w = AspectWindow::new(p, AspectWindowType::NativeWindow);
    assert_eq!(w.height(), 1080);
}

#[test]
fn window_size_matches_params() {
    let p = AspectWindowParams::new(640, 480);
    let w = AspectWindow::new(p, AspectWindowType::OffscreenBuffer);
    assert_eq!(w.size(), (640, 480));
}

#[test]
fn window_position_delegates_to_params() {
    let mut p = AspectWindowParams::new(100, 100);
    p.set_position(-10, 25);
    let w = AspectWindow::new(p, AspectWindowType::NativeWindow);
    assert_eq!(w.position(), (-10, 25));
}

#[test]
fn window_aspect_ratio_16_9() {
    let p = AspectWindowParams::new(1920, 1080);
    let w = AspectWindow::new(p, AspectWindowType::NativeWindow);
    assert!((w.aspect_ratio() - 16.0_f32 / 9.0_f32).abs() < 1e-5,
        "expected 16/9, got {}", w.aspect_ratio());
}

#[test]
fn window_aspect_ratio_4_3() {
    let p = AspectWindowParams::new(800, 600);
    let w = AspectWindow::new(p, AspectWindowType::NativeWindow);
    assert!((w.aspect_ratio() - 4.0_f32 / 3.0_f32).abs() < 1e-5,
        "expected 4/3, got {}", w.aspect_ratio());
}

#[test]
fn window_aspect_ratio_square_is_one() {
    let p = AspectWindowParams::new(512, 512);
    let w = AspectWindow::new(p, AspectWindowType::NativeWindow);
    assert!((w.aspect_ratio() - 1.0).abs() < f32::EPSILON);
}

#[test]
fn window_aspect_ratio_zero_height_returns_one() {
    // Guard against divide-by-zero
    let p = AspectWindowParams::new(1280, 0);
    let w = AspectWindow::new(p, AspectWindowType::OffscreenBuffer);
    assert!((w.aspect_ratio() - 1.0).abs() < f32::EPSILON,
        "zero-height window should return aspect_ratio == 1.0");
}

#[test]
fn window_aspect_ratio_zero_width() {
    let p = AspectWindowParams::new(0, 480);
    let w = AspectWindow::new(p, AspectWindowType::OffscreenBuffer);
    assert!((w.aspect_ratio() - 0.0).abs() < f32::EPSILON,
        "zero-width window should return aspect_ratio == 0.0");
}

#[test]
fn window_window_type_native() {
    let p = AspectWindowParams::new(100, 100);
    let w = AspectWindow::new(p, AspectWindowType::NativeWindow);
    assert_eq!(w.window_type(), AspectWindowType::NativeWindow);
}

#[test]
fn window_window_type_offscreen() {
    let p = AspectWindowParams::new(100, 100);
    let w = AspectWindow::new(p, AspectWindowType::OffscreenBuffer);
    assert_eq!(w.window_type(), AspectWindowType::OffscreenBuffer);
}

#[test]
fn window_window_type_virtual() {
    let p = AspectWindowParams::new(100, 100);
    let w = AspectWindow::new(p, AspectWindowType::VirtualWindow);
    assert_eq!(w.window_type(), AspectWindowType::VirtualWindow);
}

#[test]
fn window_params_ref_reflects_construction() {
    let mut p = AspectWindowParams::new(1024, 768);
    p.set_title("params ref");
    p.set_position(7, 8);
    p.set_background_color([0.1, 0.2, 0.3]);
    let w = AspectWindow::new(p, AspectWindowType::VirtualWindow);
    let rp = w.params();
    assert_eq!(rp.width, 1024);
    assert_eq!(rp.height, 768);
    assert_eq!(rp.title(), "params ref");
    assert_eq!(rp.position(), (7, 8));
    let bg = rp.background_color();
    assert!((bg[0] - 0.1).abs() < f32::EPSILON);
    assert!((bg[1] - 0.2).abs() < f32::EPSILON);
    assert!((bg[2] - 0.3).abs() < f32::EPSILON);
}

#[test]
fn window_clone_independent_map_state() {
    let p = AspectWindowParams::new(800, 600);
    let mut w1 = AspectWindow::new(p, AspectWindowType::NativeWindow);
    w1.map();
    let mut w2 = w1.clone();
    w2.unmap();
    // w1 still mapped, w2 unmapped
    assert!(w1.is_mapped());
    assert!(!w2.is_mapped());
}

#[test]
fn aspect_window_type_three_variants_all_distinct() {
    let variants = [
        AspectWindowType::NativeWindow,
        AspectWindowType::OffscreenBuffer,
        AspectWindowType::VirtualWindow,
    ];
    assert_eq!(variants.len(), 3);
    for i in 0..variants.len() {
        for j in 0..variants.len() {
            if i == j {
                assert_eq!(variants[i], variants[j], "variant {} should equal itself", i);
            } else {
                assert_ne!(variants[i], variants[j],
                    "variants {} and {} should differ", i, j);
            }
        }
    }
}

#[test]
fn aspect_window_type_is_copy() {
    let t = AspectWindowType::OffscreenBuffer;
    let t2 = t;
    assert_eq!(t, t2);
}

#[test]
fn aspect_window_type_debug_format() {
    assert_eq!(format!("{:?}", AspectWindowType::NativeWindow), "NativeWindow");
    assert_eq!(format!("{:?}", AspectWindowType::OffscreenBuffer), "OffscreenBuffer");
    assert_eq!(format!("{:?}", AspectWindowType::VirtualWindow), "VirtualWindow");
}

#[test]
fn window_debug_format_non_empty() {
    let p = AspectWindowParams::new(200, 100);
    let w = AspectWindow::new(p, AspectWindowType::NativeWindow);
    let s = format!("{:?}", w);
    assert!(!s.is_empty());
}

#[test]
fn params_debug_format_non_empty() {
    let p = AspectWindowParams::new(320, 240);
    let s = format!("{:?}", p);
    assert!(!s.is_empty());
}
