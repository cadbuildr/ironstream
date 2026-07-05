// FILE: i_vtk_tools_display_mode_filter.rs
// occt: IVtkTools_DisplayModeFilter

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DisplayMode {
    Wireframe,
    Shaded,
    Points,
}

/// Filter for controlling display mode of VTK objects.
#[derive(Clone, Debug)]
pub struct IVtkTools_DisplayModeFilter {
    mode: DisplayMode,
}

impl IVtkTools_DisplayModeFilter {
    /// Create a new display mode filter.
    pub fn new() -> Self {
        IVtkTools_DisplayModeFilter {
            mode: DisplayMode::Shaded,
        }
    }

    /// Set the display mode.
    pub fn set_mode(&mut self, mode: DisplayMode) {
        self.mode = mode;
    }

    /// Get the current display mode.
    pub fn mode(&self) -> DisplayMode {
        self.mode
    }

    /// Check if wireframe mode is active.
    pub fn is_wireframe(&self) -> bool {
        self.mode == DisplayMode::Wireframe
    }

    /// Check if shaded mode is active.
    pub fn is_shaded(&self) -> bool {
        self.mode == DisplayMode::Shaded
    }

    /// Check if points mode is active.
    pub fn is_points(&self) -> bool {
        self.mode == DisplayMode::Points
    }
}

impl Default for IVtkTools_DisplayModeFilter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_filter() {
        let filter = IVtkTools_DisplayModeFilter::new();
        assert!(filter.is_shaded());
    }

    #[test]
    fn test_set_wireframe() {
        let mut filter = IVtkTools_DisplayModeFilter::new();
        filter.set_mode(DisplayMode::Wireframe);
        assert!(filter.is_wireframe());
        assert!(!filter.is_shaded());
    }

    #[test]
    fn test_set_points() {
        let mut filter = IVtkTools_DisplayModeFilter::new();
        filter.set_mode(DisplayMode::Points);
        assert!(filter.is_points());
    }

    #[test]
    fn test_mode_transitions() {
        let mut filter = IVtkTools_DisplayModeFilter::new();
        assert_eq!(filter.mode(), DisplayMode::Shaded);
        filter.set_mode(DisplayMode::Wireframe);
        assert_eq!(filter.mode(), DisplayMode::Wireframe);
        filter.set_mode(DisplayMode::Points);
        assert_eq!(filter.mode(), DisplayMode::Points);
    }
}
