// FILE: rust/ironstream/crates/ironstream/src/xcaf_view.rs

// occt: XCAFDoc_View // — stores a named view with camera parameters
#[derive(Clone, Debug)]
pub struct XcafView {
    pub name: String,
    pub camera_position: [f64; 3],
    pub camera_target: [f64; 3],
    pub is_perspective: bool,
}

impl XcafView {
    /// Create a new view with the given name and default camera values.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            camera_position: [0.0, 0.0, 1.0],
            camera_target: [0.0, 0.0, 0.0],
            is_perspective: false,
        }
    }

    /// Set the camera position and target.
    pub fn set_camera(&mut self, pos: [f64; 3], tgt: [f64; 3]) {
        self.camera_position = pos;
        self.camera_target = tgt;
    }

    /// Set whether the view uses perspective projection.
    pub fn set_perspective(&mut self, b: bool) {
        self.is_perspective = b;
    }

    /// Return a deep copy of this view.
    pub fn clone_view(&self) -> XcafView {
        self.clone()
    }
}

// occt: XCAFDoc_ViewTool // — manages a collection of named views
pub struct XcafViewTool {
    pub views: Vec<XcafView>,
}

impl XcafViewTool {
    /// Create an empty view tool.
    pub fn new() -> Self {
        Self { views: Vec::new() }
    }

    /// Append a view to the tool.
    pub fn add_view(&mut self, v: XcafView) {
        self.views.push(v);
    }

    /// Find a view by name, returning a reference if found.
    pub fn find_view(&self, name: &str) -> Option<&XcafView> {
        self.views.iter().find(|v| v.name == name)
    }

    /// Return the number of views stored in this tool.
    pub fn view_count(&self) -> usize {
        self.views.len()
    }
}

/// Projection type for a view.
// occt: XCAFDoc_View // (projection field)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProjectionType {
    Parallel,
    Perspective,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xcaf_view_new_defaults() {
        let v = XcafView::new("front");
        assert_eq!(v.name, "front");
        assert_eq!(v.camera_position, [0.0, 0.0, 1.0]);
        assert_eq!(v.camera_target, [0.0, 0.0, 0.0]);
        assert!(!v.is_perspective);
    }

    #[test]
    fn test_xcaf_view_set_camera() {
        let mut v = XcafView::new("iso");
        v.set_camera([1.0, 2.0, 3.0], [4.0, 5.0, 6.0]);
        assert_eq!(v.camera_position, [1.0, 2.0, 3.0]);
        assert_eq!(v.camera_target, [4.0, 5.0, 6.0]);
    }

    #[test]
    fn test_xcaf_view_set_perspective() {
        let mut v = XcafView::new("persp");
        v.set_perspective(true);
        assert!(v.is_perspective);
        v.set_perspective(false);
        assert!(!v.is_perspective);
    }

    #[test]
    fn test_xcaf_view_clone_view() {
        let mut v = XcafView::new("orig");
        v.set_camera([1.0, 0.0, 0.0], [0.0, 0.0, 0.0]);
        v.set_perspective(true);
        let c = v.clone_view();
        assert_eq!(c.name, v.name);
        assert_eq!(c.camera_position, v.camera_position);
        assert_eq!(c.camera_target, v.camera_target);
        assert_eq!(c.is_perspective, v.is_perspective);
    }

    #[test]
    fn test_xcaf_view_tool_add_and_count() {
        let mut tool = XcafViewTool::new();
        assert_eq!(tool.view_count(), 0);
        tool.add_view(XcafView::new("top"));
        tool.add_view(XcafView::new("side"));
        assert_eq!(tool.view_count(), 2);
    }

    #[test]
    fn test_xcaf_view_tool_find_view() {
        let mut tool = XcafViewTool::new();
        tool.add_view(XcafView::new("top"));
        tool.add_view(XcafView::new("front"));
        assert!(tool.find_view("top").is_some());
        assert_eq!(tool.find_view("top").unwrap().name, "top");
        assert!(tool.find_view("missing").is_none());
    }

    #[test]
    fn test_projection_type_variants() {
        let p = ProjectionType::Parallel;
        let q = ProjectionType::Perspective;
        assert_ne!(p, q);
        assert_eq!(p, ProjectionType::Parallel);
        assert_eq!(q, ProjectionType::Perspective);
    }
}
