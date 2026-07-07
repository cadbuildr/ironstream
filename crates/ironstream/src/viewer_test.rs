// FILE: viewer_test.rs
// occt: ViewerTest

use std::collections::HashMap;

/// Parameters for creating new view
#[derive(Clone, Debug)]
pub struct ViewerTestVinitParams {
    pub view_name: String,
    pub display_name: String,
    pub is_virtual: bool,
    pub is_composer: bool,
}

impl ViewerTestVinitParams {
    pub fn new() -> Self {
        ViewerTestVinitParams {
            view_name: String::new(),
            display_name: String::new(),
            is_virtual: false,
            is_composer: false,
        }
    }
}

impl Default for ViewerTestVinitParams {
    fn default() -> Self {
        Self::new()
    }
}

/// ViewerTest: Main viewer testing framework
/// Provides commands for managing 3D views and rendering tests.
pub struct ViewerTest {
    views: HashMap<String, ViewerTestView>,
}

/// A single view in the test framework
#[derive(Clone, Debug)]
pub struct ViewerTestView {
    name: String,
    is_virtual: bool,
}

impl ViewerTest {
    /// Create a new ViewerTest instance
    pub fn new() -> Self {
        ViewerTest {
            views: HashMap::new(),
        }
    }

    /// Initialize a view with parameters
    pub fn viewer_init(&mut self, params: &ViewerTestVinitParams) -> String {
        let view_name = if params.view_name.is_empty() {
            format!("view_{}", self.views.len())
        } else {
            params.view_name.clone()
        };

        let view = ViewerTestView {
            name: view_name.clone(),
            is_virtual: params.is_virtual,
        };

        self.views.insert(view_name.clone(), view);
        view_name
    }

    /// Get a view by name
    pub fn get_view(&self, name: &str) -> Option<&ViewerTestView> {
        self.views.get(name)
    }

    /// Get number of views
    pub fn view_count(&self) -> usize {
        self.views.len()
    }

    /// Clear all views
    pub fn clear_views(&mut self) {
        self.views.clear();
    }

    /// List all view names
    pub fn view_names(&self) -> Vec<String> {
        self.views.keys().cloned().collect()
    }
}

impl Default for ViewerTest {
    fn default() -> Self {
        Self::new()
    }
}

impl ViewerTestView {
    /// Get view name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Check if view is virtual
    pub fn is_virtual(&self) -> bool {
        self.is_virtual
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_viewer_test() {
        let vt = ViewerTest::new();
        assert_eq!(vt.view_count(), 0);
    }

    #[test]
    fn test_viewer_init_default() {
        let mut vt = ViewerTest::new();
        let params = ViewerTestVinitParams::new();
        let view_name = vt.viewer_init(&params);
        assert_eq!(vt.view_count(), 1);
        assert!(vt.get_view(&view_name).is_some());
    }

    #[test]
    fn test_viewer_init_custom_name() {
        let mut vt = ViewerTest::new();
        let mut params = ViewerTestVinitParams::new();
        params.view_name = "my_view".to_string();
        let view_name = vt.viewer_init(&params);
        assert_eq!(view_name, "my_view");
        assert!(vt.get_view("my_view").is_some());
    }

    #[test]
    fn test_viewer_init_virtual() {
        let mut vt = ViewerTest::new();
        let mut params = ViewerTestVinitParams::new();
        params.view_name = "virtual_view".to_string();
        params.is_virtual = true;
        vt.viewer_init(&params);
        let view = vt.get_view("virtual_view").unwrap();
        assert!(view.is_virtual());
    }

    #[test]
    fn test_multiple_views() {
        let mut vt = ViewerTest::new();
        for i in 0..3 {
            let mut params = ViewerTestVinitParams::new();
            params.view_name = format!("view_{}", i);
            vt.viewer_init(&params);
        }
        assert_eq!(vt.view_count(), 3);
    }

    #[test]
    fn test_view_names() {
        let mut vt = ViewerTest::new();
        let mut params = ViewerTestVinitParams::new();
        params.view_name = "test_view".to_string();
        vt.viewer_init(&params);
        let names = vt.view_names();
        assert!(names.contains(&"test_view".to_string()));
    }

    #[test]
    fn test_clear_views() {
        let mut vt = ViewerTest::new();
        let mut params = ViewerTestVinitParams::new();
        params.view_name = "view1".to_string();
        vt.viewer_init(&params);
        assert_eq!(vt.view_count(), 1);
        vt.clear_views();
        assert_eq!(vt.view_count(), 0);
    }

    #[test]
    fn test_viewer_test_params_default() {
        let params = ViewerTestVinitParams::default();
        assert!(params.view_name.is_empty());
        assert!(!params.is_virtual);
    }

    #[test]
    fn test_view_test_view() {
        let view = ViewerTestView {
            name: "test".to_string(),
            is_virtual: true,
        };
        assert_eq!(view.name(), "test");
        assert!(view.is_virtual());
    }

    #[test]
    fn test_get_nonexistent_view() {
        let vt = ViewerTest::new();
        assert!(vt.get_view("nonexistent").is_none());
    }
}
