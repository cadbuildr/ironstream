// FILE: xs_control_connected_shapes.rs
// occt: XSControl_ConnectedShapes

/// Utility for finding connected shapes in a shape hierarchy.
/// Analyzes shape relationships to identify connected components.
#[derive(Clone, Debug)]
pub struct XSControlConnectedShapes {
    /// Root shape ID
    root_shape_id: u32,
    /// Connected components
    components: Vec<u32>,
}

impl XSControlConnectedShapes {
    /// Creates a new connected shapes analyzer.
    pub fn new(root_shape_id: u32) -> Self {
        Self {
            root_shape_id,
            components: Vec::new(),
        }
    }

    /// Returns the root shape ID.
    pub fn root_shape_id(&self) -> u32 {
        self.root_shape_id
    }

    /// Adds a connected component.
    pub fn add_component(&mut self, component_id: u32) {
        if !self.components.contains(&component_id) {
            self.components.push(component_id);
        }
    }

    /// Returns the number of connected components.
    pub fn nb_components(&self) -> usize {
        self.components.len()
    }

    /// Returns all components.
    pub fn components(&self) -> &[u32] {
        &self.components
    }
}

impl Default for XSControlConnectedShapes {
    fn default() -> Self {
        Self::new(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let shapes = XSControlConnectedShapes::new(100);
        assert_eq!(shapes.root_shape_id(), 100);
        assert_eq!(shapes.nb_components(), 0);
    }

    #[test]
    fn test_add_component() {
        let mut shapes = XSControlConnectedShapes::new(1);
        shapes.add_component(10);
        assert_eq!(shapes.nb_components(), 1);

        shapes.add_component(20);
        assert_eq!(shapes.nb_components(), 2);

        shapes.add_component(10); // Duplicate
        assert_eq!(shapes.nb_components(), 2);
    }
}
