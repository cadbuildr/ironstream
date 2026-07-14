// FILE: tests/occt_ais_context.rs
extern crate ironstream;
use ironstream::ais_context::*;

// --- AisDisplayMode ---

#[test]
fn display_mode_wireframe_discriminant() {
    assert_eq!(AisDisplayMode::WireFrame as i32, 0);
}

#[test]
fn display_mode_shaded_discriminant() {
    assert_eq!(AisDisplayMode::Shaded as i32, 1);
}

#[test]
fn display_mode_copy_and_eq() {
    let m = AisDisplayMode::WireFrame;
    let m2 = m;
    assert_eq!(m, m2);
    assert_ne!(m, AisDisplayMode::Shaded);
}

// --- AisSelectionMode ---

#[test]
fn selection_mode_all_discriminant() {
    assert_eq!(AisSelectionMode::All as i32, 0);
}

#[test]
fn selection_mode_all_discriminants() {
    assert_eq!(AisSelectionMode::Vertex as i32, 1);
    assert_eq!(AisSelectionMode::Edge as i32, 2);
    assert_eq!(AisSelectionMode::Wire as i32, 3);
    assert_eq!(AisSelectionMode::Face as i32, 4);
    assert_eq!(AisSelectionMode::Shell as i32, 5);
    assert_eq!(AisSelectionMode::Solid as i32, 6);
}

#[test]
fn selection_mode_copy_and_eq() {
    let s = AisSelectionMode::Face;
    let s2 = s;
    assert_eq!(s, s2);
    assert_ne!(s, AisSelectionMode::Edge);
}

// --- AisDisplayedObject ---

#[test]
fn displayed_object_new_defaults() {
    let obj = AisDisplayedObject::new("box1", AisDisplayMode::Shaded);
    assert_eq!(obj.label(), "box1");
    assert_eq!(obj.display_mode(), AisDisplayMode::Shaded);
    assert!(!obj.is_selected());
    assert!(!obj.is_highlighted());
    assert_eq!(obj.layer(), 0);
}

#[test]
fn displayed_object_set_layer() {
    let mut obj = AisDisplayedObject::new("cyl", AisDisplayMode::WireFrame);
    obj.set_layer(3);
    assert_eq!(obj.layer(), 3);
}

#[test]
fn displayed_object_wireframe_mode() {
    let obj = AisDisplayedObject::new("edge", AisDisplayMode::WireFrame);
    assert_eq!(obj.display_mode(), AisDisplayMode::WireFrame);
}

// --- AisInteractiveContext ---

#[test]
fn context_new_is_empty() {
    let ctx = AisInteractiveContext::new();
    assert_eq!(ctx.nb_displayed(), 0);
    assert_eq!(ctx.nb_selected(), 0);
    assert_eq!(ctx.selected().len(), 0);
}

#[test]
fn context_default_highlight_color() {
    let ctx = AisInteractiveContext::new();
    let c = ctx.highlight_color();
    // default is yellow-ish [1,1,0,1]
    assert_eq!(c, [1.0f32, 1.0, 0.0, 1.0]);
}

#[test]
fn context_display_returns_index_and_increments_count() {
    let mut ctx = AisInteractiveContext::new();
    let i0 = ctx.display("box1", AisDisplayMode::Shaded);
    let i1 = ctx.display("box2", AisDisplayMode::WireFrame);
    assert_eq!(i0, 0);
    assert_eq!(i1, 1);
    assert_eq!(ctx.nb_displayed(), 2);
}

#[test]
fn context_display_same_label_updates_mode_no_duplicate() {
    let mut ctx = AisInteractiveContext::new();
    ctx.display("box1", AisDisplayMode::Shaded);
    let idx = ctx.display("box1", AisDisplayMode::WireFrame);
    assert_eq!(idx, 0);
    assert_eq!(ctx.nb_displayed(), 1);
    assert_eq!(ctx.object(0).display_mode(), AisDisplayMode::WireFrame);
}

#[test]
fn context_object_accessor() {
    let mut ctx = AisInteractiveContext::new();
    ctx.display("sphere", AisDisplayMode::Shaded);
    let obj = ctx.object(0);
    assert_eq!(obj.label(), "sphere");
    assert_eq!(obj.display_mode(), AisDisplayMode::Shaded);
}

#[test]
fn context_remove_existing() {
    let mut ctx = AisInteractiveContext::new();
    ctx.display("box1", AisDisplayMode::Shaded);
    ctx.display("box2", AisDisplayMode::Shaded);
    let removed = ctx.remove("box1");
    assert!(removed);
    assert_eq!(ctx.nb_displayed(), 1);
    assert_eq!(ctx.object(0).label(), "box2");
}

#[test]
fn context_remove_nonexistent_returns_false() {
    let mut ctx = AisInteractiveContext::new();
    ctx.display("box1", AisDisplayMode::Shaded);
    assert!(!ctx.remove("missing"));
    assert_eq!(ctx.nb_displayed(), 1);
}

#[test]
fn context_remove_clears_selection() {
    let mut ctx = AisInteractiveContext::new();
    ctx.display("box1", AisDisplayMode::Shaded);
    ctx.select("box1");
    assert_eq!(ctx.nb_selected(), 1);
    ctx.remove("box1");
    assert_eq!(ctx.nb_selected(), 0);
}

#[test]
fn context_select_marks_object_and_populates_selected() {
    let mut ctx = AisInteractiveContext::new();
    ctx.display("box1", AisDisplayMode::Shaded);
    ctx.select("box1");
    assert!(ctx.object(0).is_selected());
    assert!(ctx.object(0).is_highlighted());
    assert_eq!(ctx.nb_selected(), 1);
    assert_eq!(ctx.selected(), &["box1".to_string()]);
}

#[test]
fn context_select_nonexistent_does_not_crash() {
    let mut ctx = AisInteractiveContext::new();
    ctx.select("ghost");
    // label is added to selected_labels even if no object exists
    // (or may not be, depending on impl — here we check only nb_displayed unchanged)
    assert_eq!(ctx.nb_displayed(), 0);
}

#[test]
fn context_select_twice_no_duplicate_in_selected() {
    let mut ctx = AisInteractiveContext::new();
    ctx.display("box1", AisDisplayMode::Shaded);
    ctx.select("box1");
    ctx.select("box1");
    assert_eq!(ctx.nb_selected(), 1);
}

#[test]
fn context_deselect_all_clears_everything() {
    let mut ctx = AisInteractiveContext::new();
    ctx.display("box1", AisDisplayMode::Shaded);
    ctx.display("box2", AisDisplayMode::Shaded);
    ctx.select("box1");
    ctx.select("box2");
    assert_eq!(ctx.nb_selected(), 2);
    ctx.deselect_all();
    assert_eq!(ctx.nb_selected(), 0);
    assert!(!ctx.object(0).is_selected());
    assert!(!ctx.object(0).is_highlighted());
    assert!(!ctx.object(1).is_selected());
    assert!(!ctx.object(1).is_highlighted());
}

#[test]
fn context_set_and_get_highlight_color() {
    let mut ctx = AisInteractiveContext::new();
    ctx.set_highlight_color([0.0, 0.5, 1.0, 0.8]);
    assert_eq!(ctx.highlight_color(), [0.0f32, 0.5, 1.0, 0.8]);
}

#[test]
fn context_selected_slice_contains_labels() {
    let mut ctx = AisInteractiveContext::new();
    ctx.display("a", AisDisplayMode::Shaded);
    ctx.display("b", AisDisplayMode::Shaded);
    ctx.select("a");
    ctx.select("b");
    let sel = ctx.selected();
    assert!(sel.contains(&"a".to_string()));
    assert!(sel.contains(&"b".to_string()));
}

#[test]
fn context_default_impl() {
    let ctx = AisInteractiveContext::default();
    assert_eq!(ctx.nb_displayed(), 0);
}
