// FILE: step_visual_mechanical_design_geometric_presentation_area.rs
// occt: StepVisual_MechanicalDesignGeometricPresentationArea

/// A mechanical design geometric presentation area in STEP representation.
///
/// This represents a presentation area for mechanical design geometry.
pub struct MechanicalDesignGeometricPresentationArea {
    name: String,
    area_id: i32,
}

impl MechanicalDesignGeometricPresentationArea {
    /// Creates a new mechanical design geometric presentation area.
    pub fn new(name: String) -> Self {
        MechanicalDesignGeometricPresentationArea {
            name,
            area_id: 0,
        }
    }

    /// Returns the name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Sets the area ID.
    pub fn set_area_id(&mut self, id: i32) {
        self.area_id = id;
    }

    /// Returns the area ID.
    pub fn area_id(&self) -> i32 {
        self.area_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mechanical_design_geometric_presentation_area_new() {
        let area = MechanicalDesignGeometricPresentationArea::new("Area1".to_string());
        assert_eq!(area.name(), "Area1");
        assert_eq!(area.area_id(), 0);
    }

    #[test]
    fn test_set_area_id() {
        let mut area = MechanicalDesignGeometricPresentationArea::new("Area".to_string());
        area.set_area_id(42);
        assert_eq!(area.area_id(), 42);
    }
}
