// FILE: open_gl_layer_filter.rs
// occt: OpenGl_LayerFilter

/// Filter for selecting which rendering layers to process.
#[derive(Debug, Clone)]
pub struct OpenGlLayerFilter {
    allow_transparent: bool,
    allow_opaque: bool,
}

impl OpenGlLayerFilter {
    /// Creates a new layer filter.
    pub fn new() -> Self {
        OpenGlLayerFilter {
            allow_transparent: true,
            allow_opaque: true,
        }
    }

    /// Sets transparent filtering.
    pub fn set_transparent(&mut self, allow: bool) {
        self.allow_transparent = allow;
    }

    /// Sets opaque filtering.
    pub fn set_opaque(&mut self, allow: bool) {
        self.allow_opaque = allow;
    }

    /// Checks if transparent is allowed.
    pub fn allow_transparent(&self) -> bool {
        self.allow_transparent
    }

    /// Checks if opaque is allowed.
    pub fn allow_opaque(&self) -> bool {
        self.allow_opaque
    }
}

impl Default for OpenGlLayerFilter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layer_filter_creation() {
        let filter = OpenGlLayerFilter::new();
        assert!(filter.allow_transparent());
        assert!(filter.allow_opaque());
    }

    #[test]
    fn test_layer_filter_transparent() {
        let mut filter = OpenGlLayerFilter::new();
        filter.set_transparent(false);
        assert!(!filter.allow_transparent());
    }
}
