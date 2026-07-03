// FILE: geom2d_hatch_hatcher.rs
// occt: Geom2dHatch_Hatcher

/// Main hatcher for 2D hatching operations.
pub struct Geom2dHatchHatcher {
    elements: Vec<(f64, f64)>,
    hatchings: Vec<(f64, f64)>,
}

impl Geom2dHatchHatcher {
    pub fn new() -> Self {
        Geom2dHatchHatcher {
            elements: Vec::new(),
            hatchings: Vec::new(),
        }
    }

    pub fn add_element(&mut self, x: f64, y: f64) {
        self.elements.push((x, y));
    }

    pub fn add_hatching(&mut self, x: f64, y: f64) {
        self.hatchings.push((x, y));
    }

    pub fn element_count(&self) -> usize {
        self.elements.len()
    }

    pub fn hatching_count(&self) -> usize {
        self.hatchings.len()
    }
}

impl Default for Geom2dHatchHatcher {
    fn default() -> Self {
        Geom2dHatchHatcher::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hatcher_creation() {
        let hatcher = Geom2dHatchHatcher::new();
        assert_eq!(hatcher.element_count(), 0);
    }

    #[test]
    fn test_add_element() {
        let mut hatcher = Geom2dHatchHatcher::new();
        hatcher.add_element(1.0, 2.0);
        assert_eq!(hatcher.element_count(), 1);
    }
}
