// FILE: select_mgr_base_intersector.rs
// occt: SelectMgr_BaseIntersector

/// Enumeration for selection types supported by the intersector.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SelectMgrSelectionType {
    /// Point selection
    Point,
    /// Box selection
    Box,
    /// Polyline selection
    Polyline,
}

/// Base trait for all intersector implementations.
/// Defines the interface for different types of selecting intersectors
/// (point, box, polyline selection).
pub struct SelectMgrBaseIntersector {
    selection_type: SelectMgrSelectionType,
}

impl SelectMgrBaseIntersector {
    /// Creates a new empty selecting volume.
    pub fn new() -> Self {
        SelectMgrBaseIntersector {
            selection_type: SelectMgrSelectionType::Point,
        }
    }

    /// Returns selection type of this intersector.
    pub fn selection_type(&self) -> SelectMgrSelectionType {
        self.selection_type
    }

    /// Sets the selection type.
    pub fn set_selection_type(&mut self, sel_type: SelectMgrSelectionType) {
        self.selection_type = sel_type;
    }

    /// Sets pixel tolerance (makes sense only for scalable intersectors).
    /// This method does nothing for the base class.
    pub fn set_pixel_tolerance(&mut self, _tol: i32) {
        // Base implementation does nothing
    }

    /// Sets current window size.
    /// This method does nothing for the base class.
    pub fn set_window_size(&mut self, _width: i32, _height: i32) {
        // Base implementation does nothing
    }

    /// Gets current window size.
    /// This method doesn't set any output values for the base class.
    pub fn window_size(&self) -> (i32, i32) {
        (0, 0)
    }

    /// Sets viewport parameters.
    /// This method does nothing for the base class.
    pub fn set_viewport(&mut self, _x: f64, _y: f64, _width: f64, _height: f64) {
        // Base implementation does nothing
    }
}

impl Default for SelectMgrBaseIntersector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base_intersector_creation() {
        let intersector = SelectMgrBaseIntersector::new();
        assert_eq!(
            intersector.selection_type(),
            SelectMgrSelectionType::Point
        );
    }

    #[test]
    fn test_set_selection_type() {
        let mut intersector = SelectMgrBaseIntersector::new();
        assert_eq!(
            intersector.selection_type(),
            SelectMgrSelectionType::Point
        );

        intersector.set_selection_type(SelectMgrSelectionType::Box);
        assert_eq!(intersector.selection_type(), SelectMgrSelectionType::Box);

        intersector.set_selection_type(SelectMgrSelectionType::Polyline);
        assert_eq!(
            intersector.selection_type(),
            SelectMgrSelectionType::Polyline
        );
    }

    #[test]
    fn test_set_pixel_tolerance() {
        let mut intersector = SelectMgrBaseIntersector::new();
        // Base class does nothing, but the call should not panic
        intersector.set_pixel_tolerance(2);
    }

    #[test]
    fn test_set_window_size() {
        let mut intersector = SelectMgrBaseIntersector::new();
        // Base class does nothing, but the call should not panic
        intersector.set_window_size(1024, 768);
        let (w, h) = intersector.window_size();
        assert_eq!(w, 0);
        assert_eq!(h, 0);
    }

    #[test]
    fn test_set_viewport() {
        let mut intersector = SelectMgrBaseIntersector::new();
        // Base class does nothing, but the call should not panic
        intersector.set_viewport(0.0, 0.0, 1024.0, 768.0);
    }

    #[test]
    fn test_default_intersector() {
        let intersector = SelectMgrBaseIntersector::default();
        assert_eq!(
            intersector.selection_type(),
            SelectMgrSelectionType::Point
        );
    }
}
