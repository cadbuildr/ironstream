// FILE: tests/occt_ais.rs
extern crate ironstream;
use ironstream::ais::*;

#[test]
fn display_mode_wireframe_discriminant_is_zero() {
    assert_eq!(AisDisplayMode::Wireframe as i32, 0);
}

#[test]
fn display_mode_shaded_discriminant_is_one() {
    assert_eq!(AisDisplayMode::Shaded as i32, 1);
}

#[test]
fn display_mode_copy_and_eq() {
    let m = AisDisplayMode::Wireframe;
    let m2 = m;
    assert_eq!(m, m2);
    assert_ne!(m, AisDisplayMode::Shaded);
}

#[test]
fn selection_mode_all_discriminants() {
    assert_eq!(AisSelectionMode::Default as i32, 0);
    assert_eq!(AisSelectionMode::Vertex as i32, 1);
    assert_eq!(AisSelectionMode::Edge as i32, 2);
    assert_eq!(AisSelectionMode::Wire as i32, 3);
    assert_eq!(AisSelectionMode::Face as i32, 4);
    assert_eq!(AisSelectionMode::Shell as i32, 5);
    assert_eq!(AisSelectionMode::Solid as i32, 6);
}

#[test]
fn type_of_iso_three_variants_distinct() {
    assert_ne!(AisTypeOfIso::U, AisTypeOfIso::V);
    assert_ne!(AisTypeOfIso::U, AisTypeOfIso::Both);
    assert_ne!(AisTypeOfIso::V, AisTypeOfIso::Both);
}

#[test]
fn interactive_object_new_defaults() {
    let obj = AisInteractiveObject::new();
    assert_eq!(obj.display_mode(), AisDisplayMode::Shaded);
    assert!(!obj.is_displayed());
    assert_eq!(obj.transparency(), 0.0);
}

#[test]
fn interactive_object_toggle_display() {
    let mut obj = AisInteractiveObject::new();
    obj.set_displayed(true);
    assert!(obj.is_displayed());
    obj.set_displayed(false);
    assert!(!obj.is_displayed());
}

#[test]
fn interactive_object_set_wireframe_mode() {
    let mut obj = AisInteractiveObject::new();
    obj.set_display_mode(AisDisplayMode::Wireframe);
    assert_eq!(obj.display_mode(), AisDisplayMode::Wireframe);
}

#[test]
fn interactive_object_transparency_range() {
    let mut obj = AisInteractiveObject::new();
    obj.set_transparency(0.0);
    assert_eq!(obj.transparency(), 0.0);
    obj.set_transparency(0.5);
    assert_eq!(obj.transparency(), 0.5);
    obj.set_transparency(1.0);
    assert_eq!(obj.transparency(), 1.0);
}

#[test]
fn ais_shape_stores_type_name() {
    let s = AisShape::new("Solid");
    assert_eq!(s.shape_type(), "Solid");
    let f = AisShape::new("Face");
    assert_eq!(f.shape_type(), "Face");
}

#[test]
fn ais_shape_base_delegation() {
    let mut s = AisShape::new("Wire");
    assert_eq!(s.display_mode(), AisDisplayMode::Shaded);
    assert!(!s.is_displayed());
    s.set_display_mode(AisDisplayMode::Wireframe);
    s.set_displayed(true);
    s.set_transparency(0.25);
    assert_eq!(s.display_mode(), AisDisplayMode::Wireframe);
    assert!(s.is_displayed());
    assert_eq!(s.transparency(), 0.25);
}

#[test]
fn context_new_is_empty_with_default_selection() {
    let ctx = AisInteractiveContext::new();
    assert_eq!(ctx.object_count(), 0);
    assert_eq!(ctx.selection_mode(), AisSelectionMode::Default);
}

#[test]
fn context_display_multiple_objects() {
    let mut ctx = AisInteractiveContext::new();
    ctx.display(AisInteractiveObject::new());
    ctx.display(AisInteractiveObject::new());
    ctx.display(AisInteractiveObject::new());
    assert_eq!(ctx.object_count(), 3);
}

#[test]
fn context_erase_all_clears_objects() {
    let mut ctx = AisInteractiveContext::new();
    ctx.display(AisInteractiveObject::new());
    ctx.display(AisInteractiveObject::new());
    ctx.erase_all();
    assert_eq!(ctx.object_count(), 0);
}

#[test]
fn context_erase_all_on_empty_is_safe() {
    let mut ctx = AisInteractiveContext::new();
    ctx.erase_all();
    assert_eq!(ctx.object_count(), 0);
}

#[test]
fn context_selection_mode_changes() {
    let mut ctx = AisInteractiveContext::new();
    for mode in [
        AisSelectionMode::Vertex,
        AisSelectionMode::Edge,
        AisSelectionMode::Wire,
        AisSelectionMode::Face,
        AisSelectionMode::Shell,
        AisSelectionMode::Solid,
        AisSelectionMode::Default,
    ] {
        ctx.set_selection_mode(mode);
        assert_eq!(ctx.selection_mode(), mode);
    }
}
