// FILE: step_visual_presentation_area.rs
// occt: StepVisual_PresentationArea

/// A presentation area in STEP representation.
///
/// This defines a presentation area in a drawing or view.
pub struct PresentationArea {
    name: String,
    area_extent: (f64, f64),
}

impl PresentationArea {
    /// Creates a new presentation area.
    pub fn new(name: String) -> Self {
        PresentationArea {
            name,
            area_extent: (0.0, 0.0),
        }
    }

    /// Returns the name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Sets the area extent (width, height).
    pub fn set_area_extent(&mut self, width: f64, height: f64) {
        self.area_extent = (width, height);
    }

    /// Returns the area extent.
    pub fn area_extent(&self) -> (f64, f64) {
        self.area_extent
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_presentation_area_new() {
        let area = PresentationArea::new("Area".to_string());
        assert_eq!(area.name(), "Area");
        assert_eq!(area.area_extent(), (0.0, 0.0));
    }

    #[test]
    fn test_set_area_extent() {
        let mut area = PresentationArea::new("PresentationArea".to_string());
        area.set_area_extent(500.0, 400.0);
        assert_eq!(area.area_extent(), (500.0, 400.0));
    }
}
