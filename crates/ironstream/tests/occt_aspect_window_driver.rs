use ironstream::aspect_window_driver::*;

#[test]
fn vkey_is_modifier_is_arrow_is_digit() {
    assert!(AspectVKey::Shift.is_modifier());
    assert!(AspectVKey::Control.is_modifier());
    assert!(AspectVKey::Alt.is_modifier());
    assert!(AspectVKey::Meta.is_modifier());
    assert!(!AspectVKey::A.is_modifier());

    assert!(AspectVKey::Left.is_arrow());
    assert!(AspectVKey::Right.is_arrow());
    assert!(AspectVKey::Up.is_arrow());
    assert!(AspectVKey::Down.is_arrow());
    assert!(!AspectVKey::Space.is_arrow());

    assert!(AspectVKey::Digit0.is_digit());
    assert!(AspectVKey::Digit9.is_digit());
    assert!(!AspectVKey::F1.is_digit());
}

#[test]
fn aspect_key_flags_with_methods() {
    let f = AspectKeyFlags::NONE.with_shift().with_ctrl();
    assert!(f.has_shift());
    assert!(f.has_ctrl());
    assert!(!f.has_alt());
    assert!(!f.has_meta());

    let f2 = AspectKeyFlags::NONE.with_alt();
    assert!(f2.has_alt());
    assert!(!f2.has_shift());
}

#[test]
fn aspect_vkey_set_press_release_clear() {
    let mut ks = AspectVKeySet::new();
    ks.press(AspectVKey::A);
    ks.press(AspectVKey::Shift);
    assert!(ks.is_pressed(AspectVKey::A));
    assert!(ks.flags.has_shift());
    assert_eq!(ks.nb_pressed(), 2);

    ks.release(AspectVKey::Shift);
    assert!(!ks.flags.has_shift());
    assert!(ks.is_pressed(AspectVKey::A));

    ks.clear();
    assert_eq!(ks.nb_pressed(), 0);
    assert!(!ks.flags.has_shift());
}

#[test]
fn aspect_display_connection_connect_disconnect() {
    let mut dc = AspectDisplayConnection::new();
    assert!(!dc.is_connected());
    dc.connect(":0.0");
    assert!(dc.is_connected());
    assert_eq!(dc.display_name, ":0.0");

    let (w, h) = dc.physical_size_mm();
    // 1920 pixels / 96 dpi * 25.4 = 508 mm
    assert!((w - 508.0).abs() < 1.0);
    assert!(h > 0.0);

    dc.disconnect();
    assert!(!dc.is_connected());
}

#[test]
fn aspect_window_driver_show_hide_resize_move() {
    let mut wd = AspectWindowDriver::new(800, 600, "TestWindow");
    assert!(!wd.is_visible);
    wd.show();
    assert!(wd.is_visible);

    let ratio = wd.aspect_ratio();
    assert!((ratio - 800.0 / 600.0).abs() < 1e-4);

    wd.resize(1920, 1080);
    assert_eq!(wd.width, 1920);
    assert_eq!(wd.height, 1080);

    wd.move_to(100, 200);
    assert_eq!(wd.position, [100, 200]);

    wd.hide();
    assert!(!wd.is_visible);
}

#[test]
fn aspect_window_driver_fullscreen_and_zero_height() {
    let mut wd = AspectWindowDriver::new(1920, 1080, "Full");
    wd.set_fullscreen(true);
    assert!(wd.is_fullscreen);

    // Zero height should not panic, returns 1.0
    let mut wd2 = AspectWindowDriver::new(800, 0, "ZeroHeight");
    assert!((wd2.aspect_ratio() - 1.0).abs() < 1e-6);
    wd2.resize(0, 0);
    assert!((wd2.aspect_ratio() - 1.0).abs() < 1e-6);
}
