// FILE: open_gl_layer.rs
// occt: OpenGl_Layer

/// OpenGL rendering layer for hierarchical scene organization.
#[derive(Debug, Clone)]
pub struct OpenGlLayer {
    id: u32,
    is_visible: bool,
}

impl OpenGlLayer {
    /// Creates a new layer with given ID.
    pub fn new(id: u32) -> Self {
        OpenGlLayer { id, is_visible: true }
    }

    /// Gets the layer ID.
    pub fn id(&self) -> u32 {
        self.id
    }

    /// Sets visibility.
    pub fn set_visible(&mut self, visible: bool) {
        self.is_visible = visible;
    }

    /// Gets visibility.
    pub fn is_visible(&self) -> bool {
        self.is_visible
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layer_creation() {
        let layer = OpenGlLayer::new(0);
        assert_eq!(layer.id(), 0);
        assert!(layer.is_visible());
    }

    #[test]
    fn test_layer_visibility() {
        let mut layer = OpenGlLayer::new(1);
        layer.set_visible(false);
        assert!(!layer.is_visible());
    }
}
