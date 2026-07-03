// FILE: std_select_viewer_selector3d.rs
// occt: StdSelect_ViewerSelector3d

//! 3D viewer selector for interactive object selection.

#[derive(Clone, Debug)]
pub struct ViewerSelector3d {
    pub selector_id: u32,
    pub is_active: bool,
    pub selection_tolerance: f64,
}

impl ViewerSelector3d {
    pub fn new(selector_id: u32) -> Self {
        ViewerSelector3d {
            selector_id,
            is_active: true,
            selection_tolerance: 2.0,
        }
    }

    pub fn with_tolerance(mut self, tol: f64) -> Self {
        self.selection_tolerance = tol.max(0.1);
        self
    }

    pub fn set_tolerance(&mut self, tol: f64) {
        self.selection_tolerance = tol.max(0.1);
    }

    pub fn activate(&mut self) {
        self.is_active = true;
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn viewer_selector3d_creation() {
        let sel = ViewerSelector3d::new(1);
        assert_eq!(sel.selector_id, 1);
        assert!(sel.is_active);
        assert_eq!(sel.selection_tolerance, 2.0);
    }

    #[test]
    fn viewer_selector3d_builder() {
        let sel = ViewerSelector3d::new(1)
            .with_tolerance(5.0);

        assert_eq!(sel.selection_tolerance, 5.0);
    }

    #[test]
    fn viewer_selector3d_activity() {
        let mut sel = ViewerSelector3d::new(1);
        sel.deactivate();
        assert!(!sel.is_active());
        sel.activate();
        assert!(sel.is_active());
    }
}
