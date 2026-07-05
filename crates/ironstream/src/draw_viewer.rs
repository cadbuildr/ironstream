// FILE: draw_viewer.rs
// occt: Draw_Viewer

//! Draw viewer for managing multiple views.

use std::collections::HashMap;

/// Manages multiple Draw views
pub struct DrawViewer {
    views: HashMap<i32, String>,
    current_view: Option<i32>,
}

impl DrawViewer {
    /// Create a new viewer
    pub fn new() -> Self {
        DrawViewer {
            views: HashMap::new(),
            current_view: None,
        }
    }

    /// Add a view
    pub fn add_view(&mut self, id: i32, name: impl Into<String>) {
        self.views.insert(id, name.into());
        if self.current_view.is_none() {
            self.current_view = Some(id);
        }
    }

    /// Get the current view ID
    pub fn current_view(&self) -> Option<i32> {
        self.current_view
    }

    /// Set the current view
    pub fn set_current_view(&mut self, id: i32) {
        if self.views.contains_key(&id) {
            self.current_view = Some(id);
        }
    }

    /// Get the number of views
    pub fn num_views(&self) -> usize {
        self.views.len()
    }
}

impl Default for DrawViewer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_viewer_creation() {
        let viewer = DrawViewer::new();
        assert_eq!(viewer.num_views(), 0);
        assert!(viewer.current_view().is_none());
    }

    #[test]
    fn test_add_view() {
        let mut viewer = DrawViewer::new();
        viewer.add_view(1, "View1");
        assert_eq!(viewer.num_views(), 1);
        assert_eq!(viewer.current_view(), Some(1));
    }
}
