// FILE: rust/ironstream/crates/ironstream/tests/occt_prs3d_text.rs
extern crate ironstream;
use ironstream::prs3d_text::*;

#[test]
fn text_aspect_default_color_is_black_opaque() {
    let asp = Prs3dTextAspect::new();
    assert_eq!(asp.color(), [0.0f32, 0.0, 0.0, 1.0]);
}

#[test]
fn text_aspect_default_font_is_courier() {
    let asp = Prs3dTextAspect::new();
    assert_eq!(asp.font(), "Courier");
}

#[test]
fn text_aspect_default_height_is_12() {
    let asp = Prs3dTextAspect::new();
    assert!((asp.height() - 12.0).abs() < 1e-12);
}

#[test]
fn text_aspect_default_h_align_is_left() {
    let asp = Prs3dTextAspect::new();
    assert_eq!(asp.h_align(), Prs3dTextHAlign::Left);
}

#[test]
fn text_aspect_default_v_align_is_bottom() {
    let asp = Prs3dTextAspect::new();
    assert_eq!(asp.v_align(), Prs3dTextVAlign::Bottom);
}

#[test]
fn text_aspect_set_color_roundtrip() {
    let mut asp = Prs3dTextAspect::new();
    asp.set_color([0.1, 0.2, 0.3, 0.5]);
    assert_eq!(asp.color(), [0.1f32, 0.2, 0.3, 0.5]);
}

#[test]
fn text_aspect_set_font_roundtrip() {
    let mut asp = Prs3dTextAspect::new();
    asp.set_font("Arial");
    assert_eq!(asp.font(), "Arial");
}

#[test]
fn text_aspect_set_height_roundtrip() {
    let mut asp = Prs3dTextAspect::new();
    asp.set_height(24.0);
    assert!((asp.height() - 24.0).abs() < 1e-12);
}

#[test]
fn text_aspect_set_h_align_roundtrip() {
    let mut asp = Prs3dTextAspect::new();
    asp.set_h_align(Prs3dTextHAlign::Right);
    assert_eq!(asp.h_align(), Prs3dTextHAlign::Right);
    asp.set_h_align(Prs3dTextHAlign::Center);
    assert_eq!(asp.h_align(), Prs3dTextHAlign::Center);
}

#[test]
fn text_aspect_set_v_align_roundtrip() {
    let mut asp = Prs3dTextAspect::new();
    asp.set_v_align(Prs3dTextVAlign::Top);
    assert_eq!(asp.v_align(), Prs3dTextVAlign::Top);
    asp.set_v_align(Prs3dTextVAlign::Center);
    assert_eq!(asp.v_align(), Prs3dTextVAlign::Center);
}

#[test]
fn text_aspect_default_trait_matches_new() {
    let a = Prs3dTextAspect::new();
    let b = Prs3dTextAspect::default();
    assert_eq!(a.color(), b.color());
    assert_eq!(a.font(), b.font());
    assert!((a.height() - b.height()).abs() < 1e-12);
    assert_eq!(a.h_align(), b.h_align());
    assert_eq!(a.v_align(), b.v_align());
}

#[test]
fn h_align_variants_are_distinct() {
    assert_ne!(Prs3dTextHAlign::Left, Prs3dTextHAlign::Center);
    assert_ne!(Prs3dTextHAlign::Center, Prs3dTextHAlign::Right);
    assert_ne!(Prs3dTextHAlign::Left, Prs3dTextHAlign::Right);
}

#[test]
fn v_align_variants_are_distinct() {
    assert_ne!(Prs3dTextVAlign::Bottom, Prs3dTextVAlign::Center);
    assert_ne!(Prs3dTextVAlign::Center, Prs3dTextVAlign::Top);
    assert_ne!(Prs3dTextVAlign::Bottom, Prs3dTextVAlign::Top);
}

#[test]
fn text_new_stores_text_and_position() {
    let t = Prs3dText::new("Hello", [1.0, 2.0, 3.0]);
    assert_eq!(t.text(), "Hello");
    assert_eq!(t.position(), [1.0, 2.0, 3.0]);
}

#[test]
fn text_new_has_default_aspect() {
    let t = Prs3dText::new("X", [0.0, 0.0, 0.0]);
    assert_eq!(t.aspect().color(), [0.0f32, 0.0, 0.0, 1.0]);
    assert_eq!(t.aspect().font(), "Courier");
    assert!((t.aspect().height() - 12.0).abs() < 1e-12);
    assert_eq!(t.aspect().h_align(), Prs3dTextHAlign::Left);
    assert_eq!(t.aspect().v_align(), Prs3dTextVAlign::Bottom);
}

#[test]
fn text_set_text_roundtrip() {
    let mut t = Prs3dText::new("old", [0.0, 0.0, 0.0]);
    t.set_text("new");
    assert_eq!(t.text(), "new");
}

#[test]
fn text_set_position_roundtrip() {
    let mut t = Prs3dText::new("P", [0.0, 0.0, 0.0]);
    t.set_position([4.0, 5.0, 6.0]);
    assert_eq!(t.position(), [4.0, 5.0, 6.0]);
}

#[test]
fn text_aspect_mut_modifies_aspect() {
    let mut t = Prs3dText::new("Label", [0.0, 0.0, 0.0]);
    t.aspect_mut().set_font("Times");
    t.aspect_mut().set_height(18.0);
    t.aspect_mut().set_h_align(Prs3dTextHAlign::Right);
    t.aspect_mut().set_v_align(Prs3dTextVAlign::Top);
    t.aspect_mut().set_color([1.0, 0.0, 0.0, 1.0]);
    assert_eq!(t.aspect().font(), "Times");
    assert!((t.aspect().height() - 18.0).abs() < 1e-12);
    assert_eq!(t.aspect().h_align(), Prs3dTextHAlign::Right);
    assert_eq!(t.aspect().v_align(), Prs3dTextVAlign::Top);
    assert_eq!(t.aspect().color(), [1.0f32, 0.0, 0.0, 1.0]);
}
