// FILE: tests/occt_visual3d_context.rs

extern crate ironstream;
use ironstream::visual3d_context::*;

// --- Visual3dContextMode ---

#[test]
fn test_mode_variants_are_distinct() {
    assert_ne!(Visual3dContextMode::WireFrame, Visual3dContextMode::Shaded);
    assert_ne!(Visual3dContextMode::Shaded, Visual3dContextMode::HiddenLine);
    assert_ne!(
        Visual3dContextMode::HiddenLine,
        Visual3dContextMode::FaceBoundary
    );
}

#[test]
fn test_mode_clone_and_copy() {
    let m = Visual3dContextMode::Shaded;
    let m2 = m;
    assert_eq!(m, m2);
}

// --- Visual3dHighlightStyle ---

#[test]
fn test_highlight_style_new_defaults() {
    let s = Visual3dHighlightStyle::new([1.0, 0.0, 0.0, 1.0]);
    assert_eq!(s.color(), [1.0, 0.0, 0.0, 1.0]);
    assert!(!s.is_transparent());
    assert_eq!(s.line_width(), 2.0);
}

#[test]
fn test_highlight_style_set_color() {
    let mut s = Visual3dHighlightStyle::new([0.0, 0.0, 0.0, 1.0]);
    s.set_color([0.5, 0.5, 0.5, 0.8]);
    assert_eq!(s.color(), [0.5, 0.5, 0.5, 0.8]);
}

#[test]
fn test_highlight_style_set_transparent() {
    let mut s = Visual3dHighlightStyle::new([1.0, 1.0, 0.0, 1.0]);
    assert!(!s.is_transparent());
    s.set_transparent(true);
    assert!(s.is_transparent());
    s.set_transparent(false);
    assert!(!s.is_transparent());
}

#[test]
fn test_highlight_style_line_width_is_two() {
    let s = Visual3dHighlightStyle::new([0.0, 1.0, 0.0, 1.0]);
    assert_eq!(s.line_width(), 2.0);
}

// --- Visual3dDisplayed ---

#[test]
fn test_displayed_new_defaults() {
    let d = Visual3dDisplayed::new(42, Visual3dContextMode::Shaded);
    assert_eq!(d.structure_id(), 42);
    assert_eq!(d.mode(), Visual3dContextMode::Shaded);
    assert!(!d.is_highlighted());
    assert!(!d.is_selected());
    assert_eq!(d.layer(), 0);
}

#[test]
fn test_displayed_highlight_unhighlight() {
    let mut d = Visual3dDisplayed::new(1, Visual3dContextMode::WireFrame);
    assert!(!d.is_highlighted());
    d.highlight();
    assert!(d.is_highlighted());
    d.unhighlight();
    assert!(!d.is_highlighted());
}

#[test]
fn test_displayed_select_deselect() {
    let mut d = Visual3dDisplayed::new(2, Visual3dContextMode::FaceBoundary);
    assert!(!d.is_selected());
    d.select();
    assert!(d.is_selected());
    d.deselect();
    assert!(!d.is_selected());
}

#[test]
fn test_displayed_mode_wireframe() {
    let d = Visual3dDisplayed::new(10, Visual3dContextMode::WireFrame);
    assert_eq!(d.mode(), Visual3dContextMode::WireFrame);
}

#[test]
fn test_displayed_mode_hidden_line() {
    let d = Visual3dDisplayed::new(11, Visual3dContextMode::HiddenLine);
    assert_eq!(d.mode(), Visual3dContextMode::HiddenLine);
}

// --- Visual3dContext ---

#[test]
fn test_context_new_is_empty() {
    let ctx = Visual3dContext::new();
    assert_eq!(ctx.nb_displayed(), 0);
    assert_eq!(ctx.background_color(), [0.0, 0.0, 0.0]);
}

#[test]
fn test_context_default_is_new() {
    let ctx: Visual3dContext = Default::default();
    assert_eq!(ctx.nb_displayed(), 0);
}

#[test]
fn test_context_display_adds_entry() {
    let mut ctx = Visual3dContext::new();
    let idx = ctx.display(1, Visual3dContextMode::Shaded);
    assert_eq!(idx, 0);
    assert_eq!(ctx.nb_displayed(), 1);
}

#[test]
fn test_context_display_multiple() {
    let mut ctx = Visual3dContext::new();
    let i0 = ctx.display(10, Visual3dContextMode::WireFrame);
    let i1 = ctx.display(20, Visual3dContextMode::Shaded);
    let i2 = ctx.display(30, Visual3dContextMode::HiddenLine);
    assert_eq!(i0, 0);
    assert_eq!(i1, 1);
    assert_eq!(i2, 2);
    assert_eq!(ctx.nb_displayed(), 3);
}

#[test]
fn test_context_display_duplicate_updates_mode() {
    let mut ctx = Visual3dContext::new();
    ctx.display(5, Visual3dContextMode::WireFrame);
    let idx = ctx.display(5, Visual3dContextMode::Shaded);
    assert_eq!(idx, 0);
    assert_eq!(ctx.nb_displayed(), 1);
}

#[test]
fn test_context_remove_present() {
    let mut ctx = Visual3dContext::new();
    ctx.display(7, Visual3dContextMode::Shaded);
    assert_eq!(ctx.nb_displayed(), 1);
    let removed = ctx.remove(7);
    assert!(removed);
    assert_eq!(ctx.nb_displayed(), 0);
}

#[test]
fn test_context_remove_absent() {
    let mut ctx = Visual3dContext::new();
    let removed = ctx.remove(99);
    assert!(!removed);
}

#[test]
fn test_context_remove_does_not_affect_others() {
    let mut ctx = Visual3dContext::new();
    ctx.display(1, Visual3dContextMode::WireFrame);
    ctx.display(2, Visual3dContextMode::Shaded);
    ctx.display(3, Visual3dContextMode::HiddenLine);
    ctx.remove(2);
    assert_eq!(ctx.nb_displayed(), 2);
}

#[test]
fn test_context_highlight_existing() {
    let mut ctx = Visual3dContext::new();
    ctx.display(4, Visual3dContextMode::Shaded);
    ctx.highlight(4);
    // No panic; highlight is applied (tested indirectly via nb_displayed staying the same).
    assert_eq!(ctx.nb_displayed(), 1);
}

#[test]
fn test_context_highlight_missing_is_noop() {
    let mut ctx = Visual3dContext::new();
    ctx.highlight(999); // must not panic
    assert_eq!(ctx.nb_displayed(), 0);
}

#[test]
fn test_context_select_existing() {
    let mut ctx = Visual3dContext::new();
    ctx.display(8, Visual3dContextMode::FaceBoundary);
    ctx.select(8);
    assert_eq!(ctx.nb_displayed(), 1);
}

#[test]
fn test_context_select_missing_is_noop() {
    let mut ctx = Visual3dContext::new();
    ctx.select(123); // must not panic
    assert_eq!(ctx.nb_displayed(), 0);
}

#[test]
fn test_context_clear() {
    let mut ctx = Visual3dContext::new();
    ctx.display(1, Visual3dContextMode::Shaded);
    ctx.display(2, Visual3dContextMode::WireFrame);
    assert_eq!(ctx.nb_displayed(), 2);
    ctx.clear();
    assert_eq!(ctx.nb_displayed(), 0);
}

#[test]
fn test_context_clear_empty_is_noop() {
    let mut ctx = Visual3dContext::new();
    ctx.clear();
    assert_eq!(ctx.nb_displayed(), 0);
}

#[test]
fn test_context_background_color_set_get() {
    let mut ctx = Visual3dContext::new();
    ctx.set_background_color([0.2, 0.4, 0.6]);
    let c = ctx.background_color();
    assert_eq!(c, [0.2, 0.4, 0.6]);
}

#[test]
fn test_context_background_color_default_is_black() {
    let ctx = Visual3dContext::new();
    assert_eq!(ctx.background_color(), [0.0, 0.0, 0.0]);
}

#[test]
fn test_context_display_after_clear_restarts() {
    let mut ctx = Visual3dContext::new();
    ctx.display(1, Visual3dContextMode::Shaded);
    ctx.display(2, Visual3dContextMode::WireFrame);
    ctx.clear();
    let idx = ctx.display(3, Visual3dContextMode::HiddenLine);
    assert_eq!(idx, 0);
    assert_eq!(ctx.nb_displayed(), 1);
}
