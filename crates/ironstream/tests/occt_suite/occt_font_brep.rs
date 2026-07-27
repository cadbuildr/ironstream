// FILE: tests/occt_font_brep.rs
extern crate ironstream;

use ironstream::font_brep::{BRepFont, BRepTextBuilder, FontAspect, FontManager, FontUnit, SystemFont};

#[test]
fn font_manager_register_and_find() {
    let mut mgr = FontManager::new();
    mgr.register_font(SystemFont::new("Arial", FontAspect::Regular, "/fonts/arial.ttf"));
    mgr.register_font(SystemFont::new("Arial", FontAspect::Bold, "/fonts/arialbd.ttf"));
    assert_eq!(mgr.nb_fonts(), 2);
    let f = mgr.find("Arial", FontAspect::Bold).unwrap();
    assert!(f.is_bold());
    assert!(!f.is_italic());
    assert!(mgr.find("Arial", FontAspect::Regular).is_some());
    assert!(mgr.find("Helvetica", FontAspect::Regular).is_none());
}

#[test]
fn brep_font_string_width_and_size_in_mm() {
    let f = BRepFont::new("Mono", FontAspect::Regular, 10.0)
        .with_unit(FontUnit::Mm)
        .with_fixed_advance(6.0);
    assert!((f.size_in_mm() - 10.0).abs() < 1e-10);
    let w = f.string_width("Hello");
    assert!((w - 30.0).abs() < 1e-10, "5 chars * 6mm = 30mm, got {}", w);
}

#[test]
fn brep_text_builder_nb_glyphs() {
    let font = BRepFont::new("Mono", FontAspect::Regular, 10.0).with_fixed_advance(5.0);
    let builder = BRepTextBuilder::new(font);
    let shape = builder.build("ABC", [0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
    assert_eq!(shape.nb_glyphs, 3);
    assert!((shape.width - 15.0).abs() < 1e-10);
}

#[test]
fn font_unit_to_mm_factors() {
    assert!((FontUnit::Mm.to_mm_factor() - 1.0).abs() < 1e-10);
    assert!((FontUnit::Inch.to_mm_factor() - 25.4).abs() < 1e-10);
    assert!(FontUnit::Point.to_mm_factor() > 0.0);
    assert!(FontUnit::Pixel.to_mm_factor() > 0.0);
}
