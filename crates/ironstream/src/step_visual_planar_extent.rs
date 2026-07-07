// FILE: step_visual_planar_extent.rs
// occt: StepVisual_PlanarExtent

/// A planar extent in STEP representation.
///
/// This defines the 2D size of a planar region.
pub struct PlanarExtent {
    name: String,
    size_in_x: f64,
    size_in_y: f64,
}

impl PlanarExtent {
    /// Creates a new planar extent.
    pub fn new(name: String) -> Self {
        PlanarExtent {
            name,
            size_in_x: 0.0,
            size_in_y: 0.0,
        }
    }

    /// Returns the name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Sets the size in X.
    pub fn set_size_in_x(&mut self, size: f64) {
        self.size_in_x = size;
    }

    /// Returns the size in X.
    pub fn size_in_x(&self) -> f64 {
        self.size_in_x
    }

    /// Sets the size in Y.
    pub fn set_size_in_y(&mut self, size: f64) {
        self.size_in_y = size;
    }

    /// Returns the size in Y.
    pub fn size_in_y(&self) -> f64 {
        self.size_in_y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_planar_extent_new() {
        let extent = PlanarExtent::new("Extent".to_string());
        assert_eq!(extent.name(), "Extent");
        assert_eq!(extent.size_in_x(), 0.0);
        assert_eq!(extent.size_in_y(), 0.0);
    }

    #[test]
    fn test_set_sizes() {
        let mut extent = PlanarExtent::new("MyExtent".to_string());
        extent.set_size_in_x(200.0);
        extent.set_size_in_y(150.0);
        assert_eq!(extent.size_in_x(), 200.0);
        assert_eq!(extent.size_in_y(), 150.0);
    }
}
