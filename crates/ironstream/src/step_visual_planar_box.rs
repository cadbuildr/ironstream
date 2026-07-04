// FILE: step_visual_planar_box.rs
// occt: StepVisual_PlanarBox

/// A planar box in STEP representation.
///
/// This represents a 2D rectangular region.
pub struct PlanarBox {
    name: String,
    size_x: f64,
    size_y: f64,
    location_x: f64,
    location_y: f64,
}

impl PlanarBox {
    /// Creates a new planar box.
    pub fn new(name: String) -> Self {
        PlanarBox {
            name,
            size_x: 0.0,
            size_y: 0.0,
            location_x: 0.0,
            location_y: 0.0,
        }
    }

    /// Returns the name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Sets the size.
    pub fn set_size(&mut self, x: f64, y: f64) {
        self.size_x = x;
        self.size_y = y;
    }

    /// Returns the size.
    pub fn size(&self) -> (f64, f64) {
        (self.size_x, self.size_y)
    }

    /// Sets the location.
    pub fn set_location(&mut self, x: f64, y: f64) {
        self.location_x = x;
        self.location_y = y;
    }

    /// Returns the location.
    pub fn location(&self) -> (f64, f64) {
        (self.location_x, self.location_y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_planar_box_new() {
        let bbox = PlanarBox::new("Box".to_string());
        assert_eq!(bbox.name(), "Box");
        assert_eq!(bbox.size(), (0.0, 0.0));
        assert_eq!(bbox.location(), (0.0, 0.0));
    }

    #[test]
    fn test_set_size_and_location() {
        let mut bbox = PlanarBox::new("MyBox".to_string());
        bbox.set_size(100.0, 50.0);
        bbox.set_location(10.0, 20.0);
        assert_eq!(bbox.size(), (100.0, 50.0));
        assert_eq!(bbox.location(), (10.0, 20.0));
    }
}
