// FILE: open_gl_layer_list.rs
// occt: OpenGl_LayerList

/// List of OpenGL rendering layers.
#[derive(Debug, Clone)]
pub struct OpenGlLayerList;

impl OpenGlLayerList {
    /// Creates a new layer list.
    pub fn new() -> Self {
        OpenGlLayerList
    }

    /// Gets layer count.
    pub fn count(&self) -> u32 {
        0
    }
}

impl Default for OpenGlLayerList {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layer_list_creation() {
        let _list = OpenGlLayerList::new();
    }
}
