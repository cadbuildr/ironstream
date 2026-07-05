// FILE: i_vtk_i_view.rs
// occt: IVtk_IView

/// Interface for VTK view.
pub trait IVtk_IView {
    /// Get view ID.
    fn view_id(&self) -> u32;

    /// Initialize the view.
    fn initialize(&mut self) -> bool;

    /// Check if view is initialized.
    fn is_initialized(&self) -> bool;

    /// Render the view.
    fn render(&self);

    /// Fit all objects in the view.
    fn fit_all(&self);
}

/// Default implementation of IVtk_IView.
#[derive(Clone, Debug)]
pub struct DefaultView {
    id: u32,
    initialized: bool,
}

impl DefaultView {
    /// Create a new view.
    pub fn new(id: u32) -> Self {
        DefaultView {
            id,
            initialized: false,
        }
    }
}

impl IVtk_IView for DefaultView {
    fn view_id(&self) -> u32 {
        self.id
    }

    fn initialize(&mut self) -> bool {
        self.initialized = true;
        true
    }

    fn is_initialized(&self) -> bool {
        self.initialized
    }

    fn render(&self) {
        if self.initialized {
            // Render logic
        }
    }

    fn fit_all(&self) {
        if self.initialized {
            // Fit all logic
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_view() {
        let view = DefaultView::new(1);
        assert_eq!(view.view_id(), 1);
        assert!(!view.is_initialized());
    }

    #[test]
    fn test_initialize() {
        let mut view = DefaultView::new(2);
        let success = view.initialize();
        assert!(success);
        assert!(view.is_initialized());
    }

    #[test]
    fn test_view_trait() {
        let mut view: Box<dyn IVtk_IView> = Box::new(DefaultView::new(3));
        view.initialize();
        assert!(view.is_initialized());
        assert_eq!(view.view_id(), 3);
    }
}
