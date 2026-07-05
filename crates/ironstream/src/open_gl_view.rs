// FILE: open_gl_view.rs
// occt: OpenGl_View

//! Implementation of OpenGL view.
//! Manages rendering window, resources, and view parameters.

use std::collections::HashMap;

/// Rendering context placeholder
#[derive(Debug, Clone)]
pub struct RenderingContext;

/// Window reference placeholder
#[derive(Debug, Clone)]
pub struct AspectWindow {
    id: u32,
}

impl AspectWindow {
    pub fn new(id: u32) -> Self {
        Self { id }
    }

    pub fn id(&self) -> u32 {
        self.id
    }
}

/// Structure placeholder
#[derive(Debug, Clone)]
pub struct Structure {
    id: u32,
    visible: bool,
}

impl Structure {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            visible: true,
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }
}

/// OpenGL View
#[derive(Debug)]
pub struct OpenGlView {
    window: Option<AspectWindow>,
    structures: HashMap<u32, Structure>,
    immediate_mode_to_front: bool,
    is_defined: bool,
    viewport_width: i32,
    viewport_height: i32,
    redraw_count: u32,
}

impl OpenGlView {
    /// Constructor
    pub fn new() -> Self {
        Self {
            window: None,
            structures: HashMap::new(),
            immediate_mode_to_front: true,
            is_defined: false,
            viewport_width: 512,
            viewport_height: 512,
            redraw_count: 0,
        }
    }

    /// Release OpenGL resources
    pub fn release_gl_resources(&mut self) {
        self.structures.clear();
    }

    /// Deletes and erases the view
    pub fn remove(&mut self) {
        self.window = None;
        self.structures.clear();
        self.is_defined = false;
    }

    /// Set immediate mode draw to front mode
    pub fn set_immediate_mode_draw_to_front(&mut self, to_front: bool) -> bool {
        let prev = self.immediate_mode_to_front;
        self.immediate_mode_to_front = to_front;
        prev
    }

    /// Creates and maps rendering window to the view
    pub fn set_window(&mut self, window: AspectWindow, _context: &RenderingContext) -> bool {
        self.window = Some(window);
        self.is_defined = true;
        true
    }

    /// Returns window associated with the view
    pub fn window(&self) -> Option<&AspectWindow> {
        self.window.as_ref()
    }

    /// Returns True if the window associated to the view is defined
    pub fn is_defined(&self) -> bool {
        self.is_defined
    }

    /// Handle changing size of the rendering window
    pub fn resized(&mut self, width: i32, height: i32) {
        self.viewport_width = width.max(1);
        self.viewport_height = height.max(1);
    }

    /// Redraw content of the view
    pub fn redraw(&mut self) {
        self.redraw_count += 1;
    }

    /// Redraw immediate content of the view
    pub fn redraw_immediate(&mut self) {
        self.redraw_count += 1;
    }

    /// Add structure to view
    pub fn add_structure(&mut self, structure: Structure) {
        self.structures.insert(structure.id(), structure);
    }

    /// Remove structure from view
    pub fn remove_structure(&mut self, id: u32) {
        self.structures.remove(&id);
    }

    /// Get structure by ID
    pub fn get_structure(&self, id: u32) -> Option<&Structure> {
        self.structures.get(&id)
    }

    /// Get number of structures in view
    pub fn structure_count(&self) -> usize {
        self.structures.len()
    }

    /// Get viewport dimensions
    pub fn viewport(&self) -> (i32, i32) {
        (self.viewport_width, self.viewport_height)
    }

    /// Check if immediate mode to front is enabled
    pub fn is_immediate_mode_to_front(&self) -> bool {
        self.immediate_mode_to_front
    }

    /// Get redraw count
    pub fn redraw_count(&self) -> u32 {
        self.redraw_count
    }

    /// Reset redraw count
    pub fn reset_redraw_count(&mut self) {
        self.redraw_count = 0;
    }

    /// Enable or disable view
    pub fn set_enabled(&mut self, enabled: bool) {
        if enabled {
            self.is_defined = true;
        }
    }

    /// Get list of visible structures
    pub fn visible_structures(&self) -> Vec<&Structure> {
        self.structures
            .values()
            .filter(|s| s.is_visible())
            .collect()
    }
}

impl Default for OpenGlView {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let view = OpenGlView::new();
        assert!(view.is_defined() == false);
        assert_eq!(view.structure_count(), 0);
    }

    #[test]
    fn test_set_window() {
        let mut view = OpenGlView::new();
        let window = AspectWindow::new(1);
        let ctx = RenderingContext;

        assert!(view.set_window(window, &ctx));
        assert!(view.is_defined());
        assert!(view.window().is_some());
    }

    #[test]
    fn test_immediate_mode() {
        let mut view = OpenGlView::new();
        assert!(view.is_immediate_mode_to_front());

        let prev = view.set_immediate_mode_draw_to_front(false);
        assert!(prev);
        assert!(!view.is_immediate_mode_to_front());
    }

    #[test]
    fn test_resize() {
        let mut view = OpenGlView::new();
        view.resized(1024, 768);

        let (w, h) = view.viewport();
        assert_eq!(w, 1024);
        assert_eq!(h, 768);
    }

    #[test]
    fn test_resize_invalid() {
        let mut view = OpenGlView::new();
        view.resized(0, -1);

        let (w, h) = view.viewport();
        assert_eq!(w, 1);
        assert_eq!(h, 1);
    }

    #[test]
    fn test_redraw() {
        let mut view = OpenGlView::new();
        assert_eq!(view.redraw_count(), 0);

        view.redraw();
        assert_eq!(view.redraw_count(), 1);

        view.redraw_immediate();
        assert_eq!(view.redraw_count(), 2);
    }

    #[test]
    fn test_add_remove_structure() {
        let mut view = OpenGlView::new();
        let structure = Structure::new(1);

        view.add_structure(structure);
        assert_eq!(view.structure_count(), 1);
        assert!(view.get_structure(1).is_some());

        view.remove_structure(1);
        assert_eq!(view.structure_count(), 0);
    }

    #[test]
    fn test_visible_structures() {
        let mut view = OpenGlView::new();
        view.add_structure(Structure::new(1));
        view.add_structure(Structure::new(2));

        let visible = view.visible_structures();
        assert_eq!(visible.len(), 2);
    }

    #[test]
    fn test_remove() {
        let mut view = OpenGlView::new();
        let window = AspectWindow::new(1);
        let ctx = RenderingContext;

        view.set_window(window, &ctx);
        view.add_structure(Structure::new(1));
        assert!(view.is_defined());

        view.remove();
        assert!(!view.is_defined());
        assert_eq!(view.structure_count(), 0);
    }

    #[test]
    fn test_default() {
        let view = OpenGlView::default();
        assert!(!view.is_defined());
        assert_eq!(view.viewport(), (512, 512));
    }
}
