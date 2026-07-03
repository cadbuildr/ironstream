use ironstream::prs3d_text_prop::*;

#[test]
fn text_aspect_defaults_courier_height_16_center() {
    let t = Prs3dTextAspect::new();
    assert_eq!(t.font_name(), "Courier");
    assert!((t.height() - 16.0).abs() < 1e-10);
    assert_eq!(t.h_align(), TextHAlign::Center);
    assert_eq!(t.v_align(), TextVAlign::Center);
}

#[test]
fn text_aspect_setters_font_height_halign() {
    let mut t = Prs3dTextAspect::new();
    t.set_font("Arial");
    t.set_height(24.0);
    t.set_h_align(TextHAlign::Left);
    assert_eq!(t.font_name(), "Arial");
    assert!((t.height() - 24.0).abs() < 1e-10);
    assert_eq!(t.h_align(), TextHAlign::Left);
}

#[test]
fn text_aspect_height_clamped_min_1() {
    let mut t = Prs3dTextAspect::new();
    t.set_height(-5.0);
    assert!(t.height() >= 1.0);
    t.set_height(0.0);
    assert!(t.height() >= 1.0);
}

#[test]
fn font_mgr_register_find_bold_aspect() {
    let mut mgr = FontFontMgr::new();
    mgr.register_font(FontDescriptor::new("Arial", "/fonts/arial.ttf", FontAspect::Regular));
    mgr.register_font(FontDescriptor::new("Arial", "/fonts/arialb.ttf", FontAspect::Bold));
    assert_eq!(mgr.nb_fonts(), 2);
    let f = mgr.find_font("Arial", FontAspect::Bold);
    assert!(f.is_some());
    assert_eq!(f.unwrap().aspect(), FontAspect::Bold);
}

#[test]
fn font_mgr_available_families_dedup() {
    let mut mgr = FontFontMgr::new();
    mgr.register_font(FontDescriptor::new("Arial", "", FontAspect::Regular));
    mgr.register_font(FontDescriptor::new("Arial", "", FontAspect::Bold));
    mgr.register_font(FontDescriptor::new("Courier", "", FontAspect::Regular));
    let families = mgr.available_families();
    assert_eq!(families.len(), 2);
    assert!(families.contains(&String::from("Arial")));
    assert!(families.contains(&String::from("Courier")));
}
