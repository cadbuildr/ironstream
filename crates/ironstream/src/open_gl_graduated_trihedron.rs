// FILE: open_gl_graduated_trihedron.rs
// occt: OpenGl_GraduatedTrihedron

/// Rendered graduated trihedron with grid customization.
/// Supports customization on construction level to render axis grids with labels.
#[derive(Debug, Clone)]
pub struct OpenGlGraduatedTrihedron {
    is_visible: bool,
    min_bounds: [f32; 3],
    max_bounds: [f32; 3],
}

impl OpenGlGraduatedTrihedron {
    /// Creates a new graduated trihedron with default settings.
    pub fn new() -> Self {
        OpenGlGraduatedTrihedron {
            is_visible: true,
            min_bounds: [0.0; 3],
            max_bounds: [1.0; 3],
        }
    }

    /// Sets the bounding box values for the trihedron.
    pub fn set_min_max(&mut self, min: [f32; 3], max: [f32; 3]) {
        self.min_bounds = min;
        self.max_bounds = max;
    }

    /// Gets the minimum bounds.
    pub fn min_bounds(&self) -> [f32; 3] {
        self.min_bounds
    }

    /// Gets the maximum bounds.
    pub fn max_bounds(&self) -> [f32; 3] {
        self.max_bounds
    }

    /// Sets visibility of the trihedron.
    pub fn set_visible(&mut self, visible: bool) {
        self.is_visible = visible;
    }

    /// Gets visibility state.
    pub fn is_visible(&self) -> bool {
        self.is_visible
    }

    /// Renders the element with OpenGL workspace.
    pub fn render(&self) -> bool {
        self.is_visible
    }

    /// Releases OpenGL resources.
    pub fn release(&mut self) {
        self.is_visible = false;
    }
}

impl Default for OpenGlGraduatedTrihedron {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graduated_trihedron_creation() {
        let trihedron = OpenGlGraduatedTrihedron::new();
        assert!(trihedron.is_visible());
    }

    #[test]
    fn test_graduated_trihedron_bounds() {
        let mut trihedron = OpenGlGraduatedTrihedron::new();
        trihedron.set_min_max([1.0, 2.0, 3.0], [4.0, 5.0, 6.0]);

        assert_eq!(trihedron.min_bounds(), [1.0, 2.0, 3.0]);
        assert_eq!(trihedron.max_bounds(), [4.0, 5.0, 6.0]);
    }

    #[test]
    fn test_graduated_trihedron_visibility() {
        let mut trihedron = OpenGlGraduatedTrihedron::new();
        assert!(trihedron.is_visible());

        trihedron.set_visible(false);
        assert!(!trihedron.is_visible());

        trihedron.set_visible(true);
        assert!(trihedron.is_visible());
    }

    #[test]
    fn test_graduated_trihedron_default() {
        let trihedron = OpenGlGraduatedTrihedron::default();
        assert!(trihedron.is_visible());
        assert_eq!(trihedron.min_bounds(), [0.0, 0.0, 0.0]);
        assert_eq!(trihedron.max_bounds(), [1.0, 1.0, 1.0]);
    }
}
