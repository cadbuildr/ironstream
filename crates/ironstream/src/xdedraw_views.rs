// FILE: xdedraw_views.rs
// occt: XDEDRAW_Views

//! DRAW commands for view management in XDE.
//! Original: Draw/TKXDEDRAW/XDEDRAW/XDEDRAW_Views.hxx
//!
//! Provides commands to create and manage presentation views of XDE documents.

use std::collections::HashMap;

/// View command handler for XDE documents.
#[derive(Clone, Debug)]
pub struct XDEDRAWViews {
    views: HashMap<String, ViewState>, // View name -> View state
    view_commands: Vec<String>,
}

/// Internal state of a presentation view.
#[derive(Clone, Debug)]
struct ViewState {
    camera_x: f64,
    camera_y: f64,
    camera_z: f64,
    zoom_level: f64,
}

impl ViewState {
    fn new() -> Self {
        Self {
            camera_x: 0.0,
            camera_y: 0.0,
            camera_z: 10.0,
            zoom_level: 1.0,
        }
    }
}

impl XDEDRAWViews {
    /// Creates a new view command handler.
    pub fn new() -> Self {
        Self {
            views: HashMap::new(),
            view_commands: Vec::new(),
        }
    }

    /// Registers a view command.
    pub fn register_view_command(&mut self, cmd_name: String) {
        self.view_commands.push(cmd_name);
    }

    /// Creates a new view. Returns true if created successfully.
    pub fn create_view(&mut self, view_name: String) -> bool {
        self.views.insert(view_name, ViewState::new()).is_none()
    }

    /// Sets the camera position for a view. Returns true if successful.
    pub fn set_camera(&mut self, view_name: &str, x: f64, y: f64, z: f64) -> bool {
        if let Some(view) = self.views.get_mut(view_name) {
            view.camera_x = x;
            view.camera_y = y;
            view.camera_z = z;
            true
        } else {
            false
        }
    }

    /// Gets the camera position for a view.
    pub fn get_camera(&self, view_name: &str) -> Option<(f64, f64, f64)> {
        self.views.get(view_name).map(|v| {
            (v.camera_x, v.camera_y, v.camera_z)
        })
    }

    /// Sets the zoom level for a view. Returns true if successful.
    pub fn set_zoom(&mut self, view_name: &str, zoom: f64) -> bool {
        if let Some(view) = self.views.get_mut(view_name) {
            view.zoom_level = zoom;
            true
        } else {
            false
        }
    }

    /// Gets the zoom level for a view.
    pub fn get_zoom(&self, view_name: &str) -> Option<f64> {
        self.views.get(view_name).map(|v| v.zoom_level)
    }

    /// Removes a view. Returns true if it existed.
    pub fn remove_view(&mut self, view_name: &str) -> bool {
        self.views.remove(view_name).is_some()
    }

    /// Returns the list of registered view commands.
    pub fn view_commands(&self) -> &[String] {
        &self.view_commands
    }

    /// Returns the number of views.
    pub fn view_count(&self) -> usize {
        self.views.len()
    }

    /// Clears all views and commands.
    pub fn clear(&mut self) {
        self.views.clear();
        self.view_commands.clear();
    }

    /// Initializes standard view commands.
    pub fn init_standard_view_commands(&mut self) {
        self.view_commands.push("xde_create_view".to_string());
        self.view_commands.push("xde_set_camera".to_string());
        self.view_commands.push("xde_set_zoom".to_string());
        self.view_commands.push("xde_fit_all".to_string());
    }
}

impl Default for XDEDRAWViews {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_view_handler() {
        let handler = XDEDRAWViews::new();
        assert_eq!(handler.view_count(), 0);
    }

    #[test]
    fn test_create_view() {
        let mut handler = XDEDRAWViews::new();
        assert!(handler.create_view("view1".to_string()));
        assert_eq!(handler.view_count(), 1);
    }

    #[test]
    fn test_set_and_get_camera() {
        let mut handler = XDEDRAWViews::new();
        handler.create_view("main".to_string());
        assert!(handler.set_camera("main", 1.0, 2.0, 3.0));
        assert_eq!(handler.get_camera("main"), Some((1.0, 2.0, 3.0)));
    }

    #[test]
    fn test_set_and_get_zoom() {
        let mut handler = XDEDRAWViews::new();
        handler.create_view("main".to_string());
        assert!(handler.set_zoom("main", 2.5));
        assert_eq!(handler.get_zoom("main"), Some(2.5));
    }

    #[test]
    fn test_default_camera() {
        let mut handler = XDEDRAWViews::new();
        handler.create_view("default".to_string());
        assert_eq!(handler.get_camera("default"), Some((0.0, 0.0, 10.0)));
    }

    #[test]
    fn test_default_zoom() {
        let mut handler = XDEDRAWViews::new();
        handler.create_view("default".to_string());
        assert_eq!(handler.get_zoom("default"), Some(1.0));
    }

    #[test]
    fn test_remove_view() {
        let mut handler = XDEDRAWViews::new();
        handler.create_view("v1".to_string());
        assert!(handler.remove_view("v1"));
        assert_eq!(handler.view_count(), 0);
    }

    #[test]
    fn test_register_commands() {
        let mut handler = XDEDRAWViews::new();
        handler.register_view_command("cmd1".to_string());
        assert_eq!(handler.view_commands().len(), 1);
    }

    #[test]
    fn test_init_standard_view_commands() {
        let mut handler = XDEDRAWViews::new();
        handler.init_standard_view_commands();
        assert_eq!(handler.view_commands().len(), 4);
    }

    #[test]
    fn test_clear() {
        let mut handler = XDEDRAWViews::new();
        handler.create_view("v".to_string());
        handler.register_view_command("cmd".to_string());
        handler.clear();
        assert_eq!(handler.view_count(), 0);
        assert_eq!(handler.view_commands().len(), 0);
    }
}
