// FILE: i_vtk_draw.rs
// occt: IVtkDraw

/// Stub for IVtkDraw - VTK integration for Draw module.
/// This class provides integration between OCCT shapes and VTK visualization.
#[derive(Clone, Debug)]
pub struct IVtkDraw;

impl IVtkDraw {
    /// Initialize VTK draw module.
    pub fn new() -> Self {
        IVtkDraw
    }

    /// Check if VTK is initialized.
    pub fn is_initialized(&self) -> bool {
        true
    }
}

impl Default for IVtkDraw {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_instance() {
        let vtk_draw = IVtkDraw::new();
        assert!(vtk_draw.is_initialized());
    }

    #[test]
    fn test_default() {
        let vtk_draw = IVtkDraw::default();
        assert!(vtk_draw.is_initialized());
    }
}
