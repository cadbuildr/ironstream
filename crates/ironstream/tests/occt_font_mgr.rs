extern crate ironstream;
use ironstream::font_mgr::*;

#[test]
fn test_font_mgr_basic() {
    // FontInfo
    let info = FontInfo::new("Arial", "/usr/share/fonts/arial.ttf");
    assert_eq!(info.name, "Arial");
    assert_eq!(info.path, "/usr/share/fonts/arial.ttf");
    assert_eq!(info.aspect, FontAspect::Regular);

    // FontMgr
    let mut mgr = FontMgr::new();
    assert!(!mgr.has_font("Arial"));

    mgr.register_font(info);
    assert!(mgr.has_font("Arial"));

    let found = mgr.find_font("Arial", FontAspect::Regular);
    assert!(found.is_some());
    assert_eq!(found.unwrap().name, "Arial");

    // No Bold registered: FindFont falls back to the closest name match
    // (occt: Font_FontMgr::FindFont aspect fallback).
    let fallback = mgr.find_font("Arial", FontAspect::Bold);
    assert!(fallback.is_some());
    assert_eq!(fallback.unwrap().aspect, FontAspect::Regular);

    let available = mgr.available_fonts();
    assert!(available.contains(&"Arial"));

    // FontAspect variants
    assert_ne!(FontAspect::Regular, FontAspect::Bold);
    assert_ne!(FontAspect::Italic, FontAspect::BoldItalic);

    // BRepFont
    let font = BRepFont::new("Arial", 12.0);
    assert_eq!(font.font, "Arial");
    assert!((font.size - 12.0).abs() < 1e-12);
    let contours = font.render_char('A');
    // stub returns empty contours
    assert_eq!(contours.len(), 0);
}
