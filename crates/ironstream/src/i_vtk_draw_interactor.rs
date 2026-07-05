// FILE: i_vtk_draw_interactor.rs
// occt: IVtkDraw_Interactor

/// VTK interactor for handling user interaction in Draw module.
#[derive(Clone, Debug)]
pub struct IVtkDraw_Interactor {
    is_active: bool,
}

impl IVtkDraw_Interactor {
    /// Create a new interactor.
    pub fn new() -> Self {
        IVtkDraw_Interactor { is_active: false }
    }

    /// Activate the interactor.
    pub fn activate(&mut self) {
        self.is_active = true;
    }

    /// Deactivate the interactor.
    pub fn deactivate(&mut self) {
        self.is_active = false;
    }

    /// Check if the interactor is active.
    pub fn is_active(&self) -> bool {
        self.is_active
    }

    /// Process interaction event.
    pub fn process_event(&mut self) {
        if self.is_active {
            // Process event logic
        }
    }
}

impl Default for IVtkDraw_Interactor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_interactor() {
        let interactor = IVtkDraw_Interactor::new();
        assert!(!interactor.is_active());
    }

    #[test]
    fn test_activate_interactor() {
        let mut interactor = IVtkDraw_Interactor::new();
        interactor.activate();
        assert!(interactor.is_active());
    }

    #[test]
    fn test_deactivate_interactor() {
        let mut interactor = IVtkDraw_Interactor::new();
        interactor.activate();
        interactor.deactivate();
        assert!(!interactor.is_active());
    }
}
