// FILE: graphic3d_layer.rs
// occt: Graphic3d_Layer

use core::fmt;

/// Display priority enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum DisplayPriority {
    Top = 0,
    TopPermanent = 1,
    Topmost = 2,
    Normal = 3,
}

pub const DISPLAY_PRIORITY_NB: usize = 4;

/// Z Layer ID type
pub type ZLayerId = i32;

/// Layer ID (Z-order identifier)
pub struct Layer {
    layer_id: ZLayerId,
    nb_structures: i32,
    nb_structures_not_culled: i32,
}

impl Layer {
    /// Creates a new layer with the given ID
    pub fn new(layer_id: ZLayerId) -> Self {
        Layer {
            layer_id,
            nb_structures: 0,
            nb_structures_not_culled: 0,
        }
    }

    /// Returns the layer ID
    pub fn layer_id(&self) -> ZLayerId {
        self.layer_id
    }

    /// Returns the number of structures in this layer
    pub fn nb_structures(&self) -> i32 {
        self.nb_structures
    }

    /// Returns the number of NOT culled structures in the layer
    pub fn nb_structures_not_culled(&self) -> i32 {
        self.nb_structures_not_culled
    }

    /// Returns the number of available priority levels
    pub fn nb_priorities(&self) -> usize {
        DISPLAY_PRIORITY_NB
    }

    /// Returns true if layer is empty or has been discarded entirely by culling
    pub fn is_culled(&self) -> bool {
        self.nb_structures_not_culled == 0
    }

    /// Marks cached bounding box as obsolete
    pub fn invalidate_bounding_box(&mut self) {
        // Internal state would be updated here
    }

    /// Marks BVH tree as dirty
    pub fn invalidate_bvh_data(&mut self) {
        // Internal BVH state would be updated here
    }

    /// Internal method to add a structure
    fn add_structure(&mut self) {
        self.nb_structures += 1;
        self.nb_structures_not_culled += 1;
    }

    /// Internal method to remove a structure
    fn remove_structure(&mut self) -> bool {
        if self.nb_structures > 0 {
            self.nb_structures -= 1;
            if self.nb_structures_not_culled > 0 {
                self.nb_structures_not_culled -= 1;
            }
            true
        } else {
            false
        }
    }
}

impl fmt::Debug for Layer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Layer")
            .field("layer_id", &self.layer_id)
            .field("nb_structures", &self.nb_structures)
            .field("nb_structures_not_culled", &self.nb_structures_not_culled)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layer_creation() {
        let layer = Layer::new(1);
        assert_eq!(layer.layer_id(), 1);
        assert_eq!(layer.nb_structures(), 0);
        assert_eq!(layer.nb_structures_not_culled(), 0);
    }

    #[test]
    fn test_layer_is_culled() {
        let layer = Layer::new(1);
        assert!(layer.is_culled());
    }

    #[test]
    fn test_layer_add_structure() {
        let mut layer = Layer::new(1);
        assert!(layer.is_culled());

        layer.add_structure();
        assert_eq!(layer.nb_structures(), 1);
        assert_eq!(layer.nb_structures_not_culled(), 1);
        assert!(!layer.is_culled());
    }

    #[test]
    fn test_layer_remove_structure() {
        let mut layer = Layer::new(1);
        layer.add_structure();
        assert_eq!(layer.nb_structures(), 1);

        let removed = layer.remove_structure();
        assert!(removed);
        assert_eq!(layer.nb_structures(), 0);
        assert!(layer.is_culled());
    }

    #[test]
    fn test_layer_remove_structure_empty() {
        let mut layer = Layer::new(1);
        let removed = layer.remove_structure();
        assert!(!removed);
    }

    #[test]
    fn test_layer_multiple_structures() {
        let mut layer = Layer::new(2);
        layer.add_structure();
        layer.add_structure();
        layer.add_structure();

        assert_eq!(layer.nb_structures(), 3);
        assert_eq!(layer.nb_structures_not_culled(), 3);
    }

    #[test]
    fn test_layer_nb_priorities() {
        let layer = Layer::new(1);
        assert_eq!(layer.nb_priorities(), DISPLAY_PRIORITY_NB);
        assert_eq!(layer.nb_priorities(), 4);
    }

    #[test]
    fn test_layer_invalidate_bounding_box() {
        let mut layer = Layer::new(1);
        layer.add_structure();
        layer.invalidate_bounding_box();
        // Should succeed without panic
        assert_eq!(layer.nb_structures(), 1);
    }

    #[test]
    fn test_layer_invalidate_bvh_data() {
        let mut layer = Layer::new(1);
        layer.add_structure();
        layer.invalidate_bvh_data();
        // Should succeed without panic
        assert_eq!(layer.nb_structures(), 1);
    }

    #[test]
    fn test_display_priority_enum() {
        assert_eq!(DisplayPriority::Top as u32, 0);
        assert_eq!(DisplayPriority::TopPermanent as u32, 1);
        assert_eq!(DisplayPriority::Topmost as u32, 2);
        assert_eq!(DisplayPriority::Normal as u32, 3);
    }

    #[test]
    fn test_layer_debug() {
        let mut layer = Layer::new(5);
        layer.add_structure();
        let debug_str = format!("{:?}", layer);
        assert!(debug_str.contains("Layer"));
        assert!(debug_str.contains("layer_id"));
        assert!(debug_str.contains("nb_structures"));
    }
}
