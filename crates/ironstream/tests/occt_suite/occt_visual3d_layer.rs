// FILE: tests/occt_visual3d_layer.rs
extern crate ironstream;
use ironstream::visual3d_layer::*;

#[test]
fn test_z_layer_id_values_and_overlay() {
    assert_eq!(ZLayerId::Default.id(), 0);
    assert_eq!(ZLayerId::Top.id(), 1);
    assert_eq!(ZLayerId::Topmost.id(), 2);
    assert_eq!(ZLayerId::TopOSD.id(), 4);
    assert!(ZLayerId::TopOSD.is_overlay());
    assert!(ZLayerId::BotOSD.is_overlay());
    assert!(!ZLayerId::Top.is_overlay());
    assert_eq!(ZLayerId::from_id(2), ZLayerId::Topmost);
    assert_eq!(ZLayerId::from_id(99), ZLayerId::Custom(99));
}

#[test]
fn test_z_layer_settings_presets() {
    let default_s = ZLayerSettings::default_layer();
    assert!(default_s.is_depth_test_enabled);
    assert!(default_s.is_lighting_enabled);

    let top_s = ZLayerSettings::top_layer();
    assert!(!top_s.is_depth_test_enabled);

    let overlay = ZLayerSettings::overlay();
    assert!(!overlay.is_depth_test_enabled);
    assert!(!overlay.is_lighting_enabled);
    assert!(!overlay.is_fog_enabled);

    let with_offset = ZLayerSettings::default_layer().with_depth_offset(2.0, 1.0);
    assert!(with_offset.is_depth_offset_enabled);
    assert!((with_offset.depth_offset_factor - 2.0).abs() < 1e-6);
}

#[test]
fn test_z_layer_presentations() {
    let mut layer = ZLayer::new(ZLayerSettings::default_layer());
    assert!(layer.is_empty());
    layer.add_presentation(10);
    layer.add_presentation(20);
    assert_eq!(layer.nb_presentations(), 2);
    layer.remove_presentation(10);
    assert_eq!(layer.nb_presentations(), 1);
    assert_eq!(layer.layer_id(), ZLayerId::Default);
}

#[test]
fn test_z_layer_manager_init_and_add() {
    let mut mgr = ZLayerManager::new();
    assert_eq!(mgr.nb_layers(), 3); // Default, Top, TopOSD
    assert!(mgr.layer(ZLayerId::Default).is_some());

    mgr.add_presentation_to_layer(ZLayerId::Default, 42);
    let layer = mgr.layer(ZLayerId::Default).unwrap();
    assert_eq!(layer.nb_presentations(), 1);

    let new_id = mgr.add_layer(
        ZLayerSettings::new(ZLayerId::Custom(10)).with_name("MyLayer").with_priority(5),
    );
    assert_eq!(new_id, ZLayerId::Custom(10));
    assert_eq!(mgr.nb_layers(), 4);
}

#[test]
fn test_visual3d_layer_draw_calls() {
    let mut layer = Visual3dLayer::new(0, 1920, 1080);
    assert!((layer.aspect_ratio() - 1920.0 / 1080.0).abs() < 1e-10);
    layer.begin();
    layer.draw_line(0.0, 0.0, 100.0, 100.0, [1.0, 0.0, 0.0]);
    layer.draw_text(10.0, 10.0, "label", [1.0, 1.0, 1.0]);
    layer.draw_rect(5.0, 5.0, 50.0, 30.0, [0.0, 1.0, 0.0]);
    assert_eq!(layer.nb_draw_calls(), 3);
    layer.end();
    assert!(!layer.begin_frame_called);
}
